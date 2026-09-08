import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { appendFileSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const root = process.cwd(), ticket = dirname(dirname(import.meta.dir)), product = "🧰️framework/🛍️products/📓️print";
const catalog = JSON.parse(readFileSync(join(root, product, "🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json"), "utf8"));
const api = await import(join(root, product, "🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts"));
const directories = catalog.documents.map((document: { id: string }) => ({ id: document.id, path: api.printDocumentOutputDirectory(document.id, root) }));
const snapshot = (directory: string): Record<string, string> => Object.fromEntries(readdirSync(directory).sort().map(name => [name, createHash("sha256").update(readFileSync(join(directory, name))).digest("hex")]));
const backup = join(ticket, "🗑️generated/print-collection-restore-backup");
assert.equal(existsSync(backup), false, "A previous collection probe still owns its backup");

async function run(name: string): Promise<string> {
  const log = join(ticket, `🗑️generated/print-collection-${name}.log`);
  writeFileSync(log, "");
  console.log(`[DEBUG] Starting Print collection ${name}; live task output: ${log}`);
  const child = Bun.spawn(["bun", "nx", "run-many", "--projects=@semio-tech/print", "--targets=build,build-viz", "--output-style=stream"], { cwd: root, env: { ...process.env, SEMIO_TICKET_DIR: ticket }, stdout: "pipe", stderr: "pipe" });
  const cancel = () => child.kill("SIGTERM"), drain = async (stream: ReadableStream<Uint8Array>) => { for await (const chunk of stream) appendFileSync(log, chunk); };
  process.on("SIGINT", cancel); process.on("SIGTERM", cancel);
  try {
    const [, , status] = await Promise.all([drain(child.stdout), drain(child.stderr), child.exited]);
    assert.equal(status, 0, `Print collection ${name} failed; inspect its ticket log`);
    return readFileSync(log, "utf8").replace(/\u001b\[[0-9;]*m/g, "");
  } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
}

await run("current");
const baseline = new Map<string, Record<string, string>>(directories.map((directory: { id: string; path: string }) => {
  assert.equal(JSON.parse(readFileSync(join(directory.path, ".nx-artifact.json"), "utf8")).owner, `@semio-tech/print:build-${directory.id}`);
  assert.equal(readdirSync(directory.path).filter(name => name.endsWith(".pdf")).length, 2);
  return [directory.id, snapshot(directory.path)];
}));
console.log(`[DEBUG] Print current-input collection: ${directories.length} owned pairs available PASS`);
const warm = await run("warm");
for (const directory of directories) {
  assert.ok(["local cache", "existing outputs match the cache, left as is"].some(label => warm.includes(`@semio-tech/print:build-${directory.id}  [${label}]`)), `${directory.id}: missing warm cache hit`);
  assert.deepEqual(snapshot(directory.path), baseline.get(directory.id));
}
console.log(`[DEBUG] Print identical second invocation: all ${directories.length} document leaves reused their cache entries PASS`);
mkdirSync(backup);
const moved: { id: string; path: string }[] = [];
try {
  for (const directory of directories) { renameSync(directory.path, join(backup, directory.id)); moved.push(directory); }
  const output = await run("restored");
  for (const directory of directories) {
    assert.ok(output.includes(`@semio-tech/print:build-${directory.id}  [local cache]`), `${directory.id}: missing clean-output cache hit`);
    assert.deepEqual(snapshot(directory.path), baseline.get(directory.id), `${directory.id}: restored bytes differ`);
  }
  rmSync(backup, { recursive: true });
  console.log(`[DEBUG] Print clean-output restoration: all ${directories.length * 2} PDFs and ${directories.length} ownership markers match PASS`);
} catch (error) {
  for (const directory of moved) if (!existsSync(directory.path)) renameSync(join(backup, directory.id), directory.path);
  throw error;
}
