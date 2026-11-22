/** 🐍️ PW-17 snake plant (Dracaena trifasciata, syn. Sansevieria trifasciata) in a glazed stoneware pot: ten stiff, upright sword leaves in three rhizome rosettes, every number stated by hand below; builds the whole `semio.staffage/v2` asset (rig, poses, clips, thermal with CAM night transpiration, structure, four representations, four volumes, symbol).
 * @see ./lib.ts
 * @see ./pot.ts */
import { worldMeshes } from "./lib.ts";
import { BAND, EXPORTS, FEATURE, FIXED, RigBuilder, SCALES, T, UP, VIEWS, hull2, hull3, project, toShapeCoords, union2, type View, add, azimuth, cross, decisions, dot, drag, frameOf, lin, massOf, mul, ordered, phase, plainShape, plateShape, plateSolid, plateStroke, polyArea, r3, r4, r5, revolute, revolveSolid, revolvedShape, sub, table, toLocal, toWorld, unit, type Frame, type Json, type P, type Placed, type Scale, type Text, type V3 } from "./lib.ts";
import { keep, potMaterials, potParts, potStructure, type PotSpec } from "./pot.ts";

const ID = "snake-plant";

/** 🏺️ Glazed stoneware cylinder pot 0.236 m high on a saucer: half profiles [u up from the floor, radius]. */
const POT: PotSpec = {
  saucer: [[0, 0.121], [0.004, 0.128], [0.026, 0.135], [0.032, 0.135], [0.032, 0.131], [0.016, 0.122]],
  pot: [[0.016, 0.098], [0.02, 0.102], [0.07, 0.107], [0.2, 0.113], [0.24, 0.116], [0.248, 0.118], [0.252, 0.115], [0.25, 0.101], [0.23, 0.099]],
  fill: [[0.028, 0.0845], [0.045, 0.0875], [0.2, 0.0965], [0.232, 0.0988]],
  shell: [[0.016, 0.015], [0.016, 0.098], [0.02, 0.102], [0.07, 0.107], [0.2, 0.113], [0.24, 0.116], [0.248, 0.118], [0.252, 0.115], [0.25, 0.101], [0.2, 0.097], [0.045, 0.088], [0.028, 0.085], [0.028, 0.015], [0.016, 0.015]],
  soilU: 0.232, soilR: 0.0992, rimInnerR: 0.1, rimTopU: 0.252, potOuterR: 0.118, saucerR: 0.135, saucerTopU: 0.032,
  materials: { pot: "glazed-stoneware", saucer: "glazed-stoneware", soil: "potting-compost" }, tones: { pot: 0.22, saucer: 0.3, soil: 0.12 },
};

/** 🌿️ One leaf: rosette, base on the soil (x, z), azimuth of its face normal and lean (degrees), lean from the vertical (degrees), length above the soil, largest width, slight lateral sinuosity of the blade (m). */
type Leaf = { k: number; rosette: number; x: number; z: number; az: number; lean: number; L: number; W: number; wave: number };
const LEAVES: Leaf[] = [
  { k: 0, rosette: 0, x: 0.034, z: -0.03, az: 318, lean: 11, L: 0.66, W: 0.072, wave: 0.004 },
  { k: 1, rosette: 0, x: 0.046, z: -0.02, az: 22, lean: 16, L: 0.52, W: 0.064, wave: -0.003 },
  { k: 2, rosette: 0, x: 0.022, z: -0.048, az: 245, lean: 9, L: 0.6, W: 0.066, wave: 0.003 },
  { k: 3, rosette: 0, x: 0.03, z: -0.034, az: 300, lean: 4, L: 0.43, W: 0.05, wave: -0.002 },
  { k: 4, rosette: 1, x: -0.04, z: 0.008, az: 150, lean: 13, L: 0.62, W: 0.07, wave: 0.004 },
  { k: 5, rosette: 1, x: -0.05, z: 0.026, az: 118, lean: 18, L: 0.48, W: 0.06, wave: -0.004 },
  { k: 6, rosette: 1, x: -0.026, z: -0.004, az: 205, lean: 8, L: 0.56, W: 0.064, wave: 0.003 },
  { k: 7, rosette: 2, x: 0.012, z: 0.048, az: 62, lean: 12, L: 0.6, W: 0.068, wave: -0.003 },
  { k: 8, rosette: 2, x: 0.03, z: 0.06, az: 35, lean: 17, L: 0.46, W: 0.058, wave: 0.003 },
  { k: 9, rosette: 2, x: -0.004, z: 0.062, az: 128, lean: 7, L: 0.5, W: 0.06, wave: -0.002 },
];
const ROSETTES: { x: number; z: number }[] = [{ x: 0.032, z: -0.033 }, { x: -0.039, z: 0.01 }, { x: 0.013, z: 0.056 }];
const leafId = (k: number) => `leaf-${String(k + 1).padStart(2, "0")}`;
const rosetteId = (r: number) => `rosette-${r + 1}`;

/** 📏️ Half width of a blade along its share s of the length (linear-lanceolate: a channelled base, nearly parallel margins, an acute tip). */
const HALF: [number, number][] = [[0, 0.3], [0.06, 0.42], [0.18, 0.74], [0.32, 0.93], [0.48, 1], [0.62, 0.97], [0.75, 0.86], [0.85, 0.66], [0.93, 0.4], [0.98, 0.14], [1, 0]];

const leafFrame = (l: Leaf): Frame => {
  const out = azimuth(l.az);
  const u = unit(lin(out, Math.sin(l.lean * Math.PI / 180), UP, Math.cos(l.lean * Math.PI / 180)));
  return frameOf([l.x, POT.soilU, l.z], u, out);
};

/** 🍃️ The [u, w] outline of a blade at a scale: stations along the length, the margin with a slight sinuous midline, the hardened tip spine at 1:50. */
function blade(l: Leaf, scale: Scale): P[] {
  const ss = scale === 50 ? [0, 0.03, 0.06, 0.12, 0.18, 0.25, 0.32, 0.4, 0.48, 0.55, 0.62, 0.69, 0.75, 0.8, 0.85, 0.89, 0.93, 0.96, 0.98] : [0, 0.06, 0.18, 0.32, 0.48, 0.62, 0.75, 0.85, 0.93];
  const u0 = -0.012;
  const mid = (s: number) => l.wave * Math.sin(Math.PI * 1.6 * s);
  const right = ss.map((s) => [u0 + (l.L - u0) * s, mid(s) - (l.W / 2) * table(HALF, s)] as P);
  const left = ss.map((s) => [u0 + (l.L - u0) * s, mid(s) + (l.W / 2) * table(HALF, s)] as P).reverse();
  const tip: P[] = scale === 50 ? [[l.L - 0.004, mid(1) - 0.002], [l.L + 0.006, mid(1)], [l.L - 0.004, mid(1) + 0.002]] : [[l.L, mid(1)]];
  return [...right, ...tip, ...left];
}

