import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const parseDataUrl = require("data-urls");
const { fromByteArray } = require("base64-js");
const { parse: parseJson } = require("jsonc-parser");

const root = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any";

const { cases } = JSON.parse(readFileSync(
  root + "/🚪️io/🔀️interchange/🧪️fixtures/🔣️.json", "utf8"
));
const image = cases.find(({ id }) => id === "scene-content");
const response = await fetch(image.imageDataUrl);
assert.equal(response.status, 200);
assert.equal(response.headers.get("content-type"), "image/png");
assert.deepEqual([...new Uint8Array(await response.arrayBuffer())], image.expected.imageBytes);
const independentImage = parseDataUrl(image.imageDataUrl);
assert.equal(independentImage.mimeType.essence, "image/png");
assert.deepEqual([...independentImage.body], image.expected.imageBytes);
assert.equal(fromByteArray(image.expected.imageBytes), image.imageDataUrl.split(",")[1]);

const fixture = readFileSync(
  root + "/🧪️tests/🧫️fixtures/🧬️mutations/🔀reorder-pages/🔀️moves/📸️snapshot/⬅️before/🔣️.json", "utf8"
);
const decimalGrid = fixture
  .replace(/("baselineGrid"\s*:\s*)12\b/, (_, key) => key + "12.0")
  .replace(/("baselineOffset"\s*:\s*)0\b/, (_, key) => key + "0.0");
assert.notEqual(decimalGrid, fixture);
assert.deepEqual(JSON.parse(decimalGrid), JSON.parse(fixture));
const fixtureErrors = [];
const decimalErrors = [];
const authored = parseJson(fixture, fixtureErrors, { disallowComments: true });
const decimals = parseJson(decimalGrid, decimalErrors, { disallowComments: true });
assert.deepEqual([...fixtureErrors, ...decimalErrors], []);
assert.deepEqual(decimals, authored);
console.log("WHATWG and data-urls agreed on image bytes; base64-js reproduced the payload; jsonc-parser matched the complete numeric fixture");
