/** 📦️ The paper boxes (millimetres) and rings of one filled shape and of every nearer shape that overlaps its box, per pose in one view: `bun boxes.ts <id|file.json> <scale> <shape id> <view> [pose]`; with the visible feature width the print suitability measures.
 * @see ../MH-3/probe-feature.ts */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const load = (path: string): Promise<any> => import(join(S, "🔨️modules", path));
const { prepare, sourceOf } = await load("🎨️depiction/🧩️shapes/🟦️.ts");
const { evaluateDrawing } = await load("🧮️evaluation/🟦️.ts");
const { visibleFeatureWidthMm } = await load("🖨️representation/📐️measure/🟦️.ts");
const [arg, key, only, view, onePose] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const f = (n: number): string => n.toFixed(3);
for (const pose of asset.poses.filter((candidate: any) => !candidate.key && (onePose === undefined || candidate.id === onePose))) {
  const state = evaluateDrawing(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: Number(key) } });
  const prepared = prepare(state, view, sourceOf(state), 1000 / Number(key)) as any[];
  for (const p of prepared.filter((q) => q.closed && q.shape.id === only)) {
    const covers = prepared.filter((q) => q.occluder !== null && q.order > p.order && q.box.minX <= p.box.maxX && p.box.minX <= q.box.maxX && q.box.minY <= p.box.maxY && p.box.minY <= q.box.maxY);
    const visible = covers.length === 0 ? null : visibleFeatureWidthMm(p.rings, covers.map((q) => ({ rings: q.occluder.rings, penMm: q.weightMm })), 0.2, 10);
    console.log(`${pose.id} ${view} ${only}: pen ${p.weightMm} visible ${visible === null ? "-" : `${f(visible.widthMm)} box ${f(visible.boxMinMm)}`}`);
    for (const q of [p, ...covers]) console.log(`  ${q.shape.id} order ${q.order} pen ${q.weightMm} x ${f(q.box.minX)}..${f(q.box.maxX)} y ${f(q.box.minY)}..${f(q.box.maxY)}${onePose ? " " + JSON.stringify((q.occluder?.rings ?? q.rings).map((ring: any) => ring.map((pt: any) => [Number(f(pt.x ?? pt[0])), Number(f(pt.y ?? pt[1]))]))) : ""}`);
  }
}
