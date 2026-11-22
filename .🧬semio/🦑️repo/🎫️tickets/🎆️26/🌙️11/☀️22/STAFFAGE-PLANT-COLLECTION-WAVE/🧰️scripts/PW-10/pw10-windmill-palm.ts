/** 🪭️ Windmill palm (Trachycarpus fortunei), stated by hand in world space in the calm pose: a slender single stem of about 6.3 m wrapped in the brown fibre of the old leaf sheaths (stubs of cut petioles above, ring scars below), a fibrous crown boot, twenty stiff fan leaves in four tiers (petiole with toothed margin, a palmate blade of 36 induplicate segments split to about half its radius, tips drooping) and five dead leaves hanging folded below the crown.
 * Measures: Flora of China vol. 23 (Arecaceae, Trachycarpus fortunei); Dransfield et al. (2008) Genera Palmarum; Gibbons (2003) A Pocket Guide to Palms; see `📝️PW-10.md`.
 * Usage: `repo tools exec bun -- pw10-windmill-palm.ts [--ship] [--no-2d]`
 * @see ./pw10-plant.ts */
import { D2R, FrondGeom, STILL, add, buildDocument, crownTable, drag, emptyParts, featureOf, gridSheet, mul, r4, standardClips, steps, sub, table, text, unit, weldSheet, type Frame, type Json, type Leaf, type P2, type Parts, type Scale, type Species, type Tube, type V3, type View } from "./pw10-plant.ts";
import { project, sweep, cross, dot } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-date-palm-kit.ts";

const ID = "windmill-palm";
const FIBRE = { material: "hemp-palm-fibre", role: "body", tone: 0.3 };
const LEAF = { material: "hemp-palm-leaf", role: "body", tone: 0.52 };
const DEAD = { material: "dead-palm-leaf", role: "body", tone: 0.34 };

const greenLabel = text("Fan leaf", "Fächerblatt");
const leaves: Leaf[] = [
  ...crownTable({
    crown: "crown", prefix: "lf", az0: 12,
    tiers: [
      { tier: "A", count: 5, elA: -6, elB: -30, La: 0.74, Lb: 0.6, baseU: 0.08, baseR: 0.15, massA: 0.3, massB: 0.45, label: greenLabel },
      { tier: "B", count: 6, elA: 18, elB: -2, La: 0.8, Lb: 0.6, baseU: 0.18, baseR: 0.14, massA: 0.3, massB: 0.45, label: greenLabel },
      { tier: "C", count: 5, elA: 44, elB: 26, La: 0.8, Lb: 0.58, baseU: 0.3, baseR: 0.12, massA: 0.28, massB: 0.42, label: greenLabel },
      { tier: "D", count: 4, elA: 68, elB: 57, La: 0.74, Lb: 0.54, baseU: 0.41, baseR: 0.09, massA: 0.25, massB: 0.38, label: greenLabel },
    ],
  }),
  ...crownTable({ crown: "stem-3", prefix: "dl", az0: 70, divergence: 151, tiers: [{ tier: "A", count: 5, elA: -72, elB: -84, La: 0.58, Lb: 0.42, baseU: 1.38, baseR: 0.14, massA: 0.12, massB: 0.15, state: "dead", label: text("Dead leaf", "Totes Blatt") }] }),
];
const isLeafBone = (bone: string) => /^(lf|dl)-[lr]\d+[ab]$/.test(bone);

/** 📏️ Stem radius over the fibre coat at a world height. */
const stemR = (y: number): number => table([[0, 0.16], [0.08, 0.14], [0.2, 0.122], [0.6, 0.112], [2.0, 0.118], [4.0, 0.128], [5.4, 0.138], [6.3, 0.15]], y);

/** 🎛️ What each scale shows. */
const LOD = {
  50: { segs: 36, split: 1, pleat: 0, droop: 0.12, half: 0.004, inner: 6, outer: 0, stemStep: 0.25, stubStep: 0.04, rings: true, strokes: 1, bands: 7 },
  100: { segs: 18, split: 1, pleat: 0, droop: 0.12, half: 0.0075, inner: 4, outer: 0, stemStep: 0.5, stubStep: 0.08, rings: true, strokes: 2, bands: 6 },
  200: { segs: 12, split: 1, pleat: 0, droop: 0.12, half: 0.016, inner: 3, outer: 0, stemStep: 1.0, stubStep: 0, rings: false, strokes: 0, bands: 5 },
  500: { segs: 8, split: 1, pleat: 0, droop: 0.12, half: 0.04, inner: 2, outer: 0, stemStep: 2.0, stubStep: 0, rings: false, strokes: 0, bands: 4 },
} as const;

/** 🪭️ The blade of one fan leaf as a sheet on bone b: palmate, `segs` induplicate segments over ±Θ about the hastula, split from `split` of the radius to the tips, tips drooping. Dead leaves fold to a narrow closed fan. */
function fanSheet(f: Leaf, B: Frame, scale: Scale, P: Parts): void {
  const lod = LOD[scale];
  const feature = featureOf(scale);
  const dead = f.state === "dead";
  const Theta = (dead ? 38 : 132) * D2R;
  const N = dead ? Math.max(4, Math.round(lod.segs / 3)) : lod.segs;
  const R = (th: number) => f.Lb * (0.8 + 0.2 * Math.cos(th * 0.95)) * (dead ? 1 : 1);
  const dth = (2 * Theta) / N;
  const r0 = 0.012;
  const half = Math.max(lod.half, feature * 0.52);
  const pleat = (r: number, edge: boolean) => (lod.pleat > 0 ? (edge ? 1 : -1) * lod.pleat * (0.3 + 0.7 * Math.min(1, r / (0.6 * f.Lb))) : 0);
  const droopU = (u: number) => -lod.droop * Math.max(0, (u - 0.25 * f.Lb) / (0.75 * f.Lb)) ** 2;
  const loc = (u: number, v: number, w: number): V3 => add(B.o, add(add(mul(B.u, u), mul(B.v, v)), mul(B.w, w)));
  const cols = 2 * N + 1;
  const angle = (j: number) => -Theta + (j * dth) / 2;
  const innerRows = lod.inner + 1;
  const rsOf = (th: number) => r0 + (R(th) - r0) * lod.split;
  const sheet = gridSheet(innerRows, cols, (i, j) => {
    const th = angle(j);
    const rs = rsOf(th);
    const r = r0 + ((rs - r0) * i) / lod.inner;
    const v = pleat(r, j % 2 === 0) + droopU(r * Math.cos(th));
    return { p: loc(r * Math.cos(th), v, r * Math.sin(th)), n: B.v };
  });
  if (lod.outer > 0) for (let k = 0; k < N; k++) {
    const thc = angle(2 * k + 1);
    const rt = R(thc), rs = rsOf(thc);
    const strip = gridSheet(lod.outer + 1, 3, (i, j) => {
      const s = i / lod.outer;
      const th0 = angle(2 * k + j);
      const narrow = 1 - (dead ? 0.5 : 0.88) * s;
      const th = thc + (th0 - thc) * narrow;
      const r = (j === 1 ? rsOf(th0) : rsOf(th0)) + (rt - rsOf(th0)) * s * (j === 1 ? 1 : 0.97);
      const v = pleat(r, j !== 1) + droopU(r * Math.cos(th));
      return { p: loc(r * Math.cos(th), v, r * Math.sin(th)), n: B.v };
    });
    const base = (innerRows - 1) * cols + 2 * k;
    weldSheet(sheet, strip, [[0, base], [1, base + 1], [2, base + 2]]);
  }
  const id = `${f.id}b-blade`;
  const innerFaces = lod.inner * (cols - 1);
  const faceU = sheet.faces.map((face, fi) => {
    if (fi < innerFaces) return face.reduce((a, i) => a + dot(sub(sheet.p[i]!, B.o), B.u), 0) / face.length;
    const k = Math.floor((fi - innerFaces) / (2 * lod.outer));
    const thc = angle(2 * k + 1);
    return rsOf(thc) * Math.cos(thc);
  });
  P.sheets.push({ id, bone: `${f.id}b`, ...sheet, half, fields: dead ? { ...DEAD } : { ...LEAF }, faceU });
  P.regions[id] = dead ? "dead-leaves" : "leaves";
  if (lod.strokes > 0 && !dead) {
    const lines: { a: V3; b: V3 }[] = [];
    for (let k = 0; k < N; k += lod.strokes) {
      const thc = angle(2 * k + 1);
      const rt = R(thc), rs = rsOf(thc);
      const a = 0.12 * rt, b = 0.93 * rt;
      const v = (r: number) => pleat(r, false) + droopU(r * Math.cos(thc)) + half * 1.05;
      lines.push({ a: loc(a * Math.cos(thc), v(a), a * Math.sin(thc)), b: loc(b * Math.cos(thc), v(b), b * Math.sin(thc)) });
    }
    P.strokes.push({ id: `${f.id}b-folds`, bone: `${f.id}b`, lines, role: "texture" });
  }
  const thm = dead ? 0 : angle(2 * Math.floor(N / 2) + 1);
  const mid = (r: number) => {
    const rt = R(thm), rs = rsOf(thm);
    return loc(r * Math.cos(thm), pleat(r, false) + droopU(r * Math.cos(thm)), r * Math.sin(thm));
  };
  const mr = Math.max(half * 0.42, 0.0015);
  P.members.push({ id: `m-${f.id}b`, bone: `${f.id}b`, sections: sweep(steps(0, 0.8 * R(thm), 0.8 * R(thm) / 4).map((r) => ({ p: mid(r), a: mr, b: mr })), B.v), fields: { material: "hemp-palm-stem", role: "body", tone: 0.5, slender: true } });
}

