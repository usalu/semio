import { writeFileSync, mkdirSync } from "node:fs";
import { catalog, authoredScales, queries, extentOf, toleranceM, VIEWS, type Extent } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [, , out, ...ids] = process.argv;
const assets = catalog().filter((a) => ids.includes(a.id));
const lines: string[] = [];
const log = (s: string) => { lines.push(s); console.log(s); };
const sel = (d: number) => ({ kind: "scale", scale: { denominator: d } }) as const;
const bbox = (shapes: any[], bone: string, ground = false): Extent | undefined => {
  const pts = shapes.filter((s) => s.shape.bone === bone && (ground || s.shape.role !== "ground")).flatMap((s) => s.rings.flat());
  if (!pts.length) return undefined;
  const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
  return { minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) };
};
for (const a of assets) {
  const tol = toleranceM(a);
  const dims = a.dimensions;
  for (const d of authoredScales(a)) {
    const rest = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, sel(d));
    const e = extentOf(rest.shapes.elevation)!, f = extentOf(rest.shapes.front)!, p = extentOf(rest.shapes.plan)!;
    const issues: string[] = [];
    const chk = (n: string, m: number, nom: number) => { if (Math.abs(m - nom) > tol) issues.push(`${n}=${m.toFixed(3)} vs ${nom}`); };
    chk("el-w", e.maxX - e.minX, dims.widthM); chk("el-h", e.maxY - e.minY, dims.heightM);
    chk("fr-d", f.maxX - f.minX, dims.depthM); chk("fr-h", f.maxY - f.minY, dims.heightM);
    chk("pl-w", p.maxX - p.minX, dims.widthM); chk("pl-d", p.maxY - p.minY, dims.depthM);
    const base = rest;
    const boneIds = a.rig.bones.map((b) => b.id);
    const baseBox = Object.fromEntries(boneIds.map((b) => [b, [bbox(base.shapes.elevation, b), bbox(base.shapes.front, b), bbox(base.shapes.plan, b)]]));
    const bad = new Map<string, number>();
    const asym = new Map<string, string>();
    for (const { name, query } of queries(a)) {
      let st: any;
      try { st = evaluate(a, query, sel(d)); } catch (err) { issues.push(`${name} eval fail ${(err as Error).message}`); continue; }
      const ee = extentOf(st.shapes.elevation), ff = extentOf(st.shapes.front), pp = extentOf(st.shapes.plan);
      if (!ee || !ff || !pp) { issues.push(`${name} empty view`); continue; }
      const tl = tol * 1.5;
      if (Math.abs(ee.minX - pp.minX) > tl || Math.abs(ee.maxX - pp.maxX) > tl) bad.set(`${name}:X el[${ee.minX.toFixed(2)},${ee.maxX.toFixed(2)}] pl[${pp.minX.toFixed(2)},${pp.maxX.toFixed(2)}]`, 1);
      if (Math.abs(ee.minY - ff.minY) > tl || Math.abs(ee.maxY - ff.maxY) > tl) bad.set(`${name}:Y el[${ee.minY.toFixed(2)},${ee.maxY.toFixed(2)}] fr[${ff.minY.toFixed(2)},${ff.maxY.toFixed(2)}]`, 1);
      if (Math.abs(ff.minX - pp.minY) > tl || Math.abs(ff.maxX - pp.maxY) > tl) bad.set(`${name}:Z fr[${ff.minX.toFixed(2)},${ff.maxX.toFixed(2)}] pl[${pp.minY.toFixed(2)},${pp.maxY.toFixed(2)}]`, 1);
      for (const b of boneIds) {
        const be = bbox(st.shapes.elevation, b), bf = bbox(st.shapes.front, b), bp = bbox(st.shapes.plan, b);
        const [re, rf, rp] = baseBox[b] as (Extent | undefined)[];
        if (be && bp && re && rp) {
          const dxE = (be.minX + be.maxX) / 2 - (re.minX + re.maxX) / 2, dxP = (bp.minX + bp.maxX) / 2 - (rp.minX + rp.maxX) / 2;
          if (Math.abs(dxE - dxP) > 0.03) bad.set(`bone ${b} x-travel el ${dxE.toFixed(2)} pl ${dxP.toFixed(2)}`, (bad.get(`bone ${b} x-travel el ${dxE.toFixed(2)} pl ${dxP.toFixed(2)}`) ?? 0) + 1);
        }
        if (be && bf && re && rf) {
          const dyE = (be.minY + be.maxY) / 2 - (re.minY + re.maxY) / 2, dyF = (bf.minY + bf.maxY) / 2 - (rf.minY + rf.maxY) / 2;
          if (Math.abs(dyE - dyF) > 0.03) bad.set(`bone ${b} y-travel el ${dyE.toFixed(2)} fr ${dyF.toFixed(2)}`, 1);
        }
      }
    }
    const grouped = new Map<string, number>();
    for (const k of bad.keys()) { const g = k.split(" ").slice(0, 2).join(" "); grouped.set(g, (grouped.get(g) ?? 0) + 1); }
    log(`${a.id} 1:${d} dims-issues=[${issues.join("; ")}] follow-issues=${bad.size} {${[...grouped].map(([k, v]) => `${k}x${v}`).join(", ")}}`);
    if (process.env.VERBOSE) for (const k of bad.keys()) log("   " + k);
  }
}
mkdirSync(out, { recursive: true });
writeFileSync(`${out}/audit.txt`, lines.join("\n"));
