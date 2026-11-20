/** 🌊️ Prints, for every case of the product fixture `🧫️fixtures/🧊️volumes-flood`, the flooded and the unflooded outer area against the hand value, with the time of the flood.
 *
 * Usage: `bun r4-area-probe.ts`
 * @see ./r4-area-flood.md */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { STAFFAGE } from "./volumes-lib.ts";

const { exposedArea, outsideOf } = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts");
const { tessellateSolid } = await import("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts");
const vectors = JSON.parse(readFileSync(join(STAFFAGE, "🧫️fixtures/🧊️volumes-flood/🔣️.json"), "utf8"));
const quality = { chordM: 0.002, minSegments: 32, maxSegments: 64 };
const circle = (r: number, n: number): [number, number][] => Array.from({ length: n }, (_, i) => [r * Math.cos((2 * Math.PI * i) / n), r * Math.sin((2 * Math.PI * i) / n)]);
const kernel = (id: string, solid: Record<string, unknown>) => {
  const g = tessellateSolid({ id, bone: "b", material: "m", role: "body", tone: 0.5, node: "b", ...solid } as never, quality);
  return { id, positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
};
for (const c of vectors.cases) {
  const worlds = c.solids.map((s: any, k: number) =>
    s.kind === "tube"
      ? kernel(`${s.region}-${k}`, { kind: "extrude", plane: "xy", profile: circle(s.outerRadiusM, s.segments), ...(s.innerRadiusM > 0 ? { holes: [circle(s.innerRadiusM, s.segments).reverse()] } : {}), from: s.from, to: s.to })
      : kernel(`${s.region}-${k}`, { kind: "extrude", plane: "xy", profile: [[s.min[0], s.min[1]], [s.max[0], s.min[1]], [s.max[0], s.max[1]], [s.min[0], s.max[1]]], from: s.min[2], to: s.max[2] }));
  const own = worlds.filter((_: unknown, k: number) => c.solids[k].region === c.target);
  const t0 = performance.now();
  const outside = outsideOf(worlds, vectors.featureM, vectors.probeM);
  const t1 = performance.now();
  const area = exposedArea(own, own, [], () => false, vectors.probeM, vectors.seamM, (b: never, n: never) => outside.counts(b, n, (w: unknown) => own.includes(w)));
  const t2 = performance.now();
  const plain = exposedArea(own, own, [], () => false, vectors.probeM, vectors.seamM);
  const ok = Math.abs(area - c.areaM2) <= Math.max(vectors.toleranceFraction * c.areaM2, vectors.toleranceM2);
  process.stdout.write(`${ok ? "ok  " : "FAIL"} ${c.name}: flooded ${area.toFixed(5)} hand ${c.areaM2} | unflooded ${plain.toFixed(5)} fixture ${c.unfloodedM2 ?? "-"} | flood ${(t1 - t0).toFixed(0)} ms, area ${(t2 - t1).toFixed(0)} ms, cell ${outside.cellM}\n`);
}
