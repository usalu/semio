/** 🎻️ PW-17 fiddle-leaf fig (Ficus lyrata) in a terracotta pot: a single slender trunk in three segments with twenty-two leathery fiddle leaves in a 2/5 spiral, every number stated by hand below (node height, azimuth, petiole, leaf angle, blade length and width); builds the whole `semio.staffage/v2` asset (rig, poses, clips, thermal, structure, four representations, four volumes, symbol).
 * @see ./lib.ts
 * @see ./leafy.ts
 * @see ./fig-leaf.ts */
import { BAND, EXPORTS, FEATURE, FIXED, RigBuilder, SCALES, T, UP, VIEWS, add, azimuth, cross, decisions, dot, drag, frameOf, hull2, lin, massOf, mul, ordered, phase, plateStroke, polyArea, project, r3, r4, r5, revolute, roundSection, toWorld, unit, worldMeshes, type Frame, type Json, type P, type Placed, type Scale, type Text, type V3 } from "./lib.ts";
import { crown, plate, stalk } from "./leafy.ts";
import { figBlade } from "./fig-leaf.ts";
import { keep, potParts, potStructure, type PotSpec } from "./pot.ts";

const ID = "fiddle-leaf-fig";

/** 🏺️ Classic terracotta flower pot 0.29 m high and 0.32 m across with a rim band, on a terracotta saucer: outline, shell with drainage hole and compost fill [u up from the floor, radius]. */
const POT: PotSpec = {
  saucer: [[0, 0.15], [0.005, 0.158], [0.028, 0.18], [0.034, 0.18], [0.034, 0.175], [0.016, 0.152]],
  pot: [[0.016, 0.115], [0.02, 0.12], [0.22, 0.148], [0.226, 0.158], [0.29, 0.162], [0.294, 0.16], [0.292, 0.144], [0.268, 0.142]],
  shell: [[0.016, 0.018], [0.016, 0.115], [0.02, 0.12], [0.22, 0.148], [0.226, 0.158], [0.29, 0.162], [0.294, 0.16], [0.292, 0.144], [0.2, 0.135], [0.04, 0.108], [0.03, 0.105], [0.03, 0.018], [0.016, 0.018]],
  fill: [[0.03, 0.1045], [0.04, 0.1075], [0.2, 0.1345], [0.27, 0.1415]],
  soilU: 0.27, soilR: 0.1415, rimInnerR: 0.144, rimTopU: 0.294, potOuterR: 0.162, saucerR: 0.18, saucerTopU: 0.034,
  materials: { pot: "terracotta", saucer: "terracotta", soil: "potting-compost" }, tones: { pot: 0.55, saucer: 0.5, soil: 0.12 },
};
/** 🪵️ Outer surface of trunk and petioles as the product measures it on the 1:50 thermal solids (the lateral areas add up to 0.127 m², but the petiole ends lie in the trunk and in the blades). */
const STALK_OUTER_M2 = 0.105;
const POT_PROFILES: Record<Scale, { saucer: P[]; pot: P[] }> = {
  50: { saucer: POT.saucer, pot: POT.pot },
  100: { saucer: keep(POT.saucer, [0, 2, 3, 4, 5]), pot: keep(POT.pot, [0, 2, 3, 4, 5, 6, 7]) },
  200: { saucer: keep(POT.saucer, [0, 3, 5]), pot: keep(POT.pot, [0, 2, 4, 7]) },
  500: { saucer: [[0, 0.16], [0.077, 0.18]], pot: [[0.016, 0.12], [0.294, 0.162], [0.268, 0.142]] },
};

/** 🪵️ The trunk in three segments (base point of the first, angle from +x in the side elevation, length, radii); each segment starts at the tip of the one below. */
const TRUNK = [
  { id: "trunk-1", angle: 88, length: 0.72, r0: 0.014, r1: 0.011 },
  { id: "trunk-2", angle: 91, length: 0.5, r0: 0.011, r1: 0.008 },
  { id: "trunk-3", angle: 94.5, length: 0.43, r0: 0.008, r1: 0.005 },
];
const TRUNK_BASE: V3 = [0, 0.03, 0];

/** 🍃️ One leaf: node height on the trunk, azimuth, petiole length, leaf angle above the horizontal (petiole and blade in one line), blade length and width. */
type Leaf = { k: number; y: number; az: number; P: number; e: number; L: number; W: number };
const LEAVES: Leaf[] = [
  { k: 0, y: 0.62, az: 20, P: 0.05, e: -30, L: 0.38, W: 0.24 }, { k: 1, y: 0.68, az: 164, P: 0.05, e: -26, L: 0.4, W: 0.25 }, { k: 2, y: 0.73, az: 308, P: 0.05, e: -24, L: 0.39, W: 0.25 },
  { k: 3, y: 0.79, az: 92, P: 0.05, e: -20, L: 0.38, W: 0.24 }, { k: 4, y: 0.84, az: 236, P: 0.048, e: -18, L: 0.37, W: 0.24 }, { k: 5, y: 0.89, az: 20, P: 0.048, e: -14, L: 0.36, W: 0.23 },
  { k: 6, y: 0.94, az: 164, P: 0.046, e: -12, L: 0.35, W: 0.22 }, { k: 7, y: 0.99, az: 308, P: 0.046, e: -10, L: 0.35, W: 0.22 }, { k: 8, y: 1.04, az: 92, P: 0.045, e: -6, L: 0.34, W: 0.21 },
  { k: 9, y: 1.09, az: 236, P: 0.044, e: -4, L: 0.33, W: 0.21 }, { k: 10, y: 1.14, az: 20, P: 0.043, e: 0, L: 0.32, W: 0.2 }, { k: 11, y: 1.19, az: 164, P: 0.042, e: 3, L: 0.31, W: 0.2 },
  { k: 12, y: 1.23, az: 308, P: 0.041, e: 6, L: 0.3, W: 0.19 }, { k: 13, y: 1.29, az: 92, P: 0.04, e: 8, L: 0.3, W: 0.19 }, { k: 14, y: 1.34, az: 236, P: 0.038, e: 11, L: 0.29, W: 0.18 },
  { k: 15, y: 1.39, az: 20, P: 0.037, e: 14, L: 0.28, W: 0.18 }, { k: 16, y: 1.44, az: 164, P: 0.036, e: 18, L: 0.27, W: 0.17 }, { k: 17, y: 1.49, az: 308, P: 0.035, e: 22, L: 0.26, W: 0.16 },
  { k: 18, y: 1.53, az: 92, P: 0.034, e: 26, L: 0.25, W: 0.16 }, { k: 19, y: 1.57, az: 236, P: 0.032, e: 30, L: 0.24, W: 0.15 }, { k: 20, y: 1.61, az: 20, P: 0.03, e: 34, L: 0.22, W: 0.14 },
  { k: 21, y: 1.65, az: 164, P: 0.028, e: 40, L: 0.2, W: 0.12 },
];
const nn = (k: number) => String(k + 1).padStart(2, "0");
const petioleId = (k: number) => `petiole-${nn(k)}`;
const bladeId = (k: number) => `blade-${nn(k)}`;
const THICK = 0.0016;
const LEAF_KG_M2 = 0.32;
const upper = (l: Leaf) => l.y >= 1.19;

/** 🪵️ The rest frames of the trunk segments (chained tip to base). */
function trunkFrames(): Record<string, Frame> {
  const f: Record<string, Frame> = {};
  let base = TRUNK_BASE;
  for (const t of TRUNK) {
    const u: V3 = [Math.cos(t.angle * Math.PI / 180), Math.sin(t.angle * Math.PI / 180), 0];
    f[t.id] = frameOf(base, u, [-u[1], u[0], 0]);
    base = add(base, mul(u, t.length));
  }
  return f;
}

