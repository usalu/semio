/** 🌳️ Privet hedge (Ligustrum vulgare, clipped formal hedge 4.0 x 0.75 x 2.0 m, eight multi-stemmed plants): rig, masses, poses, clips, thermal and structure, the four drawings and the four volumes, every solid and shape stated by hand in world metres with the panel, tube and outline kit of the hornbeam hedge.
 * Usage: repo tools exec bun -- privet-hedge.ts
 * @see ./lib.ts
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-hornbeam-hedge-model.ts
 * @see ../../📝️PW-13.md */
import {
  ENVIRONMENT, EXPORTS, GUST, PENS, RANGES, REFERENCES, SHADE, T, THERMAL_ASSUMPTIONS, VIEWS, clipStrip, clipTube, de1, diurnalClip, drag, f1, feature, frames, manifestRow, marks, panelPolygons, panelSolid, privetMark, r4, r5, save, trueExtentNote,
  shapeOf, swayKeys, toLocal, tube, tubePolygons, tubeSolid, unite, windAt, type Json, type Look, type P2, type Panel, type Section, type Tube, type V3, type View,
} from "./lib.ts";

const ID = "privet-hedge";
const N = 8;
const K = Array.from({ length: N }, (_, i) => i + 1);
const XK = (k: number): number => -1.75 + 0.5 * (k - 1);
const LEAF: Look = { material: "privet-leaf", tone: 0.5 };
const CORE: Look = { material: "privet-leaf", tone: 0.4 };
const BARK: Look = { material: "privet-bark", tone: 0.35 };
const WOOD: Look = { material: "privet-wood", tone: 0.4 };
const DETAIL = (tone: number): Look => ({ ...WOOD, tone, role: "detail" });
const FOOT = 0.2;
const TOP = 2.0;

