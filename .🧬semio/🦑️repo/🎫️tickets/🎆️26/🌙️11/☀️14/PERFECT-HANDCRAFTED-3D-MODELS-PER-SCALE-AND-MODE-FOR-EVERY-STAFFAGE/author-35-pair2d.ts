import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, strokeQuad } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, aS, bS] = process.argv.slice(2) as string[];
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!) as any;
const ink = (sc: number, view: string) => {
  const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: sc } }) as any;
  const rep = m.representations[String(sc)];
  const groups: any[] = [];
  for (const s of st.shapes[view]) {
    if (!["body", "detail", "glazing"].includes(s.shape.role) || s.rings.length === 0) continue;
    const pen = ((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) * sc) / 1000;
    if (s.closed && s.shape.filled !== false) { groups.push(s.rings); continue; }
    for (const ring of s.rings) { const r = s.closed ? [...ring, ring[0]] : ring; for (let i = 0; i + 1 < r.length; i++) groups.push([strokeQuad(r[i], r[i + 1], Math.max(pen, 1e-4))]); }
  }
  return groups;
};
for (const view of ["elevation", "front", "plan"]) {
  const [ga, gb] = [ink(Number(aS), view), ink(Number(bS), view)];
  let x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9; for (const g of [...ga, ...gb]) for (const r of g) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
  const cell = 0.5 * 0.15e-3 * Number(aS) / 2; const box = { minX: x0 - cell, minY: y0 - cell, maxX: x1 + cell, maxY: y1 + cell };
  const A = new Mask(box, cell), B = new Mask(box, cell); for (const g of ga) A.fill(g); for (const g of gb) B.fill(g);
  let both = 0, either = 0; for (let k = 0; k < A.cells.length; k++) { both += A.cells[k]! & B.cells[k]!; either += A.cells[k]! | B.cells[k]!; }
  console.log(view, "2D pair incl. strokes at pen width: IoU", (both / either).toFixed(3), "cell", cell);
}
