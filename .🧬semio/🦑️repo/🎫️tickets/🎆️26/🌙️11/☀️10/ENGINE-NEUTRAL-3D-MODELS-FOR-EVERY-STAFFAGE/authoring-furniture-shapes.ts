import { loadAsset, resolveAsset, restState, bboxOf } from "./volume-lib.ts";
const slug = process.argv[2]!;
const denom = Number(process.argv[3] ?? 50);
const asset = loadAsset(resolveAsset(slug).path);
const st: any = restState(asset, denom);
const f = (x: number) => (Math.round(x * 1000) / 1000).toString();
for (const view of ["elevation", "front", "plan"]) {
  console.log(`== ${view} (paper x,y: ${view === "elevation" ? "x ahead, y up" : view === "front" ? "z left, y up" : "x ahead, z left"})`);
  for (const e of st[view]) {
    const s = e.shape;
    if (!e.rings.length) { console.log(`  ${s.id} [${s.bone}] no rings`); continue; }
    const pts = e.rings.flat();
    const b = bboxOf(pts);
    const extra = e.rings.length === 1 && e.rings[0].length <= 9 ? " pts=" + e.rings[0].map((p: any) => `(${f(p.x)},${f(p.y)})`).join("") : ` rings=${e.rings.length}`;
    console.log(`  ${s.id} [${s.bone}] ${s.kind}${s.solid ? "/" + s.solid.kind : ""} ${s.role} t=${s.tone} ${s.material ?? "-"} ${s.region ?? ""}${e.closed ? "" : " open"} x ${f(b.minX)}..${f(b.maxX)} y ${f(b.minY)}..${f(b.maxY)}${extra}`);
  }
}
