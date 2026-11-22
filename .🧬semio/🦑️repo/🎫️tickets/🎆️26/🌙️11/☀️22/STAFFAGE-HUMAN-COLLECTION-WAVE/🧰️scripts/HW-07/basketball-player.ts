/** 🏀️ Generator of `👤️humans/basketball-player`: a tall athlete derived from the catalog jogger, rescaled to 1.98 m and 95 kg, in a sleeveless
 * jersey, knee-length shorts and high-top shoes with a size 7 ball: dribbling (looping), jump-shot set and release (airborne), standing with the
 * ball, idle breathing. The rig stands on a fixed `court` root (hip lift and shift prismatic, as the seated reader stands on its bench) and the ball
 * hangs on its own prismatic chain from the court, so it moves in world coordinates; the rest pose keeps the pelvis at the origin like the source, so
 * the inherited front and plan outlines (rest-world coordinates) stay where they were. Design inputs are the constants and targets below.
 * bun basketball-player.ts [--no-save] @see ../../📋️brief.md */
import { BALL, BALL_MATERIALS, ballShapes, ballVolume } from "./ball.ts";
import { binding, calibrateAreas, measureDimensions, mergeRegion, removeParts, restFrame, retone, writeManifestRow } from "./person.ts";
import { decisions, volumeDecisions, type Kit, type PropText } from "./texts.ts";
import { SEED, anchorPoint, armVars, flatFeet, legVars, type V3 } from "./athlete.ts";
import { GENERATED, at, clone, comOf, duBois, framesOf, loadHuman, mifflin, r, saveHuman, scaleAsset, setBodyMass, solidify, solvePose, tx, type Goal, type Json, type Offsets, type T2 } from "./lib.ts";

const STATURE = 1.98, SOURCE_STATURE = 1.82, BODY_KG = 95, AGE = 24;
const s = STATURE / SOURCE_STATURE;
const doc = await loadHuman("jogger");
const a = doc.asset;
scaleAsset(a, s);
setBodyMass(a, BODY_KG);
a.id = "basketball-player";
const H0 = r(0.9617 * s, 4);
const SHORTS_DROP = r(0.19 * s, 4);

//#region 🦴️Rig on the court
const bone = (id: string) => a.rig.bones.find((b: Json) => b.id === id);
a.rig.root = "court";
bone("pelvis").parent = "hip-shift";
a.rig.bones.unshift(
  { id: "court", parent: null, title: tx("Court floor point", "Punkt am Spielfeldboden"), x: 0, y: 0, angleDeg: 0, lengthM: 0, joint: { type: "fixed" } },
  { id: "hip-lift", parent: "court", title: tx("Hip height", "Hüfthöhe"), x: 0, y: 0, angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 90, minM: 0, maxM: 1.6 } },
  { id: "hip-shift", parent: "hip-lift", title: tx("Hip shift forward", "Hüftverschiebung nach vorn"), x: 0, y: 0, angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 0, minM: -0.5, maxM: 0.5 } },
);
a.rig.bones.push(
  { id: "ball-lift", parent: "court", title: tx("Ball height", "Ballhöhe"), x: 0, y: BALL.radius, angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 90, minM: 0, maxM: 3 } },
  { id: "ball-shift", parent: "ball-lift", title: tx("Ball forward", "Ball nach vorn"), x: 0.35, y: 0, angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 0, minM: -1.5, maxM: 1.5 } },
  { id: "ball-turn", parent: "ball-shift", title: tx("Lateral guide of the ball", "Seitliche Führung des Balls"), x: 0, y: 0, angleDeg: 0, abductionDeg: 90, lengthM: 0.01, joint: { type: "fixed" } },
  { id: "ball-side", parent: "ball-turn", title: tx("Ball sideways", "Ball seitwärts"), x: -0.4, y: 0, angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 0, minM: -1, maxM: 1 } },
  { id: "ball", parent: "ball-side", title: tx("Basketball", "Basketball"), x: 0, y: 0, angleDeg: 0, abductionDeg: -90, lengthM: r(2 * BALL.radius, 4), joint: { type: "fixed" }, mass: { kg: BALL.massKg, comX: 0, comY: 0, distribution: "point", source: "literature", reference: "fiba-equipment-2024" } },
);
//#endregion

