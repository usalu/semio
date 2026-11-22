/** 🍌️ Japanese fibre banana (Musa basjoo), stated by hand in world space in the calm pose: a corm with a pseudostem of rolled leaf sheaths of about 3.2 m, a crown of nine large paddle leaves (channelled petiole, strong midrib, a lamina torn by the wind along its parallel veins into strips), the furled cigar leaf in the centre, two dead leaves hanging against the pseudostem and two suckers of 1.4 m and 0.7 m beside the mother plant.
 * Measures: Flora of China vol. 24 (Musaceae, Musa basjoo); Turner, Fortescue and Thomas (2007); Taylor and Sexton (1972) on leaf tearing; see `📝️PW-10.md`.
 * Usage: `repo tools exec bun -- pw10-banana-plant.ts [--ship] [--no-2d]`
 * @see ./pw10-plant.ts */
import { D2R, FrondGeom, STILL, add, buildDocument, crownTable, drag, emptyParts, featureOf, lamina, mul, r4, standardClips, steps, sub, table, text, type Frame, type Leaf, type P2, type Parts, type Scale, type Species, type Tube, type V3, type View } from "./pw10-plant.ts";
import { project, sweep, cross, dot } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-date-palm-kit.ts";

const ID = "banana-plant";
const STEM = { material: "banana-pseudostem", role: "body", tone: 0.42 };
const LEAF = { material: "banana-leaf", role: "body", tone: 0.56 };
const DEAD = { material: "dead-banana-leaf", role: "body", tone: 0.34 };
const green = text("Paddle leaf", "Paddelblatt");
const sword = text("Sucker leaf", "Kindelblatt");

const leaves: Leaf[] = [
  ...crownTable({
    crown: "crown", prefix: "lf", az0: 20, divergence: 150,
    tiers: [
      { tier: "A", count: 3, elA: 22, elB: -26, La: 1.0, Lb: 1.45, baseU: 0.06, baseR: 0.05, massA: 0.9, massB: 0.8, label: green },
      { tier: "B", count: 3, elA: 42, elB: 4, La: 1.0, Lb: 1.4, baseU: 0.18, baseR: 0.045, massA: 0.9, massB: 0.8, label: green },
      { tier: "C", count: 2, elA: 62, elB: 34, La: 0.95, Lb: 1.3, baseU: 0.3, baseR: 0.04, massA: 0.8, massB: 0.7, label: green },
      { tier: "D", count: 1, elA: 78, elB: 66, La: 0.9, Lb: 1.1, baseU: 0.38, baseR: 0.03, massA: 0.6, massB: 0.5, label: green },
    ],
  }),
  ...crownTable({ crown: "pstem-2", prefix: "dl", az0: 100, divergence: 170, tiers: [{ tier: "A", count: 2, elA: -58, elB: -84, La: 0.55, Lb: 0.75, baseU: 1.05, baseR: 0.07, massA: 0.2, massB: 0.2, state: "dead", label: text("Dead leaf", "Totes Blatt") }] }),
  ...crownTable({ crown: "sk1-stem", prefix: "sa", az0: -30, divergence: 140, tiers: [{ tier: "C", count: 3, elA: 58, elB: 30, La: 0.42, Lb: 0.62, baseU: 1.3, baseR: 0.02, massA: 0.15, massB: 0.15, label: sword }] }),
  ...crownTable({ crown: "sk2-stem", prefix: "sb", az0: 150, divergence: 160, tiers: [{ tier: "D", count: 2, elA: 66, elB: 44, La: 0.28, Lb: 0.4, baseU: 0.62, baseR: 0.015, massA: 0.06, massB: 0.06, label: sword }] }),
];
const isLeafBone = (bone: string) => /^(lf|dl|sa|sb)-[lr]\d+[ab]$/.test(bone);

/** 📏️ Pseudostem radius of the mother plant at a world height. */
const stemR = (y: number): number => table([[0, 0.17], [0.1, 0.14], [0.3, 0.125], [1.0, 0.112], [1.85, 0.098], [2.6, 0.085], [3.2, 0.072], [3.6, 0.06]], y);

const LOD = {
  50: { row: 0.05, ti: 0.3, tears: 0.11, cols: 2, half: 0.004, veins: 0.06, stemStep: 0.25, sheaths: true, bands: [3, 5] },
  100: { row: 0.1, ti: 0.4, tears: 0.24, cols: 1, half: 0.0075, veins: 0.12, stemStep: 0.5, sheaths: true, bands: [3, 4] },
  200: { row: 0.2, ti: 1, tears: 0, cols: 1, half: 0.016, veins: 0, stemStep: 1.0, sheaths: false, bands: [3, 4] },
  500: { row: 0.35, ti: 1, tears: 0, cols: 1, half: 0.04, veins: 0, stemStep: 1.6, sheaths: false, bands: [2, 3] },
} as const;

/** 🎲️ A fixed hand-made sequence of tear spacings (factors), so every leaf tears differently but reproducibly. */
const JITTER = [1.0, 0.7, 1.3, 0.85, 1.15, 0.6, 1.4, 0.9, 1.05, 0.75];

