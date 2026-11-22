/** 🧘️ Generator of `person-stretching`: derived from the rig, outlines and volumes of `standing-adult`, redressed in sportswear (hooded training top, track pants with a side stripe,
 * running shoes) and given the three classic post-run stretches as named poses (standing quadriceps stretch, side bend, hamstring stretch) and a looping stretch routine; the trunk, head and thighs
 * get prism solids on their side-elevation and plan shapes so that the side bend and the balancing stance stay exact in every view.
 * bun stretch.ts
 * @see ../../📋️brief.md */
import { loadDoc, saveDoc, pruneUnused, solidify, fitDimensions, region, t, r, SCALES, VIEWS, type J } from "./lib.ts";
import { solve, stateOf } from "./ik.ts";

const ID = "person-stretching";
const doc = loadDoc("standing-adult");
const a = doc.asset;
a.id = ID;

//#region clothing: sportswear
const deep = (x: J, f: (o: J) => void): void => { if (Array.isArray(x)) x.forEach((y) => deep(y, f)); else if (x && typeof x === "object") { f(x); for (const k of Object.keys(x)) deep(x[k], f); } };
const renameRegion = (from: string, to: string) => deep(a, (o) => { if (o.region === from) o.region = to; if (Array.isArray(o.regions)) o.regions = o.regions.map((x: string) => (x === from ? to : x)); if (o.regions && !Array.isArray(o.regions) && o.regions[from]) { o.regions[to] = o.regions[from]; delete o.regions[from]; } if (o.id === from && o.kind && o.model) o.id = to; });
const renameMaterial = (from: string, to: string) => deep(a, (o) => { if (o.material === from) o.material = to; if (o.id === from && o.emissivity !== undefined) o.id = to; });
renameRegion("jeans", "track-pants"); renameRegion("sneakers", "running-shoes");
renameMaterial("fleece-sweatshirt", "training-fleece"); renameMaterial("denim", "track-jersey"); renameMaterial("canvas-sneaker", "running-shoe");
const mat = (id: string) => a.materials.find((m: J) => m.id === id);
Object.assign(mat("training-fleece"), { title: t("Polyester training fleece", "Trainingsfleece aus Polyester"), solarAbsorptivity: 0.7 });
Object.assign(mat("track-jersey"), { title: t("Polyester-elastane track pant jersey", "Trainingshosen-Jersey aus Polyester-Elasthan"), emissivity: 0.9, solarAbsorptivity: 0.8 });
Object.assign(mat("running-shoe"), { title: t("Running shoe with engineered mesh upper and EVA midsole", "Laufschuh mit Mesh-Obermaterial und EVA-Zwischensohle"), emissivity: 0.9, solarAbsorptivity: 0.45 });
Object.assign(region(a, "hoodie"), { title: t("T-shirt and hooded training top on trunk and arms", "T-Shirt und Trainings-Kapuzenpulli an Rumpf und Armen") });
Object.assign(region(a, "track-pants"), { title: t("Track pants over the legs", "Trainingshose über den Beinen") });
Object.assign(region(a, "running-shoes"), { title: t("Socks and running shoes", "Socken und Laufschuhe") });
const layer = (rg: string, id: string, patch: J) => Object.assign(region(a, rg).layers.find((l: J) => l.id === id), patch);
layer("hoodie", "sweatshirt", { id: "training-top", title: t("Hooded polyester training top", "Kapuzen-Trainingsoberteil aus Polyester") });
layer("track-pants", "jeans", { id: "track-pants", title: t("Jersey track pants", "Trainingshose aus Jersey"), resistanceM2KW: 0.0372, vapourResistanceM2PaW: 5.9 });
layer("running-shoes", "sneaker", { id: "running-shoe", title: t("Mesh running shoe", "Laufschuh aus Mesh"), resistanceM2KW: 0.0047, vapourResistanceM2PaW: 0.75 });
for (const k of SCALES) for (const x of a.volumes[k].thermal) { if (x.layer === "sweatshirt") x.layer = "training-top"; if (x.layer === "jeans") x.layer = "track-pants"; if (x.layer === "sneaker") x.layer = "running-shoe"; }
for (const k of SCALES) {
  const rep = a.representations[k]; if (rep.status !== "authored") continue;
  for (const v of VIEWS) for (const s of rep.geometry[v]) {
    if (s.hatch === "denim") delete s.hatch;
    if (s.region === "track-pants" && s.role === "body") s.tone = 0.28;
    if (/jeans-side-seam|jeans-seam/.test(s.id)) Object.assign(s, { id: s.id.replace(/jeans-(side-)?seam/, "pant-stripe"), tone: 0.9 });
    if (s.id.startsWith("jeans-")) s.id = s.id.replace("jeans-", "pant-");
  }
  rep.hatches = rep.hatches.filter((h: J) => h.id !== "denim");
}
for (const k of SCALES) for (const s of a.volumes[k].body) if (s.material === "track-jersey") s.tone = 0.28;
//#endregion

