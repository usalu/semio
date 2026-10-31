import { readFileSync, writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { DEFAULT_PRINT_PROFILE, featureWidthMm, gapsBetween, whiteSliver } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const { outlineRuns, prepare, sourceOf } = await import(`${S}/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { values } = parseArgs({ options: { file: { type: "string" }, scales: { type: "string", default: "500,200,100,50" }, out: { type: "string" }, only: { type: "string" } } });
const asset = JSON.parse(readFileSync(values.file!, "utf8")).asset;
const profile = DEFAULT_PRINT_PROFILE;
const result: any = {};
for (const d of values.scales!.split(",").map(Number)) {
  const rep = asset.representations[String(d)];
  const k = 1000 / d;
  const t0 = Date.now();
  const list: any[] = [];
  for (const pose of asset.poses.filter((p: any) => !p.key)) {
    if (values.only && !values.only.split(",").includes(pose.id)) continue;
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: d } });
    const prepared = prepare(state, "elevation", sourceOf(state), k);
    for (const p of prepared) {
      if (p.shape.role === "ground") continue;
      if (p.closed && p.shape.filled) {
        const w = featureWidthMm(p.rings, profile.minFeatureMm);
        if (w < profile.minFeatureMm - 1e-9) list.push({ pose: pose.id, check: "feature", shapes: [p.shape.id], mm: +w.toFixed(3) });
      }
    }
    const paper: any[] = [];
    for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") paper.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm });
    const sl = whiteSliver(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), profile.minGapMm);
    if (sl !== null) list.push({ pose: pose.id, check: "sliver", shapes: ["negative-space"], mm: +sl.widthMm.toFixed(3), at: [+sl.at.x.toFixed(2), +sl.at.y.toFixed(2)] });
    for (const g of gapsBetween(paper, 1)) if (g.gapMm < profile.minGapMm - 1e-9) list.push({ pose: pose.id, check: "gap", shapes: [g.a, g.b], mm: +g.gapMm.toFixed(3), at: [+g.at.x.toFixed(2), +g.at.y.toFixed(2)] });
  }
  result[d] = list;
  const pairs = new Map<string, any>();
  for (const v of list) { const key = v.check + ":" + v.shapes.join("+"); const o = pairs.get(key); if (!o || v.mm < o.mm) pairs.set(key, v); }
  console.log(`1:${d} violations ${list.length} distinct ${pairs.size} (${Date.now() - t0} ms)`);
  for (const v of pairs.values()) console.log(`   ${v.check} ${v.shapes.join(" + ")} ${v.mm} @${v.at ?? ""} pose ${v.pose}`);
}
if (values.out) writeFileSync(values.out, JSON.stringify(result));
