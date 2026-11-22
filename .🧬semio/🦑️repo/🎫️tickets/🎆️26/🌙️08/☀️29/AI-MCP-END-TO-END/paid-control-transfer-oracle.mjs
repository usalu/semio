import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const fixture = JSON.parse(readFileSync(new URL("./paid-control-transfer-fixture.json", import.meta.url), "utf8"));

for (const { name, body, args, expected } of fixture.cases) {
  const bytes = [0, 97, 115, 109, 1, 0, 0, 0];
  const section = (id, payload) => bytes.push(id, payload.length, ...payload);
  section(1, [1, 96, 1, 127, 1, 127]);
  section(3, [1, 0]);
  section(7, [1, 3, 114, 117, 110, 0, 0]);
  section(10, [1, body.length, ...body]);
  const wasm = new WebAssembly.Instance(new WebAssembly.Module(Uint8Array.from(bytes)));
  assert.equal(wasm.exports.run(...args), expected, name);
}

console.log(`[DEBUG] Independent WebAssembly engine verified ${fixture.cases.length} structured control cases`);
