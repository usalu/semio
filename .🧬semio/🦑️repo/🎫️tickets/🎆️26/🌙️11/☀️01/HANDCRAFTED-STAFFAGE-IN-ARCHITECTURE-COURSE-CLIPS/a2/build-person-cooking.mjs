import fs from "node:fs";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const doc = JSON.parse(fs.readFileSync(`${S}/standing-adult/🔣️.json`, "utf8"));
const a = doc.asset;
const T = (en, de) => ({ en, de });
const r = (n, d = 4) => Math.round(n * 10 ** d) / 10 ** d;
const deg = (x) => (x * 180) / Math.PI;
const rad = (x) => (x * Math.PI) / 180;

a.id = "person-cooking";
a.title = T("Person Cooking", "Kochende Person");
a.summary = T(
  "An adult of 1.69 m in a fleece sweatshirt, jeans and sneakers standing at a 0.90 m kitchen counter and stirring a pot of 2.5 litres on the hob with a wooden spoon, about 1.8 met. The counter, hob, pot and spoon are part of the figure; the steam above the pot follows the hob level.",
  "Eine erwachsene Person von 1,69 m in Fleece-Sweatshirt, Jeans und Sneakern steht an einer 0,90 m hohen Küchenzeile und rührt mit einem Holzlöffel in einem Topf mit 2,5 Litern auf dem Herd, etwa 1,8 met. Zeile, Herd, Topf und Löffel gehören zur Figur; der Dampf über dem Topf folgt der Herdstufe."
);
a.tags = ["human", "adult", "standing", "kitchen", "cooking", "counter", "pot", "steam", "internal-gains"];
a.provenance.notes = T(
  "Figure derived by hand from the archive's standing-adult asset; the counter (0.90 m worktop height, 0.60 m deep), the 0.22 m pot and the wooden spoon are drawn by hand in the same coordinate frame and move with the floor, not with the person. Arm angles for the stirring and tasting poses were solved so that the hand holds the spoon at the pot and at the mouth. Counter, pot and steam values are authored estimates.",
  "Figur von Hand aus dem Archivobjekt standing-adult abgeleitet; die Küchenzeile (0,90 m Arbeitshöhe, 0,60 m tief), der 0,22 m weite Topf und der Holzlöffel sind von Hand im selben Koordinatensystem gezeichnet und bewegen sich mit dem Boden, nicht mit der Person. Die Armwinkel der Posen Rühren und Probieren wurden so gelöst, dass die Hand den Löffel am Topf und am Mund hält. Werte von Zeile, Topf und Dampf sind Schätzwerte."
);

//#region geometry helpers
const CX = 0.34;
const poly = (pts) => "M " + pts.map(([x, y]) => `${r(x)} ${r(y)}`).join(" L ") + " Z";
const rect = (x0, y0, x1, y1) => poly([[x0, y0], [x1, y0], [x1, y1], [x0, y1]]);
const potBody = (x0, x1, y0, y1, rc) => `M ${x0} ${y1} L ${x0} ${r(y0 + rc)} C ${x0} ${r(y0 + rc * 0.3)} ${r(x0 + rc * 0.3)} ${y0} ${r(x0 + rc)} ${y0} L ${r(x1 - rc)} ${y0} C ${r(x1 - rc * 0.3)} ${y0} ${x1} ${r(y0 + rc * 0.3)} ${x1} ${r(y0 + rc)} L ${x1} ${y1} Z`;
const shape = (id, bone, role, z, filled, tone, region, d, extra = {}) => ({ id, bone, role, z, filled, tone, ...(Array.isArray(region) ? { regions: region } : { region }), kind: "path", d, ...extra });
//#endregion

const PX0 = 0.21, PX1 = 0.43, PY0 = 0.908, PY1 = 1.048;

