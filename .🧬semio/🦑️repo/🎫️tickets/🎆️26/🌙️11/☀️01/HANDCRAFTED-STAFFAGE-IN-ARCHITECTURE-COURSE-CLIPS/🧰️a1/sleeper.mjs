// Builds S/🗂️catalog/👤️humans/sleeping-person/🔣️.json: a woman lying under a duvet on the mattress datum of the catalog single-bed
// (mattress top at 0.46 m in its `asleep` pose, head end at x = 0). Body skeleton = seated-reader rig (lengths x 1.012 for 1.70 m) re-posed as a supine chain.
import { writeFileSync, mkdirSync } from "node:fs";
import { S, r4, clone, txt, load, scaleShape, fk, toWorld, wrap, smooth, rpoly, mapPts, mapPath, shape, poly, path, cap } from "./lib.mjs";

const reader = load("👤️humans/seated-reader");
const bed = load("🛋️furniture/single-bed");
const K = 1.012;
const Y0 = 0.46;
const DROP = new Set(["seat", "pad", "page", "cup", "hip-lift", "hip-shift"]);

//#region Rig
const bodySum = reader.rig.bones.filter((b) => b.mass && !DROP.has(b.id)).reduce((s, b) => s + b.mass.kg, 0);
const MASS_F = 64.6 / bodySum;
const bones = [];
bones.push({ id: "bed-surface", parent: null, title: txt("Mattress top datum", "Matratzenoberseite"), x: 0, y: Y0, angleDeg: 0, lengthM: 2.0, joint: { type: "fixed" }, mass: { kg: 2.4, comX: 0.9, comY: 0.1, distribution: "point", source: "authored-estimate" } });
bones.push({ id: "hip-lift", parent: "bed-surface", title: txt("Body vertical travel", "Vertikale Körperverschiebung"), x: 1.05, y: 0.115, angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 90, minM: -0.03, maxM: 0.3 } });
bones.push({ id: "hip-shift", parent: "hip-lift", title: txt("Body travel along the bed", "Körperverschiebung entlang des Betts"), x: 0, y: 0, angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 0, minM: -0.2, maxM: 0.2 } });
const REST_ANGLE = { pelvis: 180, torso: 0, neck: -35, head: 30, "thigh-l": 178.5, "thigh-r": 178.5, "shank-l": -2.5, "shank-r": -2.5, "foot-l": 86, "foot-r": 86, "toe-l": 10, "toe-r": 10 };
const TITLES = { head: txt("Head with loose hair", "Kopf mit offenem Haar"), torso: txt("Torso in pyjama top", "Rumpf im Schlafanzugoberteil"), "foot-l": txt("Left bare foot", "Linker nackter Fuß"), "foot-r": txt("Right bare foot", "Rechter nackter Fuß"), "toe-l": txt("Left toes", "Linke Zehen"), "toe-r": txt("Right toes", "Rechte Zehen") };
for (const b of reader.rig.bones) {
  if (DROP.has(b.id)) continue;
  const n = clone(b);
  Object.assign(n, { x: r4(b.x * K), y: r4(b.y * K), lengthM: r4(b.lengthM * K) });
  if (b.id === "pelvis") Object.assign(n, { parent: "hip-shift", x: 0, y: 0 });
  if (REST_ANGLE[b.id] !== undefined) n.angleDeg = REST_ANGLE[b.id];
  if (b.id === "pelvis") n.joint = { type: "revolute", minDeg: -25, maxDeg: 25 };
  if (b.id === "neck") n.joint = { type: "revolute", minDeg: -35, maxDeg: 35 };
  if (b.id === "head") n.joint = { type: "revolute", minDeg: -35, maxDeg: 35 };
  if (b.id.startsWith("thigh")) n.joint = { type: "revolute", minDeg: -20, maxDeg: 125 };
  if (b.id.startsWith("shank")) n.joint = { type: "revolute", minDeg: -150, maxDeg: 10 };
  if (b.id.startsWith("forearm")) n.joint = { type: "revolute", minDeg: -30, maxDeg: 150 };
  if (b.id.startsWith("torso")) n.joint = { type: "revolute", minDeg: -30, maxDeg: 40 };
  if (TITLES[b.id]) n.title = TITLES[b.id];
  if (n.mass) n.mass = { ...n.mass, kg: r4(n.mass.kg * MASS_F), comX: r4(n.mass.comX * K), comY: r4(n.mass.comY * K) };
  bones.push(n);
}
const pm = (kg, comX, comY = 0) => ({ kg, comX, comY, distribution: "point", source: "authored-estimate" });
bones.push({ id: "pillow", parent: "bed-surface", title: txt("Pillow", "Kopfkissen"), x: 0.37, y: 0, angleDeg: 0, lengthM: 0.52, joint: { type: "fixed" }, mass: pm(0.7, 0, 0.05) });
bones.push({ id: "duvet", parent: "bed-surface", title: txt("Duvet drape over the bed side", "Bettdeckenfall über der Bettseite"), x: 1.1, y: 0, angleDeg: 0, lengthM: 1.2, joint: { type: "fixed" }, mass: pm(2.0, 0, 0.05) });
const totalMass = r4(bones.reduce((s, b) => s + (b.mass?.kg ?? 0), 0));
const REST = fk(bones);
const fr = (b) => ({ ...REST.get(b), y: REST.get(b).y - Y0 });
//#endregion

