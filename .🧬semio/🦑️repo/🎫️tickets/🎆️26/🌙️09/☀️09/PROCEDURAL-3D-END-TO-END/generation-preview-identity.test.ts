import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parseGeneration3dPreviewWindowTransient } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🚪️io/📝️text/🟦️.ts";

const schema = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🧬️schema/🔣️.json").json();
const log = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🧪️tests/🧫️fixtures/generation-preview-identity.json").json();
const validate = new Ajv({ strict: false }).compile(schema);
for (const row of log.cases) {
  test(row.name, () => {
    expect(validate(row.stored)).toBe(true);
    const data = parseGeneration3dPreviewWindowTransient(row.stored);
    const id = data.generationId ?? null;
    const stores = new Map([[id, data.previewEvalText ?? null]]);
    const generation = row.mode === "generate";
    const project = generation && row.selected === null ? null : stores.get(row.selected) ?? null;
    expect(project).toBe(row.visible);
    expect(generation ? row.selected === null ? null : row.live : row.painted).toBe(row.publication);
  });
}
test("generation identity is text and window authority stays outside the payload", () => {
  for (const record of [{generationId: 1}, {windowId: "outside"}, {generationId: []}]) {
    expect(validate(record)).toBe(false);
    expect(() => parseGeneration3dPreviewWindowTransient(record)).toThrow();
  }
});