/** 🍌️ One leaf: channelled petiole and midrib along the arch and the lamina as one flat blade per bone, which the builder cuts into the plates that are body and drawing at once; the wind tears along the parallel veins are texture strokes on the blade (a slit a few millimetres wide is no outline the body and its drawing can share). Dead leaves are narrow and brown. */
function leaf(f: Leaf, F: Record<string, Frame>, scale: Scale, P: Parts): void {
  const lod = LOD[scale];
  const feature = featureOf(scale);
  const g = new FrondGeom(f, F[`${f.id}a`]!, F[`${f.id}b`]!);
  const L = f.La + f.Lb;
  const dead = f.state === "dead";
  const small = f.id.startsWith("s");
  const petioleL = small ? 0.18 * L : dead ? 0.3 : 0.42;
  const maxW = small ? (f.id.startsWith("sa") ? 0.11 : 0.07) : dead ? 0.1 : 0.29 - (f.tier === "D" ? 0.05 : 0);
  const W = (S: number) => maxW * table([[petioleL, 0.25], [petioleL + 0.1 * (L - petioleL), 0.7], [petioleL + 0.3 * (L - petioleL), 0.97], [petioleL + 0.75 * (L - petioleL), 1], [petioleL + 0.93 * (L - petioleL), 0.75], [L, 0.12]], S);
  const rw = (S: number) => table([[-0.1, 0.06], [0, 0.045], [0.15, 0.02], [petioleL, 0.017], [L, 0.004]], S) * (small ? 0.5 : 1);
  const rd = (S: number) => table([[-0.1, 0.04], [0, 0.03], [0.15, 0.016], [petioleL, 0.014], [L, 0.003]], S) * (small ? 0.5 : 1);
  const minHalf = feature * 0.55;
  const fields = dead ? { ...DEAD } : { ...LEAF };
  const region = dead ? "dead-leaves" : "leaves";
  for (const h of ["a", "b"] as const) {
    const [s0, s1] = h === "a" ? [-0.1, f.La + 0.03] : [f.La - 0.03, L - 0.02];
    const S = steps(s0, s1, scale === 50 ? 0.1 : scale === 100 ? 0.2 : 0.4);
    const sections = S.map((s) => {
      const q = g.at(s, h);
      return { p: q.p, x: q.n, y: q.w, a: Math.max(rd(s), minHalf), b: Math.max(rw(s), minHalf) };
    });
    const id = `${f.id}${h}-${h === "a" ? "petiole" : "midrib"}`;
    P.tubes.push({ id, bone: `${f.id}${h}`, sections, fields: { ...fields, tone: dead ? 0.3 : 0.62 } });
    P.regions[id] = region;
    const core = h === "a" ? [{ ...sections[0]!, p: g.at(-f.baseR, "a").p }, ...sections.slice(1)] : sections;
    P.members.push({ id: `m-${f.id}${h}`, bone: `${f.id}${h}`, sections: core.map((s) => ({ ...s, a: s.a * 0.7, b: s.b * 0.7 })), fields: { material: "banana-pseudostem", role: "body", tone: 0.5, slender: true } });
  }
  const tears = { l: [] as number[], r: [] as number[] };
  if (lod.tears > 0 && !small) for (const side of ["l", "r"] as const) {
    let s = petioleL + 0.15 + (side === "l" ? 0.03 : 0.08);
    let k = f.k * 3 + (side === "l" ? 0 : 5);
    const spacing = lod.tears * (f.tier === "A" || dead ? 0.8 : f.tier === "D" ? 1.8 : 1);
    while (s < L - 0.12) {
      tears[side].push(Math.round(s * 1000) / 1000);
      s += spacing * JITTER[k++ % JITTER.length]!;
    }
  }
  const thick = Math.max(lod.half, feature * 0.52);
  const sheets = lamina({
    f, g, F, id: "lamina", from: petioleL, to: L - 0.01, W, ti: 1, tears: { l: [], r: [] }, gap: () => 0,
    fold: () => 0,
    rowStep: lod.row, outerCols: lod.cols, half: thick, fields, region,
  });
  for (const s of sheets) {
    P.sheets.push(s);
    P.regions[s.id] = region;
  }
  for (const h of ["a", "b"] as const) {
    const [s0, s1] = h === "a" ? [petioleL, f.La] : [f.La, L];
    const lines: { a: V3; b: V3 }[] = [];
    for (const [side, list] of [[1, tears.l], [-1, tears.r]] as [number, number[]][]) for (const s of list.filter((x) => x >= s0 && x < s1)) {
      const pt = (t: number, ds: number) => { const q = g.at(s + ds, h); return add(q.p, add(mul(q.w, side * t * W(s + ds)), mul(q.n, thick * 1.05))); };
      lines.push({ a: pt(lod.ti, 0), b: pt(0.99, 0.012) });
    }
    if (lines.length) P.strokes.push({ id: `${f.id}${h}-tears`, bone: `${f.id}${h}`, lines, role: "texture" });
  }
  if (lod.veins > 0 && !dead && !small) for (const h of ["a", "b"] as const) {
    const lines: { a: V3; b: V3 }[] = [];
    const [s0, s1] = h === "a" ? [petioleL + 0.05, f.La] : [f.La + 0.02, L - 0.12];
    for (const s of steps(s0, s1, lod.veins)) for (const side of [1, -1]) {
      const q = g.at(s, h);
      const lift = thick * 1.05;
      const pt = (t: number, ds: number) => {
        const qq = g.at(s + ds, h);
        return add(qq.p, add(mul(qq.w, side * t * W(s + ds)), mul(qq.n, lift)));
      };
      void q;
      lines.push({ a: pt(0.12, 0), b: pt(0.92, 0.06) });
    }
    if (lines.length) P.strokes.push({ id: `${f.id}${h}-veins`, bone: `${f.id}${h}`, lines, role: "texture" });
  }
}