//#region Anchors
const A = (id, bone, x, y, en, de, extra = {}) => ({ id, bone, x, y, kind: "support", title: txt(en, de), support: "contact", normalDeg: 90, frictionCoefficient: 0.5, ...extra });
const readerAnchor = (id) => reader.anchors.find((a) => a.id === id);
const sc = (a) => ({ ...clone(a), x: r4(a.x * K), y: r4(a.y * K) });
const anchors = [
  A("head-pillow", "head", 0.086 * K, 0.1 * K, "Back of the head on the pillow", "Hinterkopf auf dem Kissen", { footprintLengthM: 0.14, footprintWidthM: 0.16 }),
  A("shoulders-mattress", "torso", 0.34 * K, 0.115 * K, "Shoulder blades on the mattress", "Schulterblätter auf der Matratze", { footprintLengthM: 0.2, footprintWidthM: 0.36 }),
  A("back-mattress", "torso", 0.1 * K, 0.115 * K, "Lower back on the mattress", "Unterer Rücken auf der Matratze", { footprintLengthM: 0.2, footprintWidthM: 0.3 }),
  A("pelvis-mattress", "pelvis", 0.05 * K, 0.1 * K, "Buttocks on the mattress", "Gesäß auf der Matratze", { footprintLengthM: 0.2, footprintWidthM: 0.3 }),
  A("calf-mattress", "shank-r", 0.2 * K, -0.06 * K, "Calves on the mattress", "Waden auf der Matratze", { footprintLengthM: 0.25, footprintWidthM: 0.22 }),
  A("heel-mattress", "foot-r", -0.0103 * K, -0.0819 * K, "Heels on the mattress", "Fersen auf der Matratze", { footprintLengthM: 0.07, footprintWidthM: 0.1, frictionCoefficient: 0.6 }),
  { ...sc(readerAnchor("mouth")), title: txt("Mouth opening", "Mundöffnung") },
  { ...sc(readerAnchor("nostrils")), title: txt("Nostrils", "Nasenöffnungen") },
  { id: "chest-plume", bone: "torso", x: r4(0.22 * K), y: -0.17, kind: "emitter", title: txt("Above the chest under the duvet", "Über der Brust unter der Bettdecke") },
];
//#endregion

