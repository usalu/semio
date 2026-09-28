/** 🔗️ H14 14b (ticket-local, one-off): re-derives the `🔗️compiled-dependencies` fixture's `rawCases` descriptor bytes for the
 * current app channel with the first-party Pack codec the oracle itself uses (`decodePackValue` / `encodePackValue`). The
 * canonical row is decoded, its `executionProtocol.appChannelVersion` set to `DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1`
 * and re-encoded; each hostile row keeps its own non-canonical layout and only swaps the encoded version value (the exact
 * bytes of the old value, which must occur once). Checks the oracle's own laws before writing: every row decodes to the new
 * canonical value, the canonical row round-trips, the hostile rows still do not. Usage: bun … [--write] */
import { readFileSync, writeFileSync } from "node:fs";
import { decodePackValue, encodePackValue, type PackValue } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";

const fixturePath = "/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json";
const text = readFileSync(fixturePath, "utf8");
const fixture = JSON.parse(text);
const hex = (bytes: Uint8Array) => Buffer.from(bytes).toString("hex");
const equal = (left: PackValue, right: PackValue) => hex(encodePackValue(left)) === hex(encodePackValue(right));
const canonical = decodePackValue(Buffer.from(fixture.rawCases[0].hex, "hex")) as any;
const previous = canonical.executionProtocol.appChannelVersion as PackValue;
const next = structuredClone(canonical);
next.executionProtocol.appChannelVersion = typeof previous === "number" ? DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 : (() => { throw new Error(`appChannelVersion is not a plain number: ${JSON.stringify(previous)}`); })();
const rootHeader = hex(encodePackValue(null)).slice(0, 8);
const valueBytes = (value: PackValue) => {
  const encoded = hex(encodePackValue(value));
  if (!encoded.startsWith(rootHeader)) throw new Error("a root Pack value lost its header");
  return encoded.slice(rootHeader.length);
};
const oldValue = valueBytes(previous);
const newValue = valueBytes(next.executionProtocol.appChannelVersion);
if (oldValue.length !== newValue.length) throw new Error("the version value changes its encoded length; hostile rows cannot keep their layout");
let written = text;
const rows = fixture.rawCases.map((row: any, index: number) => {
  const derived = index === 0 ? hex(encodePackValue(next)) : (() => {
    const occurrences = row.hex.split(oldValue).length - 1;
    if (occurrences !== 1) throw new Error(`${row.name}: the old version value occurs ${occurrences} times`);
    return row.hex.replace(oldValue, newValue);
  })();
  const bytes = Buffer.from(derived, "hex");
  if (!equal(decodePackValue(bytes), next)) throw new Error(`${row.name}: does not decode to the new canonical descriptor`);
  if ((hex(encodePackValue(decodePackValue(bytes))) === derived) !== row.accepted) throw new Error(`${row.name}: canonicality differs from accepted=${row.accepted}`);
  if (written.split(`"${row.hex}"`).length - 1 !== 1) throw new Error(`${row.name}: hex is not unique in the fixture text`);
  written = written.replace(`"${row.hex}"`, `"${derived}"`);
  return { name: row.name, changed: row.hex !== derived };
});
console.log(JSON.stringify({ previous, next: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, oldValue, newValue, rows }));
if (process.argv.includes("--write")) writeFileSync(fixturePath, written);
