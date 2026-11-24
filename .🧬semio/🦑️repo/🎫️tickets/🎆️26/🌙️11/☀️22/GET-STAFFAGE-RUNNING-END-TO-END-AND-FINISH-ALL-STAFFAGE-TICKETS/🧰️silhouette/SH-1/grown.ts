/** 🎈️ Finds drawn shapes that the repair blew up: per person, scale and view every authored shape (same id before and after) whose default-pose bounding box grew to more than 1.6 times its area and by more than 25 cm², with its tone and drawing order; such a shape was replaced by the outline of its whole bone and now paints over its neighbours.
 *  bun grown.ts <backup dir> <slug>… */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { fileOf } from "./people.ts";

const [backupDir, ...slugs] = process.argv.slice(2) as string[];
const boxes = (asset: any, scale: number, view: string): Map<string, number> => {
  const out = new Map<string, number>();
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  for (const s of state.shapes[view] as any[]) { const p = s.rings.flat(); if (p.length > 0 && s.closed && s.shape.filled !== false) out.set(s.shape.id, (Math.max(...p.map((q: any) => q.x)) - Math.min(...p.map((q: any) => q.x))) * (Math.max(...p.map((q: any) => q.y)) - Math.min(...p.map((q: any) => q.y)))); }
  return out;
};
for (const slug of slugs) {
  const before = (await Bun.file(`${backupDir}/${slug}.json`).json()).asset, after = (await Bun.file(fileOf(slug)).json()).asset;
  const found: string[] = [];
  for (const scale of [50, 100, 200, 500]) {
    if (after.representations[String(scale)]?.status !== "authored") continue;
    for (const view of ["elevation", "front", "plan"]) {
      if (JSON.stringify(before.representations[String(scale)].geometry[view]) === JSON.stringify(after.representations[String(scale)].geometry[view])) continue;
      const [a, b] = [boxes(before, scale, view), boxes(after, scale, view)];
      for (const [id, area] of b) { const was = a.get(id); if (was !== undefined && area > 1.6 * was && area - was > 0.0025) { const s = after.representations[String(scale)].geometry[view].find((x: any) => x.id === id); found.push(`${scale} ${view} ${id}[${s.bone}] z ${s.z} tone ${s.tone}: ${(was * 1e4).toFixed(0)} -> ${(area * 1e4).toFixed(0)} cm²`); } }
    }
  }
  console.log(`${slug}: ${found.length === 0 ? "none" : "\n   " + found.join("\n   ")}`);
}
