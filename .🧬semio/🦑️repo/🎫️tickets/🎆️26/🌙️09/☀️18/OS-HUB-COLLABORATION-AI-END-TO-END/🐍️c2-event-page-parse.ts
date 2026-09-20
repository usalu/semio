/** 📄️ C2 — runs the shell's OWN `parseDirectoryEventPageV1` against the live hub's
 * `/directory/event-page/v1?after=0`, so the refusal that puts Home into `directory-bootstrap.invalid-page`
 * is named exactly instead of inferred.
 * Usage: bun 🐍️c2-event-page-parse.ts <origin> <email> <password> */
import { existsSync } from "node:fs";
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
const origin = process.argv[2] ?? "http://127.0.0.1:7611";
const email = process.argv[3] ?? "user1@semio.dev";
const password = process.argv[4] ?? "gm1-local-dev-pass-1";
const schema = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🟦️.ts");
const { parseDirectoryEventPageV1 } = await import(schema);

const minted = await fetch(`${origin}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "c2-event-page-parse", clientClass: "browser" }),
});
const token: string = ((await minted.json()) as { token?: string }).token ?? "";
console.log(`sign-in status=${minted.status} token=${token.slice(0, 20)}…`);

const response = await fetch(`${origin}/directory/event-page/v1?after=0`, { headers: { authorization: `Bearer ${token}` } });
const text = await response.text();
console.log(`event-page status=${response.status} bytes=${new TextEncoder().encode(text).length}`);
console.log(`event-page body=${text.slice(0, 4000)}`);

try {
  const page = await parseDirectoryEventPageV1(text);
  console.log(`PARSE ok events=${page.events.length} through=${page.throughSeqInclusive} kinds=${page.events.map((event: { body: { kind: string } }) => event.body.kind).join(",")}`);
} catch (error) {
  console.log(`PARSE refused: ${error instanceof Error ? error.message : String(error)}`);
  const object = JSON.parse(text);
  console.log(`envelope keys=${JSON.stringify(Object.keys(object))}`);
  for (const [index, event] of (object.events ?? []).entries()) {
    console.log(`  event[${index}] keys=${JSON.stringify(Object.keys(event))} body.kind=${event?.body?.kind} bodyKeys=${JSON.stringify(Object.keys(event?.body ?? {}))}`);
  }
}
