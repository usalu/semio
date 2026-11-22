/** 🌿️ Cherry laurel (Prunus laurocerasus, loose unclipped evergreen shrub 3.0 m high, 3.6 by 3.1 m): five stems from one root crown carry five lumpy masses of large leathery leaves; rig, masses, poses, clips, thermal and structure, the four drawings and the four volumes, every solid and shape stated by hand in world metres.
 * Usage: repo tools exec bun -- cherry-laurel.ts
 * @see ./lib.ts
 * @see ../../📝️PW-13.md */
import {
  ENVIRONMENT, EXPORTS, GUST, PENS, RANGES, REFERENCES, SHADE, T, THERMAL_ASSUMPTIONS, VIEWS, clipTube, de1, diurnalClip, drag, extent, f1, feature, flarePolygon, flareSolid, frames, laurelMark, leafyPolygon, leafySolid, leafyVertices, manifestRow, marks, r4, r5, save, trueExtentNote,
  shapeOf, simplify, swayKeys, toLocal, tube, tubePolygons, tubeSolid, unite, windAt, type Flare, type Json, type Leafy, type Look, type P2, type Tube, type V3, type View,
} from "./lib.ts";

const ID = "cherry-laurel";
const LEAF: Look = { material: "laurel-leaf", tone: 0.55 };
const SHADOW: Look = { material: "laurel-leaf", tone: 0.4 };
const BARK: Look = { material: "laurel-bark", tone: 0.35 };
const WOOD: Look = { material: "laurel-wood", tone: 0.4 };
const MULCH: Look = { material: "bark-mulch", tone: 0.22, role: "detail" };
const DETAIL = (tone: number): Look => ({ ...WOOD, tone, role: "detail" });

/** 🌿️ The five stems and their leaf masses (world metres): stem base (x, z), stem tip (x, y, z), radius at the base and at the tip, mass centre, semi-axes (x, up, down, z), crown mass in kg. */
type Plant = { base: [number, number]; tip: V3; r0: number; r1: number; c: V3; rx: number; up: number; down: number; rz: number; kg: number; seed: number };
const PLANTS: Plant[] = [
  { base: [0.1, 0.05], tip: [0.72, 1.18, 0.34], r0: 0.05, r1: 0.032, c: [0.95, 1.55, 0.45], rx: 0.85, up: 0.75, down: 0.8, rz: 0.8, kg: 30, seed: 3 },
  { base: [-0.08, 0.08], tip: [-0.72, 1.22, 0.45], r0: 0.048, r1: 0.031, c: [-0.95, 1.5, 0.6], rx: 0.85, up: 0.8, down: 0.8, rz: 0.85, kg: 32, seed: 7 },
  { base: [0.02, -0.1], tip: [0.12, 1.28, -0.72], r0: 0.052, r1: 0.033, c: [0.12, 1.45, -0.85], rx: 0.9, up: 0.85, down: 0.85, rz: 0.8, kg: 34, seed: 13 },
  { base: [0.0, 0.0], tip: [0.04, 1.7, 0.08], r0: 0.058, r1: 0.036, c: [0.0, 2.2, 0.05], rx: 0.9, up: 0.8, down: 0.75, rz: 0.9, kg: 36, seed: 19 },
  { base: [-0.1, -0.06], tip: [-0.62, 0.98, -0.5], r0: 0.045, r1: 0.03, c: [-0.95, 1.15, -0.6], rx: 0.8, up: 0.7, down: 0.75, rz: 0.75, kg: 28, seed: 29 },
];
const K = PLANTS.map((_, i) => i + 1);
const P = (k: number): Plant => PLANTS[k - 1]!;
const deg = (r: number) => (r * 180) / Math.PI;

/** 🦴️ The planar rig: the root crown along x (world -0.5 to 0.5); each stem from its base to its fork, placed laterally by `z`; each leaf mass from the fork. */
const rig = {
  kind: "botanical",
  root: "root-plate",
  bones: [
    { id: "root-plate", parent: null, title: T("Root crown", "Wurzelstock"), x: -0.5, y: 0, angleDeg: 0, lengthM: 1, joint: { type: "fixed" }, mass: { kg: 120, comX: 0.5, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: 0.096211, secondMomentM4: 0.0007366, extremeFibreM: 0.175, material: "laurel-wood", title: T("Root crown, solid section d = 35 cm", "Wurzelstock, Vollquerschnitt d = 35 cm") } },
    ...K.map((k) => {
      const p = P(k), dx = p.tip[0] - p.base[0], dy = p.tip[1];
      const d = (p.r0 + p.r1);
      return { id: `stem-${k}`, parent: "root-plate", title: T(`Stem ${k}`, `Stamm ${k}`), x: r4(p.base[0] + 0.5), y: 0, z: p.base[1], angleDeg: r4(deg(Math.atan2(dy, dx))), lengthM: r4(Math.hypot(dx, dy)), joint: { type: "revolute", minDeg: -6, maxDeg: 6 }, mass: { kg: 8, comX: r4(Math.hypot(dx, dy) * 0.45), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: r5((Math.PI * d * d) / 4), secondMomentM4: Number(((Math.PI * d ** 4) / 64).toExponential(4)), extremeFibreM: r4(d / 2), material: "laurel-wood", title: T(`Solid section, d = ${(d * 100).toFixed(0)} cm`, `Vollquerschnitt, d = ${(d * 100).toFixed(0)} cm`) } };
    }),
    ...K.map((k) => {
      const p = P(k), dx = p.tip[0] - p.base[0], dy = p.tip[1];
      return { id: `crown-${k}`, parent: `stem-${k}`, title: T(`Leaf mass ${k}`, `Laubmasse ${k}`), x: r4(Math.hypot(dx, dy)), y: 0, z: r4(p.tip[2] - p.base[1]), angleDeg: r4(90 - deg(Math.atan2(dy, dx))), lengthM: r4(p.c[1] + p.up - p.tip[1] - 0.1), joint: { type: "revolute", minDeg: -12, maxDeg: 12 }, mass: { kg: p.kg, comX: 0, comY: 0, distribution: "point", source: "authored-estimate", reference: "jones-2014" } };
    }),
  ],
};
const F = frames({ rig });
const local = (bone: string, p: V3): V3 => toLocal(F[bone]!, p).map(r5) as V3;
for (const k of K) {
  const b = rig.bones.find((x) => x.id === `crown-${k}`)! as Json;
  const l = local(`crown-${k}`, P(k).c);
  b.mass = { ...b.mass, comX: r4(l[0]), comY: r4(l[1]) };
}
const MASS = 120 + K.reduce((n, k) => n + 8 + P(k).kg, 0);

