/** 🟢️ Boxwood ball (Buxus sempervirens, clipped topiary sphere 0.80 m, planted in the ground): rig, masses, poses, clips, thermal and structure, the four drawings and the four volumes, every solid and shape stated by hand in world metres.
 * Usage: repo tools exec bun -- boxwood-ball.ts
 * @see ./lib.ts
 * @see ../../📝️PW-13.md */
import {
  ENVIRONMENT, EXPORTS, PENS, RANGES, REFERENCES, SHADE, T, THERMAL_ASSUMPTIONS, VIEWS, boxMark, clipTube, de1, diurnalClip, drag, extent, f1, feature, flarePolygon, flareSolid, frames, leafyPolygon, leafySolid, leafyVertices, manifestRow, marks, ovoid, project, r4, r5, save, trueExtentNote,
  shapeOf, simplify, swayKeys, toLocal, tube, tubePolygons, tubeSolid, unite, windAt, GUST, type Flare, type Json, type Leafy, type Look, type P2, type Tube, type V3, type View,
} from "./lib.ts";

const ID = "boxwood-ball";
const LEAF: Look = { material: "box-leaf", tone: 0.55 };
const CORE: Look = { material: "box-leaf", tone: 0.4 };
const BARK: Look = { material: "box-bark", tone: 0.35 };
const WOOD: Look = { material: "box-wood", tone: 0.4 };
const MULCH: Look = { material: "bark-mulch", tone: 0.22, role: "detail" };
const DETAIL = (tone: number): Look => ({ ...WOOD, tone, role: "detail" });

/** 🦴️ The planar rig: the root plate along x (world -0.30 to 0.30), the stem bundle up to 0.12 m, the clipped crown up to 0.80 m. */
const rig = {
  kind: "botanical",
  root: "root-plate",
  bones: [
    { id: "root-plate", parent: null, title: T("Root ball", "Wurzelballen"), x: -0.3, y: 0, angleDeg: 0, lengthM: 0.6, joint: { type: "fixed" }, mass: { kg: 9, comX: 0.3, comY: -0.05, distribution: "point", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: 0.031416, secondMomentM4: 7.854e-5, extremeFibreM: 0.1, material: "box-wood", title: T("Root crown, solid section d = 20 cm", "Wurzelhals, Vollquerschnitt d = 20 cm") } },
    { id: "stem", parent: "root-plate", title: T("Stem bundle", "Stammbündel"), x: 0.3, y: 0, angleDeg: 90, lengthM: 0.12, joint: { type: "revolute", minDeg: -2, maxDeg: 2 }, mass: { kg: 1.2, comX: 0.06, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: 0.004021, secondMomentM4: 2.574e-7, extremeFibreM: 0.016, material: "box-wood", title: T("Five stems, d = 3.2 cm each", "Fünf Stämme, je d = 3,2 cm") } },
    { id: "crown", parent: "stem", title: T("Clipped crown ball", "Geschnittene Kronenkugel"), x: 0.12, y: 0, angleDeg: 0, lengthM: 0.68, joint: { type: "revolute", minDeg: -4, maxDeg: 4 }, mass: { kg: 23, comX: 0.31, comY: 0, distribution: "point", source: "authored-estimate", reference: "jones-2014" }, section: { areaM2: 0.001885, secondMomentM4: 4.712e-8, extremeFibreM: 0.01, material: "box-wood", title: T("Six main branches, d = 2 cm each", "Sechs Hauptäste, je d = 2 cm") } },
  ],
};
const MASS = 9 + 1.2 + 23;
/** 🪵️ The outer bark area of the stems in the open foot and the flare, as measured on the 1:50 thermal solids (`check.ts --info`). */
const STEM_AREA = 0.06;
const A0: Json = { rig };
const F = frames(A0);

/** 🟢️ The clipped ball per scale: centre 0.425 m, horizontal radius 0.392 m, 0.366 m up and 0.345 m down, so the underside clears the mulch by 7 cm; leaf-cluster relief of 9 mm at 1:50 and 6 mm at 1:100 (clipped shoots 5 to 12 cm across). */
const C: V3 = [0, 0.425, 0];
const ball = (scale: 50 | 100): Leafy =>
  scale === 50
    ? { id: "ball", bone: "crown", look: LEAF, c: C, rx: 0.396, ryTop: 0.371, ryBot: 0.345, rz: 0.396, amp: 0.013, waves: [20, 36], nLat: 34, nLon: 68, seed: 11 }
    : { id: "ball", bone: "crown", look: LEAF, c: C, rx: 0.394, ryTop: 0.369, ryBot: 0.348, rz: 0.394, amp: 0.006, waves: [12, 20], nLat: 22, nLon: 44, seed: 5 };
/** 🥚️ The smooth clipped ball of 1:200 and 1:500 as a body of revolution about the stem axis. */
const ballRev = (scale: 200 | 500): Flare => ({ id: "ball", bone: "crown", look: LEAF, c: C, profile: scale === 200 ? ovoid(0.399, 0.374, 0.352, 24) : ovoid(0.4, 0.375, 0.36, 16) });
const coreLeafy = (scale: number): Leafy => ({ id: "th-core", bone: "crown", look: CORE, c: [0, 0.44, 0], rx: 0.27, ryTop: 0.25, ryBot: 0.22, rz: 0.27, amp: scale === 50 ? 0.012 : 0.008, waves: [8, 14], nLat: scale === 50 ? 18 : 12, nLon: scale === 50 ? 36 : 24, seed: 23 });

/** 🪵️ The five stems at 1:50: base offset (x, z), offset at 0.22 m, radius at the collar and at the top, stations at 0, 0.04, 0.09, 0.15, 0.22 m. */
const STEMS: [number, number, number, number, number, number][] = [
  [0.028, 0.004, 0.12, 0.02, 0.0175, 0.012], [-0.012, 0.026, -0.04, 0.115, 0.016, 0.011], [-0.026, -0.01, -0.118, -0.03, 0.0165, 0.0115],
  [0.004, -0.03, 0.03, -0.12, 0.016, 0.011], [0.012, 0.012, 0.05, 0.05, 0.013, 0.009],
];
const stemTube = (k: number, ys: number[], id = `stem-${k + 1}`, look = BARK, shrink = 1): Tube => {
  const [bx, bz, tx, tz, r0, r1] = STEMS[k]!;
  return tube(id, "stem", look, ys.map((y) => {
    const f = (y / 0.22) ** 1.3;
    return [r4(bx + (tx - bx) * f), y, r4(bz + (tz - bz) * f), r5((y === 0 ? r0 * 1.1 : r0 + ((r1 - r0) * y) / 0.22) * shrink)];
  }));
};
const ST50 = [0, 0.04, 0.09, 0.15, 0.22];
const stems50 = STEMS.map((_, k) => stemTube(k, ST50));
const stems100 = [0, 1, 2].map((k) => stemTube(k, [0, 0.1, 0.2]));
const stem200 = tube("stem", "stem", BARK, [[0, 0, 0, 0.03], [0, 0.07, 0, 0.022], [0, 0.18, 0, 0.022]]);
const stem500 = tube("stem", "stem", BARK, [[0, 0, 0, 0.04], [0, 0.2, 0, 0.04]]);
const flare50: Flare = { id: "flare", bone: "stem", look: BARK, c: [0, 0, 0], profile: [[0, 0.062], [0.01, 0.052], [0.022, 0.04], [0.036, 0.032], [0.05, 0.028]] };
const flare100: Flare = { id: "flare", bone: "stem", look: BARK, c: [0, 0, 0], profile: [[0, 0.055], [0.02, 0.036], [0.04, 0.028]] };
const mulch = (r: number, h: number): Flare => ({ id: "mulch", bone: "root-plate", look: MULCH, c: [0, 0, 0], profile: [[0, r], [r5(h * 0.5), r5(r * 0.94)], [r5(h * 0.85), r5(r * 0.66)], [h, r5(r * 0.3)], [h, 0]] });
const MULCH50 = mulch(0.3, 0.024);
const MULCH100: Flare = { id: "mulch", bone: "root-plate", look: MULCH, c: [0, 0, 0], profile: [[0, 0.3], [0.018, 0.24], [0.026, 0]] };
const mulchSlab: Flare = { id: "mulch", bone: "root-plate", look: MULCH, c: [0, 0, 0], profile: [[0, 0.3], [0.03, 0.3], [0.03, 0]] };

