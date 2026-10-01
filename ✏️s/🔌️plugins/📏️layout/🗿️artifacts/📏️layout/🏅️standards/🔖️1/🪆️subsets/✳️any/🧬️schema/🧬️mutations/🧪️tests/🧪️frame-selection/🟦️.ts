/** 🧪️ Conformance of the layout frame-selection leaves' TS twin (`parseDragFrames`/`parseRotateFrames`/`parseScaleFrames`):
 * every committed fixture payload of `drag-frames`, `rotate-frames` and `scale-frames` is parsed by the twin and validated by
 * the third-party Ajv against its leaf schema — the twin accepts exactly what the schema accepts (the `mutation.invariant`
 * vectors are negative witnesses both reject), and an accepted payload parses to itself. */
import { expect, test } from "bun:test";
import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { parseDragFrames, parseRotateFrames, parseScaleFrames } from "../../🟦️.ts";

const here = fileURLToPath(new URL(".", import.meta.url));
const mutations = join(here, "../..");
const fixtures = join(here, "../../../../🧫️fixtures/🧬️mutations");
const leaves = [
  { directory: "✋️drag-frames", variant: "DragFrames", parse: parseDragFrames },
  { directory: "🔃️rotate-frames", variant: "RotateFrames", parse: parseRotateFrames },
  { directory: "🗜️scale-frames", variant: "ScaleFrames", parse: parseScaleFrames },
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
        expect(() => leaf.parse(payload), folder).toThrow();
        expect(existsSync(join(fixtures, leaf.directory, folder, "🔺️diff/🚫️.absent")), folder).toBe(true);
      } else {
        expect(leaf.parse(payload), folder).toEqual(payload);
      }
    }
    expect(rejected, "one negative witness per leaf").toBe(1);
  });
}

test("the twin refuses what every leaf schema forbids", () => {
  expect(() => parseDragFrames({ pageId: "page-1", targets: [], dx: 1, dy: 1 })).toThrow();
  expect(() => parseDragFrames({ pageId: "page-1", targets: ["a"], dx: 1 })).toThrow();
  expect(() => parseDragFrames({ pageId: "page-1", targets: ["a"], dx: 1, dy: 1, dz: 0 })).toThrow();
  expect(() => parseRotateFrames({ pageId: "page-1", targets: ["a"], pivotX: 0, pivotY: Number.NaN, angle: 1 })).toThrow();
  expect(() => parseScaleFrames({ pageId: "page-1", targets: ["a"], pivotX: 0, pivotY: 0, sx: 2, sy: -1 })).toThrow();
});