/** 🍃️ A leaf mass at a scale: relief of the leaf clusters 45 mm at 1:50 (clusters of 15 cm leaves, 0.25 to 0.4 m across), 28 mm at 1:100, 12 mm at 1:200, none at 1:500; the base radii grow by the relief given up, so every scale keeps the same envelope. */
const AMP: Record<number, number> = { 50: 0.045, 100: 0.028, 200: 0.012, 500: 0 };
const RES: Record<number, [number, number]> = { 50: [26, 52], 100: [18, 36], 200: [12, 24], 500: [8, 16] };
const WAVES: Record<number, [number, number]> = { 50: [11, 19], 100: [8, 13], 200: [5, 8], 500: [1, 2] };
const crown = (k: number, scale: number): Leafy => {
  const p = P(k), g = (AMP[50]! - AMP[scale]!) * 0.85, [nLat, nLon] = RES[scale]!;
  const own: Leafy = { id: `crown-${k}`, bone: `crown-${k}`, look: LEAF, c: p.c, rx: p.rx + g, ryTop: p.up + g, ryBot: p.down + g, rz: p.rz + g, amp: AMP[scale]!, waves: WAVES[scale]!, nLat, nLon, seed: p.seed + scale, squash: 0.15 };
  return scale === 50 ? own : { ...own, fit: extent(leafyVertices(crown(k, 50))) };
};
const shade = (k: number, scale: number): Leafy => {
  const p = P(k), [nLat, nLon] = RES[Math.max(scale, 200)]!;
  return { id: `th-shade-${k}`, bone: `crown-${k}`, look: SHADOW, c: [p.c[0] * 0.9, p.c[1] - 0.05, p.c[2] * 0.9], rx: p.rx * 0.55, ryTop: p.up * 0.5, ryBot: p.down * 0.55, rz: p.rz * 0.55, amp: scale === 50 ? 0.02 : 0, waves: [6, 10], nLat, nLon, seed: p.seed + 101 };
};

/** 🪵️ A stem from its base to 0.3 m into its leaf mass, bowed slightly outwards. */
const stemTube = (k: number, n: number, id = `stem-${k}`, look = BARK, shrink = 1, rmin = 0): Tube => {
  const p = P(k);
  const end: V3 = [p.tip[0] + (p.c[0] - p.tip[0]) * 0.35, p.tip[1] + (p.c[1] - p.tip[1]) * 0.35 + 0.1, p.tip[2] + (p.c[2] - p.tip[2]) * 0.35];
  const lx = end[0] - p.base[0], lz = end[2] - p.base[1], lh = Math.hypot(lx, lz) || 1;
  const foot: [number, number, number, number] = [p.base[0], 0.012, p.base[1], r5(Math.max(rmin, p.r0 * 1.15 * shrink))];
  const rise: [number, number, number, number] = [r4(p.base[0] + (0.012 * lx) / lh), 0.08, r4(p.base[1] + (0.012 * lz) / lh), r5(Math.max(rmin, p.r0 * shrink))];
  return tube(id, `stem-${k}`, look, [foot, rise, ...Array.from({ length: n }, (_, j) => {
    const i = j + 1, s = 0.06 + (0.94 * i) / n, f = s ** 0.9, bow = 0.06 * Math.sin(Math.PI * s);
    const x = p.base[0] + (end[0] - p.base[0]) * f, z = p.base[1] + (end[2] - p.base[1]) * f, y = end[1] * s;
    const h = Math.hypot(x, z) || 1;
    return [r4(x + (bow * x) / h), r4(Math.max(y, 0.16)), r4(z + (bow * z) / h), r5(Math.max(rmin, (p.r0 + (p.r1 - p.r0) * s) * shrink))] as [number, number, number, number];
  })]);
};
const FLARE: Flare = { id: "flare", bone: "root-plate", look: BARK, c: [0, 0, 0], profile: [[0, 0.2], [0.03, 0.16], [0.08, 0.11], [0.14, 0.08], [0.2, 0.06]] };
const FLARE100: Flare = { ...FLARE, profile: [[0, 0.19], [0.06, 0.12], [0.2, 0.06]] };
const mulch = (r: number, h: number, n: number): Flare => ({ id: "mulch", bone: "root-plate", look: MULCH, c: [0, 0, 0], profile: n > 2 ? [[0, r], [r5(h * 0.5), r5(r * 0.95)], [r5(h * 0.85), r5(r * 0.7)], [h, r5(r * 0.3)], [h, 0]] : [[0, r], [h, r], [h, 0]] });
const MULCH_BY: Record<number, Flare | null> = { 50: mulch(0.8, 0.025, 4), 100: mulch(0.8, 0.028, 4), 200: mulch(0.8, 0.03, 2), 500: null };
/** 🌰️ At 1:500 the root crown is one foot of 0.24 m that the five stems of 80 mm stand on. */
const FOOT500: Flare = { id: "root-foot", bone: "root-plate", look: BARK, c: [0, 0, 0], profile: [[0, 0.12], [0.06, 0.1], [0.08, 0]] };

const ofRef = (id: string, of: string, region: string): Json => ({ id, of, region });
const member = (s: Json, m: string, part: string, extra: Json = {}): Json => ({ ...s, member: m, part, ...extra });
const ell = (id: string, bone: string, look: Look, c: V3, radii: V3): Json => ({ id, kind: "ellipsoid", bone, material: look.material, role: look.role ?? "body", tone: look.tone, radii: radii.map(r5), transform: { translation: local(bone, c) } });
const box = (id: string, bone: string, look: Look, c: V3, size: V3): Json => ({ id, kind: "box", bone, material: look.material, role: look.role ?? "body", tone: look.tone, size: size.map(r5), transform: { translation: local(bone, c) } });

/** 💨️ Transpiration anchors: the sunlit outlet on the top of every mass, the shade outlet on its outer flank. */
const OUT = (k: number, which: 0 | 1): V3 => {
  const p = P(k);
  if (which === 0) return [r4(p.c[0]), r4(p.c[1] + p.up), r4(p.c[2])];
  const h = Math.hypot(p.c[0], p.c[2]) || 1, a = which === 1 && h < 0.1 ? [1, 0] : [p.c[0] / h, p.c[2] / h];
  return [r4(p.c[0] + a[0]! * p.rx * 0.75), r4(p.c[1] + 0.3), r4(p.c[2] + a[1]! * p.rz * 0.75)];
};
const lenses = (scale: number): Json[] => {
  const t = Math.max(0.01, feature(scale) / 2);
  return K.flatMap((k) => [
    { ...ell(`th-outlet-sun-${k}`, `crown-${k}`, { ...LEAF, role: "detail" }, OUT(k, 0), [0.14, t, 0.14]), region: "leaf-sun", tags: ["transpiration-sun"] },
    { ...ell(`th-outlet-shade-${k}`, `crown-${k}`, { ...SHADOW, role: "detail" }, OUT(k, 1), [0.1, t, 0.1]), region: "leaf-shade", tags: ["transpiration-shade"] },
  ]);
};

