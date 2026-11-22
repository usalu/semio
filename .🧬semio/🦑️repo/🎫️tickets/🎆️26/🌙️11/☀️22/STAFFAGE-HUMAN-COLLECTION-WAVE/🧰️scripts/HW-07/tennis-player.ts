/** 🎾️ Generator of `👤️humans/tennis-player`: a right-handed club player derived from the catalog jogger (same athletic build, rescaled to 1.74 m
 * and 66 kg), in a white tennis kit with a 27 in midplus racket; ready stance, forehand backswing, contact and follow-through, a looping forehand
 * and idle breathing. Design inputs are the constants and pose targets below. bun tennis-player.ts [--no-save]
 * @see ../../📋️brief.md */
import { RACKET, RACKET_MATERIALS, racketShapes, racketVolume } from "./racket.ts";
import { binding, calibrateAreas, measureDimensions, removeParts, restFrame, retone, writeManifestRow } from "./person.ts";
import { decisions, volumeDecisions, type Kit, type PropText } from "./texts.ts";
import { SEED, anchorPoint, armVars, flatFeet, legVars } from "./athlete.ts";
import { clone, duBois, loadHuman, mifflin, r, saveHuman, scaleAsset, setBodyMass, solidify, solvePose, framesOf, at, comOf, tx, type Goal, type Json, type Offsets, type T2, type Variable } from "./lib.ts";

const STATURE = 1.74, SOURCE_STATURE = 1.82, BODY_KG = 66, AGE = 27;
const s = STATURE / SOURCE_STATURE;
const doc = await loadHuman("jogger");
const a = doc.asset;
scaleAsset(a, s);
setBodyMass(a, BODY_KG);
a.id = "tennis-player";

//#region 🧵️Kit and materials
const mat = (id: string) => a.materials.find((m: Json) => m.id === id);
Object.assign(mat("nylon-clothing"), { title: tx("White polyester tennis kit", "Weisse Tennisbekleidung aus Polyester"), solarAbsorptivity: 0.25 });
Object.assign(mat("leather-shoe"), { title: tx("Tennis shoe (synthetic leather upper, herringbone rubber sole)", "Tennisschuh (Obermaterial aus Kunstleder, Gummisohle mit Fischgrätprofil)"), solarAbsorptivity: 0.3 });
a.materials.push(...RACKET_MATERIALS);
const region = (id: string) => a.thermal.regions.find((g: Json) => g.id === id);
Object.assign(region("torso-clothing"), { title: tx("Polo shirt", "Poloshirt") });
Object.assign(region("arm-clothing"), { title: tx("Short polo sleeves", "Kurze Ärmel des Polos") });
Object.assign(region("leg-clothing"), { title: tx("Tennis shorts", "Tennisshorts") });
Object.assign(region("shoes"), { title: tx("Tennis shoes over socks", "Tennisschuhe über Socken") });
retone(a, "torso-clothing", 0.94);
retone(a, "arm-clothing", 0.94);
retone(a, "leg-clothing", 0.9);
//#endregion

//#region 🎾️Racket
const grip = a.anchors.find((x: Json) => x.id === "grip-r");
a.rig.bones.push({
  id: "racket", parent: "hand-r", title: tx("Tennis racket", "Tennisschläger"), x: grip.x, y: grip.y, angleDeg: 95, lengthM: RACKET.tip, joint: { type: "fixed" },
  mass: { kg: RACKET.massKg, comX: r(RACKET.balanceFromButt + RACKET.butt, 4), comY: 0, distribution: "point", source: "authored-estimate", reference: "itf-rules-2024" },
});
const restZ = restFrame(a, "racket").o[2];
for (const [k, rep] of Object.entries(a.representations) as [string, Json][]) {
  for (const v of ["elevation", "front", "plan"]) rep.geometry[v].push(...racketShapes(k, restZ));
  if (k === "50" || k === "100") rep.hatches.push({ id: "strings", title: tx("String bed", "Besaitung"), pattern: "cross", angleDeg: 0, spacingMm: k === "50" ? 0.45 : 0.5, penMm: 0.1 });
}
for (const [k, vol] of Object.entries(a.volumes) as [string, Json][]) { const rv = racketVolume(k); vol.body.push(...rv.body); vol.thermal.push(...rv.thermal); vol.structure.push(...rv.structure); }
a.thermal.regions.push({
  id: "racket", title: tx("Racket frame, strings and grip", "Schlägerrahmen, Saiten und Griff"), kind: "surface", model: "passive", material: "racket-composite", areaM2: 0.19,
  characteristicLengthM: 0.26, geometry: "vertical-plate", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 1600, ratedPowerW: 0, ratedPowerBasis: "passive", source: "authored-estimate", reference: "itf-rules-2024",
});
//#endregion

