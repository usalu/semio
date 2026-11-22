/** 🌿️ Soft tree fern (Dicksonia antarctica), stated by hand in world space in the calm pose: a trunk of about 4 m that is a slender rhizome wrapped in a thick mantle of fibrous adventitious roots (flaring at the base), a crown of eighteen arching bipinnate fronds in four tiers (short hairy stipe, rachis, pinnae as lobes of the lamina, pinnules as texture), four crosiers unrolling in the centre and a skirt of five dead fronds hanging against the trunk.
 * Measures: Large and Braggins (2004) Tree Ferns; Flora of Australia vol. 48 (Dicksoniaceae); Hunt, Davidson, Unwin and Jordan (2002) on growth; see `📝️PW-10.md`.
 * Usage: `repo tools exec bun -- pw10-tree-fern.ts [--ship] [--no-2d]`
 * @see ./pw10-plant.ts */
import { D2R, FrondGeom, STILL, add, buildDocument, crownTable, drag, emptyParts, featureOf, lamina, mul, r4, standardClips, steps, sub, table, text, type Frame, type Leaf, type P2, type Parts, type Scale, type Species, type V3, type View } from "./pw10-plant.ts";
import { project, sweep, cross, dot } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-date-palm-kit.ts";

const ID = "tree-fern";
const MANTLE = { material: "tree-fern-mantle", role: "body", tone: 0.28 };
const FROND = { material: "tree-fern-frond", role: "body", tone: 0.5 };
const DEAD = { material: "dead-fern-frond", role: "body", tone: 0.34 };
const label = text("Frond", "Wedel");

const leaves: Leaf[] = [
  ...crownTable({
    crown: "crown", prefix: "fr", az0: 8,
    tiers: [
      { tier: "A", count: 6, elA: 16, elB: -24, La: 1.25, Lb: 1.25, baseU: 0.08, baseR: 0.2, massA: 0.6, massB: 0.5, label },
      { tier: "B", count: 6, elA: 32, elB: 0, La: 1.25, Lb: 1.2, baseU: 0.16, baseR: 0.17, massA: 0.6, massB: 0.5, label },
      { tier: "C", count: 4, elA: 48, elB: 22, La: 1.15, Lb: 1.1, baseU: 0.23, baseR: 0.13, massA: 0.55, massB: 0.45, label },
      { tier: "D", count: 2, elA: 60, elB: 36, La: 1.0, Lb: 0.9, baseU: 0.28, baseR: 0.09, massA: 0.45, massB: 0.35, label },
    ],
  }),
  ...crownTable({ crown: "trunk-2", prefix: "df", az0: 40, divergence: 143, tiers: [{ tier: "A", count: 5, elA: -72, elB: -86, La: 0.85, Lb: 0.8, baseU: 1.62, baseR: 0.2, massA: 0.15, massB: 0.12, state: "dead", label: text("Dead frond", "Toter Wedel") }] }),
];
const isLeafBone = (bone: string) => /^(fr|df)-[lr]\d+[ab]$/.test(bone);

/** 📏️ Trunk radius over the root mantle at a world height. */
const trunkR = (y: number): number => table([[0, 0.44], [0.12, 0.38], [0.35, 0.31], [1.0, 0.27], [2.5, 0.24], [3.6, 0.23], [4.0, 0.25]], y);

const LOD = {
  50: { row: 0.045, pinna: 0.085, ti: 0.12, cols: 2, half: 0.004, strokes: true, stemStep: 0.3, roots: 0.055, bands: [3, 5], crosiers: 4 },
  100: { row: 0.09, pinna: 0.17, ti: 0.16, cols: 1, half: 0.0075, strokes: true, stemStep: 0.6, roots: 0.11, bands: [3, 4], crosiers: 3 },
  200: { row: 0.18, pinna: 0, ti: 1, cols: 1, half: 0.016, strokes: false, stemStep: 1.0, roots: 0, bands: [3, 4], crosiers: 1 },
  500: { row: 0.3, pinna: 0, ti: 1, cols: 1, half: 0.04, strokes: false, stemStep: 1.8, roots: 0, bands: [2, 3], crosiers: 0 },
} as const;

