/** ✂️ The simplification of a drawn ring on paper keeps the extent of the ring: the tolerance of 0.01 mm cuts the curve between its kept vertices but never the tip of a tapered or rounded end, which at 1:1000 is a length of 10 mm of the object.
 *
 * THE REFERENCE is shapely (the bounds of the ring and of its simplification, and the Hausdorff distance between the two polygons) and the hand-computed simplification of a square with a point in the middle of every edge.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { simplify } from "../../🔨️modules/🎨️depiction/📐️geometry/🟦️.ts";
import { STAFFAGE_ROOT, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Vec = { readonly x: number; readonly y: number };
const TOLERANCE_MM = 0.01;

const ellipse = (rx: number, ry: number, turnDeg: number, n: number, exponent = 2): Vec[] =>
  Array.from({ length: n }, (_, i) => {
    const t = (2 * Math.PI * i) / n;
    const [c, s] = [Math.cos(t), Math.sin(t)];
    const [x, y] = [rx * Math.sign(c) * Math.abs(c) ** (2 / exponent), ry * Math.sign(s) * Math.abs(s) ** (2 / exponent)];
    const a = (turnDeg * Math.PI) / 180;
    return { x: x * Math.cos(a) - y * Math.sin(a), y: x * Math.sin(a) + y * Math.cos(a) };
  });

const rings: readonly (readonly [string, Vec[]])[] = [
  ["the hand of a 1:1000 plan, 0.124 mm long", ellipse(0.062, 0.03, 0, 64)],
  ["the same turned by 37 degrees", ellipse(0.062, 0.03, 37, 64)],
  ["a rounded foot, exponent 2.5", ellipse(0.125, 0.045, 0, 96, 2.5)],
  ["a thin leaf, 0.5 mm by 0.04 mm, 180 points", ellipse(0.25, 0.02, 12, 180)],
];

const box = (points: readonly Vec[]): [number, number, number, number] => [Math.min(...points.map((p) => p.x)), Math.min(...points.map((p) => p.y)), Math.max(...points.map((p) => p.x)), Math.max(...points.map((p) => p.y))];

describe("✂️ the paper simplification keeps the extent of a ring", () => {
  it.each(rings)("keeps every side of the box of %s", (_, ring) => {
    const out = simplify(ring, true, TOLERANCE_MM);
    box(out).forEach((value, i) => expect(value, `side ${i}`).toBeCloseTo(box(ring)[i]!, 12));
    expect(out.length).toBeLessThan(ring.length);
  });

  it.each(rings)("keeps the vertices of %s in order, each one a vertex of the ring", (_, ring) => {
    const out = simplify(ring, true, TOLERANCE_MM);
    let at = -1;
    for (const p of out) {
      at = ring.indexOf(p, at + 1);
      expect(at).toBeGreaterThanOrEqual(0);
    }
  });

  it("leaves a square with a point in the middle of every edge as the four corners", () => {
    const square: Vec[] = [{ x: 0, y: 0 }, { x: 1, y: 0 }, { x: 2, y: 0 }, { x: 2, y: 1 }, { x: 2, y: 2 }, { x: 1, y: 2 }, { x: 0, y: 2 }, { x: 0, y: 1 }];
    expect(simplify(square, true, TOLERANCE_MM)).toEqual([{ x: 0, y: 0 }, { x: 2, y: 0 }, { x: 2, y: 2 }, { x: 0, y: 2 }]);
  });

  it("equals the bounds and stays within the tolerance of the ring that shapely measures", () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-simplify-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ pairs: rings.map(([, ring]) => ({ ring: ring.map((p) => [p.x, p.y]), simplified: simplify(ring, true, TOLERANCE_MM).map((p) => [p.x, p.y]) })) }));
      const measured = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/✂️paper-simplify/🐍️.py"), "geometry", [input])) as { original: number[]; simplified: number[]; hausdorff: number }[];
      measured.forEach((m, k) => {
        m.simplified.forEach((value, i) => expect(value, `${rings[k]![0]} side ${i}`).toBeCloseTo(m.original[i]!, 12));
        expect(m.hausdorff, `${rings[k]![0]} Hausdorff distance`).toBeLessThanOrEqual(TOLERANCE_MM + 1e-9);
      });
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});