/** 🍌️ Every part of the banana at one scale. */
function parts(scale: Scale, F: Record<string, Frame>): Parts {
  const lod = LOD[scale];
  const feature = featureOf(scale);
  const P = emptyParts();
  const chain: [string, number, number, number][] = [["root-plate", 0, 0.25, 0], ["pstem-1", 0.25, 1.85, 1], ["pstem-2", 1.85, 3.2, 1]];
  for (const [bone, y0, y1] of chain) {
    const fr = F[bone]!;
    const to = y1 - y0;
    const us = bone === "root-plate" ? [0, 0.04, 0.1, 0.18, 0.25] : steps(-0.03, to, lod.stemStep);
    const sections = sweep(us.map((u) => {
      const p = add(fr.o, mul(fr.u, u));
      return { p, a: stemR(p[1]) * 1.02, b: stemR(p[1]) * 0.98, n: 2 };
    }), fr.v);
    const id = bone === "root-plate" ? "corm-collar" : bone;
    P.tubes.push({ id, bone, sections, fields: { ...STEM, tone: bone === "root-plate" ? 0.36 : 0.42 } });
    P.regions[id] = "pseudostem";
    P.groups[id] = { bone, region: "pseudostem", role: "body", tone: bone === "root-plate" ? 0.36 : 0.42, material: "banana-pseudostem", parts: [id] };
    const core = bone === "root-plate" ? 0.1 : bone === "pstem-1" ? 0.085 : 0.065;
    P.members.push({ id: `m-${bone}`, bone, sections: sweep([0, to].map((u) => ({ p: add(fr.o, mul(fr.u, u)), a: core, b: core })), fr.v), fields: { material: "banana-pseudostem", role: "body", tone: 0.5 } });
  }
  const crown = F["crown"]!;
  const top: Tube = { id: "crown-sheaths", bone: "crown", sections: sweep([[-0.03, 0.074], [0.15, 0.068], [0.3, 0.058], [0.42, 0.045], [0.48, 0.03]].map(([u, r]) => ({ p: add(crown.o, mul(crown.u, u!)), a: Math.max(r!, feature * 0.55), b: Math.max(r! * 0.95, feature * 0.55) })), crown.v), fields: { ...STEM, tone: 0.48 } };
  P.tubes.push(top);
  P.regions["crown-sheaths"] = "pseudostem";
  P.groups["crown-sheaths"] = { bone: "crown", region: "pseudostem", role: "body", tone: 0.48, material: "banana-pseudostem", parts: ["crown-sheaths"] };
  P.members.push({ id: "m-crown", bone: "crown", sections: sweep([0, 0.45].map((u) => ({ p: add(crown.o, mul(crown.u, u)), a: 0.04, b: 0.04 })), crown.v), fields: { material: "banana-pseudostem", role: "body", tone: 0.5 } });
  const cigar = steps(0, 1, scale === 50 ? 0.125 : scale === 100 ? 0.25 : 0.5).map((s) => ({ p: add(crown.o, add(mul(crown.u, 0.4 + 1.05 * s), [-0.04 * s * s, 0, 0.02 * s])), a: Math.max(feature * 0.55, 0.032 * (1 - 0.75 * s)), b: Math.max(feature * 0.55, 0.032 * (1 - 0.75 * s)) }));
  P.tubes.push({ id: "cigar-leaf", bone: "crown", sections: sweep(cigar, [1, 0, 0]), fields: { ...LEAF, tone: 0.66 } });
  P.regions["cigar-leaf"] = "leaves";
  P.groups["cigar-leaf"] = { bone: "crown", region: "leaves", role: "body", tone: 0.66, material: "banana-leaf", parts: ["cigar-leaf"] };
  for (const [sk, L, r0] of [["sk1-stem", 1.4, 0.075], ["sk2-stem", 0.7, 0.05]] as [string, number, number][]) {
    const fr = F[sk]!;
    const sections = sweep(steps(-0.03, L, Math.min(lod.stemStep, L / 3)).map((u) => ({ p: add(fr.o, mul(fr.u, u)), a: Math.max(r0 * (1 - 0.55 * Math.max(0, u) / L), feature * 0.55), b: Math.max(r0 * 0.96 * (1 - 0.55 * Math.max(0, u) / L), feature * 0.55) })), fr.v);
    P.tubes.push({ id: sk, bone: sk, sections, fields: { ...STEM, tone: 0.46 } });
    P.regions[sk] = "pseudostem";
    P.groups[sk] = { bone: sk, region: "pseudostem", role: "body", tone: 0.46, material: "banana-pseudostem", parts: [sk] };
    P.members.push({ id: `m-${sk}`, bone: sk, sections: sweep([-0.03, L].map((u) => ({ p: add(fr.o, mul(fr.u, u)), a: Math.max(r0 * 0.55, feature * 0.5), b: Math.max(r0 * 0.55, feature * 0.5) })), fr.v), fields: { material: "banana-pseudostem", role: "body", tone: 0.5 } });
  }
  const root = F["root-plate"]!;
  for (const [sk, r] of [["sk1-stem", 0.065], ["sk2-stem", 0.05]] as [string, number][]) {
    const b = F[sk]!.o;
    const id = `${sk}-runner`;
    P.rods.push({ id, bone: "root-plate", p0: [root.o[0], 0.07, root.o[2]], p1: [b[0], Math.max(0.05, r * 0.8), b[2]], r0: Math.max(0.08, feature * 0.55), r1: Math.max(r, feature * 0.55), fields: { ...STEM, tone: 0.36 } });
    P.regions[id] = "pseudostem";
    P.groups["corm-collar"]!.parts.push(id);
  }
  for (const f of leaves) leaf(f, F, scale, P);
  if (lod.sheaths) for (const bone of ["pstem-1", "pstem-2"]) {
    const fr = F[bone]!;
    const L = bone === "pstem-1" ? 1.6 : 1.35;
    P.worldTexture.push({ id: `${bone}-sheaths`, bone, role: "texture", polys: (view) => sheathLines(view, fr, L, scale === 50 ? 5 : 3, bone === "pstem-1" ? 0 : 1.6) });
  }
  const anchorLeaves = leaves.filter((f) => f.state === "green" && f.id.startsWith("lf"));
  const picks = [...anchorLeaves, ...anchorLeaves].slice(0, 12);
  picks.forEach((f, i) => {
    const g = new FrondGeom(f, F[`${f.id}a`]!, F[`${f.id}b`]!);
    const second = i >= anchorLeaves.length;
    const h = second ? "a" : "b";
    const L = f.La + f.Lb;
    const centre = second ? f.La - 0.2 : f.La + f.Lb * 0.4;
    const span = 0.16;
    const patch = Math.max(0.0025, feature * 0.52);
    const lift = Math.max(LOD[scale].half, feature * 0.52) + patch + Math.max(0.014, feature * 0.55) * 0.8;
    const side = second ? -1 : 1;
    const W0 = 0.12;
    const sections = steps(centre - span, centre + span, span / 2).map((s) => {
      const q = g.at(s, h);
      return { p: add(q.p, add(mul(q.n, lift - W0 * 0.9 * Math.tan(15 * D2R) * 0.6), mul(q.w, side * W0))), x: q.n, y: q.w, a: patch, b: Math.max(0.04, feature * 0.6) };
    });
    void L;
    const q = g.at(centre, h);
    P.emitters.push({ id: `th-tp-leaves-${i + 1}`, anchor: `tp-leaves-${i + 1}`, bone: `${f.id}${h}`, tube: { id: `th-tp-leaves-${i + 1}`, bone: `${f.id}${h}`, sections, fields: { material: "banana-leaf", role: "detail", tone: 0.5 } }, anchorP: add(q.p, add(mul(q.n, lift + patch * 2), mul(q.w, side * W0))) });
  });
  return P;
}

const TOWARD: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };

/** 🧻️ The overlapping leaf-sheath margins of the pseudostem: long oblique curves on the face that the viewer sees. */
function sheathLines(view: View, fr: Frame, L: number, n: number, y0: number): { lines: P2[][]; depth: number[] } {
  const lines: P2[][] = [], dep: number[] = [];
  const toward = TOWARD[view];
  const side = cross(fr.u, toward);
  for (let k = 0; k < n; k++) {
    const a0 = -0.75 + (1.5 * k) / Math.max(1, n - 1);
    const pts: P2[] = [];
    for (let i = 0; i <= 8; i++) {
      const s = i / 8;
      const u = 0.05 + (L - 0.1) * s;
      const ang = (a0 + 0.55 * s) * (Math.PI / 2);
      if (Math.abs(ang) > Math.PI / 2 * 0.95) break;
      const r = stemR(fr.o[1] + u) * 0.99;
      pts.push(project(view, add(add(fr.o, mul(fr.u, u)), add(mul(toward, r * Math.cos(ang)), mul(side, r * Math.sin(ang))))));
    }
    if (pts.length > 1) {
      lines.push(pts);
      dep.push(dot(fr.o, toward) + 0.1);
    }
  }
  void y0;
  return { lines, depth: dep };
}

