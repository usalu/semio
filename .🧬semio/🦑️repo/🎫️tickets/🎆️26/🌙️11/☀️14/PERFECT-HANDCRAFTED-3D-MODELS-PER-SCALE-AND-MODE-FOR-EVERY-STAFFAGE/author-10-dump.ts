import { entryOf, restState, frames, bboxOf } from "./volumes-lib.ts";
const spec = process.argv[2]!; const scale = Number(process.argv[3] ?? 50);
const { asset } = entryOf(spec);
const st = restState(asset, scale);
for (const view of ["elevation","front","plan"] as const) {
  console.log("==", view);
  for (const e of st[view]) {
    const pts = e.rings.flat(); if (!pts.length) { console.log(" ", e.shape.id, e.shape.role, "empty"); continue; }
    const b = bboxOf(pts);
    console.log(" ", String(e.shape.id).padEnd(22), String(e.shape.role).padEnd(8), e.closed?"C":"o", String(e.shape.bone??e.shape.solid?.bone??"").padEnd(12), `x ${b.minX.toFixed(3)}..${b.maxX.toFixed(3)} y ${b.minY.toFixed(3)}..${b.maxY.toFixed(3)}`);
  }
}
