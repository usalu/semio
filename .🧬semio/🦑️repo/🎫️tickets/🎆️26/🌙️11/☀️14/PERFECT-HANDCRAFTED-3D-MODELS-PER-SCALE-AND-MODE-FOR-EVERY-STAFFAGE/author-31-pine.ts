/** 🌳️ Scots pine: per-scale handcrafted model. Usage: bun author-31-pine.ts <scale|all> */
import { readFileSync, writeFileSync } from "node:fs";
import { Kit, bboxOfShapes, ivalsAll, type Sec } from "./author-02-lib.ts";
import { emitterPatches, limb3, localRadii, treeStructure, zFromPlan } from "./author-31-lib.ts";
import { TICKET } from "./volumes-lib.ts";
import { limbTube } from "./author-67-lib.ts";

const dilateX = (S: any[], d: number) => (d ? [...S, ...S.map((sh) => sh.map((r: any[]) => r.map((p) => ({ x: p.x + d, y: p.y })))), ...S.map((sh) => sh.map((r: any[]) => r.map((p) => ({ x: p.x - d, y: p.y }))))] : S);
const SPEC = "vegetation/scots-pine";
const FILE = `${TICKET}/volumes/${SPEC}.json`;
const arg = process.argv[2] ?? "all";
const scales = arg === "all" ? [50, 100, 200, 500] : [Number(arg)];
const src = JSON.parse(readFileSync(FILE, "utf8"));