//#region 🧍️Poses
const SPREAD = ["torso", "chest", "neck", "head", "thigh-l", "shank-l", "foot-l", "toe-l", "thigh-r", "shank-r", "foot-r", "toe-r"];
const FEET = ["heel-l", "contact-l", "heel-r", "contact-r"];
type PoseDesign = { id: string; title: ReturnType<typeof tx>; description: ReturnType<typeof tx>; hip: number; pelvisDeg: number; trunk: Offsets; stance: [number, number, number]; racket: { grip: [number, number, number]; dir: [number, number, number]; normal: [number, number, number] }; left: "throat" | { at: [number, number, number] }; met: number; breathing: string; wet: number };
function designPose(d: PoseDesign): { offsets: Offsets; root: Json; report: string } {
  const root = { x: 0, y: d.hip, angleDeg: d.pelvisDeg };
  let res = solvePose(a, { ...clone(SEED), ...clone(d.trunk) }, legVars(), flatFeet(a, ...d.stance), { root, iterations: 300, reg: 1e-6 });
  const e1 = res.error;
  res = solvePose(a, res.offsets, armVars("r"), [
    { kind: "point", bone: "racket", p: [0, 0, 0], target: d.racket.grip, weight: 3 },
    { kind: "axis", bone: "racket", axis: "u", dir: d.racket.dir, weight: 1 },
    { kind: "axis", bone: "racket", axis: "w", dir: d.racket.normal, weight: 0.6 },
  ], { root, iterations: 500, reg: 1e-4 });
  const e2 = res.error;
  const { frames } = framesOf(a, res.offsets, root);
  const target = d.left === "throat" ? at(frames["racket"]!, [0.2, 0, 0]) : d.left.at;
  res = solvePose(a, res.offsets, armVars("l"), [{ kind: "point", bone: "hand-l", p: anchorPoint(a, "grip-l"), target, weight: 3 }], { root, iterations: 500, reg: 1e-4 });
  const f2 = framesOf(a, res.offsets, root).frames;
  const com = comOf(a, f2);
  return { offsets: res.offsets, root, report: `${d.id}: legs ${e1.toFixed(4)} racket ${e2.toFixed(4)} left ${res.error.toFixed(4)} com x ${com[0].toFixed(3)} z ${com[2].toFixed(3)}` };
}
const H = (x: number) => r(x * s, 4);
const READY: Offsets = { torso: { angleDeg: -6 }, chest: { angleDeg: -4 }, neck: { angleDeg: 10 }, head: { angleDeg: 10 } };
const designs: PoseDesign[] = [
  { id: "ready-stance", title: tx("Ready stance", "Bereitschaftsstellung"), description: tx("Feet wider than the shoulders, knees and hips bent, weight on the balls of the feet; the racket is held in front of the body with the left hand on its throat.", "Füsse weiter als schulterbreit, Knie und Hüften gebeugt, das Gewicht auf den Fussballen; der Schläger wird vor dem Körper gehalten, die linke Hand am Herz."), hip: H(0.84), pelvisDeg: -14, trunk: READY, stance: [0.26, 12, 0.0], racket: { grip: [0.3, 0.95, -0.05], dir: [0.45, 0.8, 0.3], normal: [0, 0, 1] }, left: "throat", met: 7.3, breathing: "rally", wet: 0.2 },
  { id: "forehand-backswing", title: tx("Forehand backswing", "Vorhand-Ausholbewegung"), description: tx("Unit turn of an open-stance forehand: shoulders turned about 40 degrees to the right, the racket head up behind the right shoulder, the left arm reaching towards the ball.", "Oberkörperdrehung einer Vorhand im offenen Stand: Schultern etwa 40 Grad nach rechts gedreht, der Schlägerkopf oben hinter der rechten Schulter, der linke Arm zeigt zum Ball."), hip: H(0.82), pelvisDeg: -12, trunk: { torso: { angleDeg: -5, twistDeg: -12 }, chest: { angleDeg: -3, twistDeg: -28 }, neck: { angleDeg: 8, twistDeg: 24 }, head: { angleDeg: 8, twistDeg: 12 } }, stance: [0.26, 12, 0.0], racket: { grip: [-0.2, 1.12, -0.42], dir: [-0.25, 0.9, -0.2], normal: [0.2, 0.0, -1] }, left: { at: [0.42, 1.28, -0.12] }, met: 8, breathing: "rally", wet: 0.2 },
  { id: "forehand-contact", title: tx("Forehand contact", "Vorhand-Treffpunkt"), description: tx("Ball contact in front of the body at hip height: the shoulders square to the net, the hitting arm extended to the right, the string bed facing forward.", "Balltreffpunkt vor dem Körper auf Hüfthöhe: Schultern parallel zum Netz, der Schlagarm nach rechts gestreckt, die Besaitung zeigt nach vorn."), hip: H(0.82), pelvisDeg: -12, trunk: { torso: { angleDeg: -6, twistDeg: 2 }, chest: { angleDeg: -4, twistDeg: 6 }, neck: { angleDeg: 12, twistDeg: -6 }, head: { angleDeg: 12, twistDeg: -8 } }, stance: [0.26, 12, 0.0], racket: { grip: [0.32, 0.92, -0.5], dir: [0.1, 0.05, -1], normal: [1, 0.05, 0] }, left: { at: [0.22, 1.12, 0.22] }, met: 8, breathing: "rally", wet: 0.2 },
  { id: "follow-through", title: tx("Forehand follow-through", "Vorhand-Ausschwung"), description: tx("Finish of the forehand: shoulders turned about 45 degrees to the left, the racket over the left shoulder, the left hand catching it at the throat.", "Ende der Vorhand: Schultern etwa 45 Grad nach links gedreht, der Schläger über der linken Schulter, die linke Hand fängt ihn am Herz."), hip: H(0.84), pelvisDeg: -10, trunk: { torso: { angleDeg: -4, twistDeg: 12 }, chest: { angleDeg: -2, twistDeg: 32 }, neck: { angleDeg: 8, twistDeg: -26 }, head: { angleDeg: 8, twistDeg: -14 } }, stance: [0.26, 12, 0.0], racket: { grip: [0.1, 1.42, 0.28], dir: [-0.75, 0.35, 0.35], normal: [0.2, 0.9, -0.3] }, left: "throat", met: 8, breathing: "rally", wet: 0.2 },
  { id: "standing-between-points", title: tx("Standing between points", "Stehen zwischen den Ballwechseln"), description: tx("Upright and relaxed between two points, the racket hanging from the right hand with its head forward and down.", "Aufrecht und locker zwischen zwei Ballwechseln, der Schläger hängt in der rechten Hand, der Kopf zeigt nach vorn unten."), hip: H(0.955), pelvisDeg: 0, trunk: {}, stance: [0.11, 6, 0.0], racket: { grip: [0.03, 0.7, -0.2], dir: [0.55, -0.83, 0], normal: [0, 0, 1] }, left: { at: [0.02, 0.72, 0.2] }, met: 1.8, breathing: "rest", wet: 0.1 },
];
const wetted = (w: number) => Object.fromEntries(["head-skin", "arm-skin", "leg-skin", "hand-skin"].map((id) => [id, { wettedness: w }]));
const poses: Json[] = [];
for (const d of designs) {
  const { offsets, report } = designPose(d);
  console.log(report);
  poses.push({ id: d.id, title: d.title, description: d.description, bones: offsets, root: { x: 0, angleDeg: d.pelvisDeg, y: d.hip }, ground: "auto", contacts: FEET, thermal: binding(a, d.met, d.breathing, d.met > 2 ? 1 : 0.6, { ...wetted(d.wet), shoes: { contact: { fraction: 0.52, tempC: 22 } } }), loads: [] });
}
//#endregion

