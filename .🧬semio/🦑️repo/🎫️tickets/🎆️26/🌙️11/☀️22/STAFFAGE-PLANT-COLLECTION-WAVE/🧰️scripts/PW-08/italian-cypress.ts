/** 🌲️ Italian cypress (Cupressus sempervirens 'Stricta'), PW-08: an 18 m narrow column of fastigiate branch sprays around a straight stem. Every number below is chosen by hand from the cited references; the kit only turns them into meshes, drawings and the asset document.
 *
 * @see ./asset.ts
 * @see https://forest.jrc.ec.europa.eu/en/european-atlas/ — Caudullo and de Rigo 2016, Cupressus sempervirens */
import { RAD, add, bounds, frames, r1, r2, r3, rng, t, toLocal, tube, volume, type BoneSpec, type Mesh, type P2, type Scale, type Station, type Text, type V3, type View } from "./lib.ts";
import { onePerPart, projectedArea, regionArea, sec, type Ctx, type Group, type MarkFn, type Part, type Physics, type ScaleDesign, type Species } from "./asset.ts";
import { G, bend, binding, diurnalClip, foliageStats, gustClip, swayClip, windLoads } from "./common.ts";
import { fitExtent, profile, sleeve, stem, steps } from "./shared-geometry.ts";

const H = 18.0;
const crownR = profile([[0.6, 0.5], [1.5, 0.92], [3.5, 1.2], [5.5, 1.32], [8, 1.3], [10.5, 1.2], [12.5, 1.02], [14.5, 0.78], [16, 0.52], [17.2, 0.26], [18, 0.04]]);
const stemR = profile([[0, 0.34], [0.15, 0.285], [0.4, 0.245], [1.3, 0.225], [4.5, 0.19], [9, 0.145], [13.5, 0.095], [16, 0.055], [17.6, 0.02]]);
const flare = (th: number, y: number): number => 1 + 0.13 * Math.max(0, Math.cos(5 * th + 0.4)) ** 2 * Math.exp(-y / 0.22);
const flute = (th: number, y: number): number => flare(th, y) * (1 + 0.025 * Math.sin(9 * th + y * 0.7));

/** 🌿️ The tiers of fastigiate sprays: base and top height, number of sprays round the stem and the azimuth of the first one (degrees from ahead towards the left). */
const TIERS: { y0: number; y1: number; n: number; az: number; region: "foliage-sun" | "foliage-shade" }[] = [
  { y0: 0.6, y1: 4.4, n: 5, az: 0, region: "foliage-shade" },
  { y0: 2.6, y1: 6.6, n: 5, az: 36, region: "foliage-shade" },
  { y0: 4.8, y1: 8.9, n: 5, az: 8, region: "foliage-sun" },
  { y0: 7.0, y1: 11.1, n: 5, az: 44, region: "foliage-sun" },
  { y0: 9.4, y1: 13.2, n: 4, az: 18, region: "foliage-sun" },
  { y0: 11.6, y1: 15.0, n: 4, az: 63, region: "foliage-sun" },
  { y0: 13.6, y1: 16.6, n: 3, az: 30, region: "foliage-sun" },
];

type Spray = { id: string; tier: number; phi: number; y0: number; y1: number; parent: string; region: "foliage-sun" | "foliage-shade"; d: number };
const random = rng(20261122);
const SPRAYS: Spray[] = TIERS.flatMap((tier, ti) => Array.from({ length: tier.n }, (_, k) => {
  const jitter = (random() - 0.5) * 12;
  const dl = (random() - 0.5) * 0.4;
  const y0 = tier.y0 + (random() - 0.5) * 0.3;
  const parent = y0 < 4.5 ? "trunk-1" : y0 < 9 ? "trunk-2" : y0 < 13.5 ? "trunk-3" : "leader";
  return { id: `spray-${ti + 1}-${k + 1}`, tier: ti, phi: (tier.az + (360 * k) / tier.n + jitter) * RAD, y0, y1: tier.y1 + dl, parent, region: tier.region, d: r3(0.075 - ti * 0.006) };
}));

const g = (s: number): number => Math.pow(Math.max(0, Math.sin(Math.PI * Math.pow(s, 0.8))), 0.6);

/** 🔥️ One fastigiate spray as a closed flame: an elliptic section (radial half depth reaching the stem, tangential half width overlapping its neighbours) that swells from a rounded foot to its widest at two fifths of its height and runs out into a point, `ridges` lengthwise ridges of relative height `ridge` on its skin. */
function spray(s: Spray, nAround: number, dy: number, ridges: number, ridge: number, n: number): Mesh {
  const L = s.y1 - s.y0;
  const ys = steps(0, 1, dy / L);
  const st: Station[] = ys.map((u, i) => {
    const y = s.y0 + u * L;
    const R = crownR(Math.min(y, 17.9)) * (0.96 + 0.08 * Math.sin(s.phi * 3 + s.tier));
    const rho = 0.52 * R;
    const c: V3 = [rho * Math.cos(s.phi), y, rho * Math.sin(s.phi)];
    if (i === 0 || i === ys.length - 1) return { c, r: 0 };
    const a = 0.5 * R * g(u);
    const b = Math.max(0.05, 1.12 * Math.PI * rho / n) * g(u);
    return { c, r: (th: number) => ((a * b) / Math.hypot(b * Math.cos(th), a * Math.sin(th))) * (1 + ridge * Math.sin(ridges * th + s.tier + u * 2)) };
  });
  return tube(st, nAround, [Math.cos(s.phi), 0, Math.sin(s.phi)]);
}

/** 🗼️ The spire: the leader's own close sprays as one pointed, four-lobed cone from 15.5 m to the tip. */
function spire(nAround: number, dy: number, lobe: number): Mesh {
  const ys = steps(15.5, H, dy);
  const st: Station[] = ys.map((y, i) => {
    const c: V3 = [0, y, 0];
    if (i === 0 || i === ys.length - 1) return { c, r: 0 };
    const u = (y - 15.5) / (H - 15.5);
    const r = 0.5 * Math.pow(Math.sin(Math.PI * Math.min(1, u * 0.6 + 0.22)), 0.7) * (1 - u) ** 0.55 * 1.25;
    return { c, r: (th: number) => r * (1 + lobe * Math.cos(4 * th + 0.5)) };
  });
  return tube(st, nAround, [1, 0, 0]);
}