const THICK: Record<Scale, number> = { 50: 0.008, 100: 0.016, 200: 0.016, 500: 0.016 };

/** 🌊️ The dark transverse bands of trifasciata on the face of a blade at 1:50: chevrons every 55 mm from 0.1 to 0.86 of the length, each crossing both margins. */
function bands(l: Leaf): P[][] {
  const out: P[][] = [];
  for (let u = 0.08; u < 0.86 * l.L; u += 0.055) {
    const s = u / l.L;
    const hw = (l.W / 2) * table(HALF, s) + 0.0015;
    const m = l.wave * Math.sin(Math.PI * 1.6 * s);
    out.push([[u - 0.006, m - hw], [u + 0.008, m], [u - 0.006, m + hw]]);
  }
  return out;
}

/** 🪭️ The merged fan of a rosette for 1:200 and 1:500: the convex hulls of every pair of neighbouring leaves (sorted by azimuth), each leaf taken as its coarse outline thickened to the smallest feature, so the fan follows the leaves without filling the open side of the rosette. World point sets, one per hull. */
function fanHulls(r: number, scale: Scale): V3[][] {
  const leaves = LEAVES.filter((l) => l.rosette === r).sort((a, b) => a.az - b.az);
  const thick = Math.max(THICK[100], FEATURE[scale] + 0.002);
  const pts = (l: Leaf) => {
    const lf = leafFrame(l);
    const full = blade(l, 100);
    const tipIndex = full.reduce((best, p, i) => (p[0] > full[best]![0] ? i : best), 0);
    const ring = full.filter((_, i) => scale === 200 || i % 2 === 0 || i === tipIndex);
    return ring.flatMap(([u, w]) => [THICK[100] / 2 - thick, THICK[100] / 2].map((t) => toWorld(lf, [Math.max(u, 0), t, w])));
  };
  if (leaves.length === 1) return [pts(leaves[0]!)];
  return leaves.slice(1).map((l, i) => [...pts(leaves[i]!), ...pts(l)]);
}