//#region 🧵️Kit: sleeveless jersey, knee-length shorts, high-tops
const mat = (id: string) => a.materials.find((m: Json) => m.id === id);
Object.assign(mat("nylon-clothing"), { title: tx("Polyester mesh basketball jersey and shorts", "Basketballtrikot und -shorts aus Polyester-Mesh"), solarAbsorptivity: 0.7 });
Object.assign(mat("leather-shoe"), { title: tx("High-top basketball shoe (synthetic upper, rubber cupsole)", "Hoher Basketballschuh (synthetisches Obermaterial, Gummi-Cupsole)"), solarAbsorptivity: 0.3 });
a.materials.push(...BALL_MATERIALS);
removeParts(a, ["armband", "armband-pouch", "sleeve-hem-l", "sleeve-hem-r"]);
mergeRegion(a, "arm-clothing", "arm-skin", "skin", 0.72);
mergeRegion(a, "ankle-socks", "shoes", "leather-shoe", 0.9);
const region = (id: string) => a.thermal.regions.find((g: Json) => g.id === id);
Object.assign(region("torso-clothing"), { title: tx("Sleeveless mesh jersey", "Ärmelloses Mesh-Trikot") });
Object.assign(region("arm-skin"), { title: tx("Bare arms and shoulders", "Nackte Arme und Schultern") });
Object.assign(region("leg-clothing"), { title: tx("Knee-length basketball shorts", "Knielange Basketballshorts") });
Object.assign(region("leg-skin"), { title: tx("Bare knees and calves", "Nackte Knie und Waden") });
Object.assign(region("shoes"), { title: tx("High-top shoes over crew socks", "Hohe Schuhe über Crew-Socken") });
Object.assign(region("headband"), { title: tx("Terry headband", "Frottee-Stirnband") });
retone(a, "torso-clothing", 0.3);
retone(a, "leg-clothing", 0.3);
retone(a, "shoes", 0.9);
/** 👖️ Lengthens the shorts legs on the thighs by `drop`: every outline point and loft section at the hem moves down the thigh, so the leg hangs straight and loose to the knee. */
function lengthenShorts(drop: number): void {
  const thighs = ["thigh-l", "thigh-r"];
  for (const rep of Object.values(a.representations) as Json[]) for (const v of ["elevation", "front", "plan"]) for (const sh of rep.geometry[v]) {
    if (!thighs.includes(sh.bone) || sh.region !== "leg-clothing") continue;
    if (v === "elevation" || sh.solid?.kind === "revolved") { const hem = Math.max(...sh.points.map((p: number[]) => p[0])); sh.points = sh.points.map((p: number[]) => (p[0]! >= hem - 0.03 ? [r(p[0]! + drop, 5), p[1]] : p)); }
    else if (!sh.solid) sh.points = sh.points.map((p: number[]) => [p[0], r(p[1]! - drop, 5)]);
    else throw new Error(`lengthenShorts: ${sh.id} ${v}`);
  }
  for (const vol of Object.values(a.volumes) as Json[]) for (const list of ["body", "thermal"]) for (const so of vol[list]) {
    if (!thighs.includes(so.bone) || so.of || !(list === "thermal" ? so.region === "leg-clothing" : String(so.id).startsWith("shorts-leg"))) continue;
    if (so.kind !== "loft") throw new Error(`lengthenShorts: ${so.id} ${so.kind}`);
    const hem = Math.max(...so.sections.map((x: Json) => x.at[0]));
    for (const x of so.sections) if (x.at[0] >= hem - 0.03) x.at = [r(x.at[0] + drop, 5), x.at[1], x.at[2]];
  }
}
lengthenShorts(SHORTS_DROP);
//#endregion

//#region 🏀️Ball
const ballRest = restFrame(a, "ball");
console.log("ball rest frame", JSON.stringify(ballRest));
for (const [k, rep] of Object.entries(a.representations) as [string, Json][]) for (const v of ["elevation", "front", "plan"]) rep.geometry[v].push(...ballShapes(k, v, v === "front" ? [ballRest.o[2], ballRest.o[1]] : v === "plan" ? [ballRest.o[0], ballRest.o[2]] : [0, 0]));
for (const [k, vol] of Object.entries(a.volumes) as [string, Json][]) { const bv = ballVolume(k); vol.body.push(...bv.body); vol.thermal.push(...bv.thermal); vol.structure.push(...bv.structure); }
a.thermal.regions.push({ id: "ball", title: tx("Basketball cover", "Basketballhülle"), kind: "surface", model: "passive", material: "ball-composite", areaM2: r(4 * Math.PI * BALL.radius ** 2, 4), characteristicLengthM: r(2 * BALL.radius, 3), geometry: "sphere", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 1800, ratedPowerW: 0, ratedPowerBasis: "passive", source: "authored-estimate", reference: "fiba-equipment-2024" });
//#endregion

