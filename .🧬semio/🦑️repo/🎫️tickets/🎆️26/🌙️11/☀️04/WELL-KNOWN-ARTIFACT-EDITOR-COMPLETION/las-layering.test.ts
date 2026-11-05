import { expect, it } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import Ajv from "ajv";
const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/";
const read = (path: string): string => readFileSync(root + path, "utf8");
it("keeps native LAS codec and read-only scenes below editor mutation ownership", () => {
  const native = "🚪️io/🗃️natural/🦀️.rs";
  const output = {
    snapshotOwnsEditorUi: /pub mod editor/.test(read("🧬️schema/📸️snapshot/📦️packages/🦀️rust/🦀️.rs")),
    schemaOwnsEditorUi: /pub mod editor/.test(read("🧬️schema/📦️packages/🦀️rust/🦀️.rs")),
    editorUsesUpperEngine: /crate::engine::/.test(read("✏️editor/🦀️.rs")),
    geometryUsesUpperEngine: ["✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs", "👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"].some(path => /crate::engine::/.test(read(path))),
    nativeFunctions: existsSync(root + native) ? [...read(native).matchAll(/pub fn (decode_las|encode_las)\(/g)].map(match => match[1]) : [],
  };
  const fixture = JSON.parse(read("🚪️io/🧪️tests/🧫️fixtures/🧱️layering.json"));
  const oracle = new Ajv().compile(fixture.schema);
  expect(oracle(output), JSON.stringify(oracle.errors)).toBe(true);
});
