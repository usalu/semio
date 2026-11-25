import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

if (process.argv[2] === "--inspect") {
  const bytes = await readFile(process.argv[3]);
  const module = await WebAssembly.compile(bytes);
  const entries = values => values.map(row => [row.module, row.name, row.kind].filter(Boolean).join("\0")).sort();
  console.log(JSON.stringify({
    imports: entries(WebAssembly.Module.imports(module)),
    exports: entries(WebAssembly.Module.exports(module)),
    bytes: bytes.byteLength,
    sha256: createHash("sha256").update(bytes).digest("hex"),
  }));
} else {
  const inspect = path => {
    const child = spawnSync(process.execPath, [fileURLToPath(import.meta.url), "--inspect", path], { encoding: "utf8" });
    if (child.status !== 0) throw new Error(child.error?.message ?? child.stderr);
    return JSON.parse(child.stdout);
  };
  const before = inspect(process.argv[2]);
  const after = inspect(process.argv[3]);
  assert.deepEqual(after.imports.filter(name => !before.imports.includes(name)), []);
  assert.deepEqual(after.exports, before.exports);
  console.log(JSON.stringify({imports:after.imports.length, unusedImportsRemoved:before.imports.filter(name => !after.imports.includes(name)), exports:before.exports.length, before:before.bytes, after:after.bytes, sha256:after.sha256}));
}
