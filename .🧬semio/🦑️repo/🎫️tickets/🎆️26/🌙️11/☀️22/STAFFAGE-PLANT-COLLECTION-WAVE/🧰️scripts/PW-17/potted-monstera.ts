/** 🌴️ PW-17 potted monstera (Monstera deliciosa) on a coir moss pole: a climbing stem in two segments with nine leaves in two ranks, every number stated by hand below (node, azimuth, petiole, blade, slits and holes, pitch and roll); builds the whole `semio.staffage/v2` asset (rig, poses, clips, thermal, structure, four representations, four volumes, symbol).
 * @see ./lib.ts
 * @see ./leafy.ts
 * @see ./monstera-leaf.ts */
import { BAND, EXPORTS, FEATURE, FIXED, RigBuilder, SCALES, T, UP, VIEWS, add, azimuth, cross, decisions, dot, drag, frameOf, hull2, lin, massOf, mul, ordered, phase, polyArea, project, r3, r4, r5, revolute, roundSection, toWorld, unit, worldMeshes, type Frame, type Json, type P, type Placed, type Scale, type Text, type V3 } from "./lib.ts";
import { crown, plate, stalk } from "./leafy.ts";
import { monsteraBlade, type MonsteraBlade } from "./monstera-leaf.ts";
import { keep, potMaterials, potParts, potStructure, type PotSpec } from "./pot.ts";

const ID = "potted-monstera";

/** 🏺️ Cream-glazed stoneware pot 0.31 m high and 0.36 m across on a saucer: outline, shell with drainage hole and compost fill [u up from the floor, radius]. */
const POT: PotSpec = {
  saucer: [[0, 0.181], [0.005, 0.189], [0.03, 0.198], [0.036, 0.198], [0.036, 0.193], [0.016, 0.183]],
  pot: [[0.016, 0.14], [0.022, 0.146], [0.09, 0.157], [0.25, 0.172], [0.296, 0.176], [0.306, 0.18], [0.311, 0.177], [0.309, 0.163], [0.288, 0.1612]],
  shell: [[0.016, 0.02], [0.016, 0.14], [0.022, 0.146], [0.09, 0.157], [0.25, 0.172], [0.296, 0.176], [0.306, 0.18], [0.311, 0.177], [0.309, 0.163], [0.25, 0.158], [0.05, 0.13], [0.03, 0.126], [0.03, 0.02], [0.016, 0.02]],
  fill: [[0.03, 0.1255], [0.05, 0.1295], [0.25, 0.1575], [0.29, 0.1608]],
  soilU: 0.29, soilR: 0.1608, rimInnerR: 0.163, rimTopU: 0.311, potOuterR: 0.18, saucerR: 0.198, saucerTopU: 0.036,
  materials: { pot: "glazed-stoneware", saucer: "glazed-stoneware", soil: "potting-compost" }, tones: { pot: 0.78, saucer: 0.72, soil: 0.12 },
};
const POT_PROFILES: Record<Scale, { saucer: P[]; pot: P[] }> = {
  50: { saucer: POT.saucer, pot: POT.pot },
  100: { saucer: keep(POT.saucer, [0, 2, 3, 4, 5]), pot: keep(POT.pot, [0, 2, 3, 4, 5, 7, 8]) },
  200: { saucer: keep(POT.saucer, [0, 3, 5]), pot: keep(POT.pot, [0, 3, 5, 8]) },
  500: { saucer: [[0, 0.19], [0.077, 0.198]], pot: [[0.016, 0.145], [0.311, 0.18], [0.288, 0.1612]] },
};

/** 🪵️ The coir moss pole (pushed to the pot bottom) and the two climbing stem segments in front of it: base point, length, radii. */
const POLE = { x: -0.02, base: 0.03, length: 1.38, r: 0.024 };
const STEMS = [
  { id: "stem-1", parent: "pot", x: 0.017, base: 0.032, length: 0.79, r0: 0.0145, r1: 0.0135 },
  { id: "stem-2", parent: "stem-1", x: 0.017, base: 0.822, length: 0.56, r0: 0.0135, r1: 0.011 },
];

/** 🍃️ One leaf: node on its stem (height above the floor), petiole azimuth (deg, two ranks near 40 and 220), petiole length and rise above the horizontal, blade pitch below the horizontal, roll about the midrib, yaw of the blade against the petiole, and the blade form. */
type Leaf = { k: number; stem: string; y: number; az: number; P: number; rise: number; pitch: number; roll: number; yaw: number; blade: MonsteraBlade; upper: boolean };
const LEAVES: Leaf[] = [
  { k: 0, stem: "stem-1", y: 0.43, az: 228, P: 0.5, rise: 38, pitch: 50, roll: 12, yaw: -8, blade: { L: 0.52, wr: 0.92, slits: 6, holes: 4, depth: 0.62, shift: 0.12 }, upper: false },
  { k: 1, stem: "stem-1", y: 0.54, az: 42, P: 0.55, rise: 40, pitch: 48, roll: -10, yaw: 6, blade: { L: 0.56, wr: 0.9, slits: 6, holes: 5, depth: 0.66, shift: -0.1 }, upper: false },
  { k: 2, stem: "stem-1", y: 0.65, az: 215, P: 0.52, rise: 44, pitch: 46, roll: 14, yaw: 5, blade: { L: 0.55, wr: 0.9, slits: 6, holes: 4, depth: 0.6, shift: 0.15 }, upper: false },
  { k: 3, stem: "stem-1", y: 0.76, az: 52, P: 0.5, rise: 46, pitch: 42, roll: -12, yaw: -6, blade: { L: 0.52, wr: 0.88, slits: 5, holes: 4, depth: 0.62, shift: -0.12 }, upper: false },
  { k: 4, stem: "stem-2", y: 0.88, az: 232, P: 0.46, rise: 50, pitch: 38, roll: 10, yaw: 8, blade: { L: 0.48, wr: 0.88, slits: 5, holes: 3, depth: 0.58, shift: 0.1 }, upper: true },
  { k: 5, stem: "stem-2", y: 0.99, az: 35, P: 0.42, rise: 54, pitch: 34, roll: -14, yaw: -5, blade: { L: 0.44, wr: 0.86, slits: 4, holes: 3, depth: 0.56, shift: -0.15 }, upper: true },
  { k: 6, stem: "stem-2", y: 1.1, az: 205, P: 0.38, rise: 58, pitch: 28, roll: 12, yaw: 6, blade: { L: 0.38, wr: 0.85, slits: 3, holes: 2, depth: 0.52, shift: 0.12 }, upper: true },
  { k: 7, stem: "stem-2", y: 1.2, az: 58, P: 0.32, rise: 62, pitch: 22, roll: -10, yaw: -4, blade: { L: 0.32, wr: 0.85, slits: 2, holes: 1, depth: 0.45, shift: -0.1 }, upper: true },
  { k: 8, stem: "stem-2", y: 1.3, az: 140, P: 0.3, rise: 70, pitch: 32, roll: 18, yaw: 0, blade: { L: 0.24, wr: 0.82, slits: 0, holes: 0, depth: 0, shift: 0 }, upper: true },
];
const ROOTS = [
  { id: "aerial-root-1", stem: "stem-1", y: 0.6, az: 120, down: 78, length: 0.33, r: 0.004 },
  { id: "aerial-root-2", stem: "stem-2", y: 0.94, az: 300, down: 84, length: 0.5, r: 0.0035 },
];
const nn = (k: number) => String(k + 1).padStart(2, "0");
const petioleId = (k: number) => `petiole-${nn(k)}`;
const bladeId = (k: number) => `blade-${nn(k)}`;
const THICK = 0.0015;
const LEAF_KG_M2 = 0.28;

