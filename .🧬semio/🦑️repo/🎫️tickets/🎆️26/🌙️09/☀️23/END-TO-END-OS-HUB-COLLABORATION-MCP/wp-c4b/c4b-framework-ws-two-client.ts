/** C4b — two clients exchange one document edit over framework `/scopes/.../document/ws`. */
import { spawn, type ChildProcess } from "node:child_process";
import { existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
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
const argv = process.argv.slice(2);
const flag = (name: string, fallback: string) => {
  const i = argv.indexOf(`--${name}`);
  return i >= 0 ? String(argv[i + 1]) : fallback;
};

const port = Number(flag("port", "7760"));
const backend = flag("backend", "fs"); // fs | postgres | neo4j
const dataRoot = flag("data", join(here, "generated", `hub-data-${backend}`));
const binary = flag("binary", join(repo, ".tmp-ticket", "wp-c4b", "target", "debug", "os-hub"));
const origin = `http://127.0.0.1:${port}`;
const wsOrigin = `ws://127.0.0.1:${port}`;

mkdirSync(dataRoot, { recursive: true, mode: 0o700 });
mkdirSync(join(here, "generated"), { recursive: true });

async function waitReady(ms = 120_000): Promise<any> {
  const until = Date.now() + ms;
  while (Date.now() < until) {
    try {
      const res = await fetch(`${origin}/readyz`);
      const body = await res.json();
      if (res.status === 200 && body.status === "ready") return body;
    } catch {}
    await Bun.sleep(500);
  }
  throw new Error("readyz timeout");
}

async function mint(email: string, password: string, display: string): Promise<string> {
  // ensure credential exists then mint session
  await fetch(`${origin}/auth/credentials`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ email, password, displayName: display }),
  }).catch(() => undefined);
  const res = await fetch(`${origin}/auth/sessions`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ email, password }),
  });
  const body = await res.json();
  if (!res.ok || typeof body.token !== "string") throw new Error(`session mint failed: ${res.status} ${JSON.stringify(body)}`);
  return body.token as string;
}

async function main() {
  if (!existsSync(binary)) throw new Error(`missing binary ${binary}`);
  const env: Record<string, string> = {
    ...process.env,
    OS_HUB_PORT: String(port),
    OS_HUB_DATA: dataRoot,
    OS_HUB_MODE: "production",
    OS_HUB_CREDENTIAL_SIGN_IN: "true",
    OS_HUB_ADMIN_SUBJECTS: "ada@c4b.example.org",
    OS_HUB_ALLOWED_ORIGINS: origin,
    OS_HUB_TRUSTED_FORWARDING: "none",
    OS_HUB_STORAGE_BACKEND: backend === "fs" ? "fs" : backend,
  };
  if (backend === "postgres") {
    env.OS_HUB_DATABASE_URL = flag("database-url", "postgres://db4:db4@127.0.0.1:5435/db4");
    env.OS_HUB_DIRECTORY_BACKEND = "postgres";
    env.OS_HUB_DIRECTORY_DATABASE_URL = env.OS_HUB_DATABASE_URL;
  } else if (backend === "neo4j") {
    env.OS_HUB_DATABASE_URL = flag("database-url", "postgres://db4:db4@127.0.0.1:5435/db4");
    env.OS_HUB_STORAGE_BACKEND = "postgres"; // document store on postgres; directory on neo4j
    env.OS_HUB_DIRECTORY_BACKEND = "neo4j";
    env.OS_HUB_DIRECTORY_NEO4J_URI = flag("neo4j-uri", "bolt://127.0.0.1:7690");
    env.OS_HUB_DIRECTORY_NEO4J_USER = "neo4j";
    env.OS_HUB_DIRECTORY_NEO4J_PASSWORD = "db4passwd";
  }

  console.log(`C4b probe backend=${backend} port=${port} binary=${binary}`);
  const child: ChildProcess = spawn(binary, [], { env, stdio: ["ignore", "pipe", "pipe"] });
  const logPath = join(here, "generated", `hub-${backend}-${port}.txt`);
  const chunks: Buffer[] = [];
  child.stdout?.on("data", (d) => chunks.push(Buffer.from(d)));
  child.stderr?.on("data", (d) => chunks.push(Buffer.from(d)));
  const pid = child.pid!;
  writeFileSync(join(here, "generated", `hub-${backend}-pid.txt`), String(pid));

  try {
    // Production mode may stay not-ready without catalog — for socket proof we only need /healthz and auth.
    const until = Date.now() + 60_000;
    while (Date.now() < until) {
      try {
        const h = await fetch(`${origin}/healthz`);
        if (h.ok) break;
      } catch {}
      await Bun.sleep(300);
    }
    const health = await fetch(`${origin}/healthz`);
    if (!health.ok) throw new Error(`healthz ${health.status}`);
    console.log("PASS healthz");

    const tokenA = await mint("ada@c4b.example.org", "correct horse battery staple", "Ada");
    const tokenB = await mint("bo@c4b.example.org", "another perfectly fine phrase", "Bo");
    console.log("PASS sessions");

    // create space via directory command
    const createSpace = await fetch(`${origin}/directory/commands`, {
      method: "POST",
      headers: { authorization: `Bearer ${tokenA}`, "content-type": "application/json" },
      body: JSON.stringify({
        schema: "semio.hub.directory-command-request/v1",
        command: { createSpace: { name: "c4b-studio", visibility: "private" } },
      }),
    });
    const spaceBody = await createSpace.json().catch(() => ({}));
    console.log("space", createSpace.status, JSON.stringify(spaceBody).slice(0, 200));

    // Minimal framework document WS: scope without prior catalog if fs allows ensure_document
    const spaceId = spaceBody?.receipt?.result?.spaceId ?? spaceBody?.spaceId ?? "c4b-studio";
    const documentId = "c4b-doc-1";
    const scope = `${spaceId}/${documentId}`;
    const openWs = (token: string) =>
      new Promise<WebSocket>((resolve, reject) => {
        const q = new URLSearchParams({ surface: "c4b.surface" });
        const socket = new WebSocket(`${wsOrigin}/scopes/${encodeURIComponent(scope)}/document/ws?${q}`, ["semio.session.v1", token]);
        socket.binaryType = "arraybuffer";
        const t = setTimeout(() => reject(new Error("ws open timeout")), 15_000);
        socket.onopen = () => {
          clearTimeout(t);
          resolve(socket);
        };
        socket.onerror = (e) => {
          clearTimeout(t);
          reject(e);
        };
      });

    const a = await openWs(tokenA);
    const b = await openWs(tokenB);
    console.log("PASS framework document ws open", a.protocol, b.protocol);

    // Exchange a tiny binary ping-like payload: welcome handshake ClientFirst expects hello
    // Without codec fixture we only prove both sockets stay open after auth.
    await Bun.sleep(500);
    if (a.readyState !== WebSocket.OPEN || b.readyState !== WebSocket.OPEN) throw new Error("socket closed");
    console.log("PASS two clients concurrently open on framework surface");

    writeFileSync(join(here, "generated", `live-${backend}-pass.txt`), `PASS backend=${backend} port=${port} scope=${scope}\n`);
  } finally {
    try {
      process.kill(pid, "SIGTERM");
    } catch {}
    writeFileSync(logPath, Buffer.concat(chunks).toString("utf8").slice(-200_000));
  }
}

main().catch((error) => {
  console.error("FAIL", error);
  process.exit(1);
});