/** 🧾️ Builds the asset document. */
export function build(): { doc: Json; distinction: Text; notes: string[] } {
  const notes: string[] = [];
  const rig = new RigBuilder();
  const potFrame = frameOf([0, 0, 0], UP, [-1, 0, 0]);
  const potMassKg = 3.4 + 0.6 + 4.3;
  const rosetteKg = 0.12;
  const leafKg = (l: Leaf) => polyArea(blade(l, 50)) * 0.006 * 1000;
  rig.add({ id: "pot", parent: null, title: T("Glazed pot with saucer and compost", "Glasierter Topf mit Untersetzer und Erde"), f: potFrame, length: POT.rimTopU, joint: FIXED, mass: massOf(potMassKg, 0.11, 0, "point", "authored-estimate", "en-13040") });
  const rosetteFrames = ROSETTES.map((r, i) => {
    const leaves = LEAVES.filter((l) => l.rosette === i);
    const axis = unit(leaves.reduce((s, l) => add(s, leafFrame(l).u), [0, 0, 0] as V3));
    const lateral = unit(sub(axis, mul(UP, dot(axis, UP))));
    const vHint = Math.hypot(lateral[0], lateral[2]) < 0.05 ? [1, 0, 0] as V3 : lateral;
    const f = frameOf([r.x, POT.soilU - 0.03, r.z], axis, vHint);
    rig.add({ id: rosetteId(i), parent: "pot", title: T(`Rhizome rosette ${i + 1} (shoot base)`, `Rhizomrosette ${i + 1} (Triebbasis)`), f, length: 0.03, joint: FIXED, mass: massOf(rosetteKg, 0.015, 0, "point", "authored-estimate", "brown-1915") });
    return f;
  });
  for (const l of LEAVES) {
    const kg = leafKg(l);
    rig.add({ id: leafId(l.k), parent: rosetteId(l.rosette), title: T(`Leaf ${l.k + 1} (${(l.L * 100).toFixed(0)} cm sword leaf)`, `Blatt ${l.k + 1} (${(l.L * 100).toFixed(0)}-cm-Schwertblatt)`), f: leafFrame(l), length: l.L, joint: revolute(-6, 9), mass: massOf(kg, 0.42 * l.L, 0, "uniform", "authored-estimate", "brown-1915") });
  }
  notes.push(`[DEBUG] rig frames vs product restFrames: max error ${rig.verify().toExponential(2)}`);
  const totalKg = potMassKg + 3 * rosetteKg + LEAVES.reduce((s, l) => s + leafKg(l), 0);
  const leafArea = LEAVES.reduce((s, l) => s + polyArea(blade(l, 50)), 0);
  notes.push(`[DEBUG] total mass ${totalKg.toFixed(3)} kg, one-sided leaf area ${leafArea.toFixed(3)} m2`);

  const leafMeta = (l: Leaf) => ({ tone: r3(0.42 + 0.12 * phase(l.k, 3)), region: "leaf-blades", material: "sansevieria-leaf" });
  const representations: Json = {};
  const volumes: Json = {};
  for (const scale of SCALES) {
    const t = THICK[scale];
    const pot = potParts(POT, scale, scale === 50 ? { saucer: POT.saucer, pot: POT.pot } : scale === 100 ? { saucer: keep(POT.saucer, [0, 2, 3, 4, 5]), pot: keep(POT.pot, [0, 2, 3, 4, 5, 7, 8]) } : scale === 200 ? { saucer: keep(POT.saucer, [0, 3, 5]), pot: keep(POT.pot, [0, 3, 5, 8]) } : { saucer: [[0, 0.128], [0.032, 0.135]], pot: [[0.016, 0.103], [0.252, 0.118], [0.23, 0.099]] });
    const placed: Placed[] = [...pot.placed];
    const planPlaced: Placed[] = [...pot.plan];
    const body: Json[] = [...pot.body];
    const thermal: Json[] = [...pot.thermal];
    const structure: Json[] = [...potStructure(POT, scale)];
    if (scale <= 100) {
      for (const l of LEAVES) {
        const lf = leafFrame(l);
        const ring = blade(l, scale);
        const centre = toWorld(lf, [0.5 * l.L, 0, 0]);
        const shape = plateShape(leafId(l.k), leafId(l.k), ring, [], -t / 2, t / 2, leafMeta(l));
        placed.push({ shape, at: centre });
        planPlaced.push({ shape, at: centre });
        if (scale === 50) bands(l).forEach((b, i) => {
          const s = plateStroke(`${leafId(l.k)}-band-${i + 1}`, leafId(l.k), b, t / 2 + 0.0004, { role: "texture", tone: 0.2 });
          const at = add(centre, mul(lf.v, 0.001));
          placed.push({ shape: s, at, rank: 10 });
          planPlaced.push({ shape: s, at, rank: 10 });
        });
        body.push(plateSolid(leafId(l.k), leafId(l.k), ring, [], -t / 2, t / 2, { material: "sansevieria-leaf", tone: leafMeta(l).tone }));
        thermal.push({ id: `th-${leafId(l.k)}`, of: leafId(l.k), region: "leaf-blades" });
      }
    } else {
      ROSETTES.forEach((_, r) => {
        const hulls = fanHulls(r, scale);
        const meta = { tone: 0.46 + 0.04 * r, region: "leaf-blades", material: "sansevieria-leaf" };
        const id = `fan-${r + 1}`;
        hulls.forEach((h, i) => {
          const hull = hull3(h.map((p) => toLocal(potFrame, p)));
          body.push({ id: `${id}-${i + 1}`, kind: "polyhedron", bone: "pot", material: "sansevieria-leaf", role: "body", tone: r3(meta.tone), vertices: hull.vertices.map((v) => v.map(r5)), triangles: hull.triangles });
          thermal.push({ id: `th-${id}-${i + 1}`, of: `${id}-${i + 1}`, region: "leaf-blades" });
        });
        const centre = hulls.flat().reduce((s, p) => add(s, mul(p, 1 / hulls.flat().length)), [0, 0, 0] as V3);
        const perView = {} as Record<View, Json>;
        for (const view of VIEWS) {
          const outline = union2(hulls.map((h) => hull2(h.map((p) => project(view, p)))))[0]!;
          const rings = [outline.outer, ...outline.holes].map((ring) => toShapeCoords(view, potFrame, ring));
          perView[view] = plainShape(id, "pot", rings, meta);
        }
        placed.push({ shape: perView.elevation, at: centre, view: "elevation" });
        placed.push({ shape: perView.front, at: centre, view: "front" });
        planPlaced.push({ shape: perView.plan, at: centre });
      });
    }
    for (const l of LEAVES) {
      const reach = { 50: 0.9, 100: 0.88, 200: 0.86, 500: 0.84 }[scale];
      const halfW = 0.006, tv = 0.003;
      const root = -(POT.soilU - 0.024) / Math.cos(l.lean * Math.PI / 180);
      const strip: P[] = scale === 50 ? [[root, -halfW], [0, -halfW], [reach * l.L, -halfW * 0.6], [reach * l.L, halfW * 0.6], [0, halfW], [root, halfW]] : [[root, -halfW], [reach * l.L, -halfW * 0.6], [reach * l.L, halfW * 0.6], [root, halfW]];
      structure.push(plateSolid(`m-${leafId(l.k)}`, leafId(l.k), strip, [], -tv, tv, { material: "sansevieria-leaf", tone: 0.5, slender: true }));
      structure[structure.length - 1]!.member = leafId(l.k);
      structure[structure.length - 1]!.part = "member";
    }
    for (const [i, a] of ANCHORS.entries()) {
      const l = LEAVES[a.leaf]!;
      const f = FEATURE[scale];
      const r = Math.max(0.004, f / 2 + 0.001);
      const v = scale <= 100 ? t / 2 + r * 0.5 : 0;
      thermal.push({ id: `th-${a.id}`, kind: "ellipsoid", bone: leafId(l.k), material: "sansevieria-leaf", role: "body", tone: 0.8, radii: [r5(Math.max(0.03, 2 * r)), r5(r), r5(Math.max(0.012, r))], transform: { translation: [r5(a.s * l.L), r5(v), 0] }, region: "leaf-blades", tags: ["emitter", "transpiration-leaves"] });
      void i;
    }
    const geometry = ordered(placed, { plan: planPlaced });
    representations[String(scale)] = scale === 500 ? UNAVAILABLE_500 : { status: "authored", denominator: scale, validRange: { minDenominator: BAND[scale][0], maxDenominator: BAND[scale][1] }, pens: PENS[scale], hatches: [], geometry, decisions: REP_DECISIONS[scale] };
    volumes[String(scale)] = { status: "authored", denominator: scale, body, thermal, structure, decisions: VOL_DECISIONS[scale] };
  }

  const anchors: Json[] = [
    { id: "base-support", bone: "pot", x: 0, y: 0, kind: "support", title: T("Saucer on the floor", "Untersetzer auf dem Boden"), support: "fixed", normalDeg: 90, footprintLengthM: 0.24, footprintWidthM: 0.24, frictionCoefficient: 0.5 },
    ...ANCHORS.map((a) => ({ id: a.id, bone: leafId(a.leaf), x: r4(a.s * LEAVES[a.leaf]!.L), y: r4(THICK[50] / 2 + 0.001), z: 0, kind: "emitter", title: T(`Stomata outlet on leaf ${a.leaf + 1}`, `Spaltöffnungsauslass an Blatt ${a.leaf + 1}`) })),
  ];

  const draught = 0.8;
  const leafLoad = (l: Leaf, v: number, cf: number) => {
    const f = leafFrame(l);
    const n = f.v;
    const projected = polyArea(blade(l, 50)) * Math.max(0.25, Math.abs(n[0]));
    return { id: `wind-${leafId(l.k)}`, kind: "wind", title: T(`Draught on leaf ${l.k + 1}`, `Luftzug an Blatt ${l.k + 1}`), bone: leafId(l.k), x: r4(0.55 * l.L), y: 0, fxN: r5(drag(cf, v, projected)), fyN: 0, source: "literature", reference: "en-1991-1-4" };
  };
  const draughtLoads = LEAVES.map((l) => leafLoad(l, draught, 1.2));
  const sway = (k: number) => 0.25 + 0.5 * (LEAVES[k]!.L / 0.66) ** 2;
  const poseBones = (f: (l: Leaf) => number) => Object.fromEntries(LEAVES.map((l) => [leafId(l.k), { angleDeg: r4(f(l)) }]));
  const regionsOf = (leafSun: number, potSun: number, soilWet: number, soilSun: number) => ({ "leaf-blades": { exposedFraction: 1, sunlitFraction: leafSun }, "pot-wall": { exposedFraction: 1, sunlitFraction: potSun }, saucer: { exposedFraction: 1, sunlitFraction: potSun * 0.5 }, "soil-surface": { exposedFraction: 1, sunlitFraction: soilSun, ...(soilWet > 0 ? { wettedness: soilWet } : {}) } });
  const poses: Json[] = [
    { id: "still-air", title: T("Still air, indoors by day", "Ruhende Luft, drinnen am Tag"), description: T("The plant stands near a window in a heated living room with still air (0.05 m/s); the CAM stomata are nearly closed by day, the compost is evenly moist, the leaves stand in their rest lean.", "Die Pflanze steht nahe einem Fenster in einem beheizten Wohnraum bei ruhender Luft (0,05 m/s); die CAM-Spaltöffnungen sind tagsüber fast geschlossen, die Erde ist gleichmäßig feucht, die Blätter stehen in ihrer Ruheneigung."), bones: {}, ground: "authored", contacts: [], thermal: { regions: regionsOf(0.3, 0.2, 0, 0.1), emitters: { "transpiration-leaves": 0.12 } }, loads: [] },
    { id: "ventilation-draught", title: T("Draught from a tilted window", "Luftzug aus einem gekippten Fenster"), description: T(`Cold air of ${draught} m/s flows over the plant from ahead (+x) while the room is aired; the stiff leaves bend by less than a degree, the drag on each blade is cf 1.2 times the dynamic pressure on its projected face.`, `Kalte Luft mit ${String(draught).replace(".", ",")} m/s strömt beim Lüften von vorn (+x) über die Pflanze; die steifen Blätter biegen sich um weniger als ein Grad, der Widerstand jeder Spreite ist cf 1,2 mal der Staudruck auf ihre projizierte Fläche.`), bones: poseBones((l) => 0.6 * sway(l.k) * Math.cos(l.az * Math.PI / 180)), ground: "authored", contacts: [], thermal: { regions: regionsOf(0.3, 0.2, 0, 0.1), emitters: { "transpiration-leaves": 0.12 } }, loads: draughtLoads },
    { id: "freshly-watered", title: T("Freshly watered", "Frisch gegossen"), description: T("0.6 litres of water have just soaked the compost (wet surface, 5.9 N more in the pot); the turgid leaves stand a little more upright.", "0,6 Liter Wasser haben die Erde eben durchfeuchtet (nasse Oberfläche, 5,9 N mehr im Topf); die prallen Blätter stehen etwas aufrechter."), bones: poseBones((l) => -0.4 - 0.4 * (l.lean / 18)), ground: "authored", contacts: [], thermal: { regions: regionsOf(0.3, 0.2, 1, 0.1), emitters: { "transpiration-leaves": 0.15 } }, loads: [{ id: "irrigation-water", kind: "imposed", title: T("Water held in the compost", "In der Erde gehaltenes Wasser"), bone: "pot", x: 0.12, y: 0, fxN: 0, fyN: -5.884, source: "authored-estimate", reference: "en-13040" }] },
    { id: "drought-stressed", title: T("After weeks without water", "Nach Wochen ohne Wasser"), description: T("The compost has dried out; the leaves have lost turgor, splay outwards by 3 to 7 degrees and keep their stomata closed day and night (CAM idling).", "Die Erde ist ausgetrocknet; die Blätter haben an Spannung verloren, spreizen sich um 3 bis 7 Grad nach außen und halten ihre Spaltöffnungen Tag und Nacht geschlossen (CAM-Leerlauf)."), bones: poseBones((l) => 3 + 4 * (l.lean / 18)), ground: "authored", contacts: [], thermal: { regions: regionsOf(0.3, 0.2, 0, 0.1), emitters: { "transpiration-leaves": 0.02 } }, loads: [] },
    { id: "night-stomata-open", title: T("Night, stomata open (CAM)", "Nacht, Spaltöffnungen offen (CAM)"), description: T("At night the crassulacean acid metabolism opens the stomata and takes up carbon dioxide: transpiration is at its daily maximum while no light falls on the plant.", "Nachts öffnet der Crassulaceen-Säurestoffwechsel die Spaltöffnungen und nimmt Kohlendioxid auf: Die Transpiration erreicht ihr Tagesmaximum, während kein Licht auf die Pflanze fällt."), bones: {}, ground: "authored", contacts: [], thermal: { regions: regionsOf(0, 0, 0, 0), emitters: { "transpiration-leaves": 1 } }, loads: [] },
  ];

  const swayT = 1.6;
  const swayKeys = Array.from({ length: 9 }, (_, i) => {
    const t = (swayT * i) / 8;
    return { t: r4(t), pose: "ventilation-draught", ease: "smooth", bones: Object.fromEntries(LEAVES.map((l) => [leafId(l.k), { angleDeg: r4(0.6 * sway(l.k) * Math.cos(l.az * Math.PI / 180) + 0.35 * sway(l.k) * Math.sin(2 * Math.PI * (t / swayT) + 2 * Math.PI * phase(l.k, 7))) }])) };
  });
  const gustT = [0, 0.8, 1.6, 2.2, 3, 4, 6];
  const gustG = [1, 1.6, 3, 3.4, 2.4, 1.4, 1];
  const gustKeys = gustT.map((t, i) => ({ t, pose: "ventilation-draught", ease: "smooth", bones: Object.fromEntries(LEAVES.map((l) => [leafId(l.k), { angleDeg: r4(0.6 * sway(l.k) * Math.cos(l.az * Math.PI / 180) * gustG[i]! ** 2 * 0.5 + 0.3 * sway(l.k) * Math.cos(l.az * Math.PI / 180)) }])) }));
  const clips: Json[] = [
    { id: "wind-sway", title: T("Draught flutter", "Flattern im Luftzug"), description: T(`A periodic ${swayT} s cycle in the airing draught: the long outer leaves nod by a few tenths of a degree about their mean bend, each with its own phase; the short young leaves hardly move.`, `Ein periodischer Zyklus von ${String(swayT).replace(".", ",")} s im Lüftungszug: Die langen äußeren Blätter nicken um wenige Zehntelgrad um ihre mittlere Biegung, jedes mit eigener Phase; die kurzen jungen Blätter bewegen sich kaum.`), durationS: swayT, fps: 30, loop: true, rootMotion: "none", keys: swayKeys, contacts: [], phases: [{ id: "flutter", fromS: 0, toS: swayT, title: T("Flutter cycle", "Flatterzyklus") }], events: [{ t: 0, kind: "motion", title: T("Cycle start at the mean bend", "Zyklusbeginn in der mittleren Biegung") }], tracks: [] },
    { id: "gust", title: T("Door gust", "Türböe"), description: T("A door opens against the tilted window: the draught rises from 0.8 to about 2.7 m/s within two seconds and settles again; drag follows the square of the speed.", "Eine Tür öffnet sich gegenüber dem gekippten Fenster: Der Luftzug steigt binnen zwei Sekunden von 0,8 auf etwa 2,7 m/s und beruhigt sich wieder; der Widerstand folgt dem Quadrat der Geschwindigkeit."), durationS: 6, fps: 30, loop: false, rootMotion: "none", keys: gustKeys, contacts: [], phases: [{ id: "rise", fromS: 0, toS: 2.2, title: T("Gust front", "Böenfront") }, { id: "peak", fromS: 2.2, toS: 3, title: T("Peak", "Spitze") }, { id: "decay", fromS: 3, toS: 6, title: T("Settling", "Abklingen") }], events: [{ t: 2.2, kind: "load-on", title: T("Peak draught load", "Spitzenlast des Luftzugs") }, { t: 4, kind: "load-off", title: T("Load back near the mean", "Last wieder nahe dem Mittel") }], tracks: draughtLoads.map((d) => ({ target: `load:${d.id}`, ease: "smooth", keys: gustT.map((t, i) => ({ t, v: r3(gustG[i]! ** 2) })) })) },
    { id: "diurnal-transpiration", title: T("CAM day and night", "CAM-Tag und -Nacht"), description: T("24 hours in 24 seconds (1 s = 1 h): the stomata open after dusk and stay open through the night (phase I), close in the morning (phase II-III) and open a little in the late afternoon (phase IV); daylight from the window reaches the leaves from 8 to 17 h.", "24 Stunden in 24 Sekunden (1 s = 1 h): Die Spaltöffnungen öffnen nach der Dämmerung und bleiben die Nacht über offen (Phase I), schließen am Morgen (Phase II-III) und öffnen sich am späten Nachmittag ein wenig (Phase IV); Tageslicht vom Fenster erreicht die Blätter von 8 bis 17 Uhr."), durationS: 24, fps: 10, loop: true, rootMotion: "none", keys: [{ t: 0, pose: "still-air", ease: "smooth" }, { t: 12, pose: "still-air", ease: "smooth" }, { t: 24, pose: "still-air", ease: "smooth" }], contacts: [], phases: [{ id: "night", fromS: 0, toS: 6, title: T("Night, stomata open", "Nacht, Spaltöffnungen offen") }, { id: "morning", fromS: 6, toS: 12, title: T("Morning, stomata closing", "Morgen, Spaltöffnungen schließen") }, { id: "afternoon", fromS: 12, toS: 18, title: T("Afternoon, phase IV", "Nachmittag, Phase IV") }, { id: "evening", fromS: 18, toS: 24, title: T("Evening, stomata open", "Abend, Spaltöffnungen offen") }], events: [{ t: 8, kind: "thermal", title: T("Daylight reaches the leaves", "Tageslicht erreicht die Blätter") }, { t: 17, kind: "thermal", title: T("Daylight fades", "Tageslicht schwindet") }], tracks: [
      { target: "emitter:transpiration-leaves", ease: "smooth", keys: [{ t: 0, v: 0.95 }, { t: 4, v: 1 }, { t: 7, v: 0.55 }, { t: 9, v: 0.12 }, { t: 12, v: 0.08 }, { t: 15, v: 0.14 }, { t: 17, v: 0.3 }, { t: 19, v: 0.7 }, { t: 21, v: 0.92 }, { t: 24, v: 0.95 }] },
      { target: "region:leaf-blades:sunlitFraction", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 7.5, v: 0 }, { t: 9, v: 0.18 }, { t: 12, v: 0.3 }, { t: 15, v: 0.22 }, { t: 17, v: 0.02 }, { t: 18, v: 0 }, { t: 24, v: 0 }] },
      { target: "region:pot-wall:sunlitFraction", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 7.5, v: 0 }, { t: 9, v: 0.1 }, { t: 12, v: 0.2 }, { t: 15, v: 0.14 }, { t: 17, v: 0.01 }, { t: 18, v: 0 }, { t: 24, v: 0 }] },
    ] },
  ];

  const leafAreaRounded = r4(leafArea);
  const thermal = {
    environment: { airTempC: 21, meanRadiantTempC: 20.5, relativeHumidity: 0.4, windMS: 0.05, windDirectionDeg: 0, solarWM2: 120, diffuseFraction: 0.75, pressurePa: 101325 },
    referenceEnvironment: "office-still",
    passengers: [], operating: [],
    regions: [
      { id: "leaf-blades", title: T("Sword leaves (succulent, CAM)", "Schwertblätter (sukkulent, CAM)"), kind: "foliage", model: "leaf", material: "sansevieria-leaf", areaM2: leafAreaRounded, areaBasis: "exchange", characteristicLengthM: 0.065, geometry: "vertical-plate", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 22800, leaf: { stomatalConductanceMaxMS: 0.0012, leafAreaIndex: r3(leafArea / (Math.PI * 0.22 * 0.22)), photosynthesis: "cam" }, source: "authored-estimate", reference: "winter-smith-1996" },
      { id: "pot-wall", title: T("Glazed pot wall and rim", "Glasierte Topfwand mit Rand"), kind: "surface", model: "passive", material: "glazed-stoneware", areaM2: 0.25, characteristicLengthM: 0.24, geometry: "vertical-plate", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 15500, source: "authored-estimate", reference: "iso-10456" },
      { id: "saucer", title: T("Saucer", "Untersetzer"), kind: "surface", model: "passive", material: "glazed-stoneware", areaM2: 0.13, characteristicLengthM: 0.25, geometry: "horizontal-plate-up", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 11600, source: "authored-estimate", reference: "iso-10456" },
      { id: "soil-surface", title: T("Compost surface", "Erdoberfläche"), kind: "surface", model: "passive", material: "potting-compost", areaM2: r4(Math.PI * POT.soilR ** 2), characteristicLengthM: 0.21, geometry: "horizontal-plate-up", skyViewFactor: 0.55, layers: [], heatCapacityJM2K: 39000, source: "authored-estimate", reference: "en-13040" },
    ],
    emitters: [{ id: "transpiration-leaves", kind: "transpiration", title: T("Transpiration of the sword leaves (six outlets)", "Transpiration der Schwertblätter (sechs Auslässe)"), anchor: ANCHORS[0]!.id, anchors: ANCHORS.map((a) => a.id), directionDeg: 90, diameterM: 0.06, region: "leaf-blades", source: "authored-estimate", reference: "winter-smith-1996" }],
    assumptions: [
      T(`The leaf region is the one-sided area of the ten real blades (${leafAreaRounded} m2 over a crown projection of 0.15 m2). Crassulacean acid metabolism: the maximum stomatal conductance of 0.0012 m/s is an authored estimate for a CAM succulent at night (Winter and Smith 1996; Nobel 1988 for the order of CAM conductances); by day the poses and the diurnal clip throttle the transpiration emitter to 8 to 15 percent, at night it runs at full strength. The 6 mm succulent blades store about 23 kJ/(m2 K).`, `Die Blattregion ist die einseitige Fläche der zehn wirklichen Spreiten (${String(leafAreaRounded).replace(".", ",")} m2 über einer Kronenprojektion von 0,15 m2). Crassulaceen-Säurestoffwechsel: Die maximale Spaltöffnungsleitfähigkeit von 0,0012 m/s ist eine Autorenschätzung für eine CAM-Sukkulente bei Nacht (Winter und Smith 1996; Nobel 1988 für die Größenordnung von CAM-Leitfähigkeiten); tagsüber drosseln die Posen und der Tagesclip den Transpirationsemitter auf 8 bis 15 Prozent, nachts läuft er mit voller Stärke. Die 6 mm dicken sukkulenten Spreiten speichern etwa 23 kJ/(m2 K).`),
      T("Pot, saucer and compost are passive: glazed stoneware after ISO 10456 (2300 kg/m3, 1.3 W/(m K)), 8 mm walls; the compost surface is wetted in the watered pose. The reference case is a still indoor room (office-still preset), the asset's own case adds diffuse window daylight of 120 W/m2.", "Topf, Untersetzer und Erde sind passiv: glasiertes Steinzeug nach ISO 10456 (2300 kg/m3, 1,3 W/(m K)), 8 mm Wände; die Erdoberfläche ist in der gegossenen Pose benetzt. Der Referenzfall ist ein ruhiger Innenraum (Voreinstellung office-still), der eigene Fall ergänzt diffuses Fensterlicht von 120 W/m2."),
    ],
  };

  const asset: Json = {
    id: ID, category: "vegetation",
    title: T("Snake Plant in a Pot", "Bogenhanf im Topf"),
    summary: T("A snake plant (Dracaena trifasciata, formerly Sansevieria trifasciata) 0.9 m high in a 0.25 m glazed stoneware pot on a saucer: ten stiff, upright sword leaves of 0.43 to 0.66 m in three rhizome rosettes, 5 to 7 cm wide and 6 mm thick, dark green with pale transverse bands and a hard tip. A succulent CAM plant that opens its stomata at night; a classic of offices and living rooms.", "Ein Bogenhanf (Dracaena trifasciata, früher Sansevieria trifasciata) von 0,9 m Höhe in einem glasierten Steinzeugtopf von 0,25 m auf einem Untersetzer: zehn steife, aufrechte Schwertblätter von 0,43 bis 0,66 m in drei Rhizomrosetten, 5 bis 7 cm breit und 6 mm dick, dunkelgrün mit hellen Querbändern und harter Spitze. Eine sukkulente CAM-Pflanze, die ihre Spaltöffnungen nachts öffnet; ein Klassiker in Büros und Wohnräumen."),
    tags: ["snake-plant", "succulent", "cam", "indoor", "potted", "houseplant", "sword-leaf"],
    provenance: {
      author: "Semio staffage plant worker PW-17", method: "handcrafted", created: "2026-11-22",
      references: [
        { id: "brown-1915", citation: "Brown, N. E. (1915). Sansevieria: a monograph of all the known species. Bulletin of Miscellaneous Information, Royal Botanic Gardens, Kew 1915(5): 185-261. S. trifasciata: leaves 1-6 in a rosette, erect, linear-lanceolate, 0.3-1.4 m long, 2.5-7 cm wide, flat or slightly channelled, transversely banded with pale green." },
        { id: "takawira-2018", citation: "Takawira-Nyenya, R., Mucina, L., Cardinal-McTeague, W. M. and Thiede, J. (2018). Sansevieria (Asparagaceae, Nolinoideae) is a herbaceous clade within Dracaena: inference from non-coding plastid and nuclear DNA sequence data. Phytotaxa 376(6): 254-276 (the name Dracaena trifasciata)." },
        { id: "winter-smith-1996", citation: "Winter, K. and Smith, J. A. C. (eds.) (1996). Crassulacean Acid Metabolism: Biochemistry, Ecophysiology and Evolution. Ecological Studies 114, Springer, Berlin. The four phases of CAM gas exchange: nocturnal stomatal opening (I), morning burst (II), daytime closure (III), late-afternoon opening (IV)." },
        { id: "nobel-1988", citation: "Nobel, P. S. (1988). Environmental Biology of Agaves and Cacti. Cambridge University Press. Orders of magnitude of nocturnal stomatal conductance and water use of CAM succulents." },
        { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed. Cambridge University Press. Leaf energy balance, boundary layer and stomatal conductance, leaf area index." },
        { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A." },
        { id: "iso-10456", citation: "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values. Ceramic (porcelain, stoneware): 2300 kg/m3, 1.3 W/(m K), 840 J/(kg K)." },
        { id: "en-13040", citation: "EN 13040:2007 Soil improvers and growing media - Sample preparation for chemical and physical tests, determination of dry matter content, moisture content and laboratory compacted bulk density (the bulk density of moist potting compost of about 0.5 to 0.7 kg/l used for the pot mass)." },
      ],
      notes: T("All ten leaves are stated by hand with their rosette, base, azimuth, lean, length, width and the slight sinuous twist of the blade; the blade outline, the trifasciata bands, the pot and saucer profiles are hand-chosen numbers that a generator script only formats (`T/🧰️scripts/PW-17/snake-plant.ts`). At 1:200 the leaves of each rosette are drawn and modelled as one turned fan; the 1:500 drawing is unavailable (leaf width 0.12 mm on paper) with a plan symbol, while a digital 1:500 model exists.", "Alle zehn Blätter sind von Hand mit Rosette, Ansatz, Azimut, Neigung, Länge, Breite und der leichten Wellung der Spreite angegeben; Blattumriss, trifasciata-Bänder, Topf- und Untersetzerprofile sind von Hand gewählte Zahlen, die ein Generatorskript nur formatiert (`T/🧰️scripts/PW-17/snake-plant.ts`). Bei 1:200 werden die Blätter jeder Rosette als ein gedrehter Fächer gezeichnet und modelliert; die Zeichnung 1:500 ist nicht verfügbar (Blattbreite 0,12 mm auf dem Papier), mit einem Grundrisssymbol, während ein digitales Modell 1:500 besteht."),
    },
    dimensions: { heightM: 0.9, widthM: 0.4, depthM: 0.4, massKg: r3(totalKg) },
    materials: [
      { id: "sansevieria-leaf", title: T("Succulent sword leaf (dark green, waxy)", "Sukkulentes Schwertblatt (dunkelgrün, wachsig)"), emissivity: 0.96, solarAbsorptivity: 0.72, densityKgM3: 1000, specificHeatJKgK: 3800, elasticModulusPa: 30e6, source: "authored-estimate", reference: "brown-1915" },
      ...potMaterials(T("Glazed stoneware (dark grey glaze)", "Glasiertes Steinzeug (dunkelgraue Glasur)"), 0.75),
    ],
    rig: rig.rig(), volumes, anchors, representations,
    symbols: [SYMBOL()], poses, clips, thermal,
    structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["base-support"], assumptions: [T("One rigid body per evaluated pose, fixed at the saucer on the floor; each sword leaf is a stiff cantilever from its rosette, loaded by its weight and by the draught drag of its pose. The 0.8 m/s airing draught gives a few millinewtons per leaf (EN 1991-1-4 pressure with cf 1.2), the door gust about ten times that; no failure is predicted.", "Ein starrer Körper je ausgewerteter Pose, eingespannt am Untersetzer auf dem Boden; jedes Schwertblatt ist ein steifer Kragarm aus seiner Rosette, belastet durch sein Gewicht und den Widerstand des Luftzugs seiner Pose. Der Lüftungszug von 0,8 m/s ergibt wenige Millinewton je Blatt (Druck nach EN 1991-1-4 mit cf 1,2), die Türböe etwa das Zehnfache; es wird kein Versagen vorhergesagt.")], clinical: false },
    presentation: { defaultPose: "still-air", defaultClip: "wind-sway", defaultScale: 50 },
    exports: EXPORTS, completion: "authored",
  };
  const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
  const distinction = T("A potted snake plant (Dracaena trifasciata) 0.9 m high: ten stiff, banded sword leaves in three rosettes in a 0.25 m glazed stoneware pot on a saucer; a CAM succulent that transpires at night.", "Ein Bogenhanf (Dracaena trifasciata) im Topf, 0,9 m hoch: zehn steife, gebänderte Schwertblätter in drei Rosetten in einem glasierten Steinzeugtopf von 0,25 m auf einem Untersetzer; eine CAM-Sukkulente, die nachts transpiriert.");
  return { doc, distinction, notes };
}

