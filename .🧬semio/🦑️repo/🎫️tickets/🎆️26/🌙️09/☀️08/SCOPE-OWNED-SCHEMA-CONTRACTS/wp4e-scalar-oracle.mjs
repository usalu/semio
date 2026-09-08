/** 🔢️ WP4e — every fixture whose schema now addresses `framework.value`'s scalar exports still
 * validates, and the narrowing still rejects what it rejected before.
 *
 * Rows 149 + 151: `U64`/`NonZeroU64` moved to `framework.value`; the eight documents below reference
 * them across scopes. ajv is the independent oracle (the repo's own validator is not used here on
 * purpose). Mirrors the wiring each owning vitest case uses, so a green run here means the same
 * `addSchema` set resolves.
 */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const Ajv = require("ajv");
const read = path => JSON.parse(readFileSync(path, "utf8"));

const M = "🧰️framework/🔨️modules";
const value = read(`${M}/🌱️value/🧬️schema/🔣️.json`);
const page = read(`${M}/🎭️actor/📃️page/🧬️schema/🔣️.json`);
const lifetime = read(`${M}/🎭️actor/🚪️lifetime/🧬️schema/🔣️.json`);
const patch = read(`${M}/🎭️actor/🚪️lifetime/🩹️patch/🧬️schema/🔣️.json`);
const returned = read(`${M}/🎭️actor/📤️return/🧬️schema/🔣️.json`);
const response = read(`${M}/🎭️actor/📤️return/📨️response/🧬️schema/🔣️.json`);
const credit = read(`${M}/🎭️actor/📤️return/📨️response/🎟️credit/🧬️schema/🔣️.json`);
const output = read(`${M}/🎭️actor/🪪️activation/🚪️instance/📥️output/🧬️schema/🔣️.json`);
const admission = read(`${M}/🖱️ui/🖥️host/📥️input/🎟️admission/🧬️schema/🔣️.json`);
const root = read(`${M}/🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🧬️schema/🔣️.json`);

const ajv = new Ajv({ strict: true, allErrors: true });
for (const document of [value, page, lifetime, patch, returned, response, credit, output, admission, root]) ajv.addSchema(document);

const positives = [
  [lifetime, "LifetimeFixture", `${M}/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json`],
  [patch, "PatchFixture", `${M}/🎭️actor/🚪️lifetime/🩹️patch/🧫️fixture/🔣️.json`],
  [returned, "ReturnFixture", `${M}/🎭️actor/📤️return/🧫️fixture/🔣️.json`],
  [response, "ResponseFixture", `${M}/🎭️actor/📤️return/📨️response/🧪️fixture/🔣️.json`],
  [credit, "CreditFixture", `${M}/🎭️actor/📤️return/📨️response/🎟️credit/🧪️fixture/🔣️.json`],
  [admission, "AdmissionFixture", `${M}/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🔣️.json`],
  [root, "RootFixture", `${M}/🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🧪️tests/🔣️.json`],
];

let failures = 0;
for (const [document, exported, fixture] of positives) {
  const validate = ajv.getSchema(`${document.$id}#/$defs/${exported}`);
  if (!validate) { console.log(`  UNRESOLVED ${document.$id}#/$defs/${exported}`); failures++; continue; }
  const ok = validate(read(fixture));
  console.log(`${ok ? "pass" : "FAIL"} ${exported} ← ${fixture}${ok ? "" : `\n    ${JSON.stringify(validate.errors)}`}`);
  if (!ok) failures++;
}

// The narrowing: `framework.value#/$defs/NonZeroU64` must still reject exactly what the actor family
// rejected while it carried its own copy (🚪️lifetime/🧪️tests/…/🟦️.ts asserts these four).
const u64 = ajv.getSchema(`${value.$id}#/$defs/U64`);
const nonZero = ajv.getSchema(`${value.$id}#/$defs/NonZeroU64`);
const cases = [
  ["0", true, false],
  ["-1", false, false],
  ["01", false, false],
  ["18446744073709551616", false, false],
  ["1", true, true],
  ["18446744073709551615", true, true],
];
for (const [sample, expectedU64, expectedNonZero] of cases) {
  const gotU64 = u64(sample), gotNonZero = nonZero(sample);
  const ok = gotU64 === expectedU64 && gotNonZero === expectedNonZero;
  console.log(`${ok ? "pass" : "FAIL"} "${sample}" U64=${gotU64} NonZeroU64=${gotNonZero} (expected ${expectedU64}/${expectedNonZero})`);
  if (!ok) failures++;
}

console.log(`checks=${positives.length + cases.length} failures=${failures}`);
process.exit(failures === 0 ? 0 : 1);
