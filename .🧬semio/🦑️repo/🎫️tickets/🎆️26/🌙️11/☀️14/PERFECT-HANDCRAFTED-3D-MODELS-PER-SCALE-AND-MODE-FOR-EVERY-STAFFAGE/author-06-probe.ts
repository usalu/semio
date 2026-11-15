import { entryOf, poseState, bboxOf } from "./volumes-lib.ts";
import { CAMERAS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
const [spec, sc] = process.argv.slice(2);
const { asset } = entryOf(spec!);
console.log(JSON.stringify(CAMERAS));
const st = poseState(asset, Number(sc));
const f = (x: number) => (Math.round(x * 1000) / 1000).toFixed(3);
console.log("pose", st.pose, st.degraded);
for (const [k, v] of Object.entries(st.frames)) console.log(k, v.o.map(f).join(","), "u", v.u.map(f).join(","), "w", v.w.map(f).join(","));
for (const v of ["elevation", "front", "plan"] as const) {
  console.log("--", v);
  for (const e of st[v]) {
    const pts = e.rings.flat();
    if (!pts.length) continue;
    const b = bboxOf(pts);
    console.log(` ${e.shape.id} ${e.shape.bone} ${e.shape.kind} ${e.shape.role[0]} closed=${e.closed} rings=${e.rings.length} x[${f(b.minX)},${f(b.maxX)}] y[${f(b.minY)},${f(b.maxY)}]`);
  }
}