const f50 = [
  shape("counter-plinth", "counter", "body", 2, true, 0.25, "counter", rect(0.08, 0, 0.58, 0.1)),
  shape("counter-body", "counter", "body", 2.02, true, 0.55, "counter", rect(0, 0.1, 0.6, 0.86)),
  shape("counter-panel", "counter", "detail", 2.04, false, 0.1, "counter", rect(0.03, 0.14, 0.57, 0.82)),
  shape("counter-handle", "counter", "detail", 2.05, true, 0.3, "counter", rect(0.2, 0.775, 0.4, 0.795)),
  shape("worktop", "counter", "body", 2.1, true, 0.4, "counter", rect(-0.02, 0.86, 0.62, 0.9)),
  shape("hob-plate", "counter", "body", 2.2, true, 0.2, "hob", rect(0.06, 0.9, 0.58, 0.908)),
  shape("hob-ring", "counter", "detail", 2.22, true, 0.5, "hob", rect(0.14, 0.908, 0.5, 0.912)),
  shape("pot-handle", "pot", "body", 2.45, true, 0.4, "pot", poly([[0.21, 0.99], [0.15, 0.99], [0.125, 0.995], [0.12, 1.008], [0.15, 1.014], [0.21, 1.014]])),
  shape("pot-body", "pot", "body", 2.5, true, 0.5, "pot", potBody(PX0, PX1, PY0, PY1, 0.02)),
  shape("pot-rim", "pot", "detail", 2.52, true, 0.65, "pot", rect(PX0 - 0.008, 1.036, PX1 + 0.008, PY1)),
  shape("pot-band", "pot", "detail", 2.51, false, 0.1, "pot", "M 0.215 0.97 L 0.425 0.97"),
  shape("spoon-handle", "spoon", "body", 6.6, true, 0.45, "spoon", poly([[-0.12, -0.009], [0.14, -0.007], [0.14, 0.007], [-0.12, 0.009]]), { penMm: 0.1 }),
  shape("spoon-bowl", "spoon", "body", 6.61, true, 0.42, "spoon", `M 0.14 -0.007 C 0.165 -0.02 0.205 -0.024 0.222 0 C 0.205 0.024 0.165 0.02 0.14 0.007 Z`),
];
const f100 = [
  shape("counter-plinth", "counter", "body", 2, true, 0.25, "counter", rect(0.08, 0, 0.58, 0.1)),
  shape("counter-body", "counter", "body", 2.02, true, 0.55, "counter", rect(0, 0.1, 0.6, 0.86)),
  shape("counter-panel", "counter", "detail", 2.04, false, 0.1, "counter", rect(0.04, 0.15, 0.56, 0.81)),
  shape("worktop", "counter", "body", 2.1, true, 0.4, "counter", rect(-0.02, 0.86, 0.62, 0.9)),
  shape("hob-plate", "counter", "body", 2.2, true, 0.2, "hob", rect(0.06, 0.9, 0.58, 0.91)),
  shape("pot-handle", "pot", "body", 2.45, true, 0.4, "pot", poly([[0.21, 0.985], [0.15, 0.985], [0.12, 0.992], [0.115, 1.01], [0.15, 1.02], [0.21, 1.02]])),
  shape("pot-body", "pot", "body", 2.5, true, 0.5, "pot", potBody(PX0, PX1, PY0 + 0.002, PY1, 0.02)),
  shape("pot-rim", "pot", "detail", 2.52, true, 0.65, "pot", rect(PX0 - 0.01, 1.034, PX1 + 0.01, PY1)),
  shape("spoon-handle", "spoon", "body", 6.6, true, 0.45, "spoon", poly([[-0.12, -0.011], [0.14, -0.009], [0.14, 0.009], [-0.12, 0.011]]), { penMm: 0.1 }),
  shape("spoon-bowl", "spoon", "body", 6.61, true, 0.42, "spoon", `M 0.14 -0.009 C 0.165 -0.024 0.205 -0.028 0.224 0 C 0.205 0.028 0.165 0.024 0.14 0.009 Z`),
];
const f200 = [
  shape("counter-body", "counter", "body", 2, true, 0.5, "counter", poly([[0.08, 0], [0.58, 0], [0.58, 0.1], [0.6, 0.1], [0.6, 0.86], [0, 0.86], [0, 0.1], [0.08, 0.1]])),
  shape("worktop", "counter", "body", 2.1, true, 0.35, ["counter", "hob"], rect(-0.02, 0.86, 0.62, 0.91)),
  shape("pot-body", "pot", "body", 2.5, true, 0.5, "pot", poly([[PX0 - 0.01, 0.91], [PX1 + 0.01, 0.91], [PX1 + 0.01, PY1], [PX0 - 0.01, PY1]])),
  shape("spoon", "spoon", "body", 6.6, true, 0.45, "spoon", `M -0.12 -0.016 L 0.15 -0.014 C 0.175 -0.03 0.215 -0.03 0.23 0 C 0.215 0.03 0.175 0.03 0.15 0.014 L -0.12 0.016 Z`),
];
const f500 = [
  shape("sil-counter", "counter", "body", 2, true, 0.5, ["counter", "hob", "pot", "spoon"], poly([[0.08, 0], [0.58, 0], [0.58, 0.1], [0.62, 0.1], [0.62, 0.91], [0.43, 0.91], [0.43, 1.048], [0.21, 1.048], [0.21, 0.91], [-0.02, 0.91], [-0.02, 0.1], [0.08, 0.1]])),
];
const addTo = (rep, shapes) => {
  const g = a.representations[rep].geometry.elevation;
  for (const s of shapes) g.push(s);
};
addTo("50", f50);
addTo("100", f100);
addTo("200", f200);
addTo("500", f500);