//#region prism solids for the bones that bend sideways
solidify(a, new Set(["torso", "neck", "head", "thigh-l", "shank-l", "foot-l", "toe-l"]), SCALES, VIEWS, { torso: 0.8, neck: 0.8, head: 0.8 });
solidify(a, new Set(["thigh-r"]), SCALES, ["elevation", "plan"]);
//#endregion

//#region poses
const FEET = ["heel-l", "ball-l", "heel-r", "ball-r"];
const REGIONS = ["face-skin", "hair", "hand-skin", "hoodie", "track-pants", "running-shoes"];
const SUN: J = { "face-skin": 0.5, hair: 0.75, "hand-skin": 0.5, hoodie: 0.45, "track-pants": 0.4, "running-shoes": 0.3 };
const binding = (met: number, breathing: string, exposed: J = {}, feet = 0.3) => ({
  activityMet: met, breathing,
  regions: Object.fromEntries(REGIONS.map((id) => [id, { exposedFraction: exposed[id] ?? 1, sunlitFraction: SUN[id], ...(id === "running-shoes" ? { contact: { fraction: feet, tempC: 13 } } : {}) }])),
  emitters: { "body-plume": met > 2 ? 0.75 : 0.6 }, operating: {},
});
const relaxed = structuredClone(a.poses.find((p: J) => p.id === "relaxed"));
const hands = Object.fromEntries(Object.entries(relaxed.bones).filter(([k]) => /thumb|index|middle|ring|little/.test(k)));
const mkPose = (id: string, title: J, description: J, bones: J, contacts: string[], thermal: J, root?: J) => ({ id, title, description, bones, ...(root ? { root } : {}), ground: "auto", contacts, thermal, loads: [] });
const check = (label: string, sol: { error: number }) => { if (sol.error > 2e-3) console.log(`[DEBUG] ${label} IK error ${sol.error.toFixed(4)}`); };

const readyBones = { ...relaxed.bones };

const handsOpenL = Object.fromEntries(Object.entries(hands).filter(([k]) => !k.endsWith("-l")));
let quad: J = { ...handsOpenL, "thigh-l": { angleDeg: -2.5, abductionDeg: -9 }, "shank-l": { angleDeg: -2.5, abductionDeg: 9 }, "foot-l": { angleDeg: 5 }, "toe-l": { angleDeg: 0 },
  "thigh-r": { angleDeg: -9 }, "shank-r": { angleDeg: -138 }, "foot-r": { angleDeg: -38 }, "toe-r": { angleDeg: 0 },
  lumbar: { angleDeg: 2, abductionDeg: 3 }, torso: { angleDeg: 0 }, neck: { angleDeg: -2 }, head: { angleDeg: 1 },
  "arm-l": { angleDeg: 12, abductionDeg: 58 }, "forearm-l": { angleDeg: 14 }, "hand-l": { angleDeg: 0 },
  "arm-r": { angleDeg: -25, abductionDeg: -8 }, "forearm-r": { angleDeg: 20 }, "hand-r": { angleDeg: 10 } };
{
  const st = stateOf(a, quad, ["heel-l", "ball-l"]);
  const ankle = st.bones["foot-r"].frame.o;
  const sol = solve(a, quad, ["heel-l", "ball-l"], [{ bone: "arm-r", field: "angleDeg" }, { bone: "arm-r", field: "abductionDeg" }, { bone: "forearm-r", field: "angleDeg" }], [{ kind: "tip", bone: "hand-r", x: ankle.x + 0.035, y: ankle.y + 0.01, z: ankle.z - 0.01 }]);
  check("quad hand", sol); quad = sol.bones;
}

let side: J = { ...hands, "thigh-l": { angleDeg: -2.5 }, "shank-l": { angleDeg: -2.5 }, "foot-l": { angleDeg: 5 }, "toe-l": { angleDeg: 0 },
  "thigh-r": { angleDeg: -2.5 }, "shank-r": { angleDeg: -2.5 }, "foot-r": { angleDeg: 5 }, "toe-r": { angleDeg: 0 },
  lumbar: { angleDeg: 0, abductionDeg: 13 }, torso: { angleDeg: 0, abductionDeg: 11 }, neck: { angleDeg: 0, abductionDeg: 5 }, head: { angleDeg: 2 },
  "arm-r": { angleDeg: 168, abductionDeg: 18 }, "forearm-r": { angleDeg: 12 }, "hand-r": { angleDeg: 0 },
  "arm-l": { angleDeg: -12, abductionDeg: 30 }, "forearm-l": { angleDeg: 95 }, "hand-l": { angleDeg: -10 } };
{
  const st = stateOf(a, side, FEET);
  const hip = st.bones["thigh-l"].frame.o;
  const sol = solve(a, side, FEET, [{ bone: "arm-l", field: "angleDeg" }, { bone: "arm-l", field: "abductionDeg" }, { bone: "forearm-l", field: "angleDeg" }], [{ kind: "tip", bone: "forearm-l", x: hip.x + 0.02, y: hip.y + 0.06, z: hip.z + 0.12 }]);
  check("side hand on hip", sol); side = sol.bones;
}

