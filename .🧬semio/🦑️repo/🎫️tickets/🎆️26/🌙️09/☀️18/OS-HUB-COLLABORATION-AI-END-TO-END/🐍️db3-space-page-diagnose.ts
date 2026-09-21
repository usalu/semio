/** 🔍️ DB3 — boots one hub against an EXISTING data root + directory backend and reads the space
 * page that the H1b probe saw 500 on after a Neo4j restart, printing the hub's own captured stderr.
 * The backend env is inherited from the caller, so the same script serves the postgres and the
 * neo4j directory alike.
 *
 * Usage: `bun 🐍️db3-space-page-diagnose.ts <port> <absoluteDataDir> <absoluteBinaryPath> <email> <password>` */
import { existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, "🌎️hub", "📦️packages", "🦀️rust", "Cargo.toml"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("probe could not locate the repository root above " + start);
}

const repoRoot = findRepoRoot(import.meta.dir);
const hubRustRoot = join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
const { finishLocalHub, startLocalHub, waitForReadiness } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));

const port = Number(process.argv[2]);
const dataRoot = process.argv[3]!;
const binaryPath = process.argv[4]!;
const email = process.argv[5]!;
const password = process.argv[6]!;
mkdirSync(dataRoot, { recursive: true, mode: 0o700 });
process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";

const run = await startLocalHub(repoRoot, hubRustRoot, [{ profileId: "developer", subject: "local-developer-db3", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: true,
});
await waitForReadiness(run, true);
const origin = `http://127.0.0.1:${port}`;

const mint = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "db3-diagnose", clientClass: "browser" }) });
const token = (await mint.json() as any)?.token ?? "";
console.log(`sign-in ${mint.status} token=${token ? "yes" : "no"}`);

const list = await fetch(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${token}` } });
const rows = await list.json() as any[];
console.log(`spaces ${list.status} -> ${JSON.stringify(rows).slice(0, 400)}`);

for (const row of Array.isArray(rows) ? rows : []) {
  const id = row?.space?.id;
  if (!id) continue;
  const page = await fetch(`${origin}/directory/spaces/${encodeURIComponent(id)}`, { headers: { authorization: `Bearer ${token}` } });
  const body = await page.text();
  console.log(`space ${id} -> ${page.status} ${body.slice(0, 600)}`);
}

const { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } = await import(join(repoRoot, "\u{1F9F0}\ufe0fframework", "\u{1F6CD}\ufe0fproducts", "\u{1F4BB}\ufe0fos", "\u{1F528}\ufe0fmodules", "\u{1F4C7}\ufe0fdirectory", "\u{1F9EC}\ufe0fschema", "\u{1F7E6}\ufe0f.ts"));

async function command(requestId: string, body: any): Promise<any> {
  const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, body));
  const response = await fetch(`${origin}/directory/commands`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${token}`, origin: `http://127.0.0.1:${port}` }, body: sealed });
  const text = await response.text();
  try { return { status: response.status, json: JSON.parse(text) }; } catch { return { status: response.status, body: text }; }
}

async function page(id: string, label: string): Promise<void> {
  const answer = await fetch(`${origin}/directory/spaces/${encodeURIComponent(id)}`, { headers: { authorization: `Bearer ${token}` } });
  console.log(`  ${label}: ${answer.status} ${(await answer.text()).slice(0, 300)}`);
}

async function listId(name: string): Promise<string> {
  for (let attempt = 0; attempt < 60; attempt += 1) {
    const rows = await (await fetch(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${token}` } })).json() as any[];
    const hit = (Array.isArray(rows) ? rows : []).find((row: any) => row?.space?.name === name);
    if (hit) return hit.space.id;
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error(`space ${name} never appeared`);
}

console.log("--- bisect: a fresh space, then an invite, then a redemption ---");
const stamp = String(Date.now());
const rid = (): string => Array.from({ length: 32 }, () => "0123456789abcdef"[Math.floor(Math.random() * 16)]).join("");
console.log(`create-space: ${JSON.stringify(await command(rid(), { kind: "create-space", name: `DB3 Bisect ${stamp}`, spaceKind: "atelier", visibility: "private" }))}`);
const freshId = await listId(`DB3 Bisect ${stamp}`);
await page(freshId, "owner only");
const invited = await command(rid(), { kind: "create-invite", spaceId: freshId, role: "spectator", ttlSecs: 3600 });
console.log(`create-invite: ${invited.status}`);
await page(freshId, "owner + one open invite");

console.log("--- hub output ---");
console.log(run.output());
await finishLocalHub(run);