//#region 🧍️Poses
const SPREAD = ["torso", "chest", "neck", "head", "thigh-l", "shank-l", "foot-l", "toe-l", "thigh-r", "shank-r", "foot-r", "toe-r"];
const FEET = ["heel-l", "contact-l", "heel-r", "contact-r"];
const PALM = r(0.055 * s, 4), GAP = BALL.radius + 0.008;
const ROOT = { x: 0, y: 0, angleDeg: 0 };
/** 🖐️ A hand with its palm on the ball surface: the palm normal (+w for the right hand, -w for the left) points at the ball centre. */
const palmOn = (side: "l" | "r", centre: V3, dirToHand: V3, fingers: V3, weight = 3): Goal[] => {
  const n = Math.hypot(...dirToHand), d = dirToHand.map((x) => x / n) as V3;
  return [
    { kind: "point", bone: `hand-${side}`, p: [PALM, 0, 0], target: [centre[0] + d[0] * GAP, centre[1] + d[1] * GAP, centre[2] + d[2] * GAP], weight },
    { kind: "axis", bone: `hand-${side}`, axis: "w", dir: (side === "r" ? d.map((x) => -x) : d) as V3, weight: 1.2 },
    { kind: "axis", bone: `hand-${side}`, axis: "u", dir: fingers, weight: 0.5 },
  ];
};
type Hand = { side: "l" | "r"; dir: V3; fingers: V3 } | { side: "l" | "r"; at: V3 };
type Design = { id: string; key?: boolean; title: T2; description: T2; hip: number; shift: number; pelvisDeg: number; trunk: Offsets; feet: Goal[] | null; legs?: Offsets; ball: V3; hands: Hand[]; met: number; breathing: string; wet: number; contacts: string[] };
function design(d: Design): { offsets: Offsets; report: string } {
  const base: Offsets = { ...clone(SEED), ...clone(d.legs ?? {}), ...clone(d.trunk), "hip-lift": { dy: r(d.hip, 5) }, "hip-shift": { dx: d.shift }, pelvis: { angleDeg: d.pelvisDeg } };
  let res = d.feet ? solvePose(a, base, legVars(), d.feet, { root: ROOT, iterations: 300, reg: 1e-6 }) : { offsets: base, root: ROOT, error: 0 };
  const errs = [res.error];
  for (const h of d.hands) {
    const goals: Goal[] = "at" in h ? [{ kind: "point", bone: `hand-${h.side}`, p: anchorPoint(a, `grip-${h.side}`), target: h.at, weight: 3 }] : palmOn(h.side, d.ball, h.dir, h.fingers);
    res = solvePose(a, res.offsets, armVars(h.side, true), goals, { root: ROOT, iterations: 500, reg: 1e-4 });
    errs.push(res.error);
  }
  const off = res.offsets;
  off["ball-lift"] = { dy: r(d.ball[1] - BALL.radius, 5) };
  off["ball-shift"] = { dx: r(d.ball[0] - 0.35, 5) };
  off["ball-side"] = { dx: r(d.ball[2] + 0.4, 5) };
  const { frames } = framesOf(a, off, ROOT);
  const ballAt = at(frames["ball"]!, [0, 0, 0]);
  const com = comOf(a, frames);
  return { offsets: off, report: `${d.id}: ${errs.map((e) => e.toFixed(4)).join(" ")} ball ${ballAt.map((x) => x.toFixed(3)).join(" ")} com ${com[0].toFixed(3)} ${com[2].toFixed(3)}` };
}
const STANCE = flatFeet(a, 0.2, 14, 0.13, -0.09);
const DRIBBLE_TRUNK: Offsets = { torso: { angleDeg: -8 }, chest: { angleDeg: -5 }, neck: { angleDeg: 14 }, head: { angleDeg: 12 } };
const BALL_TOP: V3 = [0.32, 0.86, -0.46];
const designs: Design[] = [
  { id: "dribbling", title: tx("Dribbling", "Dribbeln"), description: tx("Low, staggered stance with bent knees; the right hand pushes the ball down beside the right foot, the left forearm shields it.", "Tiefe, versetzte Stellung mit gebeugten Knien; die rechte Hand drückt den Ball neben dem rechten Fuss nach unten, der linke Unterarm schirmt ihn ab."), hip: 0.96, shift: 0, pelvisDeg: -16, trunk: DRIBBLE_TRUNK, feet: STANCE, ball: BALL_TOP, hands: [{ side: "r", dir: [-0.35, 1, 0.15], fingers: [0.6, -0.45, -0.2] }, { side: "l", at: [0.3, 1.2, 0.24] }], met: 6.5, breathing: "play", wet: 0.25, contacts: FEET },
  { id: "dribble-bounce", key: true, title: tx("Ball at the floor", "Ball am Boden"), description: tx("Key pose of the dribble: the ball touches the floor, the right hand waits above it.", "Schlüsselpose des Dribbelns: der Ball berührt den Boden, die rechte Hand wartet darüber."), hip: 0.96, shift: 0, pelvisDeg: -16, trunk: DRIBBLE_TRUNK, feet: STANCE, ball: [0.36, BALL.radius, -0.46], hands: [{ side: "r", at: [0.36, 0.98, -0.42] }, { side: "l", at: [0.3, 1.2, 0.24] }], met: 6.5, breathing: "play", wet: 0.25, contacts: FEET },
  { id: "jump-shot-set", title: tx("Jump shot, set", "Sprungwurf, Ansatz"), description: tx("Square stance, knees bent for the jump; the ball rests on the right hand in front of the forehead, the left hand guides it from the side.", "Paralleler Stand, Knie zum Absprung gebeugt; der Ball liegt vor der Stirn auf der rechten Hand, die linke Hand führt ihn von der Seite."), hip: 0.92, shift: -0.02, pelvisDeg: -10, trunk: { torso: { angleDeg: -2 }, chest: { angleDeg: 2 }, neck: { angleDeg: 6 }, head: { angleDeg: 4 } }, feet: flatFeet(a, 0.18, 10, 0.03), ball: [0.24, 1.78, -0.1], hands: [{ side: "r", dir: [-0.55, -0.8, -0.1], fingers: [0.2, 0.9, 0] }, { side: "l", dir: [0, -0.15, 1], fingers: [0.4, 0.9, 0] }], met: 4.5, breathing: "play", wet: 0.25, contacts: FEET },
  { id: "jump-shot-release", title: tx("Jump shot, release", "Sprungwurf, Abwurf"), description: tx("At the top of a 0.35 m jump: body extended, feet pointed, the shooting arm reaches up and forward and the wrist has flicked the ball off the fingertips; the guide hand stays beside the ball's path.", "Am höchsten Punkt eines 0,35-m-Sprungs: Körper gestreckt, Füsse gestreckt, der Wurfarm reicht nach oben vorn und das Handgelenk hat den Ball von den Fingerspitzen geschnippt; die Führhand bleibt neben der Ballbahn."), hip: H0 + 0.35, shift: 0, pelvisDeg: 2, trunk: { torso: { angleDeg: 2 }, chest: { angleDeg: 2 }, neck: { angleDeg: 8 }, head: { angleDeg: 6 } }, feet: null, legs: { "thigh-l": { angleDeg: 6 }, "shank-l": { angleDeg: -12 }, "foot-l": { angleDeg: -38 }, "thigh-r": { angleDeg: 2 }, "shank-r": { angleDeg: -6 }, "foot-r": { angleDeg: -40 }, "toe-l": { angleDeg: 0 }, "toe-r": { angleDeg: 0 } }, ball: [0.62, 2.74, -0.12], hands: [{ side: "r", at: [0.5, 2.6, -0.14] }, { side: "l", at: [0.38, 2.3, 0.08] }], met: 8, breathing: "play", wet: 0.25, contacts: [] },
  { id: "standing-with-ball", title: tx("Standing with the ball", "Stehen mit dem Ball"), description: tx("Upright at rest between plays, the ball held in front of the waist with both hands.", "Aufrecht in einer Spielpause, der Ball vor der Taille mit beiden Händen gehalten."), hip: H0 - 0.005, shift: 0, pelvisDeg: 0, trunk: {}, feet: flatFeet(a, 0.13, 8, 0.02), ball: [0.3, 1.04, 0], hands: [{ side: "r", dir: [0, 0.1, -1], fingers: [0.5, -0.4, 0] }, { side: "l", dir: [0, 0.1, 1], fingers: [0.5, -0.4, 0] }], met: 1.8, breathing: "rest", wet: 0.1, contacts: FEET },
];
const wetted = (w: number) => Object.fromEntries(["head-skin", "arm-skin", "leg-skin", "hand-skin"].map((id) => [id, { wettedness: w }]));
const poses: Json[] = designs.map((d) => {
  const { offsets, report } = design(d);
  console.log(report);
  const feet = d.contacts.length ? { shoes: { contact: { fraction: 0.52, tempC: 20 } } } : {};
  return { id: d.id, ...(d.key ? { key: true } : {}), title: d.title, description: d.description, bones: offsets, root: ROOT, ground: "authored", contacts: d.contacts, thermal: binding(a, d.met, d.breathing, d.met > 2 ? 1 : 0.6, { ...wetted(d.wet), ...feet }), loads: [] };
});
//#endregion

