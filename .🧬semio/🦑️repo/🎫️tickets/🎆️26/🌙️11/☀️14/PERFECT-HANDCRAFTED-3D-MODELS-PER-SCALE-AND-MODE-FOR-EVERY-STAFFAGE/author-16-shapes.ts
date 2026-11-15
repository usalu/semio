import { entryOf, restState, bboxOf, memberBones, shapeBones, frames } from "./volumes-lib.ts";
const [spec, sc] = process.argv.slice(2);
const { asset } = entryOf(spec!);
const scales = sc ? [Number(sc)] : [50, 100, 200, 500];
const f = (x: number) => (Math.round(x * 1000) / 1000).toFixed(3);
const fr = frames(asset);
console.log("bones frames:", Object.entries(fr).map(([k, v]) => `${k}@(${v.o.map(f)}) u(${v.u.map((x) => Math.round(x * 100) / 100)}) w(${v.w.map((x) => Math.round(x * 100) / 100)})`).join("\n  "));
for (const s of scales) {
  const rep = asset.representations[String(s)];
  if (rep.status !== "authored") { console.log(s, "unavailable", rep.limitingFeature ?? ""); continue; }
  console.log(`=== 1:${s} members`, memberBones(asset, s).join(","), "| shapeBones", [...shapeBones(asset, s)].join(","));
  const st = restState(asset, s);
  for (const v of ["elevation", "front", "plan"] as const) {
    console.log(`-- ${v}`);
    for (const e of st[v]) {
      const pts = e.rings.flat();
      if (!pts.length) continue;
      const b = bboxOf(pts);
      console.log(`  ${e.shape.id} bone=${e.shape.bone} ${e.shape.kind} ${e.shape.role} ${e.shape.tone ?? ""} x[${f(b.minX)},${f(b.maxX)}] y[${f(b.minY)},${f(b.maxY)}] n=${pts.length}${e.shape.section ? " sec=" + JSON.stringify(e.shape.section) : ""}`);
    }
  }
}