/** 🌿️ One frond: hairy stipe and rachis lofts along the arch, the lamina as sheets lobed into pinnae, the pinna midveins as strokes. Dead fronds are narrow, brown and hang folded. */
function frond(f: Leaf, F: Record<string, Frame>, scale: Scale, P: Parts): void {
  const lod = LOD[scale];
  const feature = featureOf(scale);
  const g = new FrondGeom(f, F[`${f.id}a`]!, F[`${f.id}b`]!);
  const L = f.La + f.Lb;
  const dead = f.state === "dead";
  const stipe = dead ? 0.2 : 0.3;
  const maxW = dead ? 0.12 : f.tier === "D" ? 0.26 : 0.34;
  const W = (S: number) => maxW * table([[stipe, 0.3], [stipe + 0.15 * (L - stipe), 0.75], [stipe + 0.45 * (L - stipe), 1], [stipe + 0.75 * (L - stipe), 0.72], [L, 0.06]], S);
  const rw = (S: number) => table([[-0.1, 0.045], [0, 0.032], [stipe, 0.016], [L, 0.003]], S);
  const rd = (S: number) => table([[-0.1, 0.035], [0, 0.026], [stipe, 0.013], [L, 0.003]], S);
  const minHalf = feature * 0.55;
  const fields = dead ? { ...DEAD } : { ...FROND };
  const region = dead ? "dead-fronds" : "fronds";
  for (const h of ["a", "b"] as const) {
    const [s0, s1] = h === "a" ? [-0.1, f.La + 0.03] : [f.La - 0.03, L - 0.02];
    const S = steps(s0, s1, scale === 50 ? 0.1 : scale === 100 ? 0.2 : 0.4);
    const sections = S.map((s) => {
      const q = g.at(s, h);
      return { p: q.p, x: q.n, y: q.w, a: Math.max(rd(s), minHalf), b: Math.max(rw(s), minHalf) };
    });
    const id = `${f.id}${h}-${h === "a" ? "stipe" : "rachis"}`;
    P.tubes.push({ id, bone: `${f.id}${h}`, sections, fields: { ...fields, tone: dead ? 0.3 : 0.36 } });
    P.regions[id] = region;
    const core = h === "a" ? [{ ...sections[0]!, p: g.at(-f.baseR, "a").p }, ...sections.slice(1)] : sections;
    P.members.push({ id: `m-${f.id}${h}`, bone: `${f.id}${h}`, sections: core.map((s) => ({ ...s, a: s.a * 0.7, b: s.b * 0.7 })), fields: { material: "tree-fern-stem", role: "body", tone: 0.5, slender: true } });
  }
  const pinnae: number[] = lod.pinna > 0 ? steps(stipe + 0.06, L - 0.1, dead ? lod.pinna * 1.5 : lod.pinna).slice(1, -1) : [];
  const offset = lod.pinna * 0.35;
  const tears = { l: pinnae, r: pinnae.map((s) => s + offset).filter((s) => s < L - 0.12) };
  const thick = Math.max(lod.half, feature * 0.52);
  const fold = (): number => 0;
  const sheets = lamina({ f, g, F, id: "lamina", from: stipe, to: L - 0.01, W, ti: 1, tears: { l: [], r: [] }, gap: () => 0, fold, rowStep: lod.row, outerCols: lod.cols, half: thick, fields, region });
  for (const s of sheets) {
    P.sheets.push(s);
    P.regions[s.id] = region;
  }
  if (lod.strokes && !dead) for (const h of ["a", "b"] as const) {
    const lines: { a: V3; b: V3 }[] = [];
    const [lo, hi] = h === "a" ? [stipe, f.La] : [f.La, L - 0.1];
    for (const [side, list] of [[1, tears.l], [-1, tears.r]] as [number, number[]][]) {
      const cuts = [stipe, ...list, L - 0.02];
      for (let i = 0; i + 1 < cuts.length; i++) {
        const sc = (cuts[i]! + cuts[i + 1]!) / 2;
        if (sc < lo || sc >= hi) continue;
        const pt = (t: number, ds: number) => {
          const q = g.at(sc + ds, h);
          return add(q.p, add(mul(q.w, side * t * W(sc + ds)), mul(q.n, thick * 1.05)));
        };
        lines.push({ a: pt(0.06, 0), b: pt(0.85, lod.pinna * 0.25) });
      }
    }
    if (lines.length) P.strokes.push({ id: `${f.id}${h}-pinnae`, bone: `${f.id}${h}`, lines, role: "texture" });
  }
}

/** 🌿️ Every part of the tree fern at one scale. */
function parts(scale: Scale, F: Record<string, Frame>): Parts {
  const lod = LOD[scale];
  const feature = featureOf(scale);
  const P = emptyParts();
  const chain: [string, number, number][] = [["root-plate", 0, 0.4], ["trunk-1", 0.4, 2.2], ["trunk-2", 2.2, 4.0]];
  for (const [bone, y0, y1] of chain) {
    const fr = F[bone]!;
    const to = y1 - y0;
    const us = bone === "root-plate" ? [0, 0.05, 0.12, 0.22, 0.32, 0.4] : steps(-0.03, to, lod.stemStep);
    const sections = sweep(us.map((u) => {
      const p = add(fr.o, mul(fr.u, u));
      return { p, a: trunkR(p[1]) * 1.03, b: trunkR(p[1]) * 0.97, n: 2.1 };
    }), fr.v);
    const id = bone === "root-plate" ? "root-flare" : bone;
    P.tubes.push({ id, bone, sections, fields: { ...MANTLE, tone: bone === "root-plate" ? 0.24 : 0.28 } });
    P.regions[id] = "trunk-mantle";
    P.groups[id] = { bone, region: "trunk-mantle", role: "body", tone: bone === "root-plate" ? 0.24 : 0.28, material: "tree-fern-mantle", parts: [id] };
    const core = bone === "root-plate" ? 0.2 : 0.15;
    P.members.push({ id: `m-${bone}`, bone, sections: sweep([0, to].map((u) => ({ p: add(fr.o, mul(fr.u, u)), a: core, b: core })), fr.v), fields: { material: "tree-fern-stem", role: "body", tone: 0.5 } });
  }
  const crown = F["crown"]!;
  const rows: [number, number][] = scale === 500 ? [[-0.03, 0.25], [0.2, 0.25], [0.34, 0.12]] : [[-0.03, 0.25], [0.1, 0.27], [0.2, 0.26], [0.28, 0.21], [0.34, 0.12]];
  P.tubes.push({ id: "crown-cup", bone: "crown", sections: sweep(rows.map(([u, r]) => ({ p: add(crown.o, mul(crown.u, u)), a: r, b: r * 0.97, n: 2.2 })), crown.v), fields: { ...MANTLE, tone: 0.22 } });
  P.regions["crown-cup"] = "trunk-mantle";
  P.groups["crown-cup"] = { bone: "crown", region: "trunk-mantle", role: "body", tone: 0.22, material: "tree-fern-mantle", parts: ["crown-cup"] };
  P.members.push({ id: "m-crown", bone: "crown", sections: sweep([0, 0.3].map((u) => ({ p: add(crown.o, mul(crown.u, u)), a: 0.14, b: 0.14 })), crown.v), fields: { material: "tree-fern-stem", role: "body", tone: 0.5 } });
  const crosierIds: string[] = [];
  for (let i = 0; i < lod.crosiers; i++) {
    const az = (i * 360) / Math.max(1, lod.crosiers) + 30;
    const rad: V3 = [Math.cos(az * D2R), 0, -Math.sin(az * D2R)];
    const h = 0.22 + 0.06 * (i % 2);
    const p0 = add(crown.o, add(mul(crown.u, 0.25), mul(rad, 0.05)));
    const p1 = add(p0, add([0, h, 0], mul(rad, 0.04)));
    const r = Math.max(0.018, feature * 0.55);
    P.rods.push({ id: `crosier-${i + 1}-stalk`, bone: "crown", p0, p1, r0: r * 1.2, r1: r, fields: { ...FROND, tone: 0.4 } });
    P.blobs.push({ id: `crosier-${i + 1}-coil`, bone: "crown", c: add(p1, add([0, 0.02, 0], mul(rad, 0.035))), axes: [rad, [0, 1, 0], cross(rad, [0, 1, 0]) as V3], radii: [Math.max(0.055, feature * 0.55), Math.max(0.06, feature * 0.55), Math.max(0.035, feature * 0.55)], fields: { ...FROND, tone: 0.4 } });
    for (const id of [`crosier-${i + 1}-stalk`, `crosier-${i + 1}-coil`]) {
      P.regions[id] = "fronds";
      crosierIds.push(id);
    }
  }
  if (crosierIds.length) P.groups["crosiers"] = { bone: "crown", region: "fronds", role: "body", tone: 0.4, material: "tree-fern-frond", parts: crosierIds };
  for (const f of leaves) frond(f, F, scale, P);
  if (lod.roots > 0) for (const [bone, y0, L] of [["trunk-1", 0.4, 1.8], ["trunk-2", 2.2, 1.8]] as [string, number, number][]) {
    const fr = F[bone]!;
    P.worldTexture.push({ id: `${bone}-roots`, bone, role: "texture", polys: (view) => rootLines(view, fr, L, lod.roots, y0) });
  }
  const anchorLeaves = leaves.filter((f) => f.state === "green").slice(0, 12);
  anchorLeaves.forEach((f, i) => {
    const g = new FrondGeom(f, F[`${f.id}a`]!, F[`${f.id}b`]!);
    const centre = f.La + f.Lb * 0.25;
    const span = 0.12;
    const patch = Math.max(0.0025, feature * 0.52);
    const lift = Math.max(LOD[scale].half, feature * 0.52) + patch + Math.max(0.012, feature * 0.55) * 0.9;
    const sections = steps(centre - span, centre + span, span / 2).map((s) => {
      const q = g.at(s, "b");
      return { p: add(q.p, mul(q.n, lift)), x: q.n, y: q.w, a: patch, b: Math.max(0.03, feature * 0.6) };
    });
    const q = g.at(centre, "b");
    P.emitters.push({ id: `th-tp-fronds-${i + 1}`, anchor: `tp-fronds-${i + 1}`, bone: `${f.id}b`, tube: { id: `th-tp-fronds-${i + 1}`, bone: `${f.id}b`, sections, fields: { material: "tree-fern-frond", role: "detail", tone: 0.5 } }, anchorP: add(q.p, mul(q.n, lift + patch * 2)) });
  });
  return P;
}