const local = (bone: string, p: V3): V3 => toLocal(F[bone]!, p).map(r5) as V3;
const ofRef = (id: string, of: string, region: string): Json => ({ id, of, region });
const member = (s: Json, m: string, part: string, extra: Json = {}): Json => ({ ...s, member: m, part, ...extra });
const ell = (id: string, bone: string, look: Look, c: V3, radii: V3): Json => ({ id, kind: "ellipsoid", bone, material: look.material, role: look.role ?? "body", tone: look.tone, radii, transform: { translation: local(bone, c) } });
const box = (id: string, bone: string, look: Look, c: V3, size: V3): Json => ({ id, kind: "box", bone, material: look.material, role: look.role ?? "body", tone: look.tone, size, transform: { translation: local(bone, c) } });

/** 🦴️ The branch framework inside the ball: from the stem tops, six main branches run out and up to just under the clipped skin (hand-stated end points). */
const BRANCHES: [number, number, number][] = [[0.26, 0.62, 0.05], [-0.2, 0.66, 0.2], [-0.24, 0.55, -0.2], [0.12, 0.7, -0.24], [0.28, 0.4, -0.12], [-0.06, 0.76, 0.02]];
const branch = (i: number, r0: number, n: number): Json => {
  const [x, y, z] = BRANCHES[i]!;
  const s: V3 = n === 3 ? stemTube(i % 5, [0.22]).points[0]!.p : [0, 0.2, 0];
  const m: V3 = [r4((s[0] + x) * 0.45), r4(s[1] + (y - s[1]) * 0.55), r4((s[2] + z) * 0.45)];
  return member(tubeSolid(tube(`branch-${i + 1}`, "crown", { ...WOOD, flags: { slender: true } }, n === 3 ? [[...s, r0], [...m, r0 * 0.8], [x, y, z, r0 * 0.55]] : [[...s, r0], [x, y, z, r0 * 0.6]]), F), "crown", "member");
};

/** 💨️ Transpiration anchors on the clipped skin: six over the sunlit upper half, five for the shaded mass under the top. */
const skin = (az: number, el: number): V3 => {
  const a = (az * Math.PI) / 180, e = (el * Math.PI) / 180;
  return [r4(0.391 * Math.cos(e) * Math.cos(a)), r4(C[1] + 0.366 * Math.sin(e)), r4(0.391 * Math.cos(e) * Math.sin(a))];
};
const FACE = [skin(0, 25), skin(72, 30), skin(144, 25), skin(216, 30), skin(288, 25), skin(0, 88)];
const COREP = [skin(36, 60), skin(108, 62), skin(180, 60), skin(252, 62), skin(324, 60)];

/** 💧️ The transpiration outlets: one flat lens on the skin at every anchor, tagged with its emitter. */
const lenses = (scale: number): Json[] => {
  const t = Math.max(0.006, feature(scale) / 2), r = Math.max(0.05, feature(scale));
  return [...FACE.map((p, i) => ({ ...ell(`th-outlet-face-${i + 1}`, "crown", { ...LEAF, role: "detail" }, p, [r, t, r]), region: "leaf-face", tags: ["transpiration-face"] })), ...COREP.map((p, i) => ({ ...ell(`th-outlet-core-${i + 1}`, "crown", { ...CORE, role: "detail" }, p, [r * 0.8, t, r * 0.8]), region: "leaf-core", tags: ["transpiration-core"] }))];
};

function volume50(): Json {
  const b = ball(50);
  const body = [leafySolid(b, F), ...stems50.map((t) => tubeSolid(t, F)), flareSolid(flare50, F), flareSolid(MULCH50, F)];
  const thermal = [
    ...lenses(50),
    ofRef("th-ball", "ball", "leaf-face"),
    { ...leafySolid(coreLeafy(50), F), region: "leaf-core" },
    ...stems50.map((t) => ({ ...tubeSolid(clipTube({ ...t, id: `th-${t.id}` }, 0.1), F), region: "stem-bark" })),
    ofRef("th-flare", "flare", "stem-bark"),
  ];
  const structure = [
    member(box("member-root-plate", "root-plate", WOOD, [0, 0.009, 0], [0.4, 0.016, 0.4]), "root-plate", "member"),
    member(box("support-base-support", "root-plate", DETAIL(0.25), [0, -0.01, 0], [0.5, 0.02, 0.5]), "root-plate", "support", { anchor: "base-support" }),
    member(ell("mass-root-plate", "root-plate", DETAIL(0.3), [0, 0.01, 0], [0.04, 0.008, 0.04]), "root-plate", "mass"),
    member(ell("joint-collar", "stem", DETAIL(0.35), [0, 0.03, 0], [0.04, 0.02, 0.04]), "stem", "joint"),
    ...stems50.map((_, k) => member(tubeSolid(stemTube(k, [0, 0.06, 0.12], `member-stem-${k + 1}`, WOOD, 0.9), F), "stem", "member")),
    member(ell("joint-crown", "crown", DETAIL(0.35), [0, 0.12, 0], [0.05, 0.02, 0.05]), "crown", "joint"),
    member(tubeSolid(tube("member-leader", "crown", WOOD, [[0, 0.11, 0, 0.016], [0.01, 0.35, 0, 0.013], [0, 0.6, 0.01, 0.009], [0, 0.79, 0, 0.005]]), F), "crown", "member"),
    ...BRANCHES.map((_, i) => branch(i, 0.009, 3)),
    member(ell("mass-crown", "crown", DETAIL(0.3), [0, 0.43, 0], [0.035, 0.035, 0.035]), "crown", "mass"),
  ];
  return { status: "authored", denominator: 50, body, thermal, structure, decisions: VDEC[50] };
}

