/** C4c open-only: two clients + late joiner on framework document/ws (sqlite). */
import { spawn, spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 40; depth++) {
    if (existsSync(join(current, "bun.lock")) && existsSync(join(current, "AGENTS.md"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("repo root not found");
}

const here = fileURLToPath(new URL(".", import.meta.url));
const repo = findRepoRoot(here);
const port = Number(process.argv.includes("--port") ? process.argv[process.argv.indexOf("--port") + 1] : "7772");
const binary = process.argv.includes("--binary") ? process.argv[process.argv.indexOf("--binary") + 1] : join(here, "target", "debug", "os-hub");
const dataRoot = resolve(join(here, "generated", "hub-data-open"));
const outDir = join(here, "generated");
const origin = `http://127.0.0.1:${port}`;
const wsOrigin = `ws://127.0.0.1:${port}`;

if (existsSync(dataRoot)) rmSync(dataRoot, { recursive: true, force: true });
mkdirSync(dataRoot, { recursive: true, mode: 0o700 });
mkdirSync(outDir, { recursive: true });

const semio = readdirSync(repo).find((n) => n.startsWith(".") && n.includes("semio"))!;
const hubData = readdirSync(join(repo, semio)).find((n) => n.includes("hub") && !n.includes("repo"))!;
const root = join(repo, semio, hubData);
let catalog = "";
for (const name of ["hc1-boot", "jc1-boot", "gm1-boot"]) {
  const cand = join(root, name, "trusted-catalog");
  if (existsSync(cand)) { catalog = cand; break; }
}
if (!catalog) throw new Error("catalog missing");
cpSync(catalog, join(dataRoot, "trusted-catalog"), { recursive: true });

const accounts = [
  { email: "ada-c4c@example.org", password: "correct horse battery staple", display: "Ada C4c" },
  { email: "bo-c4c@example.org", password: "another perfectly fine phrase", display: "Bo C4c" },
  { email: "cee-c4c@example.org", password: "third late joiner phrase ok", display: "Cee C4c" },
];
for (const account of accounts) {
  const result = spawnSync(binary, ["credential", "set", "--email", account.email, "--display-name", account.display], {
    env: { ...process.env, OS_HUB_DATA: dataRoot },
    input: account.password,
    encoding: "utf8",
  });
  if (result.status !== 0) throw new Error(`credential set failed: ${result.stderr || result.stdout}`);
}

const env: Record<string, string> = {
  ...process.env,
  OS_HUB_PORT: String(port),
  OS_HUB_BIND: "127.0.0.1",
  OS_HUB_DATA: dataRoot,
  OS_HUB_MODE: "production",
  OS_HUB_CREDENTIAL_SIGN_IN: "true",
  OS_HUB_ADMIN_SUBJECTS: `credential.password.v1:${accounts[0].email}`,
  OS_HUB_ALLOWED_ORIGINS: origin,
  OS_HUB_TRUSTED_FORWARDING: "none",
  OS_HUB_STORAGE_BACKEND: "sqlite",
};
const child = spawn(binary, [], { env, stdio: ["ignore", "pipe", "pipe"] });
const chunks: Buffer[] = [];
child.stdout?.on("data", (d) => chunks.push(Buffer.from(d)));
child.stderr?.on("data", (d) => chunks.push(Buffer.from(d)));
const pid = child.pid!;

async function mint(email: string, password: string, device: string) {
  const res = await fetch(`${origin}/auth/sessions`, {
    method: "POST",
    headers: { "content-type": "application/json", origin: "http://127.0.0.1:6066" },
    body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: device, clientClass: "browser" }),
  });
  const body = await res.json();
  if (!res.ok || typeof body.token !== "string") throw new Error(`session ${res.status} ${JSON.stringify(body)}`);
  return body.token as string;
}

function openWs(token: string, actor: string, spaceId: string, documentId: string) {
  return new Promise<WebSocket>((resolveOpen, reject) => {
    const scope = `${spaceId}/${documentId}`;
    const q = new URLSearchParams({ actor, surface: "c4c.surface" });
    const socket = new WebSocket(`${wsOrigin}/scopes/${encodeURIComponent(scope)}/document/ws?${q}`, ["semio.session.v1", token]);
    const t = setTimeout(() => reject(new Error("ws open timeout")), 15000);
    socket.onopen = () => { clearTimeout(t); resolveOpen(socket); };
    socket.onerror = (e) => { clearTimeout(t); reject(e); };
  });
}

try {
  const until = Date.now() + 300000;
  while (Date.now() < until) {
    if (child.exitCode !== null) throw new Error(`hub exited ${child.exitCode}\n${Buffer.concat(chunks).toString("utf8").slice(-4000)}`);
    try { if ((await fetch(`${origin}/healthz`)).ok) break; } catch {}
    await Bun.sleep(400);
  }
  if (!(await fetch(`${origin}/healthz`)).ok) throw new Error("healthz failed");
  console.log("PASS healthz");
  const readyUntil = Date.now() + 120000;
  while (Date.now() < readyUntil) {
    try {
      const res = await fetch(`${origin}/readyz`);
      const body = await res.json();
      if (res.status === 200 && body.status === "ready") break;
    } catch {}
    await Bun.sleep(500);
  }
  console.log("PASS/readyz-attempted");
  const tokenA = await mint(accounts[0].email, accounts[0].password, "ada");
  const tokenB = await mint(accounts[1].email, accounts[1].password, "bo");
  const tokenC = await mint(accounts[2].email, accounts[2].password, "cee");
  console.log("PASS sessions");
  const spaceId = "c4c-studio";
  const documentId = "c4c-doc-1";
  const a = await openWs(tokenA, "actor-a", spaceId, documentId);
  const b = await openWs(tokenB, "actor-b", spaceId, documentId);
  console.log("PASS two clients framework document/ws", a.readyState, b.readyState);
  await Bun.sleep(300);
  if (a.readyState !== WebSocket.OPEN || b.readyState !== WebSocket.OPEN) throw new Error("socket closed");
  a.close();
  // resume-like: reopen B
  const b2 = await openWs(tokenB, "actor-b", spaceId, documentId);
  console.log("PASS rejoin open", b2.readyState);
  b2.close();
  const c = await openWs(tokenC, "actor-c", spaceId, documentId);
  console.log("PASS late joiner open", c.readyState);
  c.close();
  b.close();
  writeFileSync(join(outDir, "live-sqlite-pass.txt"), `PASS port=${port} open+rejoin+late framework document/ws\n`);
  console.log("PASS c4c open-only");
} finally {
  try { process.kill(pid, "SIGTERM"); } catch {}
  writeFileSync(join(outDir, `hub-open-${port}.txt`), Buffer.concat(chunks).toString("utf8").slice(-100000));
}