//#region Shapes
const TONES = { "face-skin": 0.82, "hand-skin": 0.82, hair: 0.22, top: 0.68, bottom: 0.66, "feet-skin": 0.8, pillow: 0.86 };
const WP = (id, bone, role, z, tone, d, region, extra = {}) => shape(id, bone, role, z, tone, path(mapPath(fr(bone), d)), { region, ...extra });
const WL = (id, bone, role, z, tone, pts, region, extra = {}) => shape(id, bone, role, z, tone, poly(mapPts(fr(bone), pts), false), { region, filled: false, ...extra });
const mound = (pts, r) => rpoly(pts.map(([x, y]) => [x, y + 0.0]), r);
const MOUNDS = {
  chest: { bone: "torso", z: 4, region: "top", pts: [[0.5, 0.0], [0.502, 0.18], [0.53, 0.262], [0.6, 0.29], [0.78, 0.298], [0.93, 0.288], [0.98, 0.0]], r: [0.01, 0.03, 0.04, 0.05, 0.05, 0.04, 0.01] },
  hips: { bone: "pelvis", z: 4.1, region: "top", pts: [[0.92, 0.0], [0.93, 0.288], [1.05, 0.288], [1.2, 0.27], [1.2, 0.0]], r: [0.01, 0.03, 0.06, 0.04, 0.01] },
  thighs: { bone: "thigh-r", z: 4.2, region: "bottom", pts: [[1.14, 0.0], [1.14, 0.272], [1.3, 0.25], [1.48, 0.208], [1.5, 0.0]], r: [0.01, 0.03, 0.08, 0.05, 0.01] },
  shins: { bone: "shank-r", z: 4.3, region: "bottom", pts: [[1.44, 0.0], [1.44, 0.21], [1.6, 0.175], [1.72, 0.15], [1.745, 0.075], [1.72, 0.0]], r: [0.01, 0.04, 0.07, 0.06, 0.04, 0.01] },
};
function duvetShapes(scale) {
  const out = [];
  for (const [k, m] of Object.entries(MOUNDS)) out.push(WP(`duvet-${k}`, m.bone, "body", m.z, TONES[m.region], mound(m.pts, m.r), m.region));
  if (scale === 500) out.push(WP("sil-duvet-skirt", "bed-surface", "body", 6, 0.67, rpoly([[0.5, -0.13], [1.72, -0.13], [1.72, 0.09], [0.5, 0.09]], [0.02, 0.02, 0.0, 0.0]), undefined, { regions: ["top", "bottom"] }));
  else {
    out.push(WP("duvet-skirt-top", "bed-surface", "body", 6, TONES.top, rpoly([[0.5, -0.13], [1.06, -0.13], [1.06, 0.09], [0.5, 0.09]], [0.012, 0.012, 0.0, 0.0]), "top"));
    out.push(WP("duvet-skirt-legs", "bed-surface", "body", 6.05, TONES.bottom, rpoly([[1.04, -0.13], [1.72, -0.13], [1.72, 0.09], [1.04, 0.09]], [0.012, 0.012, 0.0, 0.0]), "bottom"));
  }
  if (scale <= 100) {
    out.push(WL("duvet-hem", "bed-surface", "detail", 6.1, 0.3, [[0.52, -0.105], [1.7, -0.105]], "top"));
    out.push(WL("duvet-fold-chest", "torso", "detail", 4.5, 0.3, [[0.56, 0.256], [0.64, 0.275], [0.74, 0.27]], "top"));
  }
  if (scale === 50) {
    out.push(WL("duvet-fold-hips", "pelvis", "detail", 4.6, 0.3, [[0.99, 0.285], [1.07, 0.245], [1.15, 0.255]], "top"));
    out.push(WL("duvet-fold-knee", "thigh-r", "detail", 4.6, 0.3, [[1.3, 0.246], [1.33, 0.21], [1.4, 0.2]], "bottom"));
    out.push(WL("duvet-seam", "bed-surface", "texture", 6.1, 0.2, [[0.52, -0.02], [1.7, -0.02]], "top"));
  }
  return out;
}
const PILLOW = smooth([[0.1, 0.0], [0.11, 0.09], [0.17, 0.12], [0.26, 0.088], [0.34, 0.058], [0.43, 0.074], [0.52, 0.112], [0.6, 0.118], [0.645, 0.07], [0.635, 0.0]], true, 0.4);
function pillowShapes(scale) {
  const out = [WP("pillow", "bed-surface", "body", 1, TONES.pillow, PILLOW, "pillow")];
  if (scale <= 100) out.push(WL("pillow-crease", "bed-surface", "detail", 1.1, 0.3, [[0.45, 0.074], [0.52, 0.06], [0.6, 0.06]], "pillow"));
  return out;
}
const KEEP = {
  500: { 200: ["head", "hair", "hand-r", "shoe-r", "shoe-toe-r"] },
  200: { 200: ["head", "hair", "hand-r", "forearm-r", "sleeve-r", "shoe-r", "shoe-toe-r", "shoe-l", "shoe-toe-l"] },
  100: { 100: ["head", "hair", "neck", "ear", "hand-r", "thumb-r", "forearm-r", "sleeve-r", "shoe-r", "shoe-toe-r", "shoe-l", "shoe-toe-l"] },
  50: { 50: ["head", "hair", "neck", "ear", "eye", "mouth-line", "hand-r", "thumb-r", "fingers-r-0", "fingers-r-1", "fingers-r-2", "forearm-r", "sleeve-r", "cuff-r", "shoe-rear-r", "shoe-toe-r", "shoe-rear-l", "shoe-toe-l"] },
};
function bodyShapes(scale) {
  const out = [];
  const [[from, ids]] = Object.entries(KEEP[scale]);
  for (const s of reader.representations[from].geometry.elevation) {
    if (!ids.includes(s.id)) continue;
    const n = scaleShape(s, K);
    if (n.region === "sweater") n.region = "top";
    if (n.region === "shoes") n.region = "feet-skin";
    if (n.id.startsWith("shoe")) n.id = n.id.replace("shoe", "foot");
    delete n.hatch;
    delete n.material;
    if (n.role === "body" && TONES[n.region] !== undefined) n.tone = TONES[n.region];
    if (n.id.startsWith("forearm")) n.region = "hand-skin", n.tone = TONES["hand-skin"];
    if (n.id.startsWith("cuff")) n.region = "top";
    if (n.id === "sleeve-r") n.z = 6.5;
    if (n.id === "forearm-r") n.z = 6.6;
    if (n.id === "hand-r") n.z = 6.7;
    if (n.id.startsWith("thumb") || n.id.startsWith("fingers")) n.z = 6.72;
    out.push(n);
  }
  if (scale >= 100 || scale === 200) {}
  if (true) {
    out.push(WP("shin-bare", "shank-r", "body", 3, TONES["feet-skin"], rpoly([[1.66, -0.012], [1.84, -0.02], [1.87, 0.03], [1.84, 0.085], [1.66, 0.09]], 0.012), "feet-skin"));
  }
  return out;
}
//#endregion