let ham: J = { ...hands, "thigh-r": { angleDeg: 62 }, "shank-r": { angleDeg: 0 }, "foot-r": { angleDeg: 22 }, "toe-r": { angleDeg: 0 },
  "thigh-l": { angleDeg: 46 }, "shank-l": { angleDeg: -28 }, "foot-l": { angleDeg: 18 }, "toe-l": { angleDeg: 0 },
  lumbar: { angleDeg: -20 }, torso: { angleDeg: -10 }, neck: { angleDeg: 24 }, head: { angleDeg: 16 },
  "arm-l": { angleDeg: 40, abductionDeg: -8 }, "forearm-l": { angleDeg: 10 }, "hand-l": { angleDeg: 0 },
  "arm-r": { angleDeg: 40, abductionDeg: 8 }, "forearm-r": { angleDeg: 10 }, "hand-r": { angleDeg: 0 } };
const HAM_ROOT = { x: 0, y: 0, angleDeg: -38 };
const HAM_CONTACTS = ["heel-r", "heel-l", "ball-l"];
{
  let sol = solve(a, ham, HAM_CONTACTS, [{ bone: "thigh-r", field: "angleDeg" }, { bone: "thigh-l", field: "angleDeg" }, { bone: "shank-l", field: "angleDeg" }, { bone: "foot-l", field: "angleDeg" }], [{ kind: "anchor", anchor: "heel-r", y: 0 }, { kind: "anchor", anchor: "heel-l", y: 0 }, { kind: "anchor", anchor: "ball-l", y: 0 }], 40, HAM_ROOT);
  check("hamstring legs", sol); ham = sol.bones;
  const st = stateOf(a, ham, HAM_CONTACTS, HAM_ROOT);
  const knee = st.bones["shank-r"].frame.o, hipR = st.bones["thigh-r"].frame.o;
  const on = (f: number, dz: number) => ({ x: hipR.x + (knee.x - hipR.x) * f, y: hipR.y + (knee.y - hipR.y) * f + 0.07, z: hipR.z + (knee.z - hipR.z) * f + dz });
  const pr = on(0.62, -0.055), pl = on(0.58, 0.06);
  sol = solve(a, ham, HAM_CONTACTS, [{ bone: "arm-r", field: "angleDeg" }, { bone: "arm-r", field: "abductionDeg" }, { bone: "forearm-r", field: "angleDeg" }], [{ kind: "tip", bone: "forearm-r", ...pr }], 40, HAM_ROOT);
  check("hamstring right hand", sol); ham = sol.bones;
  sol = solve(a, ham, HAM_CONTACTS, [{ bone: "arm-l", field: "angleDeg" }, { bone: "arm-l", field: "abductionDeg" }, { bone: "forearm-l", field: "angleDeg" }], [{ kind: "tip", bone: "forearm-l", ...pl }], 40, HAM_ROOT);
  check("hamstring left hand", sol); ham = sol.bones;
}
a.poses = [
  mkPose("quad-stretch", t("Standing quadriceps stretch", "Stehende Oberschenkeldehnung"), t("Balancing on the left leg, the right heel drawn to the buttock with the right hand at the ankle, knees side by side, the left arm out to the side for balance; the stance leg slants a little inwards so that the centre of mass stays over the left foot.", "Auf dem linken Bein balancierend, die rechte Ferse mit der rechten Hand am Knöchel zum Gesäß gezogen, Knie nebeneinander, der linke Arm zur Seite gestreckt für das Gleichgewicht; das Standbein steht leicht nach innen geneigt, damit der Schwerpunkt über dem linken Fuß bleibt."), quad, ["heel-l", "ball-l"], binding(2.3, "stretching", { "hand-skin": 0.85, "track-pants": 0.9 }, 0.15)),
  mkPose("side-bend", t("Standing side bend to the left", "Seitbeuge nach links im Stand"), t("Feet a little wider than the hips, the right arm reaching over the head, the left hand on the hip, the trunk bending to the left from the lumbar spine (about 25 degrees together with the chest), the head following.", "Füße etwas weiter als hüftbreit, der rechte Arm greift über den Kopf, die linke Hand an der Hüfte, der Rumpf beugt sich aus der Lendenwirbelsäule nach links (mit dem Brustkorb etwa 25 Grad), der Kopf folgt."), side, FEET, binding(2.3, "stretching", { "hand-skin": 0.9 })),
  mkPose("hamstring-stretch", t("Hamstring stretch with the heel forward", "Dehnung der hinteren Oberschenkelmuskeln mit vorgestellter Ferse"), t("The right leg straight ahead on its heel with the toes pulled up, the left knee bent, the trunk hinged forward from the hips with a long back, both hands resting on the right thigh.", "Das rechte Bein gestreckt vorn auf der Ferse, die Zehen angezogen, das linke Knie gebeugt, der Rumpf mit langem Rücken aus der Hüfte nach vorn geneigt, beide Hände auf dem rechten Oberschenkel."), ham, HAM_CONTACTS, binding(2.3, "stretching", { "hand-skin": 0.7, "track-pants": 0.85 }, 0.2), HAM_ROOT),
  mkPose("ready", t("Standing between stretches", "Stehen zwischen den Dehnungen"), t("Upright and relaxed on both feet after the run, arms hanging, breathing calming down.", "Aufrecht und locker auf beiden Füßen nach dem Lauf, die Arme hängen, die Atmung beruhigt sich."), readyBones, FEET, binding(1.4, "recovering")),
];
//#endregion

