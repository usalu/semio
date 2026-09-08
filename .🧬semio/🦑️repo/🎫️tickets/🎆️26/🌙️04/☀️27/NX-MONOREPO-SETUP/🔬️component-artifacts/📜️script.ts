import assert from "node:assert/strict";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
if (process.argv[2] === "inspect") {
  const { componentWit } = await import("@bytecodealliance/jco");
  const wit = await componentWit(readFileSync(process.argv[3]));
  for (const name of ["reactor", "checkpoint", "describe", "jobs"]) assert.ok(wit.includes("export semio:framework/" + name), "Missing component interface " + name);
  writeFileSync(process.argv[4], wit);
  process.exit(0);
}
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const evidence = join(ticket, "🗑️generated", `component-restore-${Date.now()}`);
mkdirSync(evidence, { recursive: true });
const output = join(root, "✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/dist/component-dev");
const snapshot = (): unknown => Object.fromEntries(readdirSync(output).sort().map((path) => [path, { hash: createHash("sha256").update(readFileSync(join(output, path))).digest("hex"), mode: lstatSync(join(output, path)).mode & 0o777 }]));
const run = async (name: string): Promise<string> => {
  const child = Bun.spawn(["bun", "nx", "run", "@semio-tech/note-plugin:component-dev", "--output-style=stream"], { cwd: root, env: { ...process.env, npm_lifecycle_event: "", npm_lifecycle_script: "" }, stdout: "pipe", stderr: "pipe" });
  const [out, err, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(evidence, name + ".log"), out + err);
  assert.equal(code, 0, out + err); return out;
};
await run("warm");
const before = snapshot(), backup = join(evidence, "backup");
renameSync(output, backup);
try {
  const restored = await run("restore");
  assert.match(restored, /local cache|existing outputs match the cache/);
  assert.deepEqual(snapshot(), before);
} finally { if (!existsSync(output)) renameSync(backup, output); }
const wasm = readFileSync(join(output, "semio_s_plugin_note.wasm"));
assert.deepEqual([...wasm.subarray(0, 8)], [0, 97, 115, 109, 13, 0, 1, 0]);
const oracle = Bun.spawn(["node", fileURLToPath(import.meta.url), "inspect", join(output, "semio_s_plugin_note.wasm"), join(evidence, "restored.wit")], { cwd: root, stdout: "pipe", stderr: "pipe" });
const [stdout, stderr, code] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
writeFileSync(join(evidence, "oracle.log"), stdout + stderr);
assert.equal(code, 0, stdout + stderr);
writeFileSync(join(ticket, "📓️component-artifacts.md"), "# Plugin WASI Component Restoration\n\nNx restored the deleted staged component directory with identical file hashes and modes. The independent installed Bytecode Alliance component tooling read the restored binary and extracted its reactor, checkpoint, describe and jobs interfaces. This interface proof does not claim native actor execution; materialization and browser execution are verified separately.\n\n" + JSON.stringify(before, null, 2) + "\n");
console.log("[DEBUG] Note component Nx restore and independent WIT consumer: PASS");
