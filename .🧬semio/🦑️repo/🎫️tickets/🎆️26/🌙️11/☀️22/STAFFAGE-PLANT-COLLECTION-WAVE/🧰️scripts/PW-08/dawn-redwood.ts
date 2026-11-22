/** 🌲️ Dawn redwood (Metasequoia glyptostroboides), PW-08: a 30 m deciduous conifer in summer leaf, a strongly tapered, buttressed and deeply fluted trunk with armpits under the branches, opposite branches in tiers and feathery sprays building a narrow cone. Every number below is chosen by hand from the cited references; the kit only turns them into meshes, drawings and the asset document.
 *
 * @see ./asset.ts
 * @see https://doi.org/10.1016/S0378-1127(02)00567-4 — Williams et al. 2003, Structure, allometry and biomass of plantation Metasequoia glyptostroboides in Japan */
import { RAD, add, bounds, frames, mul, r1, r2, r3, rotateTo, shift, t, toLocal, tube, volume, type BoneSpec, type Mesh, type P2, type Scale, type Station, type Text, type V3, type View } from "./lib.ts";
import { onePerPart, projectedArea, regionArea, type Ctx, type Group, type MarkFn, type Part, type Physics, type ScaleDesign, type Species } from "./asset.ts";
import { G, bend, binding, diurnalClip, foliageStats, gustClip, swayClip, windLoads } from "./common.ts";
import { fitExtent, lumpy, profile, sleeve, stem, steps } from "./shared-geometry.ts";

const H = 30;
const FEAT: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
const stemR = profile([[0, 1.0], [0.3, 0.72], [0.8, 0.55], [1.3, 0.47], [4, 0.41], [7, 0.35], [13, 0.26], [19, 0.17], [25, 0.09], [29.4, 0.025]]);
const crownR = (y: number): number => 4.55 * Math.pow(Math.max(0, (H - y) / 27.4), 0.92);

/** 🌿️ The tiers of opposite branches: height on the stem, azimuth of the first of four branches (degrees from ahead towards the left) and the elevation of the branch above the horizontal. */
const TIERS = Array.from({ length: 10 }, (_, i) => ({ y: 2.2 + i * 2.55, az: (i % 2 === 0 ? 6 : 50) + 5 * Math.sin(i * 2.1), e: 3 + i * 4.4 }));
type Branch = { id: string; tier: number; k: number; phi: number; e: number; y: number; L: number; dir: V3; base: V3; tip: V3; d: number; parent: string; region: "foliage-sun" | "foliage-shade" };
const jitter = Array.from({ length: 40 }, (_, i) => Math.round(7 * Math.sin(i * 2.399 + 0.7)));
const BRANCHES: Branch[] = TIERS.flatMap((tier, ti) => [0, 1, 2, 3].map((k) => {
  const phi = (tier.az + 90 * k + jitter[ti * 4 + k]!) * RAD;
  const e = tier.e * RAD;
  const L = crownR(tier.y) * (0.94 + 0.06 * Math.cos(k * 1.7 + ti)) - 0.2;
  const dir: V3 = [Math.cos(e) * Math.cos(phi), Math.sin(e), Math.cos(e) * Math.sin(phi)];
  const base: V3 = [0.06 * Math.cos(phi), tier.y, 0.06 * Math.sin(phi)];
  const parent = tier.y < 7 ? "trunk-1" : tier.y < 13 ? "trunk-2" : tier.y < 19 ? "trunk-3" : "trunk-4";
  return { id: `branch-${ti + 1}-${k + 1}`, tier: ti, k, phi, e, y: tier.y, L, dir, base, tip: add(base, mul(dir, L)), d: r3(0.24 - ti * 0.018), parent, region: ti < 3 ? "foliage-shade" : "foliage-sun" };
}));

/** 🪵️ Buttresses at the foot, seven deep flutes that fade up the stem and an armpit (a hollow) under every branch of the four lower tiers. */
function flutes(depth: number, armpits: boolean) {
  return (th: number, y: number): number => {
    let k = 1 + 0.42 * Math.max(0, Math.cos(5 * th + 0.7)) ** 2 * Math.exp(-y / 0.45);
    k *= 1 + depth * (0.16 * Math.exp(-y / 4) + 0.05) * (Math.cos(7 * th + 0.3 + 0.04 * y) + 0.45 * Math.cos(4 * th + 1.2));
    if (armpits) for (const b of BRANCHES) {
      if (b.tier > 3) continue;
      let dth = th - b.phi;
      dth = Math.atan2(Math.sin(dth), Math.cos(dth));
      k -= 0.11 * Math.exp(-(dth * dth) / 0.09 - ((y - (b.y - 0.55)) ** 2) / 0.25);
    }
    return k;
  };
}

const BONES: BoneSpec[] = [
  { id: "root-plate", parent: null, title: t("Root plate and buttressed foot", "Wurzelteller und Brettwurzelfuß"), o: [0, 0, 0], angle: 90, length: 0.5, joint: { type: "fixed" }, section: { d: 1.2, material: "metasequoia-wood", title: t("Solid section, d = 120 cm", "Vollquerschnitt, d = 120 cm") } },
  { id: "trunk-1", parent: "root-plate", title: t("Trunk, lower (deeply fluted)", "Stamm unten (tief gerillt)"), o: [0, 0.5, 0], angle: 90, length: 6.5, joint: { type: "revolute", minDeg: -2, maxDeg: 2 }, section: { d: 0.82, material: "metasequoia-wood", title: t("Solid section, d = 82 cm", "Vollquerschnitt, d = 82 cm") } },
  { id: "trunk-2", parent: "trunk-1", title: t("Trunk, middle", "Stamm Mitte"), o: [0, 7, 0], angle: 90, length: 6, joint: { type: "revolute", minDeg: -2.5, maxDeg: 2.5 }, section: { d: 0.6, material: "metasequoia-wood", title: t("Solid section, d = 60 cm", "Vollquerschnitt, d = 60 cm") } },
  { id: "trunk-3", parent: "trunk-2", title: t("Trunk, upper", "Stamm oben"), o: [0, 13, 0], angle: 90, length: 6, joint: { type: "revolute", minDeg: -3, maxDeg: 3 }, section: { d: 0.42, material: "metasequoia-wood", title: t("Solid section, d = 42 cm", "Vollquerschnitt, d = 42 cm") } },
  { id: "trunk-4", parent: "trunk-3", title: t("Trunk, top", "Stamm Spitze"), o: [0, 19, 0], angle: 90, length: 6, joint: { type: "revolute", minDeg: -3.5, maxDeg: 3.5 }, section: { d: 0.26, material: "metasequoia-wood", title: t("Solid section, d = 26 cm", "Vollquerschnitt, d = 26 cm") } },
  { id: "leader", parent: "trunk-4", title: t("Leader", "Gipfeltrieb"), o: [0, 25, 0], angle: 90, length: 4.4, joint: { type: "revolute", minDeg: -5, maxDeg: 5 }, section: { d: 0.12, material: "metasequoia-wood", title: t("Solid section, d = 12 cm", "Vollquerschnitt, d = 12 cm") } },
  ...BRANCHES.map((b): BoneSpec => {
    const side = Math.cos(b.phi) >= 0 ? t("ahead", "vorn") : t("behind", "hinten");
    const lat = Math.sin(b.phi) >= 0 ? t("left", "links") : t("right", "rechts");
    const proj = Math.hypot(b.tip[0] - b.base[0], b.tip[1] - b.base[1]);
    return { id: b.id, parent: b.parent, title: t(`Branch with its sprays, tier ${b.tier + 1}, ${side.en} ${lat.en}`, `Ast mit seinen Zweigen, Etage ${b.tier + 1}, ${side.de} ${lat.de}`), o: b.base, angle: r2(Math.atan2(b.tip[1] - b.base[1], b.tip[0] - b.base[0]) / RAD), length: r3(Math.max(0.8, proj)), joint: { type: "revolute", minDeg: -6, maxDeg: 6 }, section: { d: b.d, material: "metasequoia-wood", title: t(`Branch base, d = ${Math.round(b.d * 100)} cm`, `Astansatz, d = ${Math.round(b.d * 100)} cm`) } };
  }),
  { id: "top-spray", parent: "leader", title: t("Sprays of the leader", "Zweige des Gipfeltriebs"), o: [0, 25.2, 0], angle: 90, length: 4.6, joint: { type: "revolute", minDeg: -7, maxDeg: 7 }, section: { d: 0.08, material: "metasequoia-wood", title: t("Shoot, d = 8 cm", "Trieb, d = 8 cm") } },
];