/** 🌿️ The petiole of one leaf on bone a: an arched, flattened stalk with a widened fibrous base inside the crown boot. */
function petiole(f: Leaf, F: Record<string, Frame>, scale: Scale, P: Parts): void {
  const feature = featureOf(scale);
  const g = new FrondGeom(f, F[`${f.id}a`]!, F[`${f.id}b`]!);
  const S = steps(-0.08, f.La + 0.035, scale === 50 ? 0.09 : scale === 100 ? 0.16 : 0.3);
  const rw = (s: number) => table([[-0.08, 0.03], [0, 0.026], [0.08, 0.014], [f.La, 0.011]], s) * (f.state === "dead" ? 0.85 : 1);
  const rd = (s: number) => table([[-0.08, 0.02], [0, 0.017], [0.08, 0.0085], [f.La, 0.0065]], s) * (f.state === "dead" ? 0.85 : 1);
  const minHalf = feature * 0.55;
  const sections = S.map((s) => {
    const q = g.at(s, "a");
    return { p: q.p, x: q.n, y: q.w, a: Math.max(rd(s), minHalf), b: Math.max(rw(s), minHalf) };
  });
  const id = `${f.id}a-petiole`;
  P.tubes.push({ id, bone: `${f.id}a`, sections, fields: f.state === "dead" ? { ...DEAD } : { ...LEAF, tone: 0.56 } });
  P.regions[id] = f.state === "dead" ? "dead-leaves" : "leaves";
  P.members.push({ id: `m-${f.id}a`, bone: `${f.id}a`, sections: [{ ...sections[0]!, p: g.at(-f.baseR, "a").p }, ...sections.filter((_, i) => i > 0)].map((s) => ({ ...s, a: s.a * 0.7, b: s.b * 0.7 })), fields: { material: "hemp-palm-stem", role: "body", tone: 0.5, slender: true } });
}

