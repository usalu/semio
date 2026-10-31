/** [DEBUG] One-pose gap pull experiment. */
import { readFileSync } from "node:fs";
import { evaluate, flattenGeometry } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { prepare, sourceOf, outlineRuns } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { gapsBetween } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";
import { apply, invert } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const file = process.argv[2]!;
const den = Number(process.argv[3] ?? 500);
const doc = JSON.parse(readFileSync(file, "utf8"));
const asset = doc.asset;
const k = 1000 / den;
const pose = asset.presentation.defaultPose;

function gapsOf(a: typeof asset): { id: string; n: number; sample: string } {
  const state = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: den } });
  const prepared = prepare(state, "elevation", sourceOf(state), k);
  const paper = [];
  for (const item of outlineRuns(prepared, true)) if (item.p.shape.role !== "ground") paper.push({ id: item.p.shape.id, rings: [item.run.points], closed: item.run.closed, penMm: item.p.weightMm });
  const gaps = gapsBetween(paper, 1).filter((g) => g.gapMm > 0 && g.gapMm < 0.15);
  return { id: a.id, n: gaps.length, sample: gaps.slice(0, 6).map((g) => `${g.a}+${g.b}:${g.gapMm.toFixed(3)}@${g.centreMm.toFixed(3)}`).join(" ") };
}

console.log("before", JSON.stringify(gapsOf(asset)));
const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: den } });
const prepared = prepare(state, "elevation", sourceOf(state), k);
const runs: { id: string; pts: { x: number; y: number }[]; pen: number }[] = [];
for (const item of outlineRuns(prepared, true)) {
  if (item.p.shape.role === "ground") continue;
  runs.push({ id: item.p.shape.id, pen: item.p.weightMm, pts: item.run.points.map((p) => ({ x: p.x / k, y: -p.y / k })) });
}
const paper = runs.map((r) => ({ id: r.id, rings: [r.pts.map((p) => ({ x: p.x * k, y: -p.y * k }))], closed: false, penMm: r.pen }));
const gaps = gapsBetween(paper, 1).filter((g) => g.gapMm > 0 && g.gapMm < 0.15);
const rep = asset.representations[String(den)];
const byId = new Map(rep.geometry.elevation.map((s: any) => [s.id, s]));
let moved = 0;
for (const gap of gaps) {
  const at = { x: gap.at.x / k, y: -gap.at.y / k };
  const ra = runs.filter((r) => r.id === gap.a);
  const rb = runs.filter((r) => r.id === gap.b);
  const sa = byId.get(gap.a);
  const sb = byId.get(gap.b);
  if (!sa || !sb) continue;
  const centreM = gap.centreMm / k;
  const reduceM = (gap.gapMm + 0.04) / k;
  for (const [shape, others] of [[sa, rb], [sb, ra]] as const) {
    const matrix = state.bones[shape.bone].world;
    const flat = flattenGeometry(shape);
    const rings = flat.rings.map((ring) => ring.map((p) => apply(matrix, p.x, p.y)));
    let localMoved = false;
    for (const ring of rings) for (const p of ring) {
      if (Math.hypot(p.x - at.x, p.y - at.y) > centreM + 0.02) continue;
      let best = Infinity;
      let q = p;
      for (const run of others) for (let i = 0; i + 1 < run.pts.length; i++) {
        const a = run.pts[i]!;
        const b = run.pts[i + 1]!;
        const dx = b.x - a.x;
        const dy = b.y - a.y;
        const l2 = dx * dx + dy * dy;
        const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / l2));
        const x = a.x + t * dx;
        const y = a.y + t * dy;
        const d = Math.hypot(p.x - x, p.y - y);
        if (d < best) {
          best = d;
          q = { x, y };
        }
      }
      if (!(best > 1e-5 && best < centreM + 0.01)) continue;
      const dist = Math.min(reduceM / 2 + 0.001, best * 0.6);
      p.x += ((q.x - p.x) / best) * dist;
      p.y += ((q.y - p.y) / best) * dist;
      localMoved = true;
    }
    if (!localMoved) continue;
    moved++;
    const inv = invert(matrix);
    const local = rings.map((ring) => ring.map((p) => apply(inv, p.x, p.y)));
    const d = local.map((ring) => ring.map((p, i) => `${i ? "L" : "M"}${p.x.toFixed(5)} ${p.y.toFixed(5)}`).join(" ") + (flat.closed ? " Z" : "")).join(" ");
    delete shape.points; delete shape.closed; delete shape.x0; delete shape.y0; delete shape.x1; delete shape.y1; delete shape.r0; delete shape.r1; delete shape.cx; delete shape.cy; delete shape.rx; delete shape.ry; delete shape.x; delete shape.y; delete shape.w; delete shape.h; delete shape.r;
    shape.kind = "path";
    shape.d = d;
  }
}
console.log("moved pairs", moved, "gaps", gaps.length);
console.log("after", JSON.stringify(gapsOf(asset)));