function volume100(): Json {
  const body = [leafySolid(ball(100), F), ...stems100.map((t) => tubeSolid(t, F)), flareSolid(flare100, F), flareSolid(MULCH100, F)];
  const thermal = [
    ...lenses(100),
    ofRef("th-ball", "ball", "leaf-face"),
    { ...leafySolid(coreLeafy(100), F), region: "leaf-core" },
    ...stems100.map((t) => ({ ...tubeSolid(clipTube({ ...t, id: `th-${t.id}` }, 0.12), F), region: "stem-bark" })),
    ofRef("th-flare", "flare", "stem-bark"),
  ];
  const structure = [
    member(box("member-root-plate", "root-plate", WOOD, [0, 0.009, 0], [0.4, 0.016, 0.4]), "root-plate", "member"),
    member(box("support-base-support", "root-plate", DETAIL(0.25), [0, -0.01, 0], [0.5, 0.02, 0.5]), "root-plate", "support", { anchor: "base-support" }),
    member(ell("mass-root-plate", "root-plate", DETAIL(0.3), [0, 0.01, 0], [0.04, 0.008, 0.04]), "root-plate", "mass"),
    member(tubeSolid(tube("member-stem", "stem", WOOD, [[0, 0, 0, 0.024], [0, 0.125, 0, 0.022]]), F), "stem", "member"),
    member(ell("joint-crown", "crown", DETAIL(0.35), [0, 0.12, 0], [0.04, 0.025, 0.04]), "crown", "joint"),
    member(tubeSolid(tube("member-leader", "crown", WOOD, [[0, 0.11, 0, 0.016], [0, 0.45, 0, 0.012], [0, 0.79, 0, 0.008]]), F), "crown", "member"),
    ...[0, 1, 2, 3].map((i) => branch(i, 0.01, 2)),
    member(ell("mass-crown", "crown", DETAIL(0.3), [0, 0.43, 0], [0.04, 0.04, 0.04]), "crown", "mass"),
  ];
  return { status: "authored", denominator: 100, body, thermal, structure, decisions: VDEC[100] };
}

function volume200(): Json {
  const body = [flareSolid(ballRev(200), F), tubeSolid(stem200, F), flareSolid(mulchSlab, F)];
  const thermal = [
    ...lenses(200),
    ofRef("th-ball", "ball", "leaf-face"),
    { ...flareSolid({ id: "th-core", bone: "crown", look: CORE, c: [0, 0.44, 0], profile: ovoid(0.27, 0.25, 0.22, 12) }, F), region: "leaf-core" },
    ...[0, 1, 2].map((k) => ({ ...tubeSolid(tube(`th-stem-${k + 1}`, "stem", BARK, [[STEMS[k]![0], 0, STEMS[k]![1], 0.016], [r4((STEMS[k]![0] + STEMS[k]![2]) / 2), 0.2, r4((STEMS[k]![1] + STEMS[k]![3]) / 2), 0.016]]), F), region: "stem-bark" })),
  ];
  const structure = [
    member(box("member-root-plate", "root-plate", WOOD, [0, 0.015, 0], [0.5, 0.03, 0.5]), "root-plate", "member"),
    member(box("support-base-support", "root-plate", DETAIL(0.25), [0, -0.015, 0], [0.5, 0.03, 0.5]), "root-plate", "support", { anchor: "base-support" }),
    member(ell("mass-root-plate", "root-plate", DETAIL(0.3), [0, 0.015, 0], [0.05, 0.015, 0.05]), "root-plate", "mass"),
    member(tubeSolid(tube("member-stem", "stem", WOOD, [[0, 0, 0, 0.025], [0, 0.125, 0, 0.025]]), F), "stem", "member"),
    member(tubeSolid(tube("member-leader", "crown", WOOD, [[0, 0.11, 0, 0.02], [0, 0.78, 0, 0.016]]), F), "crown", "member"),
    member(ell("mass-crown", "crown", DETAIL(0.3), [0, 0.43, 0], [0.05, 0.05, 0.05]), "crown", "mass"),
  ];
  return { status: "authored", denominator: 200, body, thermal, structure, decisions: VDEC[200] };
}

function volume500(): Json {
  const body = [flareSolid(ballRev(500), F), tubeSolid(stem500, F)];
  const thermal = [
    ...lenses(500),
    ofRef("th-ball", "ball", "leaf-face"),
    { ...box("th-core", "crown", CORE, [0, 0.44, 0], [0.36, 0.4, 0.4]), region: "leaf-core" },
    { ...tubeSolid(tube("th-stem", "stem", BARK, [[0, 0, 0, 0.04], [0, 0.22, 0, 0.04]]), F), region: "stem-bark" },
  ];
  const structure = [
    member(box("member-root-plate", "root-plate", WOOD, [0, 0.0375, 0], [0.08, 0.075, 0.08]), "root-plate", "member"),
    member(box("support-base-support", "root-plate", DETAIL(0.25), [0, -0.0375, 0], [0.5, 0.075, 0.5]), "root-plate", "support", { anchor: "base-support" }),
    member(box("member-stem", "stem", WOOD, [0, 0.06, 0], [0.12, 0.08, 0.08]), "stem", "member"),
    member(box("member-leader", "crown", WOOD, [0, 0.45, 0], [0.68, 0.08, 0.08]), "crown", "member"),
    member(ell("mass-crown", "crown", DETAIL(0.3), [0, 0.43, 0], [0.08, 0.08, 0.08]), "crown", "mass"),
  ];
  return { status: "authored", denominator: 500, body, thermal, structure, decisions: VDEC[500] };
}

