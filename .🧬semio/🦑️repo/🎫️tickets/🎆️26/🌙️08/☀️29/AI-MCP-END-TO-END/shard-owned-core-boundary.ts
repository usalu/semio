import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import fixture from "./shard-owned-core-boundary.json" with { type: "json" };

const coreMagic = "0061736d01000000";
const componentMagic = "0061736d0d000100";

function embeddedCores(bytes: Buffer): Buffer[] {
  const header = bytes.subarray(0, 8).toString("hex");
  if (header === coreMagic) return [bytes];
  assert.equal(header, componentMagic);
  const cores: Buffer[] = [];
  let offset = 8;
  while (offset < bytes.length) {
    const id = bytes.readUInt8(offset++);
    let length = 0;
    let shift = 0;
    for (;;) {
      assert(shift <= 28);
      const byte = bytes.readUInt8(offset++);
      length += (byte & 0x7f) * 2 ** shift;
      if (byte < 0x80) break;
      shift += 7;
    }
    const end = offset + length;
    assert(end <= bytes.length);
    if (id === 1 || id === 4) cores.push(...embeddedCores(bytes.subarray(offset, end)));
    offset = end;
  }
  return cores;
}

const path = process.argv[2];
assert(path, "Supply a built owned component or Core module");
const component = readFileSync(path);
const firstExport = Buffer.from(fixture.exports[0].name);
const candidates = embeddedCores(component)
  .filter(bytes => bytes.includes(firstExport))
  .map(bytes => new WebAssembly.Module(bytes))
  .filter(module => WebAssembly.Module.exports(module).some(entry => entry.name === fixture.exports[0].name));
assert.equal(candidates.length, 1, "Expected exactly one owned actor Core");
const module = candidates[0];
const imports: WebAssembly.Imports = Object.create(null);
for (const entry of WebAssembly.Module.imports(module)) {
  assert.equal(entry.kind, "function");
  const namespace = imports[entry.module] ??= Object.create(null);
  namespace[entry.name] = () => {
    assert.fail(`Malformed authority reached guest work through ${entry.module}/${entry.name}`);
  };
}
const instance = new WebAssembly.Instance(module, imports);
for (const row of fixture.exports) {
  const callable = instance.exports[row.name];
  assert.equal(typeof callable, "function", row.name);
  if (typeof callable !== "function") assert.fail(row.name);
  assert.equal(callable.length, row.authorityWords + (row.bytePair ? 2 : 0), row.name);
  const words = Array<bigint>(row.authorityWords).fill(0n);
  const inputs = row.bytePair ? [0, 0, ...words] : words;
  for (const index of words.keys()) {
    const invalid = [...inputs];
    invalid[index + (row.bytePair ? 2 : 0)] = 0;
    assert.throws(() => callable(...invalid), TypeError, `${row.name} authority word ${index}`);
  }
  assert.throws(() => callable(0, 0), TypeError, `${row.name} obsolete call`);
  const refusal = callable(...inputs);
  assert.equal(typeof refusal, "bigint", row.name);
  assert.notEqual(refusal, 0n, `${row.name} zero-authority refusal`);
}
console.log("[DEBUG] Verified owned Core signatures and zero-authority refusal",
  createHash("sha256").update(component).digest("hex"), fixture.exports.length);