const add = (rep, field, en, de) => {
  const d = a.representations[rep].decisions[field];
  d.en = `${d.en} ${en}`;
  d.de = `${d.de} ${de}`;
};
add("500", "silhouette", "The counter, hob and pot are one stepped column of 0.60 m depth beside the figure, the pot as a bump on the worktop.", "Zeile, Herd und Topf sind eine gestufte Säule von 0,60 m Tiefe neben der Figur, der Topf als Erhebung auf der Arbeitsplatte.");
add("500", "merged", "Counter, hob, pot and spoon are one shape listing all four regions.", "Zeile, Herd, Topf und Löffel sind eine Form mit allen vier Regionen.");
add("500", "omitted", "The 16 mm spoon and the pot handle; the spoon motion is not drawn at this scale.", "Der 16 mm breite Löffel und der Topfgriff; die Löffelbewegung wird in diesem Maßstab nicht gezeichnet.");
add("200", "silhouette", "The counter is a stepped box with its plinth recess, the worktop a slab with the hob and the pot on it.", "Die Zeile ist ein gestufter Kasten mit Sockelrücksprung, die Arbeitsplatte eine Platte mit Herd und Topf darauf.");
add("200", "merged", "Hob and worktop are one slab listing both regions; pot rim and body are one shape.", "Herd und Arbeitsplatte sind eine Platte mit beiden Regionen; Topfrand und Topfkörper sind eine Form.");
add("200", "thinElements", "The 16 mm spoon handle is thickened to 32 mm so it stays printable as the stirring cue; the pot handle is omitted.", "Der 16 mm breite Löffelstiel wird auf 32 mm verdickt, damit er als Rührzeichen druckbar bleibt; der Topfgriff entfällt.");
add("100", "retained", "Counter with plinth recess and door panel, hob plate, pot with rim and handle, wooden spoon with bowl.", "Zeile mit Sockelrücksprung und Türfeld, Herdplatte, Topf mit Rand und Griff, Holzlöffel mit Laffe.");
add("100", "omitted", "The door handle and the burner ring.", "Der Türgriff und der Brennring.");
add("50", "retained", "Counter with plinth recess, door panel and handle, worktop, hob plate with burner ring, pot with rim, band and side handle, wooden spoon with bowl.", "Zeile mit Sockelrücksprung, Türfeld und Griff, Arbeitsplatte, Herdplatte mit Brennring, Topf mit Rand, Band und Seitengriff, Holzlöffel mit Laffe.");
for (const k of ["50", "100", "200", "500"]) {
  add(k, "overlays", "The pot region shows the hob heat and the steam anchors above it; the counter and hob stay at the room temperature.", "Die Topfregion zeigt die Herdwärme und die Dampfanker darüber; Zeile und Herd bleiben auf Raumtemperatur.");
}

