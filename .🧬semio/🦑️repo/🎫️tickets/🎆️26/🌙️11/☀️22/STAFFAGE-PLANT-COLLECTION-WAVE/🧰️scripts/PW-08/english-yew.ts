/** 🌳️ English yew (Taxus baccata), PW-08: a 12 m old open-grown yew with a short, massive, fluted trunk, five ascending limbs and a broad, dense, lumpy dome of dark needles that sweeps down to a skirt. Every number below is chosen by hand from the cited references; the kit only turns them into meshes, drawings and the asset document.
 *
 * @see ./asset.ts
 * @see https://doi.org/10.1046/j.1365-2745.2003.00783.x — Thomas and Polwart 2003, Taxus baccata L., Biological Flora of the British Isles */
import { RAD, add, bezier, bounds, frames, lerp3, mul, r1, r2, r3, rng, rotateTo, sub, t, toLocal, tube, unit, len, volume, type BoneSpec, type Mesh, type P2, type Scale, type Station, type Text, type V3, type View } from "./lib.ts";
import { projectedArea, regionArea, type Ctx, type Group, type MarkFn, type Part, type Physics, type ScaleDesign, type Species } from "./asset.ts";
import { G, bend, binding, diurnalClip, foliageStats, gustClip, swayClip, windLoads } from "./common.ts";
import { fitExtent, lumpy, profile, stem, steps } from "./shared-geometry.ts";

const FEAT: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
const trunkR = profile([[0, 0.8], [0.12, 0.63], [0.35, 0.55], [1.3, 0.5], [2.0, 0.48], [2.5, 0.44]]);
const buttress = (th: number, y: number): number => 1 + 0.34 * Math.max(0, Math.cos(6 * th + 0.5)) ** 3 * Math.exp(-y / 0.2);
const fluted = (amp: number) => (th: number, y: number): number => buttress(th, y) * (1 + amp * Math.cos(6 * th + 0.5 + 0.15 * y) + 0.45 * amp * Math.cos(11 * th + 1.1));

/** 🪵️ The five main limbs: azimuth (degrees from ahead towards the left), elevation above the horizontal, length, base height and base diameter. */
const LIMBS = [
  { phi: 15, e: 35, L: 4.2, y: 2.15, d: 0.46 },
  { phi: 85, e: 50, L: 3.6, y: 2.3, d: 0.4 },
  { phi: 160, e: 30, L: 4.5, y: 1.95, d: 0.48 },
  { phi: 230, e: 45, L: 3.8, y: 2.25, d: 0.42 },
  { phi: 300, e: 40, L: 4.0, y: 2.1, d: 0.44 },
].map((l, i) => {
  const p = l.phi * RAD, e = l.e * RAD;
  const dir: V3 = [Math.cos(e) * Math.cos(p), Math.sin(e), Math.cos(e) * Math.sin(p)];
  const base: V3 = [0.12 * Math.cos(p), l.y, 0.12 * Math.sin(p)];
  const tip = add(base, mul(dir, l.L));
  const mid = add(lerp3(base, tip, 0.5), [0, 0.07 * l.L, 0]);
  return { ...l, i: i + 1, p, dir, base, tip, mid };
});

const random = rng(7101993);
const lobes = (n: number, ampMin: number, ampMax: number, seed: number) => {
  const r = rng(seed);
  return Array.from({ length: n }, () => {
    const th = r() * 2 * Math.PI, ph = Math.acos(1 - 1.6 * r());
    return { dir: [Math.sin(ph) * Math.cos(th), Math.cos(ph), Math.sin(ph) * Math.sin(th)] as V3, amp: ampMin + (ampMax - ampMin) * r(), width: 0.07 + 0.08 * r() };
  });
};

type Mass = { id: string; bone: string; parent: string; c: V3; radii: V3; region: "needles-sun" | "needles-shade"; origin: V3; angle: number; length: number; flat: number; seed: number };
const MASSES: Mass[] = [
  ...LIMBS.map((l): Mass => {
    const rc = 3.15 + 0.15 * Math.sin(l.p * 2);
    const c: V3 = [rc * Math.cos(l.p), 3.95 + 0.2 * Math.cos(l.p * 3), rc * Math.sin(l.p)];
    return { id: `mass-${l.i}-outer`, bone: `mass-${l.i}-outer`, parent: `limb-${l.i}`, c, radii: [2.15, 2.25, 2.15], region: "needles-shade", origin: l.tip, angle: r2((Math.atan2(c[1] - 1.2 - l.tip[1], c[0] * 1.2 - l.tip[0]) / RAD)), length: 1.8, flat: 0.5, seed: 100 + l.i };
  }),
  ...LIMBS.map((l): Mass => {
    const p = l.p + 10 * RAD;
    const c: V3 = [1.95 * Math.cos(p), 7.3 + 0.25 * Math.sin(l.p * 2), 1.95 * Math.sin(p)];
    return { id: `mass-${l.i}-upper`, bone: `mass-${l.i}-upper`, parent: `branch-${l.i}`, c, radii: [2.0, 1.95, 2.0], region: "needles-sun", origin: add(c, [0, -1.2, 0]), angle: 90, length: 1.6, flat: 0.2, seed: 200 + l.i };
  }),
  { id: "mass-core", bone: "mass-core", parent: "leader", c: [0, 6.0, 0], radii: [2.6, 2.4, 2.6], region: "needles-shade", origin: [0.048, 5.0, 0], angle: 90, length: 1.6, flat: 0.1, seed: 301 },
  { id: "mass-top", bone: "mass-top", parent: "leader", c: [0.2, 9.85, -0.1], radii: [2.35, 2.1, 2.35], region: "needles-sun", origin: [0.097, 7.9, 0], angle: 90, length: 2.0, flat: 0.1, seed: 302 },
];
const branchOf = (l: (typeof LIMBS)[number]) => {
  const from = lerp3(l.base, l.tip, 0.55);
  const to = add(MASSES.find((m) => m.id === `mass-${l.i}-upper`)!.c, [0, -0.6, 0]);
  return { from, to };
};