/** 🧭️ The rest frames of every bone, stated from the table above. */
function frames(): Record<string, Frame> {
  const f: Record<string, Frame> = {};
  f.pot = frameOf([0, 0, 0], UP, [-1, 0, 0]);
  f["moss-pole"] = frameOf([POLE.x, POLE.base, 0], UP, [-1, 0, 0]);
  for (const s of STEMS) f[s.id] = frameOf([s.x, s.base, 0], UP, [-1, 0, 0]);
  for (const l of LEAVES) {
    const d = azimuth(l.az);
    const s = STEMS.find((x) => x.id === l.stem)!;
    const o: V3 = [s.x, l.y, 0];
    const pu = unit(lin(d, Math.cos(l.rise * Math.PI / 180), UP, Math.sin(l.rise * Math.PI / 180)));
    f[petioleId(l.k)] = frameOf(o, pu, UP);
    const tip = add(o, mul(pu, l.P));
    const bd = azimuth(l.az + l.yaw);
    const p = l.pitch * Math.PI / 180;
    const bu = unit(lin(bd, Math.cos(p), UP, -Math.sin(p)));
    const n0 = unit(lin(bd, Math.sin(p), UP, Math.cos(p)));
    const r = l.roll * Math.PI / 180;
    const n = unit(lin(n0, Math.cos(r), cross(bu, n0), Math.sin(r)));
    f[bladeId(l.k)] = frameOf(tip, bu, n);
  }
  for (const r of ROOTS) {
    const s = STEMS.find((x) => x.id === r.stem)!;
    const d = azimuth(r.az);
    const a = r.down * Math.PI / 180;
    f[r.id] = frameOf([s.x, r.y, 0], unit(lin(d, Math.cos(a), UP, -Math.sin(a))), d);
  }
  return f;
}

