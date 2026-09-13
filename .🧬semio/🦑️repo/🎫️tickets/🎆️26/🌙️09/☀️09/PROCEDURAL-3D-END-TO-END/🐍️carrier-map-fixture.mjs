#!/usr/bin/env node
/** 🧫️ Regenerates `🧬️contract/🧫️fixtures/🛍️catalogue-carrier-map.json`.
 *
 * The fixture pins ONE packed carrier leaf exactly as `semio_framework_plugin::app::section_text_chunks`
 * builds it for the app-static catalogue — `1 + UI_FIXED_LIST_ITEMS` slices of `UI_TEXT_MAX_BYTES`, the
 * first as `TextProps.value` and the rest as `data_attributes` keyed `"01".."32"` — together with the
 * key order a JavaScript engine actually emits for that object. The wgpu bridge's `renderDocument`
 * hands the guest document to `JSON.stringify`, and every ECMAScript engine emits array-index-like
 * keys ("10".."32") first in numeric order, then the remaining string keys ("01".."09"). */
import { writeFileSync } from "node:fs";

const UI_TEXT_MAX_BYTES = 512;
const UI_FIXED_LIST_ITEMS = 32;
const PACK = 1 + UI_FIXED_LIST_ITEMS;

function cataloguePayload(operators) {
  const records = [];
  for (let index = 0; index < operators; index += 1) {
    records.push(
      `{"kind":"brep.solid.op${index}","label":"Operator ${index}","module":"brep","inputs":[{"code":"P","abbreviation":"Pln","name":"plane","default":"{\\"origin\\":[0,0,0],\\"angle\\":1.5707963267948966}","cardinality":"!"}],"outputs":[{"code":"C","abbreviation":"Crv","name":"curve"}]}`,
    );
  }
  return `{"operators":[${records.join(",")}]}`;
}

const payloadSource = cataloguePayload(96);
const encoder = new TextEncoder();
const decoder = new TextDecoder();
const bytes = encoder.encode(payloadSource);
const slices = [];
let start = 0;
while (start < bytes.length && slices.length < PACK) {
  let end = Math.min(start + UI_TEXT_MAX_BYTES, bytes.length);
  while (end > start && (bytes[end] & 0xc0) === 0x80) end -= 1;
  slices.push(decoder.decode(bytes.subarray(start, end)));
  start = end;
}
if (slices.length !== PACK) throw new Error(`payload must fill one whole pack, got ${slices.length}`);

const ascending = {};
for (let offset = 1; offset < PACK; offset += 1) ascending[String(offset).padStart(2, "0")] = slices[offset];

const javascriptOrdered = JSON.parse(JSON.stringify(ascending));
const ascendingKeys = Object.keys(ascending).slice().sort();
const javascriptKeys = Object.keys(javascriptOrdered);
if (javascriptKeys.join() === ascendingKeys.join()) throw new Error("this engine did not reorder the object — the fixture would be vacuous");

const fixture = {
  note: "One packed text carrier leaf of the app-static catalogue, plus the key order JSON.stringify actually emits for its data_attributes object. Regenerate with <ticket>/🐍️carrier-map-fixture.mjs.",
  uiTextMaxBytes: UI_TEXT_MAX_BYTES,
  uiFixedListItems: UI_FIXED_LIST_ITEMS,
  pack: PACK,
  value: slices[0],
  ascendingKeys,
  javascriptKeys,
  dataAttributesJson: JSON.stringify(javascriptOrdered),
  payload: slices.join(""),
};

const out = new URL("../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🛍️catalogue-carrier-map.json", import.meta.url);
writeFileSync(out, `${JSON.stringify(fixture, null, 2)}\n`);
console.log(`[DEBUG] carrier-map fixture slices=${slices.length} payload-bytes=${encoder.encode(fixture.payload).length} javascript-first-key=${javascriptKeys[0]} last-key=${javascriptKeys[javascriptKeys.length - 1]} -> ${out.pathname}`);