//#region 🫁️Respiration
a.thermal.respiration.patterns = [
  { id: "rest", title: tx("Calm breathing in a break", "Ruhige Atmung in einer Pause"), rateBpm: 15, tidalVolumeL: 0.8, inhaleFraction: 0.4, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.46, pauseAfterExhaleFraction: 0.1, route: "nose", source: "authored-estimate", reference: "gupta-2010" },
  { id: "play", title: tx("Breathing in play through nose and mouth", "Atmung im Spiel durch Nase und Mund"), rateBpm: 32, tidalVolumeL: 1.8, inhaleFraction: 0.44, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.04, route: "nose-and-mouth", source: "authored-estimate", reference: "astrand-rodahl-2003" },
];
//#endregion

//#region 🎞️Clips
const BOUNCE = 0.5;
const ALL_FEET = (d: number) => FEET.map((anchor) => ({ anchor, fromS: 0, toS: d }));
const clips: Json[] = [
  {
    id: "dribble", title: tx("Stationary dribble", "Dribbeln auf der Stelle"),
    description: tx("Two bounces per second beside the right foot: the hand pushes the ball down from hip height, it hits the floor after 0.2 s and rises back into the waiting hand.", "Zwei Ballkontakte pro Sekunde neben dem rechten Fuss: die Hand drückt den Ball aus Hüfthöhe nach unten, er trifft nach 0,2 s den Boden und steigt zurück in die wartende Hand."),
    durationS: BOUNCE, fps: 30, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "dribbling", ease: "in" }, { t: 0.2, pose: "dribble-bounce", ease: "out" }],
    contacts: ALL_FEET(BOUNCE),
    phases: [{ id: "push", fromS: 0, toS: 0.2, title: tx("Push down", "Abdrücken") }, { id: "rebound", fromS: 0.2, toS: BOUNCE, title: tx("Rebound into the hand", "Rückprall in die Hand") }],
    events: [{ t: 0, kind: "motion", title: tx("Hand releases the ball", "Hand gibt den Ball ab") }, { t: 0.2, kind: "motion", title: tx("Ball hits the floor", "Ball trifft den Boden") }],
    tracks: [{ target: "activityMet", ease: "step", keys: [{ t: 0, v: 6.5 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }] }, { target: "emitter:plume", ease: "step", keys: [{ t: 0, v: 1 }] }],
  },
  {
    id: "idle-breathing", title: tx("Idle breathing with the ball", "Ruhiges Atmen mit dem Ball"),
    description: tx("Standing in a break with the ball held at the waist; one calm breath every 4 s moves chest and belly.", "Stehen in einer Pause, der Ball an der Taille gehalten; alle 4 s ein ruhiger Atemzug hebt Brust und Bauch."),
    durationS: 4, fps: 30, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "standing-with-ball", ease: "smooth" }, { t: 2, pose: "standing-with-ball", ease: "smooth", bones: { head: { angleDeg: -2 } } }],
    contacts: ALL_FEET(4),
    phases: [{ id: "breath", fromS: 0, toS: 4, title: tx("One calm breath", "Ein ruhiger Atemzug") }],
    events: [],
    tracks: [{ target: "activityMet", ease: "step", keys: [{ t: 0, v: 1.8 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }] }],
  },
];
//#endregion