const TOWARD: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };

/** 🪢️ The fibrous adventitious roots of the mantle: slightly wandering vertical strands on the face that the viewer sees. */
function rootLines(view: View, fr: Frame, L: number, step: number, y0: number): { lines: P2[][]; depth: number[] } {
  const lines: P2[][] = [], dep: number[] = [];
  const toward = TOWARD[view];
  const side = cross(fr.u, toward);
  const n = Math.max(3, Math.round((Math.PI * 0.25) / step));
  for (let k = 0; k < n; k++) {
    const a0 = -0.8 + (1.6 * k) / (n - 1);
    for (const [u0, u1] of [[0.05, L * 0.5], [L * 0.45, L - 0.05]]) {
      const pts: P2[] = [];
      for (let i = 0; i <= 6; i++) {
        const u = u0! + ((u1! - u0!) * i) / 6;
        const ang = (a0 + 0.06 * Math.sin(i * 1.7 + k)) * (Math.PI / 2);
        const r = trunkR(y0 + u) * 0.99;
        pts.push(project(view, add(add(fr.o, mul(fr.u, u)), add(mul(toward, r * Math.cos(ang)), mul(side, r * Math.sin(ang))))));
      }
      lines.push(pts);
      dep.push(dot(fr.o, toward) + 0.2);
    }
  }
  return { lines, depth: dep };
}

const stems = [
  { id: "root-plate", parent: null, title: text("Root flare and root ball", "Wurzelfuß und Wurzelballen"), x: 0, y: 0, angleDeg: 90, lengthM: 0.4, joint: { type: "fixed" }, mass: { kg: 85, comX: 0.16, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "large-braggins-2004" }, section: { areaM2: 0.125664, secondMomentM4: 0.001257, extremeFibreM: 0.2, material: "tree-fern-stem", title: text("Root mantle as solid section, d = 40 cm", "Wurzelmantel als Vollquerschnitt, d = 40 cm") }, massMarker: 0.12 },
  { id: "trunk-1", parent: "root-plate", title: text("Trunk, lower", "Stamm unten"), x: 0.4, y: 0, angleDeg: 0, lengthM: 1.8, joint: { type: "revolute", minDeg: -2, maxDeg: 2 }, mass: { kg: 170, comX: 0.85, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "large-braggins-2004" }, section: { areaM2: 0.070686, secondMomentM4: 0.000398, extremeFibreM: 0.15, material: "tree-fern-stem", title: text("Rhizome with the inner root mantle, d = 30 cm", "Rhizom mit innerem Wurzelmantel, d = 30 cm") }, jointRadius: 0.15, massMarker: 0.1 },
  { id: "trunk-2", parent: "trunk-1", title: text("Trunk, upper", "Stamm oben"), x: 1.8, y: 0, angleDeg: 0.8, lengthM: 1.8, joint: { type: "revolute", minDeg: -3, maxDeg: 3 }, mass: { kg: 125, comX: 0.9, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "large-braggins-2004" }, section: { areaM2: 0.070686, secondMomentM4: 0.000398, extremeFibreM: 0.15, material: "tree-fern-stem", title: text("Rhizome with the inner root mantle, d = 30 cm", "Rhizom mit innerem Wurzelmantel, d = 30 cm") }, jointRadius: 0.15, massMarker: 0.1 },
  { id: "crown", parent: "trunk-2", title: text("Crown with stipe bases and crosiers", "Krone mit Stielbasen und Farnschnecken"), x: 1.8, y: 0, angleDeg: -0.8, lengthM: 0.35, joint: { type: "revolute", minDeg: -6, maxDeg: 6 }, mass: { kg: 25, comX: 0.15, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "large-braggins-2004" }, section: { areaM2: 0.070686, secondMomentM4: 0.000398, extremeFibreM: 0.15, material: "tree-fern-stem", title: text("Crown, d = 30 cm", "Krone, d = 30 cm") }, jointRadius: 0.14, massMarker: 0.08 },
];