/** 🧾️ Builds the asset document. */
export function build(): { doc: Json; distinction: Text; notes: string[] } {
  const notes: string[] = [];
  const F = frames();
  const rig = new RigBuilder();
  const bladeArea = (l: Leaf) => { const b = monsteraBlade(l.blade, 50); return polyArea(b.outer) - b.holes.reduce((s, h) => s + polyArea(h), 0); };
  const petR = (l: Leaf) => Math.min(0.011, l.P / 44);
  const potKg = 7.2 + 1.4 + 13.8;
  rig.add({ id: "pot", parent: null, title: T("Glazed pot with saucer, compost and root ball", "Glasierter Topf mit Untersetzer, Erde und Wurzelballen"), f: F.pot!, length: POT.rimTopU, joint: FIXED, mass: massOf(potKg, 0.14, 0, "point", "authored-estimate", "en-13040") });
  rig.add({ id: "moss-pole", parent: "pot", title: T("Coir moss pole (48 mm)", "Kokos-Moosstab (48 mm)"), f: F["moss-pole"]!, length: POLE.length, joint: FIXED, mass: massOf(1.2, POLE.length / 2, 0, "uniform", "authored-estimate", "mabberley-2017"), section: roundSection(2 * POLE.r, "coir", T("Coir pole on a plastic core", "Kokosstab auf Kunststoffkern")) });
  for (const s of STEMS) rig.add({ id: s.id, parent: s.parent, title: T(s.id === "stem-1" ? "Climbing stem, lower segment" : "Climbing stem, upper segment", s.id === "stem-1" ? "Kletterstamm, unteres Stück" : "Kletterstamm, oberes Stück"), f: F[s.id]!, length: s.length, joint: s.id === "stem-1" ? FIXED : revolute(-5, 5), mass: massOf(Math.PI * ((s.r0 + s.r1) / 2) ** 2 * s.length * 950, s.length / 2, 0, "uniform", "authored-estimate", "madison-1977"), section: roundSection(s.r0 + s.r1, "monstera-stem", T("Green stem", "Grüner Stamm")) });
  for (const l of LEAVES) {
    const r = petR(l);
    rig.add({ id: petioleId(l.k), parent: l.stem, title: T(`Petiole ${l.k + 1} (${Math.round(l.P * 100)} cm)`, `Blattstiel ${l.k + 1} (${Math.round(l.P * 100)} cm)`), f: F[petioleId(l.k)]!, length: l.P, joint: revolute(-25, 15), mass: massOf(Math.PI * (0.8 * r) ** 2 * l.P * 1000, l.P * 0.45, 0, "uniform", "authored-estimate", "madison-1977") });
    rig.add({ id: bladeId(l.k), parent: petioleId(l.k), title: T(`Leaf blade ${l.k + 1} (${Math.round(l.blade.L * 100)} cm${l.blade.slits ? ", split and perforated" : ", entire"})`, `Blattspreite ${l.k + 1} (${Math.round(l.blade.L * 100)} cm${l.blade.slits ? ", geschlitzt und gelocht" : ", ganzrandig"})`), f: F[bladeId(l.k)]!, length: l.blade.L, joint: revolute(-30, 20), mass: massOf(bladeArea(l) * LEAF_KG_M2, 0.42 * l.blade.L, 0, "uniform", "authored-estimate", "madison-1977") });
  }
  for (const r of ROOTS) rig.add({ id: r.id, parent: r.stem, title: T(r.id.endsWith("1") ? "Aerial root grown into the compost" : "Free-hanging aerial root", r.id.endsWith("1") ? "In die Erde gewachsene Luftwurzel" : "Frei hängende Luftwurzel"), f: F[r.id]!, length: r.length, joint: revolute(-10, 10), mass: massOf(Math.PI * r.r ** 2 * r.length * 900, r.length / 2, 0, "uniform", "authored-estimate", "madison-1977") });
  notes.push(`[DEBUG] rig frames vs product restFrames: max error ${rig.verify().toExponential(2)}`);
  const leafArea = LEAVES.reduce((s, l) => s + bladeArea(l), 0);
  const totalKg = rig.bones.reduce((s: number, b: Json) => s + (b.mass?.kg ?? 0), 0);
  notes.push(`[DEBUG] total mass ${totalKg.toFixed(3)} kg, one-sided leaf area ${leafArea.toFixed(3)} m2 (upper ${LEAVES.filter((l) => l.upper).reduce((s, l) => s + bladeArea(l), 0).toFixed(3)})`);

  const stub = { id: ID, dimensions: { heightM: 1.6, widthM: 1.1, depthM: 1.1, massKg: totalKg }, rig: rig.rig() };
  const leafTone = (l: Leaf) => r3(0.38 + 0.16 * phase(l.k, 5) + (l.upper ? 0.06 : 0));
  const leafRegion = (l: Leaf) => (l.upper ? "leaves-upper" : "leaves-lower");
  const representations: Json = {};
  const volumes: Json = {};
  for (const scale of SCALES) {
    const pot = potParts(POT, scale, POT_PROFILES[scale]);
    const placed: Placed[] = [...pot.placed];
    const planPlaced: Placed[] = [...pot.plan];
    const body: Json[] = [...pot.body];
    const thermal: Json[] = [...pot.thermal];
    const structure: Json[] = [...potStructure(POT, scale)];
    const fine = scale <= 100;
    const drawLeaves = scale <= 200;
    const pen = PENS[scale].body;
    const plantSolids: Json[] = [];
    const add2 = (list: Placed[]) => { for (const p of list) { if (p.view === "plan") planPlaced.push(p); else if (p.view) placed.push(p); else { placed.push(p); planPlaced.push(p); } } };
    const stalkOf = (id: string, bone: string, u0: number, u1: number, r0: number, r1: number, region: string, material: string, tone: number, drawFine: boolean) => {
      const s = stalk(id, bone, F[bone]!, u0, u1, r0, r1, scale, { tone, region, material }, { material, tone }, drawFine);
      body.push(s.solid); plantSolids.push(s.solid);
      thermal.push({ id: `th-${id}`, of: id, region });
      add2(s.placed.map((p) => ({ ...p, rank: -0.5 })));
      return s;
    };
    stalkOf("moss-pole", "moss-pole", 0, POLE.length, POLE.r, POLE.r, "moss-pole", "coir", 0.34, true);
    for (const s of STEMS) stalkOf(s.id, s.id, s.id === "stem-1" ? 0 : -0.01, s.length, s.r0, s.r1, "stems-petioles", "monstera-stem", 0.42, true);
    for (const r of ROOTS) stalkOf(r.id, r.id, -0.01, r.length, r.r, r.r * 0.8, "stems-petioles", "monstera-stem", 0.3, scale <= 200);
    for (const l of LEAVES) {
      const r = petR(l);
      stalkOf(petioleId(l.k), petioleId(l.k), 0, l.P + 0.004, r, 0.6 * r, "stems-petioles", "monstera-stem", 0.45, scale <= 200);
      const b = monsteraBlade(l.blade, scale);
      const pl = plate(bladeId(l.k), bladeId(l.k), F[bladeId(l.k)]!, b.outer, b.holes, -THICK / 2, THICK / 2, scale, { tone: leafTone(l), region: leafRegion(l), material: "monstera-leaf" }, { material: "monstera-leaf", tone: leafTone(l), sheet: true }, pen);
      body.push(pl.solid); plantSolids.push(pl.solid);
      thermal.push({ id: `th-${bladeId(l.k)}`, of: bladeId(l.k), region: leafRegion(l) });
      if (drawLeaves) add2(pl.placed);
      if (scale === 50 && (pl.unfilled.length || pl.undecided || Object.values(pl.dropped).some((x) => x > 0))) notes.push(`[DEBUG] 1:50 ${bladeId(l.k)} unfilled ${pl.unfilled.join(",") || "-"} dropped holes ${JSON.stringify(pl.dropped)} left out ${pl.undecided} of ${b.holes.length}`);
      structure.push({ ...stripMember(l, scale), member: bladeId(l.k), part: "member" });
      structure.push({ id: `m-${petioleId(l.k)}`, of: petioleId(l.k), member: petioleId(l.k), part: "member" });
    }
    if (false) add2(crown(stub, F, scale, plantSolids, "pot", F.pot!, pen, { tone: 0.46, regions: ["leaves-upper", "leaves-lower", "stems-petioles", "moss-pole"], material: "monstera-leaf" }, "crown").map((p) => ({ ...p, rank: p.view === "plan" ? 3e6 : 1.5e6 })));
    for (const id of ["moss-pole", ...STEMS.map((s) => s.id), ...ROOTS.map((r) => r.id)]) structure.push({ id: `m-${id}`, of: id, member: id, part: "member" });
    for (const a of ANCHORS) {
      const l = LEAVES[a.leaf]!;
      const fe = FEATURE[scale];
      const rr = Math.max(0.004, fe / 2 + 0.001);
      thermal.push({ id: `th-${a.id}`, kind: "ellipsoid", bone: bladeId(l.k), material: "monstera-leaf", role: "body", tone: 0.8, radii: [r5(Math.max(0.03, 2 * rr)), r5(rr), r5(Math.max(0.02, rr))], transform: { translation: [r5(a.s * l.blade.L), r5(THICK / 2 + rr * 0.6), r5(a.w * l.blade.L)] }, region: leafRegion(l), tags: ["emitter", l.upper ? "transpiration-upper" : "transpiration-lower"] });
    }
    const geometry = ordered(placed, { plan: planPlaced });
    representations[String(scale)] = scale === 500 ? UNAVAILABLE_500 : { status: "authored", denominator: scale, validRange: { minDenominator: BAND[scale][0], maxDenominator: BAND[scale][1] }, pens: PENS[scale], hatches: [], geometry, decisions: REP_DECISIONS[scale] };
    volumes[String(scale)] = { status: "authored", denominator: scale, body, thermal, structure, decisions: VOL_DECISIONS[scale] };
  }

  const anchors: Json[] = [
    { id: "base-support", bone: "pot", x: 0, y: 0, kind: "support", title: T("Saucer on the floor", "Untersetzer auf dem Boden"), support: "fixed", normalDeg: 90, footprintLengthM: 0.36, footprintWidthM: 0.36, frictionCoefficient: 0.5 },
    ...ANCHORS.map((a) => ({ id: a.id, bone: bladeId(a.leaf), x: r4(a.s * LEAVES[a.leaf]!.blade.L), y: r4(THICK / 2 + 0.001), z: r4(a.w * LEAVES[a.leaf]!.blade.L), kind: "emitter", title: T(`Stomata outlet on blade ${a.leaf + 1}`, `Spaltöffnungsauslass an Spreite ${a.leaf + 1}`) })),
  ];

  const v = 1.0;
  const nx = (l: Leaf) => dot(F[bladeId(l.k)]!.v, [1, 0, 0]);
  const loads = LEAVES.map((l) => ({ id: `wind-${bladeId(l.k)}`, kind: "wind", title: T(`Draught on blade ${l.k + 1}`, `Luftzug an Spreite ${l.k + 1}`), bone: bladeId(l.k), x: r4(0.42 * l.blade.L), y: 0, fxN: r5(drag(1.2, v, bladeArea(l) * Math.max(0.2, Math.abs(nx(l))))), fyN: 0, source: "literature", reference: "en-1991-1-4" }));
  const bend = (l: Leaf) => -1.6 * Math.sign(nx(l)) * (l.blade.L / 0.56);
  const poseBones = (pet: (l: Leaf) => number, bl: (l: Leaf) => number, stem2 = 0) => ({ ...(stem2 ? { "stem-2": { angleDeg: stem2 } } : {}), ...Object.fromEntries(LEAVES.flatMap((l) => [[petioleId(l.k), { angleDeg: r4(pet(l)) }], [bladeId(l.k), { angleDeg: r4(bl(l)) }]])) });
  const regionsOf = (upSun: number, lowSun: number, potSun: number, soilWet: number, poleWet: number) => ({ "leaves-upper": { exposedFraction: 1, sunlitFraction: upSun }, "leaves-lower": { exposedFraction: 1, sunlitFraction: lowSun }, "stems-petioles": { exposedFraction: 1, sunlitFraction: lowSun * 0.6 }, "moss-pole": { exposedFraction: 0.7, sunlitFraction: lowSun * 0.3, ...(poleWet > 0 ? { wettedness: poleWet } : {}) }, "pot-wall": { exposedFraction: 1, sunlitFraction: potSun }, saucer: { exposedFraction: 1, sunlitFraction: potSun * 0.5 }, "soil-surface": { exposedFraction: 1, sunlitFraction: potSun * 0.3, ...(soilWet > 0 ? { wettedness: soilWet } : {}) } });
  const day = { "transpiration-upper": 0.85, "transpiration-lower": 0.6 };
  const poses: Json[] = [
    { id: "still-air", title: T("Still air, bright room", "Ruhende Luft, heller Raum"), description: T("The monstera stands two metres from a window in a heated living room with still air; diffuse daylight reaches the upper leaves more than the lower ones, the stomata are open, the compost is evenly moist.", "Die Monstera steht zwei Meter vor einem Fenster in einem beheizten Wohnraum bei ruhender Luft; diffuses Tageslicht erreicht die oberen Blätter stärker als die unteren, die Spaltöffnungen sind offen, die Erde ist gleichmäßig feucht."), bones: {}, ground: "authored", contacts: [], thermal: { regions: regionsOf(0.45, 0.25, 0.15, 0, 0), emitters: day }, loads: [] },
    { id: "ventilation-draught", title: T("Draught from an open window", "Luftzug aus einem offenen Fenster"), description: T(`The room is aired: ${v} m/s of air flow towards +x over the plant; each blade is pushed by cf 1.2 times the dynamic pressure on its face seen from ahead and turns by 1 to 2 degrees on its petiole.`, `Der Raum wird gelüftet: ${String(v).replace(".", ",")} m/s Luft strömen in Richtung +x über die Pflanze; jede Spreite wird mit cf 1,2 mal dem Staudruck auf ihre von vorn gesehene Fläche gedrückt und dreht sich um 1 bis 2 Grad an ihrem Stiel.`), bones: poseBones(() => 0, bend, 0.4), ground: "authored", contacts: [], thermal: { regions: regionsOf(0.45, 0.25, 0.15, 0, 0), emitters: { "transpiration-upper": 1, "transpiration-lower": 0.72 } }, loads },
    { id: "freshly-watered", title: T("Freshly watered", "Frisch gegossen"), description: T("2 litres of water have soaked the compost and the moss pole (19.6 N more in the pot); the turgid petioles lift the blades by a few degrees.", "2 Liter Wasser haben Erde und Moosstab durchfeuchtet (19,6 N mehr im Topf); die prallen Blattstiele heben die Spreiten um einige Grad."), bones: poseBones(() => 2, () => 3), ground: "authored", contacts: [], thermal: { regions: regionsOf(0.45, 0.25, 0.15, 1, 0.8), emitters: { "transpiration-upper": 0.9, "transpiration-lower": 0.65 } }, loads: [{ id: "irrigation-water", kind: "imposed", title: T("Water held in compost and pole", "In Erde und Stab gehaltenes Wasser"), bone: "pot", x: 0.15, y: 0, fxN: 0, fyN: -19.613, source: "authored-estimate", reference: "en-13040" }] },
    { id: "drought-wilted", title: T("Wilted after drought", "Welk nach Trockenheit"), description: T("The compost has dried out: the petioles lose turgor and sink by 8 to 14 degrees, the blades hang 10 to 18 degrees lower and the stomata are nearly shut.", "Die Erde ist ausgetrocknet: Die Blattstiele verlieren an Spannung und sinken um 8 bis 14 Grad, die Spreiten hängen 10 bis 18 Grad tiefer und die Spaltöffnungen sind fast geschlossen."), bones: poseBones((l) => -8 - 6 * (l.P / 0.6), (l) => -10 - 8 * (l.blade.L / 0.56)), ground: "authored", contacts: [], thermal: { regions: regionsOf(0.45, 0.25, 0.15, 0, 0), emitters: { "transpiration-upper": 0.12, "transpiration-lower": 0.08 } }, loads: [] },
  ];
  const T0 = 2.4;
  const swayKeys = Array.from({ length: 13 }, (_, i) => {
    const t = (T0 * i) / 12;
    const w = (k: number, a: number) => a * Math.sin(2 * Math.PI * (t / T0) + 2 * Math.PI * phase(k, 9));
    return { t: r4(t), pose: "ventilation-draught", ease: "smooth", bones: { "stem-2": { angleDeg: r4(0.4 + w(20, 0.3)) }, ...Object.fromEntries(LEAVES.flatMap((l) => [[petioleId(l.k), { angleDeg: r4(w(l.k, 0.6 * l.P / 0.6)) }], [bladeId(l.k), { angleDeg: r4(bend(l) + w(l.k + 30, 2.2 * l.blade.L / 0.56)) }]])) } };
  });
  const gustT = [0, 0.8, 1.6, 2.2, 3, 4, 6];
  const gustG = [1, 1.5, 2.6, 3, 2.2, 1.3, 1];
  const gustKeys = gustT.map((t, i) => ({ t, pose: "ventilation-draught", ease: "smooth", bones: { "stem-2": { angleDeg: r4(0.4 * gustG[i]! ** 2 / 1) }, ...Object.fromEntries(LEAVES.flatMap((l) => [[petioleId(l.k), { angleDeg: r4(-0.4 * (gustG[i]! ** 2 - 1) * Math.sign(nx(l)) * l.P / 0.6) }], [bladeId(l.k), { angleDeg: r4(bend(l) * Math.min(4, gustG[i]! ** 2)) }]])) } }));
  const clips: Json[] = [
    { id: "wind-sway", title: T("Draught flutter", "Flattern im Luftzug"), description: T(`A periodic ${T0} s cycle in the airing draught: every blade nods by about two degrees about its mean turn on its petiole with its own phase, the petioles and the upper stem follow by a fraction of a degree.`, `Ein periodischer Zyklus von ${String(T0).replace(".", ",")} s im Lüftungszug: Jede Spreite nickt um etwa zwei Grad um ihre mittlere Drehung am Stiel mit eigener Phase, Blattstiele und oberer Stamm folgen um Bruchteile eines Grads.`), durationS: T0, fps: 30, loop: true, rootMotion: "none", keys: swayKeys, contacts: [], phases: [{ id: "flutter", fromS: 0, toS: T0, title: T("Flutter cycle", "Flatterzyklus") }], events: [{ t: 0, kind: "motion", title: T("Cycle start at the mean turn", "Zyklusbeginn in der mittleren Drehung") }], tracks: [] },
    { id: "gust", title: T("Door gust", "Türböe"), description: T("A door opens across the airing window: the draught rises from 1 to about 3 m/s within two seconds and settles; the blades turn up to four times their mean angle and the drag follows the square of the speed.", "Eine Tür öffnet sich gegenüber dem offenen Fenster: Der Luftzug steigt binnen zwei Sekunden von 1 auf etwa 3 m/s und legt sich; die Spreiten drehen bis zum Vierfachen ihres mittleren Winkels, der Widerstand folgt dem Quadrat der Geschwindigkeit."), durationS: 6, fps: 30, loop: false, rootMotion: "none", keys: gustKeys, contacts: [], phases: [{ id: "rise", fromS: 0, toS: 2.2, title: T("Gust front", "Böenfront") }, { id: "peak", fromS: 2.2, toS: 3, title: T("Peak", "Spitze") }, { id: "decay", fromS: 3, toS: 6, title: T("Settling", "Abklingen") }], events: [{ t: 2.2, kind: "load-on", title: T("Peak draught load", "Spitzenlast des Luftzugs") }, { t: 4, kind: "load-off", title: T("Load back near the mean", "Last wieder nahe dem Mittel") }], tracks: loads.map((d) => ({ target: `load:${d.id}`, ease: "smooth", keys: gustT.map((t, i) => ({ t, v: r3(gustG[i]! ** 2) })) })) },
    { id: "diurnal-transpiration", title: T("Day and night by the window", "Tag und Nacht am Fenster"), description: T("24 hours in 24 seconds (1 s = 1 h): the C3 stomata open with the morning light, transpiration peaks around noon when daylight from the window reaches the upper leaves, and closes after dusk; at night only cuticular loss remains.", "24 Stunden in 24 Sekunden (1 s = 1 h): Die C3-Spaltöffnungen öffnen sich mit dem Morgenlicht, die Transpiration erreicht um Mittag ihr Maximum, wenn das Tageslicht vom Fenster die oberen Blätter erreicht, und schließt nach der Dämmerung; nachts bleibt nur der Verlust durch die Kutikula."), durationS: 24, fps: 10, loop: true, rootMotion: "none", keys: [{ t: 0, pose: "still-air", ease: "smooth" }, { t: 12, pose: "still-air", ease: "smooth" }, { t: 24, pose: "still-air", ease: "smooth" }], contacts: [], phases: [{ id: "night", fromS: 0, toS: 7, title: T("Night", "Nacht") }, { id: "morning", fromS: 7, toS: 12, title: T("Morning to noon", "Morgen bis Mittag") }, { id: "afternoon", fromS: 12, toS: 18, title: T("Afternoon", "Nachmittag") }, { id: "evening", fromS: 18, toS: 24, title: T("Evening and night", "Abend und Nacht") }], events: [{ t: 7.5, kind: "thermal", title: T("Daylight reaches the leaves", "Tageslicht erreicht die Blätter") }, { t: 12.5, kind: "thermal", title: T("Brightest light", "Hellstes Licht") }, { t: 17.5, kind: "thermal", title: T("Daylight fades", "Tageslicht schwindet") }], tracks: [
      { target: "emitter:transpiration-upper", ease: "smooth", keys: [{ t: 0, v: 0.05 }, { t: 6.5, v: 0.05 }, { t: 8.5, v: 0.45 }, { t: 11, v: 0.9 }, { t: 13, v: 1 }, { t: 15.5, v: 0.8 }, { t: 18, v: 0.3 }, { t: 19.5, v: 0.06 }, { t: 24, v: 0.05 }] },
      { target: "emitter:transpiration-lower", ease: "smooth", keys: [{ t: 0, v: 0.04 }, { t: 6.5, v: 0.04 }, { t: 8.5, v: 0.3 }, { t: 11, v: 0.62 }, { t: 13, v: 0.7 }, { t: 15.5, v: 0.55 }, { t: 18, v: 0.2 }, { t: 19.5, v: 0.05 }, { t: 24, v: 0.04 }] },
      { target: "region:leaves-upper:sunlitFraction", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 7, v: 0 }, { t: 9, v: 0.25 }, { t: 12.5, v: 0.45 }, { t: 16, v: 0.3 }, { t: 18, v: 0.02 }, { t: 19, v: 0 }, { t: 24, v: 0 }] },
      { target: "region:leaves-lower:sunlitFraction", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 7, v: 0 }, { t: 9, v: 0.12 }, { t: 12.5, v: 0.25 }, { t: 16, v: 0.16 }, { t: 18, v: 0.01 }, { t: 19, v: 0 }, { t: 24, v: 0 }] },
    ] },
  ];

  const crownProjection = Math.PI * 0.5 * 0.5;
  const upperArea = LEAVES.filter((l) => l.upper).reduce((s, l) => s + bladeArea(l), 0);
  const lowerArea = leafArea - upperArea;
  const stalkArea = LEAVES.reduce((s, l) => s + 2 * Math.PI * 0.8 * petR(l) * l.P, 0) + STEMS.reduce((s, x) => s + 2 * Math.PI * ((x.r0 + x.r1) / 2) * x.length, 0) + ROOTS.reduce((s, x) => s + 2 * Math.PI * x.r * 0.9 * x.length, 0);
  const thermal = {
    environment: { airTempC: 21, meanRadiantTempC: 20.5, relativeHumidity: 0.45, windMS: 0.05, windDirectionDeg: 0, solarWM2: 120, diffuseFraction: 0.75, pressurePa: 101325 },
    referenceEnvironment: "office-still",
    passengers: [], operating: [],
    regions: [
      { id: "leaves-upper", title: T("Upper leaf blades (younger, nearer the light)", "Obere Blattspreiten (jünger, näher am Licht)"), kind: "foliage", model: "leaf", material: "monstera-leaf", areaM2: r4(upperArea), areaBasis: "exchange", characteristicLengthM: 0.3, geometry: "horizontal-plate-up", skyViewFactor: 0.55, layers: [], heatCapacityJM2K: 2500, leaf: { stomatalConductanceMaxMS: 0.004, leafAreaIndex: r3(upperArea / crownProjection), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
      { id: "leaves-lower", title: T("Lower leaf blades (older, shaded)", "Untere Blattspreiten (älter, beschattet)"), kind: "foliage", model: "leaf", material: "monstera-leaf", areaM2: r4(lowerArea), areaBasis: "exchange", characteristicLengthM: 0.4, geometry: "horizontal-plate-up", skyViewFactor: 0.35, layers: [], heatCapacityJM2K: 2700, leaf: { stomatalConductanceMaxMS: 0.003, leafAreaIndex: r3(lowerArea / crownProjection), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
      { id: "stems-petioles", title: T("Stem, petioles and aerial roots", "Stamm, Blattstiele und Luftwurzeln"), kind: "stem", model: "passive", material: "monstera-stem", areaM2: r4(stalkArea), characteristicLengthM: 0.018, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 16000, source: "authored-estimate", reference: "madison-1977" },
      { id: "moss-pole", title: T("Coir moss pole", "Kokos-Moosstab"), kind: "surface", model: "passive", material: "coir", areaM2: r4(2 * Math.PI * POLE.r * (POLE.length - (POT.soilU - POLE.base))), characteristicLengthM: 0.048, geometry: "cylinder", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 18000, source: "authored-estimate", reference: "mabberley-2017" },
      { id: "pot-wall", title: T("Glazed pot wall and rim", "Glasierte Topfwand mit Rand"), kind: "surface", model: "passive", material: "glazed-stoneware", areaM2: 0.41, characteristicLengthM: 0.33, geometry: "vertical-plate", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: 19000, source: "authored-estimate", reference: "iso-10456" },
      { id: "saucer", title: T("Saucer", "Untersetzer"), kind: "surface", model: "passive", material: "glazed-stoneware", areaM2: 0.23, characteristicLengthM: 0.4, geometry: "horizontal-plate-up", skyViewFactor: 0.25, layers: [], heatCapacityJM2K: 12000, source: "authored-estimate", reference: "iso-10456" },
      { id: "soil-surface", title: T("Compost surface", "Erdoberfläche"), kind: "surface", model: "passive", material: "potting-compost", areaM2: r4(Math.PI * POT.soilR ** 2), characteristicLengthM: 0.32, geometry: "horizontal-plate-up", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 39000, source: "authored-estimate", reference: "en-13040" },
    ],
    emitters: [
      { id: "transpiration-upper", kind: "transpiration", title: T("Transpiration of the upper blades (five outlets)", "Transpiration der oberen Spreiten (fünf Auslässe)"), anchor: "tp-up-1", anchors: ANCHORS.filter((a) => LEAVES[a.leaf]!.upper).map((a) => a.id), directionDeg: 90, diameterM: 0.3, region: "leaves-upper", source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-lower", kind: "transpiration", title: T("Transpiration of the lower blades (five outlets)", "Transpiration der unteren Spreiten (fünf Auslässe)"), anchor: "tp-low-1", anchors: ANCHORS.filter((a) => !LEAVES[a.leaf]!.upper).map((a) => a.id), directionDeg: 90, diameterM: 0.4, region: "leaves-lower", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    assumptions: [
      T(`Leaf regions use the one-sided area of the nine real blades without their holes (${r3(upperArea)} m2 upper, ${r3(lowerArea)} m2 lower, over a crown projection of ${r3(crownProjection)} m2). The maximum stomatal conductances (0.004 and 0.003 m/s) are authored estimates for a shade-tolerant C3 aroid indoors, guided by Jones (2014); leathery blades of about 0.6 mm store 2.5 to 2.7 kJ/(m2 K).`, `Blattregionen verwenden die einseitige Fläche der neun wirklichen Spreiten ohne ihre Löcher (${String(r3(upperArea)).replace(".", ",")} m2 oben, ${String(r3(lowerArea)).replace(".", ",")} m2 unten, über einer Kronenprojektion von ${String(r3(crownProjection)).replace(".", ",")} m2). Die maximalen Spaltöffnungsleitfähigkeiten (0,004 und 0,003 m/s) sind Autorenschätzungen für ein schattenverträgliches C3-Aronstabgewächs im Innenraum, angelehnt an Jones (2014); ledrige Spreiten von etwa 0,6 mm speichern 2,5 bis 2,7 kJ/(m2 K).`),
      T("Stem, petioles, aerial roots and the coir pole are passive and water-rich; the pole is wetted in the watered pose. Pot, saucer and compost are passive (glazed stoneware after ISO 10456). The reference case is the still office-like room (office-still), the asset's own case adds diffuse daylight of 120 W/m2 from a window.", "Stamm, Blattstiele, Luftwurzeln und der Kokosstab sind passiv und wasserreich; der Stab ist in der gegossenen Pose benetzt. Topf, Untersetzer und Erde sind passiv (glasiertes Steinzeug nach ISO 10456). Der Referenzfall ist der ruhige, büroähnliche Raum (office-still), der eigene Fall ergänzt diffuses Tageslicht von 120 W/m2 von einem Fenster."),
    ],
  };

  const asset: Json = {
    id: ID, category: "vegetation",
    title: T("Potted Monstera", "Monstera im Topf"),
    summary: T("A Swiss cheese plant (Monstera deliciosa) 1.6 m high in a 0.36 m cream-glazed stoneware pot on a saucer, climbing a coir moss pole: nine leathery, glossy leaves on petioles of 0.3 to 0.6 m in two ranks, the older blades of 0.5 m deeply split and perforated with oval holes, the youngest one entire; two aerial roots. A classic shade-tolerant houseplant of living rooms and offices.", "Ein Fensterblatt (Monstera deliciosa) von 1,6 m Höhe in einem cremeweiß glasierten Steinzeugtopf von 0,36 m auf einem Untersetzer, an einem Kokos-Moosstab kletternd: neun ledrige, glänzende Blätter auf Stielen von 0,3 bis 0,6 m in zwei Zeilen, die älteren Spreiten von 0,5 m tief geschlitzt und mit ovalen Löchern durchbrochen, die jüngste ganzrandig; zwei Luftwurzeln. Eine klassische schattenverträgliche Zimmerpflanze in Wohnräumen und Büros."),
    tags: ["monstera", "aroid", "climber", "indoor", "potted", "houseplant", "fenestrated", "broad-leaf"],
    provenance: {
      author: "Semio staffage plant worker PW-17", method: "handcrafted", created: "2026-11-22",
      references: [
        { id: "madison-1977", citation: "Madison, M. (1977). A revision of Monstera (Araceae). Contributions from the Gray Herbarium of Harvard University 207: 3-100. M. deliciosa: adult blades cordate-ovate, pinnatifid, 25-90 cm long, with one or more series of elliptic perforations near the midrib; petioles long, geniculate below the blade; leaves distichous on the climbing stem; aerial roots." },
        { id: "mabberley-2017", citation: "Mabberley, D. J. (2017). Mabberley's Plant-Book, 4th ed. Cambridge University Press (Monstera, cultivation as a houseplant on a moist support)." },
        { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed. Cambridge University Press. Leaf energy balance, boundary layer and stomatal conductance, leaf area index." },
        { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed. Academic Press. Penman-Monteith leaf transpiration." },
        { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A." },
        { id: "iso-10456", citation: "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values. Ceramic (porcelain, stoneware): 2300 kg/m3, 1.3 W/(m K), 840 J/(kg K)." },
        { id: "en-13040", citation: "EN 13040:2007 Soil improvers and growing media - Sample preparation for chemical and physical tests, determination of dry matter content, moisture content and laboratory compacted bulk density (the bulk density of moist potting compost of about 0.5 to 0.7 kg/l used for the pot mass)." },
      ],
      notes: T("All nine leaves are stated by hand with their node, azimuth, petiole, pitch, roll and blade form (length, width, slits, holes, slit depth, asymmetry); the margin is a hand-drawn control table, the slits follow the lateral veins and open by 26 degrees, the holes sit in one row between them (`T/🧰️scripts/PW-17/potted-monstera.ts`, `monstera-leaf.ts`). The leaves are drawn one by one down to 1:200; the 1:500 drawing is unavailable (the gaps between the leaves cannot print at 1:1000) with a plan symbol, while a digital 1:500 model of single leaf sheets exists.", "Alle neun Blätter sind von Hand mit Knoten, Azimut, Stiel, Neigung, Rollung und Spreitenform (Länge, Breite, Schlitze, Löcher, Schlitztiefe, Asymmetrie) angegeben; der Rand ist eine handgezeichnete Stützpunkttabelle, die Schlitze folgen den Seitennerven und öffnen sich um 26 Grad, die Löcher sitzen in einer Reihe dazwischen (`T/🧰️scripts/PW-17/monstera.ts`, `monstera-leaf.ts`). Die Blätter sind bis 1:200 einzeln gezeichnet; die Zeichnung 1:500 ist nicht verfügbar (die Lücken zwischen den Blättern können bei 1:1000 nicht drucken), mit einem Grundrisssymbol, während ein digitales Modell 1:500 aus einzelnen Blattflächen besteht."),
    },
    dimensions: { heightM: 1.6, widthM: 1.1, depthM: 1.1, massKg: r3(totalKg) },
    materials: [
      { id: "monstera-leaf", title: T("Leathery, glossy monstera blade", "Ledrige, glänzende Monstera-Spreite"), emissivity: 0.96, solarAbsorptivity: 0.75, densityKgM3: 950, specificHeatJKgK: 3800, source: "authored-estimate", reference: "jones-2014" },
      { id: "monstera-stem", title: T("Succulent green stem and petiole", "Saftiger grüner Stamm und Blattstiel"), emissivity: 0.95, solarAbsorptivity: 0.7, densityKgM3: 950, specificHeatJKgK: 3700, elasticModulusPa: 60e6, strengthPa: 2e6, source: "authored-estimate", reference: "madison-1977" },
      { id: "coir", title: T("Coir fibre pole (moist)", "Kokosfaserstab (feucht)"), emissivity: 0.95, solarAbsorptivity: 0.8, densityKgM3: 450, conductivityWmK: 0.1, specificHeatJKgK: 2500, elasticModulusPa: 1.5e9, strengthPa: 15e6, source: "authored-estimate", reference: "mabberley-2017" },
      ...potMaterials(T("Glazed stoneware (cream glaze)", "Glasiertes Steinzeug (cremeweiße Glasur)"), 0.3),
    ],
    rig: rig.rig(), volumes, anchors, representations,
    symbols: [SYMBOL()], poses, clips, thermal,
    structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["base-support"], assumptions: [T("One rigid body per evaluated pose, fixed at the saucer on the floor; the climbing stem is tied to the moss pole, which stands on the pot bottom; each petiole is a cantilever from its node carrying its blade, loaded by weight and by the draught drag of its pose (EN 1991-1-4 pressure with cf 1.2, about a tenth of a newton per blade at 1 m/s). Sections are solid circles of the green stem and pole; no failure is predicted.", "Ein starrer Körper je ausgewerteter Pose, eingespannt am Untersetzer auf dem Boden; der Kletterstamm ist an den Moosstab gebunden, der auf dem Topfboden steht; jeder Blattstiel ist ein Kragarm aus seinem Knoten mit seiner Spreite, belastet durch Gewicht und den Widerstand des Luftzugs seiner Pose (Druck nach EN 1991-1-4 mit cf 1,2, etwa ein Zehntel Newton je Spreite bei 1 m/s). Querschnitte sind Vollkreise des grünen Stamms und des Stabs; es wird kein Versagen vorhergesagt.")], clinical: false },
    presentation: { defaultPose: "still-air", defaultClip: "wind-sway", defaultScale: 50 },
    exports: EXPORTS, completion: "authored",
  };
  const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
  const distinction = T("A potted Swiss cheese plant (Monstera deliciosa) 1.6 m high on a coir moss pole: nine glossy leaves in two ranks, the older ones deeply split and perforated, in a 0.36 m cream-glazed pot on a saucer.", "Ein Fensterblatt (Monstera deliciosa) im Topf, 1,6 m hoch an einem Kokos-Moosstab: neun glänzende Blätter in zwei Zeilen, die älteren tief geschlitzt und gelocht, in einem cremeweiß glasierten Topf von 0,36 m auf einem Untersetzer.");
  return { doc, distinction, notes };
}

/** 🦴️ The structural member of a blade: a midrib strip 24 mm wide inside the plate, from just behind the insertion to 0.85 of the blade. */
function stripMember(l: Leaf, scale: Scale): Json {
  const reach = { 50: 0.85, 100: 0.84, 200: 0.82, 500: 0.8 }[scale];
  const hw = 0.012;
  return { id: `m-${bladeId(l.k)}`, kind: "extrude", bone: bladeId(l.k), material: "monstera-leaf", role: "body", tone: 0.5, slender: true, plane: "zx", profile: [[-hw, -0.004], [-hw * 0.6, r5(reach * l.blade.L)], [hw * 0.6, r5(reach * l.blade.L)], [hw, -0.004]], from: r5(-THICK / 2 + 0.0002), to: r5(THICK / 2 - 0.0002) };
}

/** 💨️ The ten stomata outlets: blade, share of its length, lateral share. */
const ANCHORS = [
  { id: "tp-low-1", leaf: 0, s: 0.45, w: 0.15 }, { id: "tp-low-2", leaf: 1, s: 0.45, w: -0.15 }, { id: "tp-low-3", leaf: 2, s: 0.45, w: 0.15 }, { id: "tp-low-4", leaf: 3, s: 0.45, w: -0.15 }, { id: "tp-low-5", leaf: 1, s: 0.7, w: 0.08 },
  { id: "tp-up-1", leaf: 4, s: 0.45, w: 0.15 }, { id: "tp-up-2", leaf: 5, s: 0.45, w: -0.15 }, { id: "tp-up-3", leaf: 6, s: 0.45, w: 0.12 }, { id: "tp-up-4", leaf: 7, s: 0.45, w: -0.12 }, { id: "tp-up-5", leaf: 8, s: 0.5, w: 0.05 },
];

const PENS: Record<Scale, Json> = {
  50: { body: 0.18, detail: 0.13, texture: 0.1, glazing: null, ground: 0.25 },
  100: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  200: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.2 },
  500: { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.18 },
};

/** 🔣️ The plan sign: the outline of crown and pot in true extent in all three views, filled by {@link finish}. */
function SYMBOL(): Json {
  return {
    id: "potted-monstera-plan-symbol", title: T("Plan sign of a potted monstera", "Grundrisszeichen einer Monstera im Topf"), description: T("A conventional sign for interior plans: the outline of the leaf crown around the pot with its true extent (about 1.1 m across); a separate symbol, never used silently for a pictorial scale.", "Ein konventionelles Zeichen für Innenraumpläne: der Umriss der Blattkrone um den Topf in wahrer Ausdehnung (etwa 1,1 m breit); ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."),
    semantics: "true-extent", units: "model-m", denominators: [500, 200], pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, hatches: [],
    geometry: { elevation: [], front: [], plan: [] },
    dimensionalNote: T("True extent in model metres: the outline is the reach of the leaves and the pot, about 1.1 m across and 1.6 m high.", "Wahre Ausdehnung in Modellmetern: Der Umriss ist die Reichweite der Blätter und des Topfs, etwa 1,1 m breit und 1,6 m hoch."),
  };
}

/** 🔣️ Fills the symbol outlines from the convex hull of the 1:50 body in each view. */
export function finish(asset: Json): void {
  const meshes = worldMeshes(asset, 50, asset.volumes["50"].body);
  for (const view of VIEWS) {
    const pts: P[] = [];
    for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) pts.push(project(view, [m.world[i], m.world[i + 1], m.world[i + 2]]));
    asset.symbols[0].geometry[view] = [{ id: "sign", bone: "pot", role: "body", z: 1, filled: false, tone: 0.5, regions: ["leaves-upper", "leaves-lower", "stems-petioles", "moss-pole", "pot-wall", "saucer", "soil-surface"], kind: "poly", points: hull2(pts).map(([a, b]) => [r4(a), r4(b)]), closed: true }];
  }
}

const UNAVAILABLE_500 = {
  status: "unavailable", denominator: 500, cause: "below-printable-feature",
  reason: T("At 1:500 the 1.6 m monstera is 3.2 mm high: at the end of the band (1:1000) two leaves that do not touch would have to stand 0.3 m apart and a 0.5 m blade is 0.5 mm wide, so the crown could only print as one blob that hides the split leaves and the open gaps between the two ranks.", "Bei 1:500 ist die 1,6 m hohe Monstera 3,2 mm hoch: am Ende des Bands (1:1000) müssten zwei sich nicht berührende Blätter 0,3 m auseinanderstehen und eine 0,5 m große Spreite ist 0,5 mm breit, sodass die Krone nur als ein Klecks drucken könnte, der die geschlitzten Blätter und die offenen Lücken zwischen den beiden Zeilen verbirgt."),
  limitingFeature: T("Gap between neighbouring blades (about 0.15 m)", "Lücke zwischen benachbarten Spreiten (etwa 0,15 m)"), limitingSizeM: 0.15,
  alternatives: [{ kind: "symbol", symbol: "potted-monstera-plan-symbol" }, { kind: "scale", denominator: 200 }],
};

const REP_DECISIONS: Record<Scale, Json> = {
  50: decisions({
    silhouette: T("At 1:50 (1.6 m is 32 mm) every view is the true orthographic view of the 1:50 model: nine blades as plates on their own bones with the cordate-ovate margin, the pinnatifid slits and the oval holes, the petioles, the climbing stem on the coir pole, two aerial roots and the cream-glazed pot with its rolled rim on the saucer.", "Bei 1:50 (1,6 m sind 32 mm) ist jede Ansicht die wahre Parallelprojektion des Modells 1:50: neun Spreiten als Platten auf eigenen Knochen mit herz-eiförmigem Rand, fiederspaltigen Schlitzen und ovalen Löchern, die Blattstiele, der Kletterstamm am Kokosstab, zwei Luftwurzeln und der cremeweiß glasierte Topf mit gerolltem Rand auf dem Untersetzer."),
    retained: T("Retained: every leaf with its slits (V-shaped, 26 degrees, so that they print) and the holes that print in that view, every petiole, stem, pole and root; the compost inside the rim in plan.", "Beibehalten: jedes Blatt mit seinen Schlitzen (V-förmig, 26 Grad, damit sie drucken) und den Löchern, die in dieser Ansicht drucken, jeder Blattstiel, Stamm, Stab und jede Wurzel; die Erde innerhalb des Rands im Grundriss."),
    merged: T("Merged: the narrow natural slits are widened to an opening of 26 degrees; a hole that a view foreshortens below the printable gap is closed in that view only.", "Zusammengefasst: Die schmalen natürlichen Schlitze sind auf eine Öffnung von 26 Grad geweitet; ein Loch, das eine Ansicht unter die druckbare Lücke verkürzt, ist nur in dieser Ansicht geschlossen."),
    omitted: T("Omitted: the vein pattern and the geniculum at the top of the petiole (below 0.2 mm).", "Weggelassen: das Adermuster und das Gelenk oben am Blattstiel (unter 0,2 mm)."),
    thinElements: T("Thin elements: petioles and aerial roots are strokes of their true width (0.16 to 0.35 mm), stem and pole filled outlines; a blade seen almost edge-on is drawn as its outline.", "Dünne Elemente: Blattstiele und Luftwurzeln sind Striche ihrer wahren Breite (0,16 bis 0,35 mm), Stamm und Stab gefüllte Umrisse; eine fast in Kantenansicht gesehene Spreite ist als Umriss gezeichnet."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("Lines at 0.18 mm; the solid styles fill the blades in deep greens, the pot light and the compost darkest.", "Linien mit 0,18 mm; die Flächenstile füllen die Spreiten in tiefen Grüntönen, den Topf hell und die Erde am dunkelsten."),
    overlays: T("Thermal: blades carry the upper or lower leaf region, stalks the stem region, the pole, pot, saucer and compost their own; structural: stem, pole, petioles and midrib strips are members, the saucer carries the support.", "Thermik: Spreiten tragen die obere oder untere Blattregion, Stiele die Stammregion, Stab, Topf, Untersetzer und Erde ihre eigenen; Tragwerk: Stamm, Stab, Blattstiele und Mittelrippenstreifen sind Stäbe, der Untersetzer trägt das Auflager."),
  }),
  100: decisions({
    silhouette: T("At 1:100 (1.6 m is 16 mm) the nine blades are redrawn on a coarser margin with their slits but without holes; stem and pole stay filled, petioles and roots are strokes; pot and saucer are one turned outline.", "Bei 1:100 (1,6 m sind 16 mm) sind die neun Spreiten auf einem gröberen Rand mit ihren Schlitzen, aber ohne Löcher neu gezeichnet; Stamm und Stab bleiben gefüllt, Blattstiele und Wurzeln sind Striche; Topf und Untersetzer sind ein gedrehter Umriss."),
    retained: T("Retained: every leaf with its slits, petioles, stem, pole.", "Beibehalten: jedes Blatt mit seinen Schlitzen, Blattstiele, Stamm, Stab."),
    merged: T("Merged: pot and saucer into one outline (the saucer band and the rim ring would be under 0.2 mm at 1:150).", "Zusammengefasst: Topf und Untersetzer zu einem Umriss (Untersetzerband und Randring wären bei 1:150 unter 0,2 mm)."),
    omitted: T("Omitted: the holes (narrower than the printable gap at 1:150).", "Weggelassen: die Löcher (schmaler als die druckbare Lücke bei 1:150)."),
    thinElements: T("Thin elements: petioles, stem and roots are strokes where they are thinner than 0.22 mm at 1:150.", "Dünne Elemente: Blattstiele, Stamm und Wurzeln sind Striche, wo sie bei 1:150 dünner als 0,22 mm sind."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("As at 1:50.", "Wie bei 1:50."),
    overlays: T("As at 1:50, with pot and saucer as one merged form.", "Wie bei 1:50, mit Topf und Untersetzer als eine zusammengefasste Form."),
  }),
  200: decisions({
    silhouette: T("At 1:200 (1.6 m is 8 mm) the nine blades are still drawn one by one as the true views of the 1:200 model, entire (without slits and holes), on strokes for petioles, stem and pole; pot and saucer are one turned outline.", "Bei 1:200 (1,6 m sind 8 mm) sind die neun Spreiten noch einzeln als wahre Ansichten des Modells 1:200 gezeichnet, ganzrandig (ohne Schlitze und Löcher), an Strichen für Blattstiele, Stamm und Stab; Topf und Untersetzer sind ein gedrehter Umriss."),
    retained: T("Retained: every blade with its outline, lean and roll, the petioles, stem, pole and aerial roots.", "Beibehalten: jede Spreite mit Umriss, Neigung und Rollung, die Blattstiele, Stamm, Stab und Luftwurzeln."),
    merged: T("Merged: pot and saucer into one outline.", "Zusammengefasst: Topf und Untersetzer zu einem Umriss."),
    omitted: T("Omitted: slits and holes (narrower than the printable gap at 1:350).", "Weggelassen: Schlitze und Löcher (schmaler als die druckbare Lücke bei 1:350)."),
    thinElements: T("Thin elements: petioles, stem, pole and roots are strokes of their true width (at least 0.1 mm); a blade seen almost edge-on is drawn as its outline.", "Dünne Elemente: Blattstiele, Stamm, Stab und Wurzeln sind Striche ihrer wahren Breite (mindestens 0,1 mm); eine fast in Kantenansicht gesehene Spreite ist als Umriss gezeichnet."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("Lines at 0.15 mm; the blades in two greens.", "Linien mit 0,15 mm; die Spreiten in zwei Grüntönen."),
    overlays: T("As at 1:100.", "Wie bei 1:100."),
  }),
  500: decisions({
    silhouette: T("At 1:500 (1.6 m is 3.2 mm) the crown is one closed outline of the leaves of the 1:500 model, closed for the printable gap at 1:1000, on the turned outline of pot and saucer, with the stem as one stroke.", "Bei 1:500 (1,6 m sind 3,2 mm) ist die Krone ein geschlossener Umriss der Blätter des Modells 1:500, geschlossen für die druckbare Lücke bei 1:1000, auf dem gedrehten Umriss von Topf und Untersetzer, mit dem Stamm als einem Strich."),
    retained: T("Retained: height, spread and the broad lobes of the crown, the pot.", "Beibehalten: Höhe, Ausbreitung und die breiten Lappen der Krone, der Topf."),
    merged: T("Merged: every leaf, petiole and the pole into the crown.", "Zusammengefasst: jedes Blatt, jeder Stiel und der Stab zur Krone."),
    omitted: T("Omitted: all leaf detail.", "Weggelassen: alle Blattdetails."),
    thinElements: T("Thin elements: the stem below the crown is a stroke of 0.13 mm.", "Dünne Elemente: Der Stamm unter der Krone ist ein Strich von 0,13 mm."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("Lines at 0.13 mm; the crown in one green.", "Linien mit 0,13 mm; die Krone in einem Grün."),
    overlays: T("As at 1:200.", "Wie bei 1:200."),
  }),
};

const VOL_DECISIONS: Record<Scale, Text[]> = {
  50: [T("Body at 1:50: nine 1.5 mm leaf sheets with slits and holes, slender petioles, stem segments, coir pole and aerial roots at their true sections, the pot as a shell with drainage hole around the compost fill on the saucer.", "Körper bei 1:50: neun 1,5 mm dicke Blattflächen mit Schlitzen und Löchern, schlanke Blattstiele, Stammstücke, Kokosstab und Luftwurzeln in wahrem Querschnitt, der Topf als Schale mit Abzugsloch um die Erdfüllung auf dem Untersetzer."), T("Thermal at 1:50: every surface by reference in its region, the compost surface as its own sheet that closes the pot, ten stomata outlets. Structure: stem, pole, petioles and roots by reference, a midrib strip per blade, the pot shell, the support ring on the floor and the mass of pot and compost.", "Thermik bei 1:50: jede Oberfläche per Verweis in ihrer Region, die Erdoberfläche als eigene Fläche, die den Topf schließt, zehn Spaltöffnungsauslässe. Tragwerk: Stamm, Stab, Blattstiele und Wurzeln per Verweis, ein Mittelrippenstreifen je Spreite, die Topfschale, der Auflagerring auf dem Boden und die Masse von Topf und Erde.")],
  100: [T("Body at 1:100: the blades with slits on a coarser margin and without holes, stalks as at 1:50 (petioles slender), the pot shell on fewer outline points of its saucer.", "Körper bei 1:100: die Spreiten mit Schlitzen auf gröberem Rand und ohne Löcher, Stiele wie bei 1:50 (Blattstiele schlank), die Topfschale auf weniger Umrisspunkten ihres Untersetzers."), T("Minimal: thermal and structure at 1:100 are those of 1:50 on the coarser solids; the structure is one member per bone, and leaving one out would leave a leaf or a stalk without its load path.", "Minimal: Thermik und Tragwerk bei 1:100 sind die von 1:50 auf den gröberen Körpern; das Tragwerk ist ein Stab je Knochen, und einen wegzulassen ließe ein Blatt oder einen Stiel ohne Lastpfad.")],
  200: [T("Body at 1:200: the blades as entire sheets on a seven-percent margin step, slender stalks, a filled pot of four outline points; the drawing merges them into the crown it can print.", "Körper bei 1:200: die Spreiten als ganzrandige Flächen auf einem Randschritt von sieben Prozent, schlanke Stiele, ein gefüllter Topf aus vier Umrisspunkten; die Zeichnung fasst sie zur druckbaren Krone zusammen."), T("Thermal and structure at 1:200 by reference; the members are those of 1:100 on the coarser solids, support pad and mass at least one smallest feature.", "Thermik und Tragwerk bei 1:200 per Verweis; die Stäbe sind die von 1:100 auf den gröberen Körpern, Auflager und Masse mindestens ein kleinstes Merkmal.")],
  500: [T("Body at 1:500: the blades as entire sheets on a fourteen-percent margin step, slender stalks, a three-point pot and a two-point saucer.", "Körper bei 1:500: die Spreiten als ganzrandige Flächen auf einem Randschritt von vierzehn Prozent, schlanke Stiele, ein Topf aus drei und ein Untersetzer aus zwei Punkten."), T("Thermal and structure at 1:500 as at 1:200.", "Thermik und Tragwerk bei 1:500 wie bei 1:200.")],
};