const VDEC: Record<number, Json[]> = {
  50: [
    T("Body: the clipped ball as one closed leaf skin on 34 by 68 stations (0.78 m across, 0.80 m high) with a leaf-cluster relief of 9 mm, the dense outer shell a hedge trimmer leaves; underside 7 cm above the mulch, so five short pale stems and their root flare show below it; a mulch disc of 0.60 m.", "Körper: die geschnittene Kugel als eine geschlossene Laubhaut auf 34 mal 68 Stationen (0,78 m breit, 0,80 m hoch) mit einem Relief der Triebbüschel von 9 mm, die dichte Außenschale, die die Heckenschere hinterlässt; die Unterseite liegt 7 cm über dem Mulch, sodass fünf kurze, helle Stämme mit ihrem Wurzelanlauf darunter sichtbar sind; eine Mulchscheibe von 0,60 m."),
    T("Thermal: leaf-face is the ball skin by reference; leaf-core (hidden) is the inner leaf mass about 12 cm under the skin; stem-bark the five stems up to 0.10 m (the part in the open) and the flare.", "Thermisch: leaf-face ist die Kugelhaut per Verweis; leaf-core (verborgen) ist die innere Blattmasse etwa 12 cm unter der Haut; stem-bark die fünf Stämme bis 0,10 m (der freie Teil) und der Anlauf."),
    T("Structure: root ball and fixed support, the collar joint, five stem members, the crown joint, a leader to the top and six main branches that run out to just under the clipped skin, point masses of root ball and crown.", "Tragwerk: Wurzelballen und eingespanntes Auflager, das Wurzelhalsgelenk, fünf Stammglieder, das Kronengelenk, ein Leittrieb bis zum Scheitel und sechs Hauptäste bis knapp unter die geschnittene Haut, Punktmassen von Wurzelballen und Krone."),
  ],
  100: [
    T("Body modelled anew for 1:100: the ball skin on 22 by 44 stations with a softer relief of 6 mm, three stems instead of five with a simpler flare, the mulch disc as a four-point dome.", "Körper für 1:100 neu modelliert: die Kugelhaut auf 22 mal 44 Stationen mit einem weicheren Relief von 6 mm, drei statt fünf Stämme mit einfacherem Anlauf, die Mulchscheibe als Kuppel aus vier Punkten."),
    T("Thermal: leaf-face by reference to the skin, leaf-core one coarser inner mass, stem-bark the three stems up to 0.10 m and the flare.", "Thermisch: leaf-face per Verweis auf die Haut, leaf-core eine gröbere innere Masse, stem-bark die drei Stämme bis 0,10 m und der Anlauf."),
    T("Structure: one stem member, the crown joint, the leader and four main branches, root ball, support and point masses.", "Tragwerk: ein Stammglied, das Kronengelenk, der Leittrieb und vier Hauptäste, Wurzelballen, Auflager und Punktmassen."),
  ],
  200: [
    T("Body modelled anew for 1:200: the ball as a smooth body of revolution (the relief is below the 30 mm feature), one stem of 4.4 cm for the bundle, the mulch as a slab of 3 cm.", "Körper für 1:200 neu modelliert: die Kugel als glatter Drehkörper (das Relief liegt unter dem Merkmal von 30 mm), ein Stamm von 4,4 cm für das Bündel, der Mulch als Platte von 3 cm."),
    T("Thermal: leaf-face by reference, leaf-core an inner body of revolution, stem-bark the single stem up to 0.10 m.", "Thermisch: leaf-face per Verweis, leaf-core ein innerer Drehkörper, stem-bark der einzelne Stamm bis 0,10 m."),
    T("Structure: one stem member and one leader, root ball, support and point masses; branches fall below the feature.", "Tragwerk: ein Stammglied und ein Leittrieb, Wurzelballen, Auflager und Punktmassen; die Äste liegen unter dem Merkmal."),
  ],
  500: [
    T("Body modelled anew for 1:500: the ball as one smooth body of revolution on one stem of 80 mm (the 75 mm feature) that closes the 7 cm open foot; the mulch is omitted.", "Körper für 1:500 neu modelliert: die Kugel als ein glatter Drehkörper auf einem Stamm von 80 mm (das Merkmal von 75 mm), der den offenen Fuß von 7 cm schließt; der Mulch entfällt."),
    T("Thermal: leaf-face the ball by reference, leaf-core one inner box, stem-bark the stem of 80 mm up to 0.10 m.", "Thermisch: leaf-face die Kugel per Verweis, leaf-core eine innere Box, stem-bark der Stamm von 80 mm bis 0,10 m."),
    T("Structure: square bars of 80 mm for stem and leader, the root ball, the support slab and the crown mass.", "Tragwerk: Vierkantstäbe von 80 mm für Stamm und Leittrieb, der Wurzelballen, die Auflagerplatte und die Kronenmasse."),
  ],
};

const volumes = { "50": volume50(), "100": volume100(), "200": volume200(), "500": volume500() };

/** 📐️ Dimensions from the 1:50 body (ball skin, stems, mulch) in its rest pose. */
const pts50 = [...leafyVertices(ball(50))];
const ext = extent(pts50);
const dims = { heightM: r4(ext.max[1]), widthM: r4(ext.max[0] - ext.min[0]), depthM: r4(ext.max[2] - ext.min[2]), massKg: r4(MASS) };

/** ✏️ The drawings of one scale in one view. */
type Parts = { ball: (v: View) => P2[]; core: (v: View) => P2[]; stems: Tube[]; flare: Flare | null; mulch: Flare | null; foot?: Tube };
const PARTS: Record<number, Parts> = {
  50: { ball: (v) => leafyPolygon(v, ball(50)), core: (v) => leafyPolygon(v, coreLeafy(50)), stems: stems50, flare: flare50, mulch: MULCH50 },
  100: { ball: (v) => leafyPolygon(v, ball(100)), core: (v) => leafyPolygon(v, coreLeafy(100)), stems: stems100, flare: flare100, mulch: MULCH100 },
  200: { ball: (v) => flarePolygon(v, ballRev(200)), core: (v) => flarePolygon(v, { id: "c", bone: "crown", look: CORE, c: [0, 0.44, 0], profile: ovoid(0.27, 0.25, 0.22, 12) }), stems: [stem200], flare: null, mulch: mulchSlab },
  500: { ball: (v) => flarePolygon(v, ballRev(500)), core: () => [], stems: [stem500], flare: null, mulch: null },
};

function drawing(scale: number): Json {
  const p = PARTS[scale]!;
  const g: Record<View, Json[]> = { elevation: [], front: [], plan: [] };
  for (const view of VIEWS) {
    const out = g[view];
    const ballRing = simplify(p.ball(view), scale);
    if (scale === 500) {
      const rings = unite([ballRing, ...tubePolygons(view, stem500)], scale);
      out.push(shapeOf(view, F, { id: "ball", bone: "crown", role: "body", z: 2, filled: true, tone: 0.55, regions: ["leaf-face", "leaf-core", "stem-bark"] }, view === "plan" ? [ballRing] : rings));
      continue;
    }
    if (view !== "plan") {
      if (p.mulch) out.push(shapeOf(view, F, { id: "mulch", bone: "root-plate", role: "detail", z: 1, filled: true, tone: 0.25, material: "bark-mulch" }, unite([flarePolygon(view, p.mulch)], scale)));
      const stemPolys = [...p.stems.flatMap((t) => tubePolygons(view, clipTube(t, 0.16))), ...(p.flare ? [flarePolygon(view, p.flare)] : [])];
      out.push(shapeOf(view, F, { id: "stems", bone: "stem", role: "body", z: 1.9, filled: true, tone: 0.35, region: "stem-bark", material: "box-bark" }, unite(stemPolys, scale)));
    }
    out.push(shapeOf(view, F, { id: "ball", bone: "crown", role: "body", z: 2, filled: true, tone: 0.55, region: "leaf-face", material: "box-leaf" }, [ballRing]));
    const coreRing = simplify(p.core(view), scale);
    if (view === "front" || (view === "plan" && scale === 200)) out.push(shapeOf(view, F, { id: "core", bone: "crown", role: "body", z: 2.5, filled: true, tone: 0.4, region: "leaf-core", ...(scale <= 100 ? { hatch: "shade-lines" } : {}) }, [coreRing]));
    if (scale <= 100) {
      const pitch = scale === 50 ? 0.072 : 0.11, size = scale === 50 ? 0.042 : 0.065;
      const avoid = view === "front" ? [coreRing] : [];
      const lines = marks(boxMark, ballRing, pitch, size, scale === 50 ? 0.014 : 0.02, scale * 7 + VIEWS.indexOf(view) * 101, avoid, view === "plan" ? Math.PI : 0.6);
      if (lines.length) out.push(shapeOf(view, F, { id: "leaf-marks", bone: "crown", role: "texture", z: 3, filled: false, tone: 0.1 }, lines, false));
      if (view !== "plan") {
        const shade = Array.from({ length: 19 }, (_, i) => {
          const a = Math.PI * (1.12 + (0.76 * i) / 18);
          return { x: C[0] + 0.33 * Math.cos(a), y: C[1] + 0.29 * Math.sin(a) };
        });
        out.push(shapeOf(view, F, { id: "shade-line", bone: "crown", role: "detail", z: 2.8, filled: false, tone: 0.3 }, [shade], false));
      }
    }
  }
  return { status: "authored", denominator: scale, validRange: RANGES[scale], pens: PENS[scale], hatches: SHADE[scale], geometry: g, decisions: DEC[scale] };
}

