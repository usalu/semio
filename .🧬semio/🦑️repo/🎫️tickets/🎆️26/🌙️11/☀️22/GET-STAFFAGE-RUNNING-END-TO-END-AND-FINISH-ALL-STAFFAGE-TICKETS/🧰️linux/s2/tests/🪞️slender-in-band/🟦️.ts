/** 🪞️ Slender in-band: the derived-view check measures two views of one band relative to their box, so a column or a beam is judged across its width as finely as along its length.
 *
 * THE REFERENCE is shapely (🐍️.py): `affinity.scale` to the box and `hausdorff_distance` of the point sets under the eight transforms.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🪞️slender-in-band/🔣️.json — the vectors
 * @see ../🔭️multiview-coverage/🪞️derived.ts — `relativeHausdorff`, `derivedFlags`
 */
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { EvaluatedShape } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { NEAR_MATCH, bestTransform, derivedFlags, pointSet } from "../🔭️multiview-coverage/🪞️derived.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Rings = [number, number][][];
type Vectors = { nearMatch: number; toleranceDistance: number; cases: { name: string; elevation: Rings; other: Rings; flagged: boolean }[] };
const vectors = fixture<Vectors>("🪞️slender-in-band");
const shapesOf = (rings: Rings): EvaluatedShape[] => rings.map((ring, i) => ({ shape: { id: `s${i}`, role: "body" } as never, rings: [ring.map(([x, y]) => ({ x, y }))], closed: true }));

describe("🪞️ two views of one band relative to their box", () => {
  requireTool("uv");
  const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🪞️slender-in-band/🐍️.py"), "geometry", [join(STAFFAGE_ROOT, "🧫️fixtures/🪞️slender-in-band/🔣️.json")])) as { relative: number; plain: number }[];

  it("uses the threshold of the vectors", () => {
    expect(NEAR_MATCH).toBe(vectors.nearMatch);
  });

  vectors.cases.forEach((c, i) =>
    it(`measures the shapely distance and ${c.flagged ? "flags" : "keeps"}: ${c.name}`, () => {
      const [a, b] = [pointSet(shapesOf(c.elevation))!, pointSet(shapesOf(c.other))!];
      expect(Math.abs(bestTransform(b, a, true).distance - reference[i]!.relative), "relative").toBeLessThanOrEqual(vectors.toleranceDistance);
      expect(Math.abs(bestTransform(b, a).distance - reference[i]!.plain), "plain").toBeLessThanOrEqual(vectors.toleranceDistance);
      expect(reference[i]!.relative < vectors.nearMatch, "shapely verdict").toBe(c.flagged);
      const flags = derivedFlags({ elevation: shapesOf(c.elevation), front: shapesOf(c.other), plan: [] });
      expect(flags.some((flag) => flag.view === "front" && flag.against === "elevation"), "flag").toBe(c.flagged);
    }),
  );

  it("is never below the plain distance and zero only where that one is zero", () => {
    for (const c of vectors.cases) {
      const [a, b] = [pointSet(shapesOf(c.elevation))!, pointSet(shapesOf(c.other))!];
      const [relative, plain] = [bestTransform(b, a, true).distance, bestTransform(b, a).distance];
      expect(relative, c.name).toBeGreaterThanOrEqual(plain - 1e-12);
      expect(relative < 1e-12, c.name).toBe(plain < 1e-12);
    }
  });
});