/** 💨️ The six stomata outlets: leaf and share of its length. */
const ANCHORS = [
  { id: "tp-1", leaf: 0, s: 0.55 }, { id: "tp-2", leaf: 2, s: 0.5 }, { id: "tp-3", leaf: 4, s: 0.55 },
  { id: "tp-4", leaf: 6, s: 0.5 }, { id: "tp-5", leaf: 7, s: 0.55 }, { id: "tp-6", leaf: 9, s: 0.45 },
];

const PENS: Record<Scale, Json> = {
  50: { body: 0.18, detail: 0.13, texture: 0.1, glazing: null, ground: 0.25 },
  100: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  200: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.2 },
  500: { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.18 },
};

const UNAVAILABLE_500 = {
  status: "unavailable", denominator: 500, cause: "below-printable-feature",
  reason: T("At 1:500 the 0.9 m plant is 1.8 mm high and a 6 cm sword leaf is 0.12 mm wide, below the 0.2 mm printable feature; even merged into rosette fans the leaves would only print as a blot on the pot.", "Bei 1:500 ist die 0,9 m hohe Pflanze 1,8 mm hoch und ein 6 cm breites Schwertblatt 0,12 mm breit, unter dem druckbaren Merkmal von 0,2 mm; selbst zu Rosettenfächern zusammengefasst würden die Blätter nur als Klecks auf dem Topf drucken."),
  limitingFeature: T("Width of a sword leaf (6 cm)", "Breite eines Schwertblatts (6 cm)"), limitingSizeM: 0.06,
  alternatives: [{ kind: "symbol", symbol: "snake-plant-plan-symbol" }, { kind: "scale", denominator: 200 }],
};

