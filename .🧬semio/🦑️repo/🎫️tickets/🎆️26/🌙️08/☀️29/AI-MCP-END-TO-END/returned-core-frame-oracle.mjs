import { readFileSync } from "node:fs";
import assert from "node:assert/strict";

const fixture = JSON.parse(readFileSync(new URL("./returned-core-frame-fixture.json", import.meta.url), "utf8"));
const bytes = Uint8Array.from(fixture.moduleHex.match(/../g), byte => Number.parseInt(byte, 16));
const wasm = new WebAssembly.Instance(new WebAssembly.Module(bytes));
assert.equal(wasm.exports[fixture.exportName](), fixture.expectedResult);
assert.equal(wasm.exports[fixture.exportName](), fixture.expectedResult);

const calls = fixture.callWaves.reduce((total, wave) => total + wave.outerCalls * (1 + wave.childCallsPerOuter), 0);
const instructions = 6 + fixture.callWaves.reduce((total, wave) =>
  total + wave.outerCalls * (12 + 5 * wave.childCallsPerOuter), 0);
assert.equal(calls, fixture.expectedNestedBackingBirths.withoutReuse);
assert.equal(instructions, fixture.expectedCoreInstructions);
const available = new Set();
let births = 0;
function call(depth, children) {
  if (!available.delete(depth)) births++;
  for (let child = 0; child < children; child++) call(depth + 1, 0);
  available.add(depth);
}
for (const wave of fixture.callWaves) {
  for (let outer = 0; outer < wave.outerCalls; outer++) call(2, wave.childCallsPerOuter);
}
assert.equal(births, fixture.expectedNestedBackingBirths.withDepthReuse);
console.log(`WebAssembly result 34 twice, ${calls} authored nested calls, ${instructions} authored Core instructions, ${births} modeled depth-slot births`);