/** 🦴️ The planar rig: the row base along x (world -2.0 to 2.0), eight stem bundles 0.20 m high, eight clipped blocks to 2.0 m. */
const rig = {
  kind: "botanical",
  root: "root-plate",
  bones: [
    { id: "root-plate", parent: null, title: T("Hedge row base", "Heckenfuß"), x: -2, y: 0, angleDeg: 0, lengthM: 4, joint: { type: "fixed" }, mass: { kg: 140, comX: 2, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: 0.070686, secondMomentM4: 0.0003976, extremeFibreM: 0.15, material: "privet-wood", title: T("Root mat, solid section d = 30 cm", "Wurzelfilz, Vollquerschnitt d = 30 cm") } },
    ...K.map((k) => ({ id: `stem-${k}`, parent: "root-plate", title: T(`Stem bundle ${k}`, `Stammbündel ${k}`), x: r4(0.25 + 0.5 * (k - 1)), y: 0, angleDeg: 90, lengthM: FOOT, joint: { type: "revolute", minDeg: -3, maxDeg: 3 }, mass: { kg: 3, comX: 0.1, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" }, section: { areaM2: 0.001885, secondMomentM4: 2.827e-7, extremeFibreM: 0.014, material: "privet-wood", title: T("Three stems, d = 2.8 cm each", "Drei Stämme, je d = 2,8 cm") } })),
    ...K.map((k) => ({ id: `block-${k}`, parent: `stem-${k}`, title: T(`Clipped foliage block ${k}`, `Geschnittener Laubblock ${k}`), x: FOOT, y: 0, angleDeg: 0, lengthM: TOP - FOOT, joint: { type: "revolute", minDeg: -6, maxDeg: 6 }, mass: { kg: 40, comX: 0.85, comY: 0, distribution: "point", source: "authored-estimate", reference: "jones-2014" } })),
  ],
};
const MASS = 140 + N * 3 + N * 40;
const F = frames({ rig });
const local = (bone: string, p: V3): V3 => toLocal(F[bone]!, p).map(r5) as V3;

/** 📐️ The clipped cross-profile at 1:50 (height, half thickness across the row, rounding inset of the half length): open foot to 0.20 m, rounded underside, widest 0.75 m at 0.36 m, battered 1.3 degrees per face to 0.65 m at 1.80 m, a tight shoulder and the flat clipped top at 2.0 m. */
const PROFILE_50: [number, number, number][] = [
  [0.2, 0.3, 0.06], [0.225, 0.345, 0.03], [0.27, 0.367, 0.012], [0.36, 0.375, 0], [0.7, 0.367, 0], [1.1, 0.352, 0], [1.5, 0.337, 0], [1.8, 0.325, 0],
  [1.9, 0.32, 0.003], [1.95, 0.31, 0.01], [1.978, 0.292, 0.024], [1.993, 0.262, 0.045], [2.0, 0.225, 0.068],
];
const PROFILE_100: [number, number, number][] = [[0.2, 0.31, 0.05], [0.25, 0.362, 0.015], [0.36, 0.375, 0], [1.1, 0.352, 0], [1.8, 0.325, 0], [1.94, 0.312, 0.01], [1.985, 0.28, 0.035], [2.0, 0.23, 0.065]];
const PROFILE_200: [number, number, number][] = [[0.2, 0.33, 0.04], [0.3, 0.372, 0.008], [0.4, 0.375, 0], [1.85, 0.322, 0], [1.97, 0.3, 0.02], [2.0, 0.24, 0.06]];
const PROFILE_500: [number, number, number][] = [[0.2, 0.32, 0.05], [0.26, 0.365, 0.015], [0.36, 0.375, 0], [1.8, 0.325, 0], [1.95, 0.305, 0.02], [2.0, 0.24, 0.06]];
const HX = (k: number): number => (k === 1 || k === N ? 0.25 : 0.29);
const CX = (k: number): number => (k === 1 ? -1.75 : k === N ? 1.75 : XK(k));
const LIFT = (k: number): number => (k % 2 === 0 ? -0.004 : 0);
const panel = (id: string, bone: string, look: Look, rows: [number, number, number][], cx: number, hx: number, e: number, lift = 0): Panel => ({ id, bone, look, sections: rows.map(([y, hz, inset]): Section => ({ y: y > 1.9 ? r5(y + lift) : y, cx, cz: 0, hx: hx - inset, hz, e })) });
const panels = (rows: [number, number, number][], e: number) => K.map((k) => panel(`block-${k}`, `block-${k}`, LEAF, rows, CX(k), HX(k), e, LIFT(k)));
const CORE_ROWS: Record<number, [number, number, number][]> = { 50: [[0.32, 0.17, 0.08], [0.42, 0.235, 0.03], [0.9, 0.23, 0], [1.4, 0.215, 0], [1.72, 0.19, 0.02], [1.82, 0.12, 0.07]], 100: [[0.33, 0.18, 0.07], [0.45, 0.235, 0.02], [1.4, 0.215, 0], [1.8, 0.13, 0.06]], 200: [[0.34, 0.19, 0.06], [1.0, 0.23, 0], [1.78, 0.16, 0.05]] };
const cores = (scale: number) => K.map((k) => panel(`th-core-${k}`, `block-${k}`, CORE, CORE_ROWS[scale]!, CX(k), HX(k) - 0.02, 3));

/** 🪵️ The three stems of every plant at 1:50: base offset (x, z), offset at 0.45 m, radius at the collar and at 0.45 m; they continue 0.25 m into the foliage. */
const STEMS: [number, number, number, number, number, number][] = [[-0.03, -0.012, -0.1, -0.11, 0.0155, 0.011], [0.026, -0.02, 0.09, -0.06, 0.0145, 0.0105], [0.004, 0.028, 0.02, 0.13, 0.015, 0.0105]];
const SHIFT = [0, 0.006, -0.005, 0.004, -0.006, 0.003, -0.004, 0.005];
const stemTube = (k: number, s: number, ys: number[], id = `stem-${k}-${s + 1}`, look = BARK, shrink = 1): Tube => {
  const [bx, bz, tx, tz, r0, r1] = STEMS[s]!;
  const m = k % 2 === 0 ? -1 : 1;
  return tube(id, `stem-${k}`, look, ys.map((y) => {
    const f = (y / 0.45) ** 1.2;
    return [r4(XK(k) + SHIFT[k - 1]! + m * (bx + (tx - bx) * f)), y, r4(bz + (tz - bz) * f), r5((y === 0 ? r0 * 1.12 : r0 + ((r1 - r0) * y) / 0.45) * shrink)];
  }));
};
const ST50 = [0, 0.05, 0.12, 0.2, 0.3, 0.45];
const stems50 = K.flatMap((k) => [0, 1, 2].map((s) => stemTube(k, s, ST50)));
const stems100 = K.flatMap((k) => [0, 1].map((s) => stemTube(k, s, [0, 0.2, 0.4])));
const stems200 = K.map((k) => tube(`stem-${k}`, `stem-${k}`, BARK, [[XK(k), 0, 0, 0.026], [XK(k), 0.08, 0, 0.021], [XK(k), 0.4, 0, 0.02]]));
const legs500 = K.map((k) => tube(`stem-${k}`, `stem-${k}`, BARK, [[XK(k), 0, 0, 0.04], [XK(k), 0.3, 0, 0.04]]));
const MULCH: Look = { material: "bark-mulch", tone: 0.22, role: "detail" };
const mulch = (profile: [number, number][]): Json => ({ id: "mulch", kind: "extrude", bone: "root-plate", material: MULCH.material, role: "detail", tone: MULCH.tone, profile: profile.map(([v, w]) => [r5(v), r5(w)]), plane: "yz", from: 0.03, to: 3.97 });
const MULCH50: [number, number][] = [[0, -0.3], [0.01, -0.27], [0.019, -0.2], [0.024, -0.1], [0.026, 0], [0.024, 0.1], [0.019, 0.2], [0.01, 0.27], [0, 0.3]];
const MULCH100: [number, number][] = [[0, -0.3], [0.018, -0.22], [0.028, 0], [0.018, 0.22], [0, 0.3]];
const MULCH200: [number, number][] = [[0, -0.3], [0.03, -0.3], [0.03, 0.3], [0, 0.3]];

const ofRef = (id: string, of: string, region: string): Json => ({ id, of, region });
const member = (s: Json, m: string, part: string, extra: Json = {}): Json => ({ ...s, member: m, part, ...extra });
const ell = (id: string, bone: string, look: Look, c: V3, radii: V3): Json => ({ id, kind: "ellipsoid", bone, material: look.material, role: look.role ?? "body", tone: look.tone, radii: radii.map(r5), transform: { translation: local(bone, c) } });
const box = (id: string, bone: string, look: Look, c: V3, size: V3): Json => ({ id, kind: "box", bone, material: look.material, role: look.role ?? "body", tone: look.tone, size: size.map(r5), transform: { translation: local(bone, c) } });

/** 💨️ Transpiration anchors: on the clipped top of every block, the face outlet ahead of the plant centre, the core outlet behind it. */
const OUT = (k: number, which: 0 | 1): V3 => [r4(CX(k) + (which === 0 ? -0.1 : 0.1)), TOP + LIFT(k), which === 0 ? 0.08 : -0.08];
const lenses = (scale: number): Json[] => {
  const t = Math.max(0.008, feature(scale) / 2);
  return K.flatMap((k) => [
    { ...ell(`th-outlet-face-${k}`, `block-${k}`, { ...LEAF, role: "detail" }, OUT(k, 0), [0.1, t, 0.09]), region: "leaf-face", tags: ["transpiration-face"] },
    { ...ell(`th-outlet-core-${k}`, `block-${k}`, { ...CORE, role: "detail" }, OUT(k, 1), [0.08, t, 0.06]), region: "leaf-core", tags: ["transpiration-core"] },
  ]);
};

/** 🦴️ The woody framework of one plant: every stem continues as a leader to 1.85 m; laterals run out to the clipped faces and the top (height of origin, target across the row, along the row, height of the tip). */
const LATERALS: [number, number, number, number][] = [[0.55, 0.32, 0.05, 0.85], [0.95, -0.3, -0.06, 1.3], [1.35, 0.29, 0.07, 1.65], [1.62, -0.1, -0.05, 1.93]];
const leader = (k: number, s: number, id: string, ys: number[], rs: number[]): Tube => {
  const top = stemTube(k, s, [0.45]).points[0]!.p;
  return tube(id, `block-${k}`, WOOD, ys.map((y, i) => {
    if (y <= 0.45) {
      const p = stemTube(k, s, [y]).points[0]!.p;
      return [p[0], y, p[2], rs[i]!];
    }
    const f = (y - 0.45) / 1.4;
    return [r4(top[0] * (1 - 0.15 * f) + XK(k) * 0.15 * f), y, r4(top[2] * (1 - 0.4 * f)), rs[i]!];
  }));
};
const leaderAt = (k: number, s: number, y: number): V3 => leader(k, s, "x", [y], [0.01]).points[0]!.p;
function laterals(k: number, s: number, rows: [number, number, number, number][], r: number): Json[] {
  const sign = s === 1 ? -1 : 1;
  return rows.map(([y, zt, dx, yt], i) => {
    const a = leaderAt(k, s, y);
    const b: V3 = [r4(a[0] + dx * (k % 2 === 0 ? -1 : 1)), yt, r4(sign * zt)];
    const m: V3 = [r4((a[0] + b[0]) / 2), r4(a[1] + (yt - a[1]) * 0.42), r4(a[2] + (b[2] - a[2]) * 0.62)];
    return member(tubeSolid(tube(`lateral-${k}-${s + 1}-${i + 1}`, `block-${k}`, { ...WOOD, tone: 0.42, flags: { slender: true } }, [[...a, r], [...m, r], [...b, r * 0.85]]), F), `block-${k}`, "member");
  });
}
const rootAndSupport = (scale: number): Json[] => {
  const h = Math.max(0.016, feature(scale) * 1.05);
  return [
    member(box("member-root-plate", "root-plate", WOOD, [0, h / 2 + 0.001, 0], [3.9, h, 0.5]), "root-plate", "member"),
    member(box("support-base-support", "root-plate", DETAIL(0.25), [0, -h / 2, 0], [3.9, h, 0.6]), "root-plate", "support", { anchor: "base-support" }),
    member(ell("mass-root-plate", "root-plate", DETAIL(0.3), [0, h / 2, 0], [0.1, h / 2, 0.1]), "root-plate", "mass"),
  ];
};

function volume50(): Json {
  const ps = panels(PROFILE_50, 6);
  const body = [...ps.map((p) => panelSolid(p, F)), ...stems50.map((t) => tubeSolid(t, F)), mulch(MULCH50)];
  const thermal = [
    ...lenses(50),
    ...K.map((k) => ofRef(`th-block-${k}`, `block-${k}`, "leaf-face")),
    ...cores(50).map((p) => ({ ...panelSolid(p, F), region: "leaf-core" })),
    ...stems50.map((t) => ({ ...tubeSolid(clipTube({ ...t, id: `th-${t.id}` }, 0.3), F), region: "stem-bark" })),
  ];
  const structure = [
    ...rootAndSupport(50),
    ...K.flatMap((k) => [
      member(ell(`joint-root-${k}`, `stem-${k}`, DETAIL(0.35), stemTube(k, 0, [0.03]).points[0]!.p, [0.017, 0.017, 0.017]), `stem-${k}`, "joint"),
      ...[0, 1, 2].flatMap((s) => [
        member(tubeSolid(stemTube(k, s, [0, 0.1, 0.22], `member-stem-${k}-${s + 1}`, WOOD, 0.9), F), `stem-${k}`, "member"),
        member(tubeSolid(leader(k, s, `member-block-${k}-${s + 1}`, [0.2, 0.3, 0.45, 0.9, 1.4, 1.85], [0.013, 0.012, 0.0105, 0.009, 0.0075, 0.006]), F), `block-${k}`, "member"),
        ...laterals(k, s, s === 2 ? [LATERALS[1]!, LATERALS[3]!] : [LATERALS[0]!, LATERALS[2]!], 0.005),
      ]),
      member(ell(`joint-block-${k}`, `block-${k}`, DETAIL(0.35), stemTube(k, 0, [FOOT]).points[0]!.p, [0.014, 0.014, 0.014]), `block-${k}`, "joint"),
      member(ell(`mass-block-${k}`, `block-${k}`, DETAIL(0.3), [XK(k), 1.05, 0], [0.045, 0.045, 0.045]), `block-${k}`, "mass"),
    ]),
  ];
  return { status: "authored", denominator: 50, body, thermal, structure, decisions: VDEC[50] };
}

function volume100(): Json {
  const ps = panels(PROFILE_100, 6);
  const body = [...ps.map((p) => panelSolid(p, F)), ...stems100.map((t) => tubeSolid(t, F)), mulch(MULCH100)];
  const thermal = [
    ...lenses(100),
    ...K.map((k) => ofRef(`th-block-${k}`, `block-${k}`, "leaf-face")),
    ...cores(100).map((p) => ({ ...panelSolid(p, F), region: "leaf-core" })),
    ...stems100.map((t) => ({ ...tubeSolid(clipTube({ ...t, id: `th-${t.id}` }, 0.36), F), region: "stem-bark" })),
  ];
  const structure = [
    ...rootAndSupport(100),
    ...K.flatMap((k) => [
      ...[0, 1].flatMap((s) => [
        member(tubeSolid(stemTube(k, s, [0, 0.1, 0.22], `member-stem-${k}-${s + 1}`, WOOD), F), `stem-${k}`, "member"),
        member(tubeSolid(leader(k, s, `member-block-${k}-${s + 1}`, [0.2, 0.45, 1.2, 1.85], [0.012, 0.0105, 0.009, 0.0075]), F), `block-${k}`, "member"),
        ...laterals(k, s, [LATERALS[s === 0 ? 0 : 1]!], 0.0075),
      ]),
      member(ell(`joint-block-${k}`, `block-${k}`, DETAIL(0.35), stemTube(k, 0, [FOOT]).points[0]!.p, [0.014, 0.014, 0.014]), `block-${k}`, "joint"),
      member(ell(`mass-block-${k}`, `block-${k}`, DETAIL(0.3), [XK(k), 1.05, 0], [0.045, 0.045, 0.045]), `block-${k}`, "mass"),
    ]),
  ];
  return { status: "authored", denominator: 100, body, thermal, structure, decisions: VDEC[100] };
}

function volume200(): Json {
  const ps = panels(PROFILE_200, 5);
  const body = [...ps.map((p) => panelSolid(p, F)), ...stems200.map((t) => tubeSolid(t, F)), mulch(MULCH200)];
  const thermal = [
    ...lenses(200),
    ...K.map((k) => ofRef(`th-block-${k}`, `block-${k}`, "leaf-face")),
    ...cores(200).map((p) => ({ ...panelSolid(p, F), region: "leaf-core" })),
    ...K.flatMap((k) => [0, 1].map((s) => ({ ...tubeSolid(tube(`th-stem-${k}-${s + 1}`, `stem-${k}`, BARK, [[...stemTube(k, s, [0]).points[0]!.p, 0.02], [...stemTube(k, s, [0.34]).points[0]!.p, 0.02]]), F), region: "stem-bark" }))),
  ];
  const structure = [
    ...rootAndSupport(200),
    ...K.flatMap((k) => [
      member(tubeSolid(tube(`member-stem-${k}`, `stem-${k}`, WOOD, [[XK(k), 0, 0, 0.018], [XK(k), 0.22, 0, 0.016]]), F), `stem-${k}`, "member"),
      member(ell(`joint-block-${k}`, `block-${k}`, DETAIL(0.35), [XK(k), FOOT, 0], [0.02, 0.02, 0.02]), `block-${k}`, "joint"),
      member(tubeSolid(tube(`member-block-${k}`, `block-${k}`, WOOD, [[XK(k), 0.19, 0, 0.016], [XK(k), 1.85, 0, 0.016]]), F), `block-${k}`, "member"),
      member(ell(`mass-block-${k}`, `block-${k}`, DETAIL(0.3), [XK(k), 1.05, 0], [0.05, 0.05, 0.05]), `block-${k}`, "mass"),
    ]),
  ];
  return { status: "authored", denominator: 200, body, thermal, structure, decisions: VDEC[200] };
}

const HEDGE500 = panel("hedge", "root-plate", LEAF, PROFILE_500, 0, 2.0, 10);
function volume500(): Json {
  const body = [panelSolid(HEDGE500, F), ...legs500.map((t) => tubeSolid(t, F))];
  const thermal = [
    ...lenses(500),
    ofRef("th-hedge", "hedge", "leaf-face"),
    { ...box("th-core", "root-plate", CORE, [0, 1.08, 0], [3.7, 1.3, 0.42]), region: "leaf-core" },
    ...legs500.map((t) => ({ ...tubeSolid(tube(`th-${t.id}`, t.bone, BARK, [[t.points[0]!.p[0], 0, 0, 0.04], [t.points[0]!.p[0], 0.3, 0, 0.04]]), F), region: "stem-bark" })),
  ];
  const structure = [
    ...K.map((k) => member(box(`member-root-plate-${k}`, "root-plate", WOOD, [XK(k), 0.0375, 0], [0.08, 0.075, 0.08]), "root-plate", "member")),
    member(box("support-base-support", "root-plate", DETAIL(0.25), [0, -0.0375, 0], [3.9, 0.075, 0.6]), "root-plate", "support", { anchor: "base-support" }),
    ...K.flatMap((k) => [
      member(box(`member-stem-${k}`, `stem-${k}`, WOOD, [XK(k), 0.13, 0], [0.26, 0.08, 0.08]), `stem-${k}`, "member"),
      member(box(`member-block-${k}`, `block-${k}`, WOOD, [XK(k), 1.05, 0], [1.6, 0.08, 0.08]), `block-${k}`, "member"),
      member(ell(`mass-block-${k}`, `block-${k}`, DETAIL(0.3), [XK(k), 1.05, 0], [0.08, 0.08, 0.08]), `block-${k}`, "mass"),
    ]),
  ];
  return { status: "authored", denominator: 500, body, thermal, structure, decisions: VDEC[500] };
}

const VDEC: Record<number, Json[]> = {
  50: [
    T("Body: eight clipped plants as superelliptic panel lofts (exponent 6) on 13 hand-stated sections each: open foot to 0.20 m, rounded underside, widest 0.75 m at 0.36 m, faces battered 1.3 degrees to 0.65 m at 1.80 m, a tight shoulder and the flat top at 2.0 m; the panels overlap by 4 to 8 cm and lie in one clipped plane, neighbouring tops 4 mm apart; three slender stems per plant; the mulched planting strip.", "Körper: acht geschnittene Pflanzen als superelliptische Tafel-Lofts (Exponent 6) auf je 13 von Hand festgelegten Schnitten: offener Fuß bis 0,20 m, gerundete Unterseite, größte Dicke 0,75 m in 0,36 m Höhe, Flanken mit 1,3 Grad bis 0,65 m in 1,80 m Höhe geböscht, eine enge Schulter und die ebene Oberseite in 2,0 m; die Tafeln überlappen 4 bis 8 cm und liegen in einer Schnittebene, benachbarte Oberseiten 4 mm versetzt; drei schlanke Stämme je Pflanze; der gemulchte Pflanzstreifen."),
    T("Thermal: leaf-face is the clipped panels by reference; leaf-core (hidden) one inner loft per plant about 14 cm inside the faces; stem-bark the 24 stems up to 0.30 m; transpiration outlets as lenses on the top, tagged with their emitter.", "Thermisch: leaf-face sind die geschnittenen Tafeln per Verweis; leaf-core (verborgen) ein innerer Loft je Pflanze etwa 14 cm hinter den Flächen; stem-bark die 24 Stämme bis 0,30 m; Transpirationsauslässe als Linsen auf der Oberseite, mit ihrem Emitter markiert."),
    T("Structure: root mat and fixed planting-strip support, per plant a collar joint, three stem members each continuing as a leader to 1.85 m with two laterals to the faces and the top, a block joint at 0.20 m and the block mass.", "Tragwerk: Wurzelfilz und eingespanntes Auflager des Pflanzstreifens, je Pflanze ein Wurzelhalsgelenk, drei Stammglieder, die je als Leittrieb bis 1,85 m mit zwei Seitenästen zu den Flanken und zur Oberseite weiterlaufen, ein Blockgelenk in 0,20 m und die Blockmasse."),
  ],
  100: [
    T("Body modelled anew for 1:100: the same eight clipped plants on eight sections each, two stems per plant instead of three, the mulch strip as a five-point dome.", "Körper für 1:100 neu modelliert: dieselben acht geschnittenen Pflanzen auf je acht Schnitten, zwei statt drei Stämme je Pflanze, der Mulchstreifen als Kuppel aus fünf Punkten."),
    T("Thermal: leaf-face by reference, leaf-core one four-section inner loft per plant, stem-bark the 16 stems up to 0.30 m, outlet lenses.", "Thermisch: leaf-face per Verweis, leaf-core je Pflanze ein innerer Loft mit vier Schnitten, stem-bark die 16 Stämme bis 0,30 m, Auslasslinsen."),
    T("Structure: two stem members and two leaders per plant with one lateral each, block joints, root mat, support and point masses.", "Tragwerk: je Pflanze zwei Stammglieder und zwei Leittriebe mit je einem Seitenast, Blockgelenke, Wurzelfilz, Auflager und Punktmassen."),
  ],
  200: [
    T("Body modelled anew for 1:200: eight panels on six sections (shoulder and top merged into one rounded edge), one stem of 4 to 5 cm per plant for the bundle, the mulch as a 3 cm slab.", "Körper für 1:200 neu modelliert: acht Tafeln auf sechs Schnitten (Schulter und Oberseite zu einer gerundeten Kante vereint), ein Stamm von 4 bis 5 cm je Pflanze für das Bündel, der Mulch als Platte von 3 cm."),
    T("Thermal: leaf-face by reference, leaf-core three-section inner lofts, stem-bark two of the real stems per plant up to 0.34 m at 40 mm (the bark area does not merge with the drawn stem).", "Thermisch: leaf-face per Verweis, leaf-core innere Lofts mit drei Schnitten, stem-bark zwei der wirklichen Stämme je Pflanze bis 0,34 m mit 40 mm (die Rindenfläche wird nicht mit dem gezeichneten Stamm zusammengelegt)."),
    T("Structure: one stem and one leader member per plant with the block joint, root mat, support and point masses; laterals fall below the 30 mm feature.", "Tragwerk: je Pflanze ein Stamm- und ein Leittriebglied mit dem Blockgelenk, Wurzelfilz, Auflager und Punktmassen; Seitenäste liegen unter dem Merkmal von 30 mm."),
  ],
  500: [
    T("Body modelled anew for 1:500: the whole row is one clipped mass, a loft on six horizontal sections (exponent 10) with the battered faces and the flat top at 2.0 m; eight legs of 80 mm keep the 0.20 m open foot.", "Körper für 1:500 neu modelliert: die ganze Reihe ist eine geschnittene Masse, ein Loft auf sechs waagrechten Schnitten (Exponent 10) mit geböschten Flanken und der ebenen Oberseite in 2,0 m; acht Beine von 80 mm erhalten den offenen Fuß von 0,20 m."),
    T("Thermal: leaf-face the hedge mass by reference, leaf-core one inner box, stem-bark eight stems of 80 mm up to 0.30 m carrying the bark area of the bundles.", "Thermisch: leaf-face die Heckenmasse per Verweis, leaf-core eine innere Box, stem-bark acht Stämme von 80 mm bis 0,30 m mit der Rindenfläche der Bündel."),
    T("Structure: square bars of 80 mm for the eight stems and leaders, root balls under the stems, the support slab and the block masses.", "Tragwerk: Vierkantstäbe von 80 mm für die acht Stämme und Leittriebe, Wurzelballen unter den Stämmen, die Auflagerplatte und die Blockmassen."),
  ],
};

const volumes = { "50": volume50(), "100": volume100(), "200": volume200(), "500": volume500() };
const dims = { heightM: TOP, widthM: 4.0, depthM: 0.75, massKg: MASS };

/** ✏️ The drawings of one scale. */
const STEMSET: Record<number, Tube[]> = { 50: stems50, 100: stems100, 200: stems200, 500: legs500 };
const PROFILES: Record<number, [number, number, number][]> = { 50: PROFILE_50, 100: PROFILE_100, 200: PROFILE_200 };
const MULCHES: Record<number, [number, number][]> = { 50: MULCH50, 100: MULCH100, 200: MULCH200 };
/** 🧵️ Each plant is drawn up to 2 cm past its seam with the neighbour, so the drawing is one hedge with the plant seams and no crack. */
const strip = (k: number) => [k === 1 ? -9 : XK(k) - 0.27, k === N ? 9 : XK(k) + 0.27] as const;

function drawing(scale: number): Json {
  const g: Record<View, Json[]> = { elevation: [], front: [], plan: [] };
  const tol = (polys: P2[][]) => unite(polys, scale);
  for (const view of VIEWS) {
    const out = g[view];
    const stemShapes = () => K.forEach((k) => {
      const polys = STEMSET[scale]!.filter((t) => t.bone === `stem-${k}`).flatMap((t) => tubePolygons(view, clipTube(t, FOOT + 0.04)));
      out.push(shapeOf(view, F, { id: `stem-${k}`, bone: `stem-${k}`, role: "body", z: 1.9 + k / 100, filled: true, tone: 0.35, region: "stem-bark", material: "privet-bark" }, tol(polys)));
    });
    if (scale === 500) {
      const rings = tol(panelPolygons(view, HEDGE500));
      out.push(shapeOf(view, F, { id: "hedge", bone: "root-plate", role: "body", z: 2, filled: true, tone: 0.55, regions: ["leaf-face", "leaf-core"] }, rings));
      if (view !== "plan") stemShapes();
      continue;
    }
    const ps = panels(PROFILES[scale]!, scale === 200 ? 5 : 6);
    if (view !== "plan") {
      const m = MULCHES[scale]!;
      const ring: P2[] = view === "front" ? m.map(([v, w]) => ({ x: w, y: v })) : [{ x: -1.97, y: 0 }, { x: 1.97, y: 0 }, { x: 1.97, y: Math.max(...m.map((q) => q[0])) }, { x: -1.97, y: Math.max(...m.map((q) => q[0])) }];
      out.push(shapeOf(view, F, { id: "mulch", bone: "root-plate", role: "detail", z: 1, filled: true, tone: 0.25, material: "bark-mulch" }, [ring]));
      stemShapes();
    }
    const blockRings: Record<number, P2[][]> = {};
    for (const k of K) {
      const rings = tol(panelPolygons(view, ps[k - 1]!));
      const [lo, hi] = strip(k);
      blockRings[k] = view === "front" ? rings : rings.map((r) => clipStrip(r, lo, hi));
      out.push(shapeOf(view, F, { id: `block-${k}`, bone: `block-${k}`, role: "body", z: view === "front" ? 2 + k / 100 : 2.2, filled: true, tone: 0.55, region: "leaf-face", material: "privet-leaf" }, blockRings[k]!));
    }
    const coreKs = view === "front" ? [4] : K;
    if (view === "front" || scale === 200) for (const k of coreKs) {
      const c = cores(scale)[k - 1]!;
      const rings = tol(panelPolygons(view, c));
      const [lo, hi] = strip(k);
      out.push(shapeOf(view, F, { id: `core-${k}`, bone: `block-${k}`, role: "body", z: 3.1, filled: true, tone: 0.4, region: "leaf-core", ...(scale <= 100 ? { hatch: "shade-lines" } : {}) }, view === "front" ? rings : rings.map((r) => clipStrip(r, lo + 0.06, hi - 0.06))));
    }
    if (scale <= 100) for (const k of view === "front" ? [N] : K) {
      const ring = blockRings[k]![0]!;
      const pitch = scale === 50 ? 0.1 : 0.16, size = scale === 50 ? 0.055 : 0.085;
      const avoid = view === "front" ? [tol(panelPolygons(view, cores(scale)[3]!))[0]!] : [];
      const lines = marks(privetMark, ring, pitch, size, scale === 50 ? 0.03 : 0.04, k * 31 + scale + VIEWS.indexOf(view) * 7, avoid, view === "plan" ? 0.4 : 0.8);
      if (lines.length) out.push(shapeOf(view, F, { id: `leaf-marks-${k}`, bone: `block-${k}`, role: "texture", z: 4, filled: false, tone: 0.1 }, lines, false));
    }
  }
  return { status: "authored", denominator: scale, validRange: RANGES[scale], pens: PENS[scale], hatches: SHADE[scale], geometry: g, decisions: DEC[scale] };
}

const D = (silhouette: Json, retained: Json, merged: Json, omitted: Json, thinElements: Json, hatching: Json, styles: Json, overlays: Json) => ({ silhouette, retained, merged, omitted, thinElements, hatching, styles, overlays });
const DEC: Record<number, Json> = {
  50: D(
    T("At 1:50 (1 m is 20 mm) the privet hedge is drawn in full: eight clipped plants with the shallow plant seams, a crisp flat top and tight shoulders, faces battered slightly, pairs of narrow opposite leaves as texture, three slender stems per plant in the 0.20 m open foot and the mulched planting strip. Front and plan: the front shows the battered section (0.75 m at the base, 0.65 m at the shoulder) with the hatched shaded core; the plan shows the eight clipped tops with leaf marks.", "Bei 1:50 (1 m sind 20 mm) wird die Ligusterhecke vollständig gezeichnet: acht geschnittene Pflanzen mit den flachen Pflanzenfugen, eine scharfe ebene Oberseite und enge Schultern, leicht geböschte Flanken, Paare schmaler gegenständiger Blätter als Textur, drei schlanke Stämme je Pflanze im offenen Fuß von 0,20 m und der gemulchte Pflanzstreifen. Vorder- und Draufsicht: die Vorderansicht zeigt den geböschten Querschnitt (0,75 m am Fuß, 0,65 m an der Schulter) mit dem schraffierten Schattenkern; die Draufsicht zeigt die acht geschnittenen Oberseiten mit Blattzeichen."),
    T("Retained: every plant with its seam, every stem, the shoulder and the mulch strip.", "Beibehalten: jede Pflanze mit ihrer Fuge, jeder Stamm, die Schulter und der Mulchstreifen."),
    T("Merged: the three stems of a plant into one stem outline, so no ink gap between them falls below the print minimum.", "Zusammengefasst: die drei Stämme einer Pflanze zu einem Stammumriss, sodass keine Tintenlücke zwischen ihnen unter das Druckminimum fällt."),
    T("Omitted: single leaves (3 to 6 cm), flower panicles and berries, which no pose can switch.", "Weggelassen: einzelne Blätter (3 bis 6 cm), Blütenrispen und Beeren, die keine Pose schalten kann."),
    T("The stems (2.1 to 3.5 cm) are drawn at their true width; leaf marks are the finest strokes (0.13 mm).", "Die Stämme (2,1 bis 3,5 cm) werden in wahrer Breite gezeichnet; Blattzeichen sind die feinsten Striche (0,13 mm)."),
    T("A thin 45 degree line hatch marks the shaded core in the front.", "Eine feine 45-Grad-Schraffur kennzeichnet den Schattenkern in der Vorderansicht."),
    T("Light line: 0.25 mm outlines, 0.18 mm detail, 0.13 mm texture. Dark solid: tonal fills separate stems, sun foliage and shade foliage.", "Helle Linie: Umrisse 0,25 mm, Detail 0,18 mm, Textur 0,13 mm. Dunkle Fläche: getönte Füllungen trennen Stämme, Sonnenlaub und Schattenlaub."),
    T("Each thermal region has its own shapes: leaf-face the blocks, leaf-core the core in the front, stem-bark the stems; strokes carry no region and follow their block.", "Jede thermische Region hat eigene Formen: leaf-face die Blöcke, leaf-core der Kern in der Vorderansicht, stem-bark die Stämme; Striche tragen keine Region und folgen ihrem Block."),
  ),
  100: D(
    T("At 1:100 (1 m is 10 mm) the eight clipped plants keep the seams, the battered section and the shoulder on eight sections; fewer, larger leaf marks; two stems per plant as one outline over the mulch strip. Front and plan: the front keeps the hatched core; the plan the eight tops with leaf marks.", "Bei 1:100 (1 m sind 10 mm) behalten die acht geschnittenen Pflanzen Fugen, geböschten Querschnitt und Schulter auf acht Schnitten; weniger, größere Blattzeichen; zwei Stämme je Pflanze als ein Umriss über dem Mulchstreifen. Vorder- und Draufsicht: die Vorderansicht behält den schraffierten Kern; die Draufsicht die acht Oberseiten mit Blattzeichen."),
    T("Retained: the plant seams, the open foot with the stems and the mulch.", "Beibehalten: die Pflanzenfugen, der offene Fuß mit den Stämmen und der Mulch."),
    T("Merged: three stems per plant into two, drawn as one outline.", "Zusammengefasst: drei Stämme je Pflanze zu zweien, als ein Umriss gezeichnet."),
    T("Omitted: the finer shoulder rounding and every second leaf mark.", "Weggelassen: die feinere Schulterrundung und jedes zweite Blattzeichen."),
    T("Stems stay at their true width (0.2 to 0.3 mm); nothing is thickened.", "Die Stämme bleiben in wahrer Breite (0,2 bis 0,3 mm); nichts wird verstärkt."),
    T("The shaded core in the front keeps a 0.6 mm line hatch.", "Der Schattenkern in der Vorderansicht behält eine Linienschraffur von 0,6 mm."),
    T("Light line: 0.20 mm outlines, 0.13 mm detail, 0.10 mm texture. Dark solid: one dark band with light seams over a light foot.", "Helle Linie: Umrisse 0,20 mm, Detail 0,13 mm, Textur 0,10 mm. Dunkle Fläche: ein dunkles Band mit hellen Fugen über hellem Fuß."),
    T("Each thermal region keeps its own shapes: leaf-face, leaf-core (front) and stem-bark.", "Jede thermische Region behält eigene Formen: leaf-face, leaf-core (Vorderansicht) und stem-bark."),
  ),
  200: D(
    T("At 1:200 (1 m is 5 mm) the privet hedge is drawn as eight clipped plants with one flat top and battered faces, each with its shaded core outline in the plan, on eight single stems over the mulch strip. Front and plan: the front shows the section with the core over a stem; the plan the eight tops with their cores.", "Bei 1:200 (1 m sind 5 mm) wird die Ligusterhecke als acht geschnittene Pflanzen mit einer ebenen Oberseite und geböschten Flanken gezeichnet, jede mit dem Umriss ihres Schattenkerns in der Draufsicht, auf acht einzelnen Stämmen über dem Mulchstreifen. Vorder- und Draufsicht: die Vorderansicht zeigt den Querschnitt mit dem Kern über einem Stamm; die Draufsicht die acht Oberseiten mit ihren Kernen."),
    T("Retained: the open foot with one stem per plant and the mulch strip.", "Beibehalten: der offene Fuß mit einem Stamm je Pflanze und der Mulchstreifen."),
    T("Merged: the three stems of a plant into one outline 4 to 5 cm wide; shoulder and top into one rounded edge.", "Zusammengefasst: die drei Stämme einer Pflanze zu einem Umriss von 4 bis 5 cm Breite; Schulter und Oberseite zu einer gerundeten Kante."),
    T("Omitted: leaf marks and the hatch.", "Weggelassen: Blattzeichen und Schraffur."),
    T("The stems (4 to 5 cm) lie above the 30 mm feature; nothing is thickened.", "Die Stämme (4 bis 5 cm) liegen über dem Merkmal von 30 mm; nichts wird verstärkt."),
    T("No hatching at this scale.", "Bei diesem Maßstab keine Schraffur."),
    T("Light line: 0.18 mm outlines. Dark solid: the dark hedge band, cores one tone lighter.", "Helle Linie: Umrisse 0,18 mm. Dunkle Fläche: das dunkle Heckenband, die Kerne einen Ton heller."),
    T("Every thermal region keeps its own shapes: leaf-face the blocks, leaf-core the cores (front, plan), stem-bark the stems.", "Jede thermische Region behält eigene Formen: leaf-face die Blöcke, leaf-core die Kerne (Vorder-, Draufsicht), stem-bark die Stämme."),
  ),
  500: D(
    T("The privet hedge is composed as one clipped mass with a flat top, 4.0 m by 2.0 m, on eight legs; 1 m is 2 mm, so only the overall form and the open foot are drawn, with a restrained closed outline and no internal structure. Front and plan: the front shows the battered section on one leg; the plan one clipped band, 0.75 m deep.", "Die Ligusterhecke ist als eine geschnittene Masse mit ebener Oberseite, 4,0 m mal 2,0 m, auf acht Beinen komponiert; 1 m sind 2 mm, daher werden nur Gesamtform und offener Fuß mit zurückhaltendem geschlossenem Umriss und ohne innere Struktur gezeichnet. Vorder- und Draufsicht: die Vorderansicht zeigt den geböschten Querschnitt auf einem Bein; die Draufsicht ein geschnittenes Band, 0,75 m tief."),
    T("Retained: the open foot of the hedge, one leg per plant at the 75 mm feature.", "Beibehalten: der offene Fuß der Hecke, ein Bein je Pflanze in der Merkmalsgröße von 75 mm."),
    T("Merged: the eight blocks, face leaves and shaded core into one hedge path.", "Zusammengefasst: die acht Blöcke, Außenblätter und Schattenkern zu einem Heckenpfad."),
    T("Omitted: the plant seams, the second and third stems and the mulch strip.", "Weggelassen: die Pflanzenfugen, die zweiten und dritten Stämme und der Mulchstreifen."),
    T("The stem bundles are thickened to the 75 mm feature (80 mm), one leg per plant; nothing else is thickened.", "Die Stammbündel werden auf das Merkmal von 75 mm verstärkt (80 mm), ein Bein je Pflanze; sonst wird nichts verstärkt."),
    T("No hatching at this scale.", "Bei diesem Maßstab keine Schraffur."),
    T("Light line: closed outline only. Dark solid: one dark band on light legs.", "Helle Linie: nur geschlossener Umriss. Dunkle Fläche: ein dunkles Band auf hellen Beinen."),
    T("Thermal and structural overlays map onto stem-bark (the legs) and, merged in the hedge path, leaf-face and leaf-core with their area-weighted value, labelled as merged.", "Thermische und statische Überlagerungen bilden auf stem-bark (die Beine) und, im Heckenpfad zusammengefasst, auf leaf-face und leaf-core mit ihrem flächengewichteten Wert ab, als zusammengefasst beschriftet."),
  ),
};

function symbol(): Json {
  const base = { bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5 };
  const front = unite(panelPolygons("front", panel("s", "root-plate", LEAF, PROFILE_200, 0, 2, 5)), 200)[0]!;
  const frontFoot: P2[] = [...front.filter((p) => p.y > 0.25), { x: 0.3, y: 0.2 }, { x: 0.3, y: 0 }, { x: -0.3, y: 0 }, { x: -0.3, y: 0.2 }];
  const circle = (cx: number, r: number): P2[] => Array.from({ length: 16 }, (_, i) => ({ x: cx + r * Math.cos((2 * Math.PI * i) / 16), y: r * Math.sin((2 * Math.PI * i) / 16) }));
  return {
    id: `${ID}-symbol`,
    title: T("Plan symbol with true hedge extent", "Grundrisssymbol mit wahrer Heckenausdehnung"),
    description: T("A conventional plan sign of the clipped hedge for site and landscape plans: the true outline of the clipped band, its axis and the eight stem bundles. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen der geschnittenen Hecke für Lage- und Freiraumpläne: der wahre Umriss des geschnittenen Bandes, seine Achse und die acht Stammbündel. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
    semantics: "true-extent",
    units: "model-m",
    denominators: [500, 200, 100],
    pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
    hatches: [],
    dimensionalNote: trueExtentNote(dims, T("the outline is the clipped band, the line its axis and the circles the eight stem bundles at their real positions.", "Der Umriss ist das geschnittene Band, die Linie seine Achse und die Kreise die acht Stammbündel in wirklicher Lage.")),
    geometry: {
      elevation: [shapeOf("elevation", F, { id: "hedge", ...base }, [[{ x: -2, y: 0.2 }, { x: 2, y: 0.2 }, { x: 2, y: 1.95 }, { x: 1.95, y: 2 }, { x: -1.95, y: 2 }, { x: -2, y: 1.95 }]]), shapeOf("elevation", F, { id: "foot", ...base, role: "detail" }, [[{ x: -1.97, y: 0 }, { x: 1.97, y: 0 }, { x: 1.97, y: 0.2 }, { x: -1.97, y: 0.2 }]])],
      front: [shapeOf("front", F, { id: "hedge", ...base }, [frontFoot])],
      plan: [
        shapeOf("plan", F, { id: "hedge", ...base }, [[{ x: -2, y: -0.375 }, { x: 2, y: -0.375 }, { x: 2, y: 0.375 }, { x: -2, y: 0.375 }]]),
        { id: "axis", bone: "root-plate", role: "detail", z: 3, filled: false, tone: 0.4, kind: "path", d: "M -2 0 L 2 0" },
        ...K.map((k) => shapeOf("plan", F, { id: `stem-${k}`, bone: "root-plate", role: "body", z: 4, filled: true, tone: 0.25 }, [circle(XK(k), 0.03)])),
      ],
    },
  };
}

const bind = (stem: number, face: number, core: number, wet = 0, e = 0.8) => ({
  regions: { "stem-bark": { exposedFraction: 1, sunlitFraction: stem, ...(wet ? { wettedness: wet * 0.8 } : {}) }, "leaf-face": { exposedFraction: 1, sunlitFraction: face, ...(wet ? { wettedness: wet } : {}) }, "leaf-core": { exposedFraction: 1, sunlitFraction: core, ...(wet ? { wettedness: wet } : {}) } },
  emitters: { "transpiration-face": e, "transpiration-core": e },
});
const Z = 1.1, AREA = 0.5 * 1.8;
const wind = (v10: number, cd: number) => r4(drag(cd, AREA, windAt(v10, Z)));
const F_CALM = wind(1.5, 0.8), F_WIND = wind(8, 0.8), F_STORM = wind(22, 0.6);
const SNOW = r4(500 * 0.5 * 0.46);
const loads = (fx: number, ref: string) => K.map((k) => ({ id: `wind-block-${k}`, kind: "wind", title: T(`Wind drag on clipped foliage block ${k}`, `Winddruck auf den geschnittenen Laubblock ${k}`), bone: `block-${k}`, x: 0.9, y: 0, fxN: fx, fyN: 0, source: "literature", reference: ref }));
const lean = (stem: number, block: number) => Object.fromEntries(K.flatMap((k) => [[`stem-${k}`, { angleDeg: stem }], [`block-${k}`, { angleDeg: block }]]));
const LEAN = { windy: [-0.4, -1.4], storm: [-1.0, -3.4] } as const;

const poses = [
  { id: "calm", title: T("Calm summer day", "Windstiller Sommertag"), description: T(`Light air of 1.5 m/s at 10 m (${f1(windAt(1.5, Z))} m/s at the 1.1 m mid-height): the hedge stands upright as one straight clipped wall, drag ${F_CALM.toFixed(2)} N per block.`, `Leiser Zug von 1,5 m/s in 10 m Höhe (${de1(windAt(1.5, Z))} m/s in der Mittelhöhe von 1,1 m): die Hecke steht aufrecht als eine gerade geschnittene Wand, Widerstand ${F_CALM.toFixed(2).replace(".", ",")} N je Block.`), bones: {}, ground: "authored", contacts: [], thermal: bind(0.02, 0.5, 0.05), loads: loads(F_CALM, "en-1991-1-4") },
  { id: "windy", title: T("Fresh breeze", "Frische Brise"), description: T(`Beaufort 5, 8 m/s at 10 m, which is ${f1(windAt(8, Z))} m/s at the 1.1 m mid-height; cd 0.8 on 0.9 m2 of face per block gives ${f1(F_WIND)} N per block, ${f1(F_WIND * N)} N for the segment (the planar rig carries the face-normal wind in its plane, as for the hornbeam hedge). The hedge leans 1.8 degrees downwind.`, `Beaufort 5, 8 m/s in 10 m Höhe, das sind ${de1(windAt(8, Z))} m/s in der Mittelhöhe von 1,1 m; cd 0,8 auf 0,9 m2 Fläche je Block ergibt ${de1(F_WIND)} N je Block, ${de1(F_WIND * N)} N für den Abschnitt (das ebene Rig trägt den flächennormalen Wind in seiner Ebene, wie bei der Hainbuchenhecke). Die Hecke neigt sich um 1,8 Grad nach Lee.`), bones: lean(LEAN.windy[0], LEAN.windy[1]), ground: "authored", contacts: [], thermal: bind(0.02, 0.5, 0.05, 0, 1), loads: loads(F_WIND, "en-1991-1-4") },
  { id: "storm", title: T("Gale", "Sturm"), description: T(`Beaufort 9, 22 m/s at 10 m (${f1(windAt(22, Z))} m/s at hedge height), cd 0.6 after the leaves flatten: ${f1(F_STORM)} N per block, ${f1(F_STORM * N)} N in total; the blocks lean 4.4 degrees.`, `Beaufort 9, 22 m/s in 10 m Höhe (${de1(windAt(22, Z))} m/s in Heckenhöhe), cd 0,6 nach dem Anlegen der Blätter: ${de1(F_STORM)} N je Block, insgesamt ${de1(F_STORM * N)} N; die Blöcke neigen sich um 4,4 Grad.`), bones: lean(LEAN.storm[0], LEAN.storm[1]), ground: "authored", contacts: [], thermal: bind(0, 0.25, 0, 0.4, 0.5), loads: loads(F_STORM, "vogel-1989") },
  { id: "snow", title: T("Snow on the clipped top", "Schnee auf der geschnittenen Oberseite"), description: T(`Privet keeps many leaves into winter and its flat top collects snow: 0.5 kN/m2 on the 0.23 m2 top of each block gives ${SNOW.toFixed(0)} N per block (EN 1991-1-3 order of magnitude for about 15 cm of settled snow, authored estimate); wind is calm.`, `Liguster behält viele Blätter bis in den Winter, und seine ebene Oberseite sammelt Schnee: 0,5 kN/m2 auf der 0,23 m2 großen Oberseite jedes Blocks ergeben ${SNOW.toFixed(0)} N je Block (Größenordnung nach EN 1991-1-3 für etwa 15 cm gesetzten Schnee, Schätzung); Windstille.`), bones: {}, ground: "authored", contacts: [], thermal: bind(0, 0, 0, 0.5, 0), loads: K.map((k) => ({ id: `snow-block-${k}`, kind: "snow", title: T(`Snow on clipped foliage block ${k}`, `Schnee auf dem geschnittenen Laubblock ${k}`), bone: `block-${k}`, x: 1.8, y: 0, fxN: 0, fyN: -SNOW, source: "authored-estimate", reference: "en-1991-1-3" })) },
];

const clips = [
  {
    id: "wind-sway", title: T("Wind sway", "Windschwingung"),
    description: T("A deterministic 1.4 s cycle (stiff clipped panels, estimated 0.7 Hz) about the fresh-breeze lean: each block flutters by 0.9 degrees with a phase that travels along the row, so the clipped face ripples like a gust passing.", "Ein deterministischer Zyklus von 1,4 s (steife geschnittene Tafeln, geschätzt 0,7 Hz) um die Neigung der frischen Brise: jeder Block flattert um 0,9 Grad mit einer Phase, die entlang der Reihe wandert, sodass die geschnittene Fläche wie unter einer durchziehenden Böe wellt."),
    durationS: 1.4, fps: 30, loop: true, rootMotion: "none",
    keys: swayKeys(1.4, 7, Object.fromEntries(K.flatMap((k) => [[`stem-${k}`, { base: LEAN.windy[0], amp: 0.25, phase: -0.08 * k - 0.1 }], [`block-${k}`, { base: LEAN.windy[1], amp: 0.9, phase: -0.08 * k }]]))),
    contacts: [], phases: [], events: [],
    tracks: ["transpiration-face", "transpiration-core"].map((e) => ({ target: `emitter:${e}`, ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 0.7, v: 0.9 }, { t: 1.4, v: 1 }] })),
  },
  {
    id: "gust", title: T("Gust", "Windböe"),
    description: T("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the blocks lean up to 70 % of the gale lean.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Blöcke neigen sich bis zu 70 % der Sturmneigung."),
    durationS: 8, fps: 30, loop: false, rootMotion: "none",
    keys: GUST.t.map((t, i) => ({ t, pose: "windy", ease: "smooth", bones: lean(r4(0.7 * LEAN.storm[0] * GUST.lean[i]!), r4(0.7 * LEAN.storm[1] * GUST.lean[i]!)) })),
    contacts: [], phases: [], events: [{ t: 2.5, kind: "load-on", title: T("Gust peak", "Böenspitze") }],
    tracks: K.map((k) => ({ target: `load:wind-block-${k}`, ease: "smooth", keys: GUST.t.map((t, i) => ({ t, v: GUST.v[i]! })) })),
  },
  diurnalClip({ "stem-bark": 0.02, "leaf-face": 0.5, "leaf-core": 0.05 }, ["transpiration-face", "transpiration-core"]),
];

const anchorOf = (id: string, k: number, p: V3, en: string, de: string): Json => {
  const l = local(`block-${k}`, [p[0], p[1] + 0.004, p[2]]);
  return { id, bone: `block-${k}`, x: r4(l[0]), y: r4(l[1]), z: r4(l[2]), kind: "emitter", title: T(en, de) };
};

/** 🪵️ The outer bark area of the stems in the open foot, as measured on the 1:50 thermal solids (`check.ts --info`). */
const STEM_AREA = 0.64;
const PLAN = 4.0 * 0.75;

const asset: Json = {
  id: ID,
  category: "vegetation",
  rig,
  anchors: [
    { id: "base-support", bone: "root-plate", x: 2, y: 0, kind: "support", title: T("Hedge row anchorage (planting strip)", "Verankerung der Heckenreihe (Pflanzstreifen)"), support: "fixed", normalDeg: 90, footprintLengthM: 3.9, footprintWidthM: 0.6, frictionCoefficient: 0.6 },
    ...K.map((k) => anchorOf(`tp-face-${k}`, k, OUT(k, 0), `Transpiration of the hedge face, anchor ${k}`, `Transpiration der Heckenaußenseite, Anker ${k}`)),
    ...K.map((k) => anchorOf(`tp-core-${k}`, k, OUT(k, 1), `Transpiration of the shaded hedge core, anchor ${k}`, `Transpiration des beschatteten Heckenkerns, Anker ${k}`)),
  ],
  title: T("Privet Hedge", "Ligusterhecke"),
  summary: T(
    "A clipped hedge of wild privet (Ligustrum vulgare) 4.0 m long and 2.0 m high: eight plants clipped into one dense, flat-topped wall with tight shoulders, 0.75 m thick at the base and battered slightly to 0.65 m, leafy almost to the ground; only the lowest 0.20 m is open, showing three slender stems per plant over the mulched planting strip.",
    "Eine geschnittene Hecke aus Gewöhnlichem Liguster (Ligustrum vulgare), 4,0 m lang und 2,0 m hoch: acht Pflanzen zu einer dichten, flach abgeschlossenen Wand mit engen Schultern geschnitten, am Fuß 0,75 m dick und leicht auf 0,65 m geböscht, fast bis zum Boden belaubt; nur die untersten 0,20 m sind offen und zeigen drei schlanke Stämme je Pflanze über dem gemulchten Pflanzstreifen.",
  ),
  tags: ["hedge", "semi-evergreen", "broadleaf", "clipped", "wind", "transpiration"],
  provenance: {
    author: "Semio staffage vegetation content worker PW-13",
    method: "handcrafted",
    created: "2026-11-22",
    references: REFERENCES,
    notes: T(
      "Ligustrum vulgare: semi-evergreen shrub to 5 m, opposite lanceolate leaves 3 to 6 cm, white panicles in June and July, black berries in autumn (Stace 2019; Roloff and Bärtels 2018). As a clipped formal hedge it is planted at 3 to 4 plants per metre and kept at 1.5 to 2.5 m; here 8 multi-stemmed plants on 4 m (2 per m, 6 stems per m), clipped to 2.0 m by 0.75 m, an authored typical hedge. Flowers and berries cannot be toggled by a pose and are not drawn; the poses are calm, fresh breeze, gale and snow on the clipped top. The rig follows the hornbeam hedge: eight stiff panels on short stem bundles with a phase travelling along the row.",
      "Ligustrum vulgare: halbimmergrüner Strauch bis 5 m, gegenständige lanzettliche Blätter von 3 bis 6 cm, weiße Rispen im Juni und Juli, schwarze Beeren im Herbst (Stace 2019; Roloff und Bärtels 2018). Als geschnittene Formhecke wird er mit 3 bis 4 Pflanzen je Meter gepflanzt und auf 1,5 bis 2,5 m gehalten; hier 8 mehrstämmige Pflanzen auf 4 m (2 je m, 6 Stämme je m), auf 2,0 m mal 0,75 m geschnitten, eine typische Hecke nach Autorenwahl. Blüten und Beeren lassen sich durch eine Pose nicht schalten und werden nicht gezeichnet; die Posen sind Ruhe, frische Brise, Sturm und Schnee auf der geschnittenen Oberseite. Das Rig folgt der Hainbuchenhecke: acht steife Tafeln auf kurzen Stammbündeln mit einer entlang der Reihe wandernden Phase.",
    ),
  },
  dimensions: dims,
  materials: [
    { id: "privet-bark", title: T("Smooth grey-brown privet bark with lenticels", "Glatte graubraune Ligusterrinde mit Lentizellen"), emissivity: 0.94, solarAbsorptivity: 0.65, densityKgM3: 700, conductivityWmK: 0.13, source: "authored-estimate", reference: "jones-2014" },
    { id: "privet-leaf", title: T("Narrow privet leaves in a clipped hedge face", "Schmale Ligusterblätter in der geschnittenen Heckenfläche"), emissivity: 0.97, solarAbsorptivity: 0.52, source: "literature", reference: "jones-2014" },
    { id: "privet-wood", title: T("Green privet wood", "Frisches Ligusterholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 950, conductivityWmK: 0.17, elasticModulusPa: 8000000000, strengthPa: 55000000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "bark-mulch", title: T("Bark mulch on the planting strip", "Rindenmulch auf dem Pflanzstreifen"), emissivity: 0.95, solarAbsorptivity: 0.85, densityKgM3: 250, conductivityWmK: 0.07, source: "authored-estimate", reference: "jones-2014" },
  ],
  poses,
  clips,
  thermal: {
    environment: ENVIRONMENT,
    regions: [
      { id: "stem-bark", title: T("Stems in the open foot", "Stämme im offenen Fuß"), kind: "bark", model: "passive", material: "privet-bark", areaM2: STEM_AREA, characteristicLengthM: 0.028, geometry: "cylinder", skyViewFactor: 0.15, layers: [], heatCapacityJM2K: 18900, source: "authored-estimate", reference: "wood-handbook-2021" },
      { id: "leaf-face", title: T("Leaves of the outer hedge face", "Blätter der äußeren Heckenfläche"), kind: "foliage", model: "leaf", material: "privet-leaf", areaM2: 21, areaBasis: "exchange", characteristicLengthM: 0.025, geometry: "vertical-plate", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 1000, leaf: { stomatalConductanceMaxMS: 0.004, leafAreaIndex: r4(21 / PLAN), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
      { id: "leaf-core", title: T("Shaded leaves in the hedge core", "Beschattete Blätter im Heckenkern"), kind: "foliage", model: "leaf", material: "privet-leaf", areaM2: 17, areaBasis: "exchange", characteristicLengthM: 0.025, geometry: "vertical-plate", skyViewFactor: 0.1, layers: [], heatCapacityJM2K: 1000, leaf: { stomatalConductanceMaxMS: 0.0024, leafAreaIndex: r4(17 / PLAN), photosynthesis: "c3" }, hidden: T("The shaded core lies inside the clipped face of the hedge and is never a body surface.", "Der beschattete Kern liegt innerhalb der geschnittenen Heckenfläche und ist nie eine Körperfläche."), source: "authored-estimate", reference: "jones-2014" },
    ],
    emitters: [
      { id: "transpiration-face", kind: "transpiration", title: T("Transpiration of the hedge face (distributed over the row)", "Transpiration der Heckenaußenseite (über die Reihe verteilt)"), anchor: "tp-face-1", anchors: K.map((k) => `tp-face-${k}`), directionDeg: 0, diameterM: 0.5, region: "leaf-face", source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-core", kind: "transpiration", title: T("Transpiration of the shaded hedge core (distributed over the row)", "Transpiration des beschatteten Heckenkerns (über die Reihe verteilt)"), anchor: "tp-core-1", anchors: K.map((k) => `tp-core-${k}`), directionDeg: 0, diameterM: 0.4, region: "leaf-core", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    assumptions: THERMAL_ASSUMPTIONS,
    operating: [],
    passengers: [],
  },
  structure: {
    gravityMS2: 9.80665,
    dynamics: "quasi-static",
    supports: ["base-support"],
    assumptions: [T("One rigid body per evaluated pose, fixed at the planting strip; wind drag acts at the mid-height of each block, snow on the block tops, gravity on all bone masses. The planar rig carries the face-normal wind in its plane. Dynamic amplification and root-soil failure are not modelled.", "Ein starrer Körper je ausgewerteter Pose, am Pflanzstreifen eingespannt; Winddruck wirkt in der Mittelhöhe jedes Blocks, Schnee auf den Blockoberseiten, Schwerkraft auf allen Knochenmassen. Das ebene Rig trägt den flächennormalen Wind in seiner Ebene. Dynamische Überhöhung und Wurzel-Boden-Versagen werden nicht modelliert.")],
    clinical: false,
  },
  volumes,
  presentation: { defaultPose: "calm", defaultClip: "wind-sway", defaultScale: 50 },
  exports: EXPORTS,
  completion: "authored",
  representations: { "500": drawing(500), "200": drawing(200), "100": drawing(100), "50": drawing(50) },
  symbols: [symbol()],
};

const path = save(asset, manifestRow(asset, T("A clipped hedge of eight multi-stemmed privet plants, taller and squarer than the hornbeam hedge and leafy almost to the ground, whose snow pose loads the flat top.", "Eine geschnittene Hecke aus acht mehrstämmigen Ligusterpflanzen, höher und kantiger als die Hainbuchenhecke und fast bis zum Boden belaubt; in der Schneepose lastet Schnee auf der ebenen Oberseite.")));
console.log(`[DEBUG] wrote ${path}: ${JSON.stringify(dims)}`);