//#region clips
const contactSpans = (spans: [string, number, number][]) => spans.map(([anchor, fromS, toS]) => ({ anchor, fromS, toS }));
const R = 19;
const routine: J = {
  id: "stretch-routine", title: t("Stretch routine after a run", "Dehnprogramm nach dem Laufen"),
  description: t("A loop of the three classic stretches with short pauses: right quadriceps stretch held for about four seconds, side bend to the left, hamstring stretch of the right leg, each entered and left slowly from the upright stance; breathing slow and deep during the holds.", "Eine Schleife der drei klassischen Dehnungen mit kurzen Pausen: Dehnung des rechten Oberschenkels etwa vier Sekunden gehalten, Seitbeuge nach links, Dehnung der hinteren Muskeln des rechten Beins, jede langsam aus dem aufrechten Stand eingenommen und wieder gelöst; die Atmung ist in den Haltephasen langsam und tief."),
  durationS: R, fps: 30, loop: true, rootMotion: "none",
  keys: [
    { t: 0, pose: "ready", ease: "smooth" }, { t: 1.6, pose: "quad-stretch", ease: "smooth" }, { t: 5.4, pose: "quad-stretch", ease: "smooth" }, { t: 7, pose: "ready", ease: "smooth" },
    { t: 8.6, pose: "side-bend", ease: "smooth" }, { t: 11.4, pose: "side-bend", ease: "smooth" }, { t: 13, pose: "ready", ease: "smooth" },
    { t: 14.6, pose: "hamstring-stretch", ease: "smooth" }, { t: 17.4, pose: "hamstring-stretch", ease: "smooth" }, { t: R, pose: "ready", ease: "smooth" },
  ],
  contacts: contactSpans([["heel-l", 0, R], ["ball-l", 0, R], ["heel-r", 0, 0.9], ["ball-r", 0, 0.7], ["heel-r", 6.2, R], ["ball-r", 6.4, 13.8], ["ball-r", 18.2, R]]),
  phases: [{ id: "quad", fromS: 0, toS: 7, title: t("Quadriceps stretch", "Oberschenkeldehnung") }, { id: "side", fromS: 7, toS: 13, title: t("Side bend", "Seitbeuge") }, { id: "hamstring", fromS: 13, toS: R, title: t("Hamstring stretch", "Dehnung der Oberschenkelrückseite") }],
  events: [{ t: 0.7, kind: "contact-off", title: t("Right ball lifts", "Rechter Ballen hebt ab"), target: "ball-r" }, { t: 0.9, kind: "contact-off", title: t("Right heel lifts", "Rechte Ferse hebt ab"), target: "heel-r" }, { t: 6.2, kind: "contact-on", title: t("Right heel back down", "Rechte Ferse wieder unten"), target: "heel-r" }, { t: 6.4, kind: "contact-on", title: t("Right ball back down", "Rechter Ballen wieder unten"), target: "ball-r" }, { t: 13.8, kind: "contact-off", title: t("Right toes pulled up", "Rechte Zehen angezogen"), target: "ball-r" }, { t: 18.2, kind: "contact-on", title: t("Right foot flat again", "Rechter Fuß wieder flach"), target: "ball-r" }],
  tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.6 }, { t: 1.6, v: 2.3 }, { t: 9, v: 2.3 }, { t: 17.4, v: 2.3 }, { t: R, v: 1.6 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: 1.6, v: 2 }, { t: 5.4, v: 1 }, { t: 8.6, v: 2 }, { t: 11.4, v: 1 }, { t: 14.6, v: 2 }, { t: 17.4, v: 1 }, { t: R, v: 1 }] }, { target: "emitter:body-plume", ease: "smooth", keys: [{ t: 0, v: 0.7 }, { t: R, v: 0.7 }] }],
};
const idle = structuredClone(doc.asset.clips.find((c: J) => c.id === "idle-breathing"));
for (const k of idle.keys) k.pose = "ready";
Object.assign(idle, { title: t("Idle breathing after the run", "Ruhiges Atmen nach dem Lauf"), description: t("Standing between the stretches, the chest rising and falling as the breath settles, the arms swinging slightly.", "Stehen zwischen den Dehnungen, der Brustkorb hebt und senkt sich, während sich die Atmung beruhigt, die Arme schwingen leicht.") });
idle.tracks = [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: idle.durationS, v: 1 }] }];
a.clips = [routine, idle];
a.presentation = { defaultPose: "quad-stretch", defaultClip: "stretch-routine", defaultScale: 100 };
a.structure.supports = [];
for (const k of SCALES) a.volumes[k].structure = a.volumes[k].structure.filter((x: J) => x.part !== "support");
a.structure.assumptions = [
  t("Segments are point masses at the de Leva centres (female table).", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Frauentabelle)."),
  t("Contacts are heel and ball of each foot; in the quadriceps stretch only the left foot carries, its slanted stance keeps the centre of mass over the footprint; in the hamstring stretch the right foot rests on its heel only. The pull of the hand on the ankle is an internal force of the body and carries no load.", "Kontakte sind Ferse und Ballen jedes Fußes; in der Oberschenkeldehnung trägt nur der linke Fuß, sein schräg gestelltes Standbein hält den Schwerpunkt über der Aufstandsfläche; in der Dehnung der Oberschenkelrückseite ruht der rechte Fuß nur auf der Ferse. Der Zug der Hand am Knöchel ist eine innere Kraft des Körpers und keine Last."),
];
//#endregion

