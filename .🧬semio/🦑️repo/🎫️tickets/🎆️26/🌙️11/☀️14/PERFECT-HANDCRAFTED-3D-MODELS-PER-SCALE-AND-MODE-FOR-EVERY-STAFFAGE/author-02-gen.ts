/** 🌳️ Data-driven per-scale generator of the author-02 trees: reads each scale's own 2D shapes (limbs from matching elevation/front/plan shapes, lobes from front/plan pairs, hulls from elevation/front/plan), then thermal references and structure.
 * Usage: bun author-02-gen.ts <slug> [scale] */
import { Kit, bboxOfShapes, writeSource, type Shapes } from "./author-02-lib.ts";
import { newCtx, structure, thermalBark, thermalFoliage, type Group } from "./author-02-tree.ts";

const slug = process.argv[2]!;
const only = process.argv[3] ? Number(process.argv[3]) : undefined;
const SPEC = `vegetation/${slug}`;
const out: Record<string, any> = {};
const NAMES: Record<string, string> = {};

for (const S of [50, 100, 200, 500] as const) {
  const k = new Kit(SPEC, S);
  const regions: Record<string, any> = Object.fromEntries(k.asset.thermal.regions.map((r: any) => [r.id, r]));
  const isFol = (s: any) => regions[s.region]?.kind === "foliage";
  const isBark = (s: any) => regions[s.region] && !isFol(s);
  if (only && S !== only) { out[S] = { status: "authored", denominator: S, body: [], thermal: [], structure: [], decisions: [] }; continue; }
  const c = newCtx(k);
  const step = Math.max(0.1, k.feat * 4);
  const ids = (view: "elevation" | "front" | "plan") => k.shapes(view, (s) => s.role === "body");
  const E = ids("elevation"), Fr = ids("front"), Pl = ids("plan");
  const boneHas = new Set<string>();
  const done = new Set<string>();

  for (const e of E.filter((x) => isBark(x.shape) && !/plug/.test(x.shape.id))) {
    const id = e.shape.id as string;
    if (done.has(id)) continue;
    const m = k.metaOf("elevation", (s) => s.id === id);
    const hasF = Fr.some((x) => x.shape.id === id), hasP = Pl.some((x) => x.shape.id === id);
    if (k.limb(id, e.shape.bone, k.E((s) => s.id === id), hasF ? k.F((s) => s.id === id) : null, hasP ? k.P((s) => s.id === id) : null, m)) {
      c.regionOf.set(id, e.shape.region); c.paths.set(e.shape.bone, k.lastPath); boneHas.add(e.shape.bone); done.add(id);
    }
  }
  for (const f of Fr.filter((x) => isBark(x.shape) && !/plug/.test(x.shape.id) && !done.has(x.shape.id) && !boneHas.has(x.shape.bone))) {
    const id = f.shape.id as string;
    if (k.limb(id, f.shape.bone, [], k.F((s) => s.id === id), null, k.metaOf("front", (s) => s.id === id))) { c.regionOf.set(id, f.shape.region); c.paths.set(f.shape.bone, k.lastPath); boneHas.add(f.shape.bone); done.add(id); }
  }

  const groups: Group[] = [];
  const addGroup = (id: string, bone: string, region: string, Pk: Shapes | null, Fk: Shapes | null, Ek: Shapes | null) => {
    const pb = Pk ? bboxOfShapes(Pk) : null, fb = Fk ? bboxOfShapes(Fk) : null, eb = Ek ? bboxOfShapes(Ek) : null;
    const xs: [number, number] = pb ? [pb.minX, pb.maxX] : [eb!.minX, eb!.maxX];
    const gb = bboxOfShapes(k.E((q) => isFol(q)));
    const ys: [number, number] = fb ? [fb.minY, fb.maxY] : eb ? [eb.minY, eb.maxY] : [gb.minY, gb.maxY];
    const zs: [number, number] = pb ? [pb.minY, pb.maxY] : fb ? [fb.minX, fb.maxX] : [-(xs[1] - xs[0]) / 2, (xs[1] - xs[0]) / 2];
    groups.push({ id, bone, region, c: [(xs[0] + xs[1]) / 2, (ys[0] + ys[1]) / 2, (zs[0] + zs[1]) / 2], h: [(xs[1] - xs[0]) / 2, (ys[1] - ys[0]) / 2, (zs[1] - zs[0]) / 2], yRange: ys });
  };
  const tol = S === 50 ? 0.02 : S === 100 ? 0.04 : 0.08;
  let lobes = 0;
  const pairedPlan = new Set<string>();
  for (const f of Fr.filter((x) => isFol(x.shape))) {
    const id = f.shape.id as string;
    const key = id.replace(/^f-/, "");
    const pid = `p-${key}`;
    const Pk = k.P((s) => s.id === pid);
    if (!Pk.length) continue;
    const Fk = k.F((s) => s.id === id);
    const m = k.metaOf("front", (s) => s.id === id);
    const n = k.lobeSlabs(`lobe-${key}`, f.shape.bone, Pk, Fk, m, { tol, simplify: Math.max(0.006, k.feat * 0.6) });
    for (let i = 1; i <= n; i++) c.regionOf.set(`lobe-${key}-${i}`, f.shape.region);
    pairedPlan.add(pid); boneHas.add(f.shape.bone); lobes++;
    addGroup(key, f.shape.bone, f.shape.region, Pk, Fk, null);
  }
  const folE = E.filter((x) => isFol(x.shape));
  const allFolE = k.E((s) => isFol(s)), allFolF = k.F((s) => isFol(s));
  for (const e of folE) {
    const id = e.shape.id as string;
    if (boneHas.has(e.shape.bone) && lobes > 0) continue;
    const Fk = k.F((s) => s.bone === e.shape.bone && isFol(s) && s.id !== id ? true : s.id === id);
    const Pk = k.P((s) => (s.id === id || (s.bone === e.shape.bone && isFol(s))) && !pairedPlan.has(s.id));
    const m = k.metaOf("elevation", (s) => s.id === id);
    const made = k.hull(`mass-${id}`, e.shape.bone, m, { E: k.E((s) => s.id === id), F: Fk.length ? Fk : undefined, P: Pk.length ? Pk : undefined, step });
    if (made) {
      for (const sol of k.body) if (sol.id.startsWith(`mass-${id}`)) c.regionOf.set(sol.id, e.shape.region);
      boneHas.add(e.shape.bone);
      addGroup(id, e.shape.bone, e.shape.region, Pk.length ? Pk : null, Fk.length ? Fk : null, k.E((s) => s.id === id));
    }
  }
  for (const p of Pl.filter((x) => isFol(x.shape) && !pairedPlan.has(x.shape.id) && !boneHas.has(x.shape.bone))) {
    const id = p.shape.id as string;
    const made = k.hull(`fill-${id}`, p.shape.bone, k.metaOf("plan", (s) => s.id === id), { E: allFolE.length ? allFolE : undefined, F: allFolF.length ? allFolF : undefined, P: k.P((s) => s.id === id), step: Math.max(step, 0.3) });
    if (made) { for (const sol of k.body) if (sol.id.startsWith(`fill-${id}`)) c.regionOf.set(sol.id, p.shape.region); boneHas.add(p.shape.bone); addGroup(id, p.shape.bone, p.shape.region, k.P((s) => s.id === id), null, null); }
  }
  const drawn = new Set<string>([...k.shapes("elevation", () => true, ["body", "detail", "glazing"]), ...k.shapes("front", () => true, ["body", "detail", "glazing"]), ...k.shapes("plan", () => true, ["body", "detail", "glazing"])].map((x) => x.shape.bone));
  for (const b of k.asset.rig.bones.map((x: any) => x.id)) {
    if (boneHas.has(b) || !drawn.has(b)) continue;
    const o = k.origin(b), t = k.tip(b);
    const r = Math.max(k.feat * 0.6, k.boneDef(b).section?.extremeFibreM ?? 0.02);
    k.cap("body", `bone-${b}`, b, o, t, r, r * 0.8, { material: k.asset.materials[0].id, role: "body", tone: 0.4 });
    boneHas.add(b);
  }
  c.groups.push(...groups);

  const barkRegions = Object.values(regions).filter((r: any) => r.kind !== "foliage");
  for (const r of barkRegions as any[]) {
    const bones = [...new Set([...E, ...Fr].filter((x) => x.shape.region === r.id).map((x) => x.shape.bone as string))];
    thermalBark(c, r.id, { fallbackBones: bones.length ? bones : k.asset.rig.bones.filter((b: any) => b.section).map((b: any) => b.id), minRadius: k.feat * 0.55 });
  }
  for (const r of Object.values(regions) as any[]) if (r.kind === "foliage") {
    thermalFoliage(c, r.id);
    if (!k.thermal.some((s) => s.region === r.id)) {
      const g = groups.filter((q) => q.region === r.id)[0] ?? groups[0]!;
      k.ell("thermal", `th-${r.id}`, g.bone, g.c, [g.h[0], g.h[1], g.h[2]].map((v) => Math.max(v * 0.8, k.feat)) as any, { material: r.material, role: "body", tone: 0.5 }, { region: r.id });
    }
  }
  const byB = new Map<string, Group[]>();
  for (const g of groups) (byB.get(g.bone) ?? byB.set(g.bone, []).get(g.bone)!).push(g);
  const masses: Group[] = [];
  for (const [bone, gs] of byB) {
    const lo = [Math.min(...gs.map((g) => g.c[0] - g.h[0])), Math.min(...gs.map((g) => g.yRange[0])), Math.min(...gs.map((g) => g.c[2] - g.h[2]))];
    const hi = [Math.max(...gs.map((g) => g.c[0] + g.h[0])), Math.max(...gs.map((g) => g.yRange[1])), Math.max(...gs.map((g) => g.c[2] + g.h[2]))];
    masses.push({ id: bone, bone, region: gs[0]!.region, c: [(lo[0]! + hi[0]!) / 2, (lo[1]! + hi[1]!) / 2, (lo[2]! + hi[2]!) / 2], h: [(hi[0]! - lo[0]!) / 2, (hi[1]! - lo[1]!) / 2, (hi[2]! - lo[2]!) / 2], yRange: [lo[1]!, hi[1]!] });
  }
  structure(c, { joints: S <= 100 ? "all" : "none", masses: S === 500 ? masses.slice(0, 1) : masses });
  k.note(
    `1:${S}: body derived from this scale's own 2D shapes: ${[...done].length} limb lofts along the drawn bark shapes (z centre and depth from the front/plan outline of the same shape), ${lobes} foliage lobes as stacks of frusta of the exact plan outline scaled by the front width profile, the remaining foliage masses as visual-hull lofts of the elevation, front and plan silhouettes. Thermal references the exposed bark and foliage solids; structure holds one member per load-bearing bone with its section, joints${S >= 200 ? " omitted at this scale" : ""}, mass ellipsoids at the foliage centres and the fixed root support.`,
    `1:${S}: Körper aus den eigenen 2D-Formen dieses Massstabs: ${[...done].length} Astlofts entlang der gezeichneten Rindenformen (Seitenlage und Tiefe aus dem Front-/Grundriss-Umriss derselben Form), ${lobes} Laublappen als Stapel von Kegelstümpfen des exakten Grundriss-Umrisses, skaliert mit dem Breitenprofil der Front, die übrigen Laubmassen als Visual-Hull-Lofts aus Ansicht, Front und Grundriss. Die Thermik verweist auf die exponierten Rinden- und Laubkörper; die Tragstruktur enthält je tragendem Knochen ein Stabelement mit seinem Querschnitt, Gelenke${S >= 200 ? " entfallen in diesem Massstab" : ""}, Massenellipsoide in den Laubschwerpunkten und das eingespannte Wurzelauflager.`,
  );
  out[S] = k.volume();
  void NAMES;
}
writeSource(slug, out);