/** 🧭️ The trunk segment that carries a node and the axis point of the node. */
function nodeOf(F: Record<string, Frame>, y: number): { bone: string; o: V3 } {
  for (const t of TRUNK) {
    const f = F[t.id]!;
    const s = (y - f.o[1]) / f.u[1];
    if (s <= t.length + 1e-9) return { bone: t.id, o: add(f.o, mul(f.u, s)) };
  }
  throw new Error(`no trunk at ${y}`);
}

/** 🎯️ The roll of a blade about its midrib: the smallest turn (in steps of 2 degrees up to 40) that keeps the blade normal at least 0.3 off every principal view direction, so that no blade is seen edge-on in the side, front or top view. */
function rollOf(u: V3, n0: V3): number {
  let best = { r: 0, score: -1 };
  for (let k = 0; k <= 40; k += 2) for (const r of k === 0 ? [0] : [k, -k]) {
    const a = r * Math.PI / 180;
    const n = lin(n0, Math.cos(a), cross(u, n0), Math.sin(a));
    const score = Math.min(Math.abs(n[0]), Math.abs(n[1]), Math.abs(n[2]));
    if (score >= 0.3) return r;
    if (score > best.score) best = { r, score };
  }
  return best.r;
}

/** 🧭️ The rest frames of every bone. */
function frames(): { F: Record<string, Frame>; nodes: Record<number, string>; rolls: Record<number, number> } {
  const F: Record<string, Frame> = { pot: frameOf([0, 0, 0], UP, [-1, 0, 0]), ...trunkFrames() };
  const nodes: Record<number, string> = {};
  const rolls: Record<number, number> = {};
  for (const l of LEAVES) {
    const { bone, o } = nodeOf(F, l.y);
    nodes[l.k] = bone;
    const d = azimuth(l.az);
    const e = l.e * Math.PI / 180;
    const u = unit(lin(d, Math.cos(e), UP, Math.sin(e)));
    const n0 = unit(lin(d, -Math.sin(e), UP, Math.cos(e)));
    const r = rollOf(u, n0);
    rolls[l.k] = r;
    const a = r * Math.PI / 180;
    const n = unit(lin(n0, Math.cos(a), cross(u, n0), Math.sin(a)));
    F[petioleId(l.k)] = frameOf(o, u, n);
    F[bladeId(l.k)] = frameOf(add(o, mul(u, l.P)), u, n);
  }
  return { F, nodes, rolls };
}