/** 🦴️ Branches inside a leaf mass: from the fork towards the top and the flanks (fractions of the semi-axes). */
const BRANCHES: [number, number, number][] = [[0.1, 0.85, 0.05], [0.7, 0.35, 0.2], [-0.55, 0.4, -0.5], [0.15, 0.3, 0.75]];
function branches(k: number, n: number, r: number): Json[] {
  const p = P(k);
  return BRANCHES.slice(0, n).map(([fx, fy, fz], i) => {
    const b: V3 = [r4(p.c[0] + fx * p.rx), r4(p.c[1] + fy * p.up), r4(p.c[2] + fz * p.rz)];
    const m: V3 = [r4((p.tip[0] + b[0]) / 2), r4((p.tip[1] + b[1]) / 2 + 0.05), r4((p.tip[2] + b[2]) / 2)];
    return member(tubeSolid(tube(`branch-${k}-${i + 1}`, `crown-${k}`, { ...WOOD, tone: 0.42, flags: { slender: true } }, [[...p.tip, r], [...m, r * 0.85], [...b, r * 0.6]]), F), `crown-${k}`, "member");
  });
}
const rootAndSupport = (scale: number): Json[] => {
  const h = Math.max(0.016, feature(scale) * 1.05);
  if (scale === 500) return [member(box("member-root-plate", "root-plate", WOOD, [0, 0.0375, 0], [0.3, 0.075, 0.3]), "root-plate", "member"), member(box("support-base-support", "root-plate", DETAIL(0.25), [0, -0.0375, 0], [1.0, 0.075, 1.0]), "root-plate", "support", { anchor: "base-support" })];
  return [
    member(box("member-root-plate", "root-plate", WOOD, [0, h / 2 + 0.001, 0], [0.9, h, 0.9]), "root-plate", "member"),
    member(box("support-base-support", "root-plate", DETAIL(0.25), [0, -h / 2, 0], [1.0, h, 1.0]), "root-plate", "support", { anchor: "base-support" }),
    member(ell("mass-root-plate", "root-plate", DETAIL(0.3), [0, h / 2, 0], [0.1, h / 2, 0.1]), "root-plate", "mass"),
  ];
};
const STEM_N: Record<number, number> = { 50: 8, 100: 4, 200: 2, 500: 1 };
const stemsAt = (scale: number): Tube[] => K.map((k) => stemTube(k, STEM_N[scale]!, `stem-${k}`, BARK, 1, scale === 500 ? 0.04 : scale === 200 ? 0.016 : 0));

function volume(scale: 50 | 100 | 200 | 500): Json {
  const stems = stemsAt(scale);
  const flare = scale === 50 ? FLARE : scale === 100 ? FLARE100 : null;
  const body = [...K.map((k) => leafySolid(crown(k, scale), F)), ...stems.map((t) => tubeSolid(t, F)), ...(flare ? [flareSolid(flare, F)] : []), ...(MULCH_BY[scale] ? [flareSolid(MULCH_BY[scale]!, F)] : [flareSolid(FOOT500, F)])];
  const thermal = [
    ...lenses(scale),
    ...K.map((k) => ofRef(`th-crown-${k}`, `crown-${k}`, "leaf-sun")),
    ...K.map((k) => ({ ...leafySolid(shade(k, scale), F), region: "leaf-shade" })),
    ...stems.map((t) => ({ ...tubeSolid(clipTube({ ...t, id: `th-${t.id}` }, P(Number(t.bone.slice(5))).c[1] - P(Number(t.bone.slice(5))).down + 0.05), F), region: "stem-bark" })),
    ...(flare ? [ofRef("th-flare", "flare", "stem-bark")] : []),
  ];
  const fine = scale <= 100;
  const structure = [
    ...rootAndSupport(scale),
    ...K.flatMap((k) => {
      const p = P(k);
      if (scale === 500) return [
        member(tubeSolid(tube(`member-stem-${k}`, `stem-${k}`, WOOD, [[p.base[0], 0, p.base[1], 0.04], [...p.tip, 0.04]]), F), `stem-${k}`, "member"),
        member(tubeSolid(tube(`member-crown-${k}`, `crown-${k}`, WOOD, [[...p.tip, 0.04], [p.c[0], p.c[1] + p.up * 0.85, p.c[2], 0.04]]), F), `crown-${k}`, "member"),
        member(ell(`mass-crown-${k}`, `crown-${k}`, DETAIL(0.3), p.c, [0.08, 0.08, 0.08]), `crown-${k}`, "mass"),
      ];
      return [
        ...(fine ? [member(ell(`joint-stem-${k}`, `stem-${k}`, DETAIL(0.35), [p.base[0], 0.05, p.base[1]], [0.06, 0.03, 0.06]), `stem-${k}`, "joint")] : []),
        member(tubeSolid(stemTube(k, fine ? 4 : 1, `member-stem-${k}`, WOOD, 0.9, scale === 200 ? 0.016 : 0), F), `stem-${k}`, "member"),
        member(ell(`joint-crown-${k}`, `crown-${k}`, DETAIL(0.35), p.tip, [0.05, 0.05, 0.05]), `crown-${k}`, "joint"),
        member(tubeSolid(tube(`member-crown-${k}`, `crown-${k}`, WOOD, [[...p.tip, 0.03], [r4((p.tip[0] + p.c[0]) / 2), r4((p.tip[1] + p.c[1] + p.up) / 2), r4((p.tip[2] + p.c[2]) / 2), 0.024], [p.c[0], r4(p.c[1] + p.up * 0.85), p.c[2], 0.016]]), F), `crown-${k}`, "member"),
        ...(fine ? branches(k, scale === 50 ? 4 : 2, scale === 50 ? 0.011 : 0.012) : []),
        member(ell(`mass-crown-${k}`, `crown-${k}`, DETAIL(0.3), p.c, [0.05, 0.05, 0.05]), `crown-${k}`, "mass"),
      ];
    }),
  ];
  return { status: "authored", denominator: scale, body, thermal, structure, decisions: VDEC[scale] };
}