const stems = [
  { id: "root-plate", parent: null, title: text("Corm and roots", "Rhizom (Knolle) und Wurzeln"), x: 0, y: 0, angleDeg: 90, lengthM: 0.25, joint: { type: "fixed" }, mass: { kg: 25, comX: 0.1, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "turner-2007" }, section: { areaM2: 0.070686, secondMomentM4: 0.000398, extremeFibreM: 0.15, material: "banana-pseudostem", title: text("Solid corm section, d = 30 cm", "Voller Knollenquerschnitt, d = 30 cm") }, massMarker: 0.07 },
  { id: "pstem-1", parent: "root-plate", title: text("Pseudostem, lower", "Scheinstamm unten"), x: 0.25, y: 0, angleDeg: 0, lengthM: 1.6, joint: { type: "revolute", minDeg: -4, maxDeg: 4 }, mass: { kg: 52, comX: 0.75, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "turner-2007" }, section: { areaM2: 0.038013, secondMomentM4: 0.000115, extremeFibreM: 0.11, material: "banana-pseudostem", title: text("Rolled leaf sheaths, d = 22 cm", "Gerollte Blattscheiden, d = 22 cm") }, jointRadius: 0.085, massMarker: 0.06 },
  { id: "pstem-2", parent: "pstem-1", title: text("Pseudostem, upper", "Scheinstamm oben"), x: 1.6, y: 0, angleDeg: 0.8, lengthM: 1.35, joint: { type: "revolute", minDeg: -6, maxDeg: 6 }, mass: { kg: 25, comX: 0.6, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "turner-2007" }, section: { areaM2: 0.022698, secondMomentM4: 0.000041, extremeFibreM: 0.085, material: "banana-pseudostem", title: text("Rolled leaf sheaths, d = 17 cm", "Gerollte Blattscheiden, d = 17 cm") }, jointRadius: 0.07, massMarker: 0.05 },
  { id: "crown", parent: "pstem-2", title: text("Crown: emerging petioles and cigar leaf", "Krone: austretende Blattstiele und Zigarrenblatt"), x: 1.35, y: 0, angleDeg: -0.8, lengthM: 0.45, joint: { type: "revolute", minDeg: -8, maxDeg: 8 }, mass: { kg: 6, comX: 0.2, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "turner-2007" }, section: { areaM2: 0.011310, secondMomentM4: 0.0000102, extremeFibreM: 0.06, material: "banana-pseudostem", title: text("Rolled leaf sheaths, d = 12 cm", "Gerollte Blattscheiden, d = 12 cm") }, jointRadius: 0.05, massMarker: 0.04 },
  { id: "sk1-stem", parent: "root-plate", title: text("Sword sucker, larger", "Schwertkindel, größer"), x: 0.03, y: -0.36, z: -0.2, angleDeg: -6, abductionDeg: -4, lengthM: 1.4, joint: { type: "revolute", minDeg: -6, maxDeg: 6 }, mass: { kg: 6, comX: 0.55, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "turner-2007" }, section: { areaM2: 0.017671, secondMomentM4: 0.0000249, extremeFibreM: 0.075, material: "banana-pseudostem", title: text("Rolled leaf sheaths, d = 15 cm", "Gerollte Blattscheiden, d = 15 cm") }, jointRadius: 0.05, massMarker: 0.04 },
  { id: "sk2-stem", parent: "root-plate", title: text("Sword sucker, smaller", "Schwertkindel, kleiner"), x: 0.02, y: 0.3, z: 0.24, angleDeg: 7, abductionDeg: 5, lengthM: 0.7, joint: { type: "revolute", minDeg: -6, maxDeg: 6 }, mass: { kg: 1.5, comX: 0.28, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "turner-2007" }, section: { areaM2: 0.007854, secondMomentM4: 0.0000049, extremeFibreM: 0.05, material: "banana-pseudostem", title: text("Rolled leaf sheaths, d = 10 cm", "Gerollte Blattscheiden, d = 10 cm") }, jointRadius: 0.035, massMarker: 0.03 },
];

const mains = leaves.filter((f) => f.state === "green" && f.id.startsWith("lf"));
const leafLoads = (fx: number, src: string) => mains.map((f) => ({ id: `wind-${f.id}b`, kind: "wind", title: text(`Wind drag on paddle leaf ${f.n} ${f.side === "l" ? "left" : "right"}`, `Winddruck auf Paddelblatt ${f.n} ${f.side === "l" ? "links" : "rechts"}`), bone: `${f.id}b`, x: r4(f.Lb * 0.4), y: 0, fxN: fx, fyN: 0, source: "literature", reference: src }));
const stemLoads = (w: number, src: string) => [["pstem-1", 1.6], ["pstem-2", 1.35]].map(([bone, L]) => ({ id: `wind-${bone}`, kind: "wind", title: text(`Wind drag on the pseudostem, ${bone === "pstem-1" ? "lower" : "upper"} part`, `Winddruck auf den Scheinstamm, ${bone === "pstem-1" ? "unten" : "oben"}`), fxN: 0, fyN: 0, distributed: { bone, fromM: 0, toM: L, wxNPerM: w, wyNPerM: 0 }, source: "literature", reference: src }));
const binding = (sun: [number, number, number], tp: number, wet?: number) => ({ regions: { pseudostem: { exposedFraction: 1, sunlitFraction: sun[0], ...(wet ? { wettedness: wet } : {}) }, leaves: { exposedFraction: 1, sunlitFraction: sun[1], ...(wet ? { wettedness: wet } : {}) }, "dead-leaves": { exposedFraction: 1, sunlitFraction: sun[2], ...(wet ? { wettedness: wet } : {}) } }, emitters: { "transpiration-leaves": tp } });
const WINDY = { windward: [10, 7], leeward: [-8, -14], stream: 0.35, roll: 22 };
const STORM = { windward: [21, 14], leeward: [-15, -24], stream: 0.6, roll: 42 };
const AREA = 0.5;

