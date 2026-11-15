import { Kit, bboxOfShapes, ivalsAll, type Shapes } from "./author-02-lib.ts";
import { newCtx, structure, thermalBark, thermalFoliage, type Group } from "./author-02-tree.ts";
import { writeSource } from "./author-02-lib.ts";

const SPEC = "vegetation/japanese-maple";
const only = process.argv[2] ? Number(process.argv[2]) : undefined;
const out: Record<string, any> = {};

const ellipseF = (zc: number, hz: number, y0: number, y1: number): Shapes => {
  const ring = Array.from({ length: 48 }, (_, i) => { const a = (2 * Math.PI * i) / 48; return { x: zc + hz * Math.cos(a), y: (y0 + y1) / 2 + ((y1 - y0) / 2) * Math.sin(a) }; });
  return [[ring]];
};

for (const S of [50, 100, 200, 500] as const) {
  const k = new Kit(SPEC, S);
  if (only && S !== only) { out[S] = { status: "authored", denominator: S, body: [], thermal: [], structure: [], decisions: [] }; continue; }
  const c = newCtx(k);
  const step = Math.max(0.1, k.feat * 4);
  const elevS = k.shapes("elevation", (s) => s.role === "body");
  const frontS = k.shapes("front", (s) => s.role === "body");
  const sunMeta = (view: "elevation" | "front" | "plan", id: string, shade: boolean) => k.metaOf(view, (s) => s.id === id, { material: shade ? "maple-leaf-shade" : "maple-leaf-sun" });
  const bark = (id: string) => k.metaOf("elevation", (s) => s.id === id, { material: "maple-bark" });

  for (const id of ["root-collar", "wood-stem-c", "wood-stem-w", "wood-stem-e", "wood-stem-c-top"]) {
    if (S === 500) break;
    const e = [...elevS, ...frontS].find((x) => x.shape.id === id);
    if (!e) continue;
    const hasE = elevS.some((x) => x.shape.id === id);
    if (k.limb(id, e.shape.bone, hasE ? k.E((s) => s.id === id) : [], k.F((s) => s.id === id), k.P((s) => s.id === id), hasE ? bark(id) : k.metaOf("front", (s) => s.id === id, { material: "maple-bark" }))) {
      c.regionOf.set(id, "stem-bark");
      if (id !== "root-collar") c.paths.set(e.shape.bone, k.lastPath);
    }
  }
  if (S === 500) {
    k.cap("body", "root-collar", "root-plate", [0, 0.02, 0], [0, 0.25, 0], 0.3, 0.28, bark("stems"));
    k.hull("stems", "stem-c", bark("stems"), { E: k.E((s) => s.id === "stems"), F: k.F((s) => s.id === "stems"), P: k.P((s) => s.id === "stems"), step: 0.2 });
    c.regionOf.set("stems", "stem-bark");
  }

  const tierBones = ["t1w", "t1e", "t2w", "t2e", "t3w", "t3e"];
  if (S === 100) {
    for (const b of tierBones) {
      k.cap("body", `limb-${b}`, b, k.origin(b), k.tip(b), 0.032, 0.022, k.metaOf("elevation", (s) => s.id === "root-collar", { material: "maple-bark-young" }));
      c.regionOf.set(`limb-${b}`, "limb-bark"); c.paths.set(b, [k.secX(k.origin(b), 0.03, 0.03, 2), k.secX(k.tip(b), 0.02, 0.02, 2)].map((q, i) => i ? { ...q, ax: [0, 1, 0], ay: [0, 0, 1] } : q));
    }
  }
  if (S === 50) {
    for (const b of tierBones) {
      const tid = `tier-${b}`;
      const E = k.E((s) => s.id === tid, ["texture"]);
      if (!E.length) continue;
      const m = k.metaOf("elevation", (s) => s.id === tid, { material: "maple-bark-young", role: "body" });
      if (k.limb(`limb-${b}`, b, E, null, null, m, { n: 6 })) { c.regionOf.set(`limb-${b}`, "limb-bark"); c.paths.set(b, k.lastPath); }
    }
  }

  const eMass = (bone: string): [number, number] | null => {
    const sh = elevS.filter((x) => x.shape.bone === bone && /-mass$/.test(x.shape.id));
    if (!sh.length) return null;
    const b = bboxOfShapes(sh.map((x) => x.rings));
    return [b.minY, b.maxY];
  };
  const foliage: { key: string; bone: string; shade: boolean }[] = [];
  if (S === 500) {
    const m = sunMeta("elevation", "crown", false);
    k.hull("crown", "stem-c", m, { E: k.E((s) => s.id === "crown"), F: k.F((s) => s.id === "crown"), P: k.P((s) => s.id === "crown"), step: 0.3 });
    const eb = bboxOfShapes(k.E((s) => s.id === "crown")), pb = bboxOfShapes(k.P((s) => s.id === "crown"));
    c.groups.push({ id: "crown", bone: "stem-c", region: "leaf-sun", c: [0, (eb.minY + eb.maxY) / 2, 0], h: [(eb.maxX - eb.minX) / 2, (eb.maxY - eb.minY) / 2, (pb.maxY - pb.minY) / 2], yRange: [eb.minY, eb.maxY] });
  } else {
    const fr = frontS.filter((x) => /^f-/.test(x.shape.id));
    const byBone = new Map<string, number[]>();
    for (const f of fr) { const b = bboxOfShapes(k.F((s) => s.id === f.shape.id)); (byBone.get(f.shape.bone) ?? byBone.set(f.shape.bone, []).get(f.shape.bone)!).push((b.minY + b.maxY) / 2); }
    const shiftOf = (bone: string): number => {
      const e = eMass(bone), fc = byBone.get(bone);
      if (!e || !fc || S === 500) return 0;
      const fm = fc.reduce((t, v) => t + v, 0) / fc.length;
      return Math.max(-0.07, Math.min(0.07, 0.5 * ((e[0] + e[1]) / 2 - fm)));
    };
    for (const f of fr) {
      const key = f.shape.id.slice(2);
      const bone = f.shape.bone as string;
      const shade = /core/.test(key);
      const m = sunMeta("front", f.shape.id, shade);
      const Pk = k.P((s) => s.id === `p-${key}`), Fk = k.F((s) => s.id === f.shape.id);
      if (!Pk.length) continue;
      const pb = bboxOfShapes(Pk), fb = bboxOfShapes(Fk);
      if (/core|^top$/.test(key)) {
        const ekey = key === "top" ? "fol-top-mass" : `fol-${key}-mass`;
        k.hull(`crown-${key}`, bone, m, { E: k.E((s) => s.id === ekey), F: Fk, step });
      } else k.lobeSlabs(`lobe-${key}`, bone, Pk, Fk, m, { yShift: shiftOf(bone), tol: S === 50 ? 0.016 : S === 100 ? 0.03 : 0.06, simplify: Math.max(0.006, k.feat * 0.6) });
      foliage.push({ key, bone, shade });
      c.groups.push({ id: key, bone, region: shade ? "leaf-shade" : "leaf-sun", c: [(pb.minX + pb.maxX) / 2, (fb.minY + fb.maxY) / 2, (pb.minY + pb.maxY) / 2], h: [(pb.maxX - pb.minX) / 2, (fb.maxY - fb.minY) / 2, (pb.maxY - pb.minY) / 2], fit: k.planFit(Pk), yRange: [fb.minY, fb.maxY] });
    }
    if (S === 200) {
      const allE = k.E((s) => /-mass$/.test(s.id));
      for (const pf of k.shapes("plan", (s) => /^crown-fill/.test(s.id))) {
        const Pk = k.P((s) => s.id === pf.shape.id);
        const pb = bboxOfShapes(Pk);
        const iv = ivalsAll(allE, "x", (pb.minX + pb.maxX) / 2);
        const y0 = Math.min(...iv.map((v) => v[0])) + 0.15, y1 = Math.max(...iv.map((v) => v[1])) - 0.15;
        const hz = (pb.maxY - pb.minY) / 2, zc = (pb.minY + pb.maxY) / 2;
        const m = sunMeta("plan", pf.shape.id, false);
        k.hull(`fill-${pf.shape.id.replace("crown-fill-", "")}`, pf.shape.bone, m, { E: allE, F: k.F((s) => /^f-/.test(s.id)), P: Pk, step: 0.3 });
        c.groups.push({ id: pf.shape.id, bone: pf.shape.bone, region: "leaf-sun", c: [(pb.minX + pb.maxX) / 2, (y0 + y1) / 2, zc], h: [(pb.maxX - pb.minX) / 2, (y1 - y0) / 2, hz], fit: k.planFit(Pk), yRange: [y0, y1] });
      }
    }
  }

  for (const sol of k.body) { const g = c.groups.find((q) => sol.id === `crown-${q.id}` || sol.id.startsWith(`crown-${q.id}-`) || sol.id.startsWith(`lobe-${q.id}-`) || sol.id === "crown" || sol.id.startsWith(`fill-${q.id.replace("crown-fill-", "")}`)); if (g && !c.regionOf.has(sol.id)) c.regionOf.set(sol.id, g.region); }
  thermalBark(c, "stem-bark");
  thermalBark(c, "limb-bark", { fallbackBones: tierBones, minRadius: S >= 200 ? k.feat * 0.55 : 0 });
  thermalFoliage(c, "leaf-sun");
  thermalFoliage(c, "leaf-shade");
  void ellipseF;

  const massGroups: Group[] = [];
  const byB = new Map<string, Group[]>();
  for (const g of c.groups) (byB.get(g.bone) ?? byB.set(g.bone, []).get(g.bone)!).push(g);
  for (const [bone, gs] of byB) {
    if (S === 500) { massGroups.push(gs[0]!); continue; }
    const lo = [Math.min(...gs.map((g) => g.c[0] - g.h[0])), Math.min(...gs.map((g) => g.yRange[0])), Math.min(...gs.map((g) => g.c[2] - g.h[2]))];
    const hi = [Math.max(...gs.map((g) => g.c[0] + g.h[0])), Math.max(...gs.map((g) => g.yRange[1])), Math.max(...gs.map((g) => g.c[2] + g.h[2]))];
    massGroups.push({ id: bone, bone, region: gs[0]!.region, c: [(lo[0]! + hi[0]!) / 2, (lo[1]! + hi[1]!) / 2, (lo[2]! + hi[2]!) / 2], h: [(hi[0]! - lo[0]!) / 2, (hi[1]! - lo[1]!) / 2, (hi[2]! - lo[2]!) / 2], yRange: [lo[1]!, hi[1]!] });
  }
  structure(c, { joints: S <= 100 ? "all" : S === 200 ? "main" : "none", masses: massGroups, mergeJoint: S === 200 ? ["stem-c-top", "stem-w", "stem-e", "t1w", "t1e"] : undefined });
  k.note("TODO", "TODO");
  out[S] = k.volume();
}
writeSource("japanese-maple", out);