const bark = (id: string, bone: string, region: "bark" | "branch-bark", mesh: Mesh): Part => ({ id, bone, region, material: "metasequoia-bark", tone: region === "bark" ? 0.34 : 0.4, mesh, kind: "wood" });
const leaf = (id: string, bone: string, region: string, mesh: Mesh): Part => ({ id, bone, region, material: region === "foliage-sun" ? "metasequoia-foliage-sun" : "metasequoia-foliage-shade", tone: region === "foliage-sun" ? 0.66 : 0.58, mesh, kind: "foliage" });

/** 🪵️ The wood of a branch: a tube from inside the stem, bowing down to the tip under its sprays. */
function branchWood(b: Branch, n: number, k: number): Mesh {
  const st: Station[] = steps(0, 1, 1 / k).map((u, i, a) => ({ c: add(add(b.base, mul(b.dir, u * b.L * 0.92)), [0, -0.12 * b.L * u * u, 0]), r: i === a.length - 1 ? 0.012 + b.d * 0.1 : (b.d / 2) * (1 - 0.8 * u) }));
  return tube(st, n, [0, 1, 0]);
}
/** 🍃️ The sprays of a branch as one flat, feathery mass along it: an elliptic section, broad across and thin vertically, that swells from the stem to its widest at two fifths and runs out into a drooping point past the branch tip; `ridge` ripples on its skin for the feathery edge. */
function sprayMass(b: Branch, nAround: number, k: number, ripples: number, ridge: number): Mesh {
  const len = b.L + 0.25;
  const st: Station[] = steps(0, 1, 1 / k).map((u, i, a) => {
    const c = add(add(b.base, mul(b.dir, u * len)), [0, -0.1 * b.L * u * u + 0.05 * b.L * Math.sin(Math.PI * u), 0]);
    if (i === a.length - 1) return { c, r: 0 };
    const g = Math.pow(Math.sin(Math.PI * (0.2 + 0.8 * Math.pow(u, 0.85))), 0.75);
    const h = (0.22 * b.L + 0.7) * g, w = (0.52 * b.L + 0.35) * g;
    return { c, r: (th: number) => ((h * w) / Math.hypot(w * Math.cos(th), h * Math.sin(th))) * (1 + ridge * Math.sin(ripples * th + 9 * u + b.k)) };
  });
  return tube(st, nAround, [0, 1, 0]);
}
/** 🗼️ The leader's sprays: a pointed, four-lobed cone from 25 m to the tip. */
function topSpray(n: number, dy: number, lobe: number): Mesh {
  const ys = steps(24.9, H, dy);
  return tube(ys.map((y, i): Station => {
    const c: V3 = [0, y, 0];
    if (i === 0 || i === ys.length - 1) return { c, r: 0 };
    const u = (y - 24.9) / (H - 24.9);
    const r = 1.15 * Math.pow(Math.sin(Math.PI * Math.min(1, 0.2 + 0.8 * u)), 0.6) * Math.pow(1 - u, 0.5);
    return { c, r: (th: number) => r * (1 + lobe * Math.cos(4 * th + 0.8)) };
  }), n, [1, 0, 0]);
}

/** ✒️ Feather marks: the opposite needles of a deciduous shoot as a short zigzag along the spray (horizontal in the elevations, radial in plan). */
const featherMark = (size: number): MarkFn => (view: View, _s: Scale, q: P2, _g: Group, rnd: () => number) => {
  const s = size * (0.85 + 0.3 * rnd());
  const a = view === "plan" ? Math.atan2(q[1], q[0]) : (rnd() - 0.5) * 0.4 - 0.12;
  const c = Math.cos(a), si = Math.sin(a);
  const pt = (x: number, y: number): P2 => [q[0] + c * x - si * y, q[1] + si * x + c * y];
  return [[pt(-0.5 * s, 0.1 * s), pt(-0.25 * s, -0.12 * s), pt(0, 0.1 * s), pt(0.25 * s, -0.12 * s), pt(0.5 * s, 0.08 * s)]];
};
/** 🪓️ Bark marks: the long fibrous strips of the reddish-brown bark. */
const stripMark = (lenM: number): MarkFn => (view: View, _s: Scale, q: P2, _g: Group, rnd: () => number) => (view === "plan" ? [] : [[[q[0], q[1] - lenM / 2], [q[0] + 0.02 * (rnd() - 0.5), q[1] - lenM / 6], [q[0] + 0.02 * (rnd() - 0.5), q[1] + lenM / 6], [q[0] + 0.01 * (rnd() - 0.5), q[1] + lenM / 2]]]);
const isFoliage = (g: Group) => g.regions.some((r) => r.startsWith("foliage"));
const isTrunk = (g: Group) => g.regions.length === 1 && g.regions[0] === "bark";

function members(scale: Scale): ScaleDesign["members"] {
  const out: ScaleDesign["members"] = [
    { bone: "root-plate", from: [0, 0, 0], to: [0, 0.6, 0], d0: 1.1, d1: 1.0, material: "metasequoia-wood" },
    { bone: "trunk-1", from: [0, 0.5, 0], to: [0, 7.1, 0], d0: 0.9, d1: 0.66, material: "metasequoia-wood" },
    { bone: "trunk-2", from: [0, 7, 0], to: [0, 13.1, 0], d0: 0.66, d1: 0.5, material: "metasequoia-wood" },
    { bone: "trunk-3", from: [0, 13, 0], to: [0, 19.1, 0], d0: 0.5, d1: 0.32, material: "metasequoia-wood" },
    { bone: "trunk-4", from: [0, 19, 0], to: [0, 25.1, 0], d0: 0.32, d1: 0.17, material: "metasequoia-wood" },
    { bone: "leader", from: [0, 25, 0], to: [0, 29.2, 0], d0: 0.17, d1: scale === 500 ? 0.08 : 0.05, material: "metasequoia-wood" },
    { bone: "top-spray", from: [0, 25.3, 0], to: [0, 29.6, 0], d0: 0.07, d1: 0.025, material: "metasequoia-wood", slender: true },
  ];
  for (const b of BRANCHES) out.push({ bone: b.id, from: b.base, to: add(b.base, mul(b.dir, b.L * 0.85)), d0: b.d, d1: Math.max(0.02, b.d * 0.22), material: "metasequoia-wood", slender: b.d < FEAT[scale] * 4 || scale >= 200 });
  return out;
}

