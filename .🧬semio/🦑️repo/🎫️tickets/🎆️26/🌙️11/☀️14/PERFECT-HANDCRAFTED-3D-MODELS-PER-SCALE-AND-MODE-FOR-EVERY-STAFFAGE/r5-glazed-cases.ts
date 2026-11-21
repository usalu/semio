/** 🧪️ Prints, for every case of the product fixture `🧫️fixtures/🧊️volumes-flood`, the outer area by reach and the area seen through the glazing (E3) against the hand values.
 *
 * Usage: `bun r5-glazed-cases.ts`
 * @see ./r5-glazed-cavities.md */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { STAFFAGE } from "./volumes-lib.ts";

const { exposedArea, exposedShares, outerOfShares, outsideOf, seenFromOutside } = await import(`${STAFFAGE}/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts`);
const { tessellateSolid } = await import("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts");
const vectors = JSON.parse(readFileSync(join(STAFFAGE, "🧫️fixtures/🧊️volumes-flood/🔣️.json"), "utf8"));
const quality = { chordM: 0.002, minSegments: 32, maxSegments: 64 };
const circle = (r: number, n: number): [number, number][] => Array.from({ length: n }, (_, i) => [r * Math.cos((2 * Math.PI * i) / n), r * Math.sin((2 * Math.PI * i) / n)]);
const kernel = (id: string, solid: Record<string, unknown>) => {
  const g = tessellateSolid({ id, bone: "b", material: "m", role: "body", tone: 0.5, node: "b", ...solid } as never, quality);
  return { id, positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
};
const within = (v: number, hand: number): boolean => Math.abs(v - hand) <= Math.max(vectors.toleranceFraction * hand, vectors.toleranceM2);
for (const c of vectors.cases) {
  const worlds = c.solids.map((s: any, k: number) =>
    s.kind === "tube"
      ? kernel(`${s.region}-${k}`, { kind: "extrude", plane: "xy", profile: circle(s.outerRadiusM, s.segments), ...(s.innerRadiusM > 0 ? { holes: [circle(s.innerRadiusM, s.segments).reverse()] } : {}), from: s.from, to: s.to })
      : kernel(`${s.region}-${k}`, { kind: "extrude", plane: "xy", profile: [[s.min[0], s.min[1]], [s.max[0], s.min[1]], [s.max[0], s.max[1]], [s.min[0], s.max[1]]], from: s.min[2], to: s.max[2] }));
  const own = worlds.filter((_: unknown, k: number) => c.solids[k].region === c.target);
  const t0 = performance.now();
  const outside = outsideOf(worlds, vectors.featureM, vectors.probeM);
  const shares = exposedShares(own, own, [], () => false, vectors.probeM, vectors.seamM, (b: never, n: never) => outside.reach(b, n, (w: unknown) => own.includes(w)));
  const area = outerOfShares(shares);
  let seen = "";
  if (c.seenM2 !== undefined) {
    const clear = new Set(worlds.filter((_: unknown, k: number) => (c.transparent ?? []).includes(c.solids[k].region)));
    const s = exposedArea(own, own, [], () => false, vectors.probeM, vectors.seamM, seenFromOutside(worlds, (w: unknown) => clear.has(w), vectors.featureM, vectors.probeM));
    const o = exposedArea(own, own, [], () => false, vectors.probeM, vectors.seamM, seenFromOutside(worlds, () => false, vectors.featureM, vectors.probeM));
    seen = ` | seen ${s.toFixed(5)} hand ${c.seenM2} ${within(s, c.seenM2) ? "ok" : "FAIL"}, opaque ${o.toFixed(5)}`;
  }
  process.stdout.write(`${within(area, c.areaM2) ? "ok  " : "FAIL"} ${c.name}: ${area.toFixed(5)} hand ${c.areaM2} (air ${shares.air.toFixed(5)} cover ${shares.cover.toFixed(5)} cavity ${shares.cavity.toFixed(5)})${seen} ${(performance.now() - t0).toFixed(0)} ms\n`);
}
