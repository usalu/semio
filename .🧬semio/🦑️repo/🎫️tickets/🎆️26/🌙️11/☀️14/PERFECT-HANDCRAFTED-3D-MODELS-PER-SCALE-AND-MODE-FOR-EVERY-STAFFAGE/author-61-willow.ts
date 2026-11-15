/** 🌿️ Author 61: weeping willow, per-scale handcrafted model. Trunk and limbs are lofts along the scale's own elevation outlines with depth from the front outline; every drawn curtain sheet is one spindle loft (plan footprint as oriented superellipse swept with the sheet's own front width profile); caps likewise; the 1:500 crown is one visual-hull loft. Usage: `bun author-61-willow.ts <scale|all>`.
 * @see ./author-61.md */
import { readFileSync, writeFileSync } from "node:fs";
import { Kit, bboxOfShapes, ivalsAll, type Sec } from "./author-02-lib.ts";
import { emitterPatches, limb3, localRadii, treeStructure } from "./author-31-lib.ts";
import { TICKET } from "./volumes-lib.ts";
import { hullLobes, hullSlabs, insideRings, limbTube, sheet, subPath } from "./author-61-lib.ts";

const SPEC = "vegetation/weeping-willow";
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
  const n = S === 50 ? 7 : S === 100 ? 5 : 4;
  const planFol = k.P((s) => s.role === "body" && !/^pl-limb/.test(s.id)).flatMap((s) => s);
  const keepPlan = (q: [number, number, number]): boolean => insideRings(planFol, q[0], q[2]);
  const bark = k.shapes("elevation", (s) => s.role === "body" && !isFol(s) && !/plug|flare|^crown/.test(s.id));
  for (const e of bark) {
    const id = e.shape.id as string, bone = e.shape.bone as string, region = e.shape.region as string;
    const E = k.E((s) => s.id === id);
    const F = k.F((s) => s.bone === bone && s.role === "body" && !isFol(s) && !/plug/.test(s.id));
    const m = k.metaOf("elevation", (s) => s.id === id);
    const secs = /^limb/.test(bone) ? limbTube(k, id, bone, E, F, m, { n: env("LIMBN", S === 50 ? 14 : 6), zInside: (x, z) => insideRings(planFol, x, z) && insideRings(planFol, x, z + 0.1) && insideRings(planFol, x, z - 0.1) }) : limb3(k, id, bone, E, m, { n, adaptive: k.feat * 0.5, exp: 2, F: F.length ? F : null, round: process.env.TRUNKROUND === "1" });
    paths.set(bone, secs);
    regionOf.set(id, region);
  }
  const rp = k.shapes("elevation", (s) => s.id === "root-flare");
  if (rp.length) {
    const rm = k.metaOf("elevation", (s) => s.id === "root-flare");
    paths.set("root-plate", limb3(k, "root-flare", "root-plate", k.E((s) => s.id === "root-flare"), rm, { n: 6, adaptive: k.feat * 0.5, exp: 2, F: k.F((s) => s.id === "root-flare") }));
    regionOf.set("root-flare", rp[0]!.shape.region);
  }
  if (S === 50) {
    const mb = k.metaOf("elevation", (s) => s.id === "trunk-1");
    for (const [id, c, r] of [["burr", [0.3, 1.2, 0.37], [0.09, 0.14, 0.07]], ["burr-2", [-0.33, 0.8, 0.33], [0.08, 0.12, 0.07]]] as const) {
      const lr = localRadii(k, "trunk-1", [r[0], r[1], r[2]]);
      k.ell("body", id, "trunk-1", [c[0], c[1], c[2]], lr, mb);
      regionOf.set(id, "trunk-bark");
    }
  } else if (!k.body.some((s) => s.bone === "root-plate")) {
    const mr = k.metaOf("plan", (s) => s.id === "trunk-dot");
    const r = bboxOfShapes(k.P((s) => s.id === "trunk-dot"));
    const rr = (r.maxX - r.minX) / 2;
    k.loft("body", "root-plate", "root-plate", [k.secY([0, 0.0, 0], rr, rr, 2), k.secY([0, 0.3, 0], rr, rr, 2)], mr);
    regionOf.set("root-plate", "trunk-bark");
  }
  const axes = new Map<string, Sec[]>();
  const groups: { id: string; bone: string; c: [number, number, number]; h: [number, number, number] }[] = [];
  const lobeN = env("LOBEN", S === 50 ? 11 : S === 100 ? 9 : 7);
  const fronts = k.shapes("front", (s) => isFol(s) && /^f-/.test(s.id));
  for (const f of fronts) {
    const key = (f.shape.id as string).replace(/^f-/, "");
    const Pk = k.P((s) => s.id === `p-${key}`);
    if (!Pk.length) continue;
    const Fk = k.F((s) => s.id === f.shape.id);
    const m = k.metaOf("front", (s) => s.id === f.shape.id);
    const made = sheet(k, `lobe-${key}`, f.shape.bone, Pk, Fk, m, { n: lobeN, kx: env("LK", 1) });
    if (made !== null) {
      if (/^c\d-(0|180)$/.test(key)) axes.set(f.shape.bone as string, made.secs);
      regionOf.set(`lobe-${key}`, f.shape.region);
      const pb = bboxOfShapes(Pk), fb = bboxOfShapes(Fk);
      groups.push({ id: key, bone: f.shape.bone, c: [(pb.minX + pb.maxX) / 2, (fb.minY + fb.maxY) / 2, (pb.minY + pb.maxY) / 2], h: [(pb.maxX - pb.minX) / 2, (fb.maxY - fb.minY) / 2, (pb.maxY - pb.minY) / 2] });
    }
  }
  for (const p of k.shapes("plan", (s) => isFol(s) && /^crown-fill/.test(s.id))) {
    const id = p.shape.id as string;
    const Pf = k.P((s) => s.id === id);
    const pb = bboxOfShapes(Pf);
    const cx = (pb.minX + pb.maxX) / 2, cz = (pb.minY + pb.maxY) / 2;
    let best: { id: string; d: number } | null = null;
    for (const f of fronts) {
      const key = (f.shape.id as string).replace(/^f-/, "");
      if (/cap/.test(key)) continue;
      const q = bboxOfShapes(k.P((s) => s.id === `p-${key}`));
      const d = Math.hypot((q.minX + q.maxX) / 2 - cx, (q.minY + q.maxY) / 2 - cz);
      if (best === null || d < best.d) best = { id: f.shape.id as string, d };
    }
    const Fr = k.F((s) => s.id === best!.id);
    const m = k.metaOf("plan", (s) => s.id === id);
    const nsl = k.lobeSlabs(`fill-${id}`, p.shape.bone, Pf, Fr, m, { tol: env("FILLTOL", 0.25), simplify: k.feat * 2 });
    for (let i = 1; i <= nsl; i++) regionOf.set(`fill-${id}-${i}`, p.shape.region);
  }
  const crown = k.shapes("elevation", (s) => /^crown/.test(s.id));
  if (crown.length) {
    const id = crown[0]!.shape.id as string, bone = crown[0]!.shape.bone as string;
    const Pc = k.P((s) => s.id === id), Fc = k.F((s) => s.id === id), Ee = k.E((s) => s.id === id);
    const m = k.metaOf("elevation", (s) => s.id === id, { material: regions["leaf-curtain"].material });
    if (process.env.CROWNMODE !== "lobes") {
      const eb = bboxOfShapes(Ee), fb = bboxOfShapes(Fc);
      const y0 = Math.max(eb.minY, fb.minY), y1 = Math.min(eb.maxY, fb.maxY), N = env("CROWNN", 10);
      hullSlabs(k, "crown", bone, Ee, Fc, Pc, m, { heights: Array.from({ length: N + 1 }, (_, i) => y0 + 1e-3 + ((y1 - y0 - 2e-3) * (env("CROWNCOS", 0.6) * (1 - Math.cos((Math.PI * i) / N)) / 2 + (1 - env("CROWNCOS", 0.6)) * (i / N)))), tol: env("CROWNTOL", 0.15), minArea: 0.5, gap: env("CROWNGAP", 1.6), prism: process.env.CROWNPRISM !== "0" });
    } else hullLobes(k, "crown", bone, Ee, Fc, Pc, m, { stations: env("CROWNST", 13), splits: (process.env.CROWNSPLITS ?? "-4,0,4").split(",").map(Number), overlap: env("CROWNOV", 0.4), cell: env("CROWNCELL", 0.15), recall: env("CROWNRECALL", 2) });
    for (const s of k.body) if (s.id.startsWith("crown")) regionOf.set(s.id, "leaf-curtain");
  }
  for (const s of k.body) {
    const r = regionOf.get(s.id);
    if (r) k.thermal.push({ id: `th-${s.id}`, of: s.id, region: r, ...(s.id === "root-flare" || s.id === "bole" ? { sectionFaces: ["-u"] } : {}) });
  }
  if (S === 500) {
    const k2 = new Kit(SPEC, 200);
    const reg = k2.asset.thermal.regions.find((r: any) => r.id === "limb-bark");
    const planUnion = k2.P((s) => s.role === "body" && !/^pl-limb/.test(s.id)).flatMap((s) => s);
    for (const side of ["w", "e"]) {
      const E = k2.E((s) => s.id === `limb-${side}`), F = k2.F((s) => s.bone === `limb-${side}-a` && s.role === "body");
      const secs = limbTube(k2, `scratch-${side}`, `limb-${side}-a`, E, F, { material: reg.material, role: "body", tone: 0.35 }, { n: 5, zInside: (x, z) => insideRings(planUnion, x, z) && insideRings(planUnion, x, z + 0.1) && insideRings(planUnion, x, z - 0.1) });
      k.loft("thermal", `th-limb-${side}`, `limb-${side}-a`, secs, { material: reg.material, role: "body", tone: 0.35 }, { region: "limb-bark" });
    }
  }
  emitterPatches(k, () => undefined);
  for (const [bone, secs] of axes) {
    const keep = secs.filter((s) => Math.min(s.rx, s.ry) >= env("AXISMIN", 0.12));
    if (keep.length >= 2) paths.set(bone, keep.map((s) => ({ ...s, rx: 0.03, ry: 0.03, exp: 2 })));
  }
  for (const bone of k.memberBones().filter((b) => /^limb-[we]-[bc]$/.test(b) && !paths.has(b))) {
    const merged = paths.get(bone.startsWith("limb-w") ? "limb-w-a" : "limb-e-a");
    if (merged === undefined) continue;
    const sub = subPath(merged, k.origin(bone), k.tip(bone));
    paths.set(bone, sub ?? merged.slice(-2));
  }
  const EB = k.E((s) => s.role === "body"), FB = k.F((s) => s.role === "body");
  const insideDrawn = (q: [number, number, number]) => ivalsAll(EB, "x", q[0]).some((v) => q[1] >= v[0] + 0.02 && q[1] <= v[1] - 0.02) && ivalsAll(FB, "x", q[2]).some((v) => q[1] >= v[0] + 0.02 && q[1] <= v[1] - 0.02);
  treeStructure(k, paths, { clip: insideDrawn, core: 0.62, coreOf: { "root-plate": 0.45 }, joints: S <= 100, skipJoint: S === 100 ? /^limb-[we]-c$/ : undefined, masses: [], massK: 0.28 });
  for (const s of k.structure) if (s.id === "joint-trunk-1") { const r = Math.max(...s.radii); s.transform.translation[0] += r * 1.15; }
  const vol = k.volume() as any;
  vol.decisions = src.volumes[S]?.decisions?.length && src.volumes[S].decisions[0].en?.startsWith("WILLOW") ? src.volumes[S].decisions : [{ en: "TODO", de: "TODO" }];
  src.volumes[S] = vol;
  console.log(S, vol.body.length, vol.thermal.length, vol.structure.length);
}
writeFileSync(FILE, `${JSON.stringify(src, null, 1)}\n`);