/** 🧾️ Builds the asset document. */
export function build(): { doc: Json; distinction: Text; notes: string[] } {
  const notes: string[] = [];
  const { F, nodes, rolls } = frames();
  notes.push(`[DEBUG] blade rolls ${JSON.stringify(rolls)}`);
  const rig = new RigBuilder();
  const bladeArea = (l: Leaf) => polyArea(figBlade(l.L, l.W, 50));
  const potKg = 5.6 + 1.3 + 8.8;
  rig.add({ id: "pot", parent: null, title: T("Terracotta pot with saucer, compost and root ball", "Terrakottatopf mit Untersetzer, Erde und Wurzelballen"), f: F.pot!, length: POT.rimTopU, joint: FIXED, mass: massOf(potKg, 0.13, 0, "point", "authored-estimate", "en-13040") });
  TRUNK.forEach((t, i) => rig.add({ id: t.id, parent: i === 0 ? "pot" : TRUNK[i - 1]!.id, title: T(["Trunk, lower segment (bare)", "Trunk, middle segment", "Trunk, top segment"][i]!, ["Stamm, unteres Stück (kahl)", "Stamm, mittleres Stück", "Stamm, oberes Stück"][i]!), f: F[t.id]!, length: t.length, joint: i === 0 ? FIXED : revolute(-6, 6), mass: massOf(Math.PI * ((t.r0 + t.r1) / 2) ** 2 * t.length * 900, t.length / 2, 0, "uniform", "literature", "wood-handbook-2021"), section: roundSection(t.r0 + t.r1, "ficus-wood", T("Green fig wood", "Grünes Feigenholz")) }));
  for (const l of LEAVES) {
    rig.add({ id: petioleId(l.k), parent: nodes[l.k]!, title: T(`Petiole ${l.k + 1}`, `Blattstiel ${l.k + 1}`), f: F[petioleId(l.k)]!, length: l.P, joint: revolute(-20, 12), mass: massOf(Math.PI * 0.004 ** 2 * l.P * 1000, l.P / 2, 0, "uniform", "authored-estimate", "berg-corner-2005") });
    rig.add({ id: bladeId(l.k), parent: petioleId(l.k), title: T(`Fiddle leaf ${l.k + 1} (${Math.round(l.L * 100)} cm)`, `Geigenblatt ${l.k + 1} (${Math.round(l.L * 100)} cm)`), f: F[bladeId(l.k)]!, length: l.L, joint: revolute(-25, 15), mass: massOf(bladeArea(l) * LEAF_KG_M2, 0.55 * l.L, 0, "uniform", "authored-estimate", "berg-corner-2005") });
  }
  notes.push(`[DEBUG] rig frames vs product restFrames: max error ${rig.verify().toExponential(2)}`);
  const leafArea = LEAVES.reduce((s, l) => s + bladeArea(l), 0);
  const upperArea = LEAVES.filter(upper).reduce((s, l) => s + bladeArea(l), 0);
  const lowerArea = leafArea - upperArea;
  const totalKg = rig.bones.reduce((s: number, b: Json) => s + (b.mass?.kg ?? 0), 0);
  notes.push(`[DEBUG] total mass ${totalKg.toFixed(3)} kg, one-sided leaf area ${leafArea.toFixed(3)} m2 (upper ${upperArea.toFixed(3)})`);

  const stub = { id: ID, dimensions: { heightM: 1.8, widthM: 0.8, depthM: 0.8, massKg: totalKg }, rig: rig.rig() };
  const leafTone = (l: Leaf) => r3(0.34 + 0.16 * phase(l.k, 11) + (upper(l) ? 0.08 : 0));
  const leafRegion = (l: Leaf) => (upper(l) ? "leaves-upper" : "leaves-lower");
  const representations: Json = {};
  const volumes: Json = {};
  for (const scale of SCALES) {
    const pot = potParts(POT, scale, POT_PROFILES[scale]);
    const placed: Placed[] = [...pot.placed];
    const planPlaced: Placed[] = [...pot.plan];
    const body: Json[] = [...pot.body];
    const thermal: Json[] = [...pot.thermal];
    const structure: Json[] = [...potStructure(POT, scale)];
    structure.push({ id: "m-root-ball", kind: "revolve", bone: "pot", material: "potting-compost", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[0.03, 0.06], [0.2, 0.06]], member: "pot", part: "member" });
    const fine = scale <= 100;
    const drawLeaves = scale <= 200;
    const pen = PENS[scale].body;
    const plantSolids: Json[] = [];
    const add2 = (list: Placed[]) => { for (const p of list) { if (p.view === "plan") planPlaced.push(p); else if (p.view) placed.push(p); else { placed.push(p); planPlaced.push(p); } } };
    TRUNK.forEach((t, i) => {
      const s = stalk(t.id, t.id, F[t.id]!, i === 0 ? 0 : -0.006, t.length, t.r0, t.r1, scale, { tone: 0.4, region: "trunk-petioles", material: "ficus-wood" }, { material: "ficus-wood", tone: 0.4 });
      body.push(s.solid); plantSolids.push(s.solid);
      thermal.push({ id: `th-${t.id}`, of: t.id, region: "trunk-petioles" });
      structure.push({ id: `m-${t.id}`, of: t.id, member: t.id, part: "member" });
      add2(s.placed.map((p) => ({ ...p, rank: -0.5 })));
    });
    for (const l of LEAVES) {
      if (fine) {
        const s = stalk(petioleId(l.k), petioleId(l.k), F[petioleId(l.k)]!, 0, l.P + 0.004, 0.004, 0.0035, scale, { tone: 0.45, region: "trunk-petioles", material: "ficus-wood" }, { material: "ficus-wood", tone: 0.45 });
        body.push(s.solid); plantSolids.push(s.solid);
        thermal.push({ id: `th-${petioleId(l.k)}`, of: petioleId(l.k), region: "trunk-petioles" });
        add2(s.placed);
        structure.push({ id: `m-${petioleId(l.k)}`, of: petioleId(l.k), member: petioleId(l.k), part: "member" });
      }
      const outline = figBlade(l.L, l.W, scale, fine ? 0 : l.P + 0.004);
      const pl = plate(bladeId(l.k), bladeId(l.k), F[bladeId(l.k)]!, outline, [], -THICK / 2, THICK / 2, scale, { tone: leafTone(l), region: leafRegion(l), material: "ficus-leaf" }, { material: "ficus-leaf", tone: leafTone(l), sheet: true }, pen, scale === 200 && l.k === 0);
      body.push(pl.solid); plantSolids.push(pl.solid);
      thermal.push({ id: `th-${bladeId(l.k)}`, of: bladeId(l.k), region: leafRegion(l) });
      if (drawLeaves) add2(pl.placed);
      if (scale === 50) {
        const centre = toWorld(F[bladeId(l.k)]!, [0.5 * l.L, 0, 0]);
        for (const view of VIEWS) if (!pl.unfilled.includes(view)) add2([{ view, at: add(centre, mul(F[bladeId(l.k)]!.v, 0.001)), rank: 1, shape: plateStroke(`${bladeId(l.k)}-midrib`, bladeId(l.k), [[0, 0], [0.975 * l.L, 0]], THICK / 2 + 0.0003, { role: "detail", tone: 0.25 }) }]);
      }
      if (scale === 50 && pl.unfilled.length) notes.push(`[DEBUG] 1:50 ${bladeId(l.k)} unfilled ${pl.unfilled.join(",")}`);
      const reach = { 50: 0.85, 100: 0.84, 200: 0.82, 500: 0.8 }[scale];
      const hw = 0.008;
      structure.push({ id: `m-${bladeId(l.k)}`, kind: "extrude", bone: bladeId(l.k), material: "ficus-leaf", role: "body", tone: 0.5, slender: true, plane: "zx", profile: [[-hw, fine ? -0.004 : -(l.P + 0.004)], [-hw * 0.6, r5(reach * l.L)], [hw * 0.6, r5(reach * l.L)], [hw, fine ? -0.004 : -(l.P + 0.004)]], from: r5(-THICK / 2 + 0.0002), to: r5(THICK / 2 - 0.0002), member: bladeId(l.k), part: "member" });
    }
    if (false) add2(crown(stub, F, scale, plantSolids, "pot", F.pot!, pen, { tone: 0.42, regions: ["leaves-upper", "leaves-lower", "trunk-petioles"], material: "ficus-leaf" }, "crown").map((p) => ({ ...p, rank: p.view === "plan" ? 3e6 : 1.5e6 })));
    for (const a of ANCHORS) {
      const l = LEAVES[a.leaf]!;
      const fe = FEATURE[scale];
      const rr = Math.max(0.004, fe / 2 + 0.001);
      thermal.push({ id: `th-${a.id}`, kind: "ellipsoid", bone: bladeId(l.k), material: "ficus-leaf", role: "body", tone: 0.8, radii: [r5(Math.max(0.03, 2 * rr)), r5(rr), r5(Math.max(0.02, rr))], transform: { translation: [r5(0.55 * l.L), r5(THICK / 2 + rr * 0.6), 0] }, region: leafRegion(l), tags: ["emitter", upper(l) ? "transpiration-upper" : "transpiration-lower"] });
    }
    const geometry = ordered(placed, { plan: planPlaced });
    representations[String(scale)] = scale === 500 ? UNAVAILABLE_500 : { status: "authored", denominator: scale, validRange: { minDenominator: BAND[scale][0], maxDenominator: BAND[scale][1] }, pens: PENS[scale], hatches: [], geometry, decisions: REP_DECISIONS[scale] };
    volumes[String(scale)] = { status: "authored", denominator: scale, body, thermal, structure, decisions: VOL_DECISIONS[scale] };
  }

  const anchors: Json[] = [
    { id: "base-support", bone: "pot", x: 0, y: 0, kind: "support", title: T("Saucer on the floor", "Untersetzer auf dem Boden"), support: "fixed", normalDeg: 90, footprintLengthM: 0.32, footprintWidthM: 0.32, frictionCoefficient: 0.5 },
    ...ANCHORS.map((a) => ({ id: a.id, bone: bladeId(a.leaf), x: r4(0.55 * LEAVES[a.leaf]!.L), y: r4(THICK / 2 + 0.001), z: 0, kind: "emitter", title: T(`Stomata outlet on leaf ${a.leaf + 1}`, `Spaltöffnungsauslass an Blatt ${a.leaf + 1}`) })),
  ];

  const v = 1.0;
  const nx = (l: Leaf) => dot(F[bladeId(l.k)]!.v, [1, 0, 0]);
  const loads = LEAVES.map((l) => ({ id: `wind-${bladeId(l.k)}`, kind: "wind", title: T(`Draught on leaf ${l.k + 1}`, `Luftzug an Blatt ${l.k + 1}`), bone: bladeId(l.k), x: r4(0.5 * l.L), y: 0, fxN: r5(drag(1.2, v, bladeArea(l) * Math.max(0.2, Math.abs(nx(l))))), fyN: 0, source: "literature", reference: "en-1991-1-4" }));
  const bend = (l: Leaf) => -1.2 * Math.sign(nx(l)) * (l.L / 0.4);
  const poseBones = (pet: (l: Leaf) => number, bl: (l: Leaf) => number, top = 0) => ({ ...(top ? { "trunk-3": { angleDeg: top }, "trunk-2": { angleDeg: top / 2 } } : {}), ...Object.fromEntries(LEAVES.flatMap((l) => [[petioleId(l.k), { angleDeg: r4(pet(l)) }], [bladeId(l.k), { angleDeg: r4(bl(l)) }]])) });
  const regionsOf = (upSun: number, lowSun: number, potSun: number, soilWet: number, potWet: number) => ({ "leaves-upper": { exposedFraction: 1, sunlitFraction: upSun }, "leaves-lower": { exposedFraction: 1, sunlitFraction: lowSun }, "trunk-petioles": { exposedFraction: 1, sunlitFraction: lowSun * 0.6 }, "pot-wall": { exposedFraction: 1, sunlitFraction: potSun, ...(potWet > 0 ? { wettedness: potWet } : {}) }, saucer: { exposedFraction: 1, sunlitFraction: potSun * 0.5, ...(potWet > 0 ? { wettedness: potWet } : {}) }, "soil-surface": { exposedFraction: 1, sunlitFraction: potSun * 0.3, ...(soilWet > 0 ? { wettedness: soilWet } : {}) } });
  const poses: Json[] = [
    { id: "still-air", title: T("Still air, bright room", "Ruhende Luft, heller Raum"), description: T("The fig stands a metre from a large window in a heated living room with still air; the upper leaves get more of the diffuse daylight, the stomata are open, the compost and the porous terracotta are evenly moist.", "Die Feige steht einen Meter vor einem großen Fenster in einem beheizten Wohnraum bei ruhender Luft; die oberen Blätter erhalten mehr diffuses Tageslicht, die Spaltöffnungen sind offen, Erde und poröse Terrakotta sind gleichmäßig feucht."), bones: {}, ground: "authored", contacts: [], thermal: { regions: regionsOf(0.5, 0.3, 0.2, 0, 0), emitters: { "transpiration-upper": 0.85, "transpiration-lower": 0.6 } }, loads: [] },
    { id: "ventilation-draught", title: T("Draught from an open window", "Luftzug aus einem offenen Fenster"), description: T(`The room is aired: ${v} m/s of air flow towards +x through the crown; each leaf is pushed by cf 1.2 times the dynamic pressure on its face seen from ahead and turns by about a degree, the slender top of the trunk leans by half a degree.`, `Der Raum wird gelüftet: ${String(v).replace(".", ",")} m/s Luft strömen in Richtung +x durch die Krone; jedes Blatt wird mit cf 1,2 mal dem Staudruck auf seine von vorn gesehene Fläche gedrückt und dreht sich um etwa ein Grad, die schlanke Stammspitze neigt sich um ein halbes Grad.`), bones: poseBones(() => 0, bend, -0.5), ground: "authored", contacts: [], thermal: { regions: regionsOf(0.5, 0.3, 0.2, 0, 0), emitters: { "transpiration-upper": 1, "transpiration-lower": 0.72 } }, loads },
    { id: "freshly-watered", title: T("Freshly watered", "Frisch gegossen"), description: T("1.5 litres of water have soaked the compost and darkened the porous terracotta (14.7 N more in the pot); the wet pot wall cools by evaporation.", "1,5 Liter Wasser haben die Erde durchfeuchtet und die poröse Terrakotta gedunkelt (14,7 N mehr im Topf); die nasse Topfwand kühlt durch Verdunstung."), bones: poseBones(() => 1, () => 1.5), ground: "authored", contacts: [], thermal: { regions: regionsOf(0.5, 0.3, 0.2, 1, 0.6), emitters: { "transpiration-upper": 0.9, "transpiration-lower": 0.65 } }, loads: [{ id: "irrigation-water", kind: "imposed", title: T("Water held in compost and terracotta", "In Erde und Terrakotta gehaltenes Wasser"), bone: "pot", x: 0.14, y: 0, fxN: 0, fyN: -14.71, source: "authored-estimate", reference: "en-13040" }] },
    { id: "drought-drooping", title: T("Drooping after drought", "Hängend nach Trockenheit"), description: T("The compost has dried out: the leathery leaves lose turgor and hang 6 to 12 degrees lower on their petioles, the stomata are nearly shut.", "Die Erde ist ausgetrocknet: Die ledrigen Blätter verlieren an Spannung und hängen an ihren Stielen 6 bis 12 Grad tiefer, die Spaltöffnungen sind fast geschlossen."), bones: poseBones((l) => -6 - 6 * (l.L / 0.4), () => -2), ground: "authored", contacts: [], thermal: { regions: regionsOf(0.5, 0.3, 0.2, 0, 0), emitters: { "transpiration-upper": 0.1, "transpiration-lower": 0.07 } }, loads: [] },
  ];
  const T0 = 2;
  const swayKeys = Array.from({ length: 11 }, (_, i) => {
    const t = (T0 * i) / 10;
    const w = (k: number, a: number) => a * Math.sin(2 * Math.PI * (t / T0) + 2 * Math.PI * phase(k, 13));
    return { t: r4(t), pose: "ventilation-draught", ease: "smooth", bones: { "trunk-3": { angleDeg: r4(-0.5 + w(40, 0.35)) }, "trunk-2": { angleDeg: r4(-0.25 + w(40, 0.15)) }, ...Object.fromEntries(LEAVES.flatMap((l) => [[petioleId(l.k), { angleDeg: r4(w(l.k, 0.5)) }], [bladeId(l.k), { angleDeg: r4(bend(l) + w(l.k + 30, 1.6 * l.L / 0.4)) }]])) } };
  });
  const gustT = [0, 0.8, 1.6, 2.2, 3, 4, 6];
  const gustG = [1, 1.5, 2.6, 3, 2.2, 1.3, 1];
  const gustKeys = gustT.map((t, i) => ({ t, pose: "ventilation-draught", ease: "smooth", bones: { "trunk-3": { angleDeg: r4(-0.5 * gustG[i]! ** 2) }, "trunk-2": { angleDeg: r4(-0.25 * gustG[i]! ** 2) }, ...Object.fromEntries(LEAVES.flatMap((l) => [[petioleId(l.k), { angleDeg: r4(-0.5 * (gustG[i]! ** 2 - 1) * Math.sign(nx(l))) }], [bladeId(l.k), { angleDeg: r4(bend(l) * Math.min(4, gustG[i]! ** 2)) }]])) } }));
  const clips: Json[] = [
    { id: "wind-sway", title: T("Draught flutter", "Flattern im Luftzug"), description: T(`A periodic ${T0} s cycle in the airing draught: every leaf nods by about one and a half degrees about its mean turn with its own phase, the slender top of the trunk sways by a third of a degree.`, `Ein periodischer Zyklus von ${T0} s im Lüftungszug: Jedes Blatt nickt um etwa anderthalb Grad um seine mittlere Drehung mit eigener Phase, die schlanke Stammspitze schwingt um ein Drittel Grad.`), durationS: T0, fps: 30, loop: true, rootMotion: "none", keys: swayKeys, contacts: [], phases: [{ id: "flutter", fromS: 0, toS: T0, title: T("Flutter cycle", "Flatterzyklus") }], events: [{ t: 0, kind: "motion", title: T("Cycle start at the mean turn", "Zyklusbeginn in der mittleren Drehung") }], tracks: [] },
    { id: "gust", title: T("Door gust", "Türböe"), description: T("A door opens across the airing window: the draught rises from 1 to about 3 m/s within two seconds and settles; the leaves turn up to four times their mean angle, the trunk top leans by up to 4.5 degrees and the drag follows the square of the speed.", "Eine Tür öffnet sich gegenüber dem offenen Fenster: Der Luftzug steigt binnen zwei Sekunden von 1 auf etwa 3 m/s und legt sich; die Blätter drehen bis zum Vierfachen ihres mittleren Winkels, die Stammspitze neigt sich um bis zu 4,5 Grad, der Widerstand folgt dem Quadrat der Geschwindigkeit."), durationS: 6, fps: 30, loop: false, rootMotion: "none", keys: gustKeys, contacts: [], phases: [{ id: "rise", fromS: 0, toS: 2.2, title: T("Gust front", "Böenfront") }, { id: "peak", fromS: 2.2, toS: 3, title: T("Peak", "Spitze") }, { id: "decay", fromS: 3, toS: 6, title: T("Settling", "Abklingen") }], events: [{ t: 2.2, kind: "load-on", title: T("Peak draught load", "Spitzenlast des Luftzugs") }, { t: 4, kind: "load-off", title: T("Load back near the mean", "Last wieder nahe dem Mittel") }], tracks: loads.map((d) => ({ target: `load:${d.id}`, ease: "smooth", keys: gustT.map((t, i) => ({ t, v: r3(gustG[i]! ** 2) })) })) },
    { id: "diurnal-transpiration", title: T("Day and night by the window", "Tag und Nacht am Fenster"), description: T("24 hours in 24 seconds (1 s = 1 h): the C3 stomata open with the morning light, transpiration peaks around noon when daylight from the window reaches the upper leaves, and closes after dusk; the wet terracotta keeps evaporating through the night.", "24 Stunden in 24 Sekunden (1 s = 1 h): Die C3-Spaltöffnungen öffnen sich mit dem Morgenlicht, die Transpiration erreicht um Mittag ihr Maximum, wenn das Tageslicht vom Fenster die oberen Blätter erreicht, und schließt nach der Dämmerung; die feuchte Terrakotta verdunstet die Nacht hindurch weiter."), durationS: 24, fps: 10, loop: true, rootMotion: "none", keys: [{ t: 0, pose: "still-air", ease: "smooth" }, { t: 12, pose: "still-air", ease: "smooth" }, { t: 24, pose: "still-air", ease: "smooth" }], contacts: [], phases: [{ id: "night", fromS: 0, toS: 7, title: T("Night", "Nacht") }, { id: "morning", fromS: 7, toS: 12, title: T("Morning to noon", "Morgen bis Mittag") }, { id: "afternoon", fromS: 12, toS: 18, title: T("Afternoon", "Nachmittag") }, { id: "evening", fromS: 18, toS: 24, title: T("Evening and night", "Abend und Nacht") }], events: [{ t: 7.5, kind: "thermal", title: T("Daylight reaches the leaves", "Tageslicht erreicht die Blätter") }, { t: 12.5, kind: "thermal", title: T("Brightest light", "Hellstes Licht") }, { t: 17.5, kind: "thermal", title: T("Daylight fades", "Tageslicht schwindet") }], tracks: [
      { target: "emitter:transpiration-upper", ease: "smooth", keys: [{ t: 0, v: 0.05 }, { t: 6.5, v: 0.05 }, { t: 8.5, v: 0.5 }, { t: 11, v: 0.92 }, { t: 13, v: 1 }, { t: 15.5, v: 0.78 }, { t: 18, v: 0.28 }, { t: 19.5, v: 0.06 }, { t: 24, v: 0.05 }] },
      { target: "emitter:transpiration-lower", ease: "smooth", keys: [{ t: 0, v: 0.04 }, { t: 6.5, v: 0.04 }, { t: 8.5, v: 0.32 }, { t: 11, v: 0.62 }, { t: 13, v: 0.7 }, { t: 15.5, v: 0.55 }, { t: 18, v: 0.2 }, { t: 19.5, v: 0.05 }, { t: 24, v: 0.04 }] },
      { target: "region:leaves-upper:sunlitFraction", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 7, v: 0 }, { t: 9, v: 0.3 }, { t: 12.5, v: 0.5 }, { t: 16, v: 0.34 }, { t: 18, v: 0.02 }, { t: 19, v: 0 }, { t: 24, v: 0 }] },
      { target: "region:leaves-lower:sunlitFraction", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 7, v: 0 }, { t: 9, v: 0.15 }, { t: 12.5, v: 0.3 }, { t: 16, v: 0.2 }, { t: 18, v: 0.01 }, { t: 19, v: 0 }, { t: 24, v: 0 }] },
    ] },
  ];

  const crownProjection = Math.PI * 0.4 * 0.4;
  const thermal = {
    environment: { airTempC: 21, meanRadiantTempC: 20.5, relativeHumidity: 0.45, windMS: 0.05, windDirectionDeg: 0, solarWM2: 150, diffuseFraction: 0.7, pressurePa: 101325 },
    referenceEnvironment: "office-still",
    passengers: [], operating: [],
    regions: [
      { id: "leaves-upper", title: T("Upper fiddle leaves (younger, nearer the light)", "Obere Geigenblätter (jünger, näher am Licht)"), kind: "foliage", model: "leaf", material: "ficus-leaf", areaM2: r4(upperArea), areaBasis: "exchange", characteristicLengthM: 0.2, geometry: "vertical-plate", skyViewFactor: 0.55, layers: [], heatCapacityJM2K: 2400, leaf: { stomatalConductanceMaxMS: 0.0035, leafAreaIndex: r3(upperArea / crownProjection), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
      { id: "leaves-lower", title: T("Lower fiddle leaves (older, shaded)", "Untere Geigenblätter (älter, beschattet)"), kind: "foliage", model: "leaf", material: "ficus-leaf", areaM2: r4(lowerArea), areaBasis: "exchange", characteristicLengthM: 0.24, geometry: "vertical-plate", skyViewFactor: 0.35, layers: [], heatCapacityJM2K: 2600, leaf: { stomatalConductanceMaxMS: 0.0028, leafAreaIndex: r3(lowerArea / crownProjection), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
      { id: "trunk-petioles", title: T("Trunk and petioles", "Stamm und Blattstiele"), kind: "bark", model: "passive", material: "ficus-wood", areaM2: STALK_OUTER_M2, characteristicLengthM: 0.02, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 14000, source: "authored-estimate", reference: "wood-handbook-2021" },
      { id: "pot-wall", title: T("Terracotta pot wall and rim band", "Terrakotta-Topfwand mit Randband"), kind: "surface", model: "passive", material: "terracotta", areaM2: 0.36, characteristicLengthM: 0.3, geometry: "vertical-plate", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: 16000, source: "authored-estimate", reference: "iso-10456" },
      { id: "saucer", title: T("Terracotta saucer", "Terrakotta-Untersetzer"), kind: "surface", model: "passive", material: "terracotta", areaM2: 0.185, characteristicLengthM: 0.36, geometry: "horizontal-plate-up", skyViewFactor: 0.25, layers: [], heatCapacityJM2K: 11000, source: "authored-estimate", reference: "iso-10456" },
      { id: "soil-surface", title: T("Compost surface", "Erdoberfläche"), kind: "surface", model: "passive", material: "potting-compost", areaM2: r4(Math.PI * POT.soilR ** 2), characteristicLengthM: 0.28, geometry: "horizontal-plate-up", skyViewFactor: 0.25, layers: [], heatCapacityJM2K: 39000, source: "authored-estimate", reference: "en-13040" },
    ],
    emitters: [
      { id: "transpiration-upper", kind: "transpiration", title: T("Transpiration of the upper leaves (five outlets)", "Transpiration der oberen Blätter (fünf Auslässe)"), anchor: "tp-up-1", anchors: ANCHORS.filter((a) => upper(LEAVES[a.leaf]!)).map((a) => a.id), directionDeg: 90, diameterM: 0.2, region: "leaves-upper", source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-lower", kind: "transpiration", title: T("Transpiration of the lower leaves (five outlets)", "Transpiration der unteren Blätter (fünf Auslässe)"), anchor: "tp-low-1", anchors: ANCHORS.filter((a) => !upper(LEAVES[a.leaf]!)).map((a) => a.id), directionDeg: 90, diameterM: 0.24, region: "leaves-lower", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    assumptions: [
      T(`Leaf regions use the one-sided area of the twenty-two real leaves (${r3(upperArea)} m2 upper, ${r3(lowerArea)} m2 lower, over a crown projection of ${r3(crownProjection)} m2). The maximum stomatal conductances (0.0035 and 0.0028 m/s) are authored estimates for a leathery C3 fig indoors, guided by Jones (2014); blades of about 0.6 mm store 2.4 to 2.6 kJ/(m2 K).`, `Blattregionen verwenden die einseitige Fläche der zweiundzwanzig wirklichen Blätter (${String(r3(upperArea)).replace(".", ",")} m2 oben, ${String(r3(lowerArea)).replace(".", ",")} m2 unten, über einer Kronenprojektion von ${String(r3(crownProjection)).replace(".", ",")} m2). Die maximalen Spaltöffnungsleitfähigkeiten (0,0035 und 0,0028 m/s) sind Autorenschätzungen für eine ledrige C3-Feige im Innenraum, angelehnt an Jones (2014); Spreiten von etwa 0,6 mm speichern 2,4 bis 2,6 kJ/(m2 K).`),
      T("Trunk and petioles are passive green wood; the porous terracotta pot and saucer are passive and wetted in the watered pose, so they cool by evaporation (unglazed clay after ISO 10456, 1900 kg/m3). The reference case is the still office-like room (office-still), the asset's own case adds diffuse daylight of 150 W/m2 from a large window.", "Stamm und Blattstiele sind passives grünes Holz; der poröse Terrakottatopf und sein Untersetzer sind passiv und in der gegossenen Pose benetzt, sodass sie durch Verdunstung kühlen (unglasierter Ton nach ISO 10456, 1900 kg/m3). Der Referenzfall ist der ruhige, büroähnliche Raum (office-still), der eigene Fall ergänzt diffuses Tageslicht von 150 W/m2 von einem großen Fenster."),
    ],
  };

  const asset: Json = {
    id: ID, category: "vegetation",
    title: T("Fiddle-Leaf Fig in a Pot", "Geigenfeige im Topf"),
    summary: T("A fiddle-leaf fig (Ficus lyrata) 1.8 m high in a 0.32 m terracotta pot on a saucer: a single slender trunk, bare for its lower metre, carries twenty-two large, leathery, glossy leaves in a spiral, violin-shaped with a broad rounded apex, a waist and a cordate base, 20 to 40 cm long on short petioles; the lower leaves hang, the young ones at the top stand up. A popular statement plant of bright living rooms.", "Eine Geigenfeige (Ficus lyrata) von 1,8 m Höhe in einem Terrakottatopf von 0,32 m auf einem Untersetzer: ein einzelner schlanker, im unteren Meter kahler Stamm trägt zweiundzwanzig große, ledrige, glänzende Blätter in einer Spirale, geigenförmig mit breiter runder Spitze, Taille und herzförmigem Grund, 20 bis 40 cm lang an kurzen Stielen; die unteren Blätter hängen, die jungen oben stehen aufrecht. Eine beliebte Solitärpflanze heller Wohnräume."),
    tags: ["ficus", "fig", "indoor", "potted", "houseplant", "broad-leaf", "terracotta"],
    provenance: {
      author: "Semio staffage plant worker PW-17", method: "handcrafted", created: "2026-11-22",
      references: [
        { id: "berg-corner-2005", citation: "Berg, C. C. and Corner, E. J. H. (2005). Moraceae: Ficus. Flora Malesiana, Series I, 17(2). National Herbarium Nederland, Leiden. (Ficus lyrata Warb.: leaves panduriform, 20-45 cm long, coriaceous, base cordate, apex rounded to emarginate, petiole 1-3(-5) cm.)" },
        { id: "mabberley-2017", citation: "Mabberley, D. J. (2017). Mabberley's Plant-Book, 4th ed. Cambridge University Press (Ficus lyrata, fiddle-leaf fig, a common houseplant)." },
        { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service, Madison. Green density and stiffness orders of small hardwood stems." },
        { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed. Cambridge University Press. Leaf energy balance, boundary layer and stomatal conductance, leaf area index." },
        { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed. Academic Press. Penman-Monteith leaf transpiration." },
        { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A." },
        { id: "iso-10456", citation: "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values. Clay (fired, porous): about 1900 kg/m3, 0.85 W/(m K), 840 J/(kg K)." },
        { id: "en-13040", citation: "EN 13040:2007 Soil improvers and growing media - Sample preparation for chemical and physical tests, determination of dry matter content, moisture content and laboratory compacted bulk density (the bulk density of moist potting compost of about 0.5 to 0.7 kg/l used for the pot mass)." },
      ],
      notes: T("All twenty-two leaves are stated by hand with their node, azimuth (a 2/5 spiral), petiole, leaf angle and blade size; the fiddle outline is a hand-drawn control table; the only rule is the roll of a blade about its midrib, the smallest turn that keeps it from being seen exactly edge-on in a principal view (`T/🧰️scripts/PW-17/fiddle-leaf-fig.ts`, `fig-leaf.ts`). At 1:200 and 1:500 each blade runs back over its petiole to the trunk as one sheet; the leaves are drawn one by one down to 1:200, the 1:500 drawing is unavailable with a plan symbol while a digital 1:500 model exists.", "Alle zweiundzwanzig Blätter sind von Hand mit Knoten, Azimut (eine 2/5-Spirale), Stiel, Blattwinkel und Spreitengröße angegeben; der Geigenumriss ist eine handgezeichnete Stützpunkttabelle; die einzige Regel ist die Rollung einer Spreite um ihre Mittelrippe, die kleinste Drehung, die verhindert, dass sie in einer Hauptansicht genau in Kantenansicht erscheint (`T/🧰️scripts/PW-17/fiddle-leaf-fig.ts`, `fig-leaf.ts`). Bei 1:200 und 1:500 läuft jede Spreite über ihren Stiel als eine Fläche zum Stamm zurück; die Blätter sind bis 1:200 einzeln gezeichnet, die Zeichnung 1:500 ist nicht verfügbar, mit einem Grundrisssymbol, während ein digitales Modell 1:500 besteht."),
    },
    dimensions: { heightM: 1.8, widthM: 0.8, depthM: 0.8, massKg: r3(totalKg) },
    materials: [
      { id: "ficus-leaf", title: T("Leathery, glossy fig leaf", "Ledriges, glänzendes Feigenblatt"), emissivity: 0.96, solarAbsorptivity: 0.78, densityKgM3: 900, specificHeatJKgK: 3600, source: "authored-estimate", reference: "jones-2014" },
      { id: "ficus-wood", title: T("Green fig wood with smooth grey bark", "Grünes Feigenholz mit glatter grauer Rinde"), emissivity: 0.94, solarAbsorptivity: 0.65, densityKgM3: 900, specificHeatJKgK: 2800, elasticModulusPa: 5e9, strengthPa: 40e6, source: "authored-estimate", reference: "wood-handbook-2021" },
      { id: "terracotta", title: T("Unglazed terracotta (porous fired clay)", "Unglasierte Terrakotta (poröser gebrannter Ton)"), emissivity: 0.93, solarAbsorptivity: 0.65, densityKgM3: 1900, conductivityWmK: 0.85, specificHeatJKgK: 840, source: "literature", reference: "iso-10456" },
      { id: "potting-compost", title: T("Moist potting compost (bark, coir, perlite)", "Feuchte Topferde (Rinde, Kokos, Perlit)"), emissivity: 0.95, solarAbsorptivity: 0.9, densityKgM3: 650, conductivityWmK: 0.35, specificHeatJKgK: 2000, source: "authored-estimate", reference: "en-13040" },
    ],
    rig: rig.rig(), volumes, anchors, representations,
    symbols: [SYMBOL()], poses, clips, thermal,
    structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["base-support"], assumptions: [T("One rigid body per evaluated pose, fixed at the saucer on the floor; the trunk, pushed into the compost down to the pot bottom, is a cantilever of solid green-wood sections (28 to 10 mm) carrying twenty-two leaves on short petioles, loaded by weight and the draught drag of its pose (EN 1991-1-4 pressure with cf 1.2, a few hundredths of a newton per leaf at 1 m/s); no failure is predicted.", "Ein starrer Körper je ausgewerteter Pose, eingespannt am Untersetzer auf dem Boden; der bis zum Topfboden in die Erde gesteckte Stamm ist ein Kragarm aus vollen Grünholzquerschnitten (28 bis 10 mm), der zweiundzwanzig Blätter an kurzen Stielen trägt, belastet durch Gewicht und den Widerstand des Luftzugs seiner Pose (Druck nach EN 1991-1-4 mit cf 1,2, einige Hundertstel Newton je Blatt bei 1 m/s); es wird kein Versagen vorhergesagt.")], clinical: false },
    presentation: { defaultPose: "still-air", defaultClip: "wind-sway", defaultScale: 50 },
    exports: EXPORTS, completion: "authored",
  };
  const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
  const distinction = T("A potted fiddle-leaf fig (Ficus lyrata) 1.8 m high: a slender single trunk with twenty-two large, leathery violin-shaped leaves in a spiral, in a 0.32 m terracotta pot on a saucer.", "Eine Geigenfeige (Ficus lyrata) im Topf, 1,8 m hoch: ein schlanker einzelner Stamm mit zweiundzwanzig großen, ledrigen, geigenförmigen Blättern in einer Spirale, in einem Terrakottatopf von 0,32 m auf einem Untersetzer.");
  return { doc, distinction, notes };
}

/** 💨️ The ten stomata outlets on real leaves (five lower, five upper). */
const ANCHORS = [
  { id: "tp-low-1", leaf: 1 }, { id: "tp-low-2", leaf: 4 }, { id: "tp-low-3", leaf: 6 }, { id: "tp-low-4", leaf: 8 }, { id: "tp-low-5", leaf: 10 },
  { id: "tp-up-1", leaf: 12 }, { id: "tp-up-2", leaf: 14 }, { id: "tp-up-3", leaf: 16 }, { id: "tp-up-4", leaf: 18 }, { id: "tp-up-5", leaf: 20 },
];

const PENS: Record<Scale, Json> = {
  50: { body: 0.18, detail: 0.1, texture: null, glazing: null, ground: 0.25 },
  100: { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  200: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.2 },
  500: { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.18 },
};

/** 🔣️ The plan sign: the outline of crown and pot in true extent in all three views, filled by {@link finish}. */
function SYMBOL(): Json {
  return {
    id: "fiddle-leaf-fig-plan-symbol", title: T("Plan sign of a potted fiddle-leaf fig", "Grundrisszeichen einer Geigenfeige im Topf"), description: T("A conventional sign for interior plans: the outline of the leaf crown around the pot with its true extent (about 0.8 m across); a separate symbol, never used silently for a pictorial scale.", "Ein konventionelles Zeichen für Innenraumpläne: der Umriss der Blattkrone um den Topf in wahrer Ausdehnung (etwa 0,8 m breit); ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."),
    semantics: "true-extent", units: "model-m", denominators: [500, 200], pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, hatches: [],
    geometry: { elevation: [], front: [], plan: [] },
    dimensionalNote: T("True extent in model metres: the outline is the reach of the leaves and the pot, about 0.8 m across and 1.8 m high.", "Wahre Ausdehnung in Modellmetern: Der Umriss ist die Reichweite der Blätter und des Topfs, etwa 0,8 m breit und 1,8 m hoch."),
  };
}

/** 🔣️ Fills the symbol outlines from the convex hull of the 1:50 body in each view. */
export function finish(asset: Json): void {
  const meshes = worldMeshes(asset, 50, asset.volumes["50"].body);
  for (const view of VIEWS) {
    const pts: P[] = [];
    for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) pts.push(project(view, [m.world[i], m.world[i + 1], m.world[i + 2]]));
    asset.symbols[0].geometry[view] = [{ id: "sign", bone: "pot", role: "body", z: 1, filled: false, tone: 0.5, regions: ["leaves-upper", "leaves-lower", "trunk-petioles", "pot-wall", "saucer", "soil-surface"], kind: "poly", points: hull2(pts).map(([a, b]) => [r4(a), r4(b)]), closed: true }];
  }
}

const UNAVAILABLE_500 = {
  status: "unavailable", denominator: 500, cause: "below-printable-feature",
  reason: T("At 1:500 the 1.8 m fig is 3.6 mm high: a 0.3 m leaf is 0.6 mm and at the end of the band (1:1000) two leaves that do not touch would have to stand 0.3 m apart, so the crown could only print as one blob without its leaves.", "Bei 1:500 ist die 1,8 m hohe Feige 3,6 mm hoch: ein 0,3 m großes Blatt misst 0,6 mm, und am Ende des Bands (1:1000) müssten zwei sich nicht berührende Blätter 0,3 m auseinanderstehen, sodass die Krone nur als ein Klecks ohne ihre Blätter drucken könnte."),
  limitingFeature: T("Gap between neighbouring leaves (about 0.1 m)", "Lücke zwischen benachbarten Blättern (etwa 0,1 m)"), limitingSizeM: 0.1,
  alternatives: [{ kind: "symbol", symbol: "fiddle-leaf-fig-plan-symbol" }, { kind: "scale", denominator: 200 }],
};

const REP_DECISIONS: Record<Scale, Json> = {
  50: decisions({
    silhouette: T("At 1:50 (1.8 m is 36 mm) every view is the true orthographic view of the 1:50 model: twenty-two fiddle leaves as plates on their own bones with the panduriform outline, the cordate base and the notched apex, short petioles, the slender trunk and the terracotta pot with its rim band on the saucer.", "Bei 1:50 (1,8 m sind 36 mm) ist jede Ansicht die wahre Parallelprojektion des Modells 1:50: zweiundzwanzig Geigenblätter als Platten auf eigenen Knochen mit geigenförmigem Umriss, herzförmigem Grund und eingekerbter Spitze, kurze Blattstiele, der schlanke Stamm und der Terrakottatopf mit Randband auf dem Untersetzer."),
    retained: T("Retained: every leaf with its waist and midrib, every petiole, the three trunk segments, rim band, saucer and the compost inside the rim in plan.", "Beibehalten: jedes Blatt mit Taille und Mittelrippe, jeder Blattstiel, die drei Stammstücke, Randband, Untersetzer und die Erde innerhalb des Rands im Grundriss."),
    merged: T("Merged: the 5 mm petioles are modelled at 8 mm (the 1:50 feature is 7.5 mm).", "Zusammengefasst: Die 5 mm dicken Blattstiele sind mit 8 mm modelliert (das Merkmal 1:50 sind 7,5 mm)."),
    omitted: T("Omitted: the lateral veins and the wavy margin (below 0.2 mm).", "Weggelassen: die Seitennerven und der gewellte Rand (unter 0,2 mm)."),
    thinElements: T("Thin elements: petioles and the upper trunk are strokes of their true width where they are thinner than 0.22 mm at 1:75; the midrib is a 0.1 mm detail line; a leaf seen almost edge-on is drawn as its outline.", "Dünne Elemente: Blattstiele und der obere Stamm sind Striche ihrer wahren Breite, wo sie bei 1:75 dünner als 0,22 mm sind; die Mittelrippe ist eine Detaillinie von 0,1 mm; ein fast in Kantenansicht gesehenes Blatt ist als Umriss gezeichnet."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("Lines at 0.18 mm, midribs at 0.1 mm; the solid styles fill the leaves in deep greens and the terracotta in a mid tone.", "Linien mit 0,18 mm, Mittelrippen mit 0,1 mm; die Flächenstile füllen die Blätter in tiefen Grüntönen und die Terrakotta in einem mittleren Ton."),
    overlays: T("Thermal: leaves carry the upper or lower leaf region, trunk and petioles the bark region, pot, saucer and compost their own; structural: trunk, petioles and midrib strips are members, the saucer carries the support.", "Thermik: Blätter tragen die obere oder untere Blattregion, Stamm und Stiele die Rindenregion, Topf, Untersetzer und Erde ihre eigenen; Tragwerk: Stamm, Stiele und Mittelrippenstreifen sind Stäbe, der Untersetzer trägt das Auflager."),
  }),
  100: decisions({
    silhouette: T("At 1:100 (1.8 m is 18 mm) the twenty-two leaves are redrawn on a coarser outline with a rounded apex; the trunk and petioles are strokes, pot and saucer one turned outline.", "Bei 1:100 (1,8 m sind 18 mm) sind die zweiundzwanzig Blätter auf einem gröberen Umriss mit runder Spitze neu gezeichnet; Stamm und Stiele sind Striche, Topf und Untersetzer ein gedrehter Umriss."),
    retained: T("Retained: every leaf with its waist, the trunk and petioles.", "Beibehalten: jedes Blatt mit Taille, Stamm und Stiele."),
    merged: T("Merged: pot and saucer into one outline; petioles are modelled at the 15 mm feature.", "Zusammengefasst: Topf und Untersetzer zu einem Umriss; Blattstiele sind mit dem Merkmal von 15 mm modelliert."),
    omitted: T("Omitted: midribs and the apex notch.", "Weggelassen: Mittelrippen und die Kerbe der Spitze."),
    thinElements: T("Thin elements: trunk and petioles are strokes of 0.1 to 0.28 mm.", "Dünne Elemente: Stamm und Stiele sind Striche von 0,1 bis 0,28 mm."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("As at 1:50.", "Wie bei 1:50."),
    overlays: T("As at 1:50, with pot and saucer as one merged form.", "Wie bei 1:50, mit Topf und Untersetzer als eine zusammengefasste Form."),
  }),
  200: decisions({
    silhouette: T("At 1:200 (1.8 m is 9 mm) the twenty-two leaves are still drawn one by one as the true views of the 1:200 model on a coarse fiddle outline that runs back over the petiole to the trunk; the trunk is a stroke, pot and saucer one turned outline.", "Bei 1:200 (1,8 m sind 9 mm) sind die zweiundzwanzig Blätter noch einzeln als wahre Ansichten des Modells 1:200 gezeichnet, auf einem groben Geigenumriss, der über den Stiel zum Stamm zurückläuft; der Stamm ist ein Strich, Topf und Untersetzer ein gedrehter Umriss."),
    retained: T("Retained: every leaf with its waist, lean and roll, the trunk.", "Beibehalten: jedes Blatt mit Taille, Neigung und Rollung, der Stamm."),
    merged: T("Merged: each petiole into the stalk of its blade; pot and saucer into one outline.", "Zusammengefasst: jeder Blattstiel in den Steg seiner Spreite; Topf und Untersetzer zu einem Umriss."),
    omitted: T("Omitted: midribs and the apex notch.", "Weggelassen: Mittelrippen und die Kerbe der Spitze."),
    thinElements: T("Thin elements: the trunk is a stroke of its true width; a leaf seen almost edge-on is drawn as its outline.", "Dünne Elemente: Der Stamm ist ein Strich seiner wahren Breite; ein fast in Kantenansicht gesehenes Blatt ist als Umriss gezeichnet."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("Lines at 0.15 mm; the leaves in two greens.", "Linien mit 0,15 mm; die Blätter in zwei Grüntönen."),
    overlays: T("As at 1:100.", "Wie bei 1:100."),
  }),
  500: decisions({
    silhouette: T("At 1:500 (1.8 m is 3.6 mm) the crown is one closed outline of the leaves of the 1:500 model, closed for the printable gap at 1:1000, the trunk one stroke, on the turned outline of pot and saucer.", "Bei 1:500 (1,8 m sind 3,6 mm) ist die Krone ein geschlossener Umriss der Blätter des Modells 1:500, geschlossen für die druckbare Lücke bei 1:1000, der Stamm ein Strich, auf dem gedrehten Umriss von Topf und Untersetzer."),
    retained: T("Retained: height, spread and the column of the crown, the bare trunk, the pot.", "Beibehalten: Höhe, Ausbreitung und die Säule der Krone, der kahle Stamm, der Topf."),
    merged: T("Merged: every leaf into the crown.", "Zusammengefasst: jedes Blatt zur Krone."),
    omitted: T("Omitted: all leaf detail.", "Weggelassen: alle Blattdetails."),
    thinElements: T("Thin elements: the trunk is a stroke of 0.13 mm.", "Dünne Elemente: Der Stamm ist ein Strich von 0,13 mm."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("Lines at 0.13 mm; the crown in one green.", "Linien mit 0,13 mm; die Krone in einem Grün."),
    overlays: T("As at 1:200.", "Wie bei 1:200."),
  }),
};

const VOL_DECISIONS: Record<Scale, Text[]> = {
  50: [T("Body at 1:50: twenty-two 1.6 mm leaf sheets with the notched apex, petioles at 8 mm, three slender trunk segments, the terracotta pot as a shell with drainage hole around the compost fill on the saucer.", "Körper bei 1:50: zweiundzwanzig 1,6 mm dicke Blattflächen mit eingekerbter Spitze, Blattstiele mit 8 mm, drei schlanke Stammstücke, der Terrakottatopf als Schale mit Abzugsloch um die Erdfüllung auf dem Untersetzer."), T("Thermal at 1:50: every surface by reference in its region, the compost surface as its own sheet that closes the pot, ten stomata outlets. Structure: trunk by reference, a midrib strip per leaf, the pot shell, the support ring on the floor and the mass of pot and compost.", "Thermik bei 1:50: jede Oberfläche per Verweis in ihrer Region, die Erdoberfläche als eigene Fläche, die den Topf schließt, zehn Spaltöffnungsauslässe. Tragwerk: Stamm per Verweis, ein Mittelrippenstreifen je Blatt, die Topfschale, der Auflagerring auf dem Boden und die Masse von Topf und Erde.")],
  100: [T("Body at 1:100: the leaves on a coarser outline, petioles at the 15 mm feature, trunk as at 1:50, the pot shell on fewer outline points of its saucer.", "Körper bei 1:100: die Blätter auf gröberem Umriss, Blattstiele mit dem Merkmal von 15 mm, Stamm wie bei 1:50, die Topfschale auf weniger Umrisspunkten ihres Untersetzers."), T("Minimal: thermal and structure at 1:100 are those of 1:50 on the coarser solids; the structure is one member per bone, and leaving one out would leave a leaf or a stalk without its load path.", "Minimal: Thermik und Tragwerk bei 1:100 sind die von 1:50 auf den gröberen Körpern; das Tragwerk ist ein Stab je Knochen, und einen wegzulassen ließe ein Blatt oder einen Stiel ohne Lastpfad.")],
  200: [T("Body at 1:200: each leaf as one sheet that runs back over its petiole to the trunk as a 14 mm stalk (the petiole alone would be a lump below the 30 mm feature), the slender trunk, a filled pot of four outline points.", "Körper bei 1:200: jedes Blatt als eine Fläche, die über ihren Stiel als 14 mm breiter Steg zum Stamm zurückläuft (der Stiel allein wäre ein Klumpen unter dem Merkmal von 30 mm), der schlanke Stamm, ein gefüllter Topf aus vier Umrisspunkten."), T("Thermal and structure at 1:200 by reference; the midrib strips run back to the trunk, support pad and mass at least one smallest feature.", "Thermik und Tragwerk bei 1:200 per Verweis; die Mittelrippenstreifen laufen zum Stamm zurück, Auflager und Masse mindestens ein kleinstes Merkmal.")],
  500: [T("Body at 1:500: each leaf as one sheet on six outline points with its stalk to the trunk, the slender trunk, a three-point pot and a two-point saucer.", "Körper bei 1:500: jedes Blatt als eine Fläche auf sechs Umrisspunkten mit ihrem Steg zum Stamm, der schlanke Stamm, ein Topf aus drei und ein Untersetzer aus zwei Punkten."), T("Thermal and structure at 1:500 as at 1:200.", "Thermik und Tragwerk bei 1:500 wie bei 1:200.")],
};