const D = (silhouette: Json, retained: Json, merged: Json, omitted: Json, thinElements: Json, hatching: Json, styles: Json, overlays: Json) => ({ silhouette, retained, merged, omitted, thinElements, hatching, styles, overlays });
const DEC: Record<number, Json> = {
  50: D(
    T("At 1:50 (1 m is 20 mm) the boxwood ball is drawn in full: the clipped sphere 16 mm across with the lumpy outline of its clipped shoot clusters, scattered shoot marks, a shade line on the underside, five short pale stems with their root flare in the 7 cm open foot and the mulch disc. Front and plan: the front shows the hatched shaded core inside the skin; the plan shows the round clipped top with shoot marks.", "Bei 1:50 (1 m sind 20 mm) wird die Buchskugel vollständig gezeichnet: die geschnittene Kugel von 16 mm Durchmesser mit dem knolligen Umriss ihrer Triebbüschel, verstreute Triebzeichen, eine Schattenlinie an der Unterseite, fünf kurze helle Stämme mit Wurzelanlauf im 7 cm hohen offenen Fuß und die Mulchscheibe. Vorder- und Draufsicht: die Vorderansicht zeigt den schraffierten Schattenkern in der Haut; die Draufsicht zeigt die runde geschnittene Oberseite mit Triebzeichen."),
    T("Retained: the leaf-cluster relief of the clipped skin, every stem, the flare and the mulch disc.", "Beibehalten: das Relief der Triebbüschel der geschnittenen Haut, jeder Stamm, der Anlauf und die Mulchscheibe."),
    T("Merged: the five stems and the flare into one stem outline, so no ink gap between stems falls below the print minimum.", "Zusammengefasst: die fünf Stämme und der Anlauf zu einem Stammumriss, sodass keine Tintenlücke zwischen den Stämmen unter das Druckminimum fällt."),
    T("Omitted: single leaves (1.5 to 3 cm, 0.3 to 0.6 mm on paper); they are suggested by shoot marks.", "Weggelassen: einzelne Blätter (1,5 bis 3 cm, 0,3 bis 0,6 mm auf dem Papier); sie werden durch Triebzeichen angedeutet."),
    T("The stems (2.6 to 3.5 cm) are drawn at their true width; shoot marks are the finest strokes (0.13 mm).", "Die Stämme (2,6 bis 3,5 cm) werden in wahrer Breite gezeichnet; Triebzeichen sind die feinsten Striche (0,13 mm)."),
    T("A thin 45 degree line hatch marks the shaded core in the front; the skin carries only strokes.", "Eine feine 45-Grad-Schraffur kennzeichnet den Schattenkern in der Vorderansicht; die Haut trägt nur Striche."),
    T("Light line: 0.25 mm outlines, 0.18 mm detail, 0.13 mm texture. Dark solid: the dark leaf skin over lighter stems and mulch.", "Helle Linie: Umrisse 0,25 mm, Detail 0,18 mm, Textur 0,13 mm. Dunkle Fläche: die dunkle Laubhaut über helleren Stämmen und Mulch."),
    T("Each thermal region has its own shape: leaf-face the ball, leaf-core the core in the front, stem-bark the stems; strokes carry no region and follow the crown.", "Jede thermische Region hat eine eigene Form: leaf-face die Kugel, leaf-core der Kern in der Vorderansicht, stem-bark die Stämme; Striche tragen keine Region und folgen der Krone."),
  ),
  100: D(
    T("At 1:100 (1 m is 10 mm) the ball is 8 mm across: a softer clipped outline, fewer and larger shoot marks, the shade line, three stems merged into one outline with the flare, the mulch disc. Front and plan: the front keeps the hatched core; the plan shows the round top with shoot marks.", "Bei 1:100 (1 m sind 10 mm) misst die Kugel 8 mm: ein weicherer Schnittumriss, weniger und größere Triebzeichen, die Schattenlinie, drei Stämme mit dem Anlauf zu einem Umriss vereint, die Mulchscheibe. Vorder- und Draufsicht: die Vorderansicht behält den schraffierten Kern; die Draufsicht zeigt die runde Oberseite mit Triebzeichen."),
    T("Retained: the open foot with the stems and the mulch disc, the shaded core in the front.", "Beibehalten: der offene Fuß mit den Stämmen und der Mulchscheibe, der Schattenkern in der Vorderansicht."),
    T("Merged: five stems into three, drawn as one outline with the flare.", "Zusammengefasst: fünf Stämme zu dreien, als ein Umriss mit dem Anlauf gezeichnet."),
    T("Omitted: the fine relief of single shoots (below 0.1 mm here).", "Weggelassen: das feine Relief einzelner Triebe (hier unter 0,1 mm)."),
    T("Stems stay at their true width (0.3 mm); nothing is thickened.", "Die Stämme bleiben in wahrer Breite (0,3 mm); nichts wird verstärkt."),
    T("The shaded core in the front keeps a 0.6 mm line hatch.", "Der Schattenkern in der Vorderansicht behält eine Linienschraffur von 0,6 mm."),
    T("Light line: 0.20 mm outlines, 0.13 mm detail, 0.10 mm texture. Dark solid: the ball reads as one dark disc on its light foot.", "Helle Linie: Umrisse 0,20 mm, Detail 0,13 mm, Textur 0,10 mm. Dunkle Fläche: die Kugel liest als eine dunkle Scheibe auf hellem Fuß."),
    T("Each thermal region keeps its own shape: leaf-face, leaf-core (front) and stem-bark.", "Jede thermische Region behält ihre eigene Form: leaf-face, leaf-core (Vorderansicht) und stem-bark."),
  ),
  200: D(
    T("At 1:200 (1 m is 5 mm) the boxwood ball is a smooth clipped disc of 4 mm on one stem over the mulch slab. Front and plan: the front shows the core outline inside the ball; the plan shows the disc with the core.", "Bei 1:200 (1 m sind 5 mm) ist die Buchskugel eine glatte geschnittene Scheibe von 4 mm auf einem Stamm über der Mulchplatte. Vorder- und Draufsicht: die Vorderansicht zeigt den Kernumriss in der Kugel; die Draufsicht die Scheibe mit dem Kern."),
    T("Retained: the open foot (7 cm, 0.35 mm) with one stem and the mulch.", "Beibehalten: der offene Fuß (7 cm, 0,35 mm) mit einem Stamm und dem Mulch."),
    T("Merged: the stems into one stem 4.4 cm wide.", "Zusammengefasst: die Stämme zu einem Stamm von 4,4 cm Breite."),
    T("Omitted: shoot marks, relief and the shade line.", "Weggelassen: Triebzeichen, Relief und Schattenlinie."),
    T("The stem (4.4 cm) lies above the 30 mm feature; nothing is thickened.", "Der Stamm (4,4 cm) liegt über dem Merkmal von 30 mm; nichts wird verstärkt."),
    T("No hatching at this scale.", "Bei diesem Maßstab keine Schraffur."),
    T("Light line: 0.18 mm outlines. Dark solid: one dark disc, the core one tone lighter.", "Helle Linie: Umrisse 0,18 mm. Dunkle Fläche: eine dunkle Scheibe, der Kern einen Ton heller."),
    T("Every thermal region keeps its own shape: leaf-face the ball, leaf-core the core (front, plan), stem-bark the stem.", "Jede thermische Region behält ihre eigene Form: leaf-face die Kugel, leaf-core der Kern (Vorder-, Draufsicht), stem-bark der Stamm."),
  ),
  500: D(
    T("At 1:500 (1 m is 2 mm) the boxwood ball is one closed silhouette 1.6 mm across: the ball and its stem as one outline. Front and plan: the front is the same composed outline seen from ahead; the plan is the round disc.", "Bei 1:500 (1 m sind 2 mm) ist die Buchskugel eine geschlossene Silhouette von 1,6 mm Durchmesser: Kugel und Stamm als ein Umriss. Vorder- und Draufsicht: die Vorderansicht ist derselbe komponierte Umriss von vorn; die Draufsicht ist die runde Scheibe."),
    T("Retained: the round clipped form and the foot.", "Beibehalten: die runde Schnittform und der Fuß."),
    T("Merged: ball, core and stem into one path carrying all three regions.", "Zusammengefasst: Kugel, Kern und Stamm zu einem Pfad, der alle drei Regionen trägt."),
    T("Omitted: the mulch, the open gap under the ball (7 cm, below the 75 mm feature) and all texture.", "Weggelassen: der Mulch, die offene Lücke unter der Kugel (7 cm, unter dem Merkmal von 75 mm) und jede Textur."),
    T("The stem is thickened to the 75 mm feature (80 mm); nothing else is thickened.", "Der Stamm wird auf das Merkmal von 75 mm verstärkt (80 mm); sonst wird nichts verstärkt."),
    T("No hatching at this scale.", "Bei diesem Maßstab keine Schraffur."),
    T("Light line: closed outline only. Dark solid: one dark dot on a short foot.", "Helle Linie: nur geschlossener Umriss. Dunkle Fläche: ein dunkler Punkt auf kurzem Fuß."),
    T("Thermal and structural overlays map onto the one merged path, painted with the area-weighted value of leaf-face, leaf-core and stem-bark and labelled as merged.", "Thermische und statische Überlagerungen bilden auf den einen zusammengefassten Pfad ab, gemalt mit dem flächengewichteten Wert von leaf-face, leaf-core und stem-bark und als zusammengefasst beschriftet."),
  ),
};