const green = leaves.filter((f) => f.state === "green");
const leafLoads = (fx: number, src: string) => green.map((f) => ({ id: `wind-${f.id}b`, kind: "wind", title: text(`Wind drag on frond ${f.n} ${f.side === "l" ? "left" : "right"}`, `Winddruck auf Wedel ${f.n} ${f.side === "l" ? "links" : "rechts"}`), bone: `${f.id}b`, x: r4(f.Lb * 0.4), y: 0, fxN: fx, fyN: 0, source: "literature", reference: src }));
const stemLoads = (w: number, src: string) => [["trunk-1", 1.8], ["trunk-2", 1.8]].map(([bone, L]) => ({ id: `wind-${bone}`, kind: "wind", title: text(`Wind drag on the trunk, ${bone === "trunk-1" ? "lower" : "upper"} part`, `Winddruck auf den Stamm, ${bone === "trunk-1" ? "unten" : "oben"}`), fxN: 0, fyN: 0, distributed: { bone, fromM: 0, toM: L, wxNPerM: w, wyNPerM: 0 }, source: "literature", reference: src }));
const binding = (sun: [number, number, number], tp: number, wet?: number) => ({ regions: { "trunk-mantle": { exposedFraction: 1, sunlitFraction: sun[0], ...(wet ? { wettedness: wet } : {}) }, fronds: { exposedFraction: 1, sunlitFraction: sun[1], ...(wet ? { wettedness: wet } : {}) }, "dead-fronds": { exposedFraction: 1, sunlitFraction: sun[2], ...(wet ? { wettedness: wet } : {}) } }, emitters: { "transpiration-fronds": tp } });
const WINDY = { windward: [7, 5], leeward: [-6, -10], stream: 0.15, roll: 12 };
const STORM = { windward: [17, 11], leeward: [-13, -20], stream: 0.35, roll: 30 };
const AREA = 0.6;

