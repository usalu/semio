/** 🍁️ Author 21: Japanese maple, four scales, each modelled against its own 2D drawings: visual-hull pads, root collar and stems on the drawn paths, members inside the body, thermal and structure.
 * Usage: bun author-21-maple.ts [scale]
 * @see ./author-21-lib.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { Kit, bboxOfShapes, ivalsAll, writeSource, type Sec, type Shapes } from "./author-21-lib.ts";
import { TICKET, merged, toWorld } from "./volumes-lib.ts";
import { rulingMeasurements } from "./volumes-rulings.ts";

const SPEC = "vegetation/japanese-maple";
const only = process.argv[2] ? Number(process.argv[2]) : undefined;
const prior = JSON.parse(readFileSync(join(TICKET, "volumes", "vegetation", "japanese-maple.json"), "utf8")).volumes;
const out: Record<string, any> = {};
const TIERS = ["t1w", "t1e", "t2w", "t2e", "t3w", "t3e"];
const LOWER = Number(process.env.LOWER ?? 0.4);

for (const S of [50, 100, 200, 500] as const) {
  if (only && S !== only) { out[S] = prior[S]; continue; }
  const k = new Kit(SPEC, S);
  const feat = k.feat;
  const SHR = Number(process.env.SHR ?? (S === 50 ? 0.985 : S === 100 ? 1.05 : 0.97));
  const grow = Number(process.env.GROW ?? (S === 50 ? 0 : S === 100 ? 0 : S === 200 ? 0.02 : 0.04));
  const step = S === 50 ? 0.05 : S === 100 ? 0.08 : S === 200 ? 0.16 : 0.3;
  const allE = k.E(() => true);
  const bark = (view: "elevation" | "front" | "plan", id: string) => k.metaOf(view, (s) => s.id === id, { material: "maple-bark" });
  const regionOf = new Map<string, string>();
  const paths = new Map<string, Sec[]>();
  const groups: { id: string; bone: string; c: [number, number, number]; h: [number, number, number]; yRange: [number, number] }[] = [];

  if (S === 500) {
    const rr = [0.3, 0.29, 0.26, 0.2, 0.15];
    k.loft("body", "root-collar", "root-plate", [0, 0.05, 0.12, 0.2, 0.26].map((y, i) => k.secY([0, y, 0], rr[i]!, rr[i]!, 2)), bark("elevation", "stems"));
    regionOf.set("root-collar", "stem-bark");
    k.hull("stems", "stem-c", bark("elevation", "stems"), { E: k.E((s) => s.id === "stems"), F: k.F((s) => s.id === "stems"), P: k.P((s) => s.id === "stems"), step: 0.2 });
    regionOf.set("stems", "stem-bark");
  } else {
    const E = k.E((s) => s.id === "root-collar");
    const eb = bboxOfShapes(E);
    const ys = [0, 0.05, 0.11, 0.17, 0.22, 0.26].map((y) => Math.min(eb.maxY, Math.max(eb.minY, y)));
    const secs: Sec[] = ys.map((y) => { const iv = ivalsAll(E, "y", Math.min(eb.maxY - 0.01, Math.max(eb.minY + 0.03, y))); const hw = iv.length ? (iv[0]![1] - iv[0]![0]) / 2 : 0.2; return k.secY([0, y, 0], hw, hw, 2); });
    k.loft("body", "root-collar", "root-plate", secs, bark("elevation", "root-collar"));
    regionOf.set("root-collar", "stem-bark");
    paths.set("root-plate", secs);
    for (const id of ["wood-stem-c", "wood-stem-w", "wood-stem-e", "wood-stem-c-top"]) {
      const hasE = k.E((s) => s.id === id).length > 0;
      const shape = [...k.shapes("elevation", (s) => s.id === id), ...k.shapes("front", (s) => s.id === id)][0];
      if (!shape) continue;
      if (k.limb(id, shape.shape.bone, hasE ? k.E((s) => s.id === id) : [], k.F((s) => s.id === id), k.P((s) => s.id === id), hasE ? bark("elevation", id) : bark("front", id), { ends: true })) {
        regionOf.set(id, "stem-bark");
        paths.set(shape.shape.bone, k.lastPath);
      }
    }
  }

  const young = k.metaOf("elevation", (s) => s.id === "root-collar", { material: "maple-bark-young" });
  {
    for (const b of TIERS) {
      const a = k.origin(b), t = k.tip(b);
      const r0 = S === 50 ? 0.034 : S === 100 ? 0.036 : 0.03, r1 = S === 50 ? 0.022 : S === 100 ? 0.026 : 0.021;
      const mid: [number, number, number] = [(a[0] + t[0]) / 2, (a[1] + t[1]) / 2, (a[2] + t[2]) / 2];
      const f = k.fr[b]!;
      const mk = (p: [number, number, number], r: number): Sec => ({ c: p, ax: f.v, ay: f.w, rx: r, ry: r, exp: 2 });
      const sec = S === 50 ? [mk(a, r0), mk(mid, (r0 + r1) / 2), mk(t, r1)] : [mk(a, r0), mk(t, r1)];
      k.loft("body", `limb-${b}`, b, sec, young, S >= 200 ? { slender: true } : {});
      regionOf.set(`limb-${b}`, "limb-bark");
      paths.set(b, sec);
    }
  }

  const sunMeta = (view: "elevation" | "front" | "plan", id: string, shade: boolean) => k.metaOf(view, (s) => s.id === id, { material: shade ? "maple-leaf-shade" : "maple-leaf-sun" });
  const note = (id: string, bone: string, Pk: Shapes | null, Fk: Shapes | null, region: string) => {
    const pb = Pk ? bboxOfShapes(Pk) : null, fb = Fk ? bboxOfShapes(Fk) : null;
    if (!pb || !fb) return;
    groups.push({ id, bone, c: [(pb.minX + pb.maxX) / 2, (fb.minY + fb.maxY) / 2, (pb.minY + pb.maxY) / 2], h: [(pb.maxX - pb.minX) / 2, (fb.maxY - fb.minY) / 2, (pb.maxY - pb.minY) / 2], yRange: [fb.minY, fb.maxY] });
    void region;
  };

  if (S === 500) {
    const m = sunMeta("elevation", "crown", false);
    k.hull("crown", "stem-c", m, { E: k.E((s) => s.id === "crown"), F: k.F((s) => s.id === "crown"), P: k.P((s) => s.id === "crown"), grow, step: Number(process.env.C5 ?? 0.06), shrink: Number(process.env.C5S ?? 0.985), orient: process.env.OR !== "0", expHi: Number(process.env.EH ?? 4) });
    for (const s of k.body.filter((q) => q.id.startsWith("crown"))) { regionOf.set(s.id, "leaf-sun"); if (process.env.ENV500) s.envelope = true; }
    const eb = bboxOfShapes(k.E((s) => s.id === "crown")), pb = bboxOfShapes(k.P((s) => s.id === "crown"));
    groups.push({ id: "crown", bone: "stem-c", c: [0, (eb.minY + eb.maxY) / 2, 0], h: [(eb.maxX - eb.minX) / 2, (eb.maxY - eb.minY) / 2, (pb.maxY - pb.minY) / 2], yRange: [eb.minY, eb.maxY] });
  } else {
    const fr = k.shapes("front", (s) => /^f-/.test(s.id) && s.role === "body");
    for (const f of fr) {
      const key = f.shape.id.slice(2);
      const bone = f.shape.bone as string;
      const shade = /core/.test(key);
      const Pk = k.P((s) => s.id === `p-${key}`), Fk = k.F((s) => s.id === f.shape.id);
      if (!Pk.length) continue;
      const isCore = /core|^top$/.test(key);
      if (isCore) {
        const ekey = key === "top" ? "fol-top-mass" : `fol-${key}-mass`;
        const m = sunMeta("front", f.shape.id, shade);
        k.hull(`crown-${key}`, bone, m, { E: k.E((s) => s.id === ekey), F: Fk, step, pinch: 0.45, grow });
        for (const s of k.body.filter((q) => q.id === `crown-${key}` || q.id.startsWith(`crown-${key}-`))) regionOf.set(s.id, shade ? "leaf-shade" : "leaf-sun");
      } else if (S <= 100) {
        const fb = bboxOfShapes(Fk);
        const split = fb.minY + (fb.maxY - fb.minY) * LOWER;
        const pbb = bboxOfShapes(Pk);
        const edge = Math.max(Math.abs(pbb.minY), Math.abs(pbb.maxY)) > 2.3 || Math.max(Math.abs(pbb.minX), Math.abs(pbb.maxX)) > 2.5;
        const common = { E: allE, F: Fk, P: Pk, grow, minArea: (feat * 1.5) ** 2, shrink: edge ? Math.max(1, SHR) : SHR, areaK: edge ? 0 : Number(process.env.AK ?? 1) };
        k.hull(`lobe-${key}-under`, bone, sunMeta("front", f.shape.id, true), { ...common, step: step * 0.7, yRange: [fb.minY, split], pinchLo: 0.5, pinchHi: 0.97 });
        k.hull(`lobe-${key}-top`, bone, sunMeta("front", f.shape.id, false), { ...common, step, yRange: [split, fb.maxY], pinchLo: 0.97, pinchHi: 0.5 });
        for (const s of k.body.filter((q) => q.id.startsWith(`lobe-${key}-`))) regionOf.set(s.id, /-under/.test(s.id) ? "leaf-shade" : "leaf-sun");
      } else {
        k.hull(`lobe-${key}`, bone, sunMeta("front", f.shape.id, false), { E: allE, F: Fk, P: Pk, grow, step, minArea: (feat * 1.5) ** 2 });
        for (const s of k.body.filter((q) => q.id === `lobe-${key}` || q.id.startsWith(`lobe-${key}-`))) regionOf.set(s.id, "leaf-sun");
      }
      note(key, bone, Pk, Fk, shade ? "leaf-shade" : "leaf-sun");
    }
    if (S === 200) {
      for (const bone of ["fol-t2w", "fol-t2e", "fol-t3w", "fol-t3e", "fol-t1w", "fol-t1e"]) {
        const Em = k.E((q) => q.bone === bone && /-mass$/.test(q.id));
        const Fb = k.F((q) => q.bone === bone && /^f-/.test(q.id));
        if (!Em.length || !Fb.length) continue;
        k.hull(`pad-${bone}`, bone, sunMeta("elevation", `${bone}-mass`, false), { E: Em, F: Fb, P: k.P((q) => /^p-|^crown-fill/.test(q.id)), step: 0.08, grow: 0, growF: 0.1, shrink: Number(process.env.PADSH ?? 0.8), minArea: (feat * 1.5) ** 2, pinch: 0.7 });
        for (const q of k.body.filter((z) => z.id === `pad-${bone}` || z.id.startsWith(`pad-${bone}-`))) regionOf.set(q.id, "leaf-sun");
      }
    }
    if (S === 200) {
      const allF = k.F((s) => /^f-/.test(s.id));
      for (const pf of k.shapes("plan", (s) => /^crown-fill/.test(s.id))) {
        const Pk = k.P((s) => s.id === pf.shape.id);
        const id = `fill-${pf.shape.id.replace("crown-fill-", "")}`;
        const t2 = bboxOfShapes(k.F((s) => /^f-t2-/.test(s.id)));
        k.hull(id, pf.shape.bone, sunMeta("plan", pf.shape.id, false), { E: allE, F: allF, P: Pk, grow: 0, growF: Number(process.env.FG ?? 0.1), step: 0.07, minArea: (feat * 1.5) ** 2, yRange: [t2.minY, t2.maxY], pinch: 0.8 });
        for (const s of k.body.filter((q) => q.id === id || q.id.startsWith(`${id}-`))) regionOf.set(s.id, "leaf-sun");
      }
    }
  }

  const reg = (id: string) => k.asset.thermal.regions.find((r: any) => r.id === id);
  for (const s of k.body) {
    const region = regionOf.get(s.id);
    if (!region) continue;
    if ((S === 500 && s.id === "stems") || (S === 200 && /^wood-stem/.test(s.id))) {
      const o = JSON.parse(JSON.stringify(s));
      o.id = `th-${s.id}`;
      for (const q of o.sections) { q.rx *= S === 500 ? 1.3 : 1.12; q.ry *= S === 500 ? 1.3 : 1.12; }
      k.thermal.push({ ...o, region });
    } else k.thermal.push({ id: `th-${s.id}`, of: s.id, region });
  }
  const t = Math.max(0.02, feat * 2.2);
  for (const e of k.asset.thermal.emitters ?? []) {
    const r = reg(e.region);
    const anchors: string[] = e.anchors ?? [e.anchor];
    anchors.forEach((aid, i) => {
      const a = k.asset.anchors.find((x: any) => x.id === aid);
      if (!a) return;
      const w = toWorld(k.fr[a.bone]!, [a.x, a.y, a.z ?? 0]);
      const rr = Math.max(feat * 1.2, Math.min(e.diameterM / 2, 0.3)) * 0.5;
      k.loft("thermal", `emit-${e.id}-${i + 1}`, a.bone, [k.secY([w[0], w[1] - t / 2, w[2]], rr, rr, 2), k.secY([w[0], w[1] + t / 2, w[2]], rr, rr, 2)], { material: r.material, role: "body", tone: 0.5 }, { region: e.region, tags: [e.id, "emitter"] });
    });
  }

  const memberBones = k.memberBones();
  const rOf = new Map<string, number>();
  for (const id of memberBones) {
    const s = k.boneDef(id).section;
    rOf.set(id, Math.max(s?.extremeFibreM ?? (s?.areaM2 ? Math.sqrt(s.areaM2 / Math.PI) : 0.03), feat * 0.55));
  }
  const matOf = (id: string) => k.boneDef(id).section?.material ?? k.asset.materials[0].id;
  for (const id of memberBones) {
    const r0 = rOf.get(id)!;
    const path = paths.get(id);
    const meta = { material: matOf(id), role: "body", tone: 0.5 };
    const extra = { member: id, part: "member" };
    if (path && path.length >= 2) {
      const n = path.length;
      const secs = path.map((p, i) => {
        const hw = Math.min(p.rx, p.ry) * 0.78;
        const r = Math.max(feat * 0.55, Math.min(r0 * (1 - 0.25 * (i / (n - 1))), hw));
        return { ...p, rx: r, ry: r, exp: 2 };
      });
      k.loft("structure", `member-${id}`, id, secs, meta, extra);
      if (S <= 100 && k.boneDef(id).parent && memberBones.includes(k.boneDef(id).parent)) {
        const p0 = secs[0]!;
        const pp = paths.get(k.boneDef(id).parent)?.at(-1);
        const jr = Math.max(feat * 0.55, Math.min(p0.rx, pp ? pp.rx * 0.8 : p0.rx));
        k.ell("structure", `joint-${id}`, id, p0.c, [jr, jr, jr], { material: matOf(id), role: "detail", tone: 0.35 }, { member: id, part: "joint" });
      }
    }
  }
  const sceneFor = (id: string) => (id === "stem-c" || id === "stem-w" || id === "stem-e" || id === "stem-c-top") ? undefined : undefined;
  void sceneFor;
  for (const id of memberBones.filter((b) => !paths.has(b))) {
    const r0 = rOf.get(id)!;
    const a = k.origin(id), tp = k.tip(id), f = k.fr[id]!;
    const mk = (p: [number, number, number], r: number): Sec => ({ c: p, ax: f.v, ay: f.w, rx: r, ry: r, exp: 2 });
    const Es = k.E((q) => q.id === "stems"), Fs = k.F((q) => q.id === "stems");
    const clampIn = (p: [number, number, number], r: number): [number, number, number] => {
      if (!Es.length) return p;
      const eb = bboxOfShapes(Es);
      if (p[1] < eb.minY || p[1] > eb.maxY) return p;
      const ie = ivalsAll(Es, "y", p[1]), jf = ivalsAll(Fs, "y", p[1]);
      const cl = (v: number, iv: [number, number][]) => { if (!iv.length) return v; const w = iv.reduce((u, c) => (Math.abs((c[0] + c[1]) / 2 - v) < Math.abs((u[0] + u[1]) / 2 - v) ? c : u)); const m = Math.max(0, (w[1] - w[0]) / 2 - r * 1.1); return Math.min(w[0] + (w[1] - w[0]) / 2 + m, Math.max(w[0] + (w[1] - w[0]) / 2 - m, v)); };
      return [cl(p[0], ie), p[1], cl(p[2], jf)];
    };
    const pts = [0, 0.25, 0.5, 0.75, 1].map((t) => clampIn([a[0] + (tp[0] - a[0]) * t, a[1] + (tp[1] - a[1]) * t, a[2] + (tp[2] - a[2]) * t], r0 * (1 - 0.2 * t)));
    k.loft("structure", `member-${id}`, id, pts.map((p, i) => mk(p, r0 * (1 - 0.2 * (i / 4)))), { material: matOf(id), role: "body", tone: 0.5 }, { member: id, part: "member" });
  }
  const byB = new Map<string, typeof groups>();
  for (const g of groups) (byB.get(g.bone) ?? byB.set(g.bone, []).get(g.bone)!).push(g);
  const massList = S === 500 ? [groups[0]!] : [...byB.entries()].map(([bone, gs]) => {
    const lo = [Math.min(...gs.map((g) => g.c[0] - g.h[0])), Math.min(...gs.map((g) => g.yRange[0])), Math.min(...gs.map((g) => g.c[2] - g.h[2]))];
    const hi = [Math.max(...gs.map((g) => g.c[0] + g.h[0])), Math.max(...gs.map((g) => g.yRange[1])), Math.max(...gs.map((g) => g.c[2] + g.h[2]))];
    return { id: bone, bone, c: [(lo[0]! + hi[0]!) / 2, (lo[1]! + hi[1]!) / 2, (lo[2]! + hi[2]!) / 2] as [number, number, number], h: [(hi[0]! - lo[0]!) / 2, (hi[1]! - lo[1]!) / 2, (hi[2]! - lo[2]!) / 2] as [number, number, number], yRange: [lo[1]!, hi[1]!] as [number, number] };
  });
  for (const g of massList) {
    const sc = 0.22;
    k.ellW("structure", `mass-${g.id}`, g.bone, g.c, [Math.max(feat * 0.6, g.h[0] * sc), Math.max(feat * 0.6, g.h[1] * sc), Math.max(feat * 0.6, g.h[2] * sc)], { material: "maple-leaf-sun", role: "detail", tone: 0.3 }, { member: g.bone, part: "mass" });
  }
  for (const sid of k.asset.structure.supports ?? []) {
    const a = k.asset.anchors.find((x: any) => x.id === sid);
    const w = toWorld(k.fr[a.bone]!, [a.x, a.y, 0]);
    const h = Math.max(2 * feat, 0.04);
    k.boxW("structure", `support-${sid}`, a.bone, [w[0], w[1] + h / 2, w[2]], [Math.min(a.footprintLengthM ?? 0.5, 0.6), h, Math.min(a.footprintWidthM ?? 0.5, 0.6)], { material: matOf(a.bone), role: "detail", tone: 0.25 }, { member: a.bone, part: "support", anchor: sid, tags: [sid] });
  }
  const NOTES: Record<number, [string, string][]> = {
    50: [
      ["Body: each of the 18 leaf pads is a visual hull of its own elevation, front and plan outline, cut at 40 % of its height into an upper sun-leaf cap and a lower shade-leaf skin; sections are oriented superellipses, so a pad reads as a soft cushion, not a stack of slabs. The two shade cores and the top tuft are hulls of the elevation mass and the front outline. The root collar is a loft along its drawn flare; the three stems and the top stem are lofts along the drawn stem outlines (z centre and depth from the front outline) with dense sections at both ends; the six tier branches are tapered lofts along their bones.", "Körper: Jedes der 18 Laubpolster ist eine Sichthülle aus seinem eigenen Aufriss-, Seiten- und Grundrissumriss, bei 40 % der Höhe in eine obere Sonnenblattkappe und eine untere Schattenblatthaut geteilt; die Schnitte sind orientierte Superellipsen, ein Polster liest sich also als weiches Kissen und nicht als Plattenstapel. Die zwei Schattenkerne und der Wipfelbüschel sind Hüllen aus Aufrissmasse und Seitenumriss. Der Wurzelhals ist ein Loft entlang seiner gezeichneten Ausweitung, die drei Stämme und der Wipfelstamm sind Lofts entlang der gezeichneten Stammumrisse (Tiefenlage und Tiefe aus dem Seitenumriss) mit dichten Schnitten an beiden Enden, die sechs Etagenäste sind verjüngte Lofts entlang ihrer Knochen."],
      ["Generalised here: the scalloped pad edges are smoothed into superellipses (the drawn scallops are 30 to 60 mm deep); the leaf marks, arcs and lenticels are surface texture and have no solid. Pads are shrunk by 1.5 % so the plan edge stays inside the drawn outline; the pads at the depth extremes are not shrunk, to keep the model as deep as the drawing.", "Verallgemeinert: Die gewellten Polsterränder werden zu Superellipsen geglättet (die gezeichneten Wellen sind 30 bis 60 mm tief); Blattzeichen, Bögen und Lentizellen sind Oberflächentextur und haben keinen Körper. Die Polster sind um 1,5 % verkleinert, damit der Grundrissrand im gezeichneten Umriss bleibt; die Polster an den Tiefenenden sind nicht verkleinert, damit das Modell so tief ist wie die Zeichnung."],
      ["Thermal: bark regions and sun and shade leaf regions reference the body solids they are (stems, collar, tier branches, upper caps, lower skins and cores); transpiration emitters are small discs at the 12 emitter anchors. Exposed bark area is lower than declared because stems and branches are largely hidden inside the crown (ruling C1); areaM2 of stem-bark was corrected from 3.45 to 2.3 m2 at the root (exposed area measured 2.24 m2, within the factor 1.5 of C16); limb-bark keeps 1.37 m2 although only 0.65 m2 of the branches are exposed (listed in area-review.md, a finding of the model, not corrected).", "Thermik: Rinden- sowie Sonnen- und Schattenblattregionen verweisen auf die Körper, die sie sind (Stämme, Wurzelhals, Etagenäste, obere Kappen, untere Häute und Kerne); die Transpirationsemitter sind kleine Scheiben an den 12 Emitterankern. Die freiliegende Rindenfläche ist kleiner als angegeben, weil Stämme und Äste grossteils in der Krone liegen (Entscheid C1); areaM2 von stem-bark wurde von 3,45 auf 2,3 m2 an der Wurzel korrigiert (freiliegend gemessen 2,24 m2, innerhalb des Faktors 1,5 von C16); limb-bark behält 1,37 m2, obwohl nur 0,65 m2 der Äste freiliegen (in area-review.md aufgeführt, ein Befund des Modells, nicht korrigiert)."],
      ["Structure: the load path is member solids along every member bone with the bone section, clamped to 78 % of the half width of the body it runs in so that members stay inside the body silhouette; the stems follow the drawn stem paths, the tier members follow their branches; joints at the child origins, one mass ellipsoid per foliage bone group at 22 % of its half sizes, and the fixed root plate support pad at the support anchor.", "Tragwerk: Der Lastpfad besteht aus Stabkörpern entlang jedes Tragknochens mit dem Knochenquerschnitt, auf 78 % der halben Breite des Körpers begrenzt, in dem sie laufen, damit die Stäbe im Körperumriss bleiben; die Stämme folgen den gezeichneten Stammwegen, die Etagenstäbe ihren Ästen; Gelenke an den Ursprüngen der Kinder, je Laubknochengruppe ein Massenellipsoid mit 22 % der halben Grösse und das eingespannte Wurzelteller-Auflager am Auflagerpunkt."],
    ],
    100: [
      ["Body: 14 pads (four on the first tier, six on the second with the diagonal ones, four on the third) are visual hulls of their elevation, front and plan outlines in an upper sun cap and a lower shade skin; two cores and the top tuft are elevation-mass hulls; the stems and collar are lofts along the drawn outlines; the six tier branches are two-section tapered lofts. The four diagonal first-tier pads of 1:50 are no longer drawn at 1:100, so the neighbouring pads are 5 % fuller to carry their volume.", "Körper: 14 Polster (vier in der ersten Etage, sechs in der zweiten mit den diagonalen, vier in der dritten) sind Sichthüllen aus ihren Aufriss-, Seiten- und Grundrissumrissen mit oberer Sonnenkappe und unterer Schattenhaut; zwei Kerne und der Wipfelbüschel sind Hüllen der Aufrissmasse; Stämme und Wurzelhals sind Lofts entlang der gezeichneten Umrisse; die sechs Etagenäste sind verjüngte Lofts aus zwei Schnitten. Die vier diagonalen Polster der ersten Etage aus 1:50 werden bei 1:100 nicht mehr gezeichnet, die Nachbarpolster sind deshalb 5 % voller, um ihr Volumen zu tragen."],
      ["Generalised here: coarser sections (80 mm steps), fewer pad solids, no leaf-mark detail. Thermal references the same solids as at 1:50 (no solid is shared with 1:50 in a copied list: the lists are rebuilt against the 1:100 outlines). Structure as at 1:50 with joints; stem and branch members again lie inside the body.", "Verallgemeinert: gröbere Schnitte (80-mm-Schritte), weniger Polsterkörper, keine Blattzeichen. Die Thermik verweist auf dieselben Körper wie bei 1:50 (keine Liste ist kopiert: die Listen sind gegen die 1:100-Umrisse neu aufgebaut). Tragwerk wie bei 1:50 mit Gelenken; Stamm- und Aststäbe liegen wieder im Körper."],
    ],
    200: [
      ["Body: the root collar and three stems are lofts on the drawn outlines; two cores and the top are hulls; eight pads on the first to third tier are hulls of their three outlines; six pad hulls rebuild the tier masses of the elevation (shrunk to 80 % so they stay inside the plan); two plan-fill hulls model the diagonal second-tier lobes that the plan draws as fills; the six tier branches are slender capsules because the 2D no longer draws them but the load path crosses the gaps between the tiers. The front view at 1:200 was missing the four diagonal second-tier lobes of 1:100; they were added to the 2D drawing at the root (ruling C8a).", "Körper: Wurzelhals und drei Stämme sind Lofts auf den gezeichneten Umrissen; zwei Kerne und der Wipfel sind Hüllen; acht Polster der ersten bis dritten Etage sind Hüllen ihrer drei Umrisse; sechs Polsterhüllen bauen die Etagenmassen des Aufrisses nach (auf 80 % verkleinert, damit sie im Grundriss bleiben); zwei Grundrissfüllungs-Hüllen bilden die diagonalen Lappen der zweiten Etage, die der Grundriss als Füllungen zeichnet; die sechs Etagenäste sind schlanke Kapseln, weil das 2D sie nicht mehr zeichnet, der Lastpfad aber die Lücken zwischen den Etagen quert. Der Seitenansicht bei 1:200 fehlten die vier diagonalen Lappen der zweiten Etage aus 1:100; sie wurden an der Wurzel in die 2D-Zeichnung ergänzt (Entscheid C8a)."],
      ["Thermal: bark and leaf regions reference their body solids; the exposed bark area is within 30 % of the declared (corrected) area. Structure: members with the bone sections, no separate joint solids (the joints are merged into the member ends), one mass ellipsoid per foliage bone group, support pad.", "Thermik: Rinden- und Blattregionen verweisen auf ihre Körper; die freiliegende Rindenfläche liegt innerhalb von 30 % der angegebenen (korrigierten) Fläche. Tragwerk: Stäbe mit den Knochenquerschnitten, keine eigenen Gelenkkörper (die Gelenke sind in die Stabenden aufgenommen), je Laubknochengruppe ein Massenellipsoid, Auflagerpad."],
    ],
    500: [
      ["Body: the crown is one closed hull of the single crown outline in all three views (60 mm levels, oriented superellipse sections, shrunk to 98.5 %), the three fused stems are one hull of the drawn stem group, the collar a short loft and the six tier branches slender capsules inside the crown; the notches between the tiers are 150 mm deep and are reproduced by the hull levels.", "Körper: Die Krone ist eine geschlossene Hülle des einen Kronenumrisses in allen drei Ansichten (60-mm-Ebenen, orientierte Superellipsen, auf 98,5 % verkleinert), die drei verschmolzenen Stämme sind eine Hülle der gezeichneten Stammgruppe, der Wurzelhals ein kurzer Loft und die sechs Etagenäste schlanke Kapseln in der Krone; die Kerben zwischen den Etagen sind 150 mm tief und werden von den Hüllenebenen nachgebildet."],
      ["Generalised here: the pads and cores of 1:200 are merged into one crown envelope, so its volume is larger than that of the separate pads (the gaps between the tiers are filled, as the drawing does). Thermal: the crown references the crown hull as sun-leaf surface; stems as a bark skin 10 % fuller (bark relief); structure members stay inside the fused stems and the crown, one large mass ellipsoid, support pad.", "Verallgemeinert: Die Polster und Kerne von 1:200 sind zu einer Kronenhülle verschmolzen, ihr Volumen ist deshalb grösser als das der einzelnen Polster (die Lücken zwischen den Etagen sind gefüllt, wie es die Zeichnung tut). Thermik: Die Krone verweist auf die Kronenhülle als Sonnenblattfläche; Stämme als Rindenhaut, 10 % voller (Rindenrelief); Tragwerk: Stäbe bleiben in den verschmolzenen Stämmen und in der Krone, ein grosses Massenellipsoid, Auflagerpad."],
    ],
  };
  for (const [en, de] of NOTES[S] ?? []) k.note(en, de);
  out[S] = k.volume();
}
writeSource("japanese-maple", out);