/** 🔣️ True-extent plan symbol: crown circle, stem dot and the outline in elevation and front. */
function symbol(): Json {
  const circle = (r: number, n = 48): P2[] => Array.from({ length: n }, (_, i) => ({ x: r * Math.cos((2 * Math.PI * i) / n), y: r * Math.sin((2 * Math.PI * i) / n) }));
  const side = (v: View) => unite([flarePolygon(v, ballRev(200)), ...tubePolygons(v, stem200)], 200);
  const base = { bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5 };
  return {
    id: `${ID}-symbol`,
    title: T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
    description: T("A conventional plan sign of the clipped ball for site and landscape plans: the true crown circle, a stem dot and a cross of the clipping axes. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen der geschnittenen Kugel für Lage- und Freiraumpläne: der wahre Kronenkreis, ein Stammpunkt und ein Kreuz der Schnittachsen. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
    semantics: "true-extent",
    units: "model-m",
    denominators: [500, 200, 100],
    pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
    hatches: [],
    dimensionalNote: trueExtentNote(dims, T("the circle is the clipped ball, the dot is the stem bundle at its real position and the cross marks the clipping axes.", "Der Kreis ist die geschnittene Kugel, der Punkt das Stammbündel in wirklicher Lage und das Kreuz die Schnittachsen.")),
    geometry: {
      elevation: [shapeOf("elevation", F, { id: "ball", ...base }, side("elevation"))],
      front: [shapeOf("front", F, { id: "ball", ...base }, side("front"))],
      plan: [
        shapeOf("plan", F, { id: "ball", ...base }, [circle(dims.widthM / 2)]),
        { id: "axes", bone: "root-plate", role: "detail", z: 3, filled: false, tone: 0.4, kind: "path", d: `M ${-dims.widthM / 2} 0 L ${dims.widthM / 2} 0 M 0 ${-dims.depthM / 2} L 0 ${dims.depthM / 2}` },
        shapeOf("plan", F, { id: "stem", bone: "root-plate", role: "body", z: 4, filled: true, tone: 0.25 }, [circle(0.03, 20)]),
      ],
    },
  };
}

const bind = (stem: number, face: number, core: number, wet = 0, e = 0.8) => ({
  regions: { "stem-bark": { exposedFraction: 1, sunlitFraction: stem, ...(wet ? { wettedness: wet * 0.8 } : {}) }, "leaf-face": { exposedFraction: 1, sunlitFraction: face, ...(wet ? { wettedness: wet } : {}) }, "leaf-core": { exposedFraction: 1, sunlitFraction: core, ...(wet ? { wettedness: wet } : {}) } },
  emitters: { "transpiration-face": e, "transpiration-core": e },
});
const AREA = Math.PI * (dims.widthM / 2) ** 2;
const Z = 0.425;
const wind = (v10: number, cd: number) => r4(drag(cd, AREA, windAt(v10, Z)));
const LOAD = (id: string, fx: number, ref: string) => ({ id, kind: "wind", title: T("Wind drag on the clipped ball", "Winddruck auf die geschnittene Kugel"), bone: "crown", x: 0.305, y: 0, fxN: fx, fyN: 0, source: "literature", reference: ref });
const F_CALM = wind(1.5, 0.8), F_WIND = wind(8, 0.8), F_STORM = wind(22, 0.6);
const SNOW = r4(400 * Math.PI * 0.3 ** 2);
const LEAN = { windy: { stem: -0.3, crown: -0.8 }, storm: { stem: -0.8, crown: -2.1 } };

const poses = [
  { id: "calm", title: T("Calm summer day", "Windstiller Sommertag"), description: T(`Light air of 1.5 m/s at 10 m (${f1(windAt(1.5, Z))} m/s at the ball centre): the clipped ball stands upright, drag ${F_CALM.toFixed(2)} N.`, `Leiser Zug von 1,5 m/s in 10 m Höhe (${de1(windAt(1.5, Z))} m/s in Kugelmitte): Die geschnittene Kugel steht aufrecht, Widerstand ${F_CALM.toFixed(2).replace(".", ",")} N.`), bones: {}, ground: "authored", contacts: [], thermal: bind(0.02, 0.45, 0.04), loads: [LOAD("wind-crown", F_CALM, "en-1991-1-4")] },
  { id: "windy", title: T("Fresh breeze", "Frische Brise"), description: T(`Beaufort 5, 8 m/s at 10 m, which is ${f1(windAt(8, Z))} m/s at the 0.43 m centre of the ball; cd 0.8 on the ${AREA.toFixed(2)} m2 disc gives ${f1(F_WIND)} N. The dense clipped ball is stiff: the stems lean 0.3 and the crown 0.8 degrees downwind.`, `Beaufort 5, 8 m/s in 10 m Höhe, das sind ${de1(windAt(8, Z))} m/s in der Kugelmitte auf 0,43 m; cd 0,8 auf der Scheibe von ${AREA.toFixed(2).replace(".", ",")} m2 ergibt ${de1(F_WIND)} N. Die dicht geschnittene Kugel ist steif: die Stämme neigen sich um 0,3, die Krone um 0,8 Grad nach Lee.`), bones: { stem: { angleDeg: LEAN.windy.stem }, crown: { angleDeg: LEAN.windy.crown } }, ground: "authored", contacts: [], thermal: bind(0.02, 0.45, 0.04, 0, 1), loads: [LOAD("wind-crown", F_WIND, "en-1991-1-4")] },
  { id: "storm", title: T("Gale", "Sturm"), description: T(`Beaufort 9, 22 m/s at 10 m (${f1(windAt(22, Z))} m/s at the ball), cd 0.6 since the outer shoots flatten: ${f1(F_STORM)} N; the ball leans 2.9 degrees in all and carries a base moment of about ${f1(F_STORM * Z)} N m.`, `Beaufort 9, 22 m/s in 10 m Höhe (${de1(windAt(22, Z))} m/s an der Kugel), cd 0,6, da sich die äußeren Triebe anlegen: ${de1(F_STORM)} N; die Kugel neigt sich insgesamt um 2,9 Grad und trägt ein Fußmoment von etwa ${de1(F_STORM * Z)} Nm.`), bones: { stem: { angleDeg: LEAN.storm.stem }, crown: { angleDeg: LEAN.storm.crown } }, ground: "authored", contacts: [], thermal: bind(0, 0.2, 0, 0.4, 0.5), loads: [LOAD("wind-crown", F_STORM, "vogel-1989")] },
  { id: "snow", title: T("Snow cap", "Schneehaube"), description: T(`Settled snow lies on the upper cap of the ball: 0.4 kN/m2 on the 0.28 m2 cap of 0.60 m gives ${SNOW.toFixed(0)} N (EN 1991-1-3 order of magnitude for about 15 cm of settled snow, authored estimate); wind is calm. Heavy wet snow can split a box ball open; that failure is not modelled.`, `Gesetzter Schnee liegt auf der oberen Kappe der Kugel: 0,4 kN/m2 auf der Kappe von 0,60 m und 0,28 m2 ergeben ${SNOW.toFixed(0)} N (Größenordnung nach EN 1991-1-3 für etwa 15 cm gesetzten Schnee, Schätzung); Windstille. Schwerer Nassschnee kann eine Buchskugel aufspalten; dieses Versagen wird nicht modelliert.`), bones: {}, ground: "authored", contacts: [], thermal: bind(0, 0, 0, 0.5, 0), loads: [{ id: "snow-crown", kind: "snow", title: T("Snow on the cap of the ball", "Schnee auf der Kugelkappe"), bone: "crown", x: 0.62, y: 0, fxN: 0, fyN: -SNOW, source: "authored-estimate", reference: "en-1991-1-3" }] },
];

const clips = [
  {
    id: "wind-sway", title: T("Wind sway", "Windschwingung"),
    description: T("A deterministic 0.8 s cycle (a stiff clipped shrub, estimated 1.25 Hz) about the fresh-breeze lean: the crown rocks by 0.35 degrees on the stems, which follow a quarter cycle later.", "Ein deterministischer Zyklus von 0,8 s (ein steifer geschnittener Strauch, geschätzt 1,25 Hz) um die Neigung der frischen Brise: die Krone wiegt sich um 0,35 Grad auf den Stämmen, die eine Viertelperiode später folgen."),
    durationS: 0.8, fps: 30, loop: true, rootMotion: "none",
    keys: swayKeys(0.8, 8, { stem: { base: LEAN.windy.stem, amp: 0.12, phase: -0.25 }, crown: { base: LEAN.windy.crown, amp: 0.35, phase: 0 } }),
    contacts: [], phases: [], events: [],
    tracks: ["transpiration-face", "transpiration-core"].map((e) => ({ target: `emitter:${e}`, ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 0.4, v: 0.92 }, { t: 0.8, v: 1 }] })),
  },
  {
    id: "gust", title: T("Gust", "Windböe"),
    description: T("A single 8 s gust: the wind-load track rises from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decays; the ball leans up to 1.9 degrees.", "Eine einzelne Böe über 8 s: Die Windlast-Spur steigt bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingt ab; die Kugel neigt sich bis zu 1,9 Grad."),
    durationS: 8, fps: 30, loop: false, rootMotion: "none",
    keys: GUST.t.map((t, i) => ({ t, pose: "windy", ease: "smooth", bones: { stem: { angleDeg: r4(-0.55 * GUST.lean[i]!) }, crown: { angleDeg: r4(-1.35 * GUST.lean[i]!) } } })),
    contacts: [], phases: [], events: [{ t: 2.5, kind: "load-on", title: T("Gust peak", "Böenspitze"), target: "wind-crown" }],
    tracks: [{ target: "load:wind-crown", ease: "smooth", keys: GUST.t.map((t, i) => ({ t, v: GUST.v[i]! })) }],
  },
  diurnalClip({ "stem-bark": 0.02, "leaf-face": 0.45, "leaf-core": 0.04 }, ["transpiration-face", "transpiration-core"]),
];

