import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { unit } from "mathjs";

const { nativeUnit, accepted } = JSON.parse(readFileSync(process.argv[2], "utf8"));
for (const entry of accepted) {
  const numeric = Number(entry.text);
  const quantity = Number.isFinite(numeric) ? unit(numeric, nativeUnit) : unit(entry.text);
  assert.equal(quantity.toNumber(nativeUnit), entry.value);
  assert.equal(unit(entry.canonical).toNumber(nativeUnit), entry.value);
}
console.log(`[DEBUG] Math.js verified ${accepted.length} compound-quantity cases`);
