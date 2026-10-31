import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { outlineRuns, prepare, sourceOf } = await import(`${S}/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { whiteSliver } = await import(`${S}/🔨️modules/🖨️representation/📐️measure/🟦️.ts`);
const { loadCatalogFromFiles } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const { values } = parseArgs({ options: { asset: { type: "string" }, scale: { type: "string" }, pose: { type: "string" } } });
const asset = (await loadCatalogFromFiles()).assets.find((a: any) => a.id === values.asset);
const den = Number(values.scale);
const k = 1000 / den;
const state = evaluate(asset, { kind: "pose", pose: values.pose ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: den } });
const prepared = prepare(state, "elevation", sourceOf(state), k);
const run = (omit: string | null) => {
  const pr = prepared.filter((p: any) => p.shape.id !== omit);
  const paper: any[] = [];
  for (const { p, run } of outlineRuns(pr, true)) if (p.shape.role !== "ground") paper.push({ points: run.points, closed: run.closed, penMm: p.weightMm });
  return whiteSliver(pr.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper, 0.15);
};
const base = run(null);
console.log("base", base ? `${base.widthMm.toFixed(3)} at ${base.at.x.toFixed(2)},${base.at.y.toFixed(2)}` : "none");
if (base) for (const p of prepared) {
  const r = run(p.shape.id);
  if (r === null || r.widthMm > base.widthMm + 0.002) console.log("omit", p.shape.id, r ? r.widthMm.toFixed(3) : "none");
}