const anchorOf = (id: string, p: V3, en: string, de: string): Json => {
  const l = local("crown", p);
  return { id, bone: "crown", x: r4(l[0]), y: r4(l[1]), z: r4(l[2]), kind: "emitter", title: T(en, de) };
};

const asset: Json = {
  id: ID,
  category: "vegetation",
  rig,
  anchors: [
    { id: "base-support", bone: "root-plate", x: 0.3, y: 0, kind: "support", title: T("Root ball anchorage", "Verankerung des Wurzelballens"), support: "fixed", normalDeg: 90, footprintLengthM: 0.5, footprintWidthM: 0.5, frictionCoefficient: 0.6 },
    ...FACE.map((p, i) => anchorOf(`tp-face-${i + 1}`, p, `Transpiration of the clipped leaf skin, anchor ${i + 1}`, `Transpiration der geschnittenen Laubhaut, Anker ${i + 1}`)),
    ...COREP.map((p, i) => anchorOf(`tp-core-${i + 1}`, p, `Transpiration of the shaded inner leaves, anchor ${i + 1}`, `Transpiration der beschatteten inneren Blätter, Anker ${i + 1}`)),
  ],
  title: T("Boxwood Ball", "Buchskugel"),
  summary: T(
    `A clipped boxwood ball (Buxus sempervirens) planted in the ground, ${dims.widthM.toFixed(2)} m across and ${dims.heightM.toFixed(2)} m high: a dense evergreen sphere of small glossy leaves, its outline knobbly with the clipped shoot clusters; the underside clears the mulch by 7 cm, so five short, pale stems and the root flare show at large scale.`,
    `Eine geschnittene Buchskugel (Buxus sempervirens), im Boden gepflanzt, ${dims.widthM.toFixed(2).replace(".", ",")} m breit und ${dims.heightM.toFixed(2).replace(".", ",")} m hoch: eine dichte immergrüne Kugel aus kleinen glänzenden Blättern, ihr Umriss knollig von den geschnittenen Triebbüscheln; die Unterseite liegt 7 cm über dem Mulch, sodass im großen Maßstab fünf kurze, helle Stämme und der Wurzelanlauf zu sehen sind.`,
  ),
  tags: ["shrub", "evergreen", "broadleaf", "clipped", "topiary", "wind", "transpiration"],
  provenance: {
    author: "Semio staffage vegetation content worker PW-13",
    method: "handcrafted",
    created: "2026-11-22",
    references: REFERENCES,
    notes: T(
      "Buxus sempervirens: evergreen shrub, opposite elliptic leaves 1.5 to 3 cm, glossy dark green above (Stace 2019; Roloff and Bärtels 2018). Topiary balls of 0.6 to 1.0 m are standard nursery sizes; the dimensions here are an authored, typical clipped ball. The ball is stiff, so wind only rocks the whole crown on its short stems; the snow pose loads the upper cap. Masses: a clipped crown of about 100 kg per m3 of crown volume and dense green boxwood of about 1100 kg/m3, authored estimates.",
      "Buxus sempervirens: immergrüner Strauch, gegenständige elliptische Blätter von 1,5 bis 3 cm, oberseits glänzend dunkelgrün (Stace 2019; Roloff und Bärtels 2018). Formkugeln von 0,6 bis 1,0 m sind übliche Baumschulgrößen; die Maße hier sind eine typische geschnittene Kugel, vom Autor festgelegt. Die Kugel ist steif, daher wiegt der Wind nur die ganze Krone auf ihren kurzen Stämmen; die Schneepose belastet die obere Kappe. Massen: eine geschnittene Krone von etwa 100 kg je m3 Kronenvolumen und dichtes frisches Buchsholz von etwa 1100 kg/m3, Schätzungen.",
    ),
  },
  dimensions: dims,
  materials: [
    { id: "box-bark", title: T("Pale, finely fissured boxwood bark", "Helle, fein rissige Buchsrinde"), emissivity: 0.94, solarAbsorptivity: 0.55, densityKgM3: 800, conductivityWmK: 0.15, source: "authored-estimate", reference: "jones-2014" },
    { id: "box-leaf", title: T("Small glossy boxwood leaves in a clipped skin", "Kleine glänzende Buchsblätter in einer geschnittenen Haut"), emissivity: 0.97, solarAbsorptivity: 0.55, source: "literature", reference: "jones-2014" },
    { id: "box-wood", title: T("Green boxwood", "Frisches Buchsholz"), emissivity: 0.9, solarAbsorptivity: 0.55, densityKgM3: 1100, conductivityWmK: 0.2, elasticModulusPa: 10000000000, strengthPa: 70000000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "bark-mulch", title: T("Bark mulch around the ball", "Rindenmulch um die Kugel"), emissivity: 0.95, solarAbsorptivity: 0.85, densityKgM3: 250, conductivityWmK: 0.07, source: "authored-estimate", reference: "jones-2014" },
  ],
  poses,
  clips,
  thermal: {
    environment: ENVIRONMENT,
    regions: [
      { id: "stem-bark", title: T("Stems in the open foot", "Stämme im offenen Fuß"), kind: "bark", model: "passive", material: "box-bark", areaM2: STEM_AREA, characteristicLengthM: 0.032, geometry: "cylinder", skyViewFactor: 0.1, layers: [], heatCapacityJM2K: 21600, source: "authored-estimate", reference: "wood-handbook-2021" },
      { id: "leaf-face", title: T("Leaves of the clipped skin", "Blätter der geschnittenen Haut"), kind: "foliage", model: "leaf", material: "box-leaf", areaM2: 2.6, areaBasis: "exchange", characteristicLengthM: 0.02, geometry: "vertical-plate", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 1300, leaf: { stomatalConductanceMaxMS: 0.003, leafAreaIndex: r4(2.6 / AREA), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
      { id: "leaf-core", title: T("Shaded leaves inside the ball", "Beschattete Blätter im Kugelinneren"), kind: "foliage", model: "leaf", material: "box-leaf", areaM2: 1.9, areaBasis: "exchange", characteristicLengthM: 0.02, geometry: "vertical-plate", skyViewFactor: 0.05, layers: [], heatCapacityJM2K: 1300, leaf: { stomatalConductanceMaxMS: 0.0018, leafAreaIndex: r4(1.9 / AREA), photosynthesis: "c3" }, hidden: T("The shaded leaves lie inside the clipped skin and are never a body surface.", "Die beschatteten Blätter liegen innerhalb der geschnittenen Haut und sind nie eine Körperfläche."), source: "authored-estimate", reference: "jones-2014" },
    ],
    emitters: [
      { id: "transpiration-face", kind: "transpiration", title: T("Transpiration of the clipped skin (distributed over the ball)", "Transpiration der geschnittenen Haut (über die Kugel verteilt)"), anchor: "tp-face-1", anchors: FACE.map((_, i) => `tp-face-${i + 1}`), directionDeg: 0, diameterM: 0.25, region: "leaf-face", source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-core", kind: "transpiration", title: T("Transpiration of the shaded inner leaves (distributed over the ball)", "Transpiration der beschatteten inneren Blätter (über die Kugel verteilt)"), anchor: "tp-core-1", anchors: COREP.map((_, i) => `tp-core-${i + 1}`), directionDeg: 0, diameterM: 0.2, region: "leaf-core", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    assumptions: THERMAL_ASSUMPTIONS,
    operating: [],
    passengers: [],
  },
  structure: {
    gravityMS2: 9.80665,
    dynamics: "quasi-static",
    supports: ["base-support"],
    assumptions: [T("One rigid body per evaluated pose, fixed in the root ball; wind drag acts at the centre of the ball, snow on its cap, gravity on all bone masses. Dynamic amplification, the splitting of the ball under wet snow and root-soil failure are not modelled.", "Ein starrer Körper je ausgewerteter Pose, im Wurzelballen eingespannt; Winddruck wirkt in der Kugelmitte, Schnee auf ihrer Kappe, Schwerkraft auf allen Knochenmassen. Dynamische Überhöhung, das Aufspalten der Kugel unter Nassschnee und Wurzel-Boden-Versagen werden nicht modelliert.")],
    clinical: false,
  },
  volumes,
  presentation: { defaultPose: "calm", defaultClip: "wind-sway", defaultScale: 50 },
  exports: EXPORTS,
  completion: "authored",
  representations: { "500": drawing(500), "200": drawing(200), "100": drawing(100), "50": drawing(50) },
  symbols: [symbol()],
};

const path = save(asset, manifestRow(asset, T("A single clipped topiary sphere of boxwood planted in the ground: a stiff evergreen leaf skin on five short stems, the smallest clipped plant of the collection, whose snow pose loads the cap.", "Eine einzelne geschnittene Formkugel aus Buchs, im Boden gepflanzt: eine steife immergrüne Laubhaut auf fünf kurzen Stämmen, die kleinste Schnittpflanze der Sammlung; in der Schneepose lastet Schnee auf der Kappe.")));
console.log(`[DEBUG] wrote ${path}: ${JSON.stringify(dims)}`);