//#region rig
const bones = a.rig.bones;
const PELVIS_Y = bones.find((b) => b.id === "pelvis").y;
const PELVIS_LEN = bones.find((b) => b.id === "pelvis").lengthM;
const TORSO_ARM_X = bones.find((b) => b.id === "arm-r").x;
const L1 = bones.find((b) => b.id === "arm-r").lengthM;
const L2 = bones.find((b) => b.id === "forearm-r").lengthM;
const GRIP = 0.09;
bones.push(
  { id: "counter-lift", parent: "pelvis", title: T("Counter vertical float on the floor", "Senkrechte Führung der Küchenzeile am Boden"), x: -PELVIS_Y, y: -CX, angleDeg: -90, lengthM: 0.2, joint: { type: "prismatic", axisDeg: 0, minM: -0.15, maxM: 0.15 } },
  { id: "counter", parent: "counter-lift", title: T("Kitchen counter with hob", "Küchenzeile mit Herd"), x: 0, y: 0, angleDeg: 0, lengthM: 0.6, joint: { type: "fixed" }, mass: { kg: 70, comX: 0.3, comY: 0.45, distribution: "point", source: "authored-estimate" } },
  { id: "pot", parent: "counter", title: T("Pot with 2.5 litres of water", "Topf mit 2,5 Litern Wasser"), x: 0, y: 0, angleDeg: 0, lengthM: 0.14, joint: { type: "fixed" }, mass: { kg: 4.2, comX: 0.32, comY: 0.98, distribution: "point", source: "authored-estimate" } },
  { id: "spoon", parent: "hand-r", title: T("Wooden spoon", "Holzlöffel"), x: GRIP, y: 0, angleDeg: 0, lengthM: 0.22, joint: { type: "revolute", minDeg: -180, maxDeg: 180 }, mass: { kg: 0.06, comX: 0.05, comY: 0, distribution: "point", source: "authored-estimate" } }
);
a.dimensions.massKg = r(bones.reduce((s, b) => s + (b.mass?.kg ?? 0), 0), 2);
a.dimensions.widthM = 1.088;
a.dimensions.depthM = 0.62;
a.anchors.push(
  { id: "counter-contact", bone: "counter-lift", x: 0, y: 0, kind: "contact", title: T("Counter on the floor", "Küchenzeile am Boden"), support: "contact", normalDeg: 90, footprintLengthM: 0.6, footprintWidthM: 0.6, frictionCoefficient: 0.6 },
  { id: "pot-mouth", bone: "pot", x: 0.32, y: 1.05, kind: "emitter", title: T("Pot mouth", "Topföffnung") },
  { id: "grip-r", bone: "hand-r", x: GRIP, y: 0, kind: "attachment", title: T("Right hand on the spoon", "Rechte Hand am Löffel") },
  { id: "spoon-cog", bone: "spoon", x: 0.05, y: 0, kind: "load", title: T("Spoon centre of gravity", "Schwerpunkt des Löffels") }
);
a.materials.push(
  { id: "laminate-chipboard", title: T("Laminated chipboard", "Beschichtete Spanplatte"), emissivity: 0.9, solarAbsorptivity: 0.5, source: "authored-estimate" },
  { id: "glass-ceramic", title: T("Glass-ceramic hob plate", "Glaskeramik-Kochfeld"), emissivity: 0.9, solarAbsorptivity: 0.85, source: "authored-estimate" },
  { id: "stainless-steel-pot", title: T("Brushed stainless-steel pot", "Gebürsteter Edelstahltopf"), emissivity: 0.35, solarAbsorptivity: 0.5, source: "authored-estimate" },
  { id: "beech-wood", title: T("Beech wood spoon", "Buchenholzlöffel"), emissivity: 0.9, solarAbsorptivity: 0.6, source: "authored-estimate" }
);
//#endregion

