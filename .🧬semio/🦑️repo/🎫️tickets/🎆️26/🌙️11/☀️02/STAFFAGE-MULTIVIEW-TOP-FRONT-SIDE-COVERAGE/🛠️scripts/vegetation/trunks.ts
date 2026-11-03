import { bands, load, save, stateAt } from "./lib.ts";
const id = process.argv[2]!; const write = process.argv[3] === "write"; const x = load(id); const a = x.asset;
const r4 = (v: number) => Math.round(v * 10000) / 10000;
const TREES = ["apple-tree", "english-oak", "japanese-maple", "lombardy-poplar", "london-plane-pollarded", "multi-stem-birch", "scots-pine", "small-leaved-lime", "weeping-willow"];
const SINGLE = ["apple-tree", "english-oak", "lombardy-poplar", "london-plane-pollarded", "scots-pine", "small-leaved-lime", "weeping-willow"];
const STEMS: Record<string, string[]> = { "japanese-maple": ["stem-c", "stem-w", "stem-e"], "multi-stem-birch": ["stem-l-1", "stem-c-1", "stem-r-1"], "hornbeam-hedge": ["stem-1", "stem-2", "stem-3", "stem-4", "stem-5", "stem-6"], "bamboo-clump": ["culm-1-a", "culm-2-a", "culm-3-a", "culm-4-a", "culm-5-a", "culm-6-a"] };
const bonesById = new Map<string, any>(a.rig.bones.map((b: any) => [b.id, b]));
const rest = new Map<string, { ox: number; oy: number; ang: number; tx: number; ty: number }>();
const fk = (b: any): { ox: number; oy: number; ang: number; tx: number; ty: number } => {
  if (rest.has(b.id)) return rest.get(b.id)!;
  let ox = b.x, oy = b.y, ang = b.angleDeg;
  if (b.parent) { const p = fk(bonesById.get(b.parent)); const c = Math.cos((p.ang * Math.PI) / 180), s = Math.sin((p.ang * Math.PI) / 180); ox = p.ox + c * b.x - s * b.y; oy = p.oy + s * b.x + c * b.y; ang = p.ang + b.angleDeg; }
  const r = { ox, oy, ang, tx: ox + b.lengthM * Math.cos((ang * Math.PI) / 180), ty: oy + b.lengthM * Math.sin((ang * Math.PI) / 180) };
  rest.set(b.id, r); return r;
};
const out: string[] = [];
for (const den of bands(a)) {
  const g = a.representations[String(den)].geometry; const st = stateAt(a, den);
  const zTop = Math.max(...g.plan.map((s: any) => s.z)) + 1;
  const regionOf = (bone: string) => g.elevation.find((s: any) => s.bone === bone && s.region)?.region;
  const frontBox = (bone: string) => { const pts = st.shapes.front.filter((s: any) => s.shape.bone === bone && s.shape.role !== "ground").flatMap((s: any) => s.rings.flat()); return pts.length ? { lo: Math.min(...pts.map((p: any) => p.x)), hi: Math.max(...pts.map((p: any) => p.x)), base: pts.filter((p: any) => p.y < Math.min(...pts.map((q: any) => q.y)) + 0.4) } : undefined; };
  const have = new Set(g.plan.map((s: any) => s.id));
  const add = (s: any) => { if (!have.has(s.id)) { g.plan.push(s); have.add(s.id); out.push(`${den} +${s.id}`); } };
  const dot = (sid: string, bone: string, cx: number, cy: number, r: number, region: string | undefined, minDia: number) => {
    if (r * 2 < minDia) return; add({ id: sid, bone, role: "body", z: r4(zTop), filled: true, tone: 0.25, ...(region ? { region } : {}), kind: "ellipse", cx: r4(cx), cy: r4(cy), rx: r4(r), ry: r4(r) });
  };
  const minDia = den * 0.0014;
  if (SINGLE.includes(id)) {
    const tb = a.rig.bones.find((b: any) => /^trunk(-1)?$/.test(b.id)); const r = tb.section.extremeFibreM;
    dot("trunk-dot", a.rig.root, 0, 0, r, regionOf(tb.id) ?? regionOf(a.rig.root), minDia);
  } else if (STEMS[id] && den <= 100 + (id === "bamboo-clump" || id === "hornbeam-hedge" ? -50 : 0)) {
    for (const sb of STEMS[id]!) {
      const b = bonesById.get(sb); const r = b.section?.extremeFibreM ?? 0.03; const f0 = fk(b); const fb = frontBox(sb);
      const lat = fb ? fb.base.reduce((s: number, p: any) => s + p.x, 0) / fb.base.length : 0;
      dot(`${sb}-dot`, sb, f0.ox, lat, r, regionOf(sb), minDia);
    }
  }
  if (TREES.includes(id) && den <= 100) {
    for (const b of a.rig.bones) {
      const reg = g.elevation.find((s: any) => s.bone === b.id && s.role === "body" && /bark|stem|trunk|pollard/.test(s.region ?? ""))?.region;
      if (!reg || /^(root|trunk|stem)/.test(b.id) || b.lengthM < 1.2 || b.id === "leader") continue;
      if (g.plan.some((s: any) => s.bone === b.id && /bark/.test(s.region ?? ""))) continue;
      const f = fk(b); const fb = frontBox(b.id); if (!fb) continue;
      let o = 0; if (b.parent && bonesById.get(b.parent) && !/^(root|trunk|stem)/.test(b.parent)) { const pb = frontBox(b.parent); const pr = bonesById.get(b.parent); if (pb) { const pf = fk(pr); const po = Math.abs(pb.hi) > Math.abs(pb.lo) ? pb.hi : pb.lo; o = Math.abs(po) > 0.01 ? po : 0; void pf; } }
      const tip = Math.abs(fb.hi - o) >= Math.abs(fb.lo - o) ? fb.hi : fb.lo;
      const r = Math.max(0.05, (b.section?.extremeFibreM ?? 0.1) * 0.6);
      add({ id: `plan-${b.id}`, bone: b.id, role: "detail", z: r4(zTop - 0.5), filled: true, tone: 0.35, region: reg, kind: "capsule", x0: r4(f.ox), y0: r4(o), x1: r4(f.tx), y1: r4(tip), r0: r4(r), r1: r4(r * 0.55), followTip: true });
    }
  }
}
console.log(id, out.length, out.join(" "));
if (write) save(x);
