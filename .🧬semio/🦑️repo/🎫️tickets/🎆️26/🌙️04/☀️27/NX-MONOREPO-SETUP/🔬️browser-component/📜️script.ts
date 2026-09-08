import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
if (process.argv[2] === "inspect") {
  const component = await import(pathToFileURL(process.argv[3]).href);
  for (const name of ["reactor", "checkpoint", "describe", "jobs"]) assert.ok(component[name]);
  const descriptor = await component.describe.describe();
  assert.ok(descriptor instanceof Uint8Array && descriptor.length > 0);
  writeFileSync(process.argv[4], descriptor);
  const { componentWit } = await import("@bytecodealliance/jco");
  const wit = await componentWit(readFileSync(process.argv[5]));
  for (const name of ["reactor", "checkpoint", "describe", "jobs"]) assert.ok(wit.includes("export semio:framework/" + name));
  writeFileSync(process.argv[6], wit);
  console.log("[DEBUG] Restored Note component executed describe in Node and passed independent WIT extraction");
  process.exit(0);
}
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const evidence = join(ticket, "🗑️generated", "browser-component-restoration-" + Date.now());
mkdirSync(evidence, { recursive: true });
const outputs = [join(root, "✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/dist/component-dev"), join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🗒️note")];
const snapshot = (): Record<string, unknown> => {
  const result: Record<string, unknown> = {};
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory()) walk(path);
      else result[relative(root, path)] = { sha256: createHash("sha256").update(readFileSync(path)).digest("hex"), mode: lstatSync(path).mode & 0o777 };
    }
  };
  outputs.forEach(walk);
  return result;
};
const run = async (label: string): Promise<string> => {
  const child = Bun.spawn(["bun", "nx", "run", "@semio-tech/note-plugin:materialize-dev", "--output-style=stream"], { cwd: root, stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(evidence, label + ".log"), stdout + stderr);
  assert.equal(code, 0, stdout + stderr);
  return stdout;
};
await run("warm");
const before = snapshot();
for (const [index, path] of outputs.entries()) renameSync(path, join(evidence, "backup-" + index));
try {
  const log = await run("restore");
  for (const target of ["component-dev", "materialize-dev"]) assert.match(log, new RegExp("note-plugin:" + target + " .*local cache"));
  assert.deepEqual(snapshot(), before);
} finally { for (const [index, path] of outputs.entries()) if (!existsSync(path)) renameSync(join(evidence, "backup-" + index), path); }
const oracle = Bun.spawn(["node", "--experimental-wasm-jspi", fileURLToPath(import.meta.url), "inspect", join(outputs[1], "semio_s_plugin_note_component.js"), join(evidence, "descriptor.semio"), join(outputs[0], "semio_s_plugin_note.wasm"), join(evidence, "component.wit")], { cwd: root, stdout: "pipe", stderr: "pipe" });
const [stdout, stderr, code] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
writeFileSync(join(evidence, "oracle.log"), stdout + stderr);
assert.equal(code, 0, stdout + stderr);
const { decodePackValue } = await import("@semio-tech/framework-os");
assert.equal((decodePackValue(readFileSync(join(evidence, "descriptor.semio"))) as any).manifest.pluginId, "note");
assert.equal(JSON.parse(readFileSync(join(outputs[1], "🔣️.json"), "utf8")).manifest.pluginId, "note");
writeFileSync(join(ticket, "📓️browser-component-artifacts.md"), "# Browser Component Restoration\n\nNx restored the deleted staged WASI component and browser module directory with identical hashes and modes. Node independently imported the restored transpiled module and executed its describe export. The returned native descriptor identifies Note; the installed Bytecode Alliance tooling also extracted all four actor interfaces from the restored WASI binary. Browser document/session interaction remains a separate development integration check.\n\n" + Object.keys(before).length + " files verified.\n");
console.log("[DEBUG] Note component and materializer Nx restoration plus Node/WIT consumers: PASS");