/** 🪵️ Every part of the windmill palm at one scale. */
function parts(scale: Scale, F: Record<string, Frame>): Parts {
  const lod = LOD[scale];
  const feature = featureOf(scale);
  const P = emptyParts();
  const chain: [string, number, number][] = [["root-plate", 0, 0.3], ["stem-1", 0.3, 2.5], ["stem-2", 2.5, 4.7], ["stem-3", 4.7, 6.3]];
  for (const [bone, y0, y1] of chain) {
    const fr = F[bone]!;
    const from = bone === "root-plate" ? 0 : -0.04, to = y1 - y0;
    const us = bone === "root-plate" ? [0, 0.05, 0.12, 0.2, 0.3] : steps(from, to, lod.stemStep);
    const sections = sweep(us.map((u) => {
      const p = add(fr.o, mul(fr.u, u));
      return { p, a: stemR(p[1]) * 1.01, b: stemR(p[1]) * 0.99, n: 2.1 };
    }), fr.v);
    const id = bone === "root-plate" ? "root-collar" : bone;
    P.tubes.push({ id, bone, sections, fields: { ...FIBRE, tone: bone === "stem-1" ? 0.36 : 0.3 } });
    P.regions[id] = "trunk-fibre";
    P.groups[id] = { bone, region: "trunk-fibre", role: "body", tone: bone === "stem-1" ? 0.36 : 0.3, material: "hemp-palm-fibre", parts: [id] };
    const core = (bone === "root-plate" ? 0.1 : bone === "stem-1" ? 0.1 : bone === "stem-2" ? 0.095 : 0.09);
    P.members.push({ id: `m-${bone}`, bone, sections: sweep([0, to].map((u) => ({ p: add(fr.o, mul(fr.u, u)), a: core, b: core })), fr.v), fields: { material: "hemp-palm-stem", role: "body", tone: 0.5 } });
  }
  const crown = F["crown"]!;
  const bootRows: [number, number][] = scale === 500 ? [[-0.05, 0.15], [0.2, 0.19], [0.42, 0.15], [0.55, 0.06]] : [[-0.05, 0.15], [0.08, 0.17], [0.2, 0.185], [0.32, 0.175], [0.42, 0.145], [0.5, 0.1], [0.56, 0.045]];
  const boot: Tube = { id: "crown-boot", bone: "crown", sections: sweep(bootRows.map(([u, r]) => ({ p: add(crown.o, mul(crown.u, u)), a: r, b: r * 0.97, n: 2.2 })), crown.v), fields: { ...FIBRE, tone: 0.27 } };
  P.tubes.push(boot);
  P.regions["crown-boot"] = "trunk-fibre";
  P.groups["crown-boot"] = { bone: "crown", region: "trunk-fibre", role: "body", tone: 0.27, material: "hemp-palm-fibre", parts: ["crown-boot"] };
  P.members.push({ id: "m-crown", bone: "crown", sections: sweep([0, 0.5].map((u) => ({ p: add(crown.o, mul(crown.u, u)), a: 0.085, b: 0.085 })), crown.v), fields: { material: "hemp-palm-stem", role: "body", tone: 0.5 } });
  if (scale !== 500) {
    const spearLen = 0.62;
    const pts = steps(0, 1, scale === 50 ? 0.2 : 0.5).map((s) => ({ p: add(crown.o, add(mul(crown.u, 0.42 + spearLen * s), [0.02 * s, 0, 0])), a: Math.max(feature * 0.55, 0.024 * (1 - s) + 0.006), b: Math.max(feature * 0.55, 0.03 * (1 - 0.8 * s)) }));
    P.tubes.push({ id: "spear", bone: "crown", sections: sweep(pts, [1, 0, 0]), fields: { ...LEAF, tone: 0.6 } });
    P.regions["spear"] = "leaves";
    P.groups["spear"] = { bone: "crown", region: "leaves", role: "body", tone: 0.6, material: "hemp-palm-leaf", parts: ["spear"] };
  }
  for (const f of leaves) {
    petiole(f, F, scale, P);
    fanSheet(f, F[`${f.id}b`]!, scale, P);
  }
  if (lod.stubStep > 0) {
    for (const [bone, from, to] of [["stem-2", 1.0, 2.2], ["stem-3", 0.0, 1.6]] as [string, number, number][]) {
      const fr = F[bone]!;
      const count = Math.floor((to - from) / lod.stubStep);
      const r = stemR(fr.o[1] + from) - 0.015;
      const [h, depth, wide] = scale === 50 ? [0.05, 0.03, 0.07] : [0.07, 0.035, 0.09];
      const tilt = 28 * D2R;
      P.repeats.push({ id: `${bone}-stubs`, kind: "box", bone, material: "hemp-palm-fibre", role: "texture", tone: 0.24, size: [h, depth, wide], cornerRadius: Math.min(0.008, depth / 3), texture: true, transform: { translation: [from, r + depth / 2, 0], rotation: [0, 0, -Math.sin(tilt / 2), Math.cos(tilt / 2)].map((x) => Math.round(x * 1e5) / 1e5) }, repeat: { count, rotationAxis: [1, 0, 0], rotationRad: 2.39996, translation: [Math.round(((to - from) / count) * 1e5) / 1e5, 0, 0] } });
      P.worldTexture.push({ id: `${bone}-stubs`, bone, role: "texture", polys: (view) => stubLines(view, fr, from, to, count, r, h, depth, wide) });
    }
  }
  if (lod.rings) {
    const fr = F["stem-1"]!;
    const step = scale === 50 ? 0.09 : 0.18;
    P.worldTexture.push({ id: "stem-1-rings", bone: "stem-1", role: "texture", polys: (view) => ringLines(view, fr, 0.3, 2.1, step) });
  }
  const anchorLeaves = leaves.filter((f) => f.state === "green" && f.tier !== "D").slice(0, 12);
  anchorLeaves.forEach((f, i) => {
    const B = F[`${f.id}b`]!;
    const half = Math.max(lod.half, feature * 0.52);
    const patch = Math.max(0.0025, feature * 0.52);
    const lift = (lod.pleat > 0 ? lod.pleat : 0) + half + patch + 0.002;
    const us = steps(0.12 * f.Lb, 0.5 * f.Lb, 0.38 * f.Lb / 3);
    const droop = (u: number) => -lod.droop * Math.max(0, (u - 0.25 * f.Lb) / (0.75 * f.Lb)) ** 2;
    const sections = us.map((u) => ({ p: add(B.o, add(mul(B.u, u), mul(B.v, lift + droop(u)))), x: B.v, y: B.w, a: patch, b: Math.max(0.05, feature * 0.6) }));
    const uc = 0.31 * f.Lb;
    P.emitters.push({ id: `th-tp-leaves-${i + 1}`, anchor: `tp-leaves-${i + 1}`, bone: `${f.id}b`, tube: { id: `th-tp-leaves-${i + 1}`, bone: `${f.id}b`, sections, fields: { material: "hemp-palm-leaf", role: "detail", tone: 0.5 } }, anchorP: add(B.o, add(mul(B.u, uc), mul(B.v, lift + droop(uc) + patch * 2))) });
  });
  return P;
}

const TOWARD: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };

/** 🟫️ Outlines of the cut petiole stubs facing the viewer (golden-angle spiral on the stem). */
function stubLines(view: View, fr: Frame, from: number, to: number, count: number, r: number, h: number, depth: number, wide: number): { lines: P2[][]; depth: number[] } {
  const lines: P2[][] = [], dep: number[] = [];
  const step = (to - from) / count;
  for (let k = 0; k < count; k++) {
    const ang = k * 2.39996;
    const radial = add(mul(fr.v, Math.cos(ang)), mul(fr.w, Math.sin(ang)));
    if (dot(radial, TOWARD[view]) < 0.3) continue;
    const tang = cross(fr.u, radial);
    const c = add(add(fr.o, mul(fr.u, from + k * step)), mul(radial, r + depth * 0.6));
    const pts = [[-1, -1], [1, -1], [1, 0.6], [0, 1], [-1, 0.6]].map(([a, b]) => add(add(c, mul(tang, (a! * wide) / 2.2)), mul(fr.u, (b! * h) / 2)));
    const proj = pts.map((p) => project(view, p));
    lines.push([...proj, proj[0]!]);
    dep.push(dot(c, TOWARD[view]));
  }
  return { lines, depth: dep };
}

/** ➰️ The ring scars of the bare lower stem: arcs across the face that the viewer sees. */
function ringLines(view: View, fr: Frame, from: number, to: number, step: number): { lines: P2[][]; depth: number[] } {
  const lines: P2[][] = [], dep: number[] = [];
  for (const u of steps(from, to, step)) {
    const c = add(fr.o, mul(fr.u, u));
    const r = stemR(c[1]) * 0.98;
    const pts: P2[] = [];
    for (let i = 0; i <= 10; i++) {
      const a = (-0.42 + (0.84 * i) / 10) * Math.PI;
      const toward = TOWARD[view];
      const side = unit(cross(fr.u, toward));
      const p = add(c, add(mul(toward, r * Math.cos(a)), add(mul(side, r * Math.sin(a)), mul(fr.u, 0.012 * Math.cos(a)))));
      pts.push(project(view, p));
    }
    lines.push(pts);
    dep.push(dot(c, TOWARD[view]) + r);
  }
  return { lines, depth: dep };
}

