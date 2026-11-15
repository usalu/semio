/** 🌳️ Author 61: small-leaved lime, per-scale handcrafted model. Stem and leader are lofts along the drawn elevation outlines with depth from the front outline, laterals and epicormic stems are 3D tubes whose z follows the plan strip; every drawn crown sheet (ring lobes, cores, top, epicormic masses) is one spindle loft whose section is the oriented superellipse fitted to its own plan footprint, scaled over the height by its own front width profile; the plan-only crown fill is a stack of exact plan frusta; the 1:500 crown is a stack of prisms cut from the one drawn crown outline by the elevation and front intervals. Usage: `bun author-61-lime.ts <scale|all>`.
 * @see ./author-61.md */
import { readFileSync, writeFileSync } from "node:fs";
import { Kit, bboxOfShapes, ivalsAll, type Sec } from "./author-02-lib.ts";
import { emitterPatches, limb3, treeStructure } from "./author-31-lib.ts";
import { TICKET } from "./volumes-lib.ts";
import { centreline, hullLobes, hullSlabs, insideRings, limbTube, sheet, subPath, tubeSections } from "./author-61-lib.ts";

const SPEC = "vegetation/small-leaved-lime";
const FILE = `${TICKET}/volumes/${SPEC}.json`;
const arg = process.argv[2] ?? "all";
const scales = arg === "all" ? [50, 100, 200, 500] : [Number(arg)];
const src = JSON.parse(readFileSync(FILE, "utf8"));
const env = (name: string, d: number): number => Number(process.env[name] ?? d);