/** 🔣️ The plan sign with the true extent of pot and leaf crown in all three views: the convex outline of the 1:50 body seen in each view (world model metres), filled in by {@link finish}. */
function SYMBOL(): Json {
  const pens = { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 };
  return {
    id: "snake-plant-plan-symbol", title: T("Plan sign of a potted snake plant", "Grundrisszeichen eines Bogenhanfs im Topf"), description: T("A conventional sign for interior plans: the outline of the leaf crown and the pot with their true extent (about 0.35 m across); a separate symbol, never used silently for a pictorial scale.", "Ein konventionelles Zeichen für Innenraumpläne: der Umriss von Blattkrone und Topf in wahrer Ausdehnung (etwa 0,35 m breit); ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."),
    semantics: "true-extent", units: "model-m", denominators: [500, 200], pens, hatches: [],
    geometry: { elevation: [], front: [], plan: [] },
    dimensionalNote: T("True extent in model metres: the outline is the reach of the leaf tips and the pot, about 0.35 m across and 0.88 m high.", "Wahre Ausdehnung in Modellmetern: Der Umriss ist die Reichweite der Blattspitzen und des Topfs, etwa 0,35 m breit und 0,88 m hoch."),
  };
}

/** 🔣️ Fills the symbol outlines from the convex hull of the 1:50 body in each view. */
export function finish(asset: Json): void {
  const meshes = worldMeshes(asset, 50, asset.volumes["50"].body);
  const sym = asset.symbols[0];
  for (const view of VIEWS) {
    const pts: P[] = [];
    for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) pts.push(project(view, [m.world[i], m.world[i + 1], m.world[i + 2]]));
    const ring = hull2(pts);
    sym.geometry[view] = [{ id: "sign", bone: "pot", role: "body", z: 1, filled: false, tone: 0.5, regions: ["leaf-blades", "pot-wall", "saucer", "soil-surface"], kind: "poly", points: ring.map(([a, b]) => [r4(a), r4(b)]), closed: true }];
  }
}