const VDEC: Record<number, Json[]> = {
  50: [
    T("Body: five lumpy leaf masses as closed leaf skins on 26 by 52 stations with a relief of the leaf clusters of 45 mm (clusters of 10 to 15 cm leaves, 0.25 to 0.4 m across), overlapping into one loose, broad dome 3.6 by 3.1 m and 3.0 m high; five stems from one root crown (9 to 12 cm at the collar) with an outward bow and the root flare; a mulch disc of 1.6 m. The lowest 0.4 m stays open under the outer masses.", "Körper: fünf knollige Laubmassen als geschlossene Laubhäute auf 26 mal 52 Stationen mit einem Relief der Blattbüschel von 45 mm (Büschel aus 10 bis 15 cm großen Blättern, 0,25 bis 0,4 m breit), zu einer lockeren, breiten Kuppel von 3,6 mal 3,1 m und 3,0 m Höhe überlappend; fünf Stämme aus einem Wurzelstock (9 bis 12 cm am Wurzelhals) mit einem Bogen nach außen und dem Wurzelanlauf; eine Mulchscheibe von 1,6 m. Die untersten 0,4 m bleiben unter den äußeren Massen offen."),
    T("Thermal: leaf-sun is the five leaf skins by reference; leaf-shade (hidden) one inner mass of half the size in every leaf mass; stem-bark the stems up to the underside of their masses and the flare; outlet lenses on the tops and flanks, tagged with their emitter.", "Thermisch: leaf-sun sind die fünf Laubhäute per Verweis; leaf-shade (verborgen) je Laubmasse eine innere Masse halber Größe; stem-bark die Stämme bis zur Unterseite ihrer Massen und der Anlauf; Auslasslinsen auf den Oberseiten und Flanken, mit ihrem Emitter markiert."),
    T("Structure: root crown and fixed support, per stem a collar joint, the stem member, the fork joint, a main branch to the top of the mass and four branches to its flanks, the mass of the leaves.", "Tragwerk: Wurzelstock und eingespanntes Auflager, je Stamm ein Wurzelhalsgelenk, das Stammglied, das Gabelgelenk, ein Hauptast zum Scheitel der Masse und vier Äste zu ihren Flanken, die Masse des Laubes."),
  ],
  100: [
    T("Body modelled anew for 1:100: the five leaf masses on 18 by 36 stations with a softer relief of 28 mm and base radii grown by the relief given up, so the envelope stays; stems with four segments, a simpler flare, the mulch disc.", "Körper für 1:100 neu modelliert: die fünf Laubmassen auf 18 mal 36 Stationen mit einem weicheren Relief von 28 mm und um das aufgegebene Relief vergrößerten Grundradien, sodass die Hüllform bleibt; Stämme mit vier Abschnitten, ein einfacherer Anlauf, die Mulchscheibe."),
    T("Thermal: leaf-sun by reference, leaf-shade coarser inner masses, stem-bark the stems below the masses and the flare, outlet lenses.", "Thermisch: leaf-sun per Verweis, leaf-shade gröbere innere Massen, stem-bark die Stämme unter den Massen und der Anlauf, Auslasslinsen."),
    T("Structure: stem members with collar and fork joints, a main branch and two flank branches per mass, root crown, support and masses.", "Tragwerk: Stammglieder mit Wurzelhals- und Gabelgelenk, je Masse ein Hauptast und zwei Flankenäste, Wurzelstock, Auflager und Massen."),
  ],
  200: [
    T("Body modelled anew for 1:200: five leaf masses on 12 by 24 stations with only the large lobes (12 mm), five straight stems of at least 32 mm, the mulch as a slab of 3 cm.", "Körper für 1:200 neu modelliert: fünf Laubmassen auf 12 mal 24 Stationen nur mit den großen Lappen (12 mm), fünf gerade Stämme von mindestens 32 mm, der Mulch als Platte von 3 cm."),
    T("Thermal: leaf-sun by reference, leaf-shade smooth inner masses, stem-bark the five stems below their masses.", "Thermisch: leaf-sun per Verweis, leaf-shade glatte innere Massen, stem-bark die fünf Stämme unter ihren Massen."),
    T("Structure: one stem member and one main branch per mass with the fork joint, root crown, support and masses; flank branches fall below the 30 mm feature.", "Tragwerk: je Masse ein Stammglied und ein Hauptast mit dem Gabelgelenk, Wurzelstock, Auflager und Massen; die Flankenäste liegen unter dem Merkmal von 30 mm."),
  ],
  500: [
    T("Body modelled anew for 1:500: five smooth leaf masses on 8 by 16 stations (the relief is below the 75 mm feature) on five straight stems of 80 mm; the mulch and the flare are omitted.", "Körper für 1:500 neu modelliert: fünf glatte Laubmassen auf 8 mal 16 Stationen (das Relief liegt unter dem Merkmal von 75 mm) auf fünf geraden Stämmen von 80 mm; Mulch und Anlauf entfallen."),
    T("Thermal: leaf-sun the masses by reference, leaf-shade smooth inner masses, stem-bark the stems of 80 mm below the masses.", "Thermisch: leaf-sun die Massen per Verweis, leaf-shade glatte innere Massen, stem-bark die Stämme von 80 mm unter den Massen."),
    T("Structure: round bars of 80 mm for stems and main branches, the root crown, the support slab and the masses.", "Tragwerk: Rundstäbe von 80 mm für Stämme und Hauptäste, der Wurzelstock, die Auflagerplatte und die Massen."),
  ],
};

const volumes = { "50": volume(50), "100": volume(100), "200": volume(200), "500": volume(500) };
const ext = extent(K.flatMap((k) => leafyVertices(crown(k, 50))));
const dims = { heightM: r4(ext.max[1]), widthM: r4(ext.max[0] - ext.min[0]), depthM: r4(ext.max[2] - ext.min[2]), massKg: MASS };

/** 🔭️ The order of the masses seen in a view, farthest first: the elevation looks from the right (smaller z nearer), the front from ahead (larger x nearer), the plan from above (higher nearer). */
const ORDER = (view: View): number[] => [...K].sort((a, b) => (view === "elevation" ? P(b).c[2] - P(a).c[2] : view === "front" ? P(a).c[0] - P(b).c[0] : P(a).c[1] + P(a).up - P(b).c[1] - P(b).up));

function drawing(scale: 50 | 100 | 200 | 500): Json {
  const g: Record<View, Json[]> = { elevation: [], front: [], plan: [] };
  for (const view of VIEWS) {
    const out = g[view];
    const rings = Object.fromEntries(K.map((k) => [k, simplify(leafyPolygon(view, crown(k, scale)), scale)]));
    if (scale === 500) {
      const all = unite([...K.map((k) => rings[k]!), ...(view === "plan" ? [] : [...stemsAt(500).flatMap((t) => tubePolygons(view, t)), flarePolygon(view, FOOT500)])], 500);
      out.push(shapeOf(view, F, { id: "shrub", bone: "root-plate", role: "body", z: 2, filled: true, tone: 0.55, regions: ["leaf-sun", "leaf-shade", "stem-bark"] }, all));
      continue;
    }
    if (view !== "plan") {
      const m = MULCH_BY[scale]!;
      out.push(shapeOf(view, F, { id: "mulch", bone: "root-plate", role: "detail", z: 1, filled: true, tone: 0.25, material: "bark-mulch" }, unite([flarePolygon(view, m)], scale)));
      for (const t of stemsAt(scale)) {
        const k = Number(t.bone.slice(5));
        const polys = [...tubePolygons(view, t), ...(k === 4 && scale <= 100 ? [flarePolygon(view, scale === 50 ? FLARE : FLARE100)] : [])];
        out.push(shapeOf(view, F, { id: t.id, bone: t.bone, role: "body", z: 1.5 + k / 100, filled: true, tone: 0.35, region: "stem-bark", material: "laurel-bark" }, unite(polys, scale)));
      }
    }
    ORDER(view).forEach((k, i) => {
      const z = 2 + i * 0.1;
      out.push(shapeOf(view, F, { id: `crown-${k}`, bone: `crown-${k}`, role: "body", z, filled: true, tone: 0.55, region: "leaf-sun", material: "laurel-leaf" }, [rings[k]!]));
      const coreRing = simplify(leafyPolygon(view, shade(k, scale)), scale);
      const drawCore = view === "front" || scale === 200;
      if (drawCore) out.push(shapeOf(view, F, { id: `shade-${k}`, bone: `crown-${k}`, role: "body", z: z + 0.03, filled: true, tone: 0.4, region: "leaf-shade", ...(scale <= 100 ? { hatch: "shade-lines" } : {}) }, [coreRing]));
      if (scale <= 100) {
        const pitch = scale === 50 ? 0.26 : 0.38, size = scale === 50 ? 0.16 : 0.22;
        const lines = marks(laurelMark, rings[k]!, pitch, size, scale === 50 ? 0.05 : 0.07, k * 17 + scale + VIEWS.indexOf(view) * 5, drawCore ? [coreRing] : [], view === "plan" ? Math.PI : 0.7);
        if (lines.length) out.push(shapeOf(view, F, { id: `leaf-marks-${k}`, bone: `crown-${k}`, role: "texture", z: z + 0.06, filled: false, tone: 0.1 }, lines, false));
      }
    });
  }
  return { status: "authored", denominator: scale, validRange: RANGES[scale], pens: PENS[scale], hatches: SHADE[scale], geometry: g, decisions: DEC[scale] };
}

