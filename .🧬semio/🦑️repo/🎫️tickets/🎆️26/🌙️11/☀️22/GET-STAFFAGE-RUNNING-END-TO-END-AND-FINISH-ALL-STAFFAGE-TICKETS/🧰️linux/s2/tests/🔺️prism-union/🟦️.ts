/** 🔺️ Prism union: the outline of a swept profile is built at every orientation, also where the profile plane turns through the view direction.
 *
 * THE REFERENCE is shapely `unary_union` (🧪️tests/🔺️polygon-union/🐍️.py) and polygon-clipping.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🔺️prism-union/🔣️.json — the recorded faces and the profiles
 * @see ../../🔨️modules/🔺️polygon/🟦️.ts — `unionPolygons`
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import polygonClipping from "polygon-clipping";
import { describe, expect, it } from "vitest";
import { PolygonUnionError, ringArea, unionPolygons } from "../../🔨️modules/🔺️polygon/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Ring = [number, number][];
type Sweep = { name: string; profile: Ring; span: [number, number]; tiltDeg: number; turnDeg: { from: number; to: number; step: number } };
type Vectors = { tolerance: { relativeArea: number }; recorded: { name: string; rings: number; polygons: Ring[] }[]; sweeps: Sweep[] };
const vectors = fixture<Vectors>("🔺️prism-union");

const points = (polygons: Ring[]) => polygons.map((ring) => ring.map(([x, y]) => ({ x, y })));
const areaOf = (polygons: Ring[]): { area: number; rings: number; error: string | null } => {
  try {
    const union = unionPolygons(points(polygons));
    return { area: union.reduce((sum, ring) => sum + ringArea(ring), 0), rings: union.length, error: null };
  } catch (error) {
    if (!(error instanceof PolygonUnionError)) throw error;
    return { area: Number.NaN, rings: 0, error: error.message };
  }
};

/** 🎡️ The projected faces of a prism: the profile in the (u, v) plane of a frame tilted by `tiltDeg` about the view direction and turned by 90 degrees plus `turnDeg` about the vertical, swept along w over `span`, seen along z; the profile at both ends and the sweep of every edge, as the evaluation draws an extruded shape. */
function prismFaces(sweep: Sweep, turnDeg: number): Ring[] {
  const [tilt, turn] = [(sweep.tiltDeg * Math.PI) / 180, ((90 + turnDeg) * Math.PI) / 180];
  const u = [Math.cos(tilt) * Math.cos(turn), Math.sin(tilt)] as const;
  const v = [-Math.sin(tilt) * Math.cos(turn), Math.cos(tilt)] as const;
  const w = [Math.sin(turn), 0] as const;
  const at = ([a, b]: [number, number], c: number): [number, number] => [a * u[0] + b * v[0] + c * w[0], a * u[1] + b * v[1] + c * w[1]];
  const low = sweep.profile.map((p) => at(p, sweep.span[0]));
  const high = sweep.profile.map((p) => at(p, sweep.span[1]));
  return [low, high, ...low.map((p, i): Ring => [p, low[(i + 1) % low.length]!, high[(i + 1) % high.length]!, high[i]!])];
}

const turnsOf = (sweep: Sweep): number[] => Array.from({ length: Math.round((sweep.turnDeg.to - sweep.turnDeg.from) / sweep.turnDeg.step) + 1 }, (_, i) => sweep.turnDeg.from + i * sweep.turnDeg.step);

const shapely = (cases: { polygons: Ring[] }[]): { area: number; rings: number }[] => {
  const directory = mkdtempSync(join(tmpdir(), "staffage-prism-union-"));
  try {
    const input = join(directory, "input.json");
    writeFileSync(input, JSON.stringify({ cases }));
    return JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🔺️polygon-union/🐍️.py"), "geometry", [input])) as { area: number; rings: number }[];
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
};

describe("🔺️ the recorded faces of catalog people", () => {
  requireTool("uv");
  const reference = shapely(vectors.recorded);
  vectors.recorded.forEach((c, i) =>
    it(`is one ring of the shapely and the polygon-clipping area: ${c.name.slice(0, 80)}`, () => {
      const [first, ...rest] = c.polygons.map((ring) => [[...ring, ring[0]!]]);
      const clipped = polygonClipping.union(first!, ...rest).reduce((sum, polygon) => sum + polygon.reduce((inner, ring, k) => inner + (k === 0 ? 1 : -1) * Math.abs(ringArea(ring.slice(0, -1).map(([x, y]) => ({ x: x!, y: y! })))), 0), 0);
      const got = areaOf(c.polygons);
      expect(got.error, c.name).toBeNull();
      expect(got.rings, c.name).toBe(c.rings);
      expect(reference[i]!.rings, `${c.name} shapely rings`).toBe(c.rings);
      expect(Math.abs(got.area - reference[i]!.area), `${c.name} shapely`).toBeLessThanOrEqual(vectors.tolerance.relativeArea * reference[i]!.area);
      expect(Math.abs(got.area - clipped), `${c.name} polygon-clipping`).toBeLessThanOrEqual(vectors.tolerance.relativeArea * clipped);
    }),
  );
});

describe("🔺️ a traced profile turned through the view direction", () => {
  requireTool("uv");
  vectors.sweeps.forEach((sweep) =>
    it(`is built at every step with the shapely area: ${sweep.name}`, { timeout: 600_000 }, () => {
      const turns = turnsOf(sweep);
      const cases = turns.map((turn) => ({ polygons: prismFaces(sweep, turn) }));
      const reference = shapely(cases);
      const failures: string[] = [];
      cases.forEach((c, i) => {
        const got = areaOf(c.polygons);
        if (got.error !== null) failures.push(`${turns[i]!.toFixed(5)} deg: ${got.error}`);
        else if (!(Math.abs(got.area - reference[i]!.area) <= vectors.tolerance.relativeArea * reference[i]!.area)) failures.push(`${turns[i]!.toFixed(5)} deg: area ${got.area} against ${reference[i]!.area}`);
      });
      expect(failures, `${failures.length} of ${cases.length} steps`).toEqual([]);
    }),
  );
});
