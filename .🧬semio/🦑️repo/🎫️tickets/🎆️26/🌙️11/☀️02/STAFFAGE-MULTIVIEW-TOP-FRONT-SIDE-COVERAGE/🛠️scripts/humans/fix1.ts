import { load, save } from "./lib.ts";
const find = (a: any, k: string, v: string, id: string) => a.representations[k].geometry[v].find((s: any) => s.id === id);
const r = (n: number) => +n.toFixed(5);
{
  const d = load("kneeling-gardener"); const s = find(d.asset, "100", "front", "hat-brim");
  s.points = s.points.map((p: number[]) => [p[0], p[1] === 0.2767 ? 0.235 : p[1]]); save("kneeling-gardener", d);
}
{
  const d = load("person-cooking"); const a = d.asset; const g = a.representations["500"].geometry.plan;
  const cap = (id: string, bone: string, y: number, r0: number, r1: number, z: number, tone: number, region: string) => ({ kind: "capsule", x0: 0, y0: y, x1: 0, y1: y, r0, r1, followTip: true, id, bone, role: "body", z, filled: true, tone, region });
  for (const [side, y] of [["l", 0.1859], ["r", -0.1859]] as const) {
    g.push(cap(`arm-${side}`, `arm-${side}`, y, 0.05, 0.05, 3.5, 0.45, "hoodie"));
    g.push(cap(`forearm-${side}`, `forearm-${side}`, y, 0.05, 0.045, 3.6, 0.45, "hoodie"));
    g.push(cap(`hand-${side}`, `hand-${side}`, y, 0.04, 0.035, 3.7, 0.3, "hand-skin"));
  }
  save("person-cooking", d);
}
{ const d = load("person-with-rollator"); d.asset.dimensions.heightM = 1.59; save("person-with-rollator", d); }
{ const d = load("person-with-umbrella"); d.asset.dimensions.depthM = 1.04; save("person-with-umbrella", d); }
{ const d = load("walking-commuter"); d.asset.dimensions.depthM = 0.52; save("walking-commuter", d); }
{ const d = load("runner-with-blade-prosthesis"); d.asset.dimensions.depthM = 0.31; save("runner-with-blade-prosthesis", d); }
{
  const d = load("person-on-crutches"); const a = d.asset;
  for (const [k, outer, inner, y1] of [["200", 0.325, 0.295, 0.310], ["500", 0.325, 0.273, 0.299]] as const) {
    const t = find(a, k, "front", "crutch-tube-l"); t.points[2][0] = outer; t.points[3][0] = inner;
    const u = find(a, k, "front", "crutch-tube-r"); u.points[2][0] = -inner; u.points[3][0] = -outer;
    const pl = find(a, k, "plan", "crutch-tube-l"); pl.y1 = y1; const pr = find(a, k, "plan", "crutch-tube-r"); pr.y1 = -y1;
  }
  save("person-on-crutches", d);
}
{
  const d = load("standing-phone-user"); const s = find(d.asset, "500", "front", "sil-head"); s.cy = r(s.cy + 0.02); s.ry = r(s.ry + 0.02); save("standing-phone-user", d);
}