const stems = [
  { id: "root-plate", parent: null, title: text("Root plate and stem base", "Wurzelteller und Stammbasis"), x: 0, y: 0, angleDeg: 90, lengthM: 0.3, joint: { type: "fixed" }, mass: { kg: 60, comX: 0.12, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: 0.053093, secondMomentM4: 0.000224, extremeFibreM: 0.13, material: "hemp-palm-stem", title: text("Solid section, d = 26 cm", "Vollquerschnitt, d = 26 cm") }, massMarker: 0.08 },
  { id: "stem-1", parent: "root-plate", title: text("Stem, lower", "Stamm unten"), x: 0.3, y: 0, angleDeg: 0, lengthM: 2.2, joint: { type: "revolute", minDeg: -2, maxDeg: 2 }, mass: { kg: 62, comX: 1.1, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: 0.031416, secondMomentM4: 0.0000785, extremeFibreM: 0.1, material: "hemp-palm-stem", title: text("Solid section, d = 20 cm", "Vollquerschnitt, d = 20 cm") }, jointRadius: 0.1, massMarker: 0.07 },
  { id: "stem-2", parent: "stem-1", title: text("Stem, middle", "Stamm Mitte"), x: 2.2, y: 0, angleDeg: 0.6, lengthM: 2.2, joint: { type: "revolute", minDeg: -3, maxDeg: 3 }, mass: { kg: 60, comX: 1.1, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: 0.028353, secondMomentM4: 0.000064, extremeFibreM: 0.095, material: "hemp-palm-stem", title: text("Solid section, d = 19 cm", "Vollquerschnitt, d = 19 cm") }, jointRadius: 0.095, massMarker: 0.07 },
  { id: "stem-3", parent: "stem-2", title: text("Stem, upper, with the persistent fibre", "Stamm oben, mit den bleibenden Fasern"), x: 2.2, y: 0, angleDeg: -0.6, lengthM: 1.6, joint: { type: "revolute", minDeg: -4, maxDeg: 4 }, mass: { kg: 42, comX: 0.8, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: 0.025447, secondMomentM4: 0.0000515, extremeFibreM: 0.09, material: "hemp-palm-stem", title: text("Solid section, d = 18 cm", "Vollquerschnitt, d = 18 cm") }, jointRadius: 0.09, massMarker: 0.07 },
  { id: "crown", parent: "stem-3", title: text("Crown with the fibrous leaf sheaths", "Krone mit den faserigen Blattscheiden"), x: 1.6, y: 0, angleDeg: 0, lengthM: 0.55, joint: { type: "revolute", minDeg: -6, maxDeg: 6 }, mass: { kg: 18, comX: 0.25, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "jones-2014" }, section: { areaM2: 0.025447, secondMomentM4: 0.0000515, extremeFibreM: 0.09, material: "hemp-palm-stem", title: text("Solid section, d = 18 cm", "Vollquerschnitt, d = 18 cm") }, jointRadius: 0.085, massMarker: 0.06 },
];

const green = leaves.filter((f) => f.state === "green");
const leafLoads = (fx: number, src: string, kind = "wind", fy = 0) => green.map((f) => ({ id: `${kind}-${f.id}b`, kind, title: kind === "wind" ? text(`Wind drag on fan leaf ${f.n} ${f.side === "l" ? "left" : "right"}`, `Winddruck auf Fächerblatt ${f.n} ${f.side === "l" ? "links" : "rechts"}`) : text(`Snow on fan leaf ${f.n} ${f.side === "l" ? "left" : "right"}`, `Schnee auf Fächerblatt ${f.n} ${f.side === "l" ? "links" : "rechts"}`), bone: `${f.id}b`, x: r4(f.Lb * 0.4), y: 0, fxN: fx, fyN: fy, source: "literature", reference: src }));
const stemLoads = (w: number, src: string) => [["stem-1", 2.2], ["stem-2", 2.2], ["stem-3", 1.6]].map(([bone, L]) => ({ id: `wind-${bone}`, kind: "wind", title: text(`Wind drag on the stem, ${bone === "stem-1" ? "lower" : bone === "stem-2" ? "middle" : "upper"} part`, `Winddruck auf den Stamm, ${bone === "stem-1" ? "unten" : bone === "stem-2" ? "Mitte" : "oben"}`), fxN: 0, fyN: 0, distributed: { bone, fromM: 0, toM: L, wxNPerM: w, wyNPerM: 0 }, source: "literature", reference: src }));
const binding = (sun: [number, number, number], tp: number, wet?: number) => ({ regions: { "trunk-fibre": { exposedFraction: 1, sunlitFraction: sun[0], ...(wet ? { wettedness: wet } : {}) }, leaves: { exposedFraction: 1, sunlitFraction: sun[1], ...(wet ? { wettedness: wet } : {}) }, "dead-leaves": { exposedFraction: 1, sunlitFraction: sun[2], ...(wet ? { wettedness: wet } : {}) } }, emitters: { "transpiration-leaves": tp } });
const WINDY = { windward: [9, 6], leeward: [-6, -11], stream: 0.12, roll: 10 } as const;
const STORM = { windward: [20, 13], leeward: [-14, -22], stream: 0.32, roll: 32 } as const;
const AREA = 0.36;