const PENS = { 500: { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, 200: { body: 0.2, detail: 0.13, texture: null, glazing: 0.13, ground: 0.3 }, 100: reader.representations["100"].pens, 50: reader.representations["50"].pens };
const dec = (sc) => ({
  500: {
    silhouette: txt("A low mound of duvet with a draped side, the head on the pillow and two bare feet: only the mound, the head with hair, the near hand and the feet remain.", "Ein flacher Deckenhügel mit herabhängender Seite, der Kopf auf dem Kissen und zwei nackte Füße: nur Hügel, Kopf mit Haar, nahe Hand und Füße bleiben."),
    retained: txt("The mound profile that changes between lying on the back and on the side, pillow, bare feet.", "Das Hügelprofil, das zwischen Rücken- und Seitenlage wechselt, Kissen, nackte Füße."),
    merged: txt("Skirt and mound pieces share one tone; the hand is one mitten.", "Behang und Hügelteile teilen einen Ton; die Hand ist ein Fäustling."),
    omitted: txt("Folds, hem, ear, neck, pillow crease, toes and every internal edge.", "Falten, Saum, Ohr, Hals, Kissenfalte, Zehen und jede Innenkante."),
    thinElements: txt("None thinner than the 0.18 mm pen at paper size; the duvet hem line is dropped.", "Nichts dünner als der 0,18-mm-Stift auf Papier; die Saumlinie entfällt."),
    hatching: txt("None.", "Keine."),
    styles: txt("Pen 0.18 mm, no detail pen; duvet light, hair dark, skin mid-light.", "Stift 0,18 mm, kein Detailstift; Decke hell, Haar dunkel, Haut mittelhell."),
    overlays: txt("Mound pieces carry the pyjama-top and pyjama-short regions, the skirt the same regions; head, hair, hand, feet and pillow keep their own.", "Hügelteile tragen die Regionen Schlafanzugoberteil und -hose, der Behang dieselben; Kopf, Haar, Hand, Füße und Kissen behalten eigene."),
  },
  200: {
    silhouette: txt("Duvet mound in four pieces (chest, hips, thighs, shins) with the hanging skirt, head with loose hair on the pillow, forearm and hand lying on the duvet, bare lower legs and feet.", "Deckenhügel in vier Teilen (Brust, Hüfte, Oberschenkel, Unterschenkel) mit dem Behang, Kopf mit offenem Haar auf dem Kissen, auf der Decke liegender Unterarm samt Hand, nackte Unterschenkel und Füße."),
    retained: txt("Mound profile, pyjama sleeve, hand, toe lines of the feet.", "Hügelprofil, Schlafanzugärmel, Hand, Zehenlinien der Füße."),
    merged: txt("Face and neck are one skin shape; thumb and fingers are one mitten.", "Gesicht und Hals sind eine Hautform; Daumen und Finger sind ein Fäustling."),
    omitted: txt("Eye, ear, folds, hem and pillow crease.", "Auge, Ohr, Falten, Saum und Kissenfalte."),
    thinElements: txt("Duvet hem line 0.13 mm dropped; skirt 0.22 m stays a closed outline.", "Die Saumlinie (0,13 mm) entfällt; der Behang (0,22 m) bleibt eine geschlossene Kontur."),
    hatching: txt("None.", "Keine."),
    styles: txt("Body 0.2 mm, detail 0.13 mm; duvet light, pillow lightest, hair dark.", "Körper 0,2 mm, Detail 0,13 mm; Decke hell, Kissen am hellsten, Haar dunkel."),
    overlays: txt("One region per surface: face-skin, hair, hand-skin, top, bottom, feet-skin, pillow.", "Eine Region je Fläche: Gesichtshaut, Haar, Handhaut, Oberteil, Hose, Fußhaut, Kissen."),
  },
  100: {
    silhouette: txt("Articulated sleeper: duvet pieces with hem and chest fold, face profile with ear on the creased pillow, hair mass, pyjama sleeve, forearm and mitten hand with thumb on the duvet, bare shins and feet with toe forms.", "Gegliederte schlafende Person: Deckenteile mit Saum und Brustfalte, Gesichtsprofil mit Ohr auf dem gefalteten Kissen, Haarmasse, Schlafanzugärmel, Unterarm und Fäustlingshand mit Daumen auf der Decke, nackte Schienbeine und Füße mit Zehenform."),
    retained: txt("Ear, neck, hem, chest fold and pillow crease.", "Ohr, Hals, Saum, Brustfalte und Kissenfalte."),
    merged: txt("Finger groups are one mitten.", "Fingergruppen sind ein Fäustling."),
    omitted: txt("Eye, mouth line, hips and knee folds, toenails.", "Auge, Mundlinie, Hüft- und Kniefalten, Zehennägel."),
    thinElements: txt("Fold lines 0.1 mm are 10 mm wide strokes at full size; they are drawn as detail lines.", "Faltenlinien von 0,1 mm entsprechen 10 mm Strichbreite in Originalgröße; sie werden als Detaillinien gezeichnet."),
    hatching: txt("None.", "Keine."),
    styles: txt("Body 0.2 mm, detail 0.1 mm; same tone map as 1:50.", "Körper 0,2 mm, Detail 0,1 mm; dieselbe Tonzuordnung wie 1:50."),
    overlays: txt("Regions as in 1:200; fold and hem lines take the colour of the duvet region.", "Regionen wie in 1:200; Falten- und Saumlinien übernehmen die Farbe der Deckenregion."),
  },
  50: {
    silhouette: txt("Fully drawn sleeper: closed eye and mouth line, ear, hair on the pillow, pyjama sleeve with cuff, finger groups on the duvet, duvet with hem, seam and three fold lines, bare shins and feet.", "Voll gezeichnete schlafende Person: geschlossenes Auge und Mundlinie, Ohr, Haar auf dem Kissen, Schlafanzugärmel mit Bündchen, Fingergruppen auf der Decke, Decke mit Saum, Naht und drei Faltenlinien, nackte Schienbeine und Füße."),
    retained: txt("Every authored detail of face, hand and feet.", "Jedes gezeichnete Detail von Gesicht, Hand und Füßen."),
    merged: txt("Nothing merged.", "Nichts verschmolzen."),
    omitted: txt("Individual toes, toenails, duvet stitching.", "Einzelne Zehen, Zehennägel, Deckensteppung."),
    thinElements: txt("Eye line 0.1 mm; duvet seam drawn as 0.1 mm texture line.", "Augenlinie 0,1 mm; Deckennaht als 0,1-mm-Strukturlinie."),
    hatching: txt("None.", "Keine."),
    styles: txt("Body 0.25 mm, detail 0.13 mm, texture 0.10 mm.", "Körper 0,25 mm, Detail 0,13 mm, Struktur 0,10 mm."),
    overlays: txt("Same regions as 1:200; the cuff belongs to the top region.", "Dieselben Regionen wie in 1:200; das Bündchen gehört zur Oberteilregion."),
  },
}[sc]);
const reps = {};
for (const sc of [500, 200, 100, 50]) {
  reps[String(sc)] = { status: "authored", denominator: sc, validRange: reader.representations[String(sc)].validRange, pens: { ...PENS[sc] }, hatches: [], geometry: { elevation: [...pillowShapes(sc), ...duvetShapes(sc), ...bodyShapes(sc)] }, decisions: dec(sc) };
  { let top = []; for (const e of reps[String(sc)].geometry.elevation) { const F = fk(bones, { "arm-r": { angleDeg: 14 }, "forearm-r": { angleDeg: -2 }, "hand-r": { angleDeg: -4 } }); const nums = (e.d ?? "").match(/-?\d*\.?\d+/g)?.map(Number) ?? (e.points ?? []).flat(); const t = F.get(e.bone); const pts = e.d ? nums.reduce((a, v, i) => (i % 2 ? (a[a.length - 1].push(v), a) : (a.push([v]), a)), []) : e.points ?? []; for (const [x, y] of pts) { const w = toWorld(t, [x, y]); top.push([r4(w[1] - Y0), r4(w[0]), e.id]); } } top.sort((a, b) => b[0] - a[0]); const lo = [...top].sort((a, b) => a[0] - b[0])[0]; console.error("[DEBUG]", sc, "top", JSON.stringify(top[0]), "bottom", JSON.stringify(lo), "minx", JSON.stringify([...top].sort((a, b) => a[1] - b[1])[0]), "maxx", JSON.stringify([...top].sort((a, b) => b[1] - a[1])[0])); }
}

//#region Poses
const REVS = ["pelvis", "torso", "neck", "head", "arm-r", "forearm-r", "hand-r", "arm-l", "forearm-l", "hand-l", "thigh-l", "shank-l", "foot-l", "toe-l", "thigh-r", "shank-r", "foot-r", "toe-r"];
const base = () => Object.fromEntries(REVS.map((id) => [id, { angleDeg: 0 }]));
const POSE_OFFS = {
  supine: { "arm-r": 10, "forearm-r": -1, "hand-r": -3, "arm-l": 5, neck: 0, head: 0 },
  side: { "arm-r": 22, "forearm-r": -8, "hand-r": 0, "arm-l": 20, "thigh-l": 32, "thigh-r": 32, "shank-l": -66, "shank-r": -66, "foot-l": -10, "foot-r": -10, neck: 4, head: -6, torso: 2, "hip-lift": { dy: 0.02 } },
  curled: { "arm-r": 30, "forearm-r": -15, "hand-r": 0, "arm-l": 25, "thigh-l": 58, "thigh-r": 58, "shank-l": -100, "shank-r": -100, "foot-l": -14, "foot-r": -14, neck: 8, head: -10, torso: 5, pelvis: 3, "hip-lift": { dy: 0.035 } },
};
const poses = Object.entries(POSE_OFFS).map(([id, o]) => {
  const bs = base();
  for (const [k, v] of Object.entries(o)) bs[k] = typeof v === "number" ? { angleDeg: v } : v;
  return { id, bs };
});
const TEXT = {
  supine: [txt("Lying on the back", "Rückenlage"), txt("Flat on the back, head on the pillow, legs stretched under the duvet, one hand resting on the duvet over the chest.", "Flach auf dem Rücken, der Kopf auf dem Kissen, die Beine gestreckt unter der Decke, eine Hand auf der Decke über der Brust.")],
  side: [txt("Lying on the side", "Seitenlage"), txt("On the side with hips and knees drawn up; in this elevation the near hand lies on the duvet and the knee ridge rises, the contact area with the mattress is larger.", "Auf der Seite mit angezogenen Hüften und Knien; in dieser Ansicht liegt die nahe Hand auf der Decke und die Knie bilden einen Rücken, die Auflagefläche auf der Matratze ist größer.")],
  curled: [txt("Curled up on the side", "Eingerollte Seitenlage"), txt("Drawn-up knees and a deeper hip bend: the duvet over the legs mounds high and the feet draw in.", "Hochgezogene Knie und stärker gebeugte Hüfte: die Decke über den Beinen wölbt sich hoch und die Füße ziehen ein.")],
};
const bind = (e, s = 0) => ({ exposedFraction: e, sunlitFraction: s });
const bindFor = (id) => {
  const lat = id !== "supine";
  const c = lat ? 0.34 : 0.22;
  return {
    activityMet: 0.8,
    breathing: lat ? "sleep-side" : "sleep-supine",
    regions: {
      "face-skin": bind(1), hair: bind(0.9), "hand-skin": bind(1),
      top: { ...bind(1 - c), contact: { fraction: c, tempC: 29 } },
      bottom: { ...bind(1 - c), contact: { fraction: c, tempC: 29 } },
      "feet-skin": bind(1), pillow: bind(0.7),
    },
    emitters: { "body-plume": lat ? 0.5 : 0.45 },
  };
};
const CONTACTS = { supine: ["head-pillow", "shoulders-mattress", "back-mattress", "pelvis-mattress", "calf-mattress", "heel-mattress"], side: ["head-pillow", "shoulders-mattress", "back-mattress", "pelvis-mattress"], curled: ["head-pillow", "shoulders-mattress", "back-mattress", "pelvis-mattress"] };
const poseDocs = poses.map(({ id, bs }) => ({ id, title: TEXT[id][0], description: TEXT[id][1], bones: bs, ground: "authored", contacts: CONTACTS[id], thermal: bindFor(id), loads: [] }));
const spans = (T, ids) => ids.map((a) => ({ anchor: a, fromS: 0, toS: T }));
const oscillate = (key, over) => ({ t: key, pose: "supine", ease: "smooth", bones: over });
const clipBreathing = {
  id: "breathing",
  title: txt("Sleeping breath", "Schlafatmung"),
  description: txt("A looping calm sleep: the chest rises and falls with the breath (derived from the sleep breathing pattern), the hand on the duvet follows by a fraction of a degree and the head settles slightly into the pillow.", "Ruhiger Schlaf als Schleife: die Brust hebt und senkt sich mit dem Atem (aus dem Schlafatemmuster abgeleitet), die Hand auf der Decke folgt um Bruchteile eines Grades und der Kopf sinkt leicht ins Kissen."),
  durationS: 4.8,
  fps: 30,
  loop: true,
  rootMotion: "none",
  keys: [oscillate(0, {}), oscillate(2.4, { "forearm-r": { angleDeg: -0.5 }, head: { angleDeg: 1 }, "hip-lift": { dy: 0 } }), oscillate(4.8, {})].map((k, i) => (i === 1 ? k : { ...k })),
  contacts: spans(4.8, CONTACTS.supine),
  phases: [{ id: "inhale", fromS: 0, toS: 2.4, title: txt("Chest rising", "Brust hebt sich") }, { id: "exhale", fromS: 2.4, toS: 4.8, title: txt("Chest falling", "Brust senkt sich") }],
  events: [{ t: 2.4, kind: "motion", title: txt("Hand lifts at the top of the breath", "Hand hebt sich am Atemhöhepunkt") }],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4.8, v: 0 }] }],
};
const clipTurn = {
  id: "turn-over",
  title: txt("Turning onto the side", "Auf die Seite drehen"),
  description: txt("From lying on the back she draws her knees up, turns onto the side and curls up under the duvet.", "Aus der Rückenlage zieht sie die Knie an, dreht sich auf die Seite und rollt sich unter der Decke ein."),
  durationS: 8,
  fps: 30,
  loop: false,
  rootMotion: "none",
  keys: [{ t: 0, pose: "supine", ease: "smooth" }, { t: 2, pose: "supine", ease: "smooth" }, { t: 4.5, pose: "side", ease: "smooth" }, { t: 6.5, pose: "curled", ease: "smooth" }, { t: 8, pose: "curled", ease: "smooth" }],
  contacts: spans(2, CONTACTS.supine).concat(spans(8, CONTACTS.curled).slice(0, 4).map((c) => ({ ...c, fromS: 2 }))),
  phases: [{ id: "lie", fromS: 0, toS: 2, title: txt("Lying still", "Ruhig liegen") }, { id: "turn", fromS: 2, toS: 6.5, title: txt("Turning", "Drehen") }, { id: "curl", fromS: 6.5, toS: 8, title: txt("Curled up", "Eingerollt") }],
  events: [{ t: 2, kind: "contact-off", title: txt("Calves leave the mattress", "Waden lösen sich von der Matratze"), target: "calf-mattress" }],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 3.5, v: 1 }, { t: 8, v: 1 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 0.8 }, { t: 3.5, v: 0.95 }, { t: 6.5, v: 0.8 }] }],
};
//#endregion