const sp: Species = {
  id: ID,
  title: text("Soft Tree Fern", "Tasmanischer Baumfarn"),
  summary: text(
    "A mature soft tree fern (Dicksonia antarctica): a trunk of 4 m and about 50 cm that is not wood but a slender rhizome wrapped in a thick, water-holding mantle of brown fibrous roots, flaring to almost 90 cm at the base, and a crown of eighteen arching bipinnate fronds of 2 to 2.5 m in four tiers, spreading about 5 m, with crosiers unrolling in the centre and a skirt of five dead brown fronds hanging against the trunk.",
    "Ein ausgewachsener Tasmanischer Baumfarn (Dicksonia antarctica): ein Stamm von 4 m und etwa 50 cm, der kein Holz ist, sondern ein schlankes Rhizom in einem dicken, wasserspeichernden Mantel aus braunen faserigen Wurzeln, am Fuß auf fast 90 cm verbreitert, und eine Krone aus achtzehn überhängenden doppelt gefiederten Wedeln von 2 bis 2,5 m in vier Kränzen, etwa 5 m weit ausladend, mit sich entrollenden Farnschnecken in der Mitte und einem Rock aus fünf toten braunen Wedeln, die am Stamm hängen.",
  ),
  tags: ["tree-fern", "fern", "evergreen", "unbranched", "shade", "exotic-garden", "wind", "transpiration"],
  created: "2026-11-22",
  references: [
    { id: "large-braggins-2004", citation: "Large, M. F. and Braggins, J. E. (2004). Tree Ferns. Timber Press, Portland, Oregon. Dicksonia antarctica: habit, trunk of rhizome and adventitious roots, frond size and arrangement." },
    { id: "flora-of-australia-48", citation: "Bostock, P. D. (1998). Dicksoniaceae. In: Flora of Australia, vol. 48, Ferns, Gymnosperms and Allied Groups, pp. 193-200. ABRS/CSIRO Australia, Melbourne." },
    { id: "hunt-2002", citation: "Hunt, M. A., Davidson, N. J., Unwin, G. L. and Close, D. C. (2002). Ecophysiology of the soft tree fern, Dicksonia antarctica Labill. Austral Ecology 27(4), 360-368." },
    { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m." },
    { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
    { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance." },
    { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  ],
  notes: text(
    "The trunk is authored as three bones (root flare, lower and upper trunk) and a crown bone carrying eighteen fronds of two bones each (stipe with the lower lamina, upper lamina) in four tiers at the golden angle, and five dead fronds hanging from the upper trunk. Large and Braggins (2004) describe trunks to 15 m but usually 1.5 to 5 m, up to 2 m thick at the base in old plants, and fronds of 2 to 4.5 m; here the trunk is 4 m (an age of about a century at a few centimetres a year, Hunt et al. 2002) and the fronds 2 to 2.5 m. The pinnae are the lobes of the lamina, cut almost to the rachis; the pinnules are below what the model shows and are drawn as strokes. Masses are authored estimates for a wet root mantle of about 400 kg/m3.",
    "Der Stamm ist als drei Knochen (Wurzelfuß, unterer und oberer Stamm) und ein Kronenknochen gezeichnet, der achtzehn Wedel aus je zwei Knochen (Stiel mit unterer Spreite, obere Spreite) in vier Kränzen im goldenen Winkel trägt, dazu fünf tote Wedel, die vom oberen Stamm hängen. Large und Braggins (2004) beschreiben Stämme bis 15 m, meist 1,5 bis 5 m, bei alten Pflanzen am Fuß bis 2 m dick, und Wedel von 2 bis 4,5 m; hier misst der Stamm 4 m (ein Alter von etwa einem Jahrhundert bei wenigen Zentimetern im Jahr, Hunt et al. 2002) und die Wedel 2 bis 2,5 m. Die Fiedern sind die fast bis zur Rhachis eingeschnittenen Lappen der Spreite; die Fiederchen liegen unter dem, was das Modell zeigt, und sind als Striche gezeichnet. Massen sind Autorenschätzungen für einen nassen Wurzelmantel von etwa 400 kg/m3.",
  ),
  materials: [
    { id: "tree-fern-mantle", title: text("Fibrous mantle of adventitious roots", "Faseriger Mantel aus Adventivwurzeln"), emissivity: 0.95, solarAbsorptivity: 0.82, densityKgM3: 400, conductivityWmK: 0.3, source: "authored-estimate" },
    { id: "tree-fern-frond", title: text("Harsh dark green fern frond", "Derber dunkelgrüner Farnwedel"), emissivity: 0.96, solarAbsorptivity: 0.55, source: "literature", reference: "jones-2014" },
    { id: "dead-fern-frond", title: text("Dry brown dead frond", "Trockener brauner toter Wedel"), emissivity: 0.94, solarAbsorptivity: 0.72, densityKgM3: 150, source: "authored-estimate" },
    { id: "tree-fern-stem", title: text("Rhizome and inner root mantle", "Rhizom und innerer Wurzelmantel"), emissivity: 0.93, solarAbsorptivity: 0.75, densityKgM3: 400, conductivityWmK: 0.3, elasticModulusPa: 800000000, strengthPa: 8000000, source: "authored-estimate", structural: true },
  ],
  stems,
  leaves,
  leafBone: isLeafBone,
  bands: (bone, scale) => (/b$/.test(bone) ? LOD[scale].bands[1] : LOD[scale].bands[0]),
  thermal: {
    leafRegion: "fronds", deadRegion: "dead-fronds", leafMaterial: "tree-fern-frond", deadMaterial: "dead-fern-frond", leafJointR: 0.025, supportR: 0.6, symbolStemTop: 4.0,
    root: {
      environment: { airTempC: 20, meanRadiantTempC: 22, relativeHumidity: 0.75, windMS: 1.0, windDirectionDeg: 0, solarWM2: 350, pressurePa: 101325, diffuseFraction: 0.5 },
      regions: [
        { id: "trunk-mantle", title: text("Root mantle of the trunk and crown", "Wurzelmantel von Stamm und Krone"), kind: "bark", model: "passive", material: "tree-fern-mantle", areaM2: 7.5, characteristicLengthM: 0.5, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 120000, source: "authored-estimate", reference: "large-braggins-2004" },
        { id: "fronds", title: text("Green fronds and crosiers", "Grüne Wedel und Farnschnecken"), kind: "foliage", model: "leaf", material: "tree-fern-frond", areaM2: 11.5, areaBasis: "exchange", characteristicLengthM: 0.015, geometry: "vertical-plate", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 800, leaf: { stomatalConductanceMaxMS: 0.0025, leafAreaIndex: 0.6, photosynthesis: "c3" }, source: "authored-estimate", reference: "hunt-2002" },
        { id: "dead-fronds", title: text("Skirt of dead fronds", "Rock aus toten Wedeln"), kind: "foliage", model: "passive", material: "dead-fern-frond", areaM2: 1.0, areaBasis: "exchange", characteristicLengthM: 0.02, geometry: "vertical-plate", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 400, source: "authored-estimate", reference: "jones-2014" },
      ],
      emitters: [{ id: "transpiration-fronds", kind: "transpiration", title: text("Transpiration of the fronds (distributed over the crown)", "Transpiration der Wedel (über die Krone verteilt)"), anchor: "tp-fronds-1", anchors: Array.from({ length: 12 }, (_, i) => `tp-fronds-${i + 1}`), directionDeg: 90, diameterM: 1.4, region: "fronds", source: "authored-estimate", reference: "monteith-unsworth-2013" }],
      assumptions: [
        text("Leaf regions use one-sided leaf area (eighteen laminae of about 0.65 m2 between the pinnae and the crosiers); the leaf area index is that area over the crown ground projection of about 20 m2. The characteristic length for convection is the width of a pinnule (1.5 cm), the element that sheds its own boundary layer. The maximum stomatal conductance is an authored estimate guided by the low values Hunt et al. (2002) measured on this species in the field; photosynthesis type C3. Aperture and night closure are the engine's business, not authored here.", "Blattregionen verwenden die einseitige Blattfläche (achtzehn Spreiten von etwa 0,65 m2 zwischen den Fiedern und die Farnschnecken); der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion von etwa 20 m2. Die charakteristische Länge für die Konvektion ist die Breite eines Fiederchens (1,5 cm), des Elements, das eine eigene Grenzschicht bildet. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung, angelehnt an die niedrigen Werte, die Hunt et al. (2002) an dieser Art im Freiland gemessen haben; Photosynthesetyp C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
        text("The root mantle and the dead fronds are passive. The mantle holds water like a sponge (gardeners water the trunk); its areal heat capacity is the volumetric heat capacity of a wet fibre mat (about 3 MJ per m3 and K) times an effective depth of 4 cm, an authored estimate. No surface temperature is authored.", "Wurzelmantel und tote Wedel sind passiv. Der Mantel hält Wasser wie ein Schwamm (Gärtner gießen den Stamm); seine flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität einer nassen Fasermatte (etwa 3 MJ je m3 und K) mal einer wirksamen Tiefe von 4 cm, eine Autorenschätzung. Es werden keine Oberflächentemperaturen vorgegeben."),
        text("The transpiration emitter releases the latent flux of the fronds as vapour, spread over twelve anchors on the upper laminae of twelve fronds; the direction is the vertical in the frame of the first anchor's bone, since the release is diffuse.", "Der Transpirationsemitter gibt den latenten Strom der Wedel als Dampf ab, verteilt auf zwölf Anker auf den oberen Spreiten von zwölf Wedeln; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist."),
      ],
      operating: [], passengers: [],
    },
  },
  structureAssumptions: [text("One rigid body per evaluated pose, fixed at the root ball; wind drag acts at 40 percent of each upper lamina and on the trunk as line loads. The trunk carries its loads through the rhizome and the inner, densely interwoven part of the root mantle, idealised as a solid circle of 30 cm with an authored modulus; the loose outer mantle is mass, not stiffness. Dynamic amplification and root-soil failure are not modelled.", "Ein starrer Körper je ausgewerteter Pose, am Wurzelballen eingespannt; Winddruck wirkt bei 40 Prozent jeder oberen Spreite und auf dem Stamm als Linienlast. Der Stamm trägt seine Lasten über das Rhizom und den inneren, dicht verflochtenen Teil des Wurzelmantels, idealisiert als Vollkreis von 30 cm mit geschätztem Modul; der lockere äußere Mantel ist Masse, nicht Steifigkeit. Dynamische Überhöhung und Wurzel-Boden-Versagen werden nicht modelliert.")],
  poses: [
    { id: "calm", title: text("Calm humid day in light shade", "Windstiller feuchter Tag im lichten Schatten"), description: text("Light air of 1 m/s under a canopy: the trunk stands upright and the fronds arch out in a wide shuttlecock, drag per frond about 0.1 N.", "Leiser Zug von 1 m/s unter einem Kronendach: Der Stamm steht aufrecht und die Wedel wölben sich zu einem weiten Federball, der Widerstand je Wedel beträgt etwa 0,1 N."), stem: {}, resp: STILL, thermal: binding([0.3, 0.45, 0.2], 0.8), loads: [...leafLoads(drag(0.3, AREA, 1), "en-1991-1-4"), ...stemLoads(drag(1.0, 0.5, 1), "en-1991-1-4")] },
    { id: "windy", title: text("Fresh breeze", "Frische Brise"), description: text("Beaufort 5, 8 m/s, cd 0.3 on 0.6 m2 of frontal area per frond: about 7 N per frond. The thick trunk hardly leans (1.6 degrees at the crown); windward fronds lift, leeward fronds dip and the laminae roll a little out of the wind.", "Beaufort 5, 8 m/s, cd 0,3 auf 0,6 m2 Anströmfläche je Wedel: etwa 7 N je Wedel. Der dicke Stamm neigt sich kaum (1,6 Grad an der Krone); luvseitige Wedel heben sich, leeseitige senken sich und die Spreiten drehen sich etwas aus dem Wind."), stem: { "trunk-1": { angleDeg: -0.3 }, "trunk-2": { angleDeg: -0.5 }, crown: { angleDeg: -0.8 } }, resp: WINDY as never, thermal: binding([0.3, 0.45, 0.2], 1), loads: [...leafLoads(drag(0.3, AREA, 8), "en-1991-1-4"), ...stemLoads(drag(1.0, 0.5, 8), "en-1991-1-4")] },
    { id: "storm", title: text("Gale", "Sturm"), description: text("Beaufort 9, 22 m/s, cd 0.15 once the pinnae have folded together (Vogel 1989): about 27 N per frond. The fronds stream to the lee by up to 20 degrees and roll by up to 30 degrees; the trunk leans by about 4 degrees at the crown.", "Beaufort 9, 22 m/s, cd 0,15, sobald sich die Fiedern zusammengelegt haben (Vogel 1989): etwa 27 N je Wedel. Die Wedel strömen um bis zu 20 Grad nach Lee und drehen sich um bis zu 30 Grad; der Stamm neigt sich an der Krone um etwa 4 Grad."), stem: { "trunk-1": { angleDeg: -0.8 }, "trunk-2": { angleDeg: -1.2 }, crown: { angleDeg: -2.5 } }, resp: STORM as never, thermal: binding([0.1, 0.2, 0.1], 0.5, 0.5), loads: [...leafLoads(drag(0.15, AREA, 22), "vogel-1989"), ...stemLoads(drag(0.9, 0.5, 22), "en-1991-1-4")] },
    { id: "dry-spell", title: text("Dry spell, fronds sagging", "Trockenperiode, hängende Wedel"), description: text("After two dry warm weeks without watering the trunk: the fronds lose turgor and sag by about 6 degrees at the stipe and 16 degrees at the tip, the stomata close largely (Hunt et al. 2002) and transpiration falls to a third.", "Nach zwei trockenen warmen Wochen, ohne dass der Stamm gegossen wurde: Die Wedel verlieren Turgor und hängen am Stiel um etwa 6 Grad und an der Spitze um 16 Grad durch, die Stomata schließen sich weitgehend (Hunt et al. 2002) und die Transpiration sinkt auf ein Drittel."), stem: {}, resp: { ...STILL, uniform: [-6, -22] }, thermal: binding([0.4, 0.55, 0.3], 0.3), loads: [...leafLoads(drag(0.3, AREA, 1), "en-1991-1-4"), ...stemLoads(drag(1.0, 0.5, 1), "en-1991-1-4")] },
  ],
  clips: (ctx) => standardClips(ctx, {
    stems: ["trunk-1", "trunk-2", "crown"], windyLean: [-0.3, -0.5, -0.8], swayS: 2.0, swayAmp: 0.25, flutterDeg: 5, windy: WINDY as never, storm: STORM as never, leaves,
    emitter: "transpiration-fronds", regions: [{ id: "trunk-mantle", peak: 0.3 }, { id: "fronds", peak: 0.45 }, { id: "dead-fronds", peak: 0.2 }],
    loadTargets: [...green.map((f) => `wind-${f.id}b`), "wind-trunk-1", "wind-trunk-2"],
    texts: {
      sway: text("A deterministic 2 s sway cycle about the fresh-breeze lean: the thick trunk barely moves (first mode estimated 0.5 Hz), the fronds bob by up to 5 degrees with a phase lag per frond.", "Ein deterministischer Schwingungszyklus von 2 s um die Neigung der frischen Brise: Der dicke Stamm bewegt sich kaum (erste Eigenform geschätzt 0,5 Hz), die Wedel wippen um bis zu 5 Grad mit einer Phasenverzögerung je Wedel."),
      gust: text("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the fronds stream to the lee up to 70 % of the gale deflection and spring back.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Wedel strömen bis zu 70 % der Sturmauslenkung nach Lee und federn zurück."),
      diurnal: text("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather in light shade. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night; frond and mantle temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Wetter im lichten Schatten. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; Wedel- und Manteltemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser."),
    },
  }),
  parts,
  anchors: (F, P) => [
    { id: "base-support", bone: "root-plate", x: 0, y: 0, kind: "support", title: text("Root ball anchorage", "Verankerung des Wurzelballens"), support: "fixed", normalDeg: 90, footprintLengthM: 1.2, footprintWidthM: 1.2, frictionCoefficient: 0.6 },
    ...P.emitters.map((e, i) => {
      const B = F[e.bone]!;
      const d = sub(e.anchorP, B.o);
      return { id: e.anchor, bone: e.bone, x: r4(dot(d, B.u)), y: r4(dot(d, B.v)), kind: "emitter", title: text(`Transpiration of the fronds, anchor ${i + 1}`, `Transpiration der Wedel, Anker ${i + 1}`) };
    }),
  ],
  volumeDecisions: {
    50: [
      text("1:50 shows the fern in full: the root flare, the trunk in two lofts following the root mantle (88 cm at the ground, 46 cm under the crown), the crown cup of stipe bases, four crosiers (stalk and coil) and twenty-three fronds. Every leaf bone is cut into bands along its length; in each band the blade is one flat plate with straight upper and lower faces and the rib another, and these plates are the body and the three drawings at once. The blade is the whole lanceolate outline of the frond, flat across; the pinnae are texture strokes on it every 8.5 cm, because the narrow gaps between pinnae are no outline that body and drawing can share. The plates are 8 mm thick, the 1:50 feature. The five dead fronds are narrow blades that hang against the trunk.", "1:50 zeigt den Farn vollständig: den Wurzelanlauf, den Stamm in zwei Lofts entlang des Wurzelmantels (88 cm am Boden, 46 cm unter der Krone), den Kronenbecher aus Stielbasen, vier Farnschnecken (Stiel und Spirale) und dreiundzwanzig Wedel. Jeder Blattknochen ist der Länge nach in Bänder geschnitten; in jedem Band ist die Spreite eine ebene Platte mit geraden Ober- und Unterseiten und die Rippe eine zweite, und diese Platten sind zugleich Körper und die drei Zeichnungen. Die Spreite ist der ganze lanzettliche Umriss des Wedels, quer eben; die Fiedern sind Texturstriche darauf alle 8,5 cm, weil die schmalen Lücken zwischen den Fiedern kein Umriss sind, den Körper und Zeichnung teilen können. Die Platten sind 8 mm dick, das Merkmal von 1:50. Die fünf toten Wedel sind schmale Blätter, die am Stamm hängen."),
      text("Thermal: trunk-mantle is the flare, the trunk and the crown cup by reference; fronds are every stipe, rachis, lamina and crosier by reference plus twelve transpiration outlets as thin patches on twelve laminae; dead-fronds the skirt. Structure: the rhizome with the inner mantle as solid members of the declared sections, every stipe and rachis as a slender member inside it, joints at every rig joint, the root-ball support and the masses at their centres.", "Thermik: trunk-mantle sind Fuß, Stamm und Kronenbecher per Verweis; fronds sind jeder Stiel, jede Rhachis, jede Spreite und jede Farnschnecke per Verweis sowie zwölf Transpirationsauslässe als dünne Flecken auf zwölf Spreiten; dead-fronds der Rock. Struktur: das Rhizom mit dem inneren Mantel als volle Glieder der deklarierten Querschnitte, jeder Stiel und jede Rhachis als schlankes Glied darin, Gelenke an jedem Riggelenk, die Auflagerung des Wurzelballens und die Massen in ihren Schwerpunkten."),
    ],
    100: [
      text("1:100 keeps every frond as plates 15.6 mm thick in fewer bands, one pinna stroke for two pinnae (every 17 cm), three crosiers; stipes and rachises are thickened to the feature.", "1:100 behält jeden Wedel als 15,6 mm dicke Platten in weniger Bändern, einen Fiederstrich für zwei Fiedern (alle 17 cm), drei Farnschnecken; Stiele und Rhachiden sind auf das Merkmal verdickt."),
      text("Thermal and structure as at 1:50 with the merged solids; transpiration outlets on the same twelve fronds.", "Thermik und Struktur wie bei 1:50 mit den zusammengefassten Körpern; Transpirationsauslässe auf denselben zwölf Wedeln."),
    ],
    200: [
      text("1:200 keeps every frond as whole lanceolate plates of 32 mm without pinna strokes (the pinnae are below 30 mm apart at their base), one crosier, trunk and crown as plain lofts.", "1:200 behält jeden Wedel als ganze lanzettliche Platten von 32 mm ohne Fiederstriche (die Fiedern liegen an ihrer Basis weniger als 30 mm auseinander), eine Farnschnecke, Stamm und Krone als schlichte Lofts."),
      text("Thermal references the merged solids and keeps the twelve outlets; structure keeps every member and the joints.", "Die Thermik verweist auf die zusammengefassten Körper und behält die zwölf Auslässe; die Struktur behält jedes Glied und die Gelenke."),
    ],
    500: [
      text("1:500 keeps what reads at 75 mm: the flaring trunk, the crown cup and every frond as an 80 mm blade, so the wide shuttlecock crown and the skirt still read in all three views; crosiers are omitted.", "1:500 behält, was bei 75 mm lesbar ist: den sich verbreiternden Stamm, den Kronenbecher und jeden Wedel als Blatt von 80 mm, sodass die weite Federballkrone und der Rock in allen drei Ansichten lesbar bleiben; Farnschnecken entfallen."),
      text("Thermal: the merged solids by reference and the twelve outlets; structure: trunk, crown and frond members, the trunk joints, the support and the masses; frond joints are left out at this scale.", "Thermik: die zusammengefassten Körper per Verweis und die zwölf Auslässe; Struktur: Stamm-, Kronen- und Wedelglieder, die Stammgelenke, das Auflager und die Massen; Wedelgelenke entfallen in diesem Maßstab."),
    ],
  },
  repDecisions: {
    50: {
      silhouette: text("At 1:50 (1 m is 20 mm) every drawing is the true orthographic view of the 1:50 model in the calm pose: the flaring trunk with the strands of its root mantle, the crown cup, the crosiers and twenty-three fronds drawn as tilted plates along each lamina, the pinnae as lobes and their midveins as strokes.", "Bei 1:50 (1 m sind 20 mm) ist jede Zeichnung die wahre Parallelprojektion des 1:50-Modells in der ruhigen Pose: der sich verbreiternde Stamm mit den Strängen seines Wurzelmantels, der Kronenbecher, die Farnschnecken und dreiundzwanzig Wedel als geneigte Platten entlang jeder Spreite, die Fiedern als Lappen und ihre Mittelnerven als Striche."),
      retained: text("Retained: every frond, every pinna, the crosiers, the dead skirt and the root strands facing the viewer.", "Beibehalten: jeder Wedel, jede Fieder, die Farnschnecken, der tote Rock und die dem Betrachter zugewandten Wurzelstränge."),
      merged: text("The pinnules of each pinna are merged into its lobe; the root mantle into the trunk outline.", "Die Fiederchen jeder Fieder sind in ihren Lappen aufgenommen; der Wurzelmantel in den Stammumriss."),
      omitted: text("Omitted: pinnules, the hairs of the stipes and the sori, below 0.15 mm on paper.", "Weggelassen: Fiederchen, die Haare der Stiele und die Sori, unter 0,15 mm auf dem Papier."),
      thinElements: text("Pinna midveins and root strands are the finest strokes (0.13 mm); the laminae keep their 8 mm feature thickness edge-on.", "Fiedermittelnerven und Wurzelstränge sind die feinsten Striche (0,13 mm); die Spreiten behalten hochkant ihre Merkmalsdicke von 8 mm."),
      hatching: text("No hatching: mantle and fronds are told apart by tone and strokes.", "Keine Schraffur: Mantel und Wedel unterscheiden sich durch Ton und Striche."),
      styles: text("Light line: 0.25 mm outlines, 0.18 mm detail, 0.13 mm texture. Dark solid: the dark brown mantle, the green fronds and the dry skirt in three tones.", "Helle Linie: Umrisse 0,25 mm, Detail 0,18 mm, Textur 0,13 mm. Dunkle Fläche: der dunkelbraune Mantel, die grünen Wedel und der trockene Rock in drei Tönen."),
      overlays: text("Each thermal region has its own shapes: trunk-mantle the trunk and crown cup, fronds the green plates and crosiers, dead-fronds the hanging plates; strokes carry no region and follow their bone.", "Jede thermische Region hat eigene Formen: trunk-mantle Stamm und Kronenbecher, fronds die grünen Platten und Farnschnecken, dead-fronds die hängenden Platten; Striche tragen keine Region und folgen ihrem Knochen."),
    },
    100: {
      silhouette: text("At 1:100 the drawings are the true views of the 1:100 model: trunk, crown cup, three crosiers and twenty-three fronds with a lobe for every two pinnae.", "Bei 1:100 sind die Zeichnungen die wahren Ansichten des 1:100-Modells: Stamm, Kronenbecher, drei Farnschnecken und dreiundzwanzig Wedel mit einem Lappen für je zwei Fiedern."),
      retained: text("Retained: every frond with its lobes, the root strands and the skirt.", "Beibehalten: jeder Wedel mit seinen Lappen, die Wurzelstränge und der Rock."),
      merged: text("Two pinnae are drawn as one lobe.", "Zwei Fiedern sind als ein Lappen gezeichnet."),
      omitted: text("Omitted: single pinnae closer than 15 mm, pinnules and hairs.", "Weggelassen: einzelne Fiedern unter 15 mm Abstand, Fiederchen und Haare."),
      thinElements: text("Midveins are 0.13 mm strokes; laminae keep their 15 mm feature thickness.", "Mittelnerven sind Striche von 0,13 mm; die Spreiten behalten ihre Merkmalsdicke von 15 mm."),
      hatching: text("No hatching.", "Keine Schraffur."),
      styles: text("Light line: 0.25 mm outlines, 0.13 mm texture. Dark solid: three tones as at 1:50.", "Helle Linie: Umrisse 0,25 mm, Textur 0,13 mm. Dunkle Fläche: drei Töne wie bei 1:50."),
      overlays: text("As at 1:50: trunk-mantle, fronds and dead-fronds each on their own shapes.", "Wie bei 1:50: trunk-mantle, fronds und dead-fronds je auf eigenen Formen."),
    },
    200: {
      silhouette: text("At 1:200 the drawings are the true views of the 1:200 model: trunk, crown cup, one crosier and every frond as a whole lanceolate blade.", "Bei 1:200 sind die Zeichnungen die wahren Ansichten des 1:200-Modells: Stamm, Kronenbecher, eine Farnschnecke und jeder Wedel als ganzes lanzettliches Blatt."),
      retained: text("Retained: every frond with its arch and the skirt.", "Beibehalten: jeder Wedel mit seinem Bogen und der Rock."),
      merged: text("The pinnae of each lamina are merged into a whole blade.", "Die Fiedern jeder Spreite sind zu einem ganzen Blatt zusammengefasst."),
      omitted: text("Omitted: pinnae, midveins and root strands (below 30 mm).", "Weggelassen: Fiedern, Mittelnerven und Wurzelstränge (unter 30 mm)."),
      thinElements: text("Stipes and rachises are thickened to the 30 mm feature.", "Stiele und Rhachiden sind auf das Merkmal von 30 mm verdickt."),
      hatching: text("No hatching.", "Keine Schraffur."),
      styles: text("Light line: 0.2 mm outlines. Dark solid: three tones.", "Helle Linie: Umrisse 0,2 mm. Dunkle Fläche: drei Töne."),
      overlays: text("trunk-mantle, fronds and dead-fronds each on their own shapes.", "trunk-mantle, fronds und dead-fronds je auf eigenen Formen."),
    },
    500: {
      silhouette: text("At 1:500 the drawings are the true views of the 1:500 model: trunk, crown cup and every frond as a plain blade.", "Bei 1:500 sind die Zeichnungen die wahren Ansichten des 1:500-Modells: Stamm, Kronenbecher und jeder Wedel als schlichtes Blatt."),
      retained: text("Retained: every frond and the dead skirt.", "Beibehalten: jeder Wedel und der tote Rock."),
      merged: text("Each frond is one plain blade of 80 mm; stipe and rachis are thickened into it.", "Jeder Wedel ist ein schlichtes Blatt von 80 mm; Stiel und Rhachis sind darin verdickt."),
      omitted: text("Omitted: pinnae, crosiers and root strands; they are far below 0.15 mm at this scale.", "Weggelassen: Fiedern, Farnschnecken und Wurzelstränge; sie liegen bei diesem Maßstab weit unter 0,15 mm."),
      thinElements: text("Stipes and blades are thickened to the 75 mm feature so they stay printable.", "Stiele und Spreiten sind auf das Merkmal von 75 mm verdickt, damit sie druckbar bleiben."),
      hatching: text("No hatching; the pen is 0.15 mm for outlines and 0.25 mm for the ground line.", "Keine Schraffur; der Stift ist 0,15 mm für Umrisse und 0,25 mm für die Bodenlinie."),
      styles: text("Light line: closed outlines only. Dark solid: the wide crown reads as one mass over the dark trunk.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: die weite Krone liest sich als eine Masse über dem dunklen Stamm."),
      overlays: text("Thermal and structural overlays map onto trunk-mantle, fronds and dead-fronds.", "Thermische und statische Überlagerungen bilden sich ab auf trunk-mantle, fronds und dead-fronds."),
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