const sp: Species = {
  id: ID,
  title: text("Windmill Palm", "Chinesische Hanfpalme"),
  summary: text(
    "A mature windmill palm (Trachycarpus fortunei) of about 8 m, the hardiest stem-forming palm of temperate gardens: one slender stem of about 25 cm wrapped in the brown hairy fibre of its old leaf sheaths, with cut petiole stubs on the upper part and ring scars on the bare base, a fibrous crown boot and a rounded crown of twenty stiff fan leaves in four tiers, each a toothed petiole and a palmate blade of 36 folded segments split to about half its radius, the tips drooping, and five dead leaves hanging folded below the crown.",
    "Eine ausgewachsene Chinesische Hanfpalme (Trachycarpus fortunei) von etwa 8 m, die winterhärteste stammbildende Palme gemäßigter Gärten: ein schlanker Stamm von etwa 25 cm, eingehüllt in die braunen, haarigen Fasern seiner alten Blattscheiden, oben mit abgeschnittenen Blattstielstümpfen und an der kahlen Basis mit Ringnarben, ein faseriger Kronenschaft und eine rundliche Krone aus zwanzig steifen Fächerblättern in vier Kränzen, jedes ein fein gezähnter Blattstiel und eine handförmige Spreite aus 36 gefalteten Segmenten, die bis etwa zur Hälfte des Halbmessers eingeschnitten sind, mit überhängenden Spitzen, sowie fünf tote Blätter, die zusammengefaltet unter der Krone hängen.",
  ),
  tags: ["tree", "palm", "evergreen", "unbranched", "hardy", "fan-leaves", "wind", "transpiration"],
  created: "2026-11-22",
  references: [
    { id: "flora-of-china-trachycarpus", citation: "Pei Shengji, Chen Sanyang, Guo Lixiu and Henderson, A. (2010). Arecaceae: Trachycarpus fortunei (Hooker) H. Wendland. In: Wu Zhengyi, Raven, P. H. and Hong Deyuan (eds.), Flora of China, vol. 23, pp. 145-146. Science Press, Beijing, and Missouri Botanical Garden Press, St. Louis." },
    { id: "genera-palmarum-2008", citation: "Dransfield, J., Uhl, N. W., Asmussen, C. B., Baker, W. J., Harley, M. M. and Lewis, C. E. (2008). Genera Palmarum: The Evolution and Classification of Palms. Royal Botanic Gardens, Kew. Leaf morphology of palmate palms (hastula, induplicate segments, leaf sheath fibres)." },
    { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (green density, modulus of elasticity and rupture; closest listed species used as proxy)." },
    { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m." },
    { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Annex E: bulk weight density of fresh snow about 1.0 kN/m3." },
    { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
    { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance." },
    { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  ],
  notes: text(
    "One unbranched stem and twenty-five leaves of two bones each (petiole, blade) are authored by hand: twenty green fan leaves in four tiers on the crown, spaced by the golden angle, and five dead leaves hanging from the upper stem, as an unpruned garden palm keeps them. Flora of China gives stems to 12 m (here 6.3 m of stem, 8 m in all), petioles of 65 to 100 cm with finely toothed margins (here 74 to 80 cm) and blades of 30 to 50 segments divided to about the middle (here 36, split from half the radius, blade radius 54 to 60 cm). The segments are induplicate (V-shaped) and their tips droop; the whole crown is about 3 m across. Masses are authored estimates from the volume and green density of palm tissue.",
    "Ein unverzweigter Stamm und fünfundzwanzig Blätter aus je zwei Knochen (Blattstiel, Spreite) sind von Hand gezeichnet: zwanzig grüne Fächerblätter in vier Kränzen an der Krone, im goldenen Winkel verteilt, und fünf tote Blätter, die am oberen Stamm hängen, wie sie eine ungeschnittene Gartenpalme behält. Die Flora of China nennt Stämme bis 12 m (hier 6,3 m Stamm, insgesamt 8 m), Blattstiele von 65 bis 100 cm mit fein gezähnten Rändern (hier 74 bis 80 cm) und Spreiten aus 30 bis 50 Segmenten, die etwa bis zur Mitte geteilt sind (hier 36, ab dem halben Halbmesser eingeschnitten, Spreitenhalbmesser 54 bis 60 cm). Die Segmente sind induplikat (V-förmig) gefaltet und ihre Spitzen hängen über; die ganze Krone misst etwa 3 m. Massen sind Autorenschätzungen aus dem Volumen und der Frischdichte des Palmengewebes.",
  ),
  materials: [
    { id: "hemp-palm-fibre", title: text("Brown fibre coat of the old leaf sheaths", "Brauner Fasermantel der alten Blattscheiden"), emissivity: 0.95, solarAbsorptivity: 0.78, densityKgM3: 150, conductivityWmK: 0.06, source: "authored-estimate" },
    { id: "hemp-palm-leaf", title: text("Stiff dark green fan leaf", "Steifes dunkelgrünes Fächerblatt"), emissivity: 0.96, solarAbsorptivity: 0.55, source: "literature", reference: "jones-2014" },
    { id: "dead-palm-leaf", title: text("Dry brown dead fan leaf", "Trockenes braunes totes Fächerblatt"), emissivity: 0.94, solarAbsorptivity: 0.72, densityKgM3: 300, source: "authored-estimate" },
    { id: "hemp-palm-stem", title: text("Green palm stem (fibre bundles in ground tissue)", "Frischer Palmenstamm (Faserbündel im Grundgewebe)"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 750, conductivityWmK: 0.12, elasticModulusPa: 3000000000, strengthPa: 25000000, source: "authored-estimate", structural: true },
  ],
  stems,
  leaves,
  leafBone: isLeafBone,
  bands: (bone, scale) => (/b$/.test(bone) ? (scale <= 100 ? 7 : scale === 200 ? 3 : 2) : scale <= 100 ? 3 : 2),
  thermal: {
    leafRegion: "leaves", deadRegion: "dead-leaves", leafMaterial: "hemp-palm-leaf", deadMaterial: "dead-palm-leaf", leafJointR: 0.022, supportR: 0.5, symbolStemTop: 6.3,
    plateBands: { 50: LOD[50].bands, 100: LOD[100].bands, 200: LOD[200].bands, 500: LOD[500].bands },
    root: {
      environment: { airTempC: 26, meanRadiantTempC: 32, relativeHumidity: 0.55, windMS: 1.5, windDirectionDeg: 0, solarWM2: 750, pressurePa: 101325, diffuseFraction: 0.2 },
      regions: [
        { id: "trunk-fibre", title: text("Fibre coat of the stem and crown boot", "Fasermantel von Stamm und Kronenschaft"), kind: "bark", model: "passive", material: "hemp-palm-fibre", areaM2: 5.6, characteristicLengthM: 0.25, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 40000, source: "authored-estimate", reference: "wood-handbook-2021" },
        { id: "leaves", title: text("Green fan leaves", "Grüne Fächerblätter"), kind: "foliage", model: "leaf", material: "hemp-palm-leaf", areaM2: 12.4, areaBasis: "exchange", characteristicLengthM: 0.035, geometry: "vertical-plate", skyViewFactor: 0.75, layers: [], heatCapacityJM2K: 1500, leaf: { stomatalConductanceMaxMS: 0.004, leafAreaIndex: 1.75, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
        { id: "dead-leaves", title: text("Dead leaves hanging below the crown", "Tote Blätter unter der Krone"), kind: "foliage", model: "passive", material: "dead-palm-leaf", areaM2: 1.3, areaBasis: "exchange", characteristicLengthM: 0.05, geometry: "vertical-plate", skyViewFactor: 0.35, layers: [], heatCapacityJM2K: 600, source: "authored-estimate", reference: "jones-2014" },
      ],
      emitters: [{ id: "transpiration-leaves", kind: "transpiration", title: text("Transpiration of the fan leaves (distributed over the crown)", "Transpiration der Fächerblätter (über die Krone verteilt)"), anchor: "tp-leaves-1", anchors: Array.from({ length: 12 }, (_, i) => `tp-leaves-${i + 1}`), directionDeg: 90, diameterM: 0.8, region: "leaves", source: "authored-estimate", reference: "monteith-unsworth-2013" }],
      assumptions: [
        text("Leaf regions use one-sided leaf area (twenty blades of about 0.6 m2 and their petioles); the leaf area index is that area over the crown ground projection of about 7 m2. The characteristic length for convection is the width of one segment (3.5 cm), not the blade diameter, since the split segments shed their own boundary layers. The maximum stomatal conductance is an authored estimate for a sclerophyllous palm leaf guided by Jones (2014); photosynthesis type C3. Aperture and night closure are the engine's business, not authored here.", "Blattregionen verwenden die einseitige Blattfläche (zwanzig Spreiten von etwa 0,6 m2 und ihre Blattstiele); der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion von etwa 7 m2. Die charakteristische Länge für die Konvektion ist die Breite eines Segments (3,5 cm), nicht der Spreitendurchmesser, da die geteilten Segmente eigene Grenzschichten bilden. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für ein hartlaubiges Palmblatt, angelehnt an Jones (2014); Photosynthesetyp C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
        text("The fibre coat and the dead leaves are passive. The areal heat capacity of the stem surface is the volumetric heat capacity of green palm tissue (about 2.7 MJ per m3 and K) times an effective depth of 1.5 cm under the insulating fibre, an authored estimate guided by the Wood Handbook. No surface temperature is authored.", "Fasermantel und tote Blätter sind passiv. Die flächenbezogene Wärmekapazität der Stammoberfläche ist die volumetrische Wärmekapazität frischen Palmengewebes (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1,5 cm unter dem dämmenden Fasermantel, eine Autorenschätzung angelehnt an das Wood Handbook. Es werden keine Oberflächentemperaturen vorgegeben."),
        text("The transpiration emitter releases the latent flux of the leaves as vapour, spread over twelve anchors on the blades of twelve leaves; the direction is the vertical in the frame of the first anchor's bone, since the release is diffuse.", "Der Transpirationsemitter gibt den latenten Strom der Blätter als Dampf ab, verteilt auf zwölf Anker auf den Spreiten von zwölf Blättern; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist."),
      ],
      operating: [], passengers: [],
    },
  },
  structureAssumptions: [text("One rigid body per evaluated pose, fixed at the root ball; wind drag acts at 40 percent of each green blade and on the three stem parts as line loads, snow as a vertical load on each green blade. The stem is a solid circle of green palm tissue without the fibre coat (authored estimate of stiffness); dynamic amplification and root-soil failure are not modelled.", "Ein starrer Körper je ausgewerteter Pose, am Wurzelballen eingespannt; Winddruck wirkt bei 40 Prozent jeder grünen Spreite und auf den drei Stammteilen als Linienlast, Schnee als lotrechte Last auf jeder grünen Spreite. Der Stamm ist ein Vollkreis aus frischem Palmengewebe ohne den Fasermantel (Steifigkeit geschätzt); dynamische Überhöhung und Wurzel-Boden-Versagen werden nicht modelliert.")],
  poses: [
    { id: "calm", title: text("Calm summer day", "Windstiller Sommertag"), description: text("Light air of 1.5 m/s: the stem stands vertical and the fans are held still, drag per leaf about 0.2 N.", "Leiser Zug von 1,5 m/s: Der Stamm steht senkrecht und die Fächer stehen still, der Widerstand je Blatt beträgt etwa 0,2 N."), stem: {}, resp: STILL, thermal: binding([0.6, 0.7, 0.3], 0.8), loads: [...leafLoads(drag(0.4, AREA, 1.5), "en-1991-1-4"), ...stemLoads(drag(1.0, 0.26, 1.5), "en-1991-1-4")] },
    { id: "windy", title: text("Fresh breeze", "Frische Brise"), description: text("Beaufort 5, 8 m/s, cd 0.4 on 0.36 m2 of frontal area per fan: about 5.8 N per leaf. The stem leans by 3.7 degrees at the crown, windward fans lift and fold back, leeward fans dip, and the blades roll a little out of the wind.", "Beaufort 5, 8 m/s, cd 0,4 auf 0,36 m2 Anströmfläche je Fächer: etwa 5,8 N je Blatt. Der Stamm neigt sich an der Krone um 3,7 Grad, luvseitige Fächer heben sich und klappen zurück, leeseitige senken sich, und die Spreiten drehen sich etwas aus dem Wind."), stem: { "stem-1": { angleDeg: -0.6 }, "stem-2": { angleDeg: -1 }, "stem-3": { angleDeg: -1.6 }, crown: { angleDeg: -1.5 } }, resp: WINDY as never, thermal: binding([0.6, 0.7, 0.3], 1), loads: [...leafLoads(drag(0.4, AREA, 8), "en-1991-1-4"), ...stemLoads(drag(1.0, 0.26, 8), "en-1991-1-4")] },
    { id: "storm", title: text("Gale", "Sturm"), description: text("Beaufort 9, 22 m/s, cd 0.2 after the fans have folded and rolled edge-on (Vogel 1989): about 22 N per leaf. Windward fans are thrown up over the crown, leeward fans stream down by up to 22 degrees, the blades roll by up to 32 degrees and the stem takes about 80 percent of its joint range.", "Beaufort 9, 22 m/s, cd 0,2, nachdem sich die Fächer gefaltet und hochkant gedreht haben (Vogel 1989): etwa 22 N je Blatt. Luvseitige Fächer werden über die Krone geworfen, leeseitige strömen um bis zu 22 Grad nach unten, die Spreiten drehen sich um bis zu 32 Grad und der Stamm nutzt etwa 80 Prozent seines Gelenkbereichs."), stem: { "stem-1": { angleDeg: -1.4 }, "stem-2": { angleDeg: -2.2 }, "stem-3": { angleDeg: -3.2 }, crown: { angleDeg: -4.5 } }, resp: STORM as never, thermal: binding([0.25, 0.35, 0.15], 0.5, 0.4), loads: [...leafLoads(drag(0.2, AREA, 22), "vogel-1989"), ...stemLoads(drag(0.9, 0.26, 22), "en-1991-1-4")] },
    { id: "snow", title: text("Snow-laden in winter", "Verschneit im Winter"), description: text("About 6 cm of fresh snow (EN 1991-1-3 bulk weight 1.0 kN/m3, 0.06 kN/m2) on the 0.45 m2 plan area of each green fan: 27 N per leaf, so the petioles bend down by about 10 degrees and the blades by 14; the stem stays upright. The windmill palm stands frost to about minus 15 degrees Celsius; transpiration nearly stops and the snow keeps the leaves wet.", "Etwa 6 cm Neuschnee (Wichte nach EN 1991-1-3 1,0 kN/m3, 0,06 kN/m2) auf der Grundrissfläche von 0,45 m2 jedes grünen Fächers: 27 N je Blatt, sodass sich die Blattstiele um etwa 10 Grad und die Spreiten um 14 Grad nach unten biegen; der Stamm bleibt aufrecht. Die Hanfpalme verträgt Frost bis etwa minus 15 Grad Celsius; die Transpiration kommt fast zum Erliegen und der Schnee hält die Blätter nass."), stem: {}, resp: { ...STILL, uniform: [-10, -24] }, thermal: binding([0.3, 0.3, 0.2], 0.05, 0.9), loads: [...leafLoads(0, "en-1991-1-3", "snow", -27)] },
  ],
  clips: (ctx) => standardClips(ctx, {
    stems: ["stem-1", "stem-2", "stem-3", "crown"], windyLean: [-0.6, -1, -1.6, -1.5], swayS: 3.2, swayAmp: 0.3, flutterDeg: 4, windy: WINDY as never, storm: STORM as never, leaves,
    emitter: "transpiration-leaves", regions: [{ id: "trunk-fibre", peak: 0.6 }, { id: "leaves", peak: 0.7 }, { id: "dead-leaves", peak: 0.3 }],
    loadTargets: [...green.map((f) => `wind-${f.id}b`), "wind-stem-1", "wind-stem-2", "wind-stem-3"],
    texts: {
      sway: text("A deterministic 3.2 s sway cycle (first mode of a slender 8 m palm stem, estimated 0.31 Hz) about the fresh-breeze lean: the stem moves little, the fans nod by up to 4 degrees with a phase lag per leaf.", "Ein deterministischer Schwingungszyklus von 3,2 s (erste Eigenform eines schlanken Palmenstamms von 8 m, geschätzt 0,31 Hz) um die Neigung der frischen Brise: Der Stamm bewegt sich wenig, die Fächer nicken um bis zu 4 Grad mit einer Phasenverzögerung je Blatt."),
      gust: text("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the fans fold to the lee up to 70 % of the gale deflection and spring back.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Fächer legen sich bis zu 70 % der Sturmauslenkung nach Lee und federn zurück."),
      diurnal: text("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night; leaf and fibre temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt- und Fasertemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser."),
    },
  }),
  parts,
  anchors: (F, P) => [
    { id: "base-support", bone: "root-plate", x: 0, y: 0, kind: "support", title: text("Root ball anchorage", "Verankerung des Wurzelballens"), support: "fixed", normalDeg: 90, footprintLengthM: 1, footprintWidthM: 1, frictionCoefficient: 0.6 },
    ...P.emitters.map((e, i) => {
      const B = F[e.bone]!;
      const d = sub(e.anchorP, B.o);
      return { id: e.anchor, bone: e.bone, x: r4(dot(d, B.u)), y: r4(dot(d, B.v)), kind: "emitter", title: text(`Transpiration of the fan leaves, anchor ${i + 1}`, `Transpiration der Fächerblätter, Anker ${i + 1}`) };
    }),
  ],
  volumeDecisions: {
    50: [
      text("1:50 shows the palm in full: the root collar, the stem in three lofts under its fibre coat (the radius follows the coat, wider above), cut petiole stubs in a golden-angle spiral on the upper stem as texture, the crown boot, one spear leaf and twenty-five leaves. Every leaf bone is cut into bands along its length; in each band the blade is one flat plate with straight upper and lower faces and the rib another, and these plates are the body and the three drawings at once. Each fan is a whole blade of 36 segments whose rim droops by up to 12 cm; the folds of the segments are texture strokes from the hastula to the rim, the splits between the segment tips are not cut, because a slit of a few millimetres is no outline that body and drawing can share. The plates are 8 mm thick, the 1:50 feature; the real blade is about 1 mm. Each petiole is an arched bar of plates with its widened fibrous base inside the boot. The five dead leaves are narrow fans of twelve segments hanging on their petioles below the crown.", "1:50 zeigt die Palme vollständig: den Wurzelhals, den Stamm in drei Lofts unter seinem Fasermantel (der Radius folgt dem Mantel, oben breiter), geschnittene Blattstielstümpfe in einer Spirale im Goldenen Winkel am oberen Stamm als Textur, den Kronenschaft, ein Speerblatt und fünfundzwanzig Blätter. Jeder Blattknochen ist der Länge nach in Bänder geschnitten; in jedem Band ist die Spreite eine ebene Platte mit geraden Ober- und Unterseiten und die Rippe eine zweite, und diese Platten sind zugleich Körper und die drei Zeichnungen. Jeder Fächer ist ein ganzes Blatt aus 36 Segmenten, dessen Rand um bis zu 12 cm überhängt; die Falten der Segmente sind Texturstriche von der Hastula zum Rand, die Schlitze zwischen den Segmentspitzen sind nicht geschnitten, weil ein Schlitz von wenigen Millimetern kein Umriss ist, den Körper und Zeichnung teilen können. Die Platten sind 8 mm dick, das Merkmal von 1:50; das wirkliche Blatt ist etwa 1 mm dick. Jeder Blattstiel ist ein gebogener Stab aus Platten mit seiner verbreiterten faserigen Basis im Schaft. Die fünf toten Blätter sind schmale Fächer aus zwölf Segmenten, die an ihren Stielen unter der Krone hängen."),
      text("Thermal: trunk-fibre is the collar, the stem and the boot by reference; leaves are every green petiole, blade and the spear by reference plus twelve transpiration outlets as thin patches on twelve blades; dead-leaves the five hanging leaves. The stubs are texture and carry no region. Structure: the stem as solid members of the declared sections without the fibre, every petiole and blade as a slender member inside its part, joints at every rig joint, the root-ball support and the masses at their centres.", "Thermik: trunk-fibre sind Wurzelhals, Stamm und Schaft per Verweis; leaves sind jeder grüne Blattstiel, jede Spreite und das Speerblatt per Verweis sowie zwölf Transpirationsauslässe als dünne Flecken auf zwölf Spreiten; dead-leaves die fünf hängenden Blätter. Die Stümpfe sind Textur und tragen keine Region. Struktur: der Stamm als volle Glieder der deklarierten Querschnitte ohne Fasern, jeder Blattstiel und jede Spreite als schlankes Glied in seinem Teil, Gelenke an jedem Riggelenk, die Auflagerung des Wurzelballens und die Massen in ihren Schwerpunkten."),
    ],
    100: [
      text("1:100 keeps the habit and halves the detail: each fan is a whole blade of eighteen segments (two real segments as one) in plates 15.6 mm thick, a fold stroke for every second segment; the stubs are coarser texture, the ring scars of the bare base stay as strokes in 2D.", "1:100 behält den Habitus und halbiert das Detail: jeder Fächer ist ein ganzes Blatt aus achtzehn Segmenten (zwei wirkliche Segmente als eines) in 15,6 mm dicken Platten, ein Faltenstrich für jedes zweite Segment; die Stümpfe sind gröbere Textur, die Ringnarben der kahlen Basis bleiben in 2D als Striche."),
      text("Thermal and structure as at 1:50 with the merged solids; transpiration outlets on the same twelve blades.", "Thermik und Struktur wie bei 1:50 mit den zusammengefassten Körpern; Transpirationsauslässe auf denselben zwölf Spreiten."),
    ],
    200: [
      text("1:200 keeps each fan as a whole 32 mm blade of twelve segments in plates that follow the droop of the rim, on its petiole thickened to the feature; stem and boot are plain lofts, the spear leaf stays; no texture.", "1:200 behält jeden Fächer als ganzes Blatt von 32 mm aus zwölf Segmenten in Platten, die dem Überhang des Randes folgen, auf seinem auf das Merkmal verdickten Blattstiel; Stamm und Schaft sind schlichte Lofts, das Speerblatt bleibt; keine Textur."),
      text("Thermal references the merged solids and keeps the twelve outlets; structure keeps the stem, the petiole and blade members and the joints.", "Die Thermik verweist auf die zusammengefassten Körper und behält die zwölf Auslässe; die Struktur behält Stamm, Blattstiel- und Spreitenglieder und die Gelenke."),
    ],
    500: [
      text("1:500 keeps what reads at 75 mm: the stem as three lofts, the boot as one rounded mass and every leaf as a flat fan of 80 mm on its petiole, so the round crown with its hanging skirt of dead leaves still reads in all three views. Segments, spear leaf and texture are omitted.", "1:500 behält, was bei 75 mm lesbar ist: den Stamm als drei Lofts, den Schaft als eine gerundete Masse und jedes Blatt als flachen Fächer von 80 mm auf seinem Blattstiel, sodass die runde Krone mit ihrem hängenden Rock aus toten Blättern in allen drei Ansichten lesbar bleibt. Segmente, Speerblatt und Textur entfallen."),
      text("Thermal: the merged solids by reference and the twelve outlets; structure: stem, crown and leaf members, the stem joints, the support and the masses; leaf joints are left out at this scale.", "Thermik: die zusammengefassten Körper per Verweis und die zwölf Auslässe; Struktur: Stamm-, Kronen- und Blattglieder, die Stammgelenke, das Auflager und die Massen; Blattgelenke entfallen in diesem Maßstab."),
    ],
  },
  repDecisions: {
    50: {
      silhouette: text("At 1:50 (1 m is 20 mm) every drawing is the true orthographic view of the 1:50 model in the calm pose: the stem with its fibre outline, stubs and ring scars, the boot, the spear leaf and twenty-five leaves, each fan drawn as seven tilted plates across its blade with every segment fold as a stroke.", "Bei 1:50 (1 m sind 20 mm) ist jede Zeichnung die wahre Parallelprojektion des 1:50-Modells in der ruhigen Pose: der Stamm mit seinem Faserumriss, Stümpfen und Ringnarben, der Schaft, das Speerblatt und fünfundzwanzig Blätter, jeder Fächer als sieben geneigte Platten über seine Spreite gezeichnet, mit jeder Segmentfalte als Strich."),
      retained: text("Retained: every leaf, every segment and its split, the toothless outline of the petioles, the stubs facing the viewer and the ring scars.", "Beibehalten: jedes Blatt, jedes Segment und seine Teilung, der Umriss der Blattstiele, die dem Betrachter zugewandten Stümpfe und die Ringnarben."),
      merged: text("The fibre of the coat is merged into the stem outline; the fold of each segment is drawn as one stroke on the plate of its blade band.", "Die Fasern des Mantels sind in den Stammumriss aufgenommen; die Falte jedes Segments ist als ein Strich auf der Platte ihres Spreitenstreifens gezeichnet."),
      omitted: text("Omitted: the teeth of the petiole margin (2 mm), single fibres and the flowers, below 0.15 mm on paper.", "Weggelassen: die Zähne des Blattstielrands (2 mm), einzelne Fasern und die Blüten, unter 0,15 mm auf dem Papier."),
      thinElements: text("Segment folds, stubs and ring scars are the finest strokes (0.13 mm); the blades keep their 8 mm feature thickness edge-on.", "Segmentfalten, Stümpfe und Ringnarben sind die feinsten Striche (0,13 mm); die Spreiten behalten hochkant ihre Merkmalsdicke von 8 mm."),
      hatching: text("No hatching: the fibre and the leaves are told apart by tone and strokes.", "Keine Schraffur: Fasern und Blätter unterscheiden sich durch Ton und Striche."),
      styles: text("Light line: 0.25 mm outlines, 0.18 mm detail, 0.13 mm texture. Dark solid: tonal fills separate the brown fibre, the green fans and the dry dead leaves.", "Helle Linie: Umrisse 0,25 mm, Detail 0,18 mm, Textur 0,13 mm. Dunkle Fläche: getönte Füllungen trennen die braunen Fasern, die grünen Fächer und die trockenen toten Blätter."),
      overlays: text("Each thermal region has its own shapes: trunk-fibre the stem and boot, leaves the green plates and the spear, dead-leaves the hanging plates; strokes carry no region and follow their bone.", "Jede thermische Region hat eigene Formen: trunk-fibre Stamm und Schaft, leaves die grünen Platten und das Speerblatt, dead-leaves die hängenden Platten; Striche tragen keine Region und folgen ihrem Knochen."),
    },
    100: {
      silhouette: text("At 1:100 the drawings are the true views of the 1:100 model: stem, boot, spear leaf and twenty-five fans of eighteen segments, each drawn as six plates with every second fold as a stroke.", "Bei 1:100 sind die Zeichnungen die wahren Ansichten des 1:100-Modells: Stamm, Schaft, Speerblatt und fünfundzwanzig Fächer aus achtzehn Segmenten, jeder als sechs Platten gezeichnet, mit jeder zweiten Falte als Strich."),
      retained: text("Retained: every leaf with its split segments, the stubs and the ring scars.", "Beibehalten: jedes Blatt mit seinen geteilten Segmenten, die Stümpfe und die Ringnarben."),
      merged: text("Two real segments are drawn as one; the stubs are drawn at double spacing.", "Zwei wirkliche Segmente sind als eines gezeichnet; die Stümpfe sind im doppelten Abstand gezeichnet."),
      omitted: text("Omitted: petiole teeth and fibres (below 15 mm, 0.15 mm on paper).", "Weggelassen: Blattstielzähne und Fasern (unter 15 mm, 0,15 mm auf dem Papier)."),
      thinElements: text("Folds and stubs are 0.13 mm strokes; the blades keep their 15 mm feature thickness.", "Falten und Stümpfe sind Striche von 0,13 mm; die Spreiten behalten ihre Merkmalsdicke von 15 mm."),
      hatching: text("No hatching.", "Keine Schraffur."),
      styles: text("Light line: 0.25 mm outlines, 0.13 mm texture. Dark solid: fibre, green and dead leaves in three tones.", "Helle Linie: Umrisse 0,25 mm, Textur 0,13 mm. Dunkle Fläche: Fasern, grüne und tote Blätter in drei Tönen."),
      overlays: text("As at 1:50: trunk-fibre, leaves and dead-leaves each on their own shapes.", "Wie bei 1:50: trunk-fibre, leaves und dead-leaves je auf eigenen Formen."),
    },
    200: {
      silhouette: text("At 1:200 the drawings are the true views of the 1:200 model: stem, boot, spear leaf and twenty-five lobed fans, each drawn as five plates.", "Bei 1:200 sind die Zeichnungen die wahren Ansichten des 1:200-Modells: Stamm, Schaft, Speerblatt und fünfundzwanzig gelappte Fächer, jeder als fünf Platten gezeichnet."),
      retained: text("Retained: every leaf as a lobed fan on its petiole, the boot and the spear.", "Beibehalten: jedes Blatt als gelappter Fächer auf seinem Blattstiel, der Schaft und das Speer."),
      merged: text("The segments of each fan are merged into twelve shallow lobes of one plate.", "Die Segmente jedes Fächers sind zu zwölf flachen Lappen einer Platte zusammengefasst."),
      omitted: text("Omitted: folds, stubs and ring scars (below 30 mm).", "Weggelassen: Falten, Stümpfe und Ringnarben (unter 30 mm)."),
      thinElements: text("Petioles are thickened to the 30 mm feature; nothing else is thickened.", "Blattstiele sind auf das Merkmal von 30 mm verdickt; sonst wird nichts verdickt."),
      hatching: text("No hatching.", "Keine Schraffur."),
      styles: text("Light line: 0.2 mm outlines. Dark solid: stem, green and dead leaves in three tones.", "Helle Linie: Umrisse 0,2 mm. Dunkle Fläche: Stamm, grüne und tote Blätter in drei Tönen."),
      overlays: text("trunk-fibre, leaves and dead-leaves each on their own shapes.", "trunk-fibre, leaves und dead-leaves je auf eigenen Formen."),
    },
    500: {
      silhouette: text("At 1:500 the drawings are the true views of the 1:500 model: stem, boot and twenty-five plain fans, each drawn as four plates.", "Bei 1:500 sind die Zeichnungen die wahren Ansichten des 1:500-Modells: Stamm, Schaft und fünfundzwanzig schlichte Fächer, jeder als vier Platten gezeichnet."),
      retained: text("Retained: every leaf as a fan and the dead-leaf skirt.", "Beibehalten: jedes Blatt als Fächer und der Rock aus toten Blättern."),
      merged: text("Each fan is one plain sector of 80 mm; the petiole is thickened into it.", "Jeder Fächer ist ein schlichter Sektor von 80 mm; der Blattstiel ist darin verdickt."),
      omitted: text("Omitted: segments, folds, stubs, the spear leaf and the ring scars; they are far below 0.15 mm at this scale.", "Weggelassen: Segmente, Falten, Stümpfe, das Speerblatt und die Ringnarben; sie liegen bei diesem Maßstab weit unter 0,15 mm."),
      thinElements: text("Petioles and blades are thickened to the 75 mm feature so they stay printable.", "Blattstiele und Spreiten sind auf das Merkmal von 75 mm verdickt, damit sie druckbar bleiben."),
      hatching: text("No hatching; the pen is 0.15 mm for outlines and 0.25 mm for the ground line.", "Keine Schraffur; der Stift ist 0,15 mm für Umrisse und 0,25 mm für die Bodenlinie."),
      styles: text("Light line: closed outlines only. Dark solid: the crown reads as one dark mass over the lighter stem.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: die Krone liest sich als eine dunkle Masse über dem helleren Stamm."),
      overlays: text("Thermal and structural overlays map onto trunk-fibre (stem, boot), leaves (green fans) and dead-leaves (skirt).", "Thermische und statische Überlagerungen bilden sich ab auf trunk-fibre (Stamm, Schaft), leaves (grüne Fächer) und dead-leaves (Rock)."),
    },
  },
  pens: {
    50: { body: 0.25, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 },
    100: { body: 0.25, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 },
    200: { body: 0.2, detail: 0.13, texture: null, glazing: null, ground: 0.3 },
    500: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
  },
  symbolTitle: text("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
  defaultScale: 100,
};

if (import.meta.main) {
  const say = buildDocument(sp, { ship: process.argv.includes("--ship"), no2d: process.argv.includes("--no-2d") });
  for (const l of say) console.log(l);
}
export default sp;
