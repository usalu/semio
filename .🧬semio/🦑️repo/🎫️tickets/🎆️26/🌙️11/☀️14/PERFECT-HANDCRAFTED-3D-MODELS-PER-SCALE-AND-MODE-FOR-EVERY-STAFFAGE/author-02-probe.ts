import { entryOf, restState, bboxOf, area } from "./volumes-lib.ts";
const [spec, scaleS] = process.argv.slice(2);
const { asset } = entryOf(spec!);
const st = restState(asset, Number(scaleS));
if (st.degraded) console.log("DEGRADED", st.degraded);
for (const v of ["elevation", "front", "plan"] as const) {
  console.log("==", v);
  for (const e of st[v]) {
    const pts = e.rings.flat();
    const b = pts.length ? bboxOf(pts) : null;
    console.log(e.shape.id.padEnd(22), String(e.shape.bone).padEnd(12), String(e.shape.role).padEnd(7), String(e.shape.region ?? "").padEnd(12), e.closed ? "C" : "o", "rings", e.rings.length, b ? `x ${b.minX.toFixed(2)}..${b.maxX.toFixed(2)} y ${b.minY.toFixed(2)}..${b.maxY.toFixed(2)}` : "", e.rings.length ? "A=" + e.rings.reduce((t, r) => t + area(r), 0).toFixed(2) : "");
  }
}