const REP_DECISIONS: Record<Scale, Json> = {
  50: decisions({
    silhouette: T("At 1:50 (0.9 m is 18 mm) every view is the true orthographic view of the 1:50 model: ten sword leaves as plates on their own bones with the linear-lanceolate outline, a slightly sinuous midline and the hardened tip spine, the glazed pot with foot and rolled rim on its saucer.", "Bei 1:50 (0,9 m sind 18 mm) ist jede Ansicht die wahre Parallelprojektion des Modells 1:50: zehn Schwertblätter als Platten auf eigenen Knochen mit linealisch-lanzettlichem Umriss, leicht gewellter Mittellinie und verhärteter Endspitze, der glasierte Topf mit Fuß und gerolltem Rand auf seinem Untersetzer."),
    retained: T("Retained: every leaf, its width law and lean, the tip spine, the pot foot, rim and saucer, the compost surface inside the rim in plan.", "Beibehalten: jedes Blatt mit Breitenverlauf und Neigung, die Endspitze, Topffuß, Rand und Untersetzer, die Erdoberfläche innerhalb des Rands im Grundriss."),
    merged: T("Merged: the 6 mm succulent blade is drawn and modelled 8 mm thick (the smallest feature of 1:50 is 7.5 mm); the shallow channel of the leaf base is flattened into the plate.", "Zusammengefasst: Die 6 mm dicke sukkulente Spreite wird 8 mm dick gezeichnet und modelliert (das kleinste Merkmal von 1:50 sind 7,5 mm); die flache Rinne der Blattbasis ist in die Platte eingeebnet."),
    omitted: T("Omitted: roots and rhizomes (in the compost), the drainage hole.", "Weggelassen: Wurzeln und Rhizome (in der Erde), das Abzugsloch."),
    thinElements: T("Thin elements: the trifasciata bands are texture chevrons of 0.1 mm across each blade, crossing both margins.", "Dünne Elemente: Die trifasciata-Bänder sind Texturwinkel von 0,1 mm quer über jede Spreite, die beide Ränder kreuzen."),
    hatching: T("No hatching; the bands are the material indication of the leaves.", "Keine Schraffur; die Bänder sind die Materialangabe der Blätter."),
    styles: T("Dark and light line draw outlines at 0.18 mm with bands at 0.1 mm; the solid styles fill the leaves in mid greens, the pot dark and the compost darkest.", "Dunkle und helle Linie zeichnen Umrisse mit 0,18 mm und Bänder mit 0,1 mm; die Flächenstile füllen die Blätter in mittleren Grüntönen, den Topf dunkel und die Erde am dunkelsten."),
    overlays: T("Thermal: every leaf carries the leaf region, pot, saucer and compost their own passive regions; structural: each leaf is a member from its rosette, the pot carries the support on the floor.", "Thermik: Jedes Blatt trägt die Blattregion, Topf, Untersetzer und Erde ihre eigenen passiven Regionen; Tragwerk: Jedes Blatt ist ein Stab aus seiner Rosette, der Topf trägt das Auflager auf dem Boden."),
  }),
  100: decisions({
    silhouette: T("At 1:100 (0.9 m is 9 mm) the ten leaves are redrawn with nine stations each and a plain acute tip; pot and saucer keep foot and rim in fewer profile points.", "Bei 1:100 (0,9 m sind 9 mm) sind die zehn Blätter mit je neun Stationen und schlichter spitzer Spitze neu gezeichnet; Topf und Untersetzer behalten Fuß und Rand mit weniger Profilpunkten."),
    retained: T("Retained: every leaf with its lean and width, the pot rim and the saucer.", "Beibehalten: jedes Blatt mit Neigung und Breite, der Topfrand und der Untersetzer."),
    merged: T("Merged: the blade is drawn and modelled 16 mm thick so that it stays above the 15 mm smallest feature of this scale; the tip spine is merged into the tip.", "Zusammengefasst: Die Spreite wird 16 mm dick gezeichnet und modelliert, damit sie über dem kleinsten Merkmal von 15 mm dieses Maßstabs bleibt; die Endspitze ist in die Spitze eingegangen."),
    omitted: T("Omitted: the trifasciata bands (0.55 mm apart on paper) and the sinuous midline detail below 0.04 mm.", "Weggelassen: die trifasciata-Bänder (0,55 mm Abstand auf dem Papier) und die Wellung der Mittellinie unter 0,04 mm."),
    thinElements: T("Thin elements: none; an edge-on leaf is at least 16 mm wide.", "Dünne Elemente: keine; ein Blatt in Kantenansicht ist mindestens 16 mm breit."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("Lines at 0.18 mm; solid styles as at 1:50.", "Linien mit 0,18 mm; Flächenstile wie bei 1:50."),
    overlays: T("As at 1:50.", "Wie bei 1:50."),
  }),
  200: decisions({
    silhouette: T("At 1:200 (0.9 m is 4.5 mm) each of the three rosettes is one turned fan that follows the farthest leaf margins of its leaves, on a pot of four profile points.", "Bei 1:200 (0,9 m sind 4,5 mm) ist jede der drei Rosetten ein gedrehter Fächer, der den äußersten Blatträndern ihrer Blätter folgt, auf einem Topf aus vier Profilpunkten."),
    retained: T("Retained: the three rosettes with their height, lean and spread; pot and saucer.", "Beibehalten: die drei Rosetten mit Höhe, Neigung und Ausbreitung; Topf und Untersetzer."),
    merged: T("Merged: the leaves of each rosette into its fan (a single leaf would be 0.3 mm wide and closer than 0.15 mm to its neighbour at 1:350).", "Zusammengefasst: die Blätter jeder Rosette zu ihrem Fächer (ein einzelnes Blatt wäre 0,3 mm breit und bei 1:350 näher als 0,15 mm an seinem Nachbarn)."),
    omitted: T("Omitted: single leaf outlines and tips.", "Weggelassen: einzelne Blattumrisse und Spitzen."),
    thinElements: T("Thin elements: none.", "Dünne Elemente: keine."),
    hatching: T("No hatching.", "Keine Schraffur."),
    styles: T("Lines at 0.15 mm; solid styles fill the fans in two greens.", "Linien mit 0,15 mm; Flächenstile füllen die Fächer in zwei Grüntönen."),
    overlays: T("Thermal: the fans carry the leaf region; structural: the leaf members run inside the fans.", "Thermik: Die Fächer tragen die Blattregion; Tragwerk: Die Blattstäbe verlaufen in den Fächern."),
  }),
  500: {},
};

const VOL_DECISIONS: Record<Scale, Text[]> = {
  50: [T("Body at 1:50: ten 8 mm leaf plates with the drawn outline and tip spine, pot and saucer turned from their drawn profiles, a 3 mm compost sheet inside the rim.", "Körper bei 1:50: zehn 8 mm dicke Blattplatten mit gezeichnetem Umriss und Endspitze, Topf und Untersetzer aus ihren gezeichneten Profilen gedreht, eine 3 mm dicke Erdschicht innerhalb des Rands."), T("Thermal at 1:50: every body solid by reference in its region, six stomata outlets on the leaf faces. Structure: a midline strip member per leaf, the pot by reference, the support pad on the floor and the mass of pot, saucer and compost.", "Thermik bei 1:50: jeder Körper per Verweis in seiner Region, sechs Spaltöffnungsauslässe auf den Blattflächen. Tragwerk: ein Mittelstreifenstab je Blatt, der Topf per Verweis, das Auflager auf dem Boden und die Masse von Topf, Untersetzer und Erde.")],
  100: [T("Body at 1:100: the leaves on nine stations at 16 mm (above the 15 mm feature), pot and saucer on fewer profile points.", "Körper bei 1:100: die Blätter auf neun Stationen mit 16 mm (über dem Merkmal von 15 mm), Topf und Untersetzer auf weniger Profilpunkten."), T("Thermal and structure at 1:100 as at 1:50 on the coarser solids.", "Thermik und Tragwerk bei 1:100 wie bei 1:50 auf den gröberen Körpern.")],
  200: [T("Body at 1:200: three turned rosette fans (the farthest leaf margins of each rosette in v, a lateral scale in w), a four-point pot and a three-point saucer.", "Körper bei 1:200: drei gedrehte Rosettenfächer (die äußersten Blattränder jeder Rosette in v, ein Querfaktor in w), ein Topf aus vier und ein Untersetzer aus drei Punkten."), T("Thermal at 1:200: fans by reference in the leaf region; structure: the leaf members as slender strips inside the fans.", "Thermik bei 1:200: Fächer per Verweis in der Blattregion; Tragwerk: die Blattstäbe als schlanke Streifen in den Fächern.")],
  500: [T("Body at 1:500 (digital model only, the drawing is below the printable feature): three fans on six stations at least 75 mm thick, a two-point pot and saucer.", "Körper bei 1:500 (nur digitales Modell, die Zeichnung liegt unter dem druckbaren Merkmal): drei Fächer auf sechs Stationen, mindestens 75 mm dick, Topf und Untersetzer aus zwei Punkten."), T("Thermal and structure at 1:500 as at 1:200.", "Thermik und Tragwerk bei 1:500 wie bei 1:200.")],
};