const D = (silhouette: Json, retained: Json, merged: Json, omitted: Json, thinElements: Json, hatching: Json, styles: Json, overlays: Json) => ({ silhouette, retained, merged, omitted, thinElements, hatching, styles, overlays });
const DEC: Record<number, Json> = {
  50: D(
    T("At 1:50 (1 m is 20 mm) the cherry laurel is drawn in full: five overlapping, lumpy leaf masses with the scalloped outline of their leaf clusters, large leaf marks (the curved edge and midrib of 10 to 15 cm leaves), five bowed stems with the root flare in the open foot and the mulch disc. Front and plan: the front shows the hatched shaded interior of every mass, nearer masses over farther ones; the plan shows the five masses from above with leaf marks.", "Bei 1:50 (1 m sind 20 mm) wird der Kirschlorbeer vollständig gezeichnet: fünf überlappende, knollige Laubmassen mit dem gelappten Umriss ihrer Blattbüschel, große Blattzeichen (der gebogene Rand und die Mittelrippe der 10 bis 15 cm großen Blätter), fünf gebogene Stämme mit dem Wurzelanlauf im offenen Fuß und die Mulchscheibe. Vorder- und Draufsicht: die Vorderansicht zeigt das schraffierte beschattete Innere jeder Masse, nähere Massen über ferneren; die Draufsicht zeigt die fünf Massen von oben mit Blattzeichen."),
    T("Retained: the lobes of every leaf mass, every stem, the flare and the mulch disc.", "Beibehalten: die Lappen jeder Laubmasse, jeder Stamm, der Anlauf und die Mulchscheibe."),
    T("Merged: nothing; each leaf mass, stem and stroke set stays its own shape on its own bone.", "Zusammengefasst: nichts; jede Laubmasse, jeder Stamm und jede Strichgruppe bleibt eine eigene Form auf ihrem eigenen Knochen."),
    T("Omitted: single leaves and the flower racemes of April, which no pose can switch.", "Weggelassen: einzelne Blätter und die Blütentrauben im April, die keine Pose schalten kann."),
    T("Stems (6 to 12 cm) are drawn at their true width; leaf marks are the finest strokes (0.13 mm).", "Stämme (6 bis 12 cm) werden in wahrer Breite gezeichnet; Blattzeichen sind die feinsten Striche (0,13 mm)."),
    T("A thin 45 degree line hatch marks the shaded interior of each mass in the front.", "Eine feine 45-Grad-Schraffur kennzeichnet in der Vorderansicht das beschattete Innere jeder Masse."),
    T("Light line: 0.25 mm outlines, 0.18 mm detail, 0.13 mm texture. Dark solid: the glossy dark masses overlap in depth order over lighter stems and mulch.", "Helle Linie: Umrisse 0,25 mm, Detail 0,18 mm, Textur 0,13 mm. Dunkle Fläche: die glänzend dunklen Massen überlappen in Tiefenfolge über helleren Stämmen und Mulch."),
    T("Each thermal region has its own shapes: leaf-sun the masses, leaf-shade the interiors (front), stem-bark the stems; strokes carry no region and follow their mass.", "Jede thermische Region hat eigene Formen: leaf-sun die Massen, leaf-shade das Innere (Vorderansicht), stem-bark die Stämme; Striche tragen keine Region und folgen ihrer Masse."),
  ),
  100: D(
    T("At 1:100 (1 m is 10 mm) the five masses keep a softer lobed outline and fewer, larger leaf marks; the stems with the flare over the mulch disc. Front and plan: the front keeps the hatched interiors; the plan shows the five masses with leaf marks.", "Bei 1:100 (1 m sind 10 mm) behalten die fünf Massen einen weicheren gelappten Umriss und weniger, größere Blattzeichen; die Stämme mit dem Anlauf über der Mulchscheibe. Vorder- und Draufsicht: die Vorderansicht behält das schraffierte Innere; die Draufsicht zeigt die fünf Massen mit Blattzeichen."),
    T("Retained: the five masses in depth order, the open foot with the stems.", "Beibehalten: die fünf Massen in Tiefenfolge, der offene Fuß mit den Stämmen."),
    T("Merged: small lobes into the larger leaf clusters.", "Zusammengefasst: kleine Lappen in die größeren Blattbüschel."),
    T("Omitted: every second leaf mark and the stem bow detail.", "Weggelassen: jedes zweite Blattzeichen und die Feinheit des Stammbogens."),
    T("Stems stay at their true width (0.6 to 1.2 mm); nothing is thickened.", "Die Stämme bleiben in wahrer Breite (0,6 bis 1,2 mm); nichts wird verstärkt."),
    T("The shaded interiors in the front keep a 0.6 mm line hatch.", "Das beschattete Innere in der Vorderansicht behält eine Linienschraffur von 0,6 mm."),
    T("Light line: 0.20 mm outlines, 0.13 mm detail, 0.10 mm texture. Dark solid: dark masses on a light foot.", "Helle Linie: Umrisse 0,20 mm, Detail 0,13 mm, Textur 0,10 mm. Dunkle Fläche: dunkle Massen auf hellem Fuß."),
    T("Each thermal region keeps its own shapes: leaf-sun, leaf-shade (front) and stem-bark.", "Jede thermische Region behält eigene Formen: leaf-sun, leaf-shade (Vorderansicht) und stem-bark."),
  ),
  200: D(
    T("At 1:200 (1 m is 5 mm) the cherry laurel is drawn as five masses with only their large lobes, each with its shaded interior outline in front and plan, on five straight stems over the mulch slab.", "Bei 1:200 (1 m sind 5 mm) wird der Kirschlorbeer als fünf Massen nur mit ihren großen Lappen gezeichnet, jede mit dem Umriss ihres beschatteten Inneren in Vorder- und Draufsicht, auf fünf geraden Stämmen über der Mulchplatte."),
    T("Retained: the five masses in depth order and the open foot with five stems.", "Beibehalten: die fünf Massen in Tiefenfolge und der offene Fuß mit fünf Stämmen."),
    T("Merged: the leaf clusters into the large lobes; the flare into the stems.", "Zusammengefasst: die Blattbüschel zu den großen Lappen; der Anlauf in die Stämme."),
    T("Omitted: leaf marks and hatching.", "Weggelassen: Blattzeichen und Schraffur."),
    T("The thinner stems are drawn at the 32 mm minimum, above the 30 mm feature.", "Die dünneren Stämme werden mit dem Minimum von 32 mm gezeichnet, über dem Merkmal von 30 mm."),
    T("No hatching at this scale.", "Bei diesem Maßstab keine Schraffur."),
    T("Light line: 0.18 mm outlines. Dark solid: dark masses with lighter interiors.", "Helle Linie: Umrisse 0,18 mm. Dunkle Fläche: dunkle Massen mit helleren Innenflächen."),
    T("Every thermal region keeps its own shapes: leaf-sun the masses, leaf-shade the interiors, stem-bark the stems.", "Jede thermische Region behält eigene Formen: leaf-sun die Massen, leaf-shade das Innere, stem-bark die Stämme."),
  ),
  500: D(
    T("At 1:500 (1 m is 2 mm) the cherry laurel is one composed silhouette 7 mm wide: the broad lobed dome of the five masses with the stems of the open foot. Front and plan: the front is the dome seen from ahead; the plan the lobed crown outline.", "Bei 1:500 (1 m sind 2 mm) ist der Kirschlorbeer eine komponierte Silhouette von 7 mm Breite: die breite gelappte Kuppel der fünf Massen mit den Stämmen des offenen Fußes. Vorder- und Draufsicht: die Vorderansicht ist die Kuppel von vorn; die Draufsicht der gelappte Kronenumriss."),
    T("Retained: the broad, loose dome and the open foot.", "Beibehalten: die breite, lockere Kuppel und der offene Fuß."),
    T("Merged: the five masses, their interiors and the stems into one path carrying all three regions.", "Zusammengefasst: die fünf Massen, ihr Inneres und die Stämme zu einem Pfad, der alle drei Regionen trägt."),
    T("Omitted: the mulch, the flare and all texture.", "Weggelassen: der Mulch, der Anlauf und jede Textur."),
    T("The stems are thickened to the 75 mm feature (80 mm); nothing else is thickened.", "Die Stämme werden auf das Merkmal von 75 mm verstärkt (80 mm); sonst wird nichts verstärkt."),
    T("No hatching at this scale.", "Bei diesem Maßstab keine Schraffur."),
    T("Light line: closed outline only. Dark solid: one dark dome.", "Helle Linie: nur geschlossener Umriss. Dunkle Fläche: eine dunkle Kuppel."),
    T("Thermal and structural overlays map onto the one merged path with the area-weighted value of leaf-sun, leaf-shade and stem-bark, labelled as merged.", "Thermische und statische Überlagerungen bilden auf den einen zusammengefassten Pfad mit dem flächengewichteten Wert von leaf-sun, leaf-shade und stem-bark ab, als zusammengefasst beschriftet."),
  ),
};

