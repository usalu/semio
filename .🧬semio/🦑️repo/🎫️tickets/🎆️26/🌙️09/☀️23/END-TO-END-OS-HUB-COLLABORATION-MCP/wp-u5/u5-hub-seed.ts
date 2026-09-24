#!/usr/bin/env bun
/** 🌱️ U5 — seeds the U5 hub: signs ada in over the credential route and creates named spaces through the directory
 * command route, then lists what the directory answers for her. Usage: bun u5-hub-seed.ts <origin> [--delete=<namePrefix>...] <spaceName...> */
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json")) && existsSync(join(current, "bun.lock"))) return current;
    current = dirname(current);
  }
  throw new Error("repository root not found");
}
const repoRoot = findRepoRoot(import.meta.dir);
const { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } = await import(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🟦️.ts"));
const origin = process.argv[2] ?? "http://127.0.0.1:8080";
const deletePrefixes = process.argv.slice(3).filter((arg) => arg.startsWith("--delete=")).map((arg) => arg.slice("--delete=".length));
const names = process.argv.slice(3).filter((arg) => !arg.startsWith("--delete="));

const minted = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: "ada@example.org", password: "correct horse battery staple", deviceInstanceId: "u5-seed", clientClass: "browser" }) });
const session = (await minted.json()) as { token?: string; user_id?: string };
if (!session.token) throw new Error(`sign-in failed ${minted.status}`);
for (const [index, name] of names.entries()) {
  const requestId = crypto.randomUUID().replaceAll("-", "");
  const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, { kind: "create-space", name, spaceKind: "atelier", visibility: "private" }));
  const response = await fetch(`${origin}/directory/commands`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${session.token}`, origin: "http://127.0.0.1:6580" }, body: sealed });
  console.log(`create-space ${name}: ${response.status} ${(await response.text()).slice(0, 160)}`);
}
if (deletePrefixes.length > 0) {
  const before = (await (await fetch(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${session.token}` } })).json()) as { space?: { id: string; name: string } }[];
  for (const row of before) {
    if (!row.space || !deletePrefixes.some((prefix) => row.space!.name.startsWith(prefix))) continue;
    const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(crypto.randomUUID().replaceAll("-", ""), { kind: "delete-space", spaceId: row.space.id }));
    const response = await fetch(`${origin}/directory/commands`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${session.token}`, origin: "http://127.0.0.1:6580" }, body: sealed });
    console.log(`delete-space ${row.space.name}: ${response.status}`);
  }
}
const listed = await fetch(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${session.token}` } });
const rows = (await listed.json()) as { space?: { id: string; name: string } }[];
console.log(JSON.stringify({ user: session.user_id, spaces: rows.map((row) => row.space?.name) }));