//#region Thermal
const AREA_F = 1.742 / 1.692;
const RB = Object.fromEntries(reader.thermal.regions.map((r) => [r.id, r]));
const lay = (id, en, de, kind, material, thicknessM, R, hc, reference = "iso-9920-2007") => ({ id, title: txt(en, de), kind, material, thicknessM, resistanceM2KW: r4(R), vapourResistanceM2PaW: r4(R / (16.5e-3 * 0.38)), heatCapacityJM2K: hc, source: "authored-estimate", reference });
const duvetLayer = lay("duvet", "Duvet with enclosed air (about 1.6 clo)", "Bettdecke mit eingeschlossener Luft (etwa 1,6 clo)", "insulation", "duvet-fill", 0.05, 1.6 * 0.155, 1200, "lin-deng-2008");
const regions = [
  { ...RB["face-skin"], areaM2: r4(RB["face-skin"].areaM2 * AREA_F) },
  { ...RB["hair"], title: txt("Loose hair on the pillow", "Offenes Haar auf dem Kissen"), areaM2: r4(RB["hair"].areaM2 * AREA_F) },
  { ...RB["hand-skin"], title: txt("Hand and forearm on the duvet", "Hand und Unterarm auf der Decke"), areaM2: r4(RB["hand-skin"].areaM2 * AREA_F) },
  { ...RB["sweater"], id: "top", title: txt("Duvet over the pyjama top, torso and arms", "Bettdecke über Schlafanzugoberteil, Rumpf und Armen"), material: "duvet-cover", areaM2: 1.02, characteristicLengthM: 0.4, layers: [lay("pyjama-top", "Cotton pyjama top", "Baumwoll-Schlafanzugoberteil", "clothing", "cotton-underwear", 0.0008, 0.12 * 0.155 * 1.144, 160), duvetLayer], heatCapacityJM2K: 14360, skyViewFactor: 0.45, geometry: "horizontal-plate-up" },
  { ...RB["jeans"], id: "bottom", title: txt("Duvet over the pyjama shorts and legs", "Bettdecke über Schlafanzughose und Beinen"), material: "duvet-cover", areaM2: 0.62, characteristicLengthM: 0.25, layers: [lay("pyjama-shorts", "Cotton pyjama shorts", "Baumwoll-Schlafanzughose", "clothing", "cotton-underwear", 0.0008, 0.06 * 0.155 * 1.144, 120), duvetLayer], heatCapacityJM2K: 14320, skyViewFactor: 0.4, geometry: "horizontal-plate-up" },
  { ...RB["shoes"], id: "feet-skin", title: txt("Bare feet and lower shins", "Nackte Füße und Unterschenkel"), kind: "skin", material: "skin", areaM2: 0.13, layers: [], heatCapacityJM2K: 13000 },
  { id: "pillow", title: txt("Pillow", "Kopfkissen"), kind: "surface", model: "passive", material: "duvet-fill", areaM2: 0.32, characteristicLengthM: 0.4, geometry: "horizontal-plate-up", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 1500, source: "authored-estimate" },
];
regions[3].layers[0].reference = "iso-9920-2007";
const respiration = clone(reader.thermal.respiration);
respiration.patterns = [
  { id: "sleep-supine", title: txt("Sleep on the back", "Schlaf in Rückenlage"), rateBpm: 12, tidalVolumeL: 0.5, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.05, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.12, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
  { id: "sleep-side", title: txt("Sleep on the side", "Schlaf in Seitenlage"), rateBpm: 12, tidalVolumeL: 0.45, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.05, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.12, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
];
respiration.exhaledTempC = 34;
respiration.motion = [{ bone: "torso", angleDeg: 0.8, dy: 0.006 }];
const emitters = clone(reader.thermal.emitters);
const plume = emitters.find((e) => e.id === "body-plume");
Object.assign(plume, { anchor: "chest-plume", directionDeg: -90, region: "top", diameterM: 0.3 });
const thermal = {
  environment: { airTempC: 17, meanRadiantTempC: 17, skyTempC: 17, groundTempC: 17, relativeHumidity: 0.5, windMS: 0.05, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0.5, pressurePa: 101325 },
  baseMet: 0.8,
  body: { species: "human", massKg: 64, skinAreaM2: 1.742, basalMetabolismW: 66, metWM2: 58.15, coreSetPointC: 36.8, skinSetPointC: 34.1, sweating: true, panting: false, maxSkinBloodFlowLM2H: 90, minSkinBloodFlowLM2H: 1, source: "authored-estimate", reference: "gagge-1971" },
  respiration,
  operating: [],
  passengers: [],
  regions,
  emitters,
  assumptions: [
    txt("A 1.70 m, 64 kg woman: DuBois and DuBois (1916) skin area 1.742 m2, basal metabolism 66 W after Mifflin et al. (1990) for 35 years; segment masses are the seated-reader de Leva fractions scaled to 64.6 kg including 0.6 kg of pyjamas.", "Eine 1,70 m große, 64 kg schwere Frau: Hautfläche 1,742 m2 nach DuBois und DuBois (1916), Grundumsatz 66 W nach Mifflin et al. (1990) für 35 Jahre; Segmentmassen sind die de-Leva-Anteile der Lesenden auf 64,6 kg einschließlich 0,6 kg Schlafanzug skaliert."),
    txt("Sleeping is 0.8 met (ISO 8996 gives 0.7 met for sleeping, 0.8 is used for the light sleep of the energy clips); the turning clip raises it to 0.95 met for a few seconds.", "Schlafen sind 0,8 met (ISO 8996 nennt 0,7 met für Schlafen, 0,8 gilt für den leichten Schlaf der Energieclips); der Dreh-Clip hebt den Wert für einige Sekunden auf 0,95 met."),
    txt("Light sleepwear (pyjama top 0.12 clo, shorts 0.06 clo) under a duvet of about 1.6 clo including the enclosed air layer (bedding insulation 1.4 to 3 clo; Lin and Deng 2008, authored estimate). The mattress top is the contact surface at 29 degrees C.", "Leichte Schlafkleidung (Oberteil 0,12 clo, Hose 0,06 clo) unter einer Bettdecke von etwa 1,6 clo einschließlich der eingeschlossenen Luftschicht (Bettzeugdämmung 1,4 bis 3 clo; Lin und Deng 2008, Schätzwert). Die Matratzenoberseite ist die Kontaktfläche mit 29 Grad C."),
    txt("The figure lies on the mattress datum y = 0.46 m of the single-bed in its asleep pose and has no bed of its own: draw a single-bed cast with the same origin beneath it. In the elevation both poses keep the profile convention; the side pose shows the knee ridge, the larger mattress contact is in the thermal binding.", "Die Figur liegt auf dem Matratzenbezug y = 0,46 m des Einzelbetts in der Haltung asleep und bringt kein eigenes Bett mit: darunter wird ein Einzelbett-Cast mit demselben Ursprung gezeichnet. In der Ansicht bleibt es bei der Profilkonvention; die Seitenlage zeigt den Kniehügel, die größere Matratzenauflage steckt in der thermischen Bindung."),
  ],
};
//#endregion

const refs = clone(reader.provenance.references);
refs.push({ id: "lin-deng-2008", citation: "Lin, Z. and Deng, S. (2008). A study on the thermal comfort in sleeping environments in the subtropics: measuring the total insulation values for the bedding systems commonly used in the subtropics. Building and Environment 43(5), 905-916." });
const mat = (id, en, de, emissivity, abs, source, reference) => ({ id, title: txt(en, de), emissivity, solarAbsorptivity: abs, source, ...(reference ? { reference } : {}) });
const asset = {
  id: "sleeping-person",
  category: "humans",
  title: txt("Sleeping Person", "Schlafende Person"),
  summary: txt("A woman asleep under a duvet with her head on the pillow, bare feet at the foot end and one hand on the duvet: lying on the back, on the side and curled up, a looping breathing clip and a turn-over; at 0.8 met and made to lie on the catalog single-bed.", "Eine Frau schläft unter einer Bettdecke, der Kopf auf dem Kissen, nackte Füße am Fußende und eine Hand auf der Decke: in Rücken-, Seiten- und eingerollter Lage, mit einem Atemclip als Schleife und einem Umdrehen; bei 0,8 met und gemacht, um auf dem Katalog-Einzelbett zu liegen."),
  tags: ["human", "adult", "woman", "sleeping", "lying", "bed", "duvet", "internal-gains", "night"],
  provenance: { author: "Semio staffage content worker A1", method: "handcrafted", created: "2026-11-01", references: refs, notes: txt("A 1.70 m, 64 kg woman lying as a chain of the seated-reader skeleton (hip joint at x = 1.05 m from the bed head); head, hands and feet are the reader's drawn forms, the duvet in four pieces with a hanging skirt, the pillow and the thermal model are new.", "Eine 1,70 m große, 64 kg schwere Frau, liegend als Kette aus dem Skelett der Lesenden (Hüftgelenk bei x = 1,05 m vom Bettkopf); Kopf, Hände und Füße sind die gezeichneten Formen der Lesenden, die Decke in vier Teilen mit hängendem Behang, das Kissen und das Thermikmodell sind neu.") },
  dimensions: { heightM: 0.45, widthM: 1.9, depthM: 0.4, massKg: totalMass },
  materials: [...reader.materials.filter((m) => ["skin", "hair", "cotton-underwear"].includes(m.id)), mat("duvet-fill", "Duvet and pillow filling", "Decken- und Kissenfüllung", 0.92, 0.5, "authored-estimate"), mat("duvet-cover", "Cotton duvet cover", "Baumwoll-Bettbezug", 0.93, 0.45, "literature", "ashrae-2021")],
  rig: { kind: "anatomical", root: "bed-surface", bones },
  anchors,
  representations: reps,
  symbols: [],
  poses: poseDocs,
  clips: [clipBreathing, clipTurn],
  thermal,
  structure: { gravityMS2: 9.80665, dynamics: "inertial", supports: [], assumptions: [txt("The mattress and the pillow are unilateral contacts and the bed is rigid and fixed; segments are point masses at the de Leva centres.", "Matratze und Kissen sind einseitige Kontakte und das Bett ist starr und fest; Segmente sind Punktmassen in den Schwerpunkten nach de Leva.")], clinical: false },
  presentation: { defaultPose: "supine", defaultClip: "breathing", defaultScale: 100, viewpoints: ["elevation"] },
  exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"],
  completion: "authored",
};
const outDir = `${S}/🗂️catalog/👤️humans/sleeping-person`;
mkdirSync(outDir, { recursive: true });
writeFileSync(`${outDir}/🔣️.json`, JSON.stringify({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset }, null, 2) + "\n");
console.error(`[DEBUG] wrote sleeper mass ${totalMass}`);
