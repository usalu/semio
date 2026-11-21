import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { join } from "node:path";

const root = process.cwd();
const owner = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance");
const manifestDirectory = join(owner, "📦️packages/🦀️rust");
const manifest = Bun.TOML.parse(await readFile(join(manifestDirectory, "Cargo.toml"), "utf8"));
const limits = manifest.package.metadata.semio["rust-source-boundary"];
const { createRustSourceAnalysisV1, readRustSourceClosureV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const targets = {
  "wasm32-unknown-unknown": ['target_arch=wasm32', 'target_os=unknown', 'target_env=', 'target_family=wasm', 'target_vendor=unknown', 'target_pointer_width=32', 'target_endian=little'],
  "wasm32-wasip2": ['target_arch=wasm32', 'target_os=wasi', 'target_env=p2', 'target_family=wasm', 'target_vendor=unknown', 'target_pointer_width=32', 'target_endian=little'],
};
const analyzed = [];
const readSource = path => readFile(path).catch(error => { if (error.code === "ENOENT") return null; throw error; });
for (const [target, cfg] of Object.entries(targets)) {
  const files = await readRustSourceClosureV1([join(owner, "🦀️.rs")], root, cfg, readSource, { manifestDirectory, generated: () => undefined }, createRustSourceAnalysisV1());
  const counts = { files: files.length, bytes: files.reduce((n, file) => n + file.bytes, 0), lines: files.reduce((n, file) => n + file.activeLines, 0) };
  analyzed.push({ target, ...counts });
  console.log(target, counts);
}
assert.deepEqual(limits, { maxSourceBytes: 262144, maxSourceFiles: 32, maxActiveLines: 4000 });
for (const { target, files, bytes, lines } of analyzed) {
  assert.ok(files <= limits.maxSourceFiles, `${target} source files exceed the cap`);
  assert.ok(bytes <= limits.maxSourceBytes, `${target} source bytes exceed the cap`);
  assert.ok(lines <= limits.maxActiveLines, `${target} active lines exceed the cap`);
}