//#region 🧾️Identity, provenance and texts
a.title = tx("Basketball Player", "Basketballspielende Person");
a.summary = tx(
  "A tall basketball player of 1.98 m and 95 kg (male build) in a sleeveless mesh jersey, knee-length shorts, headband and high-top shoes with a size 7 ball: a low staggered dribbling stance with a looping two-bounce-per-second dribble, the set and the airborne release of a jump shot at the top of a 0.35 m jump, and standing with the ball while breathing calmly; four individually drawn scales from 1:500 to 1:50.",
  "Ein grosser Basketballspieler von 1,98 m und 95 kg (männlicher Körperbau) in ärmellosem Mesh-Trikot, knielangen Shorts, Stirnband und hohen Basketballschuhen mit einem Ball der Grösse 7: tiefe, versetzte Dribbelstellung mit einem wiederholten Dribbeln von zwei Ballkontakten pro Sekunde, Ansatz und Abwurf eines Sprungwurfs am höchsten Punkt eines 0,35-m-Sprungs und Stehen mit dem Ball bei ruhiger Atmung; vier einzeln gezeichnete Massstäbe von 1:500 bis 1:50.");
a.tags = ["human", "sport", "basketball", "ball", "ball-sport", "athlete", "tall"];
a.provenance = {
  author: "Semio staffage human worker HW-07", method: "handcrafted", created: "2026-11-22",
  references: [
    ...a.provenance.references.filter((x: Json) => !["cavanagh-1980", "dalleau-1998", "bramble-carrier-1983"].includes(x.id)),
    { id: "din-33402-2-2020", citation: "DIN 33402-2:2020-12 Ergonomie, Körpermasse des Menschen, Teil 2: Werte (stature, segment lengths and breadths of adults)." },
    { id: "fiba-equipment-2024", citation: "FIBA (2024). Official Basketball Rules, Basketball Equipment: ball size 7, circumference 749 to 780 mm, mass 567 to 650 g.", url: "https://about.fiba.basketball/en/rules" },
    { id: "struzik-2014", citation: "Struzik, A. et al. (2014). Jump shot performance in basketball: a review of biomechanical factors (jump height and release at the apex of the jump). Journal of Human Kinetics 42, 73-79." },
    { id: "abdelkrim-2007", citation: "Ben Abdelkrim, N., El Fazaa, S., El Ati, J. (2007). Time-motion analysis and physiological data of elite under-19 basketball players during competition. British Journal of Sports Medicine 41(2), 69-75." },
  ],
  notes: tx(
    "Derived by HW-07 from the catalog jogger: rig, outlines and volumes rescaled to a stature of 1.98 m (factor 1.088), segment masses re-weighted to 95 kg with the de Leva fractions, the rig set on a fixed court root with prismatic hip lift and shift, the sleeves turned into the bare deltoids and upper arms and the armband removed (sleeveless jersey), the shorts lengthened by 0.21 m to the knee, the sock bands turned into the collars of high-top shoes, and the ball chain, poses, clips and physiology authored anew. Generator: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-07/basketball-player.ts.",
    "Von HW-07 aus der Katalogfigur Jogger abgeleitet: Rig, Umrisse und Volumen auf eine Körperhöhe von 1,98 m skaliert (Faktor 1,088), Segmentmassen mit den Anteilen nach de Leva auf 95 kg gewichtet, das Rig auf einen festen Spielfeldpunkt mit prismatischem Hüfthub und -schub gestellt, die Ärmel zu nackten Deltamuskeln und Oberarmen gemacht und das Armband entfernt (ärmelloses Trikot), die Shorts um 0,21 m bis zum Knie verlängert, die Sockenbündchen zu Schaftkragen hoher Schuhe gemacht und Ballkette, Posen, Clips und Physiologie neu verfasst. Generator: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-07/basketball-player.ts."),
};
const kit: Kit = { top: tx("sleeveless jersey", "ärmellosen Trikot"), shorts: tx("knee-length shorts", "knielange Shorts"), feet: tx("high-top shoes with collar band", "hohe Schuhe mit Schaftkragen"), band: tx("headband", "Stirnband") };
const ballText: Record<string, PropText> = {
  "50": { silhouette: tx("The ball is a 239 mm disc with its panel seams drawn as the ball shows them from the side, from ahead and from above.", "Der Ball ist eine Scheibe von 239 mm mit den Nähten seiner Felder, wie er sie von der Seite, von vorn und von oben zeigt."), thinElements: tx("Seams are 0.18 mm detail strokes; the ball only translates, so flat seams stay true.", "Nähte sind 0,18-mm-Detailstriche; der Ball verschiebt sich nur, darum bleiben flache Nähte richtig."), overlays: tx("The ball is its own passive region.", "Der Ball ist eine eigene passive Region.") },
  "100": { silhouette: tx("Ball: disc with the equator seam and one curved seam.", "Ball: Scheibe mit Äquatornaht und einer gebogenen Naht.") },
  "200": { silhouette: tx("Ball: disc with the equator seam.", "Ball: Scheibe mit der Äquatornaht."), omitted: tx("The curved panel seams of the ball.", "Die gebogenen Feldnähte des Balls.") },
  "500": { silhouette: tx("Ball: one dark disc of 0.48 mm.", "Ball: eine dunkle Scheibe von 0,48 mm."), omitted: tx("Every seam of the ball.", "Alle Nähte des Balls.") },
};
for (const k of ["50", "100", "200", "500"]) a.representations[k].decisions = decisions(k, kit, ballText[k]!);
const who = tx("a 1.98 m basketball player", "ein 1,98 m grosser Basketballspieler");
for (const k of ["50", "100", "200", "500"]) a.volumes[k].decisions = volumeDecisions(k, who, { body: tx("The ball is a sphere of 0.239 m; the shorts legs hang straight to the knee, the arms are bare from the shoulder.", "Der Ball ist eine Kugel von 0,239 m; die Hosenbeine hängen gerade bis zum Knie, die Arme sind ab der Schulter nackt."), thermal: tx("The ball is one passive region over its sphere.", "Der Ball ist eine passive Region über seiner Kugel."), structure: tx("The ball is a member of its own on the court chain; it is carried rigidly, its bounce is not a support.", "Der Ball ist ein eigenes Glied an der Spielfeldkette; er wird starr getragen, sein Aufprall ist kein Auflager.") });
a.thermal.assumptions = [
  tx(`DuBois skin area for 1.98 m and 95 kg (${r(duBois(BODY_KG, STATURE), 2)} m2); a sleeveless polyester mesh jersey (0.06 clo) and knee-length shorts (0.06 clo) after ISO 9920 leave arms, knees and calves bare; layer resistances include the enclosed air.`, `DuBois-Hautfläche für 1,98 m und 95 kg (${String(r(duBois(BODY_KG, STATURE), 2)).replace(".", ",")} m2); ein ärmelloses Polyester-Mesh-Trikot (0,06 clo) und knielange Shorts (0,06 clo) nach ISO 9920 lassen Arme, Knie und Waden frei; die Schichtwiderstände enthalten die eingeschlossene Luft.`),
  tx("Metabolic rate 6.5 met while dribbling (Compendium 15055, basketball, general), 4.5 met in the shooting set (15075, shooting baskets), 8 met in the jump, 1.8 met standing in a break; skin wettedness 0.25 in play. Skin blood flow limits 6.3 to 90 L/(m2 h), set points 36.8/34.1 degrees Celsius after the Gagge two-node model; basal metabolism by the Mifflin-St Jeor equation for men.", "Stoffwechsel 6,5 met beim Dribbeln (Compendium 15055, Basketball allgemein), 4,5 met im Wurfansatz (15075, Korbwerfen), 8 met im Sprung, 1,8 met im Stehen in einer Pause; Hautbenetzung 0,25 im Spiel. Hautdurchblutung 6,3 bis 90 L/(m2 h), Sollwerte 36,8/34,1 Grad Celsius nach dem Zwei-Knoten-Modell von Gagge; Grundumsatz nach der Mifflin-St-Jeor-Gleichung für Männer."),
  tx("Play breathing 32 breaths per minute with 1.8 L (58 L/min), calm breathing 15 per minute with 0.8 L through the nose; exhaled air 34 degrees Celsius, 95 % humidity. Authored estimates guided by the cited sources.", "Atmung im Spiel 32 Atemzüge pro Minute mit 1,8 L (58 L/min), ruhige Atmung 15 pro Minute mit 0,8 L durch die Nase; Ausatemluft 34 Grad Celsius, 95 % Feuchte. Autorenschätzungen nach den genannten Quellen."),
  tx("The ball is a passive composite-leather surface (0.62 kg, about 1.8 kJ/(m2 K)); contact conduction through the soles uses 26 % of the shoe area per planted foot at a 20 degrees Celsius hall floor.", "Der Ball ist eine passive Kompositlederoberfläche (0,62 kg, etwa 1,8 kJ/(m2 K)); die Wärmeleitung durch die Sohlen nutzt 26 % der Schuhfläche je aufgesetztem Fuss bei 20 Grad Celsius Hallenboden."),
];
a.structure = {
  gravityMS2: 9.80665, dynamics: "quasi-static", supports: [],
  assumptions: [
    tx("Quasi-static model: segment masses as point masses at the de Leva centres on four foot contact anchors. The ball (0.62 kg) is carried rigidly by its chain from the court point, so its weight reaches the feet; its bounce is not modelled as a support and the dribble is a sequence of static equilibria. The release pose is airborne and has no support by design.", "Quasistatisches Modell: Segmentmassen als Punktmassen in den Schwerpunkten nach de Leva auf vier Fusskontaktankern. Der Ball (0,62 kg) wird von seiner Kette ab dem Spielfeldpunkt starr getragen, sein Gewicht erreicht also die Füsse; sein Aufprall ist nicht als Auflager modelliert und das Dribbeln ist eine Folge statischer Gleichgewichte. Die Abwurfpose ist in der Luft und hat bewusst kein Auflager."),
    tx("Joint loads are rigid-segment resultants, not clinical bone or muscle forces.", "Gelenklasten sind Resultierende starrer Segmente, keine klinischen Knochen- oder Muskelkräfte."),
  ],
  clinical: false,
};
a.symbols = a.symbols.map((sym: Json) => ({ ...sym, geometry: Object.fromEntries(Object.entries(sym.geometry).map(([v, shapes]) => [v, (shapes as Json[]).map((sh) => ({ ...sh, bone: "court" }))])), id: "player-position-mark", title: tx("Player position and facing mark", "Positions- und Blickrichtungsmarke der Spielperson"), description: tx("A small circle with a facing tick marking where a player stands and which way they face, for court plans with many people.", "Ein kleiner Kreis mit Richtungsstrich, der Standort und Blickrichtung einer Spielperson in Spielfeldplänen mit vielen Personen markiert."), dimensionalNote: tx("Only the insertion point is true (the hip position); circle and tick are fixed paper sizes of 1 mm and 1.1 mm and carry no dimension.", "Nur der Einfügepunkt ist massgebend (Hüftposition); Kreis und Strich haben feste Papiergrössen von 1 mm und 1,1 mm und tragen keine Masse.") }));
//#endregion