//#region thermal
const t = a.thermal;
t.environment = { ...t.environment, airTempC: 22, meanRadiantTempC: 22, skyTempC: 22, groundTempC: 22, relativeHumidity: 0.5 };
t.baseMet = 1.8;
t.respiration.patterns = [
  { id: "light-work", title: T("Light standing work, slightly deeper", "Leichte Stehtätigkeit, etwas tiefer"), rateBpm: 16, tidalVolumeL: 0.5, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.03, exhaleFraction: 0.48, pauseAfterExhaleFraction: 0.11, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
];
t.operating = [{ id: "hob", title: T("Hob level", "Herdstufe"), levels: [{ v: 0, title: T("Off", "Aus") }, { v: 0.4, title: T("Simmering", "Köcheln") }, { v: 1, title: T("Boiling", "Kochen") }] }];
t.regions.push(
  { id: "counter", title: T("Kitchen counter, chipboard with laminate", "Küchenzeile, Spanplatte mit Beschichtung"), kind: "surface", model: "passive", material: "laminate-chipboard", areaM2: 2.4, characteristicLengthM: 0.8, geometry: "vertical-plate", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 17000, source: "authored-estimate" },
  { id: "hob", title: T("Glass-ceramic hob plate", "Glaskeramik-Kochfeld"), kind: "surface", model: "passive", material: "glass-ceramic", areaM2: 0.3, characteristicLengthM: 0.55, geometry: "horizontal-plate-up", skyViewFactor: 0.8, layers: [], heatCapacityJM2K: 7000, source: "authored-estimate" },
  { id: "pot", title: T("Pot with water, heated by the hob", "Topf mit Wasser, vom Herd erwärmt"), kind: "surface", model: "operating", material: "stainless-steel-pot", areaM2: 0.12, characteristicLengthM: 0.14, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 60000, operating: { key: "hob", heatW: 220 }, source: "authored-estimate" },
  { id: "spoon", title: T("Wooden spoon", "Holzlöffel"), kind: "surface", model: "passive", material: "beech-wood", areaM2: 0.012, characteristicLengthM: 0.02, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 4500, source: "authored-estimate" }
);
t.emitters.push({ id: "steam", kind: "exhaust", title: T("Steam above the pot", "Dampf über dem Topf"), anchor: "pot-mouth", directionDeg: 90, diameterM: 0.2, operatingKey: "hob", volumeFlowM3S: 0.003, temperatureRiseK: 55, vapourKgPerKg: 0.12, source: "authored-estimate" });
t.assumptions = [
  ...t.assumptions.slice(0, 2),
  T("Cooking while standing is 1.8 met (ISO 7730 Annex A lists 1.6 to 2.0 met for cooking); tasting is 1.6 met. The default surroundings are a 22 degrees C kitchen.", "Kochen im Stehen sind 1,8 met (ISO 7730 Anhang A nennt 1,6 bis 2,0 met für Kochen); Probieren 1,6 met. Die Standardumgebung ist eine Küche mit 22 Grad C."),
  T("The pot releases about 220 W as sensible heat to its surroundings at full boil and 40 % of it when simmering, the steam carries 0.36 g/s of vapour at full boil; both are authored estimates, not measurements. The counter and the hob stay at room temperature.", "Der Topf gibt beim starken Kochen etwa 220 W fühlbar an die Umgebung ab, beim Köcheln 40 % davon; der Dampf trägt beim starken Kochen 0,36 g/s Wasserdampf; beides sind Schätzwerte, keine Messungen. Zeile und Herd bleiben auf Raumtemperatur."),
];
a.structure.assumptions.push(T("The counter of 70 kg and the pot of 4.2 kg with its water are point masses on their own bones; the counter rests on the floor through its own contact and does not load the person.", "Die 70 kg schwere Zeile und der 4,2 kg schwere Topf samt Wasser sind Punktmassen auf eigenen Knochen; die Zeile steht über einen eigenen Kontakt auf dem Boden und belastet die Person nicht."));
//#endregion

//#region kinematics
const reach = (torsoDelta, target, spoonAbs, headNeck = {}) => {
  const torsoAbs = 90 + torsoDelta;
  const S = [Math.cos(rad(torsoAbs)) * TORSO_ARM_X, PELVIS_Y + PELVIS_LEN + Math.sin(rad(torsoAbs)) * TORSO_ARM_X];
  const W = [target[0] - 0, target[1]];
  const Lf = L2 + GRIP;
  const dx = W[0] - S[0], dy = W[1] - S[1];
  const d = Math.hypot(dx, dy);
  if (d > L1 + Lf - 1e-3 || d < Math.abs(L1 - Lf) + 1e-3) throw new Error(`unreachable ${d}`);
  const base = Math.atan2(dy, dx);
  const cosA = (L1 * L1 + d * d - Lf * Lf) / (2 * L1 * d);
  const ang = Math.acos(cosA);
  const phi1 = base - ang;
  const E = [S[0] + L1 * Math.cos(phi1), S[1] + L1 * Math.sin(phi1)];
  const phi2 = Math.atan2(W[1] - E[1], W[0] - E[0]);
  const arm = deg(phi1) - (torsoAbs + 180);
  const fore = deg(phi2) - deg(phi1);
  const hand = 0;
  const spoon = spoonAbs - deg(phi2);
  const norm = (x) => ((x + 180) % 360 + 360) % 360 - 180;
  return { torso: { angleDeg: torsoDelta }, "arm-r": { angleDeg: r(norm(arm), 2) }, "forearm-r": { angleDeg: r(norm(fore), 2) }, "hand-r": { angleDeg: hand }, spoon: { angleDeg: r(norm(spoon), 2) }, ...headNeck, _elbow: E };
};
const strip = ({ _elbow, ...rest }) => rest;
//#endregion

const legs = { "thigh-l": { angleDeg: 6 }, "shank-l": { angleDeg: -14 }, "foot-l": { angleDeg: 8 }, "toe-l": { angleDeg: 0 }, "thigh-r": { angleDeg: 6 }, "shank-r": { angleDeg: -14 }, "foot-r": { angleDeg: 8 }, "toe-r": { angleDeg: 0 } };
const farArm = { "arm-l": { angleDeg: 10 }, "forearm-l": { angleDeg: 28 }, "hand-l": { angleDeg: 6 } };
const regionsFor = (met) => ({
  "face-skin": { exposedFraction: 1, sunlitFraction: 0 },
  hair: { exposedFraction: 1, sunlitFraction: 0 },
  "hand-skin": { exposedFraction: 1, sunlitFraction: 0 },
  hoodie: { exposedFraction: 0.95, sunlitFraction: 0 },
  jeans: { exposedFraction: 1, sunlitFraction: 0 },
  sneakers: { exposedFraction: 1, sunlitFraction: 0, contact: { fraction: 0.3, tempC: 22 } },
  counter: { exposedFraction: 0.85, sunlitFraction: 0 },
  hob: { exposedFraction: 1, sunlitFraction: 0 },
  pot: { exposedFraction: 1, sunlitFraction: 0 },
  spoon: { exposedFraction: 1, sunlitFraction: 0 },
});
const poseOf = (id, title, description, bones, met, hob) => ({
  id, title, description, bones: { ...legs, ...farArm, ...bones }, ground: "auto",
  contacts: ["heel-l", "ball-l", "heel-r", "ball-r", "counter-contact"],
  thermal: { activityMet: met, breathing: "light-work", regions: regionsFor(met), emitters: { "body-plume": 0.6, steam: 1 }, operating: { hob } },
  loads: [{ id: "spoon-weight", kind: "carried", title: T("Spoon weight", "Gewicht des Löffels"), anchor: "spoon-cog", fxN: 0, fyN: -0.589, source: "authored-estimate" }],
});

const STIR_C = [0.57, 1.16];
const stir = reach(-9, STIR_C, -70, { neck: { angleDeg: -4 }, head: { angleDeg: -12 } });
const taste = reach(-2, [0.3, 1.52], 190, { neck: { angleDeg: -2 }, head: { angleDeg: -4 } });
console.log("stir elbow", stir._elbow.map((x) => r(x, 3)), JSON.stringify(strip(stir)));
console.log("taste elbow", taste._elbow.map((x) => r(x, 3)), JSON.stringify(strip(taste)));
a.poses = [
  poseOf("stirring", T("Stirring the pot", "Im Topf rühren"), T("Feet together at the counter, trunk leaning slightly forward, the near hand holds the wooden spoon that stands in the pot, the far arm rests at the side of the body, the head looks down into the pot.", "Füße beieinander an der Zeile, Rumpf leicht nach vorn geneigt, die nahe Hand hält den Holzlöffel, der im Topf steht, der ferne Arm ruht seitlich am Körper, der Kopf schaut in den Topf."), strip(stir), 1.8, 0.4),
  poseOf("tasting", T("Tasting from the spoon", "Vom Löffel probieren"), T("Standing upright at the counter, the near hand brings the spoon with its bowl to the mouth, the head tilted slightly forward.", "Aufrecht an der Zeile stehend, die nahe Hand führt den Löffel mit der Laffe zum Mund, der Kopf leicht nach vorn geneigt."), strip(taste), 1.6, 0.4),
  poseOf("standing-by", T("Standing by the counter", "An der Zeile abwartend"), T("Upright at the counter with the spoon hanging in the lowered hand along the thigh, the head turned toward the pot, waiting for the water to boil.", "Aufrecht an der Zeile, der Löffel hängt in der gesenkten Hand am Oberschenkel, der Kopf ist dem Topf zugewandt, die Person wartet, bis das Wasser kocht."), { torso: { angleDeg: 0 }, "arm-r": { angleDeg: 5 }, "forearm-r": { angleDeg: 10 }, "hand-r": { angleDeg: 0 }, spoon: { angleDeg: -15 }, neck: { angleDeg: -1 }, head: { angleDeg: -3 } }, 1.4, 0.4),
  poseOf("standing-by", T("Standing by the counter", "An der Zeile abwartend"), T("Upright at the counter with the spoon laid down along the thigh in the hanging hand, the head turned toward the pot, waiting for the water to boil.", "Aufrecht an der Zeile, der Löffel hängt in der Hand am Oberschenkel, der Kopf ist dem Topf zugewandt, die Person wartet, bis das Wasser kocht."), { torso: { angleDeg: 0 }, "arm-r": { angleDeg: 5 }, "forearm-r": { angleDeg: 10 }, "hand-r": { angleDeg: 0 }, spoon: { angleDeg: -15 }, neck: { angleDeg: -1 }, head: { angleDeg: -3 } }, 1.4, 0.4),
];

const keys = [];
const N = 8, DUR = 2;
for (let i = 0; i <= N; i++) {
  const th = (2 * Math.PI * (i % N)) / N;
  const tg = [STIR_C[0] + 0.045 * Math.cos(th), STIR_C[1] + 0.02 * Math.sin(th)];
  const k = strip(reach(-9 + 0.8 * Math.sin(th), tg, -70 + 7 * Math.cos(th), { neck: { angleDeg: -4 }, head: { angleDeg: -12 + 1.2 * Math.sin(th) } }));
  keys.push({ t: r((DUR * i) / N, 3), pose: "stirring", ease: "smooth", bones: k });
}
a.clips = [
  {
    id: "stirring", title: T("Stirring", "Rühren"),
    description: T("A two-second loop: the near hand moves the spoon on a flat ellipse in the pot, the trunk rocks a little with each stroke and the head follows the pot; the feet stay planted and the pot stands still.", "Eine Schleife von zwei Sekunden: Die nahe Hand führt den Löffel auf einer flachen Ellipse im Topf, der Rumpf wiegt sich mit jedem Zug leicht und der Kopf folgt dem Topf; die Füße bleiben stehen und der Topf steht still."),
    durationS: DUR, fps: 30, loop: true, rootMotion: "none", keys,
    contacts: ["heel-l", "ball-l", "heel-r", "ball-r", "counter-contact"].map((anchor) => ({ anchor, fromS: 0, toS: DUR })),
    phases: [{ id: "stir", fromS: 0, toS: DUR, title: T("One stirring stroke", "Ein Rührzug") }],
    events: [],
    tracks: [
      { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: DUR, v: 0 }] },
      { target: "operating:hob", ease: "smooth", keys: [{ t: 0, v: 0.4 }, { t: DUR, v: 0.4 }] },
    ],
  },
];
a.clips.push({
  id: "stir-and-taste", title: T("Stirring and tasting", "Rühren und Probieren"),
  description: T("A four-second loop: the person stirs, lifts the spoon to the mouth to taste, holds it a moment and goes back to stirring.", "Eine Schleife von vier Sekunden: Die Person rührt, führt den Löffel zum Probieren zum Mund, hält kurz inne und rührt weiter."),
  durationS: 4, fps: 30, loop: true, rootMotion: "none",
  keys: [{ t: 0, pose: "stirring", ease: "smooth" }, { t: 1, pose: "stirring", ease: "smooth" }, { t: 2, pose: "tasting", ease: "smooth" }, { t: 2.8, pose: "tasting", ease: "smooth" }, { t: 4, pose: "stirring", ease: "smooth" }],
  contacts: ["heel-l", "ball-l", "heel-r", "ball-r", "counter-contact"].map((anchor) => ({ anchor, fromS: 0, toS: 4 })),
  phases: [{ id: "stir", fromS: 0, toS: 1, title: T("Stirring", "Rühren") }, { id: "taste", fromS: 1, toS: 2.8, title: T("Tasting", "Probieren") }, { id: "back", fromS: 2.8, toS: 4, title: T("Back to stirring", "Zurück zum Rühren") }],
  events: [],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4, v: 0 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.8 }, { t: 2, v: 1.6 }, { t: 4, v: 1.8 }] }],
});
const taste2 = strip(taste);
const stir0 = strip(reach(-9, STIR_C, -70, { neck: { angleDeg: -4 }, head: { angleDeg: -12 } }));
void taste2; void stir0;
a.clips.push({
  id: "stir-and-taste", title: T("Stirring and tasting", "Rühren und Probieren"),
  description: T("A four-second loop: the person stirs, lifts the spoon to the mouth to taste, holds it a moment and goes back to stirring.", "Eine Schleife von vier Sekunden: Die Person rührt, führt den Löffel zum Probieren zum Mund, hält kurz inne und rührt weiter."),
  durationS: 4, fps: 30, loop: true, rootMotion: "none",
  keys: [{ t: 0, pose: "stirring", ease: "smooth" }, { t: 1, pose: "stirring", ease: "smooth" }, { t: 2, pose: "tasting", ease: "smooth" }, { t: 2.8, pose: "tasting", ease: "smooth" }, { t: 4, pose: "stirring", ease: "smooth" }],
  contacts: ["heel-l", "ball-l", "heel-r", "ball-r", "counter-contact"].map((anchor) => ({ anchor, fromS: 0, toS: 4 })),
  phases: [{ id: "stir", fromS: 0, toS: 1, title: T("Stirring", "Rühren") }, { id: "taste", fromS: 1, toS: 2.8, title: T("Tasting", "Probieren") }, { id: "back", fromS: 2.8, toS: 4, title: T("Back to stirring", "Zurück zum Rühren") }],
  events: [],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4, v: 0 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.8 }, { t: 2, v: 1.6 }, { t: 4, v: 1.8 }] }],
});
a.presentation.defaultPose = "stirring";
a.presentation.defaultClip = "stirring";
a.presentation.defaultScale = 50;

fs.mkdirSync(`${S}/person-cooking`, { recursive: true });
fs.writeFileSync(`${S}/person-cooking/🔣️.json`, JSON.stringify(doc, null, 2) + "\n");
console.log("ok mass", a.dimensions.massKg);
