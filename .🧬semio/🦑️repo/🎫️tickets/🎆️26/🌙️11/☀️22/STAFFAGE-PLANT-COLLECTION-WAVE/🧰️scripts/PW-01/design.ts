/** 🧾️ The design contract of a PW-01 maple (species geometry, physics, poses, texts) and the shared bilingual text builders and bark marks every species uses.
 * @see ./asset.ts
 * @see ./norway-maple.ts
 * @see ./field-maple.ts
 * @see ./sycamore-maple.ts */
import type { Tube, View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-small-leaved-lime-kit.ts";
import type { Json, P2 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";
import type { Scale, Species, Text } from "./model.ts";

export type PoseSpec = {
  id: string; title: Text; description: Text; v10: number;
  lean: [number, number, number, number]; droop?: [number, number];
  sun: [number, number, number, number]; transp: [number, number]; wet?: number;
  extra?: { kind: string; load: "snow" | "imposed"; kgPerM2: number; basis: "plan" | "leaf"; title: Text; source: string; reference: string };
};

export type Design = {
  tree: Species;
  core: { top: string; low: [string, number]; high: [string, number] };
  wood: { densityKgM3: number; trunkHeatCapacity: number; limbHeatCapacity: number; limbLengthM: number };
  leaf: { lai: number; sunShare: number; massPerAreaKgM2: number; lengthM: number; gsSun: number; gsShade: number; size: number; spacingMm: number; most: number; lobes: number; sharp: number; sinus: number };
  samara?: { share: number; span: number; angleDeg: number; most: number };
  bark: { marks: (view: View, trunk: Tube, scale: Scale, rand: () => number) => P2[][]; closed: boolean };
  wind: { cd: number; porosity: number; periodS: number; gustPeak: number };
  rootPlateM: number;
  environment: Json;
  poses: PoseSpec[];
  materials: Json[];
  references: Json[];
  texts: {
    volume: (s: Scale, n: { wood: number; lobes: number; tufts: number; sides: number; outlets: number }) => Text[];
    drawing: (s: Scale, n: Record<string, number>) => Json;
    symbol: Text; sway: Text; gust: Text; diurnal: Text;
    regions: { trunk: Text; limb: Text; sun: Text; shade: Text };
    thermalAssumptions: Text[]; structureAssumptions: Text[];
  };
};

export const t = (en: string, de: string): Text => ({ en, de });
const projectP = (view: View, p: [number, number, number]): P2 => (view === "elevation" ? { x: p[0], y: p[1] } : view === "front" ? { x: p[2], y: p[1] } : { x: p[0], y: p[2] });

/** 🪵️ Paper-plane frame of the visible face of a trunk: point at height y and across-offset s (−1 left edge … 1 right edge of the outline). */
function trunkPoint(view: View, trunk: Tube, y: number, s: number): P2 {
  const pts = trunk.points;
  let i = 0;
  while (i + 2 < pts.length && pts[i + 1]!.p[1] < y) i++;
  const [a, b] = [pts[i]!, pts[i + 1]!];
  const f = Math.max(0, Math.min(1, (y - a.p[1]) / (b.p[1] - a.p[1] || 1e-9)));
  const c = projectP(view, [a.p[0] + (b.p[0] - a.p[0]) * f, y, a.p[2] + (b.p[2] - a.p[2]) * f]);
  const r = a.r + (b.r - a.r) * f;
  return { x: c.x + s * r, y: c.y };
}

/** 🌲️ Long interlacing shallow ridges (Norway maple): wavy vertical strokes across the visible face. */
export function ridgeMarks(view: View, trunk: Tube, scale: Scale, rand: () => number): P2[][] {
  if (view === "plan") return [];
  const y0 = trunk.points[0]!.p[1] + 0.15, y1 = trunk.points.at(-1)!.p[1] - 0.1;
  const lanes = scale === 50 ? [-0.62, -0.38, -0.14, 0.1, 0.34, 0.58] : [-0.5, -0.05, 0.42];
  const out: P2[][] = [];
  for (const s0 of lanes) {
    let y = y0 + rand() * 0.3;
    while (y < y1 - 0.3) {
      const len = (scale === 50 ? 0.45 : 0.8) + rand() * 0.5;
      const ring: P2[] = [];
      const w = (rand() - 0.5) * 0.12;
      for (let k = 0; k <= 4; k++) ring.push(trunkPoint(view, trunk, Math.min(y1, y + (len * k) / 4), s0 + w * Math.sin(k * 1.3 + s0 * 5)));
      out.push(ring);
      y += len + (scale === 50 ? 0.18 : 0.35) + rand() * 0.2;
    }
  }
  return out;
}

/** 🪨️ Irregular flaking plates (old sycamore): small closed polygons across the visible face. */
export function plateMarks(view: View, trunk: Tube, scale: Scale, rand: () => number): P2[][] {
  if (view === "plan") return [];
  const y0 = trunk.points[0]!.p[1] + 0.2, y1 = trunk.points.at(-1)!.p[1] - 0.2;
  const out: P2[][] = [];
  const rows = scale === 50 ? 0.42 : 0.8;
  for (let y = y0; y < y1; y += rows + rand() * 0.15) {
    const lanes = scale === 50 ? [-0.5, -0.1, 0.3] : [-0.3, 0.25];
    for (const s0 of lanes) {
      const s = s0 + (rand() - 0.5) * 0.18;
      const h = (scale === 50 ? 0.16 : 0.26) + rand() * 0.08;
      const w = (scale === 50 ? 0.16 : 0.22) + rand() * 0.06;
      const ring = [[-1, -0.6], [-0.3, -1], [0.7, -0.8], [1, 0.1], [0.5, 1], [-0.6, 0.8]].map(([u, v]) => trunkPoint(view, trunk, y + v! * h, s + u! * w));
      out.push(ring);
    }
  }
  return out;
}

/** 🧱️ Corky oblong plates of a field maple: short vertical strokes staggered like brickwork. */
export function corkMarks(view: View, trunk: Tube, scale: Scale, rand: () => number): P2[][] {
  if (view === "plan") return [];
  const y0 = trunk.points[0]!.p[1] + 0.1, y1 = trunk.points.at(-1)!.p[1] - 0.1;
  const out: P2[][] = [];
  const step = scale === 50 ? 0.26 : 0.45;
  let row = 0;
  for (let y = y0; y < y1 - step * 0.6; y += step, row++) {
    const lanes = scale === 50 ? (row % 2 ? [-0.55, -0.05, 0.45] : [-0.3, 0.2, 0.62]) : row % 2 ? [-0.4, 0.35] : [0];
    for (const s0 of lanes) {
      const s = s0 + (rand() - 0.5) * 0.08;
      out.push([trunkPoint(view, trunk, y, s), trunkPoint(view, trunk, y + step * 0.62, s + (rand() - 0.5) * 0.05)]);
    }
  }
  return out;
}

const deNum = (x: number, d = 1): string => x.toFixed(d).replace(".", ",");

/** 📝️ Species names and features the shared texts speak of. */
export type Voice = { name: Text; latin: string; habit: Text; leaf: Text; bark: Text; fruit: Text; crown: Text };

/** 📝️ The volume decisions of one scale. */
export function volumeTexts(v: Voice, s: Scale, n: { wood: number; lobes: number; tufts: number; sides: number; outlets: number }): Text[] {
  const feat = (0.15 * s).toFixed(1);
  const featDe = deNum(0.15 * s);
  const base = [
    t(`Body at 1:${s}: ${n.wood} swept wood solids (root collar${s === 500 ? "" : ", buttress roots"}, stem, leaders and scaffold limbs${n.sides ? `, ${n.sides} side branches` : ""}) and ${n.lobes} foliage lobes laid out on the crown envelope of the ${v.name.en.toLowerCase()} for this scale${n.tufts ? `, ${n.tufts} of them small leaf tufts standing proud of the crown surface` : ""}.`,
      `Körper bei 1:${s}: ${n.wood} gezogene Holzkörper (Wurzelhals${s === 500 ? "" : ", Wurzelanläufe"}, Stamm, Stammverlängerung und Gerüstäste${n.sides ? `, ${n.sides} Seitenäste` : ""}) und ${n.lobes} Laubballen auf der Kronenhülle des ${v.name.de} für diesen Maßstab${n.tufts ? `, davon ${n.tufts} kleine Blattbüschel, die über die Kronenoberfläche hinausstehen` : ""}.`),
    t(`Every wood section and lobe is at least the smallest feature of 1:${s} (${feat} mm) thick; the crown rings, lobe sizes and counts are stated for this scale, not reduced from a finer one.`,
      `Jeder Holzquerschnitt und jeder Laubballen ist mindestens so dick wie das kleinste Merkmal von 1:${s} (${featDe} mm); Kronenringe, Ballengrößen und -zahlen sind für diesen Maßstab festgelegt und nicht von einem feineren abgeleitet.`),
    t(`Thermal: every body solid by reference to its bark or leaf region, plus ${n.outlets} flush outlet lenses tagged with the transpiration emitters. Structure: one wood member per rigged stem, leader and limb under the bark, joints, the root-plate support and the centres of mass.`,
      `Thermik: jeder Körper als Verweis auf seine Rinden- oder Blattregion, dazu ${n.outlets} bündige Auslasslinsen mit den Transpirationsemittern. Tragwerk: je ein Holzstab unter der Rinde für jeden Stamm, jede Stammverlängerung und jeden Ast des Rigs, Gelenke, das Auflager am Wurzelteller und die Schwerpunkte.`),
  ];
  if (s === 500) base.push(t("At 1:500 all foliage lobes ride on the first leader, so the crown moves as one mass; the limbs stay inside it.", "Bei 1:500 hängen alle Laubballen an der ersten Stammverlängerung, sodass sich die Krone als eine Masse bewegt; die Äste bleiben in ihr."));
  return base;
}

/** 📝️ The drawing decisions of one scale. */
export function drawingTexts(v: Voice, s: Scale, n: Record<string, number>): Json {
  const m = (1000 / s).toFixed(0);
  const per = s === 500 ? t("one crown mass", "eine Kronenmasse") : t(`${n.foliage} foliage outlines, one per crown bone`, `${n.foliage} Laubumrisse, je Kronenknochen einer`);
  return {
    silhouette: t(`At 1:${s} (1 m is ${m} mm) the ${v.name.en.toLowerCase()} is drawn as the true views of its own 1:${s} body: ${v.crown.en}, over the stem and root collar.`, `Bei 1:${s} (1 m sind ${m} mm) ist der ${v.name.de} als wahre Ansichten seines eigenen Körpers 1:${s} gezeichnet: ${v.crown.de}, über Stamm und Wurzelhals.`),
    retained: t(`Retained: ${n.wood} wood outlines (one per rigged stem, leader and limb) and ${per.en}, painted in the depth order of each view.`, `Beibehalten: ${n.wood} Holzumrisse (je Stamm, Stammverlängerung und Ast des Rigs einer) und ${per.de}, in der Tiefenfolge jeder Ansicht gemalt.`),
    merged: s === 500 ? t("All sun and shade foliage is one crown outline with its scallops; the limbs lie under it.", "Das ganze Sonnen- und Schattenlaub ist ein Kronenumriss mit seinen Ausbuchtungen; die Äste liegen darunter.") : t("The lobes of one crown bone are one lobed outline; the side branches of a limb join its outline.", "Die Ballen eines Kronenknochens sind ein gelappter Umriss; die Seitenäste eines Astes gehören zu seinem Umriss."),
    omitted: s === 500 ? t("Omitted: lobe arcs, leaves, fruit and bark marks.", "Weggelassen: Ballenbögen, Blätter, Früchte und Rindenzeichen.") : s === 200 ? t("Omitted: single leaves, fruit and bark marks; side branches are not modelled at this scale.", "Weggelassen: einzelne Blätter, Früchte und Rindenzeichen; Seitenäste sind bei diesem Maßstab nicht modelliert.") : s === 100 ? t("Omitted: single leaves and fruit; the bark is indicated by a few strokes.", "Weggelassen: einzelne Blätter und Früchte; die Rinde ist durch wenige Striche angedeutet.") : t("Nothing structural is omitted; leaves and fruit are drawn only where their crown mass is visible.", "Nichts Tragendes ist weggelassen; Blätter und Früchte sind nur dort gezeichnet, wo ihre Kronenmasse sichtbar ist."),
    thinElements: t(`Wood is never thinner than ${(0.093 * s).toFixed(0)} mm (about 0.19 mm on paper); finer twigs are part of the foliage masses.`, `Holz ist nie dünner als ${(0.093 * s).toFixed(0)} mm (etwa 0,19 mm auf dem Papier); feinere Zweige gehören zu den Laubmassen.`),
    hatching: s <= 100 ? t("The shaded inner crown carries a light 45 degree line hatch; no other hatching.", "Die beschattete Innenkrone trägt eine leichte Linienschraffur unter 45 Grad; sonst keine Schraffur.") : t("No hatching at this scale.", "Bei diesem Maßstab keine Schraffur."),
    styles: s === 50 ? t(`Light line: 0.3 mm outlines, 0.18 mm lobe arcs, 0.13 mm leaves (${v.leaf.en}), fruit and bark (${v.bark.en}). Dark solid: tonal masses with the strokes on top.`, `Helle Linie: Umrisse 0,3 mm, Ballenbögen 0,18 mm, Blätter (${v.leaf.de}), Früchte und Rinde (${v.bark.de}) 0,13 mm. Dunkle Fläche: getönte Massen mit den Strichen darüber.`) : s === 100 ? t("Light line: 0.25 mm outlines, 0.13 mm lobe arcs, 0.1 mm bark strokes. Dark solid: tonal masses.", "Helle Linie: Umrisse 0,25 mm, Ballenbögen 0,13 mm, Rindenstriche 0,1 mm. Dunkle Fläche: getönte Massen.") : s === 200 ? t("Light line: 0.2 mm outlines and 0.13 mm lobe arcs. Dark solid: tonal masses.", "Helle Linie: Umrisse 0,2 mm und Ballenbögen 0,13 mm. Dunkle Fläche: getönte Massen.") : t("Light line: 0.18 mm outlines only. Dark solid: one tonal crown over the stem.", "Helle Linie: nur Umrisse 0,18 mm. Dunkle Fläche: eine getönte Krone über dem Stamm."),
    overlays: s === 500 ? t("The crown shape carries both leaf regions (merged, area-weighted); stem and limbs carry their bark regions, so sway, loads and temperatures act on the drawn parts.", "Die Kronenform trägt beide Blattregionen (zusammengefasst, flächengewichtet); Stamm und Äste tragen ihre Rindenregionen, sodass Schwingung, Lasten und Temperaturen an den gezeichneten Teilen wirken.") : t("Every outline carries its own thermal region (stem bark, limb bark, sun or shade leaves) and rides on the bone that carries it, so sway, loads and temperatures act on the drawn parts.", "Jeder Umriss trägt seine eigene thermische Region (Stammrinde, Astrinde, Sonnen- oder Schattenblätter) und hängt am tragenden Knochen, sodass Schwingung, Lasten und Temperaturen an den gezeichneten Teilen wirken."),
  };
}

/** 📚️ The references shared by the three maples. */
export const REFERENCES: Json[] = [
  { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (green wood of the American maples as a guide); the European maple values are authored estimates." },
  { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, power-law profile." },
  { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet-snow accretion on a leafy crown." },
  { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
  { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance." },
  { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  { id: "roloff-2010", citation: "Roloff, A. (2010). Bäume Mitteleuropas: von Aspe bis Zirbelkiefer. Wiley-VCH, Weinheim. Habit, size, bark, leaves and fruit of Acer platanoides, A. pseudoplatanus and A. campestre." },
  { id: "mitchell-1974", citation: "Mitchell, A. (1974). A Field Guide to the Trees of Britain and Northern Europe. Collins, London. Heights, crown forms and samara angles of the European maples." },
  { id: "james-2006", citation: "James, K. R., Haritos, N. and Ades, P. K. (2006). Mechanical stability of trees under dynamic loads. American Journal of Botany 93(10), 1522-1530. Sway frequencies of open-grown trees." },
];

/** 🧱️ The materials of a maple; bark absorptivity and the green wood density are species values. */
export function materials(barkTitle: Text, barkAbs: number, densityKgM3: number, sunAbs: number): Json[] {
  return [
    { id: "maple-bark", title: barkTitle, emissivity: 0.95, solarAbsorptivity: barkAbs, densityKgM3: 600, conductivityWmK: 0.12, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "maple-bark-young", title: t("Bark of the limbs and branches", "Rinde der Äste und Zweige"), emissivity: 0.95, solarAbsorptivity: Math.round((barkAbs - 0.05) * 100) / 100, densityKgM3: 650, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "maple-leaf-sun", title: t("Sun leaves (green)", "Sonnenblätter (grün)"), emissivity: 0.97, solarAbsorptivity: sunAbs, source: "literature", reference: "jones-2014" },
    { id: "maple-leaf-shade", title: t("Shade leaves (effective)", "Schattenblätter (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.25, source: "authored-estimate", reference: "jones-2014" },
    { id: "maple-wood", title: t("Green maple wood", "Frisches Ahornholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3, conductivityWmK: 0.17, elasticModulusPa: 9.0e9, strengthPa: 5.5e7, source: "authored-estimate", reference: "wood-handbook-2021" },
  ];
}

/** 📝️ Thermal and structural assumptions shared by the maples. */
export function assumptions(v: Voice, gs: [number, number]): { thermal: Text[]; structure: Text[] } {
  return {
    thermal: [
      t(`Leaf regions use one-sided leaf area; the leaf area index is that area over the crown ground projection of the model. The maximum stomatal conductance (${(gs[0] * 1000).toFixed(1)} mm/s sun, ${(gs[1] * 1000).toFixed(1)} mm/s shade) is an authored estimate inside the typical broadleaf range of Jones (2014); no published species maximum is claimed. Photosynthesis type C3; night closure is the engine's business.`,
        `Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion des Modells. Die maximale Spaltöffnungsleitfähigkeit (${deNum(gs[0] * 1000)} mm/s Sonne, ${deNum(gs[1] * 1000)} mm/s Schatten) ist eine Autorenschätzung im typischen Laubholzbereich nach Jones (2014); es wird kein veröffentlichtes Artmaximum behauptet. Photosynthesetyp C3; das nächtliche Schließen ist Sache der Engine.`),
      t("Bark, stem and limbs are passive. Their areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times the depth a daily temperature wave reaches (a few centimetres), an authored estimate. Bark areas are the exposed lateral surfaces of the 1:50 wood model. No surface temperature is authored.",
        "Rinde, Stamm und Äste sind passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal der Tiefe, die eine Tageswelle der Temperatur erreicht (einige Zentimeter), eine Autorenschätzung. Die Rindenflächen sind die freien Mantelflächen des Holzmodells 1:50. Es werden keine Oberflächentemperaturen vorgegeben."),
      t("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors across the crown; sun leaves form the outer lobes, shade leaves the inner crown, and each pose authors the sunlit share of each region.",
        "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf mindestens fünf Anker über die Krone; die Sonnenblätter bilden die äußeren Ballen, die Schattenblätter die Innenkrone, und jede Pose gibt den besonnten Anteil jeder Region vor."),
      t(`Leaf colour, flowers and fruit are not geometric: a pose cannot hide shapes, so seasonal poses change the thermal binding and the loads only; ${v.fruit.en}`, `Laubfarbe, Blüten und Früchte sind nicht geometrisch: Eine Pose kann keine Formen ausblenden, saisonale Posen ändern daher nur die thermische Bindung und die Lasten; ${v.fruit.de}`),
    ],
    structure: [
      t("The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centroid of each foliage bone and on the stem as a line load, gravity on every bone mass. The base moment follows from statics at the posed geometry; root-soil failure and dynamic amplification are not modelled. Pose lean angles are authored estimates scaled with the drag, not a computed deflection.",
        "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck wirkt im Schwerpunkt jedes Laubknochens und am Stamm als Linienlast, Schwerkraft auf jede Knochenmasse. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen und dynamische Überhöhung werden nicht modelliert. Die Neigungswinkel der Posen sind Autorenschätzungen proportional zum Winddruck, keine berechnete Durchbiegung."),
      t("Wood masses are the volumes of the 1:50 wood model times the green density; sections are solid circles of green wood at the base of each bone (authored-estimate properties guided by the Wood Handbook); hollow or decayed stems are not represented. Foliage masses are leaf area times a fresh mass per area that includes petioles and fine twigs.",
        "Holzmassen sind die Volumina des Holzmodells 1:50 mal der Frischdichte; Querschnitte sind Vollkreise aus frischem Holz am Fuß jedes Knochens (Autorenschätzungen angelehnt an das Wood Handbook); hohle oder faule Stämme werden nicht abgebildet. Laubmassen sind Blattfläche mal einer Frischmasse je Fläche, die Blattstiele und Feinzweige einschließt."),
    ],
  };
}