const sag = (o: V3, to: V3): number => r2(Math.atan2(to[1] - o[1], to[0] - o[0]) / RAD);
const BONES: BoneSpec[] = [
  { id: "root-plate", parent: null, title: t("Root plate and buttressed foot", "Wurzelteller und Wurzelanläufe"), o: [0, 0, 0], angle: 90, length: 0.35, joint: { type: "fixed" }, section: { d: 1.1, material: "yew-wood", title: t("Solid section, d = 110 cm", "Vollquerschnitt, d = 110 cm") } },
  { id: "trunk", parent: "root-plate", title: t("Fluted trunk", "Spannrückiger Stamm"), o: [0, 0.35, 0], angle: 90, length: 2.0, joint: { type: "revolute", minDeg: -1.5, maxDeg: 1.5 }, section: { d: 0.98, material: "yew-wood", title: t("Solid section, d = 98 cm", "Vollquerschnitt, d = 98 cm") } },
  { id: "leader", parent: "trunk", title: t("Central stem", "Mittelstamm"), o: [0, 2.2, 0], angle: 89, length: 5.7, joint: { type: "revolute", minDeg: -2.5, maxDeg: 2.5 }, section: { d: 0.34, material: "yew-wood", title: t("Solid section, d = 34 cm", "Vollquerschnitt, d = 34 cm") } },
  ...LIMBS.map((l): BoneSpec => ({ id: `limb-${l.i}`, parent: "trunk", title: t(`Main limb ${l.i}`, `Starkast ${l.i}`), o: l.base, angle: sag(l.base, l.tip), length: r3(Math.max(1.2, Math.hypot(l.tip[0] - l.base[0], l.tip[1] - l.base[1]))), joint: { type: "revolute", minDeg: -3, maxDeg: 3 }, section: { d: l.d, material: "yew-wood", title: t(`Solid section, d = ${Math.round(l.d * 100)} cm`, `Vollquerschnitt, d = ${Math.round(l.d * 100)} cm`) } })),
  ...LIMBS.map((l): BoneSpec => { const b = branchOf(l); return { id: `branch-${l.i}`, parent: `limb-${l.i}`, title: t(`Ascending branch of limb ${l.i}`, `Aufsteigender Ast von Starkast ${l.i}`), o: b.from, angle: sag(b.from, b.to), length: r3(Math.max(1, Math.hypot(b.to[0] - b.from[0], b.to[1] - b.from[1]))), joint: { type: "revolute", minDeg: -4, maxDeg: 4 }, section: { d: 0.17, material: "yew-wood", title: t("Solid section, d = 17 cm", "Vollquerschnitt, d = 17 cm") } }; }),
  ...MASSES.map((m): BoneSpec => ({ id: m.bone, parent: m.parent, title: m.id.endsWith("outer") ? t(`Outer needle mass of limb ${m.id.split("-")[1]} (skirt)`, `Äußere Nadelmasse von Starkast ${m.id.split("-")[1]} (Schleppe)`) : m.id.endsWith("upper") ? t(`Upper needle mass of limb ${m.id.split("-")[1]}`, `Obere Nadelmasse von Starkast ${m.id.split("-")[1]}`) : m.id === "mass-core" ? t("Inner needle mass round the central stem", "Innere Nadelmasse um den Mittelstamm") : t("Crown top", "Kronenkuppe"), o: m.origin, angle: m.angle, length: m.length, joint: { type: "revolute", minDeg: -5, maxDeg: 5 }, section: { d: 0.09, material: "yew-wood", title: t("Shoot bundle, d = 9 cm", "Triebbündel, d = 9 cm") } })),
];

const bark = (id: string, bone: string, region: "bark" | "limb-bark", mesh: Mesh): Part => ({ id, bone, region, material: "yew-bark", tone: region === "bark" ? 0.4 : 0.44, mesh, kind: "wood" });
const leaf = (id: string, bone: string, region: string, mesh: Mesh): Part => ({ id, bone, region, material: region === "needles-sun" ? "yew-needles-sun" : "yew-needles-shade", tone: region === "needles-sun" ? 0.42 : 0.34, mesh, kind: "foliage" });

/** 🪵️ A limb tube along its bowed axis with a taper to 45 % of the base, `n` points round, `k` stations. */
function limbTube(l: (typeof LIMBS)[number], n: number, k: number, from = 0): Mesh {
  const st: Station[] = steps(from, 1, 1 / k).map((u, i, a) => ({ c: bezier(l.base, l.mid, l.tip, u), r: i === a.length - 1 ? (l.d / 2) * 0.42 : (l.d / 2) * (1 - 0.55 * u) * (1 + (u < 0.12 ? 0.25 * (1 - u / 0.12) : 0)) }));
  return tube(st, n, [0, 1, 0]);
}
const branchTube = (l: (typeof LIMBS)[number], n: number): Mesh => { const b = branchOf(l); return tube([{ c: b.from, r: 0.09 }, { c: lerp3(b.from, b.to, 0.5), r: 0.07 }, { c: b.to, r: 0.05 }], n, [0, 1, 0]); };
const leaderTube = (n: number, top: number, apex: boolean): Mesh => tube(steps(2.2, top, 0.8).map((y, i, a) => ({ c: [0.0 + (y - 2.2) * 0.017, y, 0] as V3, r: apex && i === a.length - 1 ? 0 : 0.17 - (0.13 * (y - 2.2)) / (top - 2.2) })), n, [1, 0, 0]);
const massMesh = (m: Mass, nLat: number, nLon: number, nl: number, amp: number) => lumpy(m.c, m.radii, lobes(nl, amp * 0.6, amp, m.seed), m.flat, nLat, nLon);

/** ✒️ Needle marks: the flat, two-ranked shoots seen as short drooping double dashes. */
const needleMark = (size: number): MarkFn => (view: View, _s: Scale, q: P2, _g: Group, rnd: () => number) => {
  const s = size * (0.8 + 0.4 * rnd());
  const tilt = (rnd() - 0.5) * 0.5;
  const c = Math.cos(tilt), si = Math.sin(tilt);
  const pt = (x: number, y: number): P2 => [q[0] + c * x - si * y, q[1] + si * x + c * y];
  if (view === "plan") return [[pt(-0.5 * s, 0), pt(0, 0.18 * s), pt(0.5 * s, 0)]];
  return [[pt(-0.55 * s, -0.2 * s), pt(0, 0.12 * s), pt(0.55 * s, -0.22 * s)]];
};
/** 🪓️ Bark marks: the thin reddish-brown bark flaking off in vertical strips, drawn as short staggered strokes. */
const barkMark = (lenM: number): MarkFn => (view: View, _s: Scale, q: P2, _g: Group, rnd: () => number) => (view === "plan" ? [] : [[[q[0], q[1] - lenM / 2], [q[0] + 0.03 * (rnd() - 0.5), q[1] - lenM / 6], [q[0] - 0.02 * (rnd() - 0.5), q[1] + lenM / 6], [q[0], q[1] + lenM / 2]]]);
const isFoliage = (g: Group) => g.regions.some((r) => r.startsWith("needles"));
const isTrunk = (g: Group) => g.regions.length === 1 && g.regions[0] === "bark";

function members(scale: Scale): ScaleDesign["members"] {
  const out: ScaleDesign["members"] = [
    { bone: "root-plate", from: [0, 0, 0], to: [0, 0.45, 0], d0: 1.0, d1: 0.95, material: "yew-wood" },
    { bone: "trunk", from: [0, 0.35, 0], to: [0, 2.45, 0], d0: 0.95, d1: 0.85, material: "yew-wood" },
    { bone: "leader", from: [0, 2.2, 0], to: [0.095, 7.9, 0], d0: 0.32, d1: 0.09, material: "yew-wood" },
  ];
  for (const l of LIMBS) out.push({ bone: `limb-${l.i}`, from: l.base, to: l.tip, d0: l.d * 0.85, d1: l.d * 0.4, material: "yew-wood" });
  for (const l of LIMBS) { const b = branchOf(l); out.push({ bone: `branch-${l.i}`, from: b.from, to: b.to, d0: 0.16, d1: 0.09, material: "yew-wood", slender: scale === 500 }); }
  for (const m of MASSES) out.push({ bone: m.bone, from: m.origin, to: lerp3(m.origin, m.c, 0.9), d0: 0.09, d1: 0.05, material: "yew-wood", slender: true });
  return out;
}

const DEC = (en: string, de: string): Text => t(en, de);

