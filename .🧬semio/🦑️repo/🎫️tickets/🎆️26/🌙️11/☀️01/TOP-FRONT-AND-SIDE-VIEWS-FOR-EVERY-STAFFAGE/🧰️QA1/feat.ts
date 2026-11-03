/** [DEBUG] QA1: print feature widths of the thinnest filled shapes per pose and view. usage: bun feat.ts <asset> <scale> */
import { join, resolve } from "node:path";
const S = join(resolve(import.meta.dir, "../../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage");
const { catalog } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { prepare, sourceOf } = await import(join(S, "🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts"));
const { featureWidthMm } = await import(join(S, "🔨️modules/🖨️representation/📐️measure/🟦️.ts"));
for (const pair of process.argv[2]!.split(",")) {
const [id, sc] = pair.split(":");
const asset = (catalog() as any[]).find((a) => a.id === id);
const k = 1000 / Number(sc);
console.log("##", id, sc);
for (const pose of asset.poses) for (const vp of ["elevation", "front", "plan"]) {
  const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: Number(sc) } });
  const prepared = prepare(state, vp, sourceOf(state), k);
  for (const p of prepared.filter((q: any) => q.closed && q.shape.filled && q.shape.role !== "ground")) {
    const raw = featureWidthMm(p.rings, 0.2, 0.2 - p.weightMm + 1e-9);
    const box = Math.min(p.box.maxX - p.box.minX, p.box.maxY - p.box.minY);
    const w = Math.max(raw, Math.min(box, 0.2)) + p.weightMm;
    if (w < 0.25) console.log(`${pose.id} ${vp} ${p.shape.id} raw=${raw.toFixed(3)} box=${box.toFixed(3)} weight=${p.weightMm} -> ${w.toFixed(3)}`);
  }
}
}
