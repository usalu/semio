import assert from "node:assert/strict";
import { existsSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
const root = process.cwd(), ticket = dirname(dirname(import.meta.dir)), product = "♻️mit-bestand/📋️bericht", packagePath = join(root, product, "📦️packages/🟦️typescript");
const catalog = JSON.parse(readFileSync(join(root, product, "🔨️modules/📄️documents/🔣️.json"), "utf8"));
const directories = catalog.documents.map((document: { id: string }) => ({ id: document.id, path: join(packagePath, "dist/documents", document.id) }));
const snapshot = (directory: string): Map<string, Buffer> => new Map(readdirSync(directory).map(name => [name, readFileSync(join(directory, name))]));
const before = new Map<string, Map<string, Buffer>>(directories.map((directory: { id: string; path: string }) => [directory.id, snapshot(directory.path)]));
for (const directory of directories) assert.equal(JSON.parse(before.get(directory.id)!.get(".nx-artifact.json")!.toString()).owner, `@semio-tech/mit-bestand-bericht:build-${directory.id}`);
async function run(name: string): Promise<string> {
  const child = Bun.spawn(["bun", "nx", "run", "@semio-tech/mit-bestand-bericht:build", "--parallel=1", "--output-style=stream"], { cwd: root, env: { ...process.env, SEMIO_TICKET_DIR: ticket }, stdout: "pipe", stderr: "pipe" });
  const [out, err, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(ticket, `🗑️generated/report-${name}.log`), out + err);
  assert.equal(status, 0, out + err);
  return out.replace(/\u001b\[[0-9;]*m/g, "");
}
await run("cache-current-build");
for (const directory of directories) assert.deepEqual(snapshot(directory.path), before.get(directory.id), `${directory.id}: independently compiled PDF bytes changed`);
console.log("[DEBUG] Report final producer source: all three PDF results match the qualified native repeat PASS");
const backup = join(ticket, "🗑️generated/report-document-restore-backup");
assert.equal(existsSync(backup), false); mkdirSync(backup);
const moved: { id: string; path: string }[] = [];
try {
  for (const directory of directories) { renameSync(directory.path, join(backup, directory.id)); moved.push(directory); }
  const output = await run("cache-restored");
  for (const directory of directories) {
    assert.ok(output.includes(`@semio-tech/mit-bestand-bericht:build-${directory.id}  [local cache]`), `${directory.id}: missing actual Nx cache hit`);
    assert.deepEqual(snapshot(directory.path), before.get(directory.id), `${directory.id}: restored artifact differs`);
  }
  assert.ok(["local cache", "existing outputs match the cache, left as is"].some(label => output.includes(`@semio-tech/mit-bestand-bericht:generate-actor-network  [${label}]`)), "The unchanged actor-network generator must reuse its result");
  rmSync(backup, { recursive: true });
  console.log("[DEBUG] Actual Nx restoration: all three report PDF directories and markers match; unchanged generator cached PASS");
} catch (error) {
  for (const directory of moved) if (!existsSync(directory.path)) renameSync(join(backup, directory.id), directory.path);
  throw error;
}