export function species(): Species {
  const stemPieces = (n: number, amp: number, dy: number): Part[] => [
    bark("trunk-root-plate", "root-plate", "bark", stem(0, 0, steps(0, 0.47, 0.05), trunkR, fluted(amp), n, false)),
    bark("trunk-bole", "trunk", "bark", stem(0, 0, steps(0.35, 2.5, dy), trunkR, fluted(amp), n, false)),
  ];
  const parts50: Part[] = [
    ...stemPieces(30, 0.08, 0.15),
    bark("leader-stem", "leader", "limb-bark", leaderTube(14, 8.6, true)),
    ...LIMBS.map((l) => bark(`limb-${l.i}-wood`, `limb-${l.i}`, "limb-bark", limbTube(l, 16, 12))),
    ...LIMBS.map((l) => bark(`branch-${l.i}-wood`, `branch-${l.i}`, "limb-bark", branchTube(l, 10))),
    ...MASSES.map((m) => leaf(`needles-${m.id}`, m.bone, m.region, massMesh(m, 20, 30, 10, 0.16))),
  ];
  const env50 = bounds(parts50.map((p) => p.mesh));
  const parts100: Part[] = fitExtent([
    ...stemPieces(20, 0.06, 0.3),
    bark("leader-stem", "leader", "limb-bark", leaderTube(10, 8.4, true)),
    ...LIMBS.map((l) => bark(`limb-${l.i}-wood`, `limb-${l.i}`, "limb-bark", limbTube(l, 12, 8))),
    ...LIMBS.map((l) => bark(`branch-${l.i}-wood`, `branch-${l.i}`, "limb-bark", branchTube(l, 8))),
    ...MASSES.map((m) => leaf(`needles-${m.id}`, m.bone, m.region, massMesh(m, 15, 22, 8, 0.13))),
  ], env50);
  const parts200: Part[] = fitExtent([
    ...stemPieces(14, 0.04, 0.6),
    bark("leader-stem", "leader", "limb-bark", leaderTube(8, 8.2, false)),
    ...LIMBS.map((l) => bark(`limb-${l.i}-wood`, `limb-${l.i}`, "limb-bark", limbTube(l, 8, 5))),
    ...MASSES.map((m) => leaf(`needles-${m.id}`, m.bone, m.region, massMesh(m, 11, 16, 6, 0.1))),
  ], env50);
  const parts500: Part[] = fitExtent([
    bark("trunk-bole", "trunk", "bark", stem(0, 0, [0, 0.18, 0.5, 1.4, 2.5], trunkR, buttress, 10, false)),
    bark("leader-stem", "leader", "limb-bark", leaderTube(6, 8.0, false)),
    ...LIMBS.map((l) => bark(`limb-${l.i}-wood`, `limb-${l.i}`, "limb-bark", limbTube(l, 6, 3))),
    ...MASSES.map((m) => leaf(`needles-${m.id}`, m.bone, m.region, massMesh(m, 7, 11, 0, 0))),
  ], env50);

  const sunEmit = ["mass-top", "mass-1-upper", "mass-2-upper", "mass-3-upper", "mass-4-upper", "mass-5-upper"];
  const shadeEmit = ["mass-1-outer", "mass-2-outer", "mass-3-outer", "mass-4-outer", "mass-5-outer", "mass-core"];
  const anchorOf = (bone: string): V3 => {
    const m = MASSES.find((x) => x.bone === bone)!;
    const out = m.id === "mass-core" ? ([0, 0.9, 0] as V3) : mul(unit([m.c[0], 0.6, m.c[2]]), 0.8 * m.radii[0]);
    return add(m.c, out);
  };
  const emitterSolids = (scale: Scale, all: boolean): ScaleDesign["emitterSolids"] => {
    const pick = (ids: string[]) => (all ? ids : ids.slice(0, 1));
    const r = 0.14 + 2 * FEAT[scale];
    return [
      ...pick(sunEmit).map((b) => ({ emitter: "transpiration-sun", region: "needles-sun", bone: scale === 500 && b !== "mass-top" ? "mass-top" : b, at: anchorOf(b), radii: [r, r * 0.6, r] as V3, material: "yew-needles-sun" })),
      ...pick(shadeEmit).map((b) => ({ emitter: "transpiration-shade", region: "needles-shade", bone: b, at: anchorOf(b), radii: [r, r * 0.6, r] as V3, material: "yew-needles-shade" })),
    ];
  };
  const joints = (r: number) => [{ bone: "trunk", at: [0, 0.35, 0] as V3, r: r * 2 }, { bone: "leader", at: [0, 2.2, 0] as V3, r }, ...LIMBS.map((l) => ({ bone: `limb-${l.i}`, at: l.base, r }))];
  const masses = (r: number, bones: string[]) => bones.map((b) => { const bb = BONES.find((x) => x.id === b)!; return { bone: b, member: b, at: add(bb.o, [0, Math.min(bb.length / 2, 1.2), 0]) as V3, r, material: "yew-wood" }; });

  const commonDecisions = {
    hatching: DEC("No hatch: the very dark tone of the needle masses and the needle marks carry the foliage.", "Keine Schraffur: Der sehr dunkle Ton der Nadelmassen und die Nadelzeichen tragen das Laub."),
  };
  const scales: Record<Scale, ScaleDesign> = {
    50: {
      parts: parts50,
      pens: { body: 0.25, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 },
      marks: [
        { spacingM: 0.34, marginM: 0.06, groups: isFoliage, fn: needleMark(0.15), views: ["elevation", "front", "plan"] },
        { spacingM: 0.11, marginM: 0.04, groups: isTrunk, fn: barkMark(0.22), views: ["elevation", "front"] },
      ],
      decisions: {
        silhouette: DEC("At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: the short, massive trunk with its buttressed, fluted foot, the five ascending limbs and their branches where they show under the crown, and twelve lumpy needle masses (five low outer masses that form the skirt, five upper masses, the inner mass and the crown top), layered by their distance from the viewer, so the dome reads dense, irregular and dark with the hollow under it.", "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: der kurze, mächtige Stamm mit seinem spannrückigen, gerillten Fuß, die fünf aufsteigenden Starkäste und ihre Äste, wo sie unter der Krone sichtbar sind, und zwölf knollige Nadelmassen (fünf tiefe äußere Massen, die die Schleppe bilden, fünf obere Massen, die innere Masse und die Kronenkuppe), nach ihrem Abstand zum Betrachter geschichtet, sodass die Kuppel dicht, unregelmäßig und dunkel mit dem Hohlraum darunter lesbar ist."),
        retained: DEC("Retained: every needle mass with its lobes, the trunk flutes and buttresses, the limbs and the central stem where they show, drooping double dashes of the flat two-ranked shoots, the strip-flaking bark of the trunk.", "Beibehalten: jede Nadelmasse mit ihren Lappen, die Rillen und Wurzelanläufe des Stamms, die Starkäste und der Mittelstamm, wo sie sichtbar sind, hängende Doppelstriche der flachen, zweizeiligen Triebe, die in Streifen abblätternde Rinde des Stamms."),
        merged: DEC("Nothing is merged at this scale.", "Bei diesem Maßstab ist nichts zusammengefasst."),
        omitted: DEC("Omitted: single needles, the red arils and the thin twigs inside the masses.", "Weggelassen: einzelne Nadeln, die roten Samenmäntel und die dünnen Zweige im Inneren der Massen."),
        thinElements: DEC("The shoots that carry the needle masses are slender structural members only; every drawn wooden part is at least 0.1 m thick, 2 mm on paper.", "Die Triebe, die die Nadelmassen tragen, sind nur schlanke Tragwerksstäbe; jedes gezeichnete Holzteil ist mindestens 0,1 m dick, 2 mm auf dem Papier."),
        ...commonDecisions,
        styles: DEC("Line styles draw the masses with the body pen and the needle marks with the texture pen; solid styles fill the needle masses very dark (shade masses darkest), the reddish-brown wood mid-dark.", "Die Linienstile zeichnen die Massen mit der Körperfeder und die Nadelzeichen mit der Texturfeder; die Flächenstile füllen die Nadelmassen sehr dunkel (Schattenmassen am dunkelsten), das rotbraune Holz mitteldunkel."),
        overlays: DEC("Thermal: every needle mass carries its sun or shade needle region, trunk and limbs their bark regions; structure: trunk, central stem, limbs, branches and the slender shoots are the members.", "Thermik: Jede Nadelmasse trägt ihre Sonnen- oder Schattennadelregion, Stamm und Äste ihre Rindenregionen; Tragwerk: Stamm, Mittelstamm, Starkäste, Äste und die schlanken Triebe sind die Stäbe."),
      },
      volumeDecisions: [
        DEC("Body at 1:50 (smallest feature 7.5 mm): the root plate with six buttress roots and the fluted trunk (six ridges, 0.98 m at breast height), the central stem to 8.6 m, five limbs as bowed, tapered tubes from inside the trunk, five ascending branches into the upper masses, and twelve needle masses as closed lumpy ellipsoids with ten lobes each, flattened below, that overlap each other and the wood that carries them.", "Körper bei 1:50 (kleinstes Merkmal 7,5 mm): der Wurzelteller mit sechs Wurzelanläufen und der gerillte Stamm (sechs Rippen, 0,98 m in Brusthöhe), der Mittelstamm bis 8,6 m, fünf Starkäste als gebogene, verjüngte Röhren aus dem Stamminneren, fünf aufsteigende Äste in die oberen Massen und zwölf Nadelmassen als geschlossene, knollige Ellipsoide mit je zehn Lappen, unten abgeflacht, die einander und das tragende Holz überlappen."),
        DEC("Thermal at 1:50: trunk and root plate carry the bark region, stem, limbs and branches the limb bark, the five upper masses and the top the sun needles, the five skirt masses and the inner mass the shade needles, by reference to the body; twelve transpiration outlets on the masses that carry the emitter anchors.", "Thermik bei 1:50: Stamm und Wurzelteller tragen die Rindenregion, Mittelstamm, Starkäste und Äste die Astrinde, die fünf oberen Massen und die Kuppe die Sonnennadeln, die fünf Schleppenmassen und die innere Masse die Schattennadeln, als Verweis auf den Körper; zwölf Transpirationsauslässe auf den Massen, die die Emitteranker tragen."),
        DEC("Structure at 1:50: root plate, trunk, central stem, limbs and branches as tapered round members, the shoot of every needle mass as a slender member, the trunk, stem and limb joints, the fixed root plate and the masses of trunk, stem and limbs.", "Tragwerk bei 1:50: Wurzelteller, Stamm, Mittelstamm, Starkäste und Äste als verjüngte Rundstäbe, der Trieb jeder Nadelmasse als schlanker Stab, die Gelenke von Stamm, Mittelstamm und Starkästen, der eingespannte Wurzelteller und die Massen von Stamm, Mittelstamm und Starkästen."),
      ],
      members: members(50), joints: joints(0.22), masses: masses(0.16, ["trunk", "leader", "limb-1", "limb-2", "limb-3", "limb-4", "limb-5"]), supportR: 1.3, emitterSolids: emitterSolids(50, true),
    },
    100: {
      parts: parts100,
      pens: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 },
      marks: [{ spacingM: 0.7, marginM: 0.11, groups: isFoliage, fn: needleMark(0.28), views: ["elevation", "front"] }],
      decisions: {
        silhouette: DEC("At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: trunk with a gently fluted foot, limbs and branches where they show, and the twelve needle masses with fewer, softer lobes, layered by distance.", "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: Stamm mit leicht gerilltem Fuß, Starkäste und Äste, wo sie sichtbar sind, und die zwölf Nadelmassen mit weniger, weicheren Lappen, nach Abstand geschichtet."),
        retained: DEC("Retained: the twelve masses as separate outlines, trunk, stem and limbs, sparse needle dashes in the side and front elevations.", "Beibehalten: die zwölf Massen als getrennte Umrisse, Stamm, Mittelstamm und Starkäste, spärliche Nadelstriche in Seiten- und Vorderansicht."),
        merged: DEC("The finer lobes of every mass are merged into fewer, softer ones.", "Die feineren Lappen jeder Masse sind zu weniger, weicheren zusammengefasst."),
        omitted: DEC("Omitted: bark marks, the plan texture, arils and twigs.", "Weggelassen: Rindenzeichen, die Grundrisstextur, Samenmäntel und Zweige."),
        thinElements: DEC("Shoots are slender members only; branches are drawn at their true 0.1 to 0.18 m.", "Triebe sind nur schlanke Stäbe; Äste sind mit ihren wahren 0,1 bis 0,18 m gezeichnet."),
        ...commonDecisions,
        styles: DEC("As at 1:50 with thinner pens.", "Wie bei 1:50 mit dünneren Federn."),
        overlays: DEC("Thermal and structural overlays map onto the same outlines as at 1:50.", "Thermische und statische Überlagerungen liegen auf denselben Umrissen wie bei 1:50."),
      },
      volumeDecisions: [
        DEC("Body at 1:100 (smallest feature 15 mm): the same skeleton meshed more coarsely, trunk flutes reduced to a ripple, the twelve masses with eight softer lobes each.", "Körper bei 1:100 (kleinstes Merkmal 15 mm): dasselbe Gerüst gröber vernetzt, die Stammrillen auf eine leichte Welle reduziert, die zwölf Massen mit je acht weicheren Lappen."),
        DEC("Thermal at 1:100: the four regions by reference; twelve transpiration outlets.", "Thermik bei 1:100: die vier Regionen als Verweis; zwölf Transpirationsauslässe."),
        DEC("Structure at 1:100: as at 1:50, meshed more coarsely.", "Tragwerk bei 1:100: wie bei 1:50, gröber vernetzt."),
      ],
      members: members(100), joints: joints(0.24), masses: masses(0.18, ["trunk", "leader", "limb-1", "limb-2", "limb-3", "limb-4", "limb-5"]), supportR: 1.3, emitterSolids: emitterSolids(100, true),
    },
    200: {
      parts: parts200,
      pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
      decisions: {
        silhouette: DEC("At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: trunk, the five limbs where they show under the crown, and the twelve needle masses as smooth, gently lobed clouds that build the dome and its skirt.", "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: Stamm, die fünf Starkäste, wo sie unter der Krone sichtbar sind, und die zwölf Nadelmassen als glatte, leicht gelappte Wolken, die die Kuppel und ihre Schleppe bilden."),
        retained: DEC("Retained: trunk, central stem, limbs, the twelve masses.", "Beibehalten: Stamm, Mittelstamm, Starkäste, die zwölf Massen."),
        merged: DEC("Merged: the buttresses into a plain flared foot, the branches into the upper masses.", "Zusammengefasst: die Wurzelanläufe zu einem schlichten, ausgestellten Fuß, die Äste in die oberen Massen."),
        omitted: DEC("Omitted: every texture and the trunk flutes.", "Weggelassen: jede Textur und die Stammrillen."),
        thinElements: DEC("No thin element: the thinnest drawn part is a limb of 0.2 m, 1 mm on paper.", "Kein dünnes Element: Das dünnste gezeichnete Teil ist ein Starkast von 0,2 m, 1 mm auf dem Papier."),
        hatching: DEC("No hatch.", "Keine Schraffur."),
        styles: DEC("Body pen outlines; solid styles fill the masses very dark, the wood mid-dark.", "Umrisse mit der Körperfeder; die Flächenstile füllen die Massen sehr dunkel, das Holz mitteldunkel."),
        overlays: DEC("Thermal: each mass carries its needle region; structural overlays colour trunk, stem and limbs.", "Thermik: Jede Masse trägt ihre Nadelregion; statische Überlagerungen färben Stamm, Mittelstamm und Starkäste."),
      },
      volumeDecisions: [
        DEC("Body at 1:200 (smallest feature 30 mm): trunk and root plate with six soft ridges, the central stem, five limbs of eight-sided section and the twelve masses as smooth lobed ellipsoids (six lobes); the ascending branches vanish inside the upper masses.", "Körper bei 1:200 (kleinstes Merkmal 30 mm): Stamm und Wurzelteller mit sechs weichen Rippen, der Mittelstamm, fünf Starkäste mit achteckigem Querschnitt und die zwölf Massen als glatte, gelappte Ellipsoide (sechs Lappen); die aufsteigenden Äste verschwinden in den oberen Massen."),
        DEC("Thermal at 1:200: the four regions by reference; one transpiration outlet per emitter.", "Thermik bei 1:200: die vier Regionen als Verweis; ein Transpirationsauslass je Emitter."),
        DEC("Structure at 1:200: trunk, stem, limbs and branches as members, the shoots as slender members, the fixed root plate and the masses of trunk and stem.", "Tragwerk bei 1:200: Stamm, Mittelstamm, Starkäste und Äste als Stäbe, die Triebe als schlanke Stäbe, der eingespannte Wurzelteller und die Massen von Stamm und Mittelstamm."),
      ],
      members: members(200), joints: [], masses: masses(0.2, ["trunk", "leader"]), supportR: 1.3, emitterSolids: emitterSolids(200, false),
    },
    500: {
      parts: parts500,
      pens: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
      decisions: {
        silhouette: DEC("At 1:500 (1 m is 2 mm) the yew is a broad, dark dome 24 mm tall and 21 mm wide on a short, thick trunk: its twelve needle masses as smooth overlapping clouds, the five low ones forming the skirt; the drawings are the true views of the 1:500 body.", "Bei 1:500 (1 m sind 2 mm) ist die Eibe eine breite, dunkle Kuppel von 24 mm Höhe und 21 mm Breite auf einem kurzen, dicken Stamm: ihre zwölf Nadelmassen als glatte, überlappende Wolken, die fünf tiefen bilden die Schleppe; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers."),
        retained: DEC("Retained: the trunk and the twelve needle masses as separate outlines.", "Beibehalten: der Stamm und die zwölf Nadelmassen als getrennte Umrisse."),
        merged: DEC("Merged: the lobes of every mass into one smooth cloud of few points, the buttresses into a plain flared foot, root plate into the trunk.", "Zusammengefasst: die Lappen jeder Masse zu einer glatten Wolke mit wenigen Punkten, die Wurzelanläufe zu einem schlichten, ausgestellten Fuß, der Wurzelteller in den Stamm."),
        omitted: DEC("Omitted: the ascending branches and every texture; limbs and central stem are drawn but lie under the crown.", "Weggelassen: die aufsteigenden Äste und jede Textur; Starkäste und Mittelstamm sind gezeichnet, liegen aber unter der Krone."),
        thinElements: DEC("No thin element is visible: the limbs of 0.2 to 0.5 m lie inside the masses.", "Kein dünnes Element ist sichtbar: Die Starkäste von 0,2 bis 0,5 m liegen in den Massen."),
        hatching: DEC("No hatch.", "Keine Schraffur."),
        styles: DEC("One body pen; solid styles fill the shade masses darkest, the sunlit ones a little lighter.", "Eine Körperfeder; die Flächenstile füllen die Schattenmassen am dunkelsten, die besonnten etwas heller."),
        overlays: DEC("Thermal: every mass carries its needle region; the structural overlays colour trunk, stem and limbs.", "Thermik: Jede Masse trägt ihre Nadelregion; die statischen Überlagerungen färben Stamm, Mittelstamm und Starkäste."),
      },
      volumeDecisions: [
        DEC("Body at 1:500 (smallest feature 75 mm): trunk with flared foot, the central stem and the five limbs as six-sided tubes inside the crown, and the twelve needle masses as smooth ellipsoids of few faces without lobes, stretched to the true extent of the 1:50 body.", "Körper bei 1:500 (kleinstes Merkmal 75 mm): Stamm mit ausgestelltem Fuß, der Mittelstamm und die fünf Starkäste als sechseckige Röhren in der Krone sowie die zwölf Nadelmassen als glatte Ellipsoide mit wenigen Flächen ohne Lappen, auf die wahre Ausdehnung des 1:50-Körpers gestreckt."),
        DEC("Thermal at 1:500: bark, limb bark, shade and sun needles by reference; one transpiration outlet per emitter.", "Thermik bei 1:500: Rinde, Astrinde, Schatten- und Sonnennadeln als Verweis; ein Transpirationsauslass je Emitter."),
        DEC("Structure at 1:500: trunk, stem and limbs as members, branches and shoots as slender members, the fixed root plate and two masses.", "Tragwerk bei 1:500: Stamm, Mittelstamm und Starkäste als Stäbe, Äste und Triebe als schlanke Stäbe, der eingespannte Wurzelteller und zwei Massen."),
      ],
      members: members(500), joints: [], masses: masses(0.25, ["trunk", "leader"]), supportR: 1.3, emitterSolids: emitterSolids(500, false),
    },
  };
  void random;
  return {
    id: "english-yew",
    title: t("English Yew", "Gemeine Eibe"),
    summary: t(
      "An old, open-grown English yew (Taxus baccata) of 12 m: a short, massive, fluted trunk of reddish-brown, strip-flaking bark divides at about 2 m into five ascending limbs under a broad, very dense, lumpy dome of dark green needles, 10.7 m across, whose outer masses sweep down into a skirt and leave a dark hollow round the trunk.",
      "Eine alte, freistehende Gemeine Eibe (Taxus baccata) von 12 m: Ein kurzer, mächtiger, spannrückiger Stamm mit rotbrauner, in Streifen abblätternder Rinde teilt sich in etwa 2 m Höhe in fünf aufsteigende Starkäste unter einer breiten, sehr dichten, knolligen Kuppel aus dunkelgrünen Nadeln von 10,7 m Durchmesser, deren äußere Massen zu einer Schleppe herabschwingen und um den Stamm einen dunklen Hohlraum lassen.",
    ),
    tags: ["tree", "evergreen", "conifer", "broad-crown", "shade-tolerant", "wind", "transpiration"],
    references: [
      { id: "thomas-polwart-2003", citation: "Thomas, P. A. and Polwart, A. (2003). Taxus baccata L. Biological Flora of the British Isles 229. Journal of Ecology 91(3), 489-524 (habit, heights of 10 to 20 m, broad crowns of old trees, fluted trunks formed by fusing stems, bark, needle size and shade tolerance)." },
      { id: "farjon-2010", citation: "Farjon, A. (2010). A Handbook of the World's Conifers. Brill, Leiden. Taxus baccata: tree to 20 m or more, often multi-stemmed, crown broad and dense, bark reddish-brown, flaking in thin strips, needles 10 to 35 mm by 2 to 3 mm in two flat ranks." },
      { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5, green properties of dense softwoods, used as a guide for the authored yew values." },
      { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m." },
      { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of snow on a dense crown." },
      { id: "mayhead-1973", citation: "Mayhead, G. J. (1973). Some drag coefficients for British forest trees derived from wind tunnel studies. Agricultural Meteorology 12, 123-130." },
      { id: "moore-maguire-2004", citation: "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203." },
      { id: "coutts-1986", citation: "Coutts, M. P. (1986). Components of tree stability in Sitka spruce on peaty gley soil. Forestry 59(2), 173-197 (root-plate rotation under turning moment)." },
      { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance." },
      { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
    ],
    notes: t(
      "Trunk, limbs and needle masses are authored by hand after an old churchyard yew of several centuries: a short trunk fused from several stems, five limbs that rise at 30 to 50 degrees in different directions, and a dome of twelve dense masses down to a skirt at 1.8 m. Masses are volume times an authored green density of 950 kg/m3 for the heavy yew wood plus a crown bulk density of 5 kg per cubic metre of needle mass. An evergreen has no winter-bare geometry; the poses are calm, fresh breeze, gale and snow on the dome.",
      "Stamm, Starkäste und Nadelmassen sind von Hand nach einer alten, mehrere Jahrhunderte alten Kirchhofeibe gezeichnet: ein kurzer, aus mehreren Stämmen verwachsener Stamm, fünf Starkäste, die mit 30 bis 50 Grad in verschiedene Richtungen aufsteigen, und eine Kuppel aus zwölf dichten Massen bis zu einer Schleppe in 1,8 m Höhe. Massen sind Volumen mal einer geschätzten Frischdichte von 950 kg/m3 für das schwere Eibenholz plus eine Kronenrohdichte von 5 kg je Kubikmeter Nadelmasse. Ein Immergrüner hat keine winterkahle Geometrie; die Posen sind Ruhe, frische Brise, Sturm und Schnee auf der Kuppel.",
    ),
    materials: [
      { id: "yew-bark", title: t("Reddish-brown, strip-flaking yew bark", "Rotbraune, in Streifen abblätternde Eibenrinde"), emissivity: 0.95, solarAbsorptivity: 0.7, densityKgM3: 600, conductivityWmK: 0.14, source: "authored-estimate", reference: "jones-2014" },
      { id: "yew-needles-sun", title: t("Yew needles, sunlit upper masses", "Eibennadeln, besonnte obere Massen"), emissivity: 0.97, solarAbsorptivity: 0.6, source: "authored-estimate", reference: "jones-2014" },
      { id: "yew-needles-shade", title: t("Yew needles, shaded skirt and inner masses", "Eibennadeln, beschattete Schleppe und innere Masse"), emissivity: 0.97, solarAbsorptivity: 0.65, source: "authored-estimate", reference: "jones-2014" },
      { id: "yew-wood", title: t("Green yew wood", "Frisches Eibenholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 950, conductivityWmK: 0.18, elasticModulusPa: 7000000000, strengthPa: 60000000, source: "authored-estimate", reference: "wood-handbook-2021" },
    ],
    bones: BONES,
    scales,
    woodDensity: { "yew-bark": 950 },
    foliageDensity: { "yew-needles-sun": 5, "yew-needles-shade": 5 },
    hiddenWood: (bone: string) => (MASSES.some((m) => m.bone === bone) ? 950 * (Math.PI / 12) * 2.0 * (0.09 ** 2 + 0.09 * 0.05 + 0.05 ** 2) : 0),
    regions: [],
    emitters: [
      { id: "transpiration-sun", region: "needles-sun", title: t("Transpiration of the sunlit needles (distributed over the upper dome)", "Transpiration der besonnten Nadeln (über die obere Kuppel verteilt)"), anchors: sunEmit.map((b) => ({ bone: b, at: anchorOf(b) })), diameterM: 1.8, source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-shade", region: "needles-shade", title: t("Transpiration of the shaded needles (distributed over the skirt and the inner mass)", "Transpiration der beschatteten Nadeln (über die Schleppe und die innere Masse verteilt)"), anchors: shadeEmit.map((b) => ({ bone: b, at: anchorOf(b) })), diameterM: 1.8, source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    thermalEnvironment: { airTempC: 23, meanRadiantTempC: 25, relativeHumidity: 0.6, windMS: 2, windDirectionDeg: 0, solarWM2: 750, pressurePa: 101325, diffuseFraction: 0.18 },
    thermalAssumptions: [], structureAssumptions: [], poses: [], clips: [],
    symbol: {
      title: t("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
      description: t("A conventional sign of the yew for site and landscape plans: the outline of the true crown extent and the trunk section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Zeichen der Eibe für Lage- und Freiraumpläne: der Umriss der wahren Kronenausdehnung und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
      note: t("Model metres at true size: the outline is the extent of the crown in each view, the filled shape the trunk (plan: the trunk section at breast height).", "Modellmeter in wahrer Größe: Der Umriss ist die Ausdehnung der Krone in jeder Ansicht, die gefüllte Form der Stamm (Grundriss: der Stammquerschnitt in Brusthöhe)."),
    },
    defaultScale: 200,
    symbolStemD: 1.0,
    distinction: t("A 12 m old yew with a short, fluted trunk under a dark, lumpy dome 10.7 m wide that sweeps down to a skirt, the densest and stiffest evergreen crown of the archive.", "Eine 12 m hohe alte Eibe mit kurzem, spannrückigem Stamm unter einer dunklen, knolligen Kuppel von 10,7 m Breite, die zu einer Schleppe herabschwingt, die dichteste und steifste immergrüne Krone des Archivs."),
    massSource: { wood: "authored-estimate", foliage: "authored-estimate", reference: "wood-handbook-2021" },
  };
}

/** 🌡️ The physical scenario of the yew: regions with the bark areas of the 1:50 body and a leaf area index of about 7.5, poses with EN 1991-1-4 wind and EN 1991-1-3 snow on the dome, the bending they cause and the three clips. */
export function physics(sp: Species, _ctx: Ctx): Physics {
  const stats = foliageStats(sp, 0.1);
  const vol = (r: string) => sp.scales[50].parts.filter((p) => p.region === r).reduce((s, p) => s + volume(p.mesh), 0);
  const vSun = vol("needles-sun"), vShade = vol("needles-shade");
  const plan = projectedArea(sp.scales[50].parts.filter((p) => p.kind === "foliage").map((p) => p.mesh), "plan", 0.04);
  const leafTotal = r1(7.5 * plan);
  const aSun = r1((leafTotal * vSun) / (vSun + vShade)), aShade = r1(leafTotal - aSun);
  const regions = [
    { id: "bark", title: t("Trunk bark, reddish-brown, flaking in strips", "Stammrinde, rotbraun, in Streifen abblätternd"), kind: "bark", model: "passive", material: "yew-bark", areaM2: r2(regionArea(sp, "bark")), areaBasis: "total", characteristicLengthM: 1.0, geometry: "cylinder", skyViewFactor: 0.15, layers: [], heatCapacityJM2K: 80000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "limb-bark", title: t("Bark of the limbs, branches and central stem", "Rinde der Starkäste, Äste und des Mittelstamms"), kind: "bark", model: "passive", material: "yew-bark", areaM2: r2(regionArea(sp, "limb-bark")), areaBasis: "total", characteristicLengthM: 0.3, geometry: "cylinder", skyViewFactor: 0.1, layers: [], heatCapacityJM2K: 40000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "needles-sun", title: t("Needles of the sunlit upper dome", "Nadeln der besonnten oberen Kuppel"), kind: "foliage", model: "leaf", material: "yew-needles-sun", areaM2: aSun, areaBasis: "exchange", characteristicLengthM: 0.003, geometry: "horizontal-plate-up", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 1500, leaf: { stomatalConductanceMaxMS: 0.0018, leafAreaIndex: r2(aSun / plan), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    { id: "needles-shade", title: t("Needles of the shaded skirt and inner mass", "Nadeln der beschatteten Schleppe und der inneren Masse"), kind: "foliage", model: "leaf", material: "yew-needles-shade", areaM2: aShade, areaBasis: "exchange", characteristicLengthM: 0.003, geometry: "horizontal-plate-up", skyViewFactor: 0.25, layers: [], heatCapacityJM2K: 1500, leaf: { stomatalConductanceMaxMS: 0.0012, leafAreaIndex: r2(aShade / plan), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  ];
  const ttl = (bone: string, en: string, de: string): Text => { const b = sp.bones.find((x) => x.id === bone)!; return t(`${en} ${b.title.en.charAt(0).toLowerCase()}${b.title.en.slice(1)}`, `${de} ${b.title.de}`); };
  const cases = {
    calm: { id: "calm", v10: 1.5, cd: 0.6, source: "literature", reference: "en-1991-1-4" },
    windy: { id: "windy", v10: 8, cd: 0.5, source: "literature", reference: "en-1991-1-4" },
    storm: { id: "storm", v10: 22, cd: 0.36, source: "literature", reference: "mayhead-1973" },
  };
  const stemLoad = { bone: "trunk", toM: 1.5, d: 0.98, title: t("Wind drag on the trunk under the crown", "Winddruck auf den Stamm unter der Krone") };
  const E = 7.0e9, kRoot = 1.2e7;
  const limit = (b: BoneSpec) => (b.joint.type === "revolute" ? b.joint.maxDeg * 0.95 : 0);
  const wl = Object.fromEntries(Object.entries(cases).map(([k, c]) => [k, windLoads(sp, stats, c, stemLoad, (b) => ttl(b, "Wind drag on", "Winddruck auf"))]));
  const windyBones = bend(sp, wl.windy!.forces, E, kRoot, "trunk", limit);
  const stormBones = bend(sp, wl.storm!.forces, E, kRoot, "trunk", limit);
  const snowForces = stats.map((s) => ({ bone: s.bone, at: s.c, fx: 0, fy: -15 * G * s.planM2 }));
  const snowBones = bend(sp, snowForces, E * 0.5, kRoot, "trunk", limit);
  const fr = frames(sp.bones);
  const snowLoads = stats.map((s) => { const l = toLocal(fr[s.bone]!, s.c); return { id: `snow-${s.bone}`, kind: "snow", title: ttl(s.bone, "Snow on", "Schnee auf"), bone: s.bone, x: r3(l[0]), y: r3(l[1]), fxN: 0, fyN: r1(-15 * G * s.planM2), source: "authored-estimate", reference: "en-1991-1-3" }; });
  const totalF = (k: string) => r1(wl[k]!.forces.reduce((s, f) => s + f.fx, 0) / 1000);
  const baseM = (k: string) => r1(wl[k]!.forces.reduce((s, f) => s + f.fx * f.at[1], 0) / 1000);
  const de = (x: number) => String(x).replace(".", ",");
  const tr = (b: number, sun: number, shade: number, wet?: number) => ({
    bark: { exposedFraction: 1, sunlitFraction: b, ...(wet === undefined ? {} : { wettedness: wet * 0.5 }) },
    "limb-bark": { exposedFraction: 1, sunlitFraction: b * 0.5, ...(wet === undefined ? {} : { wettedness: wet * 0.5 }) },
    "needles-sun": { exposedFraction: 1, sunlitFraction: sun, ...(wet === undefined ? {} : { wettedness: wet }) },
    "needles-shade": { exposedFraction: 1, sunlitFraction: shade, ...(wet === undefined ? {} : { wettedness: wet }) },
  });
  const snowTotal = r1(snowForces.reduce((s, f) => s - f.fy, 0) / 1000);
  const poses = [
    { id: "calm", title: t("Calm summer day", "Windstiller Sommertag"), description: t(`Light air of 1.5 m/s at 10 m: the dome stands still in its own deep shade; cd 0.6 on the frontal area of the masses gives ${totalF("calm")} kN.`, `Leiser Zug von 1,5 m/s in 10 m Höhe: Die Kuppel steht still in ihrem eigenen tiefen Schatten; cd 0,6 auf die Anströmfläche der Massen ergibt ${de(totalF("calm"))} kN.`), bones: {}, ground: "authored", contacts: [], thermal: binding(tr(0.1, 0.8, 0.15), { "transpiration-sun": 0.8, "transpiration-shade": 0.8 }), loads: wl.calm!.loads },
    { id: "windy", title: t("Fresh breeze", "Frische Brise"), description: t(`Beaufort 5, 8 m/s at 10 m, cd 0.5 on the frontal area of every needle mass with an optical porosity of 10 %: ${totalF("windy")} kN on the crown and a base moment of about ${baseM("windy")} kN m; the stiff limbs and the massive trunk hardly move.`, `Beaufort 5, 8 m/s in 10 m Höhe, cd 0,5 auf die Anströmfläche jeder Nadelmasse bei 10 % optischer Porosität: ${de(totalF("windy"))} kN auf die Krone und ein Fußmoment von etwa ${de(baseM("windy"))} kNm; die steifen Starkäste und der mächtige Stamm bewegen sich kaum.`), bones: windyBones, ground: "authored", contacts: [], thermal: binding(tr(0.1, 0.8, 0.15), { "transpiration-sun": 1, "transpiration-shade": 1 }), loads: wl.windy!.loads },
    { id: "storm", title: t("Gale", "Sturm"), description: t(`Beaufort 9, 22 m/s at 10 m, cd 0.36 after the shoots streamline (Mayhead 1973): ${totalF("storm")} kN and a base moment of about ${baseM("storm")} kN m; the outer masses swing to the lee, rain wets the needles.`, `Beaufort 9, 22 m/s in 10 m Höhe, cd 0,36 nach dem Anlegen der Triebe (Mayhead 1973): ${de(totalF("storm"))} kN und ein Fußmoment von etwa ${de(baseM("storm"))} kNm; die äußeren Massen schwingen nach Lee, Regen nässt die Nadeln.`), bones: stormBones, ground: "authored", contacts: [], thermal: binding(tr(0, 0.3, 0.05, 0.4), { "transpiration-sun": 0.35, "transpiration-shade": 0.35 }), loads: wl.storm!.loads },
    { id: "snow", title: t("Snow on the dome", "Schnee auf der Kuppel"), description: t(`An evergreen dome catches snow: 15 kg/m2 of settled snow on the plan area of every mass (order of magnitude after EN 1991-1-3), ${snowTotal} kN in all, bows the limbs and lowers the skirt; no wind.`, `Eine immergrüne Kuppel fängt Schnee: 15 kg/m2 gesetzter Schnee auf der Grundrissfläche jeder Masse (Größenordnung nach EN 1991-1-3), insgesamt ${de(snowTotal)} kN, biegt die Starkäste und senkt die Schleppe; kein Wind.`), bones: snowBones, ground: "authored", contacts: [], thermal: binding(tr(0, 0, 0, 0.5), { "transpiration-sun": 0.05, "transpiration-shade": 0.05 }), loads: snowLoads },
  ];
  const loadIds = (wl.windy!.loads as { id: string }[]).map((l) => l.id);
  const stemBones = ["trunk", "leader", "limb-1", "limb-2", "limb-3", "limb-4", "limb-5"];
  const clips = [
    swayClip(sp, windyBones, 1.7, 0.6, (b) => (stemBones.includes(b.id) ? 0.02 : 0.15), t("Wind sway", "Windschwingung"), t("A deterministic 1.7 s sway cycle (a short, heavy, broad crown, about 0.6 Hz after Moore and Maguire 2004) about the fresh-breeze lean: the needle masses nod up to 0.2 degrees behind the stiff limbs.", "Ein deterministischer Schwingungszyklus von 1,7 s (eine kurze, schwere, breite Krone, etwa 0,6 Hz nach Moore und Maguire 2004) um die Neigung der frischen Brise: Die Nadelmassen nicken bis zu 0,2 Grad hinter den steifen Starkästen her."), 12),
    gustClip(windyBones, loadIds, t("Gust", "Windböe"), t("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak velocity pressure with Iv 0.25) and decay; the masses lean and recover.", "Eine einzelne Böe über 8 s: Die Windlastspuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzengeschwindigkeitsdruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Massen neigen sich und richten sich wieder auf.")),
    diurnalClip({ "transpiration-sun": [0.03, 1], "transpiration-shade": [0.03, 0.85] }, { bark: 0.1, "limb-bark": 0.05, "needles-sun": 0.8, "needles-shade": 0.15 }, t("Day of transpiration", "Transpirationstag"), t("A 24 h summer day compressed to 24 s (1 s = 1 h), calm weather: sunlit fractions and transpiration follow the sun, the stomata are nearly closed at night; leaf and bark temperatures come from the thermal solver.", "Ein 24-h-Sommertag auf 24 s gerafft (1 s = 1 h) bei ruhigem Wetter: Besonnte Anteile und Transpiration folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt- und Rindentemperaturen ergibt der thermische Löser.")),
  ];
  const thermalAssumptions = [
    t(`Needle regions use one-sided leaf area: a leaf area index of 7.5 over the crown ground projection of ${r1(plan)} m2 (a dense, shade-tolerant evergreen, authored estimate guided by Thomas and Polwart 2003), ${leafTotal} m2 in all, split between the sunlit upper and the shaded skirt and inner masses by their volume. The maximum stomatal conductance is a low authored estimate for the shade-adapted yew, guided by Jones (2014). Photosynthesis type: C3.`, `Nadelregionen verwenden die einseitige Blattfläche: ein Blattflächenindex von 7,5 über der Kronenprojektion am Boden von ${de(r1(plan))} m2 (ein dichter, schattenverträglicher Immergrüner, Autorenschätzung angelehnt an Thomas und Polwart 2003), insgesamt ${de(leafTotal)} m2, nach ihrem Volumen auf die besonnten oberen und die beschatteten Schleppen- und Innenmassen verteilt. Die maximale Spaltöffnungsleitfähigkeit ist eine niedrige Autorenschätzung für die schattenangepasste Eibe, angelehnt an Jones (2014). Photosynthesetyp: C3.`),
    t("Bark regions are passive; their areal heat capacity is the volumetric heat capacity of green wood times an effective depth of 1.5 to 3 cm, an authored estimate guided by the Wood Handbook; their areas are the outer surfaces of the 1:50 trunk and of the 1:50 limbs, branches and stem. No surface temperature is authored.", "Rindenregionen sind passiv; ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz mal einer wirksamen Tiefe von 1,5 bis 3 cm, eine Autorenschätzung angelehnt an das Wood Handbook; ihre Flächen sind die Außenflächen des 1:50-Stamms und der 1:50-Starkäste, -Äste und des Mittelstamms. Es werden keine Oberflächentemperaturen vorgegeben."),
    t("Transpiration emitters release the latent flux of their needle region as vapour, spread over six anchors each on the skin of the masses; the direction is the vertical in the frame of the first anchor's bone.", "Transpirationsemitter geben den latenten Strom ihrer Nadelregion als Dampf ab, verteilt auf je sechs Anker auf der Haut der Massen; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers."),
  ];
  const structureAssumptions = [
    t("One rigid body per evaluated pose, fixed at the root plate. Wind drag acts at the centroid of every needle mass and on the trunk under the crown as a line load; snow at the same centroids. Pose angles are quasi-static: each joint turns by M L / (2 E I) of its section (E 7.0 GPa, an authored value for green yew), a shoot bundle under snow with half of that stiffness, the root plate by the base moment over an anchorage stiffness of 12 MN m/rad (authored estimate after Coutts 1986). Dynamic amplification, the hollowing of old trunks and root-soil failure are not modelled.", "Ein starrer Körper je ausgewerteter Pose, am Wurzelteller eingespannt. Winddruck wirkt im Schwerpunkt jeder Nadelmasse und am Stamm unter der Krone als Linienlast; Schnee in denselben Schwerpunkten. Die Posenwinkel sind quasistatisch: Jedes Gelenk dreht sich um M L / (2 E I) seines Querschnitts (E 7,0 GPa, ein geschätzter Wert für frisches Eibenholz), ein Triebbündel unter Schnee mit der halben Steifigkeit, der Wurzelteller um das Fußmoment geteilt durch eine Verankerungssteifigkeit von 12 MNm/rad (Autorenschätzung nach Coutts 1986). Dynamische Überhöhung, das Hohlwerden alter Stämme und Wurzel-Boden-Versagen werden nicht modelliert."),
  ];
  return { regions, poses, clips, thermalAssumptions, structureAssumptions, supportFootprintM: 2.4 };
}