//#region 🫁️Respiration
a.thermal.respiration.patterns = [
  { id: "rest", title: tx("Calm breathing between points", "Ruhige Atmung zwischen den Ballwechseln"), rateBpm: 16, tidalVolumeL: 0.65, inhaleFraction: 0.4, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.46, pauseAfterExhaleFraction: 0.1, route: "nose", source: "authored-estimate", reference: "gupta-2010" },
  { id: "rally", title: tx("Rally breathing through nose and mouth", "Atmung im Ballwechsel durch Nase und Mund"), rateBpm: 30, tidalVolumeL: 1.4, inhaleFraction: 0.44, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.04, route: "nose-and-mouth", source: "authored-estimate", reference: "astrand-rodahl-2003" },
];
//#endregion

//#region 🎞️Clips
const ALL_FEET = (d: number) => FEET.map((anchor) => ({ anchor, fromS: 0, toS: d }));
const SWING = 2.4;
const clips: Json[] = [
  {
    id: "forehand-swing", title: tx("Open-stance forehand", "Vorhand im offenen Stand"),
    description: tx("A looping topspin forehand from the ready stance: unit turn with the racket up behind the shoulder, a fast low-to-high forward swing to contact in front of the hip, a finish over the left shoulder and the recovery to the ready stance. The feet stay planted (open stance).", "Eine wiederholte Topspin-Vorhand aus der Bereitschaftsstellung: Oberkörperdrehung mit dem Schläger hinter der Schulter, ein schneller Vorschwung von unten nach oben zum Treffpunkt vor der Hüfte, ein Ausschwung über die linke Schulter und die Rückkehr in die Bereitschaftsstellung. Die Füsse bleiben stehen (offener Stand)."),
    durationS: SWING, fps: 30, loop: true, rootMotion: "plant", advanceM: 0,
    keys: [
      { t: 0, pose: "ready-stance", ease: "smooth" }, { t: 0.62, pose: "forehand-backswing", ease: "smooth" }, { t: 0.92, pose: "forehand-backswing", ease: "in" },
      { t: 1.1, pose: "forehand-contact", ease: "out" }, { t: 1.36, pose: "follow-through", ease: "smooth" }, { t: 1.75, pose: "follow-through", ease: "smooth" },
    ],
    contacts: ALL_FEET(SWING),
    phases: [
      { id: "preparation", fromS: 0, toS: 0.62, title: tx("Unit turn", "Oberkörperdrehung") },
      { id: "loop", fromS: 0.62, toS: 0.92, title: tx("Racket loop", "Schlägerschleife") },
      { id: "forward-swing", fromS: 0.92, toS: 1.1, title: tx("Forward swing", "Vorschwung") },
      { id: "follow-through", fromS: 1.1, toS: 1.75, title: tx("Follow-through", "Ausschwung") },
      { id: "recovery", fromS: 1.75, toS: SWING, title: tx("Recovery", "Rückkehr") },
    ],
    events: [
      { t: 0.62, kind: "motion", title: tx("Shoulders fully turned", "Schultern ganz gedreht") },
      { t: 1.1, kind: "motion", title: tx("Ball contact", "Balltreffpunkt") },
      { t: 1.1, kind: "thermal", title: tx("Forced exhalation at contact", "Kräftiges Ausatmen beim Treffpunkt") },
    ],
    tracks: [
      { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 7.3 }, { t: 0.62, v: 8 }, { t: 1.75, v: 8 }] },
      { target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }] },
      { target: "emitter:plume", ease: "smooth", keys: [{ t: 0, v: 1 }] },
    ],
  },
  {
    id: "idle-breathing", title: tx("Idle breathing between points", "Ruhiges Atmen zwischen den Ballwechseln"),
    description: tx("Standing between two points with the racket hanging from the right hand; one calm breath through the nose every 3.75 s moves the chest and the belly.", "Stehen zwischen zwei Ballwechseln, der Schläger hängt in der rechten Hand; alle 3,75 s ein ruhiger Atemzug durch die Nase hebt Brust und Bauch."),
    durationS: 3.75, fps: 30, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "standing-between-points", ease: "smooth" }, { t: 1.875, pose: "standing-between-points", ease: "smooth", bones: { head: { angleDeg: -2 } } }],
    contacts: ALL_FEET(3.75),
    phases: [{ id: "breath", fromS: 0, toS: 3.75, title: tx("One calm breath", "Ein ruhiger Atemzug") }],
    events: [],
    tracks: [{ target: "activityMet", ease: "step", keys: [{ t: 0, v: 1.8 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }] }],
  },
];
//#endregion