function symbol(): Json {
  const base = { bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5 };
  const view = (v: View) => unite([...K.map((k) => leafyPolygon(v, crown(k, 500))), ...(v === "plan" ? [] : stemsAt(500).flatMap((t) => tubePolygons(v, t)))], 200);
  const circle = (r: number): P2[] => Array.from({ length: 20 }, (_, i) => ({ x: r * Math.cos((2 * Math.PI * i) / 20), y: r * Math.sin((2 * Math.PI * i) / 20) }));
  return {
    id: `${ID}-symbol`,
    title: T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
    description: T("A conventional plan sign of the shrub for site and landscape plans: the lobed outline of the true crown extent and the root crown. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen des Strauches für Lage- und Freiraumpläne: der gelappte Umriss der wahren Kronenausdehnung und der Wurzelstock. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
    semantics: "true-extent",
    units: "model-m",
    denominators: [500, 200, 100],
    pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
    hatches: [],
    dimensionalNote: trueExtentNote(dims, T("the lobed outline is the crown and the circle the root crown at its real position.", "Der gelappte Umriss ist die Krone und der Kreis der Wurzelstock in wirklicher Lage.")),
    geometry: {
      elevation: [shapeOf("elevation", F, { id: "shrub", ...base }, view("elevation"))],
      front: [shapeOf("front", F, { id: "shrub", ...base }, view("front"))],
      plan: [shapeOf("plan", F, { id: "shrub", ...base }, view("plan")), shapeOf("plan", F, { id: "stem", bone: "root-plate", role: "body", z: 4, filled: true, tone: 0.25 }, [circle(0.12)])],
    },
  };
}

const bind = (stem: number, sun: number, shd: number, wet = 0, e = 0.8) => ({
  regions: { "stem-bark": { exposedFraction: 1, sunlitFraction: stem, ...(wet ? { wettedness: wet * 0.8 } : {}) }, "leaf-sun": { exposedFraction: 1, sunlitFraction: sun, ...(wet ? { wettedness: wet } : {}) }, "leaf-shade": { exposedFraction: 1, sunlitFraction: shd, ...(wet ? { wettedness: wet } : {}) } },
  emitters: { "transpiration-sun": e, "transpiration-shade": e },
});
const areaOf = (k: number) => Math.PI * ((P(k).up + P(k).down) / 2) * P(k).rz;
const force = (k: number, v10: number, cd: number) => r4(drag(cd, areaOf(k), windAt(v10, P(k).c[1])));
const loads = (v10: number, cd: number, ref: string) => K.map((k) => {
  const l = local(`crown-${k}`, P(k).c);
  return { id: `wind-crown-${k}`, kind: "wind", title: T(`Wind drag on leaf mass ${k}`, `Winddruck auf Laubmasse ${k}`), bone: `crown-${k}`, x: r4(l[0]), y: r4(l[1]), fxN: force(k, v10, cd), fyN: 0, source: "literature", reference: ref };
});
const total = (v10: number, cd: number) => K.reduce((n, k) => n + force(k, v10, cd), 0);
const lean = (stem: number, crown: number) => Object.fromEntries(K.flatMap((k) => [[`stem-${k}`, { angleDeg: r4(stem * (k === 4 ? 0.8 : 1)) }], [`crown-${k}`, { angleDeg: r4(crown * (k === 4 ? 1.2 : 1)) }]]));
const LEAN = { windy: [-0.6, -2.2], storm: [-1.8, -6.5] } as const;
const SNOWQ = 250;
const snowOf = (k: number) => r4(SNOWQ * Math.PI * P(k).rx * P(k).rz * 0.8);
/** ❄️ Under wet snow the outer masses bow outwards and down (the stems spread), the top mass sinks. */
const SPLAY = Object.fromEntries(K.flatMap((k) => {
  const s = k === 4 ? 0 : Math.sign(P(k).c[0]) || 1;
  return [[`stem-${k}`, { angleDeg: r4(-1.5 * s) }], [`crown-${k}`, { angleDeg: r4(-6 * s) }]];
}));

const poses = [
  { id: "calm", title: T("Calm summer day", "Windstiller Sommertag"), description: T(`Light air of 1.5 m/s at 10 m: the shrub stands in its loose, broad habit; drag ${f1(total(1.5, 0.5))} N on all five leaf masses.`, `Leiser Zug von 1,5 m/s in 10 m Höhe: der Strauch steht in seinem lockeren, breiten Wuchs; Widerstand ${de1(total(1.5, 0.5))} N auf allen fünf Laubmassen.`), bones: {}, ground: "authored", contacts: [], thermal: bind(0.03, 0.45, 0.04), loads: loads(1.5, 0.5, "en-1991-1-4") },
  { id: "windy", title: T("Fresh breeze", "Frische Brise"), description: T(`Beaufort 5, 8 m/s at 10 m (${f1(windAt(8, 1.5))} m/s at 1.5 m); cd 0.5 on about 2 m2 of frontal area per leaf mass gives ${f1(force(1, 8, 0.5))} to ${f1(force(4, 8, 0.5))} N per mass, ${f1(total(8, 0.5))} N in all. The stems lean 0.6 and the masses 2.2 degrees downwind.`, `Beaufort 5, 8 m/s in 10 m Höhe (${de1(windAt(8, 1.5))} m/s in 1,5 m); cd 0,5 auf etwa 2 m2 Anströmfläche je Laubmasse ergibt ${de1(force(1, 8, 0.5))} bis ${de1(force(4, 8, 0.5))} N je Masse, insgesamt ${de1(total(8, 0.5))} N. Die Stämme neigen sich um 0,6, die Massen um 2,2 Grad nach Lee.`), bones: lean(LEAN.windy[0], LEAN.windy[1]), ground: "authored", contacts: [], thermal: bind(0.03, 0.45, 0.04, 0, 1), loads: loads(8, 0.5, "en-1991-1-4") },
  { id: "storm", title: T("Gale", "Sturm"), description: T(`Beaufort 9, 22 m/s at 10 m; the large leathery leaves fold and cluster, so cd falls to 0.3 (Vogel 1989): ${f1(total(22, 0.3))} N on the shrub; the masses lean about 8 degrees and the base moment reaches about ${f1(K.reduce((n, k) => n + force(k, 22, 0.3) * P(k).c[1], 0) / 1000)} kN m.`, `Beaufort 9, 22 m/s in 10 m Höhe; die großen ledrigen Blätter falten sich und legen sich zusammen, sodass cd auf 0,3 sinkt (Vogel 1989): ${de1(total(22, 0.3))} N auf den Strauch; die Massen neigen sich um etwa 8 Grad, das Fußmoment erreicht etwa ${de1(K.reduce((n, k) => n + force(k, 22, 0.3) * P(k).c[1], 0) / 1000)} kNm.`), bones: lean(LEAN.storm[0], LEAN.storm[1]), ground: "authored", contacts: [], thermal: bind(0, 0.2, 0, 0.4, 0.5), loads: loads(22, 0.3, "vogel-1989") },
  { id: "wet-snow", title: T("Wet snow", "Nassschnee"), description: T(`The broad evergreen leaves hold wet snow: 0.25 kN/m2 on about 80 % of each mass's plan area gives ${snowOf(5).toFixed(0)} to ${snowOf(4).toFixed(0)} N per mass (EN 1991-1-3 order of magnitude, authored estimate). The outer masses bow outwards by 7.5 degrees and the stems spread, the typical splaying of a laurel under snow; wind is calm.`, `Die breiten immergrünen Blätter halten Nassschnee: 0,25 kN/m2 auf etwa 80 % der Grundfläche jeder Masse ergeben ${snowOf(5).toFixed(0)} bis ${snowOf(4).toFixed(0)} N je Masse (Größenordnung nach EN 1991-1-3, Schätzung). Die äußeren Massen biegen sich um 7,5 Grad nach außen und die Stämme spreizen sich, das typische Auseinanderfallen eines Lorbeers unter Schnee; Windstille.`), bones: SPLAY, ground: "authored", contacts: [], thermal: bind(0, 0, 0, 0.5, 0), loads: K.map((k) => { const l = local(`crown-${k}`, [P(k).c[0], P(k).c[1] + P(k).up * 0.8, P(k).c[2]]); return { id: `snow-crown-${k}`, kind: "snow", title: T(`Wet snow on leaf mass ${k}`, `Nassschnee auf Laubmasse ${k}`), bone: `crown-${k}`, x: r4(l[0]), y: r4(l[1]), fxN: 0, fyN: -snowOf(k), source: "authored-estimate", reference: "en-1991-1-3" }; }) },
];

const PHASES = [0, 0.31, 0.62, 0.15, 0.47];
const clips = [
  {
    id: "wind-sway", title: T("Wind sway", "Windschwingung"),
    description: T("A deterministic 2.4 s cycle about the fresh-breeze lean: the five leaf masses rock by 1.2 degrees on their stems at about 0.8 Hz with their own phases (two swings per cycle for the light outer masses), so the loose shrub moves in parts rather than as one block.", "Ein deterministischer Zyklus von 2,4 s um die Neigung der frischen Brise: die fünf Laubmassen wiegen sich mit etwa 0,8 Hz um 1,2 Grad auf ihren Stämmen, jede mit eigener Phase, sodass sich der lockere Strauch in Teilen bewegt statt als ein Block."),
    durationS: 2.4, fps: 30, loop: true, rootMotion: "none",
    keys: swayKeys(2.4, 12, Object.fromEntries(K.flatMap((k) => [[`stem-${k}`, { base: LEAN.windy[0], amp: 0.3, phase: PHASES[k - 1]! - 0.1 }], [`crown-${k}`, { base: LEAN.windy[1], amp: 1.2, phase: PHASES[k - 1]! }]]))),
    contacts: [], phases: [], events: [],
    tracks: ["transpiration-sun", "transpiration-shade"].map((e) => ({ target: `emitter:${e}`, ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 1.2, v: 0.9 }, { t: 2.4, v: 1 }] })),
  },
  {
    id: "gust", title: T("Gust", "Windböe"),
    description: T("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the masses lean up to 70 % of the gale lean.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Massen neigen sich bis zu 70 % der Sturmneigung."),
    durationS: 8, fps: 30, loop: false, rootMotion: "none",
    keys: GUST.t.map((t, i) => ({ t, pose: "windy", ease: "smooth", bones: lean(r4(0.7 * LEAN.storm[0] * GUST.lean[i]!), r4(0.7 * LEAN.storm[1] * GUST.lean[i]!)) })),
    contacts: [], phases: [], events: [{ t: 2.5, kind: "load-on", title: T("Gust peak", "Böenspitze") }],
    tracks: K.map((k) => ({ target: `load:wind-crown-${k}`, ease: "smooth", keys: GUST.t.map((t, i) => ({ t, v: GUST.v[i]! })) })),
  },
  diurnalClip({ "stem-bark": 0.03, "leaf-sun": 0.45, "leaf-shade": 0.04 }, ["transpiration-sun", "transpiration-shade"]),
];

