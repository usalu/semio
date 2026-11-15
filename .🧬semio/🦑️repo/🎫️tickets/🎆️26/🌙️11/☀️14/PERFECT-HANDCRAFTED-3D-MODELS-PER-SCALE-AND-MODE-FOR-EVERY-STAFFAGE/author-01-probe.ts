import { loadAsset, resolveAsset, restState, bboxOf } from "./volumes-base.ts";
const slug = process.argv[2]!;
const a = loadAsset(resolveAsset(slug).path);
console.log("dims", JSON.stringify(a.dimensions));
for (const s of [50, 100, 200, 500]) {
  if (a.representations[String(s)].status === "unavailable") { console.log(s, "unavailable"); continue; }
  const st = restState(a, s);
  const line: string[] = [];
  for (const v of ["elevation", "front", "plan"] as const) {
    const pts = st[v].filter((e) => e.closed && ["body"].includes(e.shape.role)).flatMap((e) => e.rings.flat());
    const b = bboxOf(pts);
    line.push(`${v}[${b.minX.toFixed(2)},${b.maxX.toFixed(2)}]x[${b.minY.toFixed(2)},${b.maxY.toFixed(2)}] n=${st[v].filter((e) => e.closed && e.shape.role === "body").length}`);
  }
  console.log(s, line.join("  "), st.degraded ? "DEGRADED " + st.degraded.slice(0, 60) : "");
}
