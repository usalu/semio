import { cp, mkdir, mkdtemp, readFile, rename, rm, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";

const family = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm");
const laws = {
  host: ["🖥️host", "testPublishedFlowHost"],
  clock: ["⏱️consumed-browser-clock", "testFlowBrowserClock"],
};
if (process.argv[2] === "--law") {
  const [directory, name] = laws[process.argv[3]];
  const module = await import(join(family, "🧪️tests", directory, "🟨️.js"));
  await module[name](resolve(process.argv[4]));
  process.exit(0);
}
const [source, destination] = process.argv.slice(2).map(value => resolve(value));
for (const name of Object.keys(laws)) {
  const result = spawnSync(process.execPath, [import.meta.path, "--law", name, source], {stdio:"inherit", timeout:120000});
  if (result.status !== 0) throw new Error("Compiled Flow law failed: " + name, {cause:result.error});
}
console.log("Compiled Flow host, clock and scene synchronization passed");
const marker = ".semio-artifact.json";
const oldMarker = await readFile(join(destination, marker), "utf8");
const expected = JSON.parse(oldMarker);
const manifest = JSON.parse(await readFile(join(source, "package.json"), "utf8"));
assert.equal(expected.owner, manifest.semioWasmOwner.sourceOwner + "/Cargo.toml:flow-browser");
assert.deepEqual(expected.files, [...manifest.files,"package.json"].sort());
const digest = bytes => createHash("sha256").update(bytes).digest("hex");
async function fileHashes(directory) {
  const result = [];
  for (const file of expected.files) result.push(digest(await readFile(join(directory, file))));
  return result;
}
const current = await fileHashes(destination);
const compiled = await fileHashes(source);
if (compiled.some((hash, index) => hash !== current[index])) {
  const transaction = await mkdtemp(join(dirname(destination), ".flow-publication-"));
  const staging = join(transaction, "verified");
  const backup = join(transaction, "previous");
  await mkdir(staging);
  for (const file of expected.files) {
    await mkdir(dirname(join(staging, file)), {recursive:true});
    await cp(join(source, file), join(staging, file));
  }
  await writeFile(join(staging, marker), oldMarker);
  assert.deepEqual(await fileHashes(staging), compiled);
  assert.deepEqual(await fileHashes(destination), current);
  await rename(destination, backup);
  try { await rename(staging, destination); } catch (error) { await rename(backup, destination); throw error; }
  await rm(transaction, {recursive:true});
}
console.log("Served Flow SHA-256", digest(await readFile(join(destination, "flow_core_bg.wasm"))));
