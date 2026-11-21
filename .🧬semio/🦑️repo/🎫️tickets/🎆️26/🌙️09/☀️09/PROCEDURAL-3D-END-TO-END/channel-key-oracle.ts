import { expect, test } from "bun:test";
import Ajv from "ajv";
import iconv from "iconv-lite";
import { readFile } from "node:fs/promises";

const fixture = JSON.parse(await readFile("✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📊️channel-values/🧪️tests/📛️keys.json", "utf8"));
test("dictionary fixture uses the portable contract", () => {
  const validate = new Ajv().compile({
    type: "object", required: ["inlineBytes", "cases"], additionalProperties: false,
    properties: {
      inlineBytes: { type: "integer", minimum: 1 },
      cases: { type: "array", items: {
        type: "object", required: ["name", "key", "detail", "value"], additionalProperties: false,
        properties: { name: { type: "string" }, key: { type: "string" }, detail: { type: "boolean" }, value: {} },
      } },
    },
  });
  expect(validate(fixture)).toBe(true);
});

for (const row of fixture.cases) {
  test(row.name, () => {
    const encoded = iconv.encode(row.key, "UTF-8");
    const inline = encoded.length > 0 && encoded.length <= fixture.inlineBytes
      && row.key.trim() === row.key && !/ {2}|[^\p{L}\p{N}_.:$ -]/u.test(row.key);
    expect(!inline).toEqual(row.detail);
    expect(iconv.decode(encoded, "UTF-8", { stripBOM: false })).toBe(row.key);
    expect(JSON.parse(JSON.stringify(Object.fromEntries([[row.key, row.value]])))[row.key]).toEqual(row.value);
  });
}

test("two visually identical prefixes retain different dictionary entries", () => {
  const dictionary = Object.fromEntries(fixture.cases.map(row => [row.key, row.value]));
  const entries = fixture.cases.filter(row => row.name.startsWith("long identical prefix"));
  expect(entries[0].key.slice(0, -1)).toBe(entries[1].key.slice(0, -1));
  expect(entries.map(row => dictionary[row.key])).toEqual([{ shape: "A" }, { shape: "B" }]);
});