const anchorOf = (id: string, k: number, p: V3, en: string, de: string): Json => {
  const l = local(`crown-${k}`, p);
  return { id, bone: `crown-${k}`, x: r4(l[0]), y: r4(l[1]), z: r4(l[2]), kind: "emitter", title: T(en, de) };
};

/** 🪵️ The outer bark area of the stems below the leaf masses and the flare, as measured on the 1:50 thermal solids (`check.ts --info`). */
const STEM_AREA = 1.55;
const PLAN = Math.PI * (dims.widthM / 2) * (dims.depthM / 2);
const SUN = 26, SHADE_AREA = 18;

const asset: Json = {
  id: ID,
  category: "vegetation",
  rig,
  anchors: [
    { id: "base-support", bone: "root-plate", x: 0.5, y: 0, kind: "support", title: T("Root crown anchorage", "Verankerung des Wurzelstocks"), support: "fixed", normalDeg: 90, footprintLengthM: 1, footprintWidthM: 1, frictionCoefficient: 0.6 },
    ...K.map((k) => anchorOf(`tp-sun-${k}`, k, OUT(k, 0), `Transpiration of the sunlit leaves, anchor ${k}`, `Transpiration der besonnten Blätter, Anker ${k}`)),
    ...K.map((k) => anchorOf(`tp-shade-${k}`, k, OUT(k, 1), `Transpiration of the shaded leaves, anchor ${k}`, `Transpiration der beschatteten Blätter, Anker ${k}`)),
  ],
  title: T("Cherry Laurel", "Kirschlorbeer"),
  summary: T(
    `An unclipped cherry laurel (Prunus laurocerasus) ${dims.heightM.toFixed(1)} m high and ${dims.widthM.toFixed(1)} by ${dims.depthM.toFixed(1)} m broad: a loose evergreen shrub of five stems from one root crown, each carrying a lumpy mass of large, glossy, leathery leaves; the masses overlap into a broad dome that stays open in the lowest 0.4 m, where the bowed stems and the mulched bed show.`,
    `Ein ungeschnittener Kirschlorbeer (Prunus laurocerasus), ${dims.heightM.toFixed(1).replace(".", ",")} m hoch und ${dims.widthM.toFixed(1).replace(".", ",")} mal ${dims.depthM.toFixed(1).replace(".", ",")} m breit: ein lockerer immergrüner Strauch aus fünf Stämmen aus einem Wurzelstock, jeder mit einer knolligen Masse großer, glänzender, ledriger Blätter; die Massen überlappen zu einer breiten Kuppel, die in den untersten 0,4 m offen bleibt, wo die gebogenen Stämme und das gemulchte Beet zu sehen sind.`,
  ),
  tags: ["shrub", "evergreen", "broadleaf", "unclipped", "wind", "snow", "transpiration"],
  provenance: {
    author: "Semio staffage vegetation content worker PW-13",
    method: "handcrafted",
    created: "2026-11-22",
    references: REFERENCES,
    notes: T(
      "Prunus laurocerasus: evergreen shrub or small tree, usually 2 to 6 m, often broader than high, with alternate, leathery, glossy leaves 5 to 15 cm long and erect white racemes in April and May (Stace 2019; Roloff and Bärtels 2018). An unclipped garden shrub of 3 m with a spread of 3.6 m is an authored typical specimen. Masses: about 32 kg per leaf mass (leaves and branches) and 8 kg per stem with green Prunus wood of about 800 kg/m3 (black cherry of the Wood Handbook as proxy), authored estimates. Flowers cannot be toggled by a pose and are not drawn; the fourth pose is wet snow, under which laurels typically splay open.",
      "Prunus laurocerasus: immergrüner Strauch oder kleiner Baum, meist 2 bis 6 m, oft breiter als hoch, mit wechselständigen, ledrigen, glänzenden Blättern von 5 bis 15 cm Länge und aufrechten weißen Trauben im April und Mai (Stace 2019; Roloff und Bärtels 2018). Ein ungeschnittener Gartenstrauch von 3 m Höhe und 3,6 m Breite ist ein typisches Exemplar nach Autorenwahl. Massen: etwa 32 kg je Laubmasse (Blätter und Äste) und 8 kg je Stamm bei frischem Prunusholz von etwa 800 kg/m3 (Spätblühende Traubenkirsche des Wood Handbook als Ersatz), Schätzungen. Blüten lassen sich durch eine Pose nicht schalten und werden nicht gezeichnet; die vierte Pose ist Nassschnee, unter dem Lorbeer typischerweise auseinanderfällt.",
    ),
  },
  dimensions: dims,
  materials: [
    { id: "laurel-bark", title: T("Smooth dark grey-brown laurel bark with lenticels", "Glatte dunkel graubraune Lorbeerrinde mit Lentizellen"), emissivity: 0.94, solarAbsorptivity: 0.7, densityKgM3: 700, conductivityWmK: 0.13, source: "authored-estimate", reference: "jones-2014" },
    { id: "laurel-leaf", title: T("Large glossy leathery laurel leaves", "Große glänzende ledrige Lorbeerblätter"), emissivity: 0.96, solarAbsorptivity: 0.55, source: "literature", reference: "jones-2014" },
    { id: "laurel-wood", title: T("Green cherry laurel wood", "Frisches Kirschlorbeerholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 800, conductivityWmK: 0.16, elasticModulusPa: 9000000000, strengthPa: 55000000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "bark-mulch", title: T("Bark mulch on the bed", "Rindenmulch auf dem Beet"), emissivity: 0.95, solarAbsorptivity: 0.85, densityKgM3: 250, conductivityWmK: 0.07, source: "authored-estimate", reference: "jones-2014" },
  ],
  poses,
  clips,
  thermal: {
    environment: ENVIRONMENT,
    regions: [
      { id: "stem-bark", title: T("Stems under the leaf masses", "Stämme unter den Laubmassen"), kind: "bark", model: "passive", material: "laurel-bark", areaM2: STEM_AREA, characteristicLengthM: 0.08, geometry: "cylinder", skyViewFactor: 0.1, layers: [], heatCapacityJM2K: 54000, source: "authored-estimate", reference: "wood-handbook-2021" },
      { id: "leaf-sun", title: T("Sunlit leaves of the outer masses", "Besonnte Blätter der äußeren Massen"), kind: "foliage", model: "leaf", material: "laurel-leaf", areaM2: SUN, areaBasis: "exchange", characteristicLengthM: 0.05, geometry: "vertical-plate", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 1700, leaf: { stomatalConductanceMaxMS: 0.0035, leafAreaIndex: r4(SUN / PLAN), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
      { id: "leaf-shade", title: T("Shaded leaves inside the masses", "Beschattete Blätter im Inneren der Massen"), kind: "foliage", model: "leaf", material: "laurel-leaf", areaM2: SHADE_AREA, areaBasis: "exchange", characteristicLengthM: 0.06, geometry: "vertical-plate", skyViewFactor: 0.1, layers: [], heatCapacityJM2K: 1700, leaf: { stomatalConductanceMaxMS: 0.002, leafAreaIndex: r4(SHADE_AREA / PLAN), photosynthesis: "c3" }, hidden: T("The shaded leaves lie inside the outer leaf masses and are never a body surface.", "Die beschatteten Blätter liegen im Inneren der äußeren Laubmassen und sind nie eine Körperfläche."), source: "authored-estimate", reference: "jones-2014" },
    ],
    emitters: [
      { id: "transpiration-sun", kind: "transpiration", title: T("Transpiration of the sunlit leaves (distributed over the crown)", "Transpiration der besonnten Blätter (über die Krone verteilt)"), anchor: "tp-sun-1", anchors: K.map((k) => `tp-sun-${k}`), directionDeg: 0, diameterM: 0.8, region: "leaf-sun", source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-shade", kind: "transpiration", title: T("Transpiration of the shaded leaves (distributed over the crown)", "Transpiration der beschatteten Blätter (über die Krone verteilt)"), anchor: "tp-shade-1", anchors: K.map((k) => `tp-shade-${k}`), directionDeg: 0, diameterM: 0.6, region: "leaf-shade", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    assumptions: THERMAL_ASSUMPTIONS,
    operating: [],
    passengers: [],
  },
  structure: {
    gravityMS2: 9.80665,
    dynamics: "quasi-static",
    supports: ["base-support"],
    assumptions: [T("One rigid body per evaluated pose, fixed in the root crown; wind drag acts at the centre of every leaf mass, snow on their tops, gravity on all bone masses. The stems are planar bones whose lateral spread is carried by their placement; dynamic amplification, stem breakage under snow and root-soil failure are not modelled.", "Ein starrer Körper je ausgewerteter Pose, im Wurzelstock eingespannt; Winddruck wirkt in der Mitte jeder Laubmasse, Schnee auf ihren Oberseiten, Schwerkraft auf allen Knochenmassen. Die Stämme sind ebene Knochen, deren seitliche Spreizung ihre Lage trägt; dynamische Überhöhung, Stammbruch unter Schnee und Wurzel-Boden-Versagen werden nicht modelliert.")],
    clinical: false,
  },
  volumes,
  presentation: { defaultPose: "calm", defaultClip: "wind-sway", defaultScale: 50 },
  exports: EXPORTS,
  completion: "authored",
  representations: { "500": drawing(500), "200": drawing(200), "100": drawing(100), "50": drawing(50) },
  symbols: [symbol()],
};

const path = save(asset, manifestRow(asset, T("A loose, unclipped evergreen shrub of five stems carrying lumpy masses of large leathery leaves, broader than high, whose wet-snow pose splays the masses outwards.", "Ein lockerer, ungeschnittener immergrüner Strauch aus fünf Stämmen mit knolligen Massen großer ledriger Blätter, breiter als hoch; in der Nassschneepose spreizen sich die Massen nach außen.")));
console.log(`[DEBUG] wrote ${path}: ${JSON.stringify(dims)}`);
