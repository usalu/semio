import { createRequire } from "node:module";
import { resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";

const require = createRequire(resolve("package.json"));
const patch = require("fast-json-patch");
const root = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧵️patch-page";
const { checkPageArtboard } = await import(resolve(root, "🖼️drawing/📐️artboard/🧪️tests.ts"));
console.log(JSON.stringify(checkPageArtboard({
  patch: (document, operations) => patch.applyPatch(structuredClone(document), operations).newDocument,
  equal: isDeepStrictEqual,
})));
