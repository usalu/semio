/** 🧪️ Named native JSON admission and semantic validation agree with selection leaf schemas. */
import { expect, test } from "bun:test";
import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { layoutMutationFromNativeJson } from "../../🪪️native-json/🟦️.ts";
import { evaluateLayoutMutation } from "../../⚙️evaluate/🟦️.ts";
import { layoutArtifactFromNativeJson } from "../../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts";

const here = fileURLToPath(new URL(".", import.meta.url));
const mutations = join(here, "../..");
const fixtures = join(here, "../../../../🧪️tests/🧫️fixtures/🧬️mutations");
const leaves = [
  { directory: "✋️drag-frames", variant: "DragFrames" },
  { directory: "🔃️rotate-frames", variant: "RotateFrames" },
  { directory: "🗜️scale-frames", variant: "ScaleFrames" },
] as const;

for (const leaf of leaves) {
  test(`${leaf.directory}: the twin and Ajv agree on every committed payload`, () => {
    const schema = JSON.parse(readFileSync(join(mutations, leaf.directory, "🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
    const cases = readdirSync(join(fixtures, leaf.directory));
    expect(cases.length).toBeGreaterThanOrEqual(5);
    let rejected = 0;
    for (const folder of cases) {
      const wire = JSON.parse(readFileSync(join(fixtures, leaf.directory, folder, "🦠️mutation/🔣️.json"), "utf8"));
      const payload = wire[leaf.variant];
      const outcome = JSON.parse(readFileSync(join(fixtures, leaf.directory, folder, "🎯️outcome/🔣️.json"), "utf8"));
      const invariant = outcome.code === "mutation.invariant";
      expect(Object.keys(wire), folder).toEqual([leaf.variant]);
      expect(validate(payload), `${folder}: Ajv`).toBe(!invariant);
      if (invariant) {
        rejected += 1;
        const base = layoutArtifactFromNativeJson(JSON.parse(readFileSync(join(fixtures, leaf.directory, folder, "📸️snapshot/⬅️before/🔣️.json"), "utf8")));
        expect(evaluateLayoutMutation(base, layoutMutationFromNativeJson(wire)).messages.some(message => message.code === "mutation.invariant"), folder).toBe(true);
        expect(existsSync(join(fixtures, leaf.directory, folder, "🔺️diff/🚫️.absent")), folder).toBe(true);
      } else {
        expect(Object.keys(layoutMutationFromNativeJson(wire)), folder).toEqual([leaf.variant]);
      }
    }
    expect(rejected, "one negative witness per leaf").toBe(1);
  });
}

test("the named native JSON boundary refuses malformed scalar fields", () => {
  expect(() => layoutMutationFromNativeJson({ DragFrames: { pageId: "page-1", targets: ["a"], dx: 1 } })).toThrow();
  expect(() => layoutMutationFromNativeJson({ DragFrames: { pageId: "page-1", targets: ["a"], dx: 1, dy: 1, dz: 0 } })).toThrow();
  expect(() => layoutMutationFromNativeJson({ RotateFrames: { pageId: "page-1", targets: ["a"], pivotX: 0, pivotY: Number.NaN, angle: 1 } })).toThrow();
});