for (const S of scales) {
  const k = new Kit(SPEC, S);
  const regions: Record<string, any> = Object.fromEntries(k.asset.thermal.regions.map((r: any) => [r.id, r]));
  const isFol = (s: any) => regions[s.region]?.kind === "foliage";
  const paths = new Map<string, Sec[]>();
  const regionOf = new Map<string, string>();
  const planAll = k.P((s) => s.role === "body").flatMap((s) => s);
  const inPlan = (x: number, z: number): boolean => insideRings(planAll, x, z) && insideRings(planAll, x, z + 0.1) && insideRings(planAll, x, z - 0.1);
  const bark = k.shapes("elevation", (s) => s.role === "body" && !isFol(s) && /^(stem|wood-|epicormic)/.test(s.id));
  for (const e of bark) {
    const id = e.shape.id as string, bone = e.shape.bone as string;
    const region = (e.shape.region as string | undefined) ?? (id === "epicormic" ? "epicormic-stem" : undefined);
    const E = k.E((s) => s.id === id);
    const F = k.F((s) => s.bone === bone && s.role === "body" && !isFol(s) && !/^f-/.test(s.id));
    const Pl = k.P((s) => s.bone === bone && s.role !== "body" && /^plan-/.test(s.id));
    const m = k.metaOf("elevation", (s) => s.id === id, e.shape.region === undefined ? { material: regions["leaf-epicormic"].material } : {});
    const tube = /^(wood-l[we]|wood-epi|epicormic)/.test(id);
    const secs = tube
      ? limbTube(k, id, bone, E, F, m, { n: env("LIMBN", S === 50 ? 12 : 6), P: Pl.length ? Pl : undefined, zInside: id === "epicormic" ? undefined : (x, z) => inPlan(x, z) })
      : limb3(k, id, bone, E, m, { n: S === 50 ? 7 : 5, adaptive: k.feat * 0.5, exp: 2, F: F.length ? F : null });
    paths.set(bone, secs);
    if (region !== undefined) regionOf.set(id, region);
  }
  for (const e of k.shapes("front", (s) => s.role === "body" && !isFol(s) && /^epi-[ew]$/.test(s.id) && !paths.has(s.bone))) {
    const id = `wood-${e.shape.id as string}`, bone = e.shape.bone as string;
    const F = k.F((s) => s.id === e.shape.id);
    const f = k.fr[bone]!, L = k.boneDef(bone).lengthM as number;
    const cf = centreline(F, { x: 0, y: f.o[1] }, 6);
    const path = cf.map((c) => { const t = Math.min(1, Math.max(0, (c.c.y - f.o[1]) / (f.u[1] * L || 1))); return [f.o[0] + f.u[0] * L * t, c.c.y, c.c.x] as [number, number, number]; });
    const secs = tubeSections(path, cf.map((c) => Math.max(c.w / 2, k.feat * 0.55)));
    k.loft("body", id, bone, secs, k.metaOf("front", (s) => s.id === e.shape.id));
    paths.set(bone, secs);
    regionOf.set(id, e.shape.region as string);
  }
  if (!k.body.some((s) => s.bone === "root-plate")) {
    const mr = k.metaOf("plan", (s) => s.id === "trunk-dot", { material: regions["trunk-bark"].material });
    const r = bboxOfShapes(k.P((s) => s.id === "trunk-dot"));
    const rr = (r.maxX - r.minX) / 2;
    k.loft("body", "root-plate", "root-plate", [k.secY([0, 0.0, 0], rr, rr, 2), k.secY([0, 0.3, 0], rr, rr, 2)], mr);
    regionOf.set("root-plate", "trunk-bark");
  }
  const lobeN = env("LOBEN", S === 50 ? 11 : S === 100 ? 9 : 7);
  const fronts = k.shapes("front", (s) => isFol(s) && /^f-/.test(s.id));
  for (const f of fronts) {
    const key = (f.shape.id as string).replace(/^f-/, "");
    const Pk = k.P((s) => s.id === `p-${key}`);
    if (!Pk.length) continue;
    const Fk = k.F((s) => s.id === f.shape.id);
    const m = k.metaOf("front", (s) => s.id === f.shape.id);
    const Em = k.E((s) => s.id === `${f.shape.bone}-mass` || (/^(core-low|core-high|top)$/.test(key) && s.id === `fol-${key}-mass`));
    const pbx = bboxOfShapes(Pk);
    const cell = Math.max(pbx.maxX - pbx.minX, pbx.maxY - pbx.minY) / env("SHEETCELLS", 28);
    const made = process.env.HULLSHEETS !== "0" && Em.length ? hullLobes(k, `lobe-${key}`, f.shape.bone, Em, Fk, Pk, m, { stations: lobeN, splits: [], overlap: 0, cell, recall: env("SHEETRECALL", 3) }) : sheet(k, `lobe-${key}`, f.shape.bone, Pk, Fk, m, { n: lobeN }) === null ? 0 : 1;
    if (made > 0) regionOf.set(`lobe-${key}`, f.shape.region);
  }
  for (const p of k.shapes("plan", (s) => isFol(s) && /^crown-fill/.test(s.id))) {
    const id = p.shape.id as string;
    const Pf = k.P((s) => s.id === id);
    const pb = bboxOfShapes(Pf);
    const cx = (pb.minX + pb.maxX) / 2, cz = (pb.minY + pb.maxY) / 2;
    let best: { id: string; d: number } | null = null;
    for (const f of fronts) {
      const key = (f.shape.id as string).replace(/^f-/, "");
      if (/core|top|epi/.test(key)) continue;
      const q = bboxOfShapes(k.P((s) => s.id === `p-${key}`));
      const d = Math.hypot((q.minX + q.maxX) / 2 - cx, (q.minY + q.maxY) / 2 - cz);
      if (best === null || d < best.d) best = { id: f.shape.id as string, d };
    }
    const Fr = k.F((s) => s.id === best!.id);
    const m = k.metaOf("plan", (s) => s.id === id);
    void Fr;
    const Ef = k.E((s) => isFol(s) && /-mass$/.test(s.id)), Ff = k.F((s) => isFol(s) && /^f-/.test(s.id));
    const eb = bboxOfShapes(Ef), fb = bboxOfShapes(Ff);
    const y0 = Math.max(eb.minY, fb.minY), y1 = Math.min(eb.maxY, fb.maxY), N = env("FILLN", S === 50 ? 14 : S === 100 ? 11 : 9);
    const nsl = hullSlabs(k, `fill-${id}`, p.shape.bone, Ef, Ff, Pf, m, { heights: Array.from({ length: N + 1 }, (_, i) => y0 + 1e-3 + ((y1 - y0 - 2e-3) * (0.6 * (1 - Math.cos((Math.PI * i) / N)) / 2 + 0.4 * (i / N)))), tol: env("FILLTOL", 0.12), minArea: 0.15, gap: 0.6, prism: true, widest: process.env.FILLWIDEST === "1" });
    for (let i = 1; i <= nsl; i++) regionOf.set(`fill-${id}-${i}`, p.shape.region);
  }
  const crown = k.shapes("elevation", (s) => s.id === "crown");
  if (crown.length) {
    const bone = crown[0]!.shape.bone as string;
    const Pc = k.P((s) => s.id === "crown"), Fc = k.F((s) => s.id === "crown"), Ee = k.E((s) => s.id === "crown");
    const m = k.metaOf("elevation", (s) => s.id === "crown", { material: regions["leaf-sun"].material });
    const eb = bboxOfShapes(Ee), fb = bboxOfShapes(Fc);
    const y0 = Math.max(eb.minY, fb.minY), y1 = Math.min(eb.maxY, fb.maxY), N = env("CROWNN", 10);
    hullSlabs(k, "crown", bone, Ee, Fc, Pc, m, { heights: Array.from({ length: N + 1 }, (_, i) => y0 + 1e-3 + ((y1 - y0 - 2e-3) * (env("CROWNCOS", 0.6) * (1 - Math.cos((Math.PI * i) / N)) / 2 + (1 - env("CROWNCOS", 0.6)) * (i / N)))), tol: env("CROWNTOL", 0.1), minArea: 0.3, gap: env("CROWNGAP", 1.0), prism: true });
    for (const s of k.body) if (s.id.startsWith("crown")) regionOf.set(s.id, "leaf-sun");
  }
  const ep = k.shapes("elevation", (s) => s.id === "epicormic" && s.region === undefined);
  if (ep.length && S === 500) regionOf.set("epicormic", "leaf-epicormic");
  for (const s of k.body) {
    const r = regionOf.get(s.id);
    if (r) k.thermal.push({ id: `th-${s.id}`, of: s.id, region: r, ...(s.id === "stem" ? { sectionFaces: ["-u"] } : {}) });
  }
  if (S === 500) {
    const k2 = new Kit(SPEC, 200);
    const reg = k2.asset.thermal.regions.find((r: any) => r.id === "limb-bark");
    const mt = { material: reg.material, role: "body", tone: 0.35 };
    const planU = k2.P((s) => s.role === "body").flatMap((s) => s);
    const inU = (x: number, z: number): boolean => insideRings(planU, x, z) && insideRings(planU, x, z + 0.1) && insideRings(planU, x, z - 0.1);
    for (const e of k2.shapes("elevation", (s) => /^wood-l[we]\da$/.test(s.id))) {
      const id = e.shape.id as string, bone = e.shape.bone as string;
      const secs = limbTube(k2, `scratch-${id}`, bone, k2.E((s) => s.id === id), k2.F((s) => s.bone === bone && s.role === "body" && !/^f-/.test(s.id)), mt, { n: 4, P: k2.P((s) => s.bone === bone && s.role !== "body" && /^plan-/.test(s.id)), zInside: inU });
      k.loft("thermal", `th-${id}`, bone, secs, mt, { region: "limb-bark" });
    }
    const rs = regions["epicormic-stem"];
    k.cap("thermal", "th-epicormic-stem", "trunk", [0.3, 1.2, 0.25], [1.0, 2.2, 0.45], 0.05, 0.03, { material: rs.material, role: "body", tone: 0.35 }, { region: "epicormic-stem" });
    const core = new Kit(SPEC, 200);
    for (const id of ["core-low", "core-high"]) {
      const made = sheet(k, `th-leaf-shade-${id}`, `fol-${id}`, core.P((s) => s.id === `p-${id}`), core.F((s) => s.id === `f-${id}`), core.metaOf("front", (s) => s.id === `f-${id}`), { n: 4 });
      void made;
    }
    for (const s of k.body.filter((b) => b.id.startsWith("th-leaf-shade"))) {
      k.body.splice(k.body.indexOf(s), 1);
      k.thermal.push({ ...s, region: "leaf-shade" });
    }
  }
  emitterPatches(k, () => undefined);
  for (const bone of k.memberBones().filter((b) => !paths.has(b))) {
    const f = k.fr[bone]!;
    const o = k.origin(bone), t = k.tip(bone);
    let best: { sub: Sec[]; d: number } | null = null;
    for (const [pb, secs] of paths) {
      const sub = subPath(secs, o, t);
      if (sub === null) continue;
      const near = (p: [number, number, number]) => Math.min(...secs.map((s) => Math.hypot(s.c[0] - p[0], s.c[1] - p[1])));
      const d = near(o) + near(t);
      if (best === null || d < best.d) best = { sub, d };
      void pb;
    }
    void f;
    if (best !== null && best.d < 0.8) paths.set(bone, best.sub);
  }
  const EB = k.E((s) => s.role === "body"), FB = k.F((s) => s.role === "body");
  const insideDrawn = (q: [number, number, number]) => ivalsAll(EB, "x", q[0]).some((v) => q[1] >= v[0] + 0.02 && q[1] <= v[1] - 0.02) && ivalsAll(FB, "x", q[2]).some((v) => q[1] >= v[0] + 0.02 && q[1] <= v[1] - 0.02);
  treeStructure(k, paths, { clip: insideDrawn, core: 0.62, coreOf: { "root-plate": 0.45 }, joints: S <= 100, masses: [], massK: 0.28 });
  const vol = k.volume() as any;
  vol.decisions = src.volumes[S]?.decisions?.length && src.volumes[S].decisions[0].en?.startsWith("LIME") ? src.volumes[S].decisions : [{ en: "TODO", de: "TODO" }];
  src.volumes[S] = vol;
  console.log(S, vol.body.length, vol.thermal.length, vol.structure.length);
}
writeFileSync(FILE, `${JSON.stringify(src, null, 1)}\n`);
