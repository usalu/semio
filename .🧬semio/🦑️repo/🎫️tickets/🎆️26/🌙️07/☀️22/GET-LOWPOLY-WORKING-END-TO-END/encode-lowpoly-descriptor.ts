/** 📦 Re-encodes the checked-in descriptor through the owning pack API. */
import { readFileSync, writeFileSync } from "node:fs";
import { strict as assert } from "node:assert";
import { encodePackValue, decodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
const root = "🌎️hub/🧩️compositions/💠️lowpoly/🧩️component/";
const descriptor = JSON.parse(readFileSync(root + "🔣️.json", "utf8"));
const bytes = encodePackValue(descriptor);
assert.deepEqual(packValueToExactJson(decodePackValue(bytes)), descriptor);
writeFileSync(root + "🛂️.descriptor.semio", bytes);
console.log("[DEBUG] Descriptor encoded and roundtrip verified");