const sp: Species = {
  id: ID,
  title: text("Japanese Fibre Banana", "Japanische Faserbanane"),
  summary: text(
    "A mature clump of the hardy Japanese fibre banana (Musa basjoo) of about 5 m: a green pseudostem of rolled leaf sheaths, 3.2 m tall and 25 cm thick at the base, carrying nine paddle leaves up to 2.4 m long and 58 cm wide on channelled petioles, their blades torn by the wind along the parallel veins into strips, the furled cigar leaf rising in the centre, two dead leaves hanging brown against the pseudostem and two sword suckers of 1.4 m and 0.7 m beside the mother plant.",
    "Ein ausgewachsener Horst der winterharten Japanischen Faserbanane (Musa basjoo) von etwa 5 m: ein grüner Scheinstamm aus gerollten Blattscheiden, 3,2 m hoch und an der Basis 25 cm dick, der neun Paddelblätter von bis zu 2,4 m Länge und 58 cm Breite an rinnigen Blattstielen trägt, deren Spreiten der Wind entlang der parallelen Adern in Streifen gerissen hat, in der Mitte das aufsteigende eingerollte Zigarrenblatt, zwei tote Blätter, die braun am Scheinstamm hängen, und zwei Schwertkindel von 1,4 m und 0,7 m neben der Mutterpflanze.",
  ),
  tags: ["herbaceous-perennial", "banana", "pseudostem", "large-leaves", "hardy", "exotic-garden", "wind", "transpiration"],
  created: "2026-11-22",
  references: [
    { id: "flora-of-china-musa", citation: "Wu Delin and Kress, W. J. (2000). Musaceae: Musa basjoo Siebold and Zuccarini ex Iinuma. In: Wu Zhengyi and Raven, P. H. (eds.), Flora of China, vol. 24, pp. 297-313. Science Press, Beijing, and Missouri Botanical Garden Press, St. Louis." },
    { id: "turner-2007", citation: "Turner, D. W., Fortescue, J. A. and Thomas, D. S. (2007). Environmental physiology of the bananas (Musa spp.). Brazilian Journal of Plant Physiology 19(4), 463-484." },
    { id: "taylor-sexton-1972", citation: "Taylor, S. E. and Sexton, O. J. (1972). Some implications of leaf tearing in Musaceae. Ecology 53(1), 143-149." },
    { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m." },
    { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
    { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance." },
    { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  ],
  notes: text(
    "The pseudostem is not wood but the rolled sheaths of the leaves; it is authored as three bones (corm, lower and upper pseudostem) and a crown bone from which nine leaves of two bones each (petiole with the lower lamina, upper lamina) unfold in a spiral; two suckers stand on their own bones on the corm. Flora of China gives pseudostems of 2.5 to 4 m and oblong blades up to about 3 m; the modelled blades are 1.9 to 2.0 m on petioles of 42 cm. The lamina tears along its parallel lateral veins in wind (Taylor and Sexton 1972), which is modelled as strips between tears that start 30 percent of the half width from the midrib. Masses are authored estimates from the volume of the water-rich tissue (density about 850 kg/m3).",
    "Der Scheinstamm ist kein Holz, sondern die gerollten Scheiden der Blätter; er ist als drei Knochen (Knolle, unterer und oberer Scheinstamm) und ein Kronenknochen gezeichnet, aus dem sich neun Blätter aus je zwei Knochen (Blattstiel mit unterer Spreite, obere Spreite) spiralig entfalten; zwei Kindel stehen auf eigenen Knochen auf der Knolle. Die Flora of China nennt Scheinstämme von 2,5 bis 4 m und längliche Spreiten bis etwa 3 m; die gezeichneten Spreiten messen 1,9 bis 2,0 m an Blattstielen von 42 cm. Die Spreite reißt im Wind entlang ihrer parallelen Seitenadern (Taylor und Sexton 1972), dargestellt als Streifen zwischen Rissen, die bei 30 Prozent der Halbbreite von der Mittelrippe beginnen. Massen sind Autorenschätzungen aus dem Volumen des wasserreichen Gewebes (Dichte etwa 850 kg/m3).",
  ),
  materials: [
    { id: "banana-pseudostem", title: text("Green pseudostem of rolled leaf sheaths", "Grüner Scheinstamm aus gerollten Blattscheiden"), emissivity: 0.95, solarAbsorptivity: 0.6, densityKgM3: 850, conductivityWmK: 0.45, elasticModulusPa: 150000000, strengthPa: 2000000, source: "authored-estimate", structural: true },
    { id: "banana-leaf", title: text("Thin bright green banana lamina", "Dünne hellgrüne Bananenspreite"), emissivity: 0.96, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: "dead-banana-leaf", title: text("Dry brown dead banana leaf", "Trockenes braunes totes Bananenblatt"), emissivity: 0.94, solarAbsorptivity: 0.7, densityKgM3: 200, source: "authored-estimate" },
  ],
  stems,
  leaves,
  leafBone: isLeafBone,
  bands: (bone, scale) => (/b$/.test(bone) ? LOD[scale].bands[1] : LOD[scale].bands[0]),
  thermal: {
    leafRegion: "leaves", deadRegion: "dead-leaves", leafMaterial: "banana-leaf", deadMaterial: "dead-banana-leaf", leafJointR: 0.025, supportR: 0.45, symbolStemTop: 3.2,
    root: {
      environment: { airTempC: 28, meanRadiantTempC: 34, relativeHumidity: 0.6, windMS: 1.5, windDirectionDeg: 0, solarWM2: 800, pressurePa: 101325, diffuseFraction: 0.2 },
      regions: [
        { id: "pseudostem", title: text("Pseudostem, crown sheaths and suckers", "Scheinstamm, Kronenscheiden und Kindel"), kind: "stem", model: "passive", material: "banana-pseudostem", areaM2: 3.16, characteristicLengthM: 0.2, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 80000, source: "authored-estimate", reference: "turner-2007" },
        { id: "leaves", title: text("Green paddle leaves", "Grüne Paddelblätter"), kind: "foliage", model: "leaf", material: "banana-leaf", areaM2: 7.6, areaBasis: "exchange", characteristicLengthM: 0.08, geometry: "vertical-plate", skyViewFactor: 0.7, layers: [], heatCapacityJM2K: 900, leaf: { stomatalConductanceMaxMS: 0.008, leafAreaIndex: 0.75, photosynthesis: "c3" }, source: "authored-estimate", reference: "turner-2007" },
        { id: "dead-leaves", title: text("Dead leaves hanging on the pseudostem", "Tote Blätter am Scheinstamm"), kind: "foliage", model: "passive", material: "dead-banana-leaf", areaM2: 0.35, areaBasis: "exchange", characteristicLengthM: 0.06, geometry: "vertical-plate", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 400, source: "authored-estimate", reference: "jones-2014" },
      ],
      emitters: [{ id: "transpiration-leaves", kind: "transpiration", title: text("Transpiration of the paddle leaves (distributed over the crown)", "Transpiration der Paddelblätter (über die Krone verteilt)"), anchor: "tp-leaves-1", anchors: Array.from({ length: 12 }, (_, i) => `tp-leaves-${i + 1}`), directionDeg: 90, diameterM: 1.0, region: "leaves", source: "authored-estimate", reference: "monteith-unsworth-2013" }],
      assumptions: [
        text("Leaf regions use one-sided leaf area (nine blades of 0.6 to 0.9 m2 and the sucker leaves); the leaf area index is that area over the crown ground projection of about 10 m2. The characteristic length for convection is the mean width of the torn strips (about 8 cm), not the blade width: Taylor and Sexton (1972) showed that torn banana leaves run cooler for this reason. The maximum stomatal conductance is an authored estimate for a well-watered banana leaf guided by Turner et al. (2007); photosynthesis type C3. Aperture and night closure are the engine's business, not authored here.", "Blattregionen verwenden die einseitige Blattfläche (neun Spreiten von 0,6 bis 0,9 m2 und die Kindelblätter); der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion von etwa 10 m2. Die charakteristische Länge für die Konvektion ist die mittlere Breite der gerissenen Streifen (etwa 8 cm), nicht die Spreitenbreite: Taylor und Sexton (1972) zeigten, dass gerissene Bananenblätter deshalb kühler bleiben. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für ein gut versorgtes Bananenblatt, angelehnt an Turner et al. (2007); Photosynthesetyp C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
        text("The pseudostem and the dead leaves are passive. The pseudostem is about 90 percent water; its areal heat capacity is the volumetric heat capacity of water-rich tissue (about 4 MJ per m3 and K) times an effective depth of 2 cm, an authored estimate. No surface temperature is authored.", "Scheinstamm und tote Blätter sind passiv. Der Scheinstamm besteht zu etwa 90 Prozent aus Wasser; seine flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität wasserreichen Gewebes (etwa 4 MJ je m3 und K) mal einer wirksamen Tiefe von 2 cm, eine Autorenschätzung. Es werden keine Oberflächentemperaturen vorgegeben."),
        text("The transpiration emitter releases the latent flux of the leaves as vapour, spread over twelve anchors on the nine main leaves; the direction is the vertical in the frame of the first anchor's bone, since the release is diffuse.", "Der Transpirationsemitter gibt den latenten Strom der Blätter als Dampf ab, verteilt auf zwölf Anker auf den neun Hauptblättern; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist."),
      ],
      operating: [], passengers: [],
    },
  },
  structureAssumptions: [text("One rigid body per evaluated pose, fixed at the corm; wind drag acts at 40 percent of each upper lamina of the nine main leaves and on the pseudostem as line loads. The pseudostem is a solid circle of turgid leaf sheaths with a low modulus (authored estimate, a few hundred MPa at most); it stands by turgor, so the strength is low and a gale can snap it. Dynamic amplification and root-soil failure are not modelled.", "Ein starrer Körper je ausgewerteter Pose, an der Knolle eingespannt; Winddruck wirkt bei 40 Prozent jeder oberen Spreite der neun Hauptblätter und auf dem Scheinstamm als Linienlast. Der Scheinstamm ist ein Vollkreis aus prallen Blattscheiden mit niedrigem Modul (Autorenschätzung, höchstens einige hundert MPa); er steht durch Turgor, daher ist die Festigkeit gering und ein Sturm kann ihn knicken. Dynamische Überhöhung und Wurzel-Boden-Versagen werden nicht modelliert.")],
  poses: [
    { id: "calm", title: text("Calm summer day", "Windstiller Sommertag"), description: text("Light air of 1.5 m/s: the pseudostem stands upright and the leaves arch out, drag per leaf about 0.4 N.", "Leiser Zug von 1,5 m/s: Der Scheinstamm steht aufrecht und die Blätter wölben sich nach außen, der Widerstand je Blatt beträgt etwa 0,4 N."), stem: {}, resp: STILL, thermal: binding([0.6, 0.75, 0.3], 0.8), loads: [...leafLoads(drag(0.5, AREA, 1.5), "en-1991-1-4"), ...stemLoads(drag(1.0, 0.2, 1.5), "en-1991-1-4")] },
    { id: "windy", title: text("Fresh breeze", "Frische Brise"), description: text("Beaufort 5, 8 m/s, cd 0.5 on 0.5 m2 of frontal area per leaf: 10 N per leaf. The soft pseudostem leans by 4 degrees, the leaves stream to the lee and roll their blades out of the wind; the old tears flap open.", "Beaufort 5, 8 m/s, cd 0,5 auf 0,5 m2 Anströmfläche je Blatt: 10 N je Blatt. Der weiche Scheinstamm neigt sich um 4 Grad, die Blätter strömen nach Lee und drehen ihre Spreiten aus dem Wind; die alten Risse flattern auf."), stem: { "pstem-1": { angleDeg: -1.2 }, "pstem-2": { angleDeg: -1.8 }, crown: { angleDeg: -1.5 } }, resp: WINDY as never, thermal: binding([0.6, 0.75, 0.3], 1), loads: [...leafLoads(drag(0.5, AREA, 8), "en-1991-1-4"), ...stemLoads(drag(1.0, 0.2, 8), "en-1991-1-4")] },
    { id: "storm", title: text("Gale", "Sturm"), description: text("Beaufort 9, 22 m/s, cd 0.25 once the torn leaves have streamed out like flags (Vogel 1989): about 38 N per leaf. The leaves stream to the lee by up to 24 degrees and roll by up to 42 degrees, the pseudostem leans by about 9 degrees, close to the limit at which it would snap.", "Beaufort 9, 22 m/s, cd 0,25, sobald die gerissenen Blätter wie Fahnen ausgeströmt sind (Vogel 1989): etwa 38 N je Blatt. Die Blätter strömen um bis zu 24 Grad nach Lee und drehen sich um bis zu 42 Grad, der Scheinstamm neigt sich um etwa 9 Grad, nahe der Grenze, an der er knicken würde."), stem: { "pstem-1": { angleDeg: -2.8 }, "pstem-2": { angleDeg: -4.5 }, crown: { angleDeg: -5 } }, resp: STORM as never, thermal: binding([0.25, 0.35, 0.15], 0.5, 0.4), loads: [...leafLoads(drag(0.25, AREA, 22), "vogel-1989"), ...stemLoads(drag(0.9, 0.2, 22), "en-1991-1-4")] },
    { id: "midday-wilt", title: text("Hot afternoon, leaves folded", "Heißer Nachmittag, Blätter gefaltet"), description: text("Air above 32 degrees Celsius and strong sun: the turgor of the large blades drops and the leaves sag by about 8 degrees at the petiole and 16 degrees at the tip, the halves fold down along the pulvinar bands of the midrib to cut the absorbed radiation (Turner et al. 2007), and the stomata close partly.", "Luft über 32 Grad Celsius und starke Sonne: Der Turgor der großen Spreiten sinkt, die Blätter hängen am Blattstiel um etwa 8 Grad und an der Spitze um 16 Grad durch, die Hälften klappen entlang der Gelenkstreifen der Mittelrippe nach unten, um die aufgenommene Strahlung zu verringern (Turner et al. 2007), und die Stomata schließen sich teilweise."), stem: {}, resp: { ...STILL, uniform: [-8, -24] }, thermal: binding([0.8, 0.85, 0.5], 0.4), loads: [...leafLoads(drag(0.5, AREA, 1.5), "en-1991-1-4"), ...stemLoads(drag(1.0, 0.2, 1.5), "en-1991-1-4")] },
  ],
  clips: (ctx) => standardClips(ctx, {
    stems: ["pstem-1", "pstem-2", "crown"], windyLean: [-1.2, -1.8, -1.5], swayS: 2.4, swayAmp: 0.35, flutterDeg: 8, windy: WINDY as never, storm: STORM as never, leaves,
    emitter: "transpiration-leaves", regions: [{ id: "pseudostem", peak: 0.6 }, { id: "leaves", peak: 0.75 }, { id: "dead-leaves", peak: 0.3 }],
    loadTargets: [...mains.map((f) => `wind-${f.id}b`), "wind-pstem-1", "wind-pstem-2"],
    texts: {
      sway: text("A deterministic 2.4 s sway cycle about the fresh-breeze lean: the soft pseudostem swings a little (first mode estimated 0.4 Hz), the large leaves flap by up to 8 degrees with a phase lag per leaf.", "Ein deterministischer Schwingungszyklus von 2,4 s um die Neigung der frischen Brise: Der weiche Scheinstamm schwingt etwas (erste Eigenform geschätzt 0,4 Hz), die großen Blätter schlagen um bis zu 8 Grad mit einer Phasenverzögerung je Blatt."),
      gust: text("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the leaves stream to the lee up to 70 % of the gale deflection and swing back.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Blätter strömen bis zu 70 % der Sturmauslenkung nach Lee und schwingen zurück."),
      diurnal: text("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night; leaf and stem temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt- und Stammtemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser."),
    },
  }),
  parts,
  anchors: (F, P) => [
    { id: "base-support", bone: "root-plate", x: 0, y: 0, kind: "support", title: text("Corm and root anchorage", "Verankerung von Knolle und Wurzeln"), support: "fixed", normalDeg: 90, footprintLengthM: 0.9, footprintWidthM: 0.9, frictionCoefficient: 0.6 },
    ...P.emitters.map((e, i) => {
      const B = F[e.bone]!;
      const d = sub(e.anchorP, B.o);
      return { id: e.anchor, bone: e.bone, x: r4(dot(d, B.u)), y: r4(dot(d, B.v)), kind: "emitter", title: text(`Transpiration of the paddle leaves, anchor ${i + 1}`, `Transpiration der Paddelblätter, Anker ${i + 1}`) };
    }),
  ],
  volumeDecisions: {
    50: [
      text("1:50 shows the clump in full: the corm, the pseudostem of rolled sheaths in two lofts tapering from 25 to 12 cm, the crown sheaths and the furled cigar leaf, nine main leaves, two dead leaves and two sword suckers with three and two leaves. Every leaf bone is cut into bands along its length; in each band the blade is one flat plate with straight upper and lower faces and the rib another, and these plates are the body and the three drawings at once. The blade is whole and flat across (no fold along the midrib); the wind tears are texture strokes on it, because a slit of a few millimetres is no outline that body and drawing can share. The plates are 8 mm thick, the 1:50 feature; the real lamina is under 1 mm.", "1:50 zeigt den Horst vollständig: die Knolle, den Scheinstamm aus gerollten Scheiden in zwei Lofts, die sich von 25 auf 12 cm verjüngen, die Kronenscheiden und das eingerollte Zigarrenblatt, neun Hauptblätter, zwei tote Blätter und zwei Schwertkindel mit drei und zwei Blättern. Jeder Blattknochen ist der Länge nach in Bänder geschnitten; in jedem Band ist die Spreite eine ebene Platte mit geraden Ober- und Unterseiten und die Rippe eine zweite, und diese Platten sind zugleich Körper und die drei Zeichnungen. Die Spreite ist ganz und quer eben (kein Knick an der Mittelrippe); die Windrisse sind Texturstriche darauf, weil ein Schlitz von wenigen Millimetern kein Umriss ist, den Körper und Zeichnung teilen können. Die Platten sind 8 mm dick, das Merkmal von 1:50; die wirkliche Spreite ist unter 1 mm dick."),
      text("Thermal: pseudostem is the corm collar, both pseudostem lofts, the crown sheaths and the suckers by reference; leaves are every green petiole, midrib, lamina and the cigar leaf by reference plus twelve transpiration outlets as thin patches on the lamina; dead-leaves the two hanging leaves. Structure: the pseudostem as solid members of the declared sections, every petiole and midrib as a slender member inside it, joints at every rig joint, the corm support and the masses at their centres.", "Thermik: pseudostem sind Knollenhals, beide Scheinstamm-Lofts, die Kronenscheiden und die Kindel per Verweis; leaves sind jeder grüne Blattstiel, jede Mittelrippe, jede Spreite und das Zigarrenblatt per Verweis sowie zwölf Transpirationsauslässe als dünne Flecken auf der Spreite; dead-leaves die zwei hängenden Blätter. Struktur: der Scheinstamm als volle Glieder der deklarierten Querschnitte, jeder Blattstiel und jede Mittelrippe als schlankes Glied darin, Gelenke an jedem Riggelenk, das Knollenauflager und die Massen in ihren Schwerpunkten."),
    ],
    100: [
      text("1:100 keeps every leaf as plates 15.6 mm thick in fewer bands; the tears are fewer and coarser texture strokes, petioles and midribs are thickened to the feature.", "1:100 behält jedes Blatt als 15,6 mm dicke Platten in weniger Bändern; die Risse sind weniger und gröbere Texturstriche, Blattstiele und Mittelrippen sind auf das Merkmal verdickt."),
      text("Thermal and structure as at 1:50 with the merged solids; transpiration outlets on the same leaves.", "Thermik und Struktur wie bei 1:50 mit den zusammengefassten Körpern; Transpirationsauslässe auf denselben Blättern."),
    ],
    200: [
      text("1:200 keeps every leaf as whole plates of 32 mm on their thickened midribs without tear strokes (the tears are below 30 mm apart at their base); pseudostem and suckers are plain lofts.", "1:200 behält jedes Blatt als ganze Platten von 32 mm auf ihren verdickten Mittelrippen ohne Rissstriche (die Risse liegen an ihrer Basis weniger als 30 mm auseinander); Scheinstamm und Kindel sind schlichte Lofts."),
      text("Thermal references the merged solids and keeps the twelve outlets; structure keeps every member and the joints.", "Die Thermik verweist auf die zusammengefassten Körper und behält die zwölf Auslässe; die Struktur behält jedes Glied und die Gelenke."),
    ],
    500: [
      text("1:500 keeps what reads at 75 mm: the pseudostem, the two suckers and every leaf as an 80 mm blade on its stalk, so the arching crown of large leaves still reads in all three views.", "1:500 behält, was bei 75 mm lesbar ist: den Scheinstamm, die zwei Kindel und jedes Blatt als Blatt von 80 mm an seinem Stiel, sodass die überhängende Krone großer Blätter in allen drei Ansichten lesbar bleibt."),
      text("Thermal: the merged solids by reference and the twelve outlets; structure: pseudostem, crown and leaf members, the stem joints, the support and the masses; leaf joints are left out at this scale.", "Thermik: die zusammengefassten Körper per Verweis und die zwölf Auslässe; Struktur: Scheinstamm-, Kronen- und Blattglieder, die Stammgelenke, das Auflager und die Massen; Blattgelenke entfallen in diesem Maßstab."),
    ],
  },
  repDecisions: {
    50: {
      silhouette: text("At 1:50 (1 m is 20 mm) every drawing is the true orthographic view of the 1:50 model in the calm pose: corm and pseudostem with the margins of the rolled sheaths, the cigar leaf, nine torn paddle leaves drawn as tilted plates along each blade with the lateral veins as strokes, two dead leaves and the suckers.", "Bei 1:50 (1 m sind 20 mm) ist jede Zeichnung die wahre Parallelprojektion des 1:50-Modells in der ruhigen Pose: Knolle und Scheinstamm mit den Rändern der gerollten Scheiden, das Zigarrenblatt, neun gerissene Paddelblätter als geneigte Platten entlang jeder Spreite mit den Seitenadern als Strichen, zwei tote Blätter und die Kindel."),
      retained: text("Retained: every leaf with its tears, the midribs, the sheath margins and the suckers.", "Beibehalten: jedes Blatt mit seinen Rissen, die Mittelrippen, die Scheidenränder und die Kindel."),
      merged: text("The lateral veins are drawn every 6 cm, a third of the real ones.", "Die Seitenadern sind alle 6 cm gezeichnet, ein Drittel der wirklichen."),
      omitted: text("Omitted: the fine parallel venation and the waxy bloom, below 0.15 mm on paper.", "Weggelassen: die feine Parallelnervatur und der Wachsbelag, unter 0,15 mm auf dem Papier."),
      thinElements: text("Veins and sheath margins are the finest strokes (0.13 mm); the laminae keep their 8 mm feature thickness edge-on.", "Adern und Scheidenränder sind die feinsten Striche (0,13 mm); die Spreiten behalten hochkant ihre Merkmalsdicke von 8 mm."),
      hatching: text("No hatching: the pseudostem and the leaves are told apart by tone and strokes.", "Keine Schraffur: Scheinstamm und Blätter unterscheiden sich durch Ton und Striche."),
      styles: text("Light line: 0.25 mm outlines, 0.18 mm detail, 0.13 mm texture. Dark solid: the pseudostem mid tone, the leaves lighter, the dead leaves dark.", "Helle Linie: Umrisse 0,25 mm, Detail 0,18 mm, Textur 0,13 mm. Dunkle Fläche: der Scheinstamm im Mittelton, die Blätter heller, die toten Blätter dunkel."),
      overlays: text("Each thermal region has its own shapes: pseudostem the stems, leaves the green plates and the cigar leaf, dead-leaves the hanging plates; strokes carry no region and follow their bone.", "Jede thermische Region hat eigene Formen: pseudostem die Stämme, leaves die grünen Platten und das Zigarrenblatt, dead-leaves die hängenden Platten; Striche tragen keine Region und folgen ihrem Knochen."),
    },
    100: {
      silhouette: text("At 1:100 the drawings are the true views of the 1:100 model: pseudostem, cigar leaf, nine leaves with coarser tears, the dead leaves and the suckers.", "Bei 1:100 sind die Zeichnungen die wahren Ansichten des 1:100-Modells: Scheinstamm, Zigarrenblatt, neun Blätter mit gröberen Rissen, die toten Blätter und die Kindel."),
      retained: text("Retained: every leaf and its main tears, veins every 12 cm, the sheath margins.", "Beibehalten: jedes Blatt und seine Hauptrisse, Adern alle 12 cm, die Scheidenränder."),
      merged: text("Neighbouring tears are merged into strips of 12 to 30 cm.", "Benachbarte Risse sind zu Streifen von 12 bis 30 cm zusammengefasst."),
      omitted: text("Omitted: tears closer than 15 mm and the fine venation.", "Weggelassen: Risse unter 15 mm Abstand und die feine Nervatur."),
      thinElements: text("Veins are 0.13 mm strokes; laminae keep their 15 mm feature thickness.", "Adern sind Striche von 0,13 mm; die Spreiten behalten ihre Merkmalsdicke von 15 mm."),
      hatching: text("No hatching.", "Keine Schraffur."),
      styles: text("Light line: 0.25 mm outlines, 0.13 mm texture. Dark solid: three tones as at 1:50.", "Helle Linie: Umrisse 0,25 mm, Textur 0,13 mm. Dunkle Fläche: drei Töne wie bei 1:50."),
      overlays: text("As at 1:50: pseudostem, leaves and dead-leaves each on their own shapes.", "Wie bei 1:50: pseudostem, leaves und dead-leaves je auf eigenen Formen."),
    },
    200: {
      silhouette: text("At 1:200 the drawings are the true views of the 1:200 model: pseudostem, cigar leaf, the suckers and every leaf as a whole folded blade.", "Bei 1:200 sind die Zeichnungen die wahren Ansichten des 1:200-Modells: Scheinstamm, Zigarrenblatt, die Kindel und jedes Blatt als ganzes gefaltetes Blatt."),
      retained: text("Retained: every leaf with its arch, the suckers and the dead leaves.", "Beibehalten: jedes Blatt mit seinem Bogen, die Kindel und die toten Blätter."),
      merged: text("The torn strips of each lamina are merged into a whole blade.", "Die gerissenen Streifen jeder Spreite sind zu einem ganzen Blatt zusammengefasst."),
      omitted: text("Omitted: tears, veins and sheath margins (below 30 mm).", "Weggelassen: Risse, Adern und Scheidenränder (unter 30 mm)."),
      thinElements: text("Petioles and midribs are thickened to the 30 mm feature.", "Blattstiele und Mittelrippen sind auf das Merkmal von 30 mm verdickt."),
      hatching: text("No hatching.", "Keine Schraffur."),
      styles: text("Light line: 0.2 mm outlines. Dark solid: three tones.", "Helle Linie: Umrisse 0,2 mm. Dunkle Fläche: drei Töne."),
      overlays: text("pseudostem, leaves and dead-leaves each on their own shapes.", "pseudostem, leaves und dead-leaves je auf eigenen Formen."),
    },
    500: {
      silhouette: text("At 1:500 the drawings are the true views of the 1:500 model: pseudostem, suckers and every leaf as a plain blade.", "Bei 1:500 sind die Zeichnungen die wahren Ansichten des 1:500-Modells: Scheinstamm, Kindel und jedes Blatt als schlichtes Blatt."),
      retained: text("Retained: every leaf and both suckers.", "Beibehalten: jedes Blatt und beide Kindel."),
      merged: text("Each leaf is one plain blade of 80 mm; petiole and midrib are thickened into it.", "Jedes Blatt ist ein schlichtes Blatt von 80 mm; Blattstiel und Mittelrippe sind darin verdickt."),
      omitted: text("Omitted: tears, veins, the cigar leaf and the sheath margins; they are far below 0.15 mm at this scale.", "Weggelassen: Risse, Adern, das Zigarrenblatt und die Scheidenränder; sie liegen bei diesem Maßstab weit unter 0,15 mm."),
      thinElements: text("Petioles and blades are thickened to the 75 mm feature so they stay printable.", "Blattstiele und Spreiten sind auf das Merkmal von 75 mm verdickt, damit sie druckbar bleiben."),
      hatching: text("No hatching; the pen is 0.15 mm for outlines and 0.25 mm for the ground line.", "Keine Schraffur; der Stift ist 0,15 mm für Umrisse und 0,25 mm für die Bodenlinie."),
      styles: text("Light line: closed outlines only. Dark solid: the leaves read as one light crown over the darker pseudostem.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: die Blätter lesen sich als eine helle Krone über dem dunkleren Scheinstamm."),
      overlays: text("Thermal and structural overlays map onto pseudostem, leaves and dead-leaves.", "Thermische und statische Überlagerungen bilden sich ab auf pseudostem, leaves und dead-leaves."),
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