const BONES: BoneSpec[] = [
  { id: "root-plate", parent: null, title: t("Root plate and stem foot", "Wurzelteller und Stammfuß"), o: [0, 0, 0], angle: 90, length: 0.3, joint: { type: "fixed" }, section: { d: 0.56, material: "cypress-wood", title: t("Solid section, d = 56 cm", "Vollquerschnitt, d = 56 cm") } },
  { id: "trunk-1", parent: "root-plate", title: t("Stem, lower", "Stamm unten"), o: [0, 0.3, 0], angle: 90, length: 4.2, joint: { type: "revolute", minDeg: -2.5, maxDeg: 2.5 }, section: { d: 0.42, material: "cypress-wood", title: t("Solid section, d = 42 cm", "Vollquerschnitt, d = 42 cm") } },
  { id: "trunk-2", parent: "trunk-1", title: t("Stem, middle", "Stamm Mitte"), o: [0, 4.5, 0], angle: 90, length: 4.5, joint: { type: "revolute", minDeg: -3, maxDeg: 3 }, section: { d: 0.33, material: "cypress-wood", title: t("Solid section, d = 33 cm", "Vollquerschnitt, d = 33 cm") } },
  { id: "trunk-3", parent: "trunk-2", title: t("Stem, upper", "Stamm oben"), o: [0, 9, 0], angle: 90, length: 4.5, joint: { type: "revolute", minDeg: -3.5, maxDeg: 3.5 }, section: { d: 0.24, material: "cypress-wood", title: t("Solid section, d = 24 cm", "Vollquerschnitt, d = 24 cm") } },
  { id: "leader", parent: "trunk-3", title: t("Leader", "Gipfeltrieb"), o: [0, 13.5, 0], angle: 90, length: 4.3, joint: { type: "revolute", minDeg: -5, maxDeg: 5 }, section: { d: 0.13, material: "cypress-wood", title: t("Solid section, d = 13 cm", "Vollquerschnitt, d = 13 cm") } },
  ...SPRAYS.map((s): BoneSpec => {
    const L = s.y1 - s.y0;
    const rho = 0.52 * crownR(s.y0 + 0.45 * L);
    const angle = Math.atan2(0.75 * L, rho * Math.cos(s.phi)) / RAD;
    const side = Math.cos(s.phi) >= 0 ? t("ahead", "vorn") : t("behind", "hinten");
    const lat = Math.sin(s.phi) >= 0 ? t("left", "links") : t("right", "rechts");
    return {
      id: s.id, parent: s.parent, title: t(`Fastigiate branch spray, tier ${s.tier + 1}, ${side.en} ${lat.en}`, `Aufrechter Zweigbüschel, Etage ${s.tier + 1}, ${side.de} ${lat.de}`), o: [0, s.y0, 0], angle: r2(angle), length: r3(Math.hypot(0.75 * L, rho * Math.cos(s.phi))),
      joint: { type: "revolute", minDeg: -6, maxDeg: 6 }, section: { d: s.d, material: "cypress-wood", title: t(`Branch base, d = ${Math.round(s.d * 100)} cm`, `Astansatz, d = ${Math.round(s.d * 100)} cm`) },
    };
  }),
  { id: "spire", parent: "leader", title: t("Spire of the leader's sprays", "Spitze aus den Zweigen des Gipfeltriebs"), o: [0, 15.5, 0], angle: 90, length: 2.5, joint: { type: "revolute", minDeg: -7, maxDeg: 7 }, section: { d: 0.07, material: "cypress-wood", title: t("Shoot, d = 7 cm", "Trieb, d = 7 cm") } },
];

const bark = (id: string, bone: string, mesh: Mesh, tone = 0.3): Part => ({ id, bone, region: "bark", material: "cypress-bark", tone, mesh, kind: "wood" });
const leaf = (id: string, bone: string, region: string, mesh: Mesh): Part => ({ id, bone, region, material: region === "foliage-sun" ? "cypress-foliage-sun" : "cypress-foliage-shade", tone: region === "foliage-sun" ? 0.5 : 0.42, mesh, kind: "foliage" });

/** 🪵️ The stem of a scale as four bone pieces (root foot, lower, middle, upper with the leader), each overlapping the next by a short collar so sway never opens a seam. */
function stemParts(n: number, dy: number, fluted: boolean): Part[] {
  const k = fluted ? flute : flare;
  const pieces: [string, number, number, boolean][] = [["root-plate", 0, 0.42, false], ["trunk-1", 0.3, 4.62, false], ["trunk-2", 4.5, 9.12, false], ["trunk-3", 9, 13.62, false], ["leader", 13.5, 17.6, true]];
  return pieces.map(([bone, a, b, apex]) => bark(`stem-${bone}`, bone, stem(0, 0, steps(a, b, bone === "root-plate" ? 0.06 : dy), stemR, k, n, apex)));
}

const allSprays = (nAround: number, dy: number, ridges: number, ridge: number): Mesh[] => SPRAYS.map((s) => spray(s, nAround, dy, ridges, ridge, TIERS[s.tier]!.n));

/** ✒️ Foliage marks: small upturned chevrons (the scale-leaf sprays seen from the side) in elevation and front, short tangential ticks round the stem in plan. */
const foliageMark = (size: number): MarkFn => (view: View, _scale: Scale, q: P2, _g: Group, rnd: () => number) => {
  const s = size * (0.85 + 0.3 * rnd());
  if (view === "plan") {
    const a = Math.atan2(q[1], q[0]) + Math.PI / 2;
    const [c, si] = [Math.cos(a) * s * 0.5, Math.sin(a) * s * 0.5];
    return [[[q[0] - c, q[1] - si], [q[0] + c * 0.2 + si * 0.25, q[1] + si * 0.2 - c * 0.25], [q[0] + c, q[1] + si]]];
  }
  return [[[q[0] - 0.45 * s, q[1] - 0.55 * s], [q[0] + 0.05 * s * (rnd() - 0.5), q[1] + 0.35 * s], [q[0] + 0.45 * s, q[1] - 0.5 * s]]];
};
/** 🪓️ Bark marks: short vertical furrows of the fibrous, finely fissured grey-brown bark. */
const barkMark = (lenM: number): MarkFn => (view: View, _scale: Scale, q: P2, _g: Group, rnd: () => number) => (view === "plan" ? [] : [[[q[0], q[1] - lenM / 2], [q[0] + 0.012 * (rnd() - 0.5), q[1]], [q[0] + 0.01 * (rnd() - 0.5), q[1] + lenM / 2]]]);

const isFoliage = (g: Group) => g.regions.some((r) => r.startsWith("foliage"));
const isBark = (g: Group) => g.regions.length === 1 && g.regions[0] === "bark";

/** 🏛️ The members of a scale: the stem pieces at their sections and every spray and the spire as a slender rod of its branch base diameter inside its foliage. */
function members(scale: Scale): ScaleDesign["members"] {
  const out: ScaleDesign["members"] = [
    { bone: "root-plate", from: [0, 0, 0], to: [0, 0.42, 0], d0: 0.5, d1: 0.44, material: "cypress-wood" },
    { bone: "trunk-1", from: [0, 0.3, 0], to: [0, 4.6, 0], d0: 0.44, d1: 0.37, material: "cypress-wood" },
    { bone: "trunk-2", from: [0, 4.5, 0], to: [0, 9.1, 0], d0: 0.37, d1: 0.28, material: "cypress-wood" },
    { bone: "trunk-3", from: [0, 9, 0], to: [0, 13.6, 0], d0: 0.28, d1: 0.18, material: "cypress-wood" },
    { bone: "leader", from: [0, 13.5, 0], to: [0, 17.4, 0], d0: 0.18, d1: scale === 500 ? 0.08 : 0.04, material: "cypress-wood" },
  ];
  for (const s of SPRAYS) {
    const L = s.y1 - s.y0;
    const rho = 0.52 * crownR(s.y0 + 0.45 * L);
    const top: V3 = [rho * Math.cos(s.phi), s.y0 + 0.75 * L, rho * Math.sin(s.phi)];
    const d0 = s.d, d1 = Math.max(0.015, s.d * 0.3);
    out.push({ bone: s.id, from: [0, s.y0 + 0.05, 0], to: top, d0, d1, material: "cypress-wood", slender: true });
  }
  out.push({ bone: "spire", from: [0, 15.45, 0], to: [0, 17.7, 0], d0: 0.07, d1: 0.02, material: "cypress-wood", slender: true });
  return out;
}
const FEAT: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };

const DEC = (en: string, de: string): Text => t(en, de);

export function species(): Species {
  const s50Sprays = allSprays(22, 0.2, 6, 0.05);
  const s100Sprays = allSprays(16, 0.32, 4, 0.035);
  const parts50: Part[] = [...stemParts(20, 0.45, true), ...SPRAYS.map((s, i) => leaf(`foliage-${s.id}`, s.id, s.region, s50Sprays[i]!)), leaf("foliage-spire", "spire", "foliage-sun", spire(20, 0.12, 0.08))];
  const parts100: Part[] = [...stemParts(14, 0.8, false), ...SPRAYS.map((s, i) => leaf(`foliage-${s.id}`, s.id, s.region, s100Sprays[i]!)), leaf("foliage-spire", "spire", "foliage-sun", spire(14, 0.2, 0.06))];
  const tierSleeve = (tiers: number[], dy: number, nT: number, sm: number, extra: Mesh[] = []): Mesh => sleeve([...SPRAYS.filter((s) => tiers.includes(s.tier)).map((s) => s50Sprays[SPRAYS.indexOf(s)]!), ...extra], 0, 0, dy, nT, sm, 0.2);
  const spire50 = spire(20, 0.12, 0.08);
  const env50 = bounds(parts50.map((p) => p.mesh));
  const s200Sprays = allSprays(10, 0.6, 0, 0);
  const parts200: Part[] = [...stemParts(12, 1.5, false), ...SPRAYS.map((s, i) => leaf(`foliage-${s.id}`, s.id, s.region, s200Sprays[i]!)), leaf("foliage-spire", "spire", "foliage-sun", spire(10, 0.4, 0.04))];
  const parts500: Part[] = fitExtent([
    bark("stem-root-plate", "root-plate", stem(0, 0, [0, 0.12, 0.42], stemR, flare, 10, false)),
    bark("stem-trunk-1", "trunk-1", stem(0, 0, steps(0.3, 9.1, 2.2), stemR, () => 1, 10, false)),
    bark("stem-trunk-3", "trunk-3", stem(0, 0, steps(9, 17.6, 2.2), stemR, () => 1, 10, true)),
    leaf("crown-tier-1", "trunk-1", "foliage-shade", tierSleeve([0], 0.6, 12, 1)),
    leaf("crown-tier-2", "trunk-1", "foliage-shade", tierSleeve([1], 0.6, 12, 1)),
    leaf("crown-tier-3", "trunk-2", "foliage-sun", tierSleeve([2], 0.6, 12, 1)),
    leaf("crown-tier-4", "trunk-2", "foliage-sun", tierSleeve([3], 0.6, 12, 1)),
    leaf("crown-tier-5", "trunk-3", "foliage-sun", tierSleeve([4], 0.6, 12, 1)),
    leaf("crown-tier-6", "trunk-3", "foliage-sun", tierSleeve([5], 0.6, 12, 1)),
    leaf("crown-tier-7", "leader", "foliage-sun", tierSleeve([6], 0.6, 12, 1, [spire50])),
  ], env50);

  const sprayIds = SPRAYS.map((s) => s.id);
  const sunEmit = ["spray-3-1", "spray-4-2", "spray-5-3", "spray-6-1", "spray-7-2", "spire"];
  const shadeEmit = ["spray-1-1", "spray-1-3", "spray-2-2", "spray-2-4", "spray-1-5"];
  const anchorOf = (bone: string): V3 => {
    if (bone === "spire") return [0, 16.6, 0.28];
    const s = SPRAYS.find((x) => x.id === bone)!;
    const L = s.y1 - s.y0;
    const R = crownR(s.y0 + 0.4 * L);
    return [0.8 * R * Math.cos(s.phi), s.y0 + 0.4 * L, 0.8 * R * Math.sin(s.phi)];
  };
  const emitterSolids = (scale: Scale, all: boolean): ScaleDesign["emitterSolids"] => {
    const pick = (ids: string[]) => (all ? ids : ids.slice(0, 1));
    const r = 0.1 + 2 * FEAT[scale];
    return [
      ...pick(sunEmit).map((b) => ({ emitter: "transpiration-sun", region: "foliage-sun", bone: b, at: anchorOf(b), radii: [r, r * 0.6, r] as V3, material: "cypress-foliage-sun" })),
      ...pick(shadeEmit).map((b) => ({ emitter: "transpiration-shade", region: "foliage-shade", bone: b, at: anchorOf(b), radii: [r, r * 0.6, r] as V3, material: "cypress-foliage-shade" })),
    ];
  };

  const trunkJoints = (r: number) => [["trunk-1", 0.3], ["trunk-2", 4.5], ["trunk-3", 9], ["leader", 13.5]].map(([b, y]) => ({ bone: b as string, at: [0, y as number, 0] as V3, r }));
  const trunkMasses = (r: number, which: string[]) => which.map((b) => { const bb = BONES.find((x) => x.id === b)!; return { bone: b, member: b, at: [0, bb.o[1] + bb.length / 2, 0] as V3, r, material: "cypress-wood" }; });

  const scales: Record<Scale, ScaleDesign> = {
    50: {
      parts: parts50,
      pens: { body: 0.25, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 },
      marks: [
        { spacingM: 0.3, marginM: 0.055, groups: isFoliage, fn: foliageMark(0.13), views: ["elevation", "front", "plan"] },
        { spacingM: 0.09, marginM: 0.035, groups: isBark, fn: barkMark(0.16), views: ["elevation", "front"] },
      ],
      decisions: {
        silhouette: DEC("At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: a straight stem with a fluted, flared foot that shows below the lowest sprays, and the narrow column of thirty-one fastigiate branch sprays in seven tiers plus the spire, each spray a tall flame with a rounded foot and a pointed top, layered by its distance from the viewer, so the column reads with its vertical grooves and its scalloped edge.", "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: ein gerader Stamm mit gerilltem, ausgestelltem Fuß, der unter den untersten Zweigen sichtbar ist, und die schmale Säule aus einunddreißig aufrechten Zweigbüscheln in sieben Etagen und der Spitze, jedes Büschel eine hohe Flamme mit gerundetem Fuß und spitzem Ende, nach ihrem Abstand zum Betrachter geschichtet, sodass die Säule mit ihren senkrechten Rinnen und ihrem gewellten Rand lesbar ist."),
        retained: DEC("Retained: every spray as its own outline, the spire, the stem foot with its flutes, upturned chevrons of the scale-leaf sprays inside each visible spray, fine furrows on the visible stem foot.", "Beibehalten: jedes Büschel als eigener Umriss, die Spitze, der Stammfuß mit seinen Rillen, aufwärts weisende Winkel der Schuppenblattzweige in jedem sichtbaren Büschel, feine Furchen am sichtbaren Stammfuß."),
        merged: DEC("Nothing is merged at this scale: the ridges of each spray stay in its own outline.", "Bei diesem Maßstab ist nichts zusammengefasst: Die Rippen jedes Büschels bleiben in seinem eigenen Umriss."),
        omitted: DEC("Omitted: single scale leaves, the cones and the branches inside the column, which are hidden by the dense foliage.", "Weggelassen: einzelne Schuppenblätter, die Zapfen und die Äste im Inneren der Säule, die das dichte Laub verdeckt."),
        thinElements: DEC("The branches of the sprays are not drawn: they are hidden in the foliage and exist as slender members of the structural model only.", "Die Äste der Büschel sind nicht gezeichnet: Sie liegen verdeckt im Laub und bestehen nur als schlanke Stäbe des Tragwerksmodells."),
        hatching: DEC("No hatch: the dark tone of the shaded lower tiers and the chevron texture carry the foliage.", "Keine Schraffur: Der dunkle Ton der beschatteten unteren Etagen und die Winkeltextur tragen das Laub."),
        styles: DEC("Light and dark line draw the spray outlines with the body pen and the chevrons with the texture pen; the solid styles fill the sunlit sprays mid-grey and the lower shaded tiers darker, the stem dark.", "Helle und dunkle Linie zeichnen die Büschelumrisse mit der Körperfeder und die Winkel mit der Texturfeder; die Flächenstile füllen die besonnten Büschel mittelgrau, die unteren beschatteten Etagen dunkler und den Stamm dunkel."),
        overlays: DEC("Thermal: every spray outline carries its foliage region (sun or shade), the stem its bark; structure: the stem pieces and the slender spray branches are the members.", "Thermik: Jeder Büschelumriss trägt seine Laubregion (Sonne oder Schatten), der Stamm seine Rinde; Tragwerk: die Stammstücke und die schlanken Büscheläste sind die Stäbe."),
      },
      volumeDecisions: [
        DEC("Body at 1:50 (smallest feature 7.5 mm): the stem in five pieces with a fluted, flared foot of 0.68 m and a breast-height diameter of 0.45 m tapering to the leader tip at 17.6 m; thirty-one fastigiate sprays in seven tiers, each a closed flame with an elliptic section and six lengthwise ridges that reaches in to the stem and overlaps its neighbours, and the four-lobed spire to 18 m; every spray follows its own bone, so sway never opens the column.", "Körper bei 1:50 (kleinstes Merkmal 7,5 mm): der Stamm in fünf Stücken mit gerilltem, ausgestelltem Fuß von 0,68 m und einem Brusthöhendurchmesser von 0,45 m, der sich bis zur Spitze des Gipfeltriebs in 17,6 m verjüngt; einunddreißig aufrechte Büschel in sieben Etagen, jedes eine geschlossene Flamme mit elliptischem Querschnitt und sechs Längsrippen, die bis an den Stamm reicht und die Nachbarn überlappt, und die vierlappige Spitze bis 18 m; jedes Büschel folgt seinem eigenen Knochen, sodass die Säule beim Schwingen nie aufreißt."),
        DEC("Thermal at 1:50: the stem pieces carry the bark region, the sprays of the two lowest tiers the shade foliage and all others with the spire the sun foliage, by reference to the body; eleven transpiration outlets sit on the skin of the sprays that carry the emitter anchors.", "Thermik bei 1:50: Die Stammstücke tragen die Rindenregion, die Büschel der zwei untersten Etagen das Schattenlaub und alle übrigen mit der Spitze das Sonnenlaub, als Verweis auf den Körper; elf Transpirationsauslässe sitzen auf der Haut der Büschel, die die Emitteranker tragen."),
        DEC("Structure at 1:50: the five stem pieces as tapered round members, every spray branch and the spire shoot as a slender member of its branch base diameter from the stem into its flame, the four stem joints, the root plate as the fixed support and the masses of the four stem pieces.", "Tragwerk bei 1:50: die fünf Stammstücke als verjüngte Rundstäbe, jeder Büschelast und der Spitzentrieb als schlanker Stab mit dem Durchmesser seines Astansatzes vom Stamm in seine Flamme, die vier Stammgelenke, der Wurzelteller als Einspannung und die Massen der vier Stammstücke."),
      ],
      members: members(50),
      joints: trunkJoints(0.2),
      masses: trunkMasses(0.12, ["trunk-1", "trunk-2", "trunk-3", "leader"]),
      supportR: 0.9,
      emitterSolids: emitterSolids(50, true),
    },
    100: {
      parts: parts100,
      pens: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 },
      marks: [{ spacingM: 0.62, marginM: 0.1, groups: isFoliage, fn: foliageMark(0.24), views: ["elevation", "front"] }],
      decisions: {
        silhouette: DEC("At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: the stem foot below the column and the thirty-one sprays with the spire, each as a smooth flame without ridges, layered by distance; the column keeps its grooves and scalloped edge.", "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: der Stammfuß unter der Säule und die einunddreißig Büschel mit der Spitze, jedes als glatte Flamme ohne Rippen, nach Abstand geschichtet; die Säule behält ihre Rinnen und ihren gewellten Rand."),
        retained: DEC("Retained: every spray as its own outline, the spire, the stem foot, a sparse chevron texture in the side and front elevations.", "Beibehalten: jedes Büschel als eigener Umriss, die Spitze, der Stammfuß, eine spärliche Winkeltextur in Seiten- und Vorderansicht."),
        merged: DEC("The lengthwise ridges of each spray are merged into its smooth flame outline.", "Die Längsrippen jedes Büschels sind in seinen glatten Flammenumriss aufgegangen."),
        omitted: DEC("Omitted: bark furrows, the plan texture, single scale leaves and cones.", "Weggelassen: Rindenfurchen, die Grundrisstextur, einzelne Schuppenblätter und Zapfen."),
        thinElements: DEC("The branches of the sprays are not drawn; they are slender structural members inside the foliage.", "Die Äste der Büschel sind nicht gezeichnet; sie sind schlanke Tragwerksstäbe im Laub."),
        hatching: DEC("No hatch; tones and the sparse chevrons carry the foliage.", "Keine Schraffur; Töne und die spärlichen Winkel tragen das Laub."),
        styles: DEC("Line styles draw the spray outlines with the body pen; solid styles fill sun sprays mid-grey, shade tiers darker, the stem dark.", "Die Linienstile zeichnen die Büschelumrisse mit der Körperfeder; die Flächenstile füllen Sonnenbüschel mittelgrau, Schattenetagen dunkler und den Stamm dunkel."),
        overlays: DEC("Thermal and structural overlays map onto the same spray outlines as at 1:50.", "Thermische und statische Überlagerungen liegen auf denselben Büschelumrissen wie bei 1:50."),
      },
      volumeDecisions: [
        DEC("Body at 1:100 (smallest feature 15 mm): the stem in five pieces without flutes, the thirty-one sprays as smooth elliptic flames meshed more coarsely (sixteen points round, stations every 0.32 m) with four faint ridges, and the spire.", "Körper bei 1:100 (kleinstes Merkmal 15 mm): der Stamm in fünf Stücken ohne Rillen, die einunddreißig Büschel als glatte elliptische Flammen, gröber vernetzt (sechzehn Punkte im Umfang, Stationen alle 0,32 m) mit vier schwachen Rippen, und die Spitze."),
        DEC("Thermal at 1:100: bark, sun and shade foliage by reference to the body; eleven transpiration outlets on the emitter sprays.", "Thermik bei 1:100: Rinde, Sonnen- und Schattenlaub als Verweis auf den Körper; elf Transpirationsauslässe auf den Emitterbüscheln."),
        DEC("Structure at 1:100: stem members, the slender spray branches, four stem joints, the fixed root plate and four stem masses.", "Tragwerk bei 1:100: Stammstäbe, die schlanken Büscheläste, vier Stammgelenke, der eingespannte Wurzelteller und vier Stammmassen."),
      ],
      members: members(100),
      joints: trunkJoints(0.22),
      masses: trunkMasses(0.13, ["trunk-1", "trunk-2", "trunk-3", "leader"]),
      supportR: 0.9,
      emitterSolids: emitterSolids(100, true),
    },
    200: {
      parts: parts200,
      pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
      decisions: {
        silhouette: DEC("At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: the stem foot under the column and the thirty-one sprays with the spire as smooth flames, layered by distance, so the column keeps its vertical grooves and its scalloped edge without any texture.", "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: der Stammfuß unter der Säule und die einunddreißig Büschel mit der Spitze als glatte Flammen, nach Abstand geschichtet, sodass die Säule ihre senkrechten Rinnen und ihren gewellten Rand ohne jede Textur behält."),
        retained: DEC("Retained: every spray as its own outline, the spire and the stem foot.", "Beibehalten: jedes Büschel als eigener Umriss, die Spitze und der Stammfuß."),
        merged: DEC("Merged: the ridges and the fine swelling of every spray into a plain flame of few points; the flutes of the foot into a plain flare.", "Zusammengefasst: die Rippen und die feine Schwellung jedes Büschels zu einer schlichten Flamme mit wenigen Punkten; die Rillen des Fußes zu einem schlichten Anlauf."),
        omitted: DEC("Omitted: every texture.", "Weggelassen: jede Textur."),
        thinElements: DEC("No thin element is drawn; the stem shows only below the column, 0.6 m wide, 3 mm on paper.", "Kein dünnes Element ist gezeichnet; der Stamm zeigt sich nur unter der Säule, 0,6 m breit, 3 mm auf dem Papier."),
        hatching: DEC("No hatch.", "Keine Schraffur."),
        styles: DEC("Line styles: body pen outlines only; solid styles: lower tiers darker than the sunlit ones, the stem dark.", "Linienstile: nur Umrisse mit der Körperfeder; Flächenstile: untere Etagen dunkler als die besonnten, der Stamm dunkel."),
        overlays: DEC("Thermal: every spray outline carries its foliage region, the stem its bark; structural overlays colour the stem and the spray members.", "Thermik: Jeder Büschelumriss trägt seine Laubregion, der Stamm seine Rinde; statische Überlagerungen färben den Stamm und die Büschelstäbe."),
      },
      volumeDecisions: [
        DEC("Body at 1:200 (smallest feature 30 mm): the stem in five plain pieces and the thirty-one sprays with the spire as smooth flames of ten points round with stations every 0.6 m, every spray on its own bone as at 1:50.", "Körper bei 1:200 (kleinstes Merkmal 30 mm): der Stamm in fünf schlichten Stücken und die einunddreißig Büschel mit der Spitze als glatte Flammen mit zehn Punkten im Umfang und Stationen alle 0,6 m, jedes Büschel wie bei 1:50 auf seinem eigenen Knochen."),
        DEC("Thermal at 1:200: stem bark, shade and sun foliage by reference; one transpiration outlet per emitter.", "Thermik bei 1:200: Stammrinde, Schatten- und Sonnenlaub als Verweis; ein Transpirationsauslass je Emitter."),
        DEC("Structure at 1:200: stem members and the slender spray branches, the fixed root plate and three stem masses.", "Tragwerk bei 1:200: Stammstäbe und die schlanken Büscheläste, der eingespannte Wurzelteller und drei Stammmassen."),
      ],
      members: members(200),
      joints: [],
      masses: trunkMasses(0.15, ["trunk-1", "trunk-2", "trunk-3"]),
      supportR: 0.9,
      emitterSolids: emitterSolids(200, false),
    },
    500: {
      parts: parts500,
      groups: onePerPart(parts500),
      pens: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
      decisions: {
        silhouette: DEC("At 1:500 (1 m is 2 mm) the cypress is a slim, pointed column 36 mm tall and 5 mm wide on a short stem foot, drawn as its seven stacked tiers, each the lobed envelope of its sprays, so the column keeps a hint of its grooves; the drawings are the true views of the 1:500 body.", "Bei 1:500 (1 m sind 2 mm) ist die Zypresse eine schlanke, spitze Säule von 36 mm Höhe und 5 mm Breite auf einem kurzen Stammfuß, gezeichnet als ihre sieben gestapelten Etagen, jede die gelappte Hülle ihrer Büschel, sodass die Säule eine Andeutung ihrer Rinnen behält; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers."),
        retained: DEC("Retained: the stem foot, the two shaded lower tiers and the five sunlit upper tiers as separate outlines.", "Beibehalten: der Stammfuß, die zwei beschatteten unteren und die fünf besonnten oberen Etagen als getrennte Umrisse."),
        merged: DEC("Merged: the sprays of every tier into one lobed tier mass, the spire into the top tier, the stem into two pieces.", "Zusammengefasst: die Büschel jeder Etage zu einer gelappten Etagenmasse, die Spitze in die oberste Etage, der Stamm in zwei Stücke."),
        omitted: DEC("Omitted: every texture and the single sprays.", "Weggelassen: jede Textur und die einzelnen Büschel."),
        thinElements: DEC("No thin element: the stem foot is 0.6 m wide, 1.2 mm on paper.", "Kein dünnes Element: Der Stammfuß ist 0,6 m breit, 1,2 mm auf dem Papier."),
        hatching: DEC("No hatch.", "Keine Schraffur."),
        styles: DEC("One body pen; solid styles shade the two lower tiers darker.", "Eine Körperfeder; die Flächenstile tönen die zwei unteren Etagen dunkler."),
        overlays: DEC("Thermal: the tier outlines carry their foliage regions; structural overlays colour the stem.", "Thermik: Die Etagenumrisse tragen ihre Laubregionen; statische Überlagerungen färben den Stamm."),
      },
      volumeDecisions: [
        DEC("Body at 1:500 (smallest feature 75 mm): the stem foot and the stem in two pieces, and seven tier masses, each the envelope sleeve of its sprays (largest radius per 0.6 m band and per twelfth of the round, dilated over one neighbour), the spire in the top one, stretched to the true extent of the 1:50 body; nothing floats, every tier encloses the stem.", "Körper bei 1:500 (kleinstes Merkmal 75 mm): der Stammfuß und der Stamm in zwei Stücken sowie sieben Etagenmassen, jede die Hüllmanschette ihrer Büschel (größter Radius je 0,6-m-Band und je Zwölftel des Umfangs, über einen Nachbarn geweitet), die Spitze in der obersten, auf die wahre Ausdehnung des 1:50-Körpers gestreckt; nichts schwebt, jede Etage umschließt den Stamm."),
        DEC("Thermal at 1:500: stem bark, the two shaded and five sunlit tiers by reference; one transpiration outlet per emitter.", "Thermik bei 1:500: Stammrinde, die zwei beschatteten und fünf besonnten Etagen als Verweis; ein Transpirationsauslass je Emitter."),
        DEC("Structure at 1:500: the stem members at their sections, the spray branches as slender members inside the tier masses, the fixed root plate and two stem masses.", "Tragwerk bei 1:500: die Stammstäbe mit ihren Querschnitten, die Büscheläste als schlanke Stäbe in den Etagenmassen, der eingespannte Wurzelteller und zwei Stammmassen."),
      ],
      members: members(500),
      joints: [],
      masses: trunkMasses(0.2, ["trunk-1", "trunk-3"]),
      supportR: 0.9,
      emitterSolids: emitterSolids(500, false),
    },
  };

  const sp: Species = {
    id: "italian-cypress",
    title: t("Italian Cypress", "Säulenzypresse"),
    summary: t(
      "A mature Italian cypress (Cupressus sempervirens 'Stricta') of 18 m: a straight stem with a fluted, flared foot carries a narrow, dense column of fastigiate branch sprays, 2.6 m across at a third of its height and running out into a pointed spire; the dark scale-leaf foliage clothes the stem almost to the ground.",
      "Eine ausgewachsene Säulenzypresse (Cupressus sempervirens 'Stricta') von 18 m: Ein gerader Stamm mit gerilltem, ausgestelltem Fuß trägt eine schmale, dichte Säule aus aufrechten Zweigbüscheln, 2,6 m breit in einem Drittel ihrer Höhe und in eine spitze Krone auslaufend; das dunkle Schuppenlaub bekleidet den Stamm fast bis zum Boden.",
    ),
    tags: ["tree", "evergreen", "conifer", "columnar", "mediterranean", "wind", "transpiration"],
    references: [
      { id: "caudullo-2016", citation: "Caudullo, G. and de Rigo, D. (2016). Cupressus sempervirens in Europe: distribution, habitat, usage and threats. In: San-Miguel-Ayanz, J. et al. (eds.), European Atlas of Forest Tree Species. Publications Office of the EU, Luxembourg, e01fe7d+ (habit, heights to 30 m, columnar 'Stricta' crown)." },
      { id: "farjon-2010", citation: "Farjon, A. (2010). A Handbook of the World's Conifers. Brill, Leiden. Cupressus sempervirens: tree to 35 m, crown columnar in cultivated forms, bark grey-brown, finely fissured, foliage of scale leaves on dense sprays." },
      { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5, green properties; Alaska yellow-cedar (Cupressaceae) used as the closest listed proxy for Cupressus: E 7.9 GPa, MOR 44 MPa." },
      { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m." },
      { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet snow on foliage." },
      { id: "mayhead-1973", citation: "Mayhead, G. J. (1973). Some drag coefficients for British forest trees derived from wind tunnel studies. Agricultural Meteorology 12, 123-130." },
      { id: "moore-maguire-2004", citation: "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203." },
      { id: "coutts-1986", citation: "Coutts, M. P. (1986). Components of tree stability in Sitka spruce on peaty gley soil. Forestry 59(2), 173-197 (root-plate rotation under turning moment)." },
      { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance." },
      { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
    ],
    notes: t(
      "Stem, sprays and spire are authored by hand after a free-standing 'Stricta' cypress of about 80 years in an Italian garden: seven tiers of fastigiate sprays, each a flame that reaches in to the stem, overlapping round the column. Masses are volume times green density for the stem (yellow-cedar proxy of the Wood Handbook) plus a crown bulk density of 6 kg per cubic metre of spray volume and the hidden branch wood of every spray. An evergreen has no winter-bare geometry; the poses are calm, fresh breeze, gale and wet snow.",
      "Stamm, Büschel und Spitze sind von Hand nach einer freistehenden, etwa 80-jährigen 'Stricta'-Zypresse in einem italienischen Garten gezeichnet: sieben Etagen aufrechter Zweigbüschel, jedes eine Flamme, die bis an den Stamm reicht und rund um die Säule überlappt. Massen sind Volumen mal Frischholzdichte für den Stamm (Proxy Nootka-Scheinzypresse des Wood Handbook) plus eine Kronenrohdichte von 6 kg je Kubikmeter Büschelvolumen und das verdeckte Astholz jedes Büschels. Ein Immergrüner hat keine winterkahle Geometrie; die Posen sind Ruhe, frische Brise, Sturm und Nassschnee.",
    ),
    materials: [
      { id: "cypress-bark", title: t("Grey-brown, finely fissured cypress bark", "Graubraune, fein rissige Zypressenrinde"), emissivity: 0.95, solarAbsorptivity: 0.75, densityKgM3: 500, conductivityWmK: 0.12, source: "authored-estimate", reference: "jones-2014" },
      { id: "cypress-foliage-sun", title: t("Cypress scale-leaf sprays, sunlit", "Zypressen-Schuppenlaub, besonnt"), emissivity: 0.97, solarAbsorptivity: 0.55, source: "authored-estimate", reference: "jones-2014" },
      { id: "cypress-foliage-shade", title: t("Cypress scale-leaf sprays, shaded lower tiers", "Zypressen-Schuppenlaub, beschattete untere Etagen"), emissivity: 0.97, solarAbsorptivity: 0.6, source: "authored-estimate", reference: "jones-2014" },
      { id: "cypress-wood", title: t("Green cypress wood", "Frisches Zypressenholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 750, conductivityWmK: 0.14, elasticModulusPa: 7900000000, strengthPa: 44000000, source: "literature", reference: "wood-handbook-2021" },
    ],
    bones: BONES,
    scales,
    woodDensity: { "cypress-bark": 750 },
    foliageDensity: { "cypress-foliage-sun": 6, "cypress-foliage-shade": 6 },
    hiddenWood: (bone: string) => {
      const s = SPRAYS.find((x) => x.id === bone);
      if (bone === "spire") return 750 * (Math.PI / 12) * 2.25 * (0.07 ** 2 + 0.07 * 0.02 + 0.02 ** 2);
      if (s === undefined) return 0;
      const L = 0.75 * (s.y1 - s.y0) * 1.05;
      const d1 = Math.max(0.015, s.d * 0.3);
      return 750 * (Math.PI / 12) * L * (s.d ** 2 + s.d * d1 + d1 ** 2);
    },
    regions: [],
    emitters: [
      { id: "transpiration-sun", region: "foliage-sun", title: t("Transpiration of the sunlit sprays (distributed over the column)", "Transpiration der besonnten Büschel (über die Säule verteilt)"), anchors: sunEmit.map((b) => ({ bone: b, at: anchorOf(b) })), diameterM: 1.2, source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-shade", region: "foliage-shade", title: t("Transpiration of the shaded lower sprays (distributed over the lower column)", "Transpiration der beschatteten unteren Büschel (über die untere Säule verteilt)"), anchors: shadeEmit.map((b) => ({ bone: b, at: anchorOf(b) })), diameterM: 1.2, source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    thermalEnvironment: { airTempC: 30, meanRadiantTempC: 33, relativeHumidity: 0.4, windMS: 2, windDirectionDeg: 0, solarWM2: 850, pressurePa: 101325, diffuseFraction: 0.12 },
    thermalAssumptions: [],
    structureAssumptions: [],
    poses: [],
    clips: [],
    symbol: {
      title: t("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
      description: t("A conventional sign of the cypress for site and landscape plans: the outline of the true extent of the column and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Zeichen der Zypresse für Lage- und Freiraumpläne: der Umriss der wahren Ausdehnung der Säule und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
      note: t("Model metres at true size: the outline is the extent of the column in each view, the filled shape the stem (plan: the stem section at the foot).", "Modellmeter in wahrer Größe: Der Umriss ist die Ausdehnung der Säule in jeder Ansicht, die gefüllte Form der Stamm (Grundriss: der Stammquerschnitt am Fuß)."),
    },
    defaultScale: 200,
    symbolStemD: 0.46,
    distinction: t("A slim 18 m column of fastigiate scale-leaf sprays only 2.6 m across, clothed almost to the ground, the narrowest evergreen of the archive next to the deciduous Lombardy poplar.", "Eine schlanke, 18 m hohe Säule aus aufrechten Schuppenblattzweigen von nur 2,6 m Breite, fast bis zum Boden belaubt, der schmalste Immergrüne des Archivs neben der laubabwerfenden Pyramidenpappel."),
    massSource: { wood: "authored-estimate", foliage: "authored-estimate", reference: "wood-handbook-2021" },
  };
  return sp;
}

export const SPRAY_IDS = SPRAYS.map((s) => s.id);
export const STEM_BONES = ["trunk-1", "trunk-2", "trunk-3", "leader"];
export { H, crownR, stemR, regionArea, sec, add, r1 };

/** 🌡️ The physical scenario of the cypress: thermal regions (bark area from the 1:50 body, leaf areas from the crown leaf-area density), poses with EN 1991-1-4 wind and EN 1991-1-3 wet snow, the bending they cause, and the three clips. */
export function physics(sp: Species, ctx: Ctx): Physics {
  const stats = foliageStats(sp, 0.15);
  const foliageVol = (region: string) => sp.scales[50].parts.filter((p) => p.region === region).reduce((s, p) => s + volume(p.mesh), 0);
  const vSun = foliageVol("foliage-sun"), vShade = foliageVol("foliage-shade");
  const leafTotal = 160;
  const plan = projectedArea(sp.scales[50].parts.filter((p) => p.kind === "foliage").map((p) => p.mesh), "plan", 0.03);
  const aSun = r1((leafTotal * vSun) / (vSun + vShade)), aShade = r1(leafTotal - (leafTotal * vSun) / (vSun + vShade));
  const regions = [
    { id: "bark", title: t("Stem bark, grey-brown and finely fissured", "Stammrinde, graubraun und fein rissig"), kind: "bark", model: "passive", material: "cypress-bark", areaM2: r2(regionArea(sp, "bark")), areaBasis: "total", characteristicLengthM: 0.4, geometry: "cylinder", skyViewFactor: 0.15, layers: [], heatCapacityJM2K: 60000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "foliage-sun", title: t("Scale-leaf sprays of the sunlit column", "Schuppenlaub der besonnten Säule"), kind: "foliage", model: "leaf", material: "cypress-foliage-sun", areaM2: aSun, areaBasis: "exchange", characteristicLengthM: 0.003, geometry: "cylinder", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 1300, leaf: { stomatalConductanceMaxMS: 0.0022, leafAreaIndex: r2(aSun / plan), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    { id: "foliage-shade", title: t("Scale-leaf sprays of the shaded lower column", "Schuppenlaub der beschatteten unteren Säule"), kind: "foliage", model: "leaf", material: "cypress-foliage-shade", areaM2: aShade, areaBasis: "exchange", characteristicLengthM: 0.003, geometry: "cylinder", skyViewFactor: 0.35, layers: [], heatCapacityJM2K: 1300, leaf: { stomatalConductanceMaxMS: 0.0016, leafAreaIndex: r2(aShade / plan), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  ];
  const sprayTitle = (bone: string): Text => {
    const b = sp.bones.find((x) => x.id === bone)!;
    return t(`Wind drag on ${b.title.en.charAt(0).toLowerCase()}${b.title.en.slice(1)}`, `Winddruck auf ${b.title.de}`);
  };
  const stemTitle = t("Wind drag on the bare stem foot", "Winddruck auf den kahlen Stammfuß");
  const cases = {
    calm: { id: "calm", v10: 1.5, cd: 0.55, source: "literature", reference: "en-1991-1-4" },
    windy: { id: "windy", v10: 8, cd: 0.45, source: "literature", reference: "en-1991-1-4" },
    storm: { id: "storm", v10: 22, cd: 0.32, source: "literature", reference: "mayhead-1973" },
  };
  const stemLoad = { bone: "trunk-1", toM: 0.3, d: 0.46, title: stemTitle };
  const E = 7.9e9, kRoot = 2.2e6;
  const limit = (b: BoneSpec) => (b.joint.type === "revolute" ? b.joint.maxDeg * 0.95 : 0);
  const wl = Object.fromEntries(Object.entries(cases).map(([k, c]) => [k, windLoads(sp, stats, c, stemLoad, sprayTitle)]));
  const bonesOf = (k: string) => bend(sp, wl[k]!.forces, E, kRoot, "trunk-1", limit);
  const windyBones = bonesOf("windy");
  const stormBones = bonesOf("storm");
  const snowForces = stats.map((s) => ({ bone: s.bone, at: s.c, fx: 0, fy: -12 * G * s.planM2 }));
  const snowBones = bend(sp, snowForces, E * 0.35, kRoot, "trunk-1", limit);
  const snowLoads = stats.map((s) => {
    const local = toLocal(frames(sp.bones)[s.bone]!, s.c);
    const b = sp.bones.find((x) => x.id === s.bone)!;
    return { id: `snow-${s.bone}`, kind: "snow", title: t(`Wet snow on ${b.title.en.charAt(0).toLowerCase()}${b.title.en.slice(1)}`, `Nassschnee auf ${b.title.de}`), bone: s.bone, x: r3(local[0]), y: r3(local[1]), fxN: 0, fyN: r1(-12 * G * s.planM2), source: "authored-estimate", reference: "en-1991-1-3" };
  });
  const totalF = (k: string) => r1(wl[k]!.forces.reduce((s, f) => s + f.fx, 0) / 1000);
  const baseM = (k: string) => r1(wl[k]!.forces.reduce((s, f) => s + f.fx * f.at[1], 0) / 1000);
  const tr = (sunB: number, sunS: number, shade: number, wet?: number) => ({
    bark: { exposedFraction: 1, sunlitFraction: sunB, ...(wet === undefined ? {} : { wettedness: wet * 0.6 }) },
    "foliage-sun": { exposedFraction: 1, sunlitFraction: sunS, ...(wet === undefined ? {} : { wettedness: wet }) },
    "foliage-shade": { exposedFraction: 1, sunlitFraction: shade, ...(wet === undefined ? {} : { wettedness: wet }) },
  });
  const poses = [
    { id: "calm", title: t("Calm summer day", "Windstiller Sommertag"), description: t(`Light air of 1.5 m/s at 10 m on a hot Mediterranean afternoon: the column stands still; cd 0.55 on the frontal area of the sprays gives ${totalF("calm")} kN in all.`, `Leiser Zug von 1,5 m/s in 10 m Höhe an einem heißen Mittelmeernachmittag: Die Säule steht still; cd 0,55 auf die Anströmfläche der Büschel ergibt insgesamt ${String(totalF("calm")).replace(".", ",")} kN.`), bones: {}, ground: "authored", contacts: [], thermal: binding(tr(0.25, 0.85, 0.2), { "transpiration-sun": 0.8, "transpiration-shade": 0.8 }), loads: wl.calm!.loads },
    { id: "windy", title: t("Fresh breeze", "Frische Brise"), description: t(`Beaufort 5, 8 m/s at 10 m (power-law profile, exponent 0.22), cd 0.45 on the frontal area of every spray with an optical porosity of 15 %: ${totalF("windy")} kN on the column and a base moment of about ${baseM("windy")} kN m; stem and root plate bend by beam theory.`, `Beaufort 5, 8 m/s in 10 m Höhe (Potenzprofil, Exponent 0,22), cd 0,45 auf die Anströmfläche jedes Büschels bei 15 % optischer Porosität: ${String(totalF("windy")).replace(".", ",")} kN auf die Säule und ein Fußmoment von etwa ${String(baseM("windy")).replace(".", ",")} kNm; Stamm und Wurzelteller biegen sich nach der Balkentheorie.`), bones: windyBones, ground: "authored", contacts: [], thermal: binding(tr(0.25, 0.85, 0.2), { "transpiration-sun": 1, "transpiration-shade": 1 }), loads: wl.windy!.loads },
    { id: "storm", title: t("Gale", "Sturm"), description: t(`Beaufort 9, 22 m/s at 10 m, cd 0.32 after the sprays streamline (Mayhead 1973): ${totalF("storm")} kN and a base moment of about ${baseM("storm")} kN m; the column leans and its top swings out, rain wets the foliage.`, `Beaufort 9, 22 m/s in 10 m Höhe, cd 0,32 nach dem Anlegen der Büschel (Mayhead 1973): ${String(totalF("storm")).replace(".", ",")} kN und ein Fußmoment von etwa ${String(baseM("storm")).replace(".", ",")} kNm; die Säule neigt sich, ihre Spitze schwingt aus, Regen nässt das Laub.`), bones: stormBones, ground: "authored", contacts: [], thermal: binding(tr(0.05, 0.3, 0.05, 0.4), { "transpiration-sun": 0.35, "transpiration-shade": 0.35 }), loads: wl.storm!.loads },
    { id: "wet-snow", title: t("Wet snow on the column", "Nassschnee auf der Säule"), description: t("A rare snowfall: 12 kg/m2 of wet snow on the plan area of every spray (order of magnitude after EN 1991-1-3) bends the fastigiate sprays outwards, the typical way a cypress column opens under snow; no wind.", "Ein seltener Schneefall: 12 kg/m2 Nassschnee auf der Grundrissfläche jedes Büschels (Größenordnung nach EN 1991-1-3) biegt die aufrechten Büschel nach außen, so wie sich eine Zypressensäule unter Schnee typisch öffnet; kein Wind."), bones: snowBones, ground: "authored", contacts: [], thermal: binding(tr(0, 0, 0, 0.6), { "transpiration-sun": 0.05, "transpiration-shade": 0.05 }), loads: snowLoads },
  ];
  const loadIds = (wl.windy!.loads as { id: string }[]).map((l) => l.id);
  const clips = [
    swayClip(sp, windyBones, 2.4, 0.6, (b) => (STEM_BONES.includes(b.id) ? 0.03 : 0.25), t("Wind sway", "Windschwingung"), t("A deterministic 2.4 s sway cycle (first mode of a slender column, about 0.42 Hz after Moore and Maguire 2004) about the fresh-breeze lean: the stem bends gently and every spray swings up to 0.3 degrees behind it.", "Ein deterministischer Schwingungszyklus von 2,4 s (erste Eigenform einer schlanken Säule, etwa 0,42 Hz nach Moore und Maguire 2004) um die Neigung der frischen Brise: Der Stamm biegt sich sanft, jedes Büschel schwingt bis zu 0,3 Grad hinterher."), H),
    gustClip(windyBones, loadIds, t("Gust", "Windböe"), t("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak velocity pressure with Iv 0.25) and decay; the column leans and recovers.", "Eine einzelne Böe über 8 s: Die Windlastspuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzengeschwindigkeitsdruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Säule neigt sich und richtet sich wieder auf.")),
    diurnalClip({ "transpiration-sun": [0.03, 1], "transpiration-shade": [0.03, 0.9] }, { bark: 0.25, "foliage-sun": 0.85, "foliage-shade": 0.2 }, t("Day of transpiration", "Transpirationstag"), t("A 24 h summer day compressed to 24 s (1 s = 1 h), calm weather: the sunlit fraction of every region and the transpiration strength follow the sun, the stomata are nearly closed at night; leaf and bark temperatures are not authored, the thermal solver finds them.", "Ein 24-h-Sommertag auf 24 s gerafft (1 s = 1 h) bei ruhigem Wetter: Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt- und Rindentemperaturen werden nicht vorgegeben, der thermische Löser ermittelt sie.")),
  ];
  const thermalAssumptions = [
    t(`Leaf regions use one-sided leaf area: 160 m2 for the whole column (a leaf-area density of about 3.5 m2 per m3 of crown, authored estimate), split between sun and shade sprays by their volume; the leaf area index is that area over the crown ground projection of ${r1(plan)} m2, high because the column is tall and narrow. The maximum stomatal conductance is an authored estimate for a drought-adapted Mediterranean conifer, guided by Jones (2014). Photosynthesis type: C3.`, `Blattregionen verwenden die einseitige Blattfläche: 160 m2 für die ganze Säule (eine Blattflächendichte von etwa 3,5 m2 je m3 Krone, Autorenschätzung), nach ihrem Volumen auf Sonnen- und Schattenbüschel verteilt; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden von ${String(r1(plan)).replace(".", ",")} m2, hoch, weil die Säule hoch und schmal ist. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für einen trockenheitsangepassten Nadelbaum des Mittelmeerraums, angelehnt an Jones (2014). Photosynthesetyp: C3.`),
    t("The bark is passive; its areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of about 2 cm, an authored estimate guided by the Wood Handbook; its area is the outer surface of the 1:50 stem. No surface temperature is authored.", "Die Rinde ist passiv; ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von etwa 2 cm, eine Autorenschätzung angelehnt an das Wood Handbook; ihre Fläche ist die Außenfläche des 1:50-Stamms. Es werden keine Oberflächentemperaturen vorgegeben."),
    t("Transpiration emitters release the latent flux of their foliage region as vapour, spread over five or six anchors on the outer sprays; the direction is the vertical in the frame of the first anchor's bone, since the release is diffuse.", "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf fünf oder sechs Anker auf den äußeren Büscheln; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist."),
  ];
  const structureAssumptions = [
    t(`One rigid body per evaluated pose, fixed at the root plate. Wind drag acts at the foliage centroid of every spray and on the bare stem foot as a line load. Pose angles are quasi-static: each joint turns by M L / (2 E I) of its section (E 7.9 GPa, yellow-cedar proxy), the root plate by the base moment over an anchorage stiffness of 2.2 MN m/rad (authored estimate after Coutts 1986); snow bends a spray as a bundle of shoots with 35 % of the stiffness of its solid base section. Dynamic amplification and root-soil failure are not modelled.`, `Ein starrer Körper je ausgewerteter Pose, am Wurzelteller eingespannt. Winddruck wirkt im Laubschwerpunkt jedes Büschels und am kahlen Stammfuß als Linienlast. Die Posenwinkel sind quasistatisch: Jedes Gelenk dreht sich um M L / (2 E I) seines Querschnitts (E 7,9 GPa, Proxy Nootka-Scheinzypresse), der Wurzelteller um das Fußmoment geteilt durch eine Verankerungssteifigkeit von 2,2 MNm/rad (Autorenschätzung nach Coutts 1986); Schnee biegt ein Büschel als Bündel von Trieben mit 35 % der Steifigkeit seines vollen Ansatzquerschnitts. Dynamische Überhöhung und Wurzel-Boden-Versagen werden nicht modelliert.`),
  ];
  return { regions, poses, clips, thermalAssumptions, structureAssumptions, supportFootprintM: 1.4 };
}