//#region 🧾️Identity, provenance and texts
a.title = tx("Tennis Player", "Tennisspielende Person");
a.summary = tx(
  "A right-handed club player of 1.74 m and 66 kg (athletic build, sex-neutral) in a white polo, tennis shorts, sweatband and tennis shoes with a 68.6 cm midplus racket: ready stance, open-stance forehand from the unit turn over contact to the finish, a looping forehand at 7.3 to 8 met and idle breathing between points; four individually drawn scales from 1:500 to 1:50.",
  "Eine rechtshändige Clubspielerin oder ein Clubspieler von 1,74 m und 66 kg (sportlicher Körperbau, geschlechtsneutral) in weissem Polo, Tennisshorts, Schweissband und Tennisschuhen mit einem 68,6 cm langen Midplus-Schläger: Bereitschaftsstellung, Vorhand im offenen Stand von der Oberkörperdrehung über den Treffpunkt bis zum Ausschwung, eine wiederholte Vorhand bei 7,3 bis 8 met und ruhiges Atmen zwischen den Ballwechseln; vier einzeln gezeichnete Massstäbe von 1:500 bis 1:50.");
a.tags = ["human", "sport", "tennis", "racket", "ball-sport", "athlete"];
a.provenance = {
  author: "Semio staffage human worker HW-07", method: "handcrafted", created: "2026-11-22",
  references: [
    ...a.provenance.references.filter((x: Json) => !["cavanagh-1980", "dalleau-1998", "bramble-carrier-1983"].includes(x.id)),
    { id: "din-33402-2-2020", citation: "DIN 33402-2:2020-12 Ergonomie, Körpermasse des Menschen, Teil 2: Werte (stature, segment lengths and breadths of adults)." },
    { id: "itf-rules-2024", citation: "International Tennis Federation (2024). ITF Rules of Tennis, Appendix II: The Racket (overall length at most 73.7 cm, strung surface at most 39.4 x 29.2 cm).", url: "https://www.itftennis.com/en/about-us/governance/rules-and-regulations/" },
    { id: "elliott-2006", citation: "Elliott, B. (2006). Biomechanics and tennis. British Journal of Sports Medicine 40(5), 392-396 (open-stance forehand, trunk rotation, racket path)." },
    { id: "kovacs-2007", citation: "Kovacs, M. S. (2007). Tennis physiology: training the competitive athlete. Sports Medicine 37(3), 189-198 (rally and rest durations, heart rate and ventilation in match play)." },
  ],
  notes: tx(
    "Derived by HW-07 from the catalog jogger: rig, outlines and volumes rescaled to a stature of 1.74 m (factor 0.956), segment masses re-weighted to 66 kg with the de Leva fractions, the kit re-dressed in white, the armband removed, and the racket, poses, clips and physiology authored anew. Generator: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-07/tennis-player.ts.",
    "Von HW-07 aus der Katalogfigur Jogger abgeleitet: Rig, Umrisse und Volumen auf eine Körperhöhe von 1,74 m skaliert (Faktor 0,956), Segmentmassen mit den Anteilen nach de Leva auf 66 kg gewichtet, die Kleidung weiss neu eingekleidet, das Armband entfernt und Schläger, Posen, Clips und Physiologie neu verfasst. Generator: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-07/tennis-player.ts."),
};
const kit: Kit = { top: tx("white polo", "weissen Polo"), shorts: tx("tennis shorts", "Tennisshorts"), feet: tx("tennis shoes over ankle socks", "Tennisschuhe über Knöchelsocken"), band: tx("sweatband", "Schweissband") };
const racketText: Record<string, PropText> = {
  "50": { silhouette: tx("The racket is drawn in its parts: frame oval with the strung bed (cross-hatched), open throat of two arms, shaft, grip tape and butt cap.", "Der Schläger ist in seinen Teilen gezeichnet: Rahmenoval mit der Besaitung (kreuzschraffiert), offenes Herz aus zwei Armen, Schaft, Griffband und Endkappe."), hatching: tx("The string bed is a 0.45 mm cross hatch (typified: real string spacing about 13 to 17 mm).", "Die Besaitung ist eine 0,45-mm-Kreuzschraffur (typisiert: reale Saitenabstände etwa 13 bis 17 mm)."), thinElements: tx("The 13 mm frame beam is 0.26 mm on paper and stays true.", "Der 13 mm breite Rahmen ist 0,26 mm auf dem Papier und bleibt massstäblich."), overlays: tx("All racket parts carry the passive region of the racket.", "Alle Schlägerteile tragen die passive Region des Schlägers.") },
  "100": { silhouette: tx("Racket: the head as one strung oval whose outline is the frame, a closed throat wedge and the grip.", "Schläger: der Kopf als ein besaitetes Oval, dessen Umriss der Rahmen ist, ein geschlossener Herzkeil und der Griff."), merged: tx("The open throat is closed (its arms would be 0.13 mm wide).", "Das offene Herz ist geschlossen (seine Arme wären 0,13 mm breit)."), hatching: tx("The string bed keeps a 0.5 mm cross hatch.", "Die Besaitung behält eine 0,5-mm-Kreuzschraffur.") },
  "200": { silhouette: tx("Racket: one paddle form of head, throat and grip in a mid tone.", "Schläger: eine Paddelform aus Kopf, Herz und Griff in mittlerem Ton."), thinElements: tx("The 31 mm grip is widened to 40 mm (0.2 mm of paper) inside the hand.", "Der 31 mm breite Griff ist in der Hand auf 40 mm (0,2 mm Papiermass) verbreitert.") },
  "500": { silhouette: tx("Racket: one dark paddle form that still shows the oval head at 0.7 x 0.5 mm; seen edge-on (front and plan) it is drawn 32 mm thick so that outline and fill reach the 0.2 mm feature.", "Schläger: eine dunkle Paddelform, die den ovalen Kopf mit 0,7 x 0,5 mm noch zeigt; von der Kante gesehen (Vorderansicht und Grundriss) ist sie 32 mm dick gezeichnet, damit Umriss und Fläche das Merkmal von 0,2 mm erreichen."), thinElements: tx("Throat and grip are one tail of 0.124 m (0.25 mm of paper) into the hand.", "Herz und Griff sind ein Schwanz von 0,124 m (0,25 mm Papiermass) in die Hand.") },
};
for (const k of ["50", "100", "200", "500"]) a.representations[k].decisions = decisions(k, kit, racketText[k]!);
const racketVol: Record<string, T2> = {
  "50": tx("The racket is a frame ring extruded 23 mm deep around a 2 mm string sheet, two throat arms, a shaft, the grip and the butt.", "Der Schläger ist ein 23 mm tief extrudierter Rahmenring um eine 2 mm starke Saitenfläche, zwei Herzarme, ein Schaft, der Griff und die Endkappe."),
  "100": tx("The racket is the frame ring around the string sheet, a closed throat and the grip.", "Der Schläger ist der Rahmenring um die Saitenfläche, ein geschlossenes Herz und der Griff."),
  "200": tx("The racket is one 10 mm paddle sheet of head, throat and grip (the 23 mm frame is below the 30 mm feature).", "Der Schläger ist eine 10 mm starke Paddelfläche aus Kopf, Herz und Griff (der 23-mm-Rahmen liegt unter dem 30-mm-Merkmal)."),
  "500": tx("The racket is one 10 mm paddle sheet with a 0.124 m tail into the hand.", "Der Schläger ist eine 10 mm starke Paddelfläche mit einem 0,124 m breiten Schwanz in die Hand."),
};
const who = tx("a 1.74 m athletic tennis player", "eine 1,74 m grosse, sportliche Tennisperson");
for (const k of ["50", "100", "200", "500"]) a.volumes[k].decisions = volumeDecisions(k, who, { body: racketVol[k], thermal: tx("The racket is one passive region over all its solids.", "Der Schläger ist eine passive Region über alle seine Körper."), structure: tx("The racket is one slender member from the butt to the tip on the fixed grip.", "Der Schläger ist ein schlankes Glied vom Griffende zur Spitze am festen Griff.") });
a.thermal.assumptions = [
  tx(`DuBois skin area for 1.74 m and 66 kg (${r(duBois(BODY_KG, STATURE), 2)} m2); a white polyester polo (0.09 clo) and shorts (0.05 clo) after ISO 9920, layer resistances including the enclosed air; the white kit absorbs 25 % of the sun.`, `DuBois-Hautfläche für 1,74 m und 66 kg (${String(r(duBois(BODY_KG, STATURE), 2)).replace(".", ",")} m2); ein weisses Polyester-Polo (0,09 clo) und Shorts (0,05 clo) nach ISO 9920, Schichtwiderstände mit eingeschlossener Luft; die weisse Kleidung absorbiert 25 % der Sonne.`),
  tx("Metabolic rate 7.3 met in the ready stance (Compendium 15675, tennis, general), 8 met through the forehand (15685, singles), 1.8 met standing between points; skin wettedness 0.2 in play. Skin blood flow limits 6.3 to 90 L/(m2 h), set points 36.8/34.1 degrees Celsius after the Gagge two-node model; basal metabolism the mean of the Mifflin-St Jeor equations for both sexes.", "Stoffwechsel 7,3 met in der Bereitschaftsstellung (Compendium 15675, Tennis allgemein), 8 met in der Vorhand (15685, Einzel), 1,8 met im Stehen zwischen den Ballwechseln; Hautbenetzung 0,2 im Spiel. Hautdurchblutung 6,3 bis 90 L/(m2 h), Sollwerte 36,8/34,1 Grad Celsius nach dem Zwei-Knoten-Modell von Gagge; Grundumsatz als Mittel der Mifflin-St-Jeor-Gleichungen beider Geschlechter."),
  tx("Rally breathing 30 breaths per minute with 1.4 L (42 L/min, about 25 L per litre of oxygen at 8 met), calm breathing 16 per minute with 0.65 L through the nose; exhaled air 34 degrees Celsius, 95 % humidity. Authored estimates guided by the cited sources.", "Atmung im Ballwechsel 30 Atemzüge pro Minute mit 1,4 L (42 L/min, etwa 25 L je Liter Sauerstoff bei 8 met), ruhige Atmung 16 pro Minute mit 0,65 L durch die Nase; Ausatemluft 34 Grad Celsius, 95 % Feuchte. Autorenschätzungen nach den genannten Quellen."),
  tx("The racket is a passive graphite surface (0.3 kg, about 1.6 kJ/(m2 K)); contact conduction through the soles uses 26 % of the shoe area per planted foot at a 22 degrees Celsius court.", "Der Schläger ist eine passive Graphitoberfläche (0,3 kg, etwa 1,6 kJ/(m2 K)); die Wärmeleitung durch die Sohlen nutzt 26 % der Schuhfläche je aufgesetztem Fuss bei 22 Grad Celsius Platzbelag."),
];
a.structure = {
  gravityMS2: 9.80665, dynamics: "quasi-static", supports: [],
  assumptions: [
    tx("Quasi-static model: segment masses as point masses at the de Leva centres, both feet planted on four contact anchors; the racket (0.3 kg, balance 33 cm from the butt) hangs rigidly on the right hand. Swing inertia is not included: the forehand is a sequence of static equilibria.", "Quasistatisches Modell: Segmentmassen als Punktmassen in den Schwerpunkten nach de Leva, beide Füsse stehen auf vier Kontaktankern; der Schläger (0,3 kg, Balancepunkt 33 cm vom Griffende) hängt starr an der rechten Hand. Die Trägheit des Schwungs ist nicht enthalten: die Vorhand ist eine Folge statischer Gleichgewichte."),
    tx("Joint loads are rigid-segment resultants, not clinical bone or muscle forces.", "Gelenklasten sind Resultierende starrer Segmente, keine klinischen Knochen- oder Muskelkräfte."),
  ],
  clinical: false,
};
a.symbols = a.symbols.map((sym: Json) => ({ ...sym, id: "player-position-mark", title: tx("Player position and facing mark", "Positions- und Blickrichtungsmarke der Spielperson"), description: tx("A small circle with a facing tick marking where a player stands and which way they face, for court plans with many people.", "Ein kleiner Kreis mit Richtungsstrich, der Standort und Blickrichtung einer Spielperson in Platzplänen mit vielen Personen markiert."), dimensionalNote: tx("Only the insertion point is true (the hip position); circle and tick are fixed paper sizes of 1 mm and 1.1 mm and carry no dimension.", "Nur der Einfügepunkt ist massgebend (Hüftposition); Kreis und Strich haben feste Papiergrössen von 1 mm und 1,1 mm und tragen keine Masse.") }));
//#endregion