//#region thermal
const th = a.thermal;
th.environment = { airTempC: 13, meanRadiantTempC: 13, skyTempC: 0, groundTempC: 14, relativeHumidity: 0.7, windMS: 1.5, windDirectionDeg: 180, solarWM2: 350, diffuseFraction: 0.4, pressurePa: 101325 };
th.baseMet = 1.4;
Object.assign(th.body, { basalMetabolismW: 63.9, source: "authored-estimate", reference: "mifflin-1990" });
const quiet = th.respiration.patterns.find((p: J) => p.id === "quiet-standing");
th.respiration.patterns = [
  { ...quiet, id: "quiet-standing", title: t("Quiet standing", "Ruhiges Stehen") },
  { id: "recovering", title: t("Breath settling after the run", "Atmung beruhigt sich nach dem Lauf"), rateBpm: 18, tidalVolumeL: 0.9, inhaleFraction: 0.42, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.06, route: "nose-and-mouth", source: "authored-estimate", reference: "icrp-66-1994" },
  { id: "stretching", title: t("Slow deep breathing in a held stretch", "Langsames tiefes Atmen in der gehaltenen Dehnung"), rateBpm: 10, tidalVolumeL: 1.0, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.08, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
];
th.assumptions = [
  th.assumptions[0],
  t("Clothing is light outdoor sportswear of about 0.6 clo (T-shirt, hooded polyester training top, jersey track pants, socks and mesh running shoes) after the ISO 9920 tables (1 clo = 0.155 m2K/W); vapour resistance R/(0.0165 K/Pa x 0.38). The default surroundings are a cool spring morning in a park: 13 degrees C, light wind, some sun.", "Die Kleidung ist leichte Sportkleidung für draußen von etwa 0,6 clo (T-Shirt, Kapuzen-Trainingsoberteil aus Polyester, Jersey-Trainingshose, Socken und Mesh-Laufschuhe) nach den Tabellen von ISO 9920 (1 clo = 0,155 m2K/W); Dampfwiderstand R/(0,0165 K/Pa x 0,38). Die Standardumgebung ist ein kühler Frühlingsmorgen im Park: 13 Grad C, leichter Wind, etwas Sonne."),
  t("Stretching (mild) is 2.3 MET after the 2011 Compendium of Physical Activities (code 02101); standing between the stretches right after a run is set to 1.4 MET while the breath settles (ISO 8996 standing 1.2 MET plus recovery, authored). Basal metabolism 63.9 W from the Mifflin-St Jeor equation (woman, 31 years, 58 kg, 169 cm).", "Leichtes Dehnen sind 2,3 MET nach dem Compendium of Physical Activities 2011 (Code 02101); das Stehen zwischen den Dehnungen direkt nach dem Lauf wird mit 1,4 MET angesetzt, während sich die Atmung beruhigt (ISO 8996 Stehen 1,2 MET plus Erholung, geschätzt). Grundumsatz 63,9 W aus der Mifflin-St-Jeor-Gleichung (Frau, 31 Jahre, 58 kg, 169 cm)."),
  t("Breathing patterns are authored and only guided by adult values (ICRP 66): slow deep nasal breathing in the holds, faster mixed breathing while the breath settles.", "Die Atemmuster sind geschätzt und nur an Erwachsenenwerten (ICRP 66) orientiert: langsames, tiefes Nasenatmen in den Haltephasen, schnellere gemischte Atmung, während sich die Atmung beruhigt."),
];
a.provenance.references.push({ id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (code 02101 stretching, mild, 2.3 MET)." });
//#endregion

//#region texts
a.title = t("Person Stretching", "Person beim Dehnen");
a.summary = t("A 31-year-old runner of 1.69 m in a hooded training top, track pants with a light side stripe and mesh running shoes, cooling down after a run in the park: a standing quadriceps stretch balanced on one leg, a side bend with the arm over the head and a hamstring stretch with the heel forward, held in a looping stretch routine at 2.3 MET with slow, deep breathing.", "Eine 31-jährige Läuferin von 1,69 m in Kapuzen-Trainingsoberteil, Trainingshose mit hellem Seitenstreifen und Mesh-Laufschuhen beim Auslaufen im Park: eine stehende Oberschenkeldehnung auf einem Bein, eine Seitbeuge mit dem Arm über dem Kopf und eine Dehnung der Oberschenkelrückseite mit vorgestellter Ferse, gehalten in einem Dehnprogramm als Schleife bei 2,3 MET mit langsamer, tiefer Atmung.");
a.tags = ["human", "adult", "woman", "runner", "stretching", "cool-down", "sportswear", "outdoor", "sports-and-leisure", "balance", "fitness"];
a.provenance = { ...a.provenance, author: "Semio staffage human worker HW-08", created: "2026-11-22", notes: t("Rig, outlines and volumes derived by hand from the archive's standing-adult (1.69 m barefoot, 1.71 m shod, de Leva female fractions); redressed as sportswear (hooded training top, jersey track pants with a light side stripe instead of denim, mesh running shoes), trunk, neck, head and thighs given prism solids on their side-elevation and plan shapes so the side bend and the slanted stance stay exact in every view; the three stretches solved with the library's own evaluation (hand at the ankle, hand on the hip, hands on the thigh, both feet on the ground) and a looping routine authored around them. Generator: ticket STAFFAGE-HUMAN-COLLECTION-WAVE, scripts HW-08/stretch.ts.", "Skelett, Umrisse und Volumen von Hand aus dem standing-adult des Archivs abgeleitet (1,69 m barfuß, 1,71 m mit Schuhen, weibliche Anteile nach de Leva); als Sportkleidung neu eingekleidet (Kapuzen-Trainingsoberteil, Jersey-Trainingshose mit hellem Seitenstreifen statt Denim, Mesh-Laufschuhe), Rumpf, Hals, Kopf und Oberschenkel mit Prismenkörpern auf ihren Seiten- und Draufsichtformen, damit Seitbeuge und schräges Standbein in jeder Ansicht stimmen; die drei Dehnungen mit der bibliothekseigenen Auswertung gelöst (Hand am Knöchel, Hand an der Hüfte, Hände am Oberschenkel, beide Füße am Boden) und ein Dehnprogramm als Schleife darum gelegt. Generator: Ticket STAFFAGE-HUMAN-COLLECTION-WAVE, Skripte HW-08/stretch.ts.") };
const SW: [RegExp, string][] = [[/jeans seat/g, "track pants seat"], [/jeans/g, "track pants"], [/Jeans/g, "Track pants"], [/denim/g, "jersey"], [/sneakers/g, "running shoes"], [/sneaker/g, "running shoe"], [/sweatshirt/g, "training top"], [/Sweatshirt/g, "Training top"]];
const SWDE: [RegExp, string][] = [[/Jeansgesäß/g, "Hosengesäß"], [/Jeans/g, "Trainingshose"], [/Denim/g, "Jersey"], [/Sneakern/g, "Laufschuhen"], [/Sneaker/g, "Laufschuhe"], [/Kapuzensweatshirt/g, "Kapuzen-Trainingsoberteil"], [/Sweatshirts/g, "Trainingsoberteils"], [/Sweatshirt/g, "Trainingsoberteil"]];
const swap = (x: J): J => ({ en: SW.reduce((s, [p, q]) => s.replace(p, q), x.en), de: SWDE.reduce((s, [p, q]) => s.replace(p, q), x.de) });
for (const k of SCALES) a.volumes[k].decisions = a.volumes[k].decisions.map(swap);
const POSE_NOTE = t(" Default pose: the standing quadriceps stretch on the left leg.", " Standardpose: die stehende Oberschenkeldehnung auf dem linken Bein.");
const DEC: Record<string, J> = {
  "50": {
    silhouette: t("The true orthographic views of the 1:50 body (34.4 mm tall): head with jaw, nose and chin under the short hair cap, ear and eye, neck, hooded training top with collar rib, hem rib, hood on the upper back and its rolled edge, sleeves with rib cuffs, hands with thumb and fingers, track pants with seat, kneecaps and ankle ribs, socks, running shoes with sole, toe cap and toe sole. The light side stripe runs down the near leg. Every part is bound to its own bone; trunk, neck, head and thighs are prisms of their outlines, arms and hands revolved or extruded profiles, so the side bend and the slanted stance read right in all three views." + POSE_NOTE.en, "Die wahren orthogonalen Ansichten des 1:50-Körpers (34,4 mm hoch): Kopf mit Kiefer, Nase und Kinn unter der kurzen Haarkappe, Ohr und Auge, Hals, Kapuzen-Trainingsoberteil mit Kragenbund, Saumbund, Kapuze auf dem oberen Rücken und ihrem Rollrand, Ärmel mit Bündchen, Hände mit Daumen und Fingern, Trainingshose mit Gesäß, Kniescheiben und Knöchelbündchen, Socken, Laufschuhe mit Sohle, Zehenkappe und Zehensohle. Der helle Seitenstreifen läuft am nahen Bein hinab. Jedes Teil ist an seinen eigenen Knochen gebunden; Rumpf, Hals, Kopf und Oberschenkel sind Prismen ihrer Umrisse, Arme und Hände gedrehte oder extrudierte Profile, sodass Seitbeuge und schräges Standbein in allen drei Ansichten stimmen." + POSE_NOTE.de),
    retained: t("Everything of the 1:100 drawing plus collar rib, drawstrings, hood fold, kangaroo pocket, knee folds, ankle ribs, laces, finger separations, brow and mouth; the side stripe of the track pants as a light band.", "Alles aus der Zeichnung 1:100 dazu Kragenbund, Kordeln, Kapuzenfalte, Kängurutasche, Kniefalten, Knöchelbündchen, Schnürung, Fingertrennungen, Braue und Mund; der Seitenstreifen der Trainingshose als helles Band."),
    merged: t("Eyelets merge into the lace strokes; hair strands are three strokes; lips, chin and brows merge into the head outline.", "Ösen gehen in den Schnürstrichen auf; Haarsträhnen sind drei Striche; Lippen, Kinn und Brauen gehen im Kopfumriss auf."),
    omitted: t("Pores, single hairs, stitching, fabric fibres, the zip of the pocket and the reflective logo of the shoes; finger separations in the front view, where the hand is seen edge-on.", "Poren, einzelne Haare, Nähte, Gewebefasern, der Taschenreißverschluss und das Reflexlogo der Schuhe; Fingertrennungen in der Vorderansicht, in der die Hand von der Kante gesehen ist."),
    thinElements: t("Drawstrings, folds, laces and the 12 mm side stripe stay at their true width or as detail strokes of 0.13 mm; no element is thickened.", "Kordeln, Falten, Schnürung und der 12 mm breite Seitenstreifen bleiben in wahrer Breite oder als Detailstriche von 0,13 mm; kein Element wird verdickt."),
    hatching: t("The fleece of the training top keeps its vertical knit lines on the sleeves; the jersey of the track pants is plain, a dark tone instead of the denim twill of the base figure.", "Das Fleece des Trainingsoberteils behält seine senkrechten Strickstriche an den Ärmeln; der Jersey der Trainingshose ist glatt, ein dunkler Ton statt des Denim-Köpers der Ausgangsfigur."),
    styles: t("Body 0.25 mm, detail 0.15 mm, texture 0.1 mm; mid-grey training top, dark track pants with a light stripe, light shoes, light skin.", "Körper 0,25 mm, Detail 0,15 mm, Textur 0,1 mm; mittelgraues Trainingsoberteil, dunkle Trainingshose mit hellem Streifen, helle Schuhe, helle Haut."),
    overlays: t("One region per shape: face skin, hair, hand skin, training top (hoodie), track pants and running shoes; the stripe and the folds carry the region of the garment they sit on.", "Eine Region je Form: Gesichtshaut, Haar, Handhaut, Trainingsoberteil (Kapuzenpulli), Trainingshose und Laufschuhe; Streifen und Falten tragen die Region des Kleidungsstücks, auf dem sie sitzen."),
  },
  "100": {
    silhouette: t("The true three views of the 1:100 body (17.2 mm tall): head with nose under the hair cap, neck, training top with hood, hem rib and pocket, sleeves, hands as mitten forms with thumb, track pants, socks and running shoes; the side stripe on the near leg." + POSE_NOTE.en, "Die wahren drei Ansichten des 1:100-Körpers (17,2 mm hoch): Kopf mit Nase unter der Haarkappe, Hals, Trainingsoberteil mit Kapuze, Saumbund und Tasche, Ärmel, Hände als Fäustlingsformen mit Daumen, Trainingshose, Socken und Laufschuhe; der Seitenstreifen am nahen Bein." + POSE_NOTE.de),
    retained: t("Hood, hem rib, pocket, collar, the stripe as one detail band, shoe sole.", "Kapuze, Saumbund, Tasche, Kragen, der Streifen als ein Detailband, Schuhsohle."),
    merged: t("Fingers into the mitten; drawstrings and hood fold into the hood; knee folds into the trouser leg.", "Finger in den Fäustling; Kordeln und Kapuzenfalte in die Kapuze; Kniefalten in das Hosenbein."),
    omitted: t("Laces, ankle ribs, brows, finger separations, sleeve knit lines.", "Schnürung, Knöchelbündchen, Brauen, Fingertrennungen, Strickstriche der Ärmel."),
    thinElements: t("The stripe and the seams are detail strokes of 0.1 mm; nothing thinner than the profile's feature is filled.", "Streifen und Nähte sind Detailstriche von 0,1 mm; nichts Dünneres als das Profilmerkmal wird gefüllt."),
    hatching: t("None; the track pants are a plain dark tone.", "Keine; die Trainingshose ist ein glatter dunkler Ton."),
    styles: t("Body 0.18 mm, detail 0.1 mm; the tones of 1:50.", "Körper 0,18 mm, Detail 0,1 mm; die Töne von 1:50."),
    overlays: t("The same six regions as at 1:50.", "Dieselben sechs Regionen wie bei 1:50."),
  },
  "200": {
    silhouette: t("The true three views of the 1:200 body (8.6 mm tall) in main masses: head with hair, neck, training top with hood, sleeve, forearm and hand per arm, track pants seat, thigh, shank and shoe per leg; the poses read by the bent knee, the raised arm and the hinged trunk." + POSE_NOTE.en, "Die wahren drei Ansichten des 1:200-Körpers (8,6 mm hoch) in Hauptmassen: Kopf mit Haar, Hals, Trainingsoberteil mit Kapuze, Ärmel, Unterarm und Hand je Arm, Hosengesäß, Oberschenkel, Unterschenkel und Schuh je Bein; die Posen lesen sich am gebeugten Knie, am erhobenen Arm und am vorgeneigten Rumpf." + POSE_NOTE.de),
    retained: t("Hood mass on the back, the dark trouser legs against the light shoes.", "Kapuzenmasse auf dem Rücken, die dunklen Hosenbeine gegen die hellen Schuhe."),
    merged: t("Hood and collar into the trunk; hands into simple forms.", "Kapuze und Kragen in den Rumpf; Hände zu einfachen Formen."),
    omitted: t("Stripe, pocket, ribs, laces and every face detail.", "Streifen, Tasche, Bündchen, Schnürung und alle Gesichtsdetails."),
    thinElements: t("None below 0.2 mm on paper.", "Nichts unter 0,2 mm auf dem Papier."),
    hatching: t("None.", "Keine."),
    styles: t("Body 0.13 mm only; mid top, dark legs, light shoes and skin.", "Nur Körperstift 0,13 mm; mittleres Oberteil, dunkle Beine, helle Schuhe und Haut."),
    overlays: t("All six regions keep their own shapes.", "Alle sechs Regionen behalten ihre eigenen Formen."),
  },
  "500": {
    silhouette: t("A figure of 1.72 m (3.4 mm), the true orthographic views of the 1:500 body: head with hair as one mass, neck, training top with the hood merged into the upper back, track pants seat, and per side sleeve, forearm, hand, thigh, shank and shoe; the stretch poses still read as a raised heel, a raised arm or a hinged trunk." + POSE_NOTE.en, "Eine Figur von 1,72 m (3,4 mm), die wahren orthogonalen Ansichten des 1:500-Körpers: Kopf mit Haar als eine Masse, Hals, Trainingsoberteil mit in den oberen Rücken verschmolzener Kapuze, Hosengesäß und je Seite Ärmel, Unterarm, Hand, Oberschenkel, Unterschenkel und Schuh; die Dehnposen lesen sich noch als angehobene Ferse, erhobener Arm oder vorgeneigter Rumpf." + POSE_NOTE.de),
    retained: t("The pose silhouette, the exact stature and extent, the dark legs against light shoes.", "Die Posensilhouette, die genaue Körpergröße und Ausdehnung, die dunklen Beine gegen die hellen Schuhe."),
    merged: t("Face and hair; hood into the training top.", "Gesicht und Haar; Kapuze in das Trainingsoberteil."),
    omitted: t("Sleeve cuffs, stripe, laces, sole line and digits.", "Ärmelbündchen, Streifen, Schnürung, Sohlenlinie und Finger."),
    thinElements: t("Nothing is thickened; drawstrings and ribs are not drawn.", "Nichts wird verdickt; Kordeln und Bündchen werden nicht gezeichnet."),
    hatching: t("None.", "Keine."),
    styles: t("Pen 0.13 mm only; mid-grey top, dark legs, light shoes; light mode dark lines on white, dark mode inverted.", "Nur Stift 0,13 mm; mittelgraues Oberteil, dunkle Beine, helle Schuhe; hell: dunkle Linien auf Weiß, dunkel: invertiert."),
    overlays: t("Merged shapes list all regions they show; head, hands, legs and shoes keep their own value.", "Verschmolzene Formen führen alle Regionen auf, die sie zeigen; Kopf, Hände, Beine und Schuhe behalten ihren eigenen Wert."),
  },
};
for (const k of SCALES) a.representations[k].decisions = DEC[k];
//#endregion

pruneUnused(a);
fitDimensions(a);
console.log("[DEBUG] dimensions", JSON.stringify(a.dimensions));
saveDoc(ID, doc);
