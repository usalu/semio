/** 🔍️ Round 3 tram: where the drawing and the body disagree in one view (`bun r3-tram-sil.ts 50 elevation [minMm]`): mismatch cells farther than minMm from the other silhouette, binned by 0.25 m, as `3d-only` (body not drawn) or `2d-only` (drawn, not modelled). Owned raster, slender solids left out. */
import { distanceField, entryOf, loadSource, merged, poseState, scaleMeshes, viewMasks } from "./volumes-lib.ts";

const [scale, view, minMm] = [Number(process.argv[2] ?? 50), process.argv[3] ?? "elevation", Number(process.argv[4] ?? 20)];
const { entry, asset } = entryOf("vehicles/tram");
const a = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(a, scale, a.volumes[String(scale)]);
const list = sm.body.filter((_, i) => !sm.solids.body[i]?.slender && !sm.solids.body[i]?.texture);
const m = viewMasks(view as never, poseState(a, scale) as never, list);
const dT = distanceField(m.truth);
const dV = distanceField(m.volume);
const bins = new Map<string, { n: number; max: number }>();
const w = m.truth.width;
for (let i = 0; i < m.truth.cells.length; i++) {
  const t = m.truth.cells[i]!;
  const v = m.volume.cells[i]!;
  if (t === v) continue;
  const d = t ? dV[i]! : dT[i]!;
  if (d * 1000 < minMm) continue;
  const x = m.box.minX + ((i % w) + 0.5) * m.cell;
  const y = m.box.minY + (Math.floor(i / w) + 0.5) * m.cell;
  const k = `${t ? "2d-only" : "3d-only"} x=${(Math.round(x * 4) / 4).toFixed(2)} y=${(Math.round(y * 4) / 4).toFixed(2)}`;
  const b = bins.get(k) ?? { n: 0, max: 0 };
  b.n++;
  b.max = Math.max(b.max, d);
  bins.set(k, b);
}
console.log(`[DEBUG] cell ${(m.cell * 1000).toFixed(1)} mm`);
for (const [k, b] of [...bins].sort((p, q) => q[1].n - p[1].n).slice(0, 40)) console.log(`[DEBUG] ${k} cells ${b.n} max ${(b.max * 1000).toFixed(0)} mm`);