a.poses = poses;
a.clips = clips;
removeParts(a, ["armband", "armband-pouch"]);
solidify(a, SPREAD);
a.presentation = { defaultPose: "ready-stance", defaultClip: "forehand-swing", defaultScale: 100 };
a.completion = "authored";
a.thermal.body = { ...a.thermal.body, massKg: BODY_KG, skinAreaM2: r(duBois(BODY_KG, STATURE), 3), basalMetabolismW: r((mifflin(BODY_KG, STATURE, AGE, "male") + mifflin(BODY_KG, STATURE, AGE, "female")) / 2, 1), source: "authored-estimate", reference: "gagge-1971" };
console.log("areas", calibrateAreas(a));
const dims = measureDimensions(a);
console.log("dims", JSON.stringify(a.dimensions), JSON.stringify(dims.byScale));
if (!process.argv.includes("--no-save")) {
  console.log(await saveHuman(doc));
  console.log(JSON.stringify(await writeManifestRow(a, tx("A racket sport figure: the forehand turns the shoulders out of the side plane and swings a 0.3 kg racket from behind the body to over the opposite shoulder, while both feet stay planted; no other person in the catalog holds a striking implement.", "Eine Rückschlagsportfigur: die Vorhand dreht die Schultern aus der Seitenebene und schwingt einen 0,3 kg schweren Schläger von hinter dem Körper bis über die Gegenschulter, während beide Füsse stehen bleiben; keine andere Person im Katalog hält ein Schlaggerät."))));
}
