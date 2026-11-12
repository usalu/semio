/** Before/after comparison of the three views of an asset: raster IoU per view at every scale, pose and four clip instants, plus IoU against the volume at rest.
 * Usage: bun lateral-snap.ts <slug> <beforeAsset.json> [--scales 50,100]  (after = the catalog file) */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, analyse, catalogEntries, loadAsset, VIEWS, type Json } from "./volume-lib.ts";
const FRACTIONS = [0, 0.25, 0.5, 0.75];
export const statesOf = (asset: Json) => [
  ...asset.poses.map((p: Json) => ({ name: `pose ${p.id}`, query: { kind: "pose", pose: p.id } })),
  ...asset.clips.flatMap((c: Json) => FRACTIONS.map((f) => ({ name: `clip ${c.id} @${(c.durationS * f).toFixed(2)}`, query: { kind: "clip", clip: c.id, timeS: c.durationS * f } }))),
];
export const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as never;
type Rings = { x: number; y: number }[][];
export function viewRings(asset: Json, query: Json, scale: number): Record<string, Rings> {
  const state = evaluate(asset, query, select(scale)) as Json;
  const out: Record<string, Rings> = {};
  for (const v of VIEWS) out[v] = state.shapes[v].filter((s: Json) => s.shape.role !== "ground" && s.closed !== false).map((s: Json) => s.rings as Rings).flat().length ? state.shapes[v].filter((s: Json) => s.shape.role !== "ground" && s.closed !== false).flatMap((s: Json) => [s.rings as Rings]).flat() : [];
  return out;
}
export function shapeSets(asset: Json, query: Json, scale: number): Record<string, Rings[]> {
  const state = evaluate(asset, query, select(scale)) as Json;
  const out: Record<string, Rings[]> = {};
  for (const v of VIEWS) out[v] = state.shapes[v].filter((s: Json) => s.shape.role !== "ground" && s.closed !== false).map((s: Json) => s.rings as Rings);
  return out;
}
const boxOf = (sets: Rings[][]) => {
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const set of sets) for (const rings of set) for (const ring of rings) for (const p of ring) { minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y); }
  return Number.isFinite(minX) ? { minX: minX - 1e-3, minY: minY - 1e-3, maxX: maxX + 1e-3, maxY: maxY + 1e-3 } : { minX: 0, minY: 0, maxX: 1, maxY: 1 };
};
export function iouOf(a: Rings[], b: Rings[]): number {
  const box = boxOf([a, b]);
  const cell = Math.max(box.maxX - box.minX, box.maxY - box.minY) / 500;
  const ma = new Mask(box, cell), mb = new Mask(box, cell);
  for (const rings of a) ma.fill(rings);
  for (const rings of b) mb.fill(rings);
  let inter = 0, uni = 0;
  for (let i = 0; i < ma.cells.length; i++) { const x = ma.cells[i]!, y = mb.cells[i]!; inter += x & y; uni += x | y; }
  return uni === 0 ? 1 : inter / uni;
}
if (import.meta.main) {
  const slug = process.argv[2]!;
  const before = JSON.parse(readFileSync(process.argv[3]!, "utf8")).asset as Json;
  const after = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
  const scaleArg = process.argv.indexOf("--scales");
  const scales = scaleArg > 0 ? process.argv[scaleArg + 1]!.split(",").map(Number) : [50, 100, 200, 500];
  console.log(`# ${slug}: before/after raster IoU per view (min over states; state of the minimum)`);
  for (const scale of scales) {
    const worst: Record<string, { iou: number; at: string }> = Object.fromEntries(VIEWS.map((v) => [v, { iou: 2, at: "" }]));
    const rest: Record<string, number> = {};
    for (const st of statesOf(after)) {
      const b = shapeSets(before, st.query, scale), a = shapeSets(after, st.query, scale);
      for (const v of VIEWS) {
        const i = iouOf(b[v]!, a[v]!);
        if (i < worst[v]!.iou) worst[v] = { iou: i, at: st.name };
        if (st.name === `pose ${after.presentation.defaultPose}`) rest[v] = i;
      }
    }
    console.log(`1:${scale}  default pose: ${VIEWS.map((v) => `${v} ${rest[v]!.toFixed(4)}`).join("  ")}  | min: ${VIEWS.map((v) => `${v} ${worst[v]!.iou.toFixed(4)} (${worst[v]!.at})`).join("  ")}`);
  }
  console.log(`# IoU against the volume at rest (before -> after)`);
  for (const scale of scales) {
    const ab = analyse(before, before.volume, scale), aa = analyse(after, after.volume, scale);
    console.log(`1:${scale}  ${VIEWS.map((v) => `${v} ${ab.views[v].iou.toFixed(4)} -> ${aa.views[v].iou.toFixed(4)}`).join("   ")}`);
  }
}
