// 🔢️ LB2 p20 pin oracle: every hub/stdio count pin p20 moves equals the count the tree actually has — stdio linked receipts, GIS and
// VCS receipts, the provider set, the stdio+GIS bootstrap closure — and the surface fixture validates against its owning schema.
// usage: node lb2-p20-pin-check.mjs <root>
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const root = process.argv[2];
const read = (path) => JSON.parse(readFileSync(`${root}/${path}`, "utf8"));
const stdio = read("✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json").receipts.length;
const gis = read("✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🔣️.json").receipts.length;
const vcs = read("✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🔣️.json").receipts.length;
const frontier = read("🌎️hub/🧫️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json").production.receiptCount;
const provider = read("🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🪪️v1/🔣️.json").receiptCount;
const bootstrap = read("🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json");
const rust = readFileSync(`${root}/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs`, "utf8");
const pin = (name) => Number(rust.match(new RegExp(`${name}: usize = (\\d+);`))[1]);
const script = readFileSync(`${root}/🌎️hub/📦️packages/🦀️rust/📜️script.ts`, "utf8");
const checks = [
  ["frontier fixture = stdio + gis + vcs", frontier, stdio + gis + vcs],
  ["provider set pin = stdio + gis + vcs", pin("NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS"), stdio + gis + vcs],
  ["stdio provider pin", pin("NATIVE_STDIO_PROVIDER_RECEIPTS"), stdio],
  ["provider fixture", provider, stdio],
  ["bootstrap stdio package", bootstrap.profile.packages[1].codecCount, stdio],
  ["bootstrap limit = stdio + gis", bootstrap.limits.codecCount, stdio + gis],
  ["publisher frontier literal", Number(script.match(/receiptCount\) !== (\d+)/)[1]), stdio + gis + vcs],
  ["publisher bootstrap literal", Number(script.match(/codecs\.stdio\.length !== (\d+)/)[1]), stdio],
];
let failures = 0;
for (const [name, actual, expected] of checks) {
  console.log(`${actual === expected ? "ok  " : "FAIL"} ${name}: ${actual} vs ${expected}`);
  failures += actual === expected ? 0 : 1;
}
const Ajv = require("ajv").default;
const schema = read("✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🔣️.json");
const find = (node) => {
  if (!node || typeof node !== "object") return null;
  if (node.properties?.schema?.const === "semio.stdio.native-catalog-surface/v1") return node;
  for (const value of Object.values(node)) {
    const found = find(value);
    if (found) return found;
  }
  return null;
};
const surfaceSchema = find(schema);
const surface = read("✏️s/🔌️plugins/🗄️stdio/📇️registry/🧫️fixtures/📇️native-catalog-surface/🔣️.json");
const ajv = new Ajv({ strict: false, allErrors: true });
const valid = surfaceSchema ? ajv.validate(surfaceSchema, surface) : false;
console.log(`${valid ? "ok  " : "FAIL"} surface fixture validates against its owning schema ${valid ? "" : JSON.stringify(ajv.errors)}`);
failures += valid ? 0 : 1;
console.log(`PIN-CHECK failures=${failures}`);
process.exit(failures ? 1 : 0);
