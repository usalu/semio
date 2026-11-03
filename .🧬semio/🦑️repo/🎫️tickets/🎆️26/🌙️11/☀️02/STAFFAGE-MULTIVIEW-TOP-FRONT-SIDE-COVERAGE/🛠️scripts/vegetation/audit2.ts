import { readdirSync, readFileSync, writeFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const g = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const { extentOf, toleranceM, queries, authoredScales, consistency, sweep } = g;
const root = `${L}/🗂️catalog/🌳️vegetation`;
const only = process.argv[2];
const f = (n: number) => n.toFixed(2);
const out: string[] = [];
const V = ["elevation", "front", "plan"] as const;
for (const d of readdirSync(root)) {
  if (only && only !== "all" && d !== only) continue;
  const a = JSON.parse(readFileSync(`${root}/${d}/🔣️.json`, "utf8")).asset;
  const D = a.dimensions;
  out.push(`## ${a.id} H${D.heightM} W${D.widthM} D${D.depthM} tol ${f(toleranceM(a))}`);
  for (const den of authoredScales(a)) {
    const fin = [...consistency(a, den), ...sweep(a, den)];
    for (const x of fin) out.push(`  FINDING 1:${den} ${x.check}: ${x.message}`);
    const rep = a.representations[String(den)];
    const regs: Record<string, Set<string>> = {}, bones: Record<string, Set<string>> = {};
    for (const v of V) { regs[v] = new Set(rep.geometry[v].filter((s: any) => s.role !== "ground").map((s: any) => s.region ?? "-")); bones[v] = new Set(rep.geometry[v].map((s: any) => s.bone)); }
    for (const v of ["front", "plan"] as const) {
      const rm = [...regs.elevation].filter((r) => !regs[v].has(r)), bm = [...bones.elevation].filter((b) => !bones[v].has(b));
      if (rm.length || bm.length) out.push(`  PARITY 1:${den} ${v} lacks regions [${rm}] bones [${bm}]`);
    }
    const ex = (v: string, q: any) => extentOf(evaluate(a, q, { kind: "scale", scale: { denominator: den } }).shapes[v]);
    const lat = { fx0: 1e9, fx1: -1e9, py0: 1e9, py1: -1e9, px0: 1e9, px1: -1e9, fy0: 1e9, fy1: -1e9 };
    for (const { query } of queries(a)) {
      const fr = ex("front", query), pl = ex("plan", query);
      lat.fx0 = Math.min(lat.fx0, fr.minX); lat.fx1 = Math.max(lat.fx1, fr.maxX); lat.fy0 = Math.min(lat.fy0, fr.maxY); lat.fy1 = Math.max(lat.fy1, fr.maxY);
      lat.py0 = Math.min(lat.py0, pl.minY); lat.py1 = Math.max(lat.py1, pl.maxY); lat.px0 = Math.min(lat.px0, pl.minX); lat.px1 = Math.max(lat.px1, pl.maxX);
    }
    out.push(`  1:${den} el ${rep.geometry.elevation.length} fr ${rep.geometry.front.length} pl ${rep.geometry.plan.length} | over all poses/clips: front x [${f(lat.fx0)},${f(lat.fx1)}] top y ${f(lat.fy0)}..${f(lat.fy1)} | plan x [${f(lat.px0)},${f(lat.px1)}] y [${f(lat.py0)},${f(lat.py1)}]`);
  }
  for (const s of a.symbols) {
    const sel = { kind: "symbol", symbol: s.id };
    const q = { kind: "pose", pose: a.presentation.defaultPose };
    const st = evaluate(a, q, sel);
    const r = (v: string) => { const e = extentOf(st.shapes[v]); return e ? `[${f(e.minX)},${f(e.maxX)}]x[${f(e.minY)},${f(e.maxY)}]` : "NONE"; };
    out.push(`  SYMBOL ${s.id} ${s.semantics}/${s.units} den ${s.denominators}: el ${r("elevation")} fr ${r("front")} pl ${r("plan")}`);
  }
}
writeFileSync(process.argv[3], out.join("\n"));
