/** 🌲️ Author 21: Lombardy poplar, four scales, each modelled against its own 2D drawings: visual-hull crown lobes, bole on the drawn outline, branches along their bones, members inside the body, thermal and structure.
 * Usage: bun author-21-poplar.ts [scale]
 * @see ./author-21-lib.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { Kit, bboxOfShapes, ivalsAll, writeSource, type Sec, type Shapes } from "./author-21-lib.ts";
import { TICKET, toWorld } from "./volumes-lib.ts";

const SPEC = "vegetation/lombardy-poplar";
const only = process.argv[2] ? Number(process.argv[2]) : undefined;
const prior = JSON.parse(readFileSync(join(TICKET, "volumes", "vegetation", "lombardy-poplar.json"), "utf8")).volumes;
const out: Record<string, any> = {};
const BRANCHES = ["br-l1", "br-r1", "br-l2", "br-r2", "br-l3", "br-r3", "br-l4", "br-r4"];

for (const S of [50, 100, 200, 500] as const) {
  if (only && S !== only) { out[S] = prior[S]; continue; }
  const k = new Kit(SPEC, S);
  const feat = k.feat;
  const SHR = Number(process.env.SHR ?? (S === 50 ? 1 : S === 100 ? 1.02 : 1));
  const step = Number(process.env.STEP ?? (S === 50 ? 0.25 : S === 100 ? 0.4 : S === 200 ? 0.6 : 0.8));
  const grow = Number(process.env.GROW ?? (S === 100 ? 0.008 : S === 200 ? 0.04 : 0));
  const allE = k.E(() => true);
  const regionOf = new Map<string, string>();
  const paths = new Map<string, Sec[]>();
  const groups: { id: string; bone: string; c: [number, number, number]; h: [number, number, number]; yRange: [number, number] }[] = [];
  const bark = (view: "elevation" | "front" | "plan", id: string) => k.metaOf(view, (s) => s.id === id, { material: "poplar-bark" });
  const trunkIds = (view: "elevation" | "front" | "plan") => k.shapes(view, (s) => s.region === "stem-bark" && !/plug|dot/.test(s.id)).map((e) => e.shape.id as string);

  const barkShapes = [...new Set([...k.shapes("elevation", (s) => (s.region === "stem-bark") && !/plug|dot/.test(s.id)).map((e) => e.shape.id as string), ...k.shapes("front", (s) => s.region === "stem-bark" && !/plug|dot/.test(s.id)).map((e) => e.shape.id as string)])];
  void trunkIds;
  if (S !== 500 || true) {
    for (const id of barkShapes) {
      if (id === "root-flare") {
        const Ef = k.E((q) => q.id === id), eb = bboxOfShapes(Ef);
        const ys = [0, 0.06, 0.13, 0.2, 0.28, 0.35].map((y) => Math.min(eb.maxY, Math.max(eb.minY, y)));
        const secs = ys.map((y) => { const iv = ivalsAll(Ef, "y", Math.min(eb.maxY - 0.01, Math.max(eb.minY + 0.03, y))); const hw = iv.length ? (iv[0]![1] - iv[0]![0]) / 2 : 0.4; return k.secY([0, y, 0], hw, hw, 2); });
        k.loft("body", id, "root-plate", secs, bark("elevation", id));
        regionOf.set(id, "stem-bark");
        paths.set("root-plate", secs);
        continue;
      }
      const shape = [...k.shapes("elevation", (s) => s.id === id), ...k.shapes("front", (s) => s.id === id)][0]!;
      const hasE = k.E((s) => s.id === id).length > 0;
      const E = hasE ? k.E((s) => s.id === id) : [];
      const ok = k.limb(id, shape.shape.bone, E, k.F((s) => s.id === id), k.P((s) => s.id === id), hasE ? bark("elevation", id) : bark("front", id), { ends: true, n: id === "bole" ? 24 : undefined });
      if (ok) { regionOf.set(id, "stem-bark"); paths.set(`${id}`, k.lastPath); }
    }
  }
  if (!k.body.some((s) => s.bone === "root-plate")) {
    const Eb = k.E((s) => s.id === "bole"), eb = bboxOfShapes(Eb);
    const rr = [0, 0.1, 0.2, 0.3].map((y) => { const iv = ivalsAll(Eb, "y", Math.max(eb.minY + 0.02, y)); return iv.length ? (iv[0]![1] - iv[0]![0]) / 2 * 0.9 : 0.4; });
    k.loft("body", "root-flare", "root-plate", [0, 0.1, 0.2, 0.3].map((y, i) => k.secY([0, y, 0], rr[i]!, rr[i]!, 2)), bark("elevation", "bole"));
    regionOf.set("root-flare", "stem-bark");
    paths.set("root-plate", [0, 0.1, 0.2, 0.3].map((y, i) => k.secY([0, y, 0], rr[i]!, rr[i]!, 2)));
  }

  const branchShapes = [...new Set(k.shapes("elevation", (s) => s.region === "branch-bark").map((e) => e.shape.id as string))];
  const young = k.metaOf("elevation", (s) => s.region === "stem-bark" || true, { material: "poplar-bark" });
  for (const b of BRANCHES) {
    const id = branchShapes.find((x) => x === b);
    if (id && S <= 200) {
      const E = k.E((s) => s.id === id), F = k.F((s) => s.id === id);
      if (k.limb(id, b, E, F, null, k.metaOf("elevation", (s) => s.id === id, { material: "poplar-bark" }), { n: 5 })) {
        for (const q of k.body.filter((z) => z.id === id)) {
          for (const sec of q.sections) { sec.rx = Math.min(sec.rx, 0.09); sec.ry = Math.min(sec.ry, 0.09); }
        }
        regionOf.set(id, "branch-bark");
        paths.set(b, k.lastPath.map((p) => ({ ...p, rx: Math.min(p.rx, 0.09), ry: Math.min(p.ry, 0.09) })));
      }
    } else {
      const a = k.origin(b), t = k.tip(b), f = k.fr[b]!;
      const mk = (p: [number, number, number], r: number): Sec => ({ c: p, ax: f.v, ay: f.w, rx: r, ry: r, exp: 2 });
      const sec = [mk(a, 0.06), mk(t, 0.04)];
      k.loft("body", `limb-${b}`, b, sec, young, S >= 200 ? { slender: true } : {});
      regionOf.set(`limb-${b}`, "branch-bark");
      paths.set(b, sec);
    }
  }

  const sunMeta = (view: "elevation" | "front" | "plan", id: string) => k.metaOf(view, (s) => s.id === id, { material: "poplar-leaf" });
  if (S === 500) {
    const Ec = k.E((s) => s.id === "crown"), Fc = k.F((s) => s.id === "crown"), Pc = k.P((s) => s.id === "crown");
    k.hull("crown", "trunk-2", sunMeta("elevation", "crown"), { E: Ec, F: Fc, P: Pc, grow, step: Number(process.env.C5 ?? 0.3), shrink: Number(process.env.C5S ?? 1) });
    for (const s of k.body.filter((q) => q.id === "crown" || q.id.startsWith("crown-"))) regionOf.set(s.id, "leaf-crown");
    const eb = bboxOfShapes(Ec), pb = bboxOfShapes(Pc);
    groups.push({ id: "crown", bone: "trunk-2", c: [(eb.minX + eb.maxX) / 2, (eb.minY + eb.maxY) / 2, (pb.minY + pb.maxY) / 2], h: [(eb.maxX - eb.minX) / 2, (eb.maxY - eb.minY) / 2, (pb.maxY - pb.minY) / 2], yRange: [eb.minY, eb.maxY] });
  } else {
    const fr = k.shapes("front", (s) => /^f-/.test(s.id) && s.role === "body");
    for (const f of fr) {
      const key = f.shape.id.slice(2);
      const bone = f.shape.bone as string;
      const Pk = k.P((s) => s.id === `p-${key}`), Fk = k.F((s) => s.id === f.shape.id);
      if (!Pk.length) continue;
      const Ebone = k.E((s) => s.id === bone);
      const m = sunMeta("front", f.shape.id);
      if (bone === "fol-top") k.hull(`lobe-${key}`, bone, m, { E: Ebone, F: Fk, P: Pk, grow, step, shrink: SHR, minArea: (feat * 1.5) ** 2 });
      else k.hull(`lobe-${key}`, bone, m, { E: allE, F: Fk, P: Pk, grow, step, shrink: SHR, areaK: Number(process.env.AK ?? 0), minArea: (feat * 1.5) ** 2 });
      for (const s of k.body.filter((q) => q.id === `lobe-${key}` || q.id.startsWith(`lobe-${key}-`))) regionOf.set(s.id, "leaf-crown");
      const pb = bboxOfShapes(Pk), fb = bboxOfShapes(Fk);
      groups.push({ id: key, bone, c: [(pb.minX + pb.maxX) / 2, (fb.minY + fb.maxY) / 2, (pb.minY + pb.maxY) / 2], h: [(pb.maxX - pb.minX) / 2, (fb.maxY - fb.minY) / 2, (pb.maxY - pb.minY) / 2], yRange: [fb.minY, fb.maxY] });
    }
  }

  const reg = (id: string) => k.asset.thermal.regions.find((r: any) => r.id === id);
  for (const s of k.body) {
    const region = regionOf.get(s.id);
    if (region) k.thermal.push({ id: `th-${s.id}`, of: s.id, region });
  }
  const t = Math.max(0.05, feat * 3);
  for (const e of k.asset.thermal.emitters ?? []) {
    const r = reg(e.region);
    const anchors: string[] = e.anchors ?? [e.anchor];
    anchors.forEach((aid, i) => {
      const a = k.asset.anchors.find((x: any) => x.id === aid);
      if (!a) return;
      const w = toWorld(k.fr[a.bone]!, [a.x, a.y, a.z ?? 0]);
      const rr = Math.min(0.3, feat * 8);
      k.loft("thermal", `emit-${e.id}-${i + 1}`, a.bone, [k.secY([w[0], w[1] - t / 2, w[2]], rr, rr, 2), k.secY([w[0], w[1] + t / 2, w[2]], rr, rr, 2)], { material: r.material, role: "body", tone: 0.5 }, { region: e.region, tags: [e.id, "emitter"] });
    });
  }

  const memberBones = k.memberBones();
  const matOf = (id: string) => k.boneDef(id).section?.material ?? k.asset.materials[0].id;
  const boleSecs = paths.get("bole") ?? [...paths.entries()].filter(([n]) => /^trunk-\d$/.test(n)).flatMap(([, v]) => v);
  const trunkSecs = (y0: number, y1: number, r0: number): Sec[] => {
    const ids = barkShapes;
    const Es = ids.flatMap((i) => k.E((s) => s.id === i)), Fs = ids.flatMap((i) => k.F((s) => s.id === i));
    const out: Sec[] = [];
    const nsec = S === 50 ? 8 : S === 100 ? 4 : 2;
    for (let i = 0; i <= nsec; i++) {
      const y = y0 + ((y1 - y0) * i) / nsec;
      const ie = ivalsAll(Es, "y", y), jf = ivalsAll(Fs, "y", y);
      const cx = ie.length ? (ie[0]![0] + ie[ie.length - 1]![1]) / 2 : 0, hx = ie.length ? (ie[ie.length - 1]![1] - ie[0]![0]) / 2 : r0;
      const cz = jf.length ? (jf[0]![0] + jf[jf.length - 1]![1]) / 2 : 0, hz = jf.length ? (jf[jf.length - 1]![1] - jf[0]![0]) / 2 : r0;
      const r = Math.max(feat * 0.55, Math.min(r0 * (1 - 0.15 * (i / nsec)), 0.78 * Math.min(hx, hz)));
      out.push(k.secY([cx, y, cz], r, r, 2));
    }
    return out;
  };
  void boleSecs;
  const rOf = new Map<string, number>();
  for (const id of memberBones) {
    const s = k.boneDef(id).section;
    rOf.set(id, Math.max(s?.extremeFibreM ?? (id.startsWith("br-") ? 0.07 : id.startsWith("fol-") ? 0.05 : 0.04), feat * 0.55));
  }
  for (const id of memberBones) {
    const r0 = rOf.get(id)!;
    const meta = { material: matOf(id), role: "body", tone: 0.5 };
    const extra = { member: id, part: "member" };
    let secs: Sec[];
    if (/^trunk-\d$/.test(id) || id === "root-plate") {
      const o = k.origin(id), tp = k.tip(id);
      const lo = id === "root-plate" ? o[1] + 0.05 : o[1], hi = id === "root-plate" ? tp[1] - 0.02 : tp[1];
      secs = trunkSecs(lo, hi, r0);
      paths.set(id, secs);
    } else if (id.startsWith("br-") && paths.has(id)) {
      const p = paths.get(id)!;
      secs = p.map((q, i) => { const r = Math.max(feat * 0.55, Math.min(r0 * (1 - 0.3 * (i / Math.max(1, p.length - 1))), 0.78 * Math.min(q.rx, q.ry))); return { ...q, rx: r, ry: r, exp: 2 }; });
    } else {
      const a = k.origin(id), tp = k.tip(id), f = k.fr[id]!;
      const mk = (p: [number, number, number], r: number): Sec => ({ c: p, ax: f.v, ay: f.w, rx: r, ry: r, exp: 2 });
      secs = [mk(a, r0), mk([(a[0] + tp[0]) / 2, (a[1] + tp[1]) / 2, (a[2] + tp[2]) / 2], r0 * 0.9), mk(tp, r0 * 0.8)];
    }
    k.loft("structure", `member-${id}`, id, secs, meta, extra);
    if (S === 50 && k.boneDef(id).parent && memberBones.includes(k.boneDef(id).parent) && !/^fol-/.test(id)) {
      const p0 = secs[0]!;
      const jr = Math.max(feat * 0.55, Math.min(p0.rx, 0.1));
      k.ell("structure", `joint-${id}`, id, p0.c, [jr, jr, jr], { material: matOf(id), role: "detail", tone: 0.35 }, { member: id, part: "joint" });
    }
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
    k.ellW("structure", `mass-${g.id}`, g.bone, g.c, [Math.max(feat * 0.6, g.h[0] * sc), Math.max(feat * 0.6, g.h[1] * sc), Math.max(feat * 0.6, g.h[2] * sc)], { material: "poplar-leaf", role: "detail", tone: 0.3 }, { member: g.bone, part: "mass" });
  }
  for (const sid of k.asset.structure.supports ?? []) {
    const a = k.asset.anchors.find((x: any) => x.id === sid);
    const w = toWorld(k.fr[a.bone]!, [a.x, a.y, 0]);
    const h = Math.max(2 * feat, 0.04);
    k.boxW("structure", `support-${sid}`, a.bone, [w[0], w[1] + h / 2, w[2]], [Math.min(a.footprintLengthM ?? 0.5, 0.6), h, Math.min(a.footprintWidthM ?? 0.5, 0.6)], { material: matOf(a.bone), role: "detail", tone: 0.25 }, { member: a.bone, part: "support", anchor: sid, tags: [sid] });
  }
  const NOTES: Record<number, [string, string][]> = {
    50: [
      ["Body: each of the 25 crown lobes (six per tier pair and the top) is a visual hull of its own elevation mass, front outline and plan outline with oriented superellipse sections every 250 mm; the four trunk pieces, their plugs and the eight branches are lofts along their drawn outlines (centre and depth from the front outline), the root flare a circular loft on its drawn flare. The alternating left and right offset of the lobes is kept, so the crown reads as a column of tufts, not as one cylinder.", "Körper: Jeder der 25 Kronenlappen (sechs je Etagenpaar und der Wipfel) ist eine Sichthülle aus seiner Aufrissmasse, seinem Seitenumriss und seinem Grundrissumriss mit orientierten Superellipsen-Schnitten alle 250 mm; die vier Stammstücke, ihre Zapfen und die acht Äste sind Lofts entlang ihrer gezeichneten Umrisse (Lage und Tiefe aus dem Seitenumriss), der Wurzelanlauf ein kreisrunder Loft auf der gezeichneten Ausweitung. Der Wechsel der Lappen nach links und rechts bleibt erhalten, die Krone liest sich also als Säule aus Büscheln und nicht als ein Zylinder."],
      ["Generalised here: the scalloped lobe edges become smooth superellipses; leaf marks and arcs are surface texture. Thermal: bark and leaf regions reference the body solids; nine transpiration discs sit at the emitter anchors. The trunk and the branches lie inside the crown lobes, so only about 3 m2 of the declared 30 m2 stem bark and none of the declared 10 m2 branch bark is exposed; areaM2 is not changed (beyond the factor 1.5 of C16, listed in area-review.md).", "Verallgemeinert: Die gewellten Lappenränder werden zu glatten Superellipsen; Blattzeichen und Bögen sind Oberflächentextur. Thermik: Rinden- und Blattregionen verweisen auf die Körper; neun Transpirationsscheiben sitzen an den Emitterankern. Stamm und Äste liegen in den Kronenlappen, es liegen daher nur etwa 3 m2 der angegebenen 30 m2 Stammrinde und nichts der angegebenen 10 m2 Astrinde frei; areaM2 bleibt unverändert (über dem Faktor 1,5 von C16, in area-review.md aufgeführt)."],
      ["Structure: member solids for the root plate, the four trunk bones (along the drawn bole, radius 78 % of its half width at most), the eight branches (along the body branches) and the nine crown bones (bone line, 50 mm radius), joints at the branch and trunk origins, one mass ellipsoid per crown bone group (22 % of the half sizes, aligned to the world axes) and the fixed root plate support pad.", "Tragwerk: Stabkörper für den Wurzelteller, die vier Stammknochen (entlang des gezeichneten Schaftes, Radius höchstens 78 % der halben Breite), die acht Äste (entlang der Astkörper) und die neun Kronenknochen (Knochenlinie, 50 mm Radius), Gelenke an den Ursprüngen von Ästen und Stamm, je Kronenknochengruppe ein Massenellipsoid (22 % der halben Grösse, an den Weltachsen ausgerichtet) und das eingespannte Wurzelteller-Auflager."],
    ],
    100: [
      ["Body: 17 lobes (four per tier and the top) are visual hulls of their three outlines at 400 mm levels and 2 % fuller than the drawing to keep the depth; the bole is one loft along its drawn outline (the 2D draws the four trunk pieces as one bole), a root flare loft closes the root plate bone; the eight branches are lofts along their drawn outlines.", "Körper: 17 Lappen (vier je Etage und der Wipfel) sind Sichthüllen ihrer drei Umrisse auf 400-mm-Ebenen und 2 % voller als die Zeichnung, um die Tiefe zu halten; der Schaft ist ein Loft entlang seines gezeichneten Umrisses (das 2D zeichnet die vier Stammstücke als einen Schaft), ein Loft des Wurzelanlaufs schliesst den Wurzeltellerknochen; die acht Äste sind Lofts entlang ihrer gezeichneten Umrisse."],
      ["Generalised here: the two diagonal lobes per tier of 1:50 are merged into the neighbouring lobes, trunk pieces and plugs are merged into one bole. Thermal and structure are rebuilt for this body (coarser member sections, no joints, emitter discs 120 mm).", "Verallgemeinert: Die zwei diagonalen Lappen je Etage aus 1:50 sind in die Nachbarlappen aufgegangen, Stammstücke und Zapfen zu einem Schaft verschmolzen. Thermik und Tragwerk sind für diesen Körper neu aufgebaut (gröbere Stabquerschnitte, keine Gelenke, Emitterscheiben 120 mm)."],
    ],
    200: [
      ["Body: 17 lobes as hulls of their three outlines at 600 mm levels (grown by 40 mm so the crown keeps its drawn depth), one bole loft, a root flare loft and eight branch lofts; the structure members stay inside the body.", "Körper: 17 Lappen als Hüllen ihrer drei Umrisse auf 600-mm-Ebenen (um 40 mm vergrössert, damit die Krone ihre gezeichnete Tiefe behält), ein Schaftloft, ein Loft des Wurzelanlaufs und acht Astlofts; die Tragwerkstäbe bleiben im Körper."],
      ["Generalised here: coarser sections, 100-mm class of detail dropped; thermal: nine emitter discs of 240 mm; structure: members with two-section trunks, no joints, one mass ellipsoid per bone group, support pad.", "Verallgemeinert: gröbere Schnitte, Details der 100-mm-Klasse entfallen; Thermik: neun Emitterscheiben zu 240 mm; Tragwerk: Stäbe mit Stämmen aus zwei Schnitten, keine Gelenke, je Knochengruppe ein Massenellipsoid, Auflagerpad."],
    ],
    500: [
      ["Body: the crown is one closed hull of its single outline in the three views at 300 mm levels, the bole a loft on its drawn outline and a root flare, the eight branches slender capsules inside the crown (the 2D no longer draws them).", "Körper: Die Krone ist eine geschlossene Hülle ihres einen Umrisses in den drei Ansichten auf 300-mm-Ebenen, der Schaft ein Loft auf seinem gezeichneten Umriss mit Wurzelanlauf, die acht Äste schlanke Kapseln in der Krone (das 2D zeichnet sie nicht mehr)."],
      ["Generalised here: the eight tiers are merged into one crown, so only the tufted outline remains (notches of 150 mm). Thermal: crown surface as leaf area, bole bark, branch capsules; structure: members inside bole and crown, one mass ellipsoid, support pad.", "Verallgemeinert: Die acht Etagen sind zu einer Krone verschmolzen, nur der gebüschelte Umriss bleibt (Kerben von 150 mm). Thermik: Kronenoberfläche als Blattfläche, Schaftrinde, Astkapseln; Tragwerk: Stäbe in Schaft und Krone, ein Massenellipsoid, Auflagerpad."],
    ],
  };
  for (const [en, de] of NOTES[S] ?? []) k.note(en, de);
  out[S] = k.volume();
}
writeSource("lombardy-poplar", out);