a.poses = poses;
a.clips = clips;
solidify(a, SPREAD);
a.presentation = { defaultPose: "dribbling", defaultClip: "dribble", defaultScale: 100 };
a.completion = "authored";
a.thermal.body = { ...a.thermal.body, massKg: BODY_KG, skinAreaM2: r(duBois(BODY_KG, STATURE), 3), basalMetabolismW: r(mifflin(BODY_KG, STATURE, AGE, "male"), 1), source: "authored-estimate", reference: "gagge-1971" };
console.log("areas", calibrateAreas(a));
if (process.argv.includes("--dump")) await Bun.write(`${GENERATED}/${a.id}.json`, JSON.stringify(doc));
const dims = measureDimensions(a);
console.log("dims", JSON.stringify(a.dimensions), JSON.stringify(dims.byScale));
if (!process.argv.includes("--no-save")) {
  console.log(await saveHuman(doc));
  console.log(JSON.stringify(await writeManifestRow(a, tx("The tallest person of the catalog and the only one with a free ball: the ball hangs on a chain of its own from the court, bounces to the floor in the dribble and leaves the fingertips at the top of an airborne jump shot.", "Die grösste Person des Katalogs und die einzige mit einem freien Ball: der Ball hängt an einer eigenen Kette vom Spielfeld, springt beim Dribbeln auf den Boden und verlässt beim Sprungwurf in der Luft die Fingerspitzen."))));
}
