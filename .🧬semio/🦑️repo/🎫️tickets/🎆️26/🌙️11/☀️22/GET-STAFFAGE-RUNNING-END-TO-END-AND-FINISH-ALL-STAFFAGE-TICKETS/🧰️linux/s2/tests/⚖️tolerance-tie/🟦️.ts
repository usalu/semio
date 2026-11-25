/** ⚖️ Tolerance tie: a drawn extent whose deviation equals D18 has one verdict in metres and in paper millimetres at every denominator.
 *
 * THE REFERENCE is exact rational arithmetic (CPython `fractions.Fraction`, 🐍️.py).
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/⚖️tolerance-tie/🔣️.json — the vectors
 * @see ../🔭️multiview-coverage/🧰️.ts — `exceeds`, `paperFindings`, `geometryFindings`
 */
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import type { DrawingScene, EvaluatedShape } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { ROUNDING_ULPS, d18, exceeds, paperFindings } from "../🔭️multiview-coverage/🧰️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Vectors = { d18: { absoluteM: string; relative: string }; denominators: number[]; originsMm: number[]; cases: { name: string; measuredM: string; nominalM: string }[] };
const vectors = fixture<Vectors>("⚖️tolerance-tie");

const sceneOf = (heightMm: number, originMm: number, mmPerMetre: number): DrawingScene =>
  ({
    layers: [{ id: "fill", items: [{ kind: "path", id: "shape:body", commands: [{ op: "M", x: originMm, y: originMm }, { op: "L", x: originMm + 10, y: originMm }, { op: "L", x: originMm + 10, y: originMm + heightMm }, { op: "L", x: originMm, y: originMm + heightMm }, { op: "Z" }], fill: { color: "#000000" } }] }],
    metadata: { mmPerMetre },
  }) as unknown as DrawingScene;
const shapes = [{ shape: { id: "body", role: "body" }, rings: [], closed: true }] as unknown as EvaluatedShape[];

describe("⚖️ a deviation at the tolerance", () => {
  requireTool("uv");
  const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/⚖️tolerance-tie/🐍️.py"), null, [join(STAFFAGE_ROOT, "🧫️fixtures/⚖️tolerance-tie/🔣️.json")])) as boolean[];

  it("uses the tolerance of the vectors", () => {
    expect(d18(1.15)).toBe(Math.max(Number(vectors.d18.absoluteM), Number(vectors.d18.relative) * 1.15));
    expect(ROUNDING_ULPS * Number.EPSILON).toBeLessThan(1e-13);
  });

  vectors.cases.forEach((c, i) =>
    it(`has the verdict of exact arithmetic in metres and on paper at every denominator: ${c.name}`, () => {
      const [measured, nominal] = [Number(c.measuredM), Number(c.nominalM)];
      expect(exceeds(Math.abs(measured - nominal), d18(nominal), Math.max(measured, nominal)), "metres").toBe(reference[i]);
      for (const denominator of vectors.denominators) {
        const k = 1000 / denominator;
        expect(exceeds(Math.abs(measured * k - nominal * k), d18(nominal) * k, Math.max(measured, nominal) * k), `paper 1:${denominator}`).toBe(reference[i]);
        for (const origin of vectors.originsMm) {
          const asset = { dimensions: { widthM: 10 / k, heightM: nominal, depthM: 10 / k, massKg: 1 } } as unknown as Asset;
          const heights = paperFindings(asset, "elevation", sceneOf(measured * k, origin, k), shapes).filter((finding) => finding.check === "paper-height");
          expect(heights.length === 1, `paperFindings 1:${denominator} at ${origin} mm`).toBe(reference[i]);
        }
      }
    }),
  );

  it("counts a deviation that is not a number as exceeding", () => {
    expect(exceeds(Number.NaN, 0.02, 1)).toBe(true);
  });
});
