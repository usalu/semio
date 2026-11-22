/** 🔬️ Visible feature width of shapes per named pose and view with their occluders: bun feature.ts <slug|file.json> <scale> <shapeId>[,<shapeId>…] [clip@t …]
 * @see ../HW-08/suit5.ts — the probe this one generalises */
import { join, resolve } from "node:path";
const SR = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { prepare, sourceOf } = await import(`${SR}/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts`);
const { evaluateDrawing } = await import(`${SR}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { featureWidthMm, visibleFeatureWidthMm } = await import(`${SR}/🔨️modules/🖨️representation/📐️measure/🟦️.ts`);
const [slug, sc, ids, ...instants] = process.argv.slice(2);
const file = slug!.endsWith(".json") ? slug! : join(resolve(import.meta.dir, SR, "🗂️catalog/👤️humans"), slug!, "🔣️.json");
const a = (await Bun.file(file).json()).asset;
const k = 1000 / Number(sc);
const queries = [...a.poses.filter((p: any) => !p.key).map((p: any) => ({ name: p.id, q: { kind: "pose", pose: p.id } })), ...instants.map((i) => ({ name: i, q: { kind: "clip", clip: i.split("@")[0], timeS: Number(i.split("@")[1]) } }))];
for (const { name, q } of queries) {
  const state = evaluateDrawing(a, q, { kind: "scale", scale: { denominator: Number(sc) } });
  const source = sourceOf(state);
  for (const vp of ["elevation", "front", "plan"]) {
    const prepared = prepare(state, vp, source, k);
    for (const id of ids!.split(",")) {
      const p = prepared.find((x: any) => x.shape.id === id);
      if (!p) continue;
      const covers = prepared.filter((x: any) => x.occluder !== null && x.order > p.order && x.box.minX <= p.box.maxX && p.box.minX <= x.box.maxX && x.box.minY <= p.box.maxY && p.box.minY <= x.box.maxY);
      const raw = featureWidthMm(p.rings, 0.2, 5);
      const vis = covers.length ? visibleFeatureWidthMm(p.rings, covers.map((x: any) => ({ rings: x.occluder.rings, penMm: x.weightMm })), 0.2, 5) : null;
      console.log(name, vp, id, "raw", raw?.toFixed?.(3), "visible", vis ? vis.widthMm.toFixed(3) : "-", "covers", covers.map((x: any) => x.shape.id).join(","));
    }
  }
}
