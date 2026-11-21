import { expect, test } from "bun:test";
import Ajv from "ajv";
import iconv from "iconv-lite";
import { readFile } from "node:fs/promises";

const fixture = JSON.parse(await readFile("✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📊️channel-values/🧪️tests/🔣️.json", "utf8"));
test("UTF-8 channel preview fixture uses the portable contract", () => {
  const validate = new Ajv().compile({
    type: "object", required: ["pageBytes", "cases"], additionalProperties: false,
    properties: {
      pageBytes: { type: "integer", minimum: 4 },
      cases: { type: "array", items: {
        type: "object", required: ["name", "text", "pages"], additionalProperties: false,
        properties: { name: { type: "string" }, text: { type: "string" }, pages: { type: "array", items: { type: "string" } } },
      } },
    },
  });
  expect(validate(fixture)).toBe(true);
});
for (const row of fixture.cases) {
  test(row.name, () => {
    const encoded = iconv.encode(row.text, "UTF-8");
    const pages = [];
    let start = 0;
    for (let cut = fixture.pageBytes; start < encoded.length; cut += fixture.pageBytes) {
      let end = Math.min(cut, encoded.length);
      while (end < encoded.length && (encoded[end] & 0xc0) === 0x80) end -= 1;
      pages.push(iconv.decode(encoded.subarray(start, end), "UTF-8", { stripBOM: false }));
      start = end;
    }
    expect(pages).toEqual(row.pages);
    expect(iconv.encode(pages.join(""), "UTF-8")).toEqual(encoded);
  });
}