for (const S of scales) {
  const k = new Kit(SPEC, S);
  const regions: Record<string, any> = Object.fromEntries(k.asset.thermal.regions.map((r: any) => [r.id, r]));
  const isFol = (s: any) => regions[s.region]?.kind === "foliage";
  const paths = new Map<string, Sec[]>();
  const regionOf = new Map<string, string>();
  const barkOf = (id: string, bone: string, region: string, tone?: number) => {
    const E = k.E((s) => s.id === id);
    const F = k.F((s) => s.bone === bone && s.role === "body" && !isFol(s));
    const P = k.P((s) => s.bone === bone && !isFol(s) && /^(plan-|stem|head|trunk-dot)/.test(s.id));
    return { E, F, P, m: k.metaOf("elevation", (s) => s.id === id, tone === undefined ? {} : { tone }) };
  };
  void barkOf;
  const ordered = k.shapes("elevation", (s) => s.role === "body" && !isFol(s) && !/crown|plug|^stems$/.test(s.id));
  const parent = (b: string) => k.boneDef(b).parent as string;
  const zEnd = new Map<string, number>();
  const n = S === 50 ? 7 : S === 100 ? 5 : 4;
  const elevBones = new Set(ordered.map((e) => e.shape.bone as string));
  const frontBones = [...new Set(k.shapes("front", (s) => !isFol(s) && s.role === "body").map((e) => e.shape.bone as string))];
  for (const e of ordered) {
    const id = e.shape.id as string, bone = e.shape.bone as string, region = e.shape.region as string;
    const E = k.E((s) => s.id === id);
    const eb = bboxOfShapes(E);
    const kids: string[] = [];
    void frontBones;
    const m = k.metaOf("elevation", (s) => s.id === id);
    const parts: { b: string; id: string; lo: number; hi: number }[] = [];
    const cut = kids.length ? k.origin(kids[0]!)[1] : eb.maxY;
    parts.push({ b: bone, id, lo: eb.minY, hi: kids.length ? cut + 0.04 : eb.maxY });
    for (const kd of kids) parts.push({ b: kd, id: `${id}-${kd}`, lo: cut - 0.04, hi: eb.maxY });
    for (const pt of parts) {
      const Pl = k.P((s) => s.bone === pt.b && /^plan-limb/.test(s.id));
      const zp = zFromPlan(Pl.length ? Pl : null);
      const trunk = /^(trunk|leader)/.test(pt.b);
      const Ft: any[] = /^(stub|limb)/.test(pt.b) ? k.F((s) => s.id === pt.b && s.kind === "poly") : /^(trunk|leader|root-plate)/.test(pt.b) ? k.F((s) => s.id === "bole-lower" || s.id === "bole-upper") : [];
      void trunk;
      if (/^(stub|limb)/.test(pt.b)) { paths.set(pt.b, limbTube(k, pt.id, pt.b, E, Ft, m)); regionOf.set(pt.id, region); continue; }
      const secs = limb3(k, pt.id, pt.b, E, m, { n, adaptive: k.feat * 0.5, round: /^(stub|limb)/.test(pt.b), exp: 2, axis: pt.b === "root-plate" || /^(trunk|leader)/.test(pt.b) ? "y" : undefined, F: Ft.length ? Ft : null, zOf: Ft.length && /^(stub|limb)/.test(pt.b) ? undefined : zp ?? undefined, yRange: [pt.lo, pt.hi] });
      paths.set(pt.b, secs);
      regionOf.set(pt.id, region);
    }
  }
  const boneHas = new Set(k.body.map((s) => s.bone as string));
  const rp = k.shapes("plan", (s) => s.bone === "root-plate");
  if (rp.length && !boneHas.has("root-plate")) {
    const r = bboxOfShapes(k.P((s) => s.bone === "root-plate"));
    const rr = (r.maxX - r.minX) / 2;
    const rm = k.metaOf("plan", (s) => s.bone === "root-plate");
    k.loft("body", "root-plate", "root-plate", [k.secY([0, 0, 0], rr * 1.3, rr * 1.3, 2), k.secY([0, 0.2, 0], rr * 1.2, rr * 1.2, 2), k.secY([0, 0.45, 0], rr * 1.05, rr * 1.05, 2)], rm);
    regionOf.set("root-plate", rp[0]!.shape.region);
    boneHas.add("root-plate");
  }
  const EB = k.E((s) => s.role === "body"), FB = k.F((s) => s.role === "body");
  const insideDrawn = (q: [number, number, number]) => ivalsAll(EB, "x", q[0]).some((v) => q[1] >= v[0] + 0.05 && q[1] <= v[1] - 0.05) && ivalsAll(FB, "x", q[2]).some((v) => q[1] >= v[0] + 0.05 && q[1] <= v[1] - 0.05);
  const groups: { id: string; bone: string; c: [number, number, number]; h: [number, number, number] }[] = [];
  const lobeN = S === 50 ? 9 : S === 100 ? 7 : 5;
  const hull200 = false;
  const fronts = hull200 ? [] : k.shapes("front", (s) => isFol(s) && /^f-/.test(s.id));
  for (const f of fronts) {
    const key = (f.shape.id as string).replace(/^f-/, "");
    const Pk = k.P((s) => s.id === `p-${key}`);
    if (!Pk.length) continue;
    const Fk = k.F((s) => s.id === f.shape.id);
    const m = k.metaOf("front", (s) => s.id === f.shape.id);
    const pbx = bboxOfShapes(Pk);
    const outer = Math.max(Math.abs(pbx.minX), Math.abs(pbx.maxX)) >= 5.0 || Math.max(Math.abs(pbx.minY), Math.abs(pbx.maxY)) >= 3.6;
    const lid = `lobe-${outer ? "out" : "in"}-${key}`;
    const Em = k.E((s) => isFol(s));
    const before = k.body.length;
    const pb0 = bboxOfShapes(Pk), fb0 = bboxOfShapes(Fk);
    let inE = 0, tot = 0;
    for (let a = 0; a < 12; a++) for (let b = 0; b < 12; b++) {
      const u = -1 + (2 * (a + 0.5)) / 12, w = -1 + (2 * (b + 0.5)) / 12;
      if (u * u + w * w > 1) continue;
      const x = (pb0.minX + pb0.maxX) / 2 + (u * (pb0.maxX - pb0.minX)) / 2, y = (fb0.minY + fb0.maxY) / 2 + (w * (fb0.maxY - fb0.minY)) / 2;
      tot++;
      if (ivalsAll(Em, "x", x).some((v) => y >= v[0] && y <= v[1])) inE++;
    }
    if (inE / tot >= Number(process.env.LOBECOV ?? 0.8)) { if (process.env.SLABS === "1") k.lobeSlabs(lid, f.shape.bone, Pk, Fk, m, {}); else k.lobe(lid, f.shape.bone, Pk, Fk, m, { n: S === 50 ? 9 : S === 100 ? 7 : 5 }); }
    else k.hull(lid, f.shape.bone, m, { E: dilateX(Em, Number(process.env.HULLDX ?? 0)), F: Fk, P: Pk, step: S === 50 ? 0.3 : S === 100 ? 0.45 : 0.6, expLo: 2, expHi: 4, minArea: (2 * k.feat) ** 2 });
    if (k.body.length > before) {
      for (const sol of k.body.slice(before)) regionOf.set(sol.id, f.shape.region);
      const pb = bboxOfShapes(Pk), fb = bboxOfShapes(Fk);
      groups.push({ id: key, bone: f.shape.bone, c: [(pb.minX + pb.maxX) / 2, (fb.minY + fb.maxY) / 2, (pb.minY + pb.maxY) / 2], h: [(pb.maxX - pb.minX) / 2, (fb.maxY - fb.minY) / 2, (pb.maxY - pb.minY) / 2] });
      boneHas.add(f.shape.bone);
    }
  }
  for (const p of hull200 || (S === 200 && process.env.FILL200 !== "1") ? [] : k.shapes("plan", (s) => isFol(s) && /^crown-fill/.test(s.id))) {
    const id = p.shape.id as string;
    const Pf = k.P((s) => s.id === id);
    const pb = bboxOfShapes(Pf);
    const Fall = k.F((s) => isFol(s));
    const Eall = k.E((s) => isFol(s) && /mass/.test(s.id));
    const xs: number[] = Array.from({ length: 9 }, (_, i) => pb.minX + (pb.maxX - pb.minX) * (0.1 + (0.8 * i) / 8));
    const zs: number[] = Array.from({ length: 9 }, (_, i) => pb.minY + (pb.maxY - pb.minY) * (0.1 + (0.8 * i) / 8));
    let ylo = -Infinity, yhi = Infinity;
    for (const z of zs) { const iv = ivalsAll(Fall, "x", z); if (iv.length) { ylo = Math.max(ylo, iv[0]![0]); yhi = Math.min(yhi, iv[iv.length - 1]![1]); } }
    for (const x of xs) { const iv = ivalsAll(Eall, "x", x); if (iv.length) { ylo = Math.max(ylo, iv[0]![0]); yhi = Math.min(yhi, iv[iv.length - 1]![1]); } }
    const own = bboxOfShapes(k.E((s) => s.bone === p.shape.bone && isFol(s)));
    const oh = own.maxY - own.minY;
    const mass = S === 200 ? { minY: ylo, maxY: yhi } : { minY: own.minY + 0.15 * oh, maxY: own.maxY - 0.25 * oh };
    const h = mass.maxY - mass.minY;
    const m = k.metaOf("plan", (s) => s.id === id);
    if (k.lobe(`fill-${id}`, p.shape.bone, k.P((s) => s.id === id), null, m, { n: lobeN, yRange: [mass.minY + 0.05 * h, mass.maxY - 0.05 * h] })) regionOf.set(`fill-${id}`, p.shape.region);
  }
  if (hull200) {
    const m = k.metaOf("front", (s) => isFol(s));
    k.hull("crown", "fol-core", m, { E: k.E((s) => isFol(s)), F: k.F((s) => isFol(s)), P: k.P((s) => isFol(s)), step: 0.25, expLo: 2.2, expHi: 4 });
    for (const sol of k.body) if (sol.id.startsWith("crown")) regionOf.set(sol.id, "needles-sun");
  }
  const Ec = k.shapes("elevation", (s) => /crown/.test(s.id));
  if (Ec.length) {
    const id = Ec[0]!.shape.id as string, bone = Ec[0]!.shape.bone as string;
    const Pc = k.P((s) => s.id === id), Fc = k.F((s) => s.id === id), Ee = k.E((s) => s.id === id);
    const m = k.metaOf("elevation", (s) => s.id === id, { material: regions["needles-sun"].material });
    const made = k.hull("crown", bone, m, { E: Ee, F: Fc, P: Pc, step: 0.45, expLo: 2.2, expHi: 4 });
    for (const s of k.body) if (s.id.startsWith("crown")) regionOf.set(s.id, "needles-sun");
    if (made) { const pb = bboxOfShapes(Pc), fb = bboxOfShapes(Fc); groups.push({ id: "crown", bone, c: [(pb.minX + pb.maxX) / 2, (fb.minY + fb.maxY) / 2, (pb.minY + pb.maxY) / 2], h: [(pb.maxX - pb.minX) / 2, (fb.maxY - fb.minY) / 2, (pb.maxY - pb.minY) / 2] }); }
  }
  void boneHas;

  for (const s of k.body) {
    const r = regionOf.get(s.id);
    if (r) k.thermal.push({ id: `th-${s.id}`, of: s.id, region: r });
  }
  if (S === 500) {
    for (const b of k.asset.rig.bones.map((x: any) => x.id).filter((x: string) => /^limb-[we]1$/.test(x))) {
      const a = k.origin(b), t = k.tip(b);
      const pts = Array.from({ length: 41 }, (_, i) => [a[0] + ((t[0] - a[0]) * i) / 40, a[1] + ((t[1] - a[1]) * i) / 40, a[2] + ((t[2] - a[2]) * i) / 40] as [number, number, number]);
      const ins = pts.map((q) => insideDrawn(q));
      const i0 = ins.indexOf(true), i1 = ins.lastIndexOf(true);
      if (i0 < 0 || i1 - i0 < 2) continue;
      k.cap("thermal", `th-limb-${b}`, b, pts[i0]!, pts[i1]!, 0.06, 0.06, { material: regions["limb-bark"].material, role: "body", tone: 0.35 }, { region: "limb-bark" });
    }
  }
  if (S === 500) k.ell("thermal", "th-shade-core", "trunk-3", [0, 15.2, 0], localRadii(k, "trunk-3", [2.4, 1.4, 1.9]), { material: regions["needles-shade"].material, role: "body", tone: 0.4 }, { region: "needles-shade" });
  emitterPatches(k, () => undefined);
  const byBone = new Map<string, typeof groups>();
  for (const g of groups) (byBone.get(g.bone) ?? byBone.set(g.bone, []).get(g.bone)!).push(g);
  const masses = [...byBone.entries()].map(([bone, gs]) => {
    const g = gs.reduce((a, b) => (a.h[0] * a.h[2] >= b.h[0] * b.h[2] ? a : b));
    return { id: bone, bone, c: g.c, h: g.h };
  });
  const inside = (q: [number, number, number]) => { const iv = (S: any, a: number, b: number) => S.some((r: any) => r); void iv; return true; };
  void inside;
  treeStructure(k, paths, { clip: (q) => insideDrawn(q), skipJoint: /^pl-/, jointK: 0.75,  supportBox: [0.05, Math.max(0.025, k.feat * 0.55), 0.05], core: 0.62, joints: S <= 100, masses: S === 500 ? masses.slice(0, 1) : masses, massK: 0.28 });
  const vol = k.volume() as any;
  vol.decisions = src.volumes[S]?.decisions?.length && src.volumes[S].decisions[0].en?.startsWith("PLANE") ? src.volumes[S].decisions : [{ en: "TODO", de: "TODO" }];
  src.volumes[S] = vol;
  console.log(S, vol.body.length, vol.thermal.length, vol.structure.length);
}
writeFileSync(FILE, `${JSON.stringify(src, null, 1)}\n`);