const DEC = (en: string, de: string): Text => t(en, de);

export function species(): Species {
  const trunkPieces = (n: number, depth: number, dy: number, armpits: boolean): Part[] => [
    ["root-plate", 0, 0.62], ["trunk-1", 0.5, 7.12], ["trunk-2", 7, 13.12], ["trunk-3", 13, 19.12], ["trunk-4", 19, 25.12],
  ].map(([bone, a, b]) => bark(`stem-${bone}`, bone as string, "bark", stem(0, 0, steps(a as number, b as number, bone === "root-plate" ? 0.06 : dy), stemR, flutes(depth, armpits), n, false)))
    .concat([bark("stem-leader", "leader", "bark", stem(0, 0, steps(25, 29.4, dy), stemR, () => 1, Math.max(8, n / 2), true))]);
  const parts50: Part[] = [
    ...trunkPieces(36, 1, 0.25, true),
    ...BRANCHES.map((b) => bark(`wood-${b.id}`, b.id, "branch-bark", branchWood(b, 10, 8))),
    ...BRANCHES.map((b) => leaf(`sprays-${b.id}`, b.id, b.region, sprayMass(b, 22, 14, 7, 0.06))),
    leaf("sprays-top", "top-spray", "foliage-sun", topSpray(18, 0.2, 0.1)),
  ];
  const env50 = bounds(parts50.map((p) => p.mesh));
  const parts100: Part[] = fitExtent([
    ...trunkPieces(24, 0.75, 0.5, true),
    ...BRANCHES.map((b) => bark(`wood-${b.id}`, b.id, "branch-bark", branchWood(b, 8, 5))),
    ...BRANCHES.map((b) => leaf(`sprays-${b.id}`, b.id, b.region, sprayMass(b, 16, 9, 5, 0.04))),
    leaf("sprays-top", "top-spray", "foliage-sun", topSpray(14, 0.35, 0.07)),
  ], env50);
  const parts200: Part[] = fitExtent([
    ...trunkPieces(14, 0.45, 1.2, false),
    ...BRANCHES.map((b) => bark(`wood-${b.id}`, b.id, "branch-bark", branchWood(b, 6, 3))),
    ...BRANCHES.map((b) => leaf(`sprays-${b.id}`, b.id, b.region, sprayMass(b, 10, 6, 0, 0))),
    leaf("sprays-top", "top-spray", "foliage-sun", topSpray(10, 0.7, 0.04)),
  ], env50);
  const fine = BRANCHES.map((b) => sprayMass(b, 22, 14, 7, 0.06));
  const tierSleeve = (ti: number, extra: Mesh[] = []): Mesh => sleeve([...BRANCHES.filter((b) => b.tier === ti).map((b) => fine[BRANCHES.indexOf(b)]!), ...extra], 0, 0, 0.45, 12, 1, 0.3);
  const tierBone = (ti: number) => BRANCHES.find((b) => b.tier === ti)!.parent;
  const parts500: Part[] = fitExtent([
    bark("stem-trunk-1", "trunk-1", "bark", stem(0, 0, [0, 0.15, 0.45, 1.2, 4, 7, 13.1], stemR, flutes(0.25, false), 10, false)),
    bark("stem-trunk-3", "trunk-3", "bark", stem(0, 0, [13, 19, 25, 29.4], stemR, () => 1, 8, true)),
    ...BRANCHES.filter((b) => b.tier === 0).map((b) => bark(`wood-${b.id}`, b.id, "branch-bark", branchWood(b, 6, 2))),
    ...TIERS.map((_, ti) => leaf(`crown-tier-${ti + 1}`, tierBone(ti), ti < 3 ? "foliage-shade" : "foliage-sun", tierSleeve(ti, ti === TIERS.length - 1 ? [topSpray(18, 0.2, 0.1)] : []))),
  ], env50);

  const sunEmit = ["branch-4-1", "branch-5-2", "branch-6-3", "branch-7-4", "branch-8-1", "top-spray"];
  const shadeEmit = ["branch-1-1", "branch-1-3", "branch-2-2", "branch-2-4", "branch-3-1", "branch-3-3"];
  const anchorOf = (bone: string): V3 => (bone === "top-spray" ? [0.3, 27, 0] : (() => { const b = BRANCHES.find((x) => x.id === bone)!; return add(add(b.base, mul(b.dir, b.L * 0.6)), [0, 0.15 * b.L + 0.1, 0]); })());
  const emitterSolids = (scale: Scale, all: boolean): ScaleDesign["emitterSolids"] => {
    const pick = (ids: string[]) => (all ? ids : ids.slice(0, 1));
    const r = 0.14 + 2 * FEAT[scale];
    return [
      ...pick(sunEmit).map((b) => ({ emitter: "transpiration-sun", region: "foliage-sun", bone: b, at: anchorOf(b), radii: [r, r * 0.6, r] as V3, material: "metasequoia-foliage-sun" })),
      ...pick(shadeEmit).map((b) => ({ emitter: "transpiration-shade", region: "foliage-shade", bone: b, at: anchorOf(b), radii: [r, r * 0.6, r] as V3, material: "metasequoia-foliage-shade" })),
    ];
  };
  const stemBones = ["trunk-1", "trunk-2", "trunk-3", "trunk-4", "leader"];
  const joints = (r: number) => stemBones.map((b) => ({ bone: b, at: BONES.find((x) => x.id === b)!.o, r }));
  const masses = (r: number, which: string[]) => which.map((b) => { const bb = BONES.find((x) => x.id === b)!; return { bone: b, member: b, at: add(bb.o, [0, bb.length / 2, 0]) as V3, r, material: "metasequoia-wood" }; });

  const scales: Record<Scale, ScaleDesign> = {
    50: {
      parts: parts50,
      pens: { body: 0.25, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 },
      marks: [
        { spacingM: 0.36, marginM: 0.06, groups: isFoliage, fn: featherMark(0.2), views: ["elevation", "front", "plan"] },
        { spacingM: 0.13, marginM: 0.05, groups: isTrunk, fn: stripMark(0.42), views: ["elevation", "front"] },
      ],
      decisions: {
        silhouette: DEC("At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: the buttressed foot and the strongly tapered, deeply fluted trunk with a hollow under every lower branch, forty branches in ten tiers of opposite pairs, each carrying one flat, feathery spray mass that reaches in to the stem, and the pointed leader sprays; layered by distance, the tiers build a narrow cone with the stem showing between them.", "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: der Brettwurzelfuß und der stark abholzige, tief gerillte Stamm mit einer Mulde unter jedem unteren Ast, vierzig Äste in zehn Etagen gegenständiger Paare, jeder mit einer flachen, gefiederten Zweigmasse, die bis an den Stamm reicht, und die spitzen Zweige des Gipfeltriebs; nach Abstand geschichtet bauen die Etagen einen schmalen Kegel, zwischen dem der Stamm sichtbar ist."),
        retained: DEC("Retained: every spray mass as its own outline, the branch wood where it shows under the sprays, the flutes, buttresses and armpits of the trunk, zigzag marks of the opposite needles, the long fibrous bark strips.", "Beibehalten: jede Zweigmasse als eigener Umriss, das Astholz, wo es unter den Zweigen sichtbar ist, Rillen, Brettwurzeln und Achselmulden des Stamms, Zickzackzeichen der gegenständigen Nadeln, die langen faserigen Rindenstreifen."),
        merged: DEC("Nothing is merged at this scale.", "Bei diesem Maßstab ist nichts zusammengefasst."),
        omitted: DEC("Omitted: single needles and shoots, the cones and the deciduous side shoots that fall with the needles.", "Weggelassen: einzelne Nadeln und Triebe, die Zapfen und die abfallenden Kurztriebe, die mit den Nadeln fallen."),
        thinElements: DEC("Branch tips thinner than the 7.5 mm feature end inside their spray masses.", "Astspitzen, die dünner als das Merkmal von 7,5 mm sind, enden in ihren Zweigmassen."),
        hatching: DEC("No hatch: tones and the needle marks carry the foliage, the strip marks the bark.", "Keine Schraffur: Töne und Nadelzeichen tragen das Laub, die Streifenzeichen die Rinde."),
        styles: DEC("Line styles draw outlines with the body pen and the marks with the texture pen; solid styles fill the light summer foliage mid-grey (lower shaded tiers a little darker), the reddish-brown trunk dark.", "Die Linienstile zeichnen die Umrisse mit der Körperfeder und die Zeichen mit der Texturfeder; die Flächenstile füllen das helle Sommerlaub mittelgrau (die unteren beschatteten Etagen etwas dunkler), den rotbraunen Stamm dunkel."),
        overlays: DEC("Thermal: every spray mass carries its sun or shade foliage region, the trunk its bark, the branches their branch bark; structure: trunk pieces and branches are the members.", "Thermik: Jede Zweigmasse trägt ihre Sonnen- oder Schattenlaubregion, der Stamm seine Rinde, die Äste ihre Astrinde; Tragwerk: Stammstücke und Äste sind die Stäbe."),
      },
      volumeDecisions: [
        DEC("Body at 1:50 (smallest feature 7.5 mm): the root plate with five buttresses and the trunk in five pieces with seven flutes that fade from the foot upwards and a hollow under each of the sixteen lower branches, 1.1 m at the foot and 0.82 m at breast height, the leader to 29.4 m; forty branches as bowed, tapered tubes from inside the stem and on each one flat, feathery spray mass (an elliptic section along the branch, broad and thin, swelling to two fifths and drooping into a point past the tip, with seven fine ridges) that reaches in to the stem; the pointed, four-lobed leader sprays to 30 m.", "Körper bei 1:50 (kleinstes Merkmal 7,5 mm): der Wurzelteller mit fünf Brettwurzeln und der Stamm in fünf Stücken mit sieben Rillen, die vom Fuß nach oben auslaufen, und einer Mulde unter jedem der sechzehn unteren Äste, 1,1 m am Fuß und 0,82 m in Brusthöhe, der Gipfeltrieb bis 29,4 m; vierzig Äste als gebogene, verjüngte Röhren aus dem Stamminneren und auf jedem eine flache, gefiederte Zweigmasse (ein elliptischer Querschnitt entlang des Asts, breit und flach, bis zu zwei Fünfteln anschwellend und über die Spitze hinaus hängend zugespitzt, mit sieben feinen Rippen), die bis an den Stamm reicht; die spitzen, vierlappigen Zweige des Gipfeltriebs bis 30 m."),
        DEC("Thermal at 1:50: trunk bark, branch bark, the sprays of the three lower tiers as shade foliage and all others with the leader sprays as sun foliage, by reference to the body; twelve transpiration outlets on the sprays that carry the emitter anchors.", "Thermik bei 1:50: Stammrinde, Astrinde, die Zweige der drei unteren Etagen als Schattenlaub und alle übrigen mit den Zweigen des Gipfeltriebs als Sonnenlaub, als Verweis auf den Körper; zwölf Transpirationsauslässe auf den Zweigen, die die Emitteranker tragen."),
        DEC("Structure at 1:50: the root plate and the five stem pieces as tapered round members, the forty branches at their base sections, the leader shoot as a slender member, five stem joints, the fixed root plate and five stem masses.", "Tragwerk bei 1:50: der Wurzelteller und die fünf Stammstücke als verjüngte Rundstäbe, die vierzig Äste mit ihren Ansatzquerschnitten, der Gipfeltrieb als schlanker Stab, fünf Stammgelenke, der eingespannte Wurzelteller und fünf Stammmassen."),
      ],
      members: members(50), joints: joints(0.3), masses: masses(0.18, stemBones), supportR: 1.6, emitterSolids: emitterSolids(50, true),
    },
    100: {
      parts: parts100,
      pens: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 },
      marks: [
        { spacingM: 0.75, marginM: 0.11, groups: isFoliage, fn: featherMark(0.36), views: ["elevation", "front"] },
        { spacingM: 0.3, marginM: 0.09, groups: isTrunk, fn: stripMark(0.8), views: ["elevation", "front"] },
      ],
      decisions: {
        silhouette: DEC("At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: the fluted, buttressed trunk, the branches where they show and the forty spray masses with softer ridges in ten tiers, layered by distance.", "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: der gerillte Stamm mit Brettwurzeln, die Äste, wo sie sichtbar sind, und die vierzig Zweigmassen mit weicheren Rippen in zehn Etagen, nach Abstand geschichtet."),
        retained: DEC("Retained: every spray mass, the branch wood, flutes and armpits, sparse needle and bark marks in the side and front elevations.", "Beibehalten: jede Zweigmasse, das Astholz, Rillen und Achselmulden, spärliche Nadel- und Rindenzeichen in Seiten- und Vorderansicht."),
        merged: DEC("The finer ridges of every spray mass into fewer, softer ones; the flutes are shallower.", "Die feineren Rippen jeder Zweigmasse zu weniger, weicheren; die Rillen sind flacher."),
        omitted: DEC("Omitted: the plan texture, needles, cones.", "Weggelassen: die Grundrisstextur, Nadeln, Zapfen."),
        thinElements: DEC("Branch tips below 15 mm end inside their sprays.", "Astspitzen unter 15 mm enden in ihren Zweigen."),
        hatching: DEC("No hatch.", "Keine Schraffur."),
        styles: DEC("As at 1:50 with thinner pens.", "Wie bei 1:50 mit dünneren Federn."),
        overlays: DEC("Thermal and structural overlays map onto the same outlines as at 1:50.", "Thermische und statische Überlagerungen liegen auf denselben Umrissen wie bei 1:50."),
      },
      volumeDecisions: [
        DEC("Body at 1:100 (smallest feature 15 mm): the same trunk with shallower flutes and the armpits, meshed more coarsely, the branches with eight points round, the spray masses with five softer ridges.", "Körper bei 1:100 (kleinstes Merkmal 15 mm): derselbe Stamm mit flacheren Rillen und den Achselmulden, gröber vernetzt, die Äste mit acht Punkten im Umfang, die Zweigmassen mit fünf weicheren Rippen."),
        DEC("Thermal at 1:100: the four regions by reference; twelve transpiration outlets.", "Thermik bei 1:100: die vier Regionen als Verweis; zwölf Transpirationsauslässe."),
        DEC("Structure at 1:100: as at 1:50, meshed more coarsely.", "Tragwerk bei 1:100: wie bei 1:50, gröber vernetzt."),
      ],
      members: members(100), joints: joints(0.32), masses: masses(0.2, stemBones), supportR: 1.6, emitterSolids: emitterSolids(100, true),
    },
    200: {
      parts: parts200,
      pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
      decisions: {
        silhouette: DEC("At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: the flared trunk and forty smooth spray masses in ten tiers, so the narrow cone keeps its layering and the stem shows between the lower tiers.", "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: der ausgestellte Stamm und vierzig glatte Zweigmassen in zehn Etagen, sodass der schmale Kegel seine Schichtung behält und der Stamm zwischen den unteren Etagen sichtbar ist."),
        retained: DEC("Retained: the trunk with its buttressed foot, every spray mass.", "Beibehalten: der Stamm mit seinem Brettwurzelfuß, jede Zweigmasse."),
        merged: DEC("Merged: the armpits and most of the flutes into a plainer stem; the branch wood is drawn but lies almost wholly under its sprays.", "Zusammengefasst: die Achselmulden und die meisten Rillen zu einem schlichteren Stamm; das Astholz ist gezeichnet, liegt aber fast ganz unter seinen Zweigen."),
        omitted: DEC("Omitted: every texture.", "Weggelassen: jede Textur."),
        thinElements: DEC("No thin element: the thinnest drawn part is the upper stem of 0.2 m, 1 mm on paper.", "Kein dünnes Element: Das dünnste gezeichnete Teil ist der obere Stamm mit 0,2 m, 1 mm auf dem Papier."),
        hatching: DEC("No hatch.", "Keine Schraffur."),
        styles: DEC("Body pen outlines; solid styles fill the shade tiers a little darker.", "Umrisse mit der Körperfeder; die Flächenstile füllen die Schattenetagen etwas dunkler."),
        overlays: DEC("Thermal: each spray carries its foliage region, the trunk its bark; structural overlays colour the stem and the branch members.", "Thermik: Jeder Zweig trägt seine Laubregion, der Stamm seine Rinde; statische Überlagerungen färben den Stamm und die Aststäbe."),
      },
      volumeDecisions: [
        DEC("Body at 1:200 (smallest feature 30 mm): the trunk in six pieces with five soft buttresses and shallow flutes, the forty branches as six-sided tubes and their spray masses as smooth pointed forms of ten points round, and the leader sprays.", "Körper bei 1:200 (kleinstes Merkmal 30 mm): der Stamm in sechs Stücken mit fünf weichen Brettwurzeln und flachen Rillen, die vierzig Äste als sechseckige Röhren und ihre Zweigmassen als glatte, spitze Formen mit zehn Punkten im Umfang sowie die Zweige des Gipfeltriebs."),
        DEC("Thermal at 1:200: trunk bark, branch bark and the two foliage regions by reference; one transpiration outlet per emitter.", "Thermik bei 1:200: Stammrinde, Astrinde und die zwei Laubregionen als Verweis; ein Transpirationsauslass je Emitter."),
        DEC("Structure at 1:200: stem members, the branches as slender members inside their sprays, the fixed root plate and three stem masses.", "Tragwerk bei 1:200: Stammstäbe, die Äste als schlanke Stäbe in ihren Zweigen, der eingespannte Wurzelteller und drei Stammmassen."),
      ],
      members: members(200), joints: [], masses: masses(0.22, ["trunk-1", "trunk-2", "trunk-3"]), supportR: 1.6, emitterSolids: emitterSolids(200, false),
    },
    500: {
      parts: parts500,
      groups: onePerPart(parts500),
      pens: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
      decisions: {
        silhouette: DEC("At 1:500 (1 m is 2 mm) the dawn redwood is a narrow, layered cone 60 mm tall and 17 mm wide over a short, flared trunk: ten tier masses, each the four-lobed envelope of its opposite branches, stacked to the pointed top; the drawings are the true views of the 1:500 body.", "Bei 1:500 (1 m sind 2 mm) ist der Urweltmammutbaum ein schmaler, geschichteter Kegel von 60 mm Höhe und 17 mm Breite über einem kurzen, ausgestellten Stamm: zehn Etagenmassen, jede die vierlappige Hülle ihrer gegenständigen Äste, bis zur spitzen Krone gestapelt; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers."),
        retained: DEC("Retained: the trunk, the ten tiers (three shaded, seven sunlit) as separate outlines.", "Beibehalten: der Stamm, die zehn Etagen (drei beschattete, sieben besonnte) als getrennte Umrisse."),
        merged: DEC("Merged: the four spray masses of every tier into one tier mass, the leader sprays into the top tier, the stem into two pieces.", "Zusammengefasst: die vier Zweigmassen jeder Etage zu einer Etagenmasse, die Zweige des Gipfeltriebs in die oberste Etage, der Stamm in zwei Stücke."),
        omitted: DEC("Omitted: the branches above the lowest tier, the flutes and every texture; the four lowest branches stay, hidden in their tier, as the branch bark of this scale.", "Weggelassen: die Äste oberhalb der untersten Etage, die Rillen und jede Textur; die vier untersten Äste bleiben, in ihrer Etage verborgen, als Astrinde dieses Maßstabs."),
        thinElements: DEC("No thin element: the top of the stem ends inside the upper tiers.", "Kein dünnes Element: Die Stammspitze endet in den oberen Etagen."),
        hatching: DEC("No hatch.", "Keine Schraffur."),
        styles: DEC("One body pen; solid styles shade the three lower tiers darker.", "Eine Körperfeder; die Flächenstile tönen die drei unteren Etagen dunkler."),
        overlays: DEC("Thermal: the tier outlines carry their foliage regions; the structural overlays colour the stem.", "Thermik: Die Etagenumrisse tragen ihre Laubregionen; die statischen Überlagerungen färben den Stamm."),
      },
      volumeDecisions: [
        DEC("Body at 1:500 (smallest feature 75 mm): the trunk in two pieces with a flared foot, the four branches of the lowest tier and ten tier masses, each the envelope sleeve of its four spray masses (largest radius per 0.45 m band and per twelfth of the round, dilated over one neighbour), the leader sprays in the top tier, stretched to the true extent of the 1:50 body; every tier encloses the stem.", "Körper bei 1:500 (kleinstes Merkmal 75 mm): der Stamm in zwei Stücken mit ausgestelltem Fuß, die vier Äste der untersten Etage und zehn Etagenmassen, jede die Hüllmanschette ihrer vier Zweigmassen (größter Radius je 0,45-m-Band und je Zwölftel des Umfangs, über einen Nachbarn geweitet), die Zweige des Gipfeltriebs in der obersten Etage, auf die wahre Ausdehnung des 1:50-Körpers gestreckt; jede Etage umschließt den Stamm."),
        DEC("Thermal at 1:500: trunk bark, the branch bark of the four lowest branches and the two foliage regions by reference; one transpiration outlet per emitter.", "Thermik bei 1:500: Stammrinde, die Astrinde der vier untersten Äste und die zwei Laubregionen als Verweis; ein Transpirationsauslass je Emitter."),
        DEC("Structure at 1:500: the stem members at their sections, the branches as slender members inside the tier masses, the fixed root plate and two stem masses.", "Tragwerk bei 1:500: die Stammstäbe mit ihren Querschnitten, die Äste als schlanke Stäbe in den Etagenmassen, der eingespannte Wurzelteller und zwei Stammmassen."),
      ],
      members: members(500), joints: [], masses: masses(0.28, ["trunk-1", "trunk-3"]), supportR: 1.6, emitterSolids: emitterSolids(500, false),
    },
  };
  return {
    id: "dawn-redwood",
    title: t("Dawn Redwood", "Urweltmammutbaum"),
    summary: t(
      "A dawn redwood (Metasequoia glyptostroboides) of 30 m in summer leaf: a strongly tapered trunk of reddish-brown, fibrous bark, buttressed and deeply fluted at the foot with a hollow under every lower branch, carries opposite branches in ten tiers whose soft, feathery sprays build a narrow cone 8.8 m across at the base and pointed at the top.",
      "Ein Urweltmammutbaum (Metasequoia glyptostroboides) von 30 m im Sommerlaub: Ein stark abholziger Stamm mit rotbrauner, faseriger Rinde, am Fuß mit Brettwurzeln und tief gerillt und mit einer Mulde unter jedem unteren Ast, trägt gegenständige Äste in zehn Etagen, deren weiche, gefiederte Zweige einen schmalen Kegel von 8,8 m Breite an der Basis bilden, der oben spitz ausläuft.",
    ),
    tags: ["tree", "deciduous", "conifer", "conical", "fluted-trunk", "wind", "transpiration"],
    references: [
      { id: "williams-2003", citation: "Williams, C. J., LePage, B. A., Vann, D. R., Tange, T., Ikeda, H., Ando, M., Kusakabe, T., Tsuzuki, H. and Sweda, T. (2003). Structure, allometry, and biomass of plantation Metasequoia glyptostroboides in Japan. Forest Ecology and Management 180, 287-301 (conical crowns, branch architecture, stem form and biomass allocation)." },
      { id: "farjon-2010", citation: "Farjon, A. (2010). A Handbook of the World's Conifers. Brill, Leiden. Metasequoia glyptostroboides: deciduous tree to 50 m, trunk buttressed and fluted, bark reddish-brown, fibrous, exfoliating in strips, branchlets and needles opposite, crown conical." },
      { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5, green properties; second-growth redwood (Cupressaceae, Sequoioideae) used as the closest listed proxy for Metasequoia: E 6.6 GPa, MOR 41 MPa." },
      { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m." },
      { id: "mayhead-1973", citation: "Mayhead, G. J. (1973). Some drag coefficients for British forest trees derived from wind tunnel studies. Agricultural Meteorology 12, 123-130." },
      { id: "moore-maguire-2004", citation: "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203." },
      { id: "coutts-1986", citation: "Coutts, M. P. (1986). Components of tree stability in Sitka spruce on peaty gley soil. Forestry 59(2), 173-197 (root-plate rotation under turning moment)." },
      { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, rainfall interception, boundary-layer conductance." },
      { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
    ],
    notes: t(
      "Trunk, branches and sprays are authored by hand after a park-grown dawn redwood of about 60 years: a trunk that tapers fast from a buttressed foot, branches in opposite pairs that alternate their direction from tier to tier, and the feathery summer foliage. Masses are volume times a green density of 780 kg/m3 (redwood proxy of the Wood Handbook, wet heartwood) plus a crown bulk density of 3 kg per cubic metre of spray mass. A deciduous tree is shown in leaf: shapes cannot be switched off, so there is no winter-bare pose; the poses are calm, fresh breeze, gale and rain-laden sprays.",
      "Stamm, Äste und Zweige sind von Hand nach einem etwa 60-jährigen Urweltmammutbaum in einem Park gezeichnet: ein Stamm, der sich von einem Brettwurzelfuß rasch verjüngt, Äste in gegenständigen Paaren, die von Etage zu Etage ihre Richtung wechseln, und das gefiederte Sommerlaub. Massen sind Volumen mal einer Frischdichte von 780 kg/m3 (Redwood-Proxy des Wood Handbook, nasses Kernholz) plus eine Kronenrohdichte von 3 kg je Kubikmeter Zweigmasse. Ein laubabwerfender Baum wird belaubt gezeigt: Formen lassen sich nicht ausschalten, daher gibt es keine winterkahle Pose; die Posen sind Ruhe, frische Brise, Sturm und regennasse Zweige.",
    ),
    materials: [
      { id: "metasequoia-bark", title: t("Reddish-brown, fibrous bark in long strips", "Rotbraune, faserige Rinde in langen Streifen"), emissivity: 0.95, solarAbsorptivity: 0.72, densityKgM3: 450, conductivityWmK: 0.1, source: "authored-estimate", reference: "jones-2014" },
      { id: "metasequoia-foliage-sun", title: t("Soft summer needles, sunlit tiers", "Weiche Sommernadeln, besonnte Etagen"), emissivity: 0.97, solarAbsorptivity: 0.5, source: "authored-estimate", reference: "jones-2014" },
      { id: "metasequoia-foliage-shade", title: t("Soft summer needles, shaded lower tiers", "Weiche Sommernadeln, beschattete untere Etagen"), emissivity: 0.97, solarAbsorptivity: 0.55, source: "authored-estimate", reference: "jones-2014" },
      { id: "metasequoia-wood", title: t("Green dawn redwood wood", "Frisches Holz des Urweltmammutbaums"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 780, conductivityWmK: 0.12, elasticModulusPa: 6600000000, strengthPa: 41000000, source: "literature", reference: "wood-handbook-2021" },
    ],
    bones: BONES,
    scales,
    woodDensity: { "metasequoia-bark": 780 },
    foliageDensity: { "metasequoia-foliage-sun": 3, "metasequoia-foliage-shade": 3 },
    hiddenWood: (bone: string) => (bone === "top-spray" ? 780 * (Math.PI / 12) * 4.3 * (0.07 ** 2 + 0.07 * 0.025 + 0.025 ** 2) : 0),
    regions: [],
    emitters: [
      { id: "transpiration-sun", region: "foliage-sun", title: t("Transpiration of the sunlit sprays (distributed over the upper cone)", "Transpiration der besonnten Zweige (über den oberen Kegel verteilt)"), anchors: sunEmit.map((b) => ({ bone: b, at: anchorOf(b) })), diameterM: 1.6, source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-shade", region: "foliage-shade", title: t("Transpiration of the shaded lower sprays (distributed over the lower tiers)", "Transpiration der beschatteten unteren Zweige (über die unteren Etagen verteilt)"), anchors: shadeEmit.map((b) => ({ bone: b, at: anchorOf(b) })), diameterM: 1.6, source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    thermalEnvironment: { airTempC: 26, meanRadiantTempC: 28, relativeHumidity: 0.65, windMS: 2, windDirectionDeg: 0, solarWM2: 800, pressurePa: 101325, diffuseFraction: 0.15 },
    thermalAssumptions: [], structureAssumptions: [], poses: [], clips: [],
    symbol: {
      title: t("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
      description: t("A conventional sign of the dawn redwood for site and landscape plans: the outline of the true crown extent and the trunk section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Zeichen des Urweltmammutbaums für Lage- und Freiraumpläne: der Umriss der wahren Kronenausdehnung und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
      note: t("Model metres at true size: the outline is the extent of the crown in each view, the filled shape the trunk (plan: the trunk section at breast height).", "Modellmeter in wahrer Größe: Der Umriss ist die Ausdehnung der Krone in jeder Ansicht, die gefüllte Form der Stamm (Grundriss: der Stammquerschnitt in Brusthöhe)."),
    },
    defaultScale: 200,
    symbolStemD: 0.82,
    distinction: t("A 30 m deciduous conifer in summer leaf, a narrow layered cone of opposite branches over a buttressed, deeply fluted trunk with a hollow under every lower branch.", "Ein 30 m hoher laubabwerfender Nadelbaum im Sommerlaub, ein schmaler, geschichteter Kegel aus gegenständigen Ästen über einem tief gerillten Stamm mit Brettwurzeln und einer Mulde unter jedem unteren Ast."),
    massSource: { wood: "authored-estimate", foliage: "authored-estimate", reference: "wood-handbook-2021" },
  };
}

/** 🌡️ The physical scenario of the dawn redwood: regions with the bark areas of the 1:50 body and a leaf area index of 5, poses with EN 1991-1-4 wind and intercepted rain, the bending they cause and the three clips. */
export function physics(sp: Species, _ctx: Ctx): Physics {
  const stats = foliageStats(sp, 0.3);
  const vol = (r: string) => sp.scales[50].parts.filter((p) => p.region === r).reduce((s, p) => s + volume(p.mesh), 0);
  const vSun = vol("foliage-sun"), vShade = vol("foliage-shade");
  const plan = projectedArea(sp.scales[50].parts.filter((p) => p.kind === "foliage").map((p) => p.mesh), "plan", 0.04);
  const leafTotal = r1(5 * plan);
  const aSun = r1((leafTotal * vSun) / (vSun + vShade)), aShade = r1(leafTotal - aSun);
  const de = (x: number) => String(x).replace(".", ",");
  const regions = [
    { id: "bark", title: t("Trunk bark, reddish-brown and fibrous", "Stammrinde, rotbraun und faserig"), kind: "bark", model: "passive", material: "metasequoia-bark", areaM2: r2(regionArea(sp, "bark")), areaBasis: "total", characteristicLengthM: 0.7, geometry: "cylinder", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 70000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "branch-bark", title: t("Bark of the branches", "Rinde der Äste"), kind: "bark", model: "passive", material: "metasequoia-bark", areaM2: r2(regionArea(sp, "branch-bark")), areaBasis: "total", characteristicLengthM: 0.1, geometry: "cylinder", skyViewFactor: 0.2, layers: [], heatCapacityJM2K: 25000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "foliage-sun", title: t("Feathery sprays of the sunlit upper tiers", "Gefiederte Zweige der besonnten oberen Etagen"), kind: "foliage", model: "leaf", material: "metasequoia-foliage-sun", areaM2: aSun, areaBasis: "exchange", characteristicLengthM: 0.002, geometry: "horizontal-plate-up", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 800, leaf: { stomatalConductanceMaxMS: 0.004, leafAreaIndex: r2(aSun / plan), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    { id: "foliage-shade", title: t("Feathery sprays of the shaded lower tiers", "Gefiederte Zweige der beschatteten unteren Etagen"), kind: "foliage", model: "leaf", material: "metasequoia-foliage-shade", areaM2: aShade, areaBasis: "exchange", characteristicLengthM: 0.002, geometry: "horizontal-plate-up", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 800, leaf: { stomatalConductanceMaxMS: 0.0028, leafAreaIndex: r2(aShade / plan), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  ];
  const ttl = (bone: string, en: string, gde: string): Text => { const b = sp.bones.find((x) => x.id === bone)!; return t(`${en} ${b.title.en.charAt(0).toLowerCase()}${b.title.en.slice(1)}`, `${gde} ${b.title.de}`); };
  const cases = {
    calm: { id: "calm", v10: 1.5, cd: 0.45, source: "literature", reference: "en-1991-1-4" },
    windy: { id: "windy", v10: 8, cd: 0.38, source: "literature", reference: "en-1991-1-4" },
    storm: { id: "storm", v10: 22, cd: 0.26, source: "literature", reference: "mayhead-1973" },
  };
  const stemLoad = { bone: "trunk-1", toM: 1.5, d: 0.82, title: t("Wind drag on the bare trunk foot", "Winddruck auf den kahlen Stammfuß") };
  const E = 6.6e9, kRoot = 9e6;
  const limit = (b: BoneSpec) => (b.joint.type === "revolute" ? b.joint.maxDeg * 0.95 : 0);
  const wl = Object.fromEntries(Object.entries(cases).map(([k, c]) => [k, windLoads(sp, stats, c, stemLoad, (b) => ttl(b, "Wind drag on", "Winddruck auf"))]));
  const windyBones = bend(sp, wl.windy!.forces, E, kRoot, "trunk-1", limit);
  const stormBones = bend(sp, wl.storm!.forces, E, kRoot, "trunk-1", limit);
  const rainForces = stats.map((s) => ({ bone: s.bone, at: s.c, fx: 0, fy: -1.5 * G * s.planM2 }));
  const rainBones = bend(sp, rainForces, E, kRoot, "trunk-1", limit);
  const fr = frames(sp.bones);
  const rainLoads = stats.map((s) => { const l = toLocal(fr[s.bone]!, s.c); return { id: `rain-${s.bone}`, kind: "imposed", title: ttl(s.bone, "Intercepted rain on", "Zurückgehaltener Regen auf"), bone: s.bone, x: r3(l[0]), y: r3(l[1]), fxN: 0, fyN: r1(-1.5 * G * s.planM2), source: "authored-estimate", reference: "jones-2014" }; });
  const totalF = (k: string) => r1(wl[k]!.forces.reduce((s, f) => s + f.fx, 0) / 1000);
  const baseM = (k: string) => r1(wl[k]!.forces.reduce((s, f) => s + f.fx * f.at[1], 0) / 1000);
  const rainTotal = r1(rainForces.reduce((s, f) => s - f.fy, 0) / 1000);
  const tr = (b: number, sun: number, shade: number, wet?: number) => ({
    bark: { exposedFraction: 1, sunlitFraction: b, ...(wet === undefined ? {} : { wettedness: wet * 0.7 }) },
    "branch-bark": { exposedFraction: 1, sunlitFraction: b * 0.6, ...(wet === undefined ? {} : { wettedness: wet * 0.7 }) },
    "foliage-sun": { exposedFraction: 1, sunlitFraction: sun, ...(wet === undefined ? {} : { wettedness: wet }) },
    "foliage-shade": { exposedFraction: 1, sunlitFraction: shade, ...(wet === undefined ? {} : { wettedness: wet }) },
  });
  const poses = [
    { id: "calm", title: t("Calm summer day", "Windstiller Sommertag"), description: t(`Light air of 1.5 m/s at 10 m on a humid summer day: the cone stands still; cd 0.45 on the frontal area of the sprays gives ${totalF("calm")} kN.`, `Leiser Zug von 1,5 m/s in 10 m Höhe an einem schwülen Sommertag: Der Kegel steht still; cd 0,45 auf die Anströmfläche der Zweige ergibt ${de(totalF("calm"))} kN.`), bones: {}, ground: "authored", contacts: [], thermal: binding(tr(0.3, 0.85, 0.25), { "transpiration-sun": 0.8, "transpiration-shade": 0.8 }), loads: wl.calm!.loads },
    { id: "windy", title: t("Fresh breeze", "Frische Brise"), description: t(`Beaufort 5, 8 m/s at 10 m, cd 0.38 on the frontal area of every spray with an optical porosity of 30 %: ${totalF("windy")} kN on the crown and a base moment of about ${baseM("windy")} kN m; the tall stem bends progressively towards the top.`, `Beaufort 5, 8 m/s in 10 m Höhe, cd 0,38 auf die Anströmfläche jedes Zweigs bei 30 % optischer Porosität: ${de(totalF("windy"))} kN auf die Krone und ein Fußmoment von etwa ${de(baseM("windy"))} kNm; der hohe Stamm biegt sich zur Spitze hin zunehmend.`), bones: windyBones, ground: "authored", contacts: [], thermal: binding(tr(0.3, 0.85, 0.25), { "transpiration-sun": 1, "transpiration-shade": 1 }), loads: wl.windy!.loads },
    { id: "storm", title: t("Gale", "Sturm"), description: t(`Beaufort 9, 22 m/s at 10 m, cd 0.26 after the soft sprays streamline (Mayhead 1973): ${totalF("storm")} kN and a base moment of about ${baseM("storm")} kN m; the top swings out and the sprays fold to the lee, rain wets them.`, `Beaufort 9, 22 m/s in 10 m Höhe, cd 0,26 nach dem Anlegen der weichen Zweige (Mayhead 1973): ${de(totalF("storm"))} kN und ein Fußmoment von etwa ${de(baseM("storm"))} kNm; die Spitze schwingt aus und die Zweige legen sich nach Lee, Regen nässt sie.`), bones: stormBones, ground: "authored", contacts: [], thermal: binding(tr(0.05, 0.3, 0.05, 0.4), { "transpiration-sun": 0.35, "transpiration-shade": 0.35 }), loads: wl.storm!.loads },
    { id: "rain-laden", title: t("Rain-laden sprays", "Regennasse Zweige"), description: t(`After a summer downpour: 1.5 kg/m2 of water held on the plan area of every spray (canopy interception storage after Jones 2014), ${rainTotal} kN in all, weighs the branches down a little; no wind, the foliage is wet and in shade.`, `Nach einem Sommerplatzregen: 1,5 kg/m2 Wasser, auf der Grundrissfläche jedes Zweigs zurückgehalten (Interzeptionsspeicher der Krone nach Jones 2014), insgesamt ${de(rainTotal)} kN, drückt die Äste etwas hinunter; kein Wind, das Laub ist nass und im Schatten.`), bones: rainBones, ground: "authored", contacts: [], thermal: binding(tr(0, 0.05, 0, 0.9), { "transpiration-sun": 0.1, "transpiration-shade": 0.1 }), loads: rainLoads },
  ];
  const loadIds = (wl.windy!.loads as { id: string }[]).map((l) => l.id);
  const stems = ["trunk-1", "trunk-2", "trunk-3", "trunk-4", "leader"];
  const clips = [
    swayClip(sp, windyBones, 3.3, 0.6, (b) => (stems.includes(b.id) ? 0.04 : 0.3), t("Wind sway", "Windschwingung"), t("A deterministic 3.3 s sway cycle (first mode of a tall conifer, about 0.3 Hz after Moore and Maguire 2004) about the fresh-breeze lean: the stem bends gently and every branch swings up to 0.4 degrees behind it.", "Ein deterministischer Schwingungszyklus von 3,3 s (erste Eigenform eines hohen Nadelbaums, etwa 0,3 Hz nach Moore und Maguire 2004) um die Neigung der frischen Brise: Der Stamm biegt sich sanft, jeder Ast schwingt bis zu 0,4 Grad hinterher."), H),
    gustClip(windyBones, loadIds, t("Gust", "Windböe"), t("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak velocity pressure with Iv 0.25) and decay; the cone leans and recovers.", "Eine einzelne Böe über 8 s: Die Windlastspuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzengeschwindigkeitsdruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; der Kegel neigt sich und richtet sich wieder auf.")),
    diurnalClip({ "transpiration-sun": [0.03, 1], "transpiration-shade": [0.03, 0.9] }, { bark: 0.3, "branch-bark": 0.18, "foliage-sun": 0.85, "foliage-shade": 0.25 }, t("Day of transpiration", "Transpirationstag"), t("A 24 h summer day compressed to 24 s (1 s = 1 h), calm weather: sunlit fractions and transpiration follow the sun, the stomata are nearly closed at night; leaf and bark temperatures come from the thermal solver.", "Ein 24-h-Sommertag auf 24 s gerafft (1 s = 1 h) bei ruhigem Wetter: Besonnte Anteile und Transpiration folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt- und Rindentemperaturen ergibt der thermische Löser.")),
  ];
  const thermalAssumptions = [
    t(`Foliage regions use one-sided leaf area: a leaf area index of 5 over the crown ground projection of ${r1(plan)} m2 (authored estimate for a vigorous deciduous conifer in summer, guided by Williams et al. 2003), ${leafTotal} m2 in all, split between the three shaded lower tiers and the sunlit upper ones by volume. The maximum stomatal conductance is an authored estimate, higher than for evergreen conifers, guided by Jones (2014). Photosynthesis type: C3.`, `Laubregionen verwenden die einseitige Blattfläche: ein Blattflächenindex von 5 über der Kronenprojektion am Boden von ${de(r1(plan))} m2 (Autorenschätzung für einen wüchsigen laubabwerfenden Nadelbaum im Sommer, angelehnt an Williams et al. 2003), insgesamt ${de(leafTotal)} m2, nach Volumen auf die drei beschatteten unteren und die besonnten oberen Etagen verteilt. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung, höher als bei immergrünen Nadelbäumen, angelehnt an Jones (2014). Photosynthesetyp: C3.`),
    t("Bark regions are passive; their areal heat capacity is the volumetric heat capacity of green wood times an effective depth of 1 to 2.5 cm, an authored estimate guided by the Wood Handbook; their areas are the outer surfaces of the 1:50 trunk and of the 1:50 branches. No surface temperature is authored.", "Rindenregionen sind passiv; ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz mal einer wirksamen Tiefe von 1 bis 2,5 cm, eine Autorenschätzung angelehnt an das Wood Handbook; ihre Flächen sind die Außenflächen des 1:50-Stamms und der 1:50-Äste. Es werden keine Oberflächentemperaturen vorgegeben."),
    t("Transpiration emitters release the latent flux of their foliage region as vapour, spread over six anchors each on the upper side of the sprays; the direction is the vertical in the frame of the first anchor's bone.", "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf je sechs Anker auf der Oberseite der Zweige; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers."),
  ];
  const structureAssumptions = [
    t("One rigid body per evaluated pose, fixed at the root plate. Wind drag acts at the centroid of every spray mass and on the bare trunk foot as a line load; intercepted rain at the same centroids. Pose angles are quasi-static: each joint turns by M L / (2 E I) of its section (E 6.6 GPa, second-growth redwood proxy), the root plate by the base moment over an anchorage stiffness of 9 MN m/rad (authored estimate after Coutts 1986); the fluted section is taken as the solid circle of its mean diameter. Dynamic amplification and root-soil failure are not modelled.", "Ein starrer Körper je ausgewerteter Pose, am Wurzelteller eingespannt. Winddruck wirkt im Schwerpunkt jeder Zweigmasse und am kahlen Stammfuß als Linienlast; zurückgehaltener Regen in denselben Schwerpunkten. Die Posenwinkel sind quasistatisch: Jedes Gelenk dreht sich um M L / (2 E I) seines Querschnitts (E 6,6 GPa, Proxy Redwood aus zweitem Aufwuchs), der Wurzelteller um das Fußmoment geteilt durch eine Verankerungssteifigkeit von 9 MNm/rad (Autorenschätzung nach Coutts 1986); der gerillte Querschnitt gilt als Vollkreis seines mittleren Durchmessers. Dynamische Überhöhung und Wurzel-Boden-Versagen werden nicht modelliert."),
  ];
  return { regions, poses, clips, thermalAssumptions, structureAssumptions, supportFootprintM: 3.0 };
}
