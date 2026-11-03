// Builds S/🗂️catalog/👤️humans/seated-office-worker/🔣️.json. The body is the seated-reader skeleton scaled to the 1.78 m design (K = 1/0.944);
// everything that makes this person an office worker (clothing, short hair, chair sub-assembly, desk fragment, laptop, poses, clips,
// thermal model) is authored here. Run: repo tools exec bun -- worker.mjs
import { writeFileSync, mkdirSync } from "node:fs";
import { S, r4, clone, txt, load, scaleShape, fk, toWorld, ik2, wrap, smooth, rpoly, mapPts, mapPath, shape, poly, path } from "./lib.mjs";

const reader = load("👤️humans/seated-reader");
const chairA = load("🛋️furniture/office-swivel-chair");
const K = 1 / 0.944;
const DROP_BONES = new Set(["seat", "pad", "page", "cup"]);
const LEGS = ["l", "r"];

//#region Rig
const bones = [];
bones.push({ id: "floor", parent: null, title: txt("Floor datum", "Bodenbezug"), x: 0, y: 0, angleDeg: 0, lengthM: 1, joint: { type: "fixed" } });
for (const b of chairA.rig.bones) bones.push({ ...clone(b), id: `chair-${b.id}`, parent: b.parent ? `chair-${b.parent}` : "floor", ...(b.parent ? {} : { x: 0.04 }) });
const bodyMassSum = reader.rig.bones.filter((b) => b.mass && !DROP_BONES.has(b.id)).reduce((s, b) => s + b.mass.kg, 0);
const MASS_F = 78.6 / bodyMassSum;
const TITLES = {
  torso: txt("Torso with shirt", "Rumpf mit Hemd"),
  head: txt("Head with short hair", "Kopf mit Kurzhaarschnitt"),
  "foot-l": txt("Left foot and leather shoe", "Linker Fuß mit Lederschuh"),
  "foot-r": txt("Right foot and leather shoe", "Rechter Fuß mit Lederschuh"),
};
for (const b of reader.rig.bones) {
  if (DROP_BONES.has(b.id)) continue;
  const n = clone(b);
  if (b.id === "hip-lift") Object.assign(n, { parent: "chair-seat", x: 0, y: 0.125, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 90, minM: -0.1, maxM: 0.1 } });
  else if (b.id === "hip-shift") Object.assign(n, { x: 0, y: 0, joint: { type: "prismatic", axisDeg: 0, minM: -0.15, maxM: 0.15 } });
  else Object.assign(n, { x: r4(b.x * K), y: r4(b.y * K), lengthM: r4(b.lengthM * K) });
  if (TITLES[b.id]) n.title = TITLES[b.id];
  if (n.mass) n.mass = { ...n.mass, kg: r4(n.mass.kg * MASS_F), comX: r4(n.mass.comX * K), comY: r4(n.mass.comY * K) };
  bones.push(n);
}
const fixed = { type: "fixed" };
const pm = (kg, comX) => ({ kg, comX, comY: 0, distribution: "point", source: "authored-estimate" });
bones.push({ id: "laptop", parent: "floor", title: txt("Laptop base with keyboard", "Laptop-Unterteil mit Tastatur"), x: 0.74, y: 0.74, angleDeg: 180, lengthM: 0.32, joint: fixed, mass: pm(1.2, 0.16) });
bones.push({ id: "screen", parent: "laptop", title: txt("Laptop lid with display", "Laptop-Deckel mit Display"), x: 0, y: 0, angleDeg: -110, lengthM: 0.215, joint: { type: "revolute", minDeg: -25, maxDeg: 110 }, mass: pm(0.5, 0.1) });
bones.push({ id: "desk-top", parent: "floor", title: txt("Desk top fragment", "Schreibtischplatte (Ausschnitt)"), x: 0.28, y: 0.695, angleDeg: 0, lengthM: 0.78, joint: fixed, mass: { ...pm(11, 0.39), comY: 0.0125 } });
bones.push({ id: "desk-leg", parent: "floor", title: txt("Desk end panel", "Schreibtisch-Stirnwand"), x: 1.02, y: 0, angleDeg: 90, lengthM: 0.695, joint: fixed, mass: pm(5, 0.35) });
const totalMass = r4(bones.reduce((s, b) => s + (b.mass?.kg ?? 0), 0));
const REST = fk(bones);
//#endregion Rig

//#region Anchors
const ANCHOR_TITLES = {
  seat: txt("Seat bones on the chair seat", "Sitzbeinhöcker auf dem Stuhlsitz"),
  "thigh-support": txt("Thighs on the front of the seat", "Oberschenkel auf der Sitzvorderkante"),
  backrest: txt("Back against the backrest", "Rücken an der Rückenlehne"),
};
const anchors = reader.anchors
  .filter((a) => a.id !== "pad-grip")
  .map((a) => {
    const n = clone(a);
    n.x = r4(a.x * K);
    n.y = r4(a.y * K);
    if (n.footprintLengthM) n.footprintLengthM = r4(n.footprintLengthM * K);
    if (n.footprintWidthM) n.footprintWidthM = r4(n.footprintWidthM * K);
    if (ANCHOR_TITLES[a.id]) n.title = ANCHOR_TITLES[a.id];
    return n;
  });
anchors.push({ id: "laptop-vent", bone: "laptop", x: 0.03, y: 0, kind: "emitter", title: txt("Laptop exhaust vent", "Lüftungsschlitz des Laptops") });
//#endregion Anchors

//#region Hand-drawn shapes
const HAIR = smooth([[0.1314, -0.0586], [0.15, -0.062], [0.1655, -0.0455], [0.1745, -0.0215], [0.1795, 0.0095], [0.1735, 0.0405], [0.1615, 0.0705], [0.1455, 0.0935], [0.1215, 0.109], [0.0885, 0.1155], [0.0615, 0.1105], [0.0455, 0.0985], [0.0365, 0.0805], [0.0555, 0.0605], [0.0775, 0.0485], [0.0985, 0.0385], [0.1165, 0.0115], [0.1265, -0.0295]], true, 0.5);
const HAIR_PART = [[0.1418, -0.0575], [0.1575, -0.0285], [0.1655, 0.0155]];
const COLLAR = rpoly([[-0.026, -0.062], [0.034, -0.054], [0.04, 0.058], [-0.026, 0.07]], 0.011);
const COLLAR_FOLD = [[0.0345, -0.053], [0.012, 0.0], [0.04, 0.056]];
const TONES = { "face-skin": 0.82, "hand-skin": 0.82, hair: 0.22, shirt: 0.74, trousers: 0.3, shoes: 0.16 };
const REGION_RENAME = { sweater: "shirt", jeans: "trousers" };
const GONE_REGIONS = new Set(["bench", "cup", "notebook"]);
const GONE_IDS = new Set(["hair-bun", "hair-tie", "hair", "cowl", "sweater-hem", "sweater-rib", "sweater-rib-1", "sweater-rib-2", "sweater-rib-3"]);
//#endregion

//#region Desk, laptop and chair shapes (world coordinates at rest, mapped into the bone frames)
const frame = (bone) => REST.get(bone);
const WPATH = (id, bone, role, z, tone, d, region, extra = {}) => shape(id, bone, role, z, tone, path(mapPath(frame(bone), d)), { region, ...extra });
const WLINE = (id, bone, role, z, tone, pts, region, extra = {}) => shape(id, bone, role, z, tone, poly(mapPts(frame(bone), pts), false), { region, filled: false, ...extra });
const WPOLY = (id, bone, role, z, tone, pts, region, extra = {}) => shape(id, bone, role, z, tone, poly(mapPts(frame(bone), pts), true), { region, ...extra });

function stationShapes(scale) {
  const out = [];
  const deskTop = rpoly([[0.28, 0.695], [1.06, 0.695], [1.06, 0.72], [0.28, 0.72]], [0.008, 0.003, 0.003, 0.008]);
  if (scale === 500) {
    out.push(WPOLY("sil-desk", "desk-top", "body", 1.2, 0.45, [[0.28, 0.695], [1.0, 0.695], [1.0, 0], [1.04, 0], [1.04, 0.695], [1.06, 0.695], [1.06, 0.72], [0.28, 0.72]], "desk"));
    out.push(WPOLY("sil-laptop", "laptop", "body", 2, 0.6, [[0.42, 0.72], [0.74, 0.72], [0.74, 0.741], [0.7455, 0.741], [0.81, 0.9], [0.8, 0.905], [0.7345, 0.748], [0.42, 0.7345]], "laptop"));
    return out;
  }
  out.push(WPATH("desk-top", "desk-top", "body", 1.2, 0.45, deskTop, "desk"));
  out.push(WPATH("desk-end-panel", "desk-leg", "body", 1.1, 0.4, rpoly([[1.0, 0], [1.04, 0], [1.04, 0.695], [1.0, 0.695]], 0.003), "desk"));
  if (scale <= 100) out.push(WLINE("desk-edge-line", "desk-top", "detail", 1.25, 0.3, [[0.286, 0.7105], [1.054, 0.7105]], "desk"));
  if (scale === 50) {
    out.push(WPATH("desk-foot-plate", "desk-leg", "body", 1.1, 0.35, rpoly([[0.97, 0], [1.06, 0], [1.06, 0.012], [0.97, 0.012]], 0.004), "desk"));
    out.push(WLINE("desk-apron-line", "desk-top", "detail", 1.25, 0.3, [[0.3, 0.7005], [1.0, 0.7005]], "desk"));
  }
  const base = rpoly([[0.42, 0.72], [0.74, 0.72], [0.74, 0.741], [0.42, 0.7345]], [0.003, 0.002, 0.003, 0.003]);
  out.push(WPATH("laptop-base", "laptop", "body", 2, 0.6, base, "laptop"));
  out.push(shape("laptop-lid", "screen", "body", 2.1, 0.55, path(rpoly([[0, -0.0045], [0.215, -0.0045], [0.215, 0.0045], [0, 0.0045]], 0.003)), { region: "laptop" }));
  if (scale <= 100) {
    out.push(WLINE("laptop-keys", "laptop", "detail", 2.2, 0.2, [[0.44, 0.7385], [0.7, 0.7405]], "laptop"));
    out.push(shape("laptop-display", "screen", "detail", 2.2, 0.2, poly([[0.012, 0.0035], [0.205, 0.0035]], false), { region: "laptop", filled: false }));
  }
  if (scale === 50) {
    out.push(WLINE("laptop-trackpad", "laptop", "detail", 2.2, 0.2, [[0.52, 0.7368], [0.6, 0.7378]], "laptop"));
    out.push(WLINE("laptop-key-rows", "laptop", "texture", 2.2, 0.15, [[0.45, 0.7375], [0.45, 0.7395], [0.48, 0.7395], [0.48, 0.7375], [0.51, 0.7375], [0.51, 0.7395], [0.54, 0.7395], [0.54, 0.7378]], "laptop"));
    out.push(shape("laptop-hinge", "laptop", "detail", 2.3, 0.3, poly(mapPts(frame("laptop"), [[0.7355, 0.7355], [0.7455, 0.7355], [0.7455, 0.7455], [0.7355, 0.7455]]), true), { region: "laptop", filled: false }));
  }
  return out;
}

function chairShapes(scale) {
  const rep = chairA.representations[String(scale)];
  return rep.geometry.elevation.map((s) => {
    const n = clone(s);
    n.id = `chair-${s.id}`;
    n.bone = `chair-${s.bone}`;
    n.z = /^(chair-)?armrest/.test(s.id) ? r4(5 + s.z / 100) : r4(0.05 + s.z * 0.09);
    return n;
  });
}

function bodyShapes(scale) {
  const rep = reader.representations[String(scale)];
  const out = [];
  for (const s of rep.geometry.elevation) {
    if (DROP_BONES.has(s.bone) || GONE_IDS.has(s.id)) continue;
    const regs = s.regions ?? (s.region ? [s.region] : []);
    if (regs.some((r) => GONE_REGIONS.has(r))) continue;
    const n = scaleShape(s, K);
    n.id = s.id.replace("sweater", "shirt").replace("jeans", "trousers");
    if (n.region) n.region = REGION_RENAME[n.region] ?? n.region;
    if (n.regions) n.regions = n.regions.map((r) => REGION_RENAME[r] ?? r);
    delete n.hatch;
    if (n.id === "sil-torso") {
      const nums = n.d.match(/-?\d*\.?\d+/g).map(Number);
      const xs = nums.filter((_, i) => i % 2 === 0);
      console.error("[DEBUG] sil-torso x range", Math.min(...xs), Math.max(...xs));
      n.d = n.d.replace(/(-?\d*\.?\d+) (-?\d*\.?\d+)/g, (m, x, y) => { const X = parseFloat(x); return X > 0.62 ? `${r4(0.62 + (X - 0.62) * 0.78)} ${y}` : m; });
    }
    if (n.material) n.material = { denim: "wool-blend", "knit-wool": "cotton-shirt", "canvas-sneaker": "leather-shoe" }[n.material] ?? n.material;
    if (n.role === "body" && n.region && TONES[n.region] !== undefined) n.tone = TONES[n.region];
    out.push(n);
  }
  const sc = (pts) => pts.map(([x, y]) => [r4(x * K), r4(y * K)]);
  if (scale !== 500) {
    out.push(shape("hair", "head", "body", 5.5, TONES.hair, path(scaleShape({ d: HAIR }, K).d), { region: "hair" }));
    out.push(shape("collar", "neck", "body", 4.5, TONES.shirt, path(scaleShape({ d: COLLAR }, K).d), { region: "shirt" }));
  }
  if (scale <= 100) {
    out.push(shape("hair-parting", "head", "detail", 5.6, 0.1, poly(sc(HAIR_PART), false), { region: "hair", filled: false }));
    out.push(shape("collar-fold", "neck", "detail", 4.6, 0.1, poly(sc(COLLAR_FOLD), false), { region: "shirt", filled: false }));
  }
  return out;
}
//#endregion

//#region Representation decisions
const PENS = { 500: { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, 200: { body: 0.2, detail: 0.13, texture: null, glazing: 0.13, ground: 0.3 }, 100: reader.representations["100"].pens, 50: reader.representations["50"].pens };
const D = (a, b, c, d) => ({ en: a, de: b, ...{} }) && { en: a, de: b };
function decisions(scale) {
  const t = {
    500: {
      silhouette: txt("One composed silhouette per bone: seated person on a chair at a desk, 3.6 mm tall in total height 0.9 mm per 0.45 m of seat; the chair is reduced to base bar, column, seat band and backrest, the laptop and the desk to one closed outline each.", "Eine zusammengesetzte Silhouette je Knochen: sitzende Person auf einem Stuhl am Schreibtisch; der Stuhl ist auf Fußbalken, Säule, Sitzband und Lehne reduziert, Laptop und Schreibtisch auf je eine geschlossene Kontur."),
      retained: txt("Seated posture with forward-tilted back, forearms reaching the laptop, backrest line, desk top with end panel, open laptop lid.", "Sitzhaltung mit vorgeneigtem Rücken, zum Laptop greifende Unterarme, Lehnenlinie, Tischplatte mit Stirnwand, geöffneter Laptop-Deckel."),
      merged: txt("Head, hair, neck and shirt share the torso silhouette; hand and forearm are one outline; the laptop base and lid are one shape.", "Kopf, Haar, Hals und Hemd teilen die Rumpfsilhouette; Hand und Unterarm sind eine Kontur; Laptop-Unterteil und -Deckel sind eine Form."),
      omitted: txt("Armrest, castors, hair parting, collar, keyboard, seams and every internal edge.", "Armlehne, Rollen, Scheitel, Kragen, Tastatur, Nähte und jede Innenkante."),
      thinElements: txt("Laptop lid 9 mm thick is drawn 9 mm (0.02 mm at paper), so it is merged into the base outline; the desk end panel is 40 mm and stays a closed outline.", "Der 9 mm dicke Laptop-Deckel wird mit der Basis zu einer Kontur verschmolzen; die Stirnwand (40 mm) bleibt eine geschlossene Kontur."),
      hatching: txt("None.", "Keine."),
      styles: txt("Pen 0.13 mm; no detail pen; dark and light solid separate shirt, trousers, shoes and furniture by tone only.", "Stift 0,13 mm; kein Detailstift; dunkel und hell massiv trennen Hemd, Hose, Schuhe und Möbel nur über den Ton."),
      overlays: txt("The torso silhouette lists face skin, hair and shirt as merged regions; chair, desk and laptop keep their own regions.", "Die Rumpfsilhouette führt Gesichtshaut, Haar und Hemd als verschmolzene Regionen; Stuhl, Tisch und Laptop behalten eigene Regionen."),
    },
    200: {
      silhouette: txt("Articulated seated person: shirt body with sleeves, short-haired head, trousers and shoes; chair as base star, column, seat, backrest and armrest; desk top with end panel; open laptop.", "Gegliederte sitzende Person: Hemd mit Ärmeln, Kopf mit Kurzhaar, Hose und Schuhe; Stuhl aus Fußstern, Säule, Sitz, Lehne und Armlehne; Tischplatte mit Stirnwand; geöffneter Laptop."),
      retained: txt("Hair mass, shirt and collar as separate forms, mitten hands, shoe toe, chair parts, laptop lid angle.", "Haarmasse, Hemd und Kragen als getrennte Formen, Fäustlingshände, Schuhspitze, Stuhlteile, Neigung des Laptop-Deckels."),
      merged: txt("Face and neck share one skin shape; thumb and fingers are one mitten; keyboard and base are one wedge.", "Gesicht und Hals teilen eine Hautform; Daumen und Finger sind ein Fäustling; Tastatur und Unterteil sind ein Keil."),
      omitted: txt("Eye, ear, hair parting, seams, key rows, hinge, castor hubs.", "Auge, Ohr, Haarscheitel, Nähte, Tastenreihen, Scharnier, Rollennaben."),
      thinElements: txt("Laptop lid 9 mm stays a closed outline with 0.18 mm pen; display edge line dropped.", "Der 9 mm dicke Laptop-Deckel bleibt eine geschlossene Kontur mit 0,18-mm-Stift; die Displaykante entfällt."),
      hatching: txt("None; shirt and trousers read by tone.", "Keine; Hemd und Hose lesen sich über den Ton."),
      styles: txt("Body 0.18 mm, detail 0.10 mm; shirt light, trousers dark, shoes darkest, furniture mid.", "Körper 0,18 mm, Detail 0,10 mm; Hemd hell, Hose dunkel, Schuhe am dunkelsten, Möbel mittel."),
      overlays: txt("One region per drawn surface: face-skin, hair, hand-skin, shirt, trousers, shoes, laptop, desk and the four chair regions.", "Eine Region je gezeichneter Fläche: Gesichtshaut, Haar, Handhaut, Hemd, Hose, Schuhe, Laptop, Tisch und die vier Stuhlregionen."),
    },
    100: {
      silhouette: txt("Articulated clothing outlines: shirt with turned collar, trousers with seam, leather shoes with sole line and flexing forefoot, hair mass with parting, mitten hands with thumb, chair with castors, desk edge and open laptop with keyboard line.", "Gegliederte Kleidungskonturen: Hemd mit umgelegtem Kragen, Hose mit Naht, Lederschuhe mit Sohlenlinie und beweglichem Vorfuß, Haarmasse mit Scheitel, Fäustlingshände mit Daumen, Stuhl mit Rollen, Tischkante und geöffneter Laptop mit Tastaturlinie."),
      retained: txt("Face profile with nose and ear, collar fold, hair parting, desk edge line, keyboard and display lines.", "Gesichtsprofil mit Nase und Ohr, Kragenfalte, Haarscheitel, Tischkantenlinie, Tastatur- und Displaylinie."),
      merged: txt("Finger groups are one mitten outline; the eye and mouth line are not drawn.", "Fingergruppen sind eine Fäustlingskontur; Auge und Mundlinie werden nicht gezeichnet."),
      omitted: txt("Eye (0.14 mm), shoe laces, key rows, trackpad, hinge barrel.", "Auge (0,14 mm), Schnürsenkel, Tastenreihen, Trackpad, Scharnierwalze."),
      thinElements: txt("Laptop lid 9 mm = 0.09 mm at paper is drawn as a closed outline of the 0.2 mm pen, display and keyboard as 0.1 mm lines.", "Der 9 mm dicke Laptop-Deckel (0,09 mm auf Papier) wird als geschlossene Kontur des 0,2-mm-Stifts gezeichnet, Display und Tastatur als 0,1-mm-Linien."),
      hatching: txt("None: plain cotton and wool-blend cloth have no printable texture at this scale.", "Keine: Baumwolle und Wollmischung haben in diesem Maßstab keine druckbare Struktur."),
      styles: txt("Body 0.2 mm, detail 0.1 mm; same tone map as 1:50.", "Körper 0,2 mm, Detail 0,1 mm; dieselbe Tonzuordnung wie 1:50."),
      overlays: txt("Regions as in 1:200; collar and shirt share one colour, the display edge line takes the laptop colour.", "Regionen wie in 1:200; Kragen und Hemd teilen eine Farbe, die Displaykante übernimmt die Laptopfarbe."),
    },
    50: {
      silhouette: txt("Fully articulated seated person: shirt with collar and cuffs, finger groups on the keys, face profile with eye and mouth line, short hair with parting, leather shoes, chair with five-star base, armrest and lever, desk top with apron and foot plate, laptop with keyboard rows, trackpad and hinge.", "Voll gegliederte sitzende Person: Hemd mit Kragen und Manschetten, Fingergruppen auf den Tasten, Gesichtsprofil mit Auge und Mundlinie, Kurzhaar mit Scheitel, Lederschuhe, Stuhl mit Fünfsternfuß, Armlehne und Hebel, Tischplatte mit Zarge und Fußplatte, Laptop mit Tastenreihen, Trackpad und Scharnier."),
      retained: txt("Every authored detail of the body; the chair keeps all hand-drawn parts of the office swivel chair.", "Jedes gezeichnete Körperdetail; der Stuhl behält alle gezeichneten Teile des Bürodrehstuhls."),
      merged: txt("Nothing merged except the two keyboard bars into one key-row line.", "Nichts verschmolzen außer den Tastenbalken zu einer Tastenreihenlinie."),
      omitted: txt("Individual keycaps, shoe laces, screen content, cables.", "Einzelne Tastenkappen, Schnürsenkel, Bildschirminhalt, Kabel."),
      thinElements: txt("Laptop lid 9 mm = 0.45 mm at paper, drawn as a closed outline; key row lines 0.1 mm.", "Der 9 mm dicke Laptop-Deckel (0,45 mm auf Papier) wird als geschlossene Kontur gezeichnet; Tastenreihenlinien 0,1 mm."),
      hatching: txt("None.", "Keine."),
      styles: txt("Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; shirt light, trousers dark, shoes darkest.", "Körper 0,25 mm, Detail 0,13 mm, Struktur 0,10 mm; Hemd hell, Hose dunkel, Schuhe am dunkelsten."),
      overlays: txt("Same regions as 1:200; the forearm cuffs belong to the shirt region and the key rows to the laptop region.", "Dieselben Regionen wie in 1:200; die Manschetten gehören zur Hemdregion und die Tastenreihen zur Laptopregion."),
    },
  }[scale];
  return t;
}
const reps = {};
for (const sc of [500, 200, 100, 50]) {
  const src = reader.representations[String(sc)];
  const chairRep = chairA.representations[String(sc)];
  reps[String(sc)] = {
    status: "authored",
    denominator: sc,
    validRange: src.validRange,
    pens: { ...PENS[sc] },
    hatches: clone(chairRep.hatches ?? []),
    geometry: { elevation: [...chairShapes(sc), ...stationShapes(sc), ...bodyShapes(sc)] },
    decisions: decisions(sc),
  };
}
//#endregion

//#region Poses
const ids = (side) => ({ arm: `arm-${side}`, fore: `forearm-${side}`, hand: `hand-${side}`, thigh: `thigh-${side}`, shank: `shank-${side}`, foot: `foot-${side}`, toe: `toe-${side}` });
const bone = (id) => bones.find((b) => b.id === id);
const anchor = (id) => anchors.find((a) => a.id === id);
const REV = ["pelvis", "torso", "neck", "head", "arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r", "thigh-l", "shank-l", "foot-l", "toe-l", "thigh-r", "shank-r", "foot-r", "toe-r"];
const anchorWorld = (F, a) => toWorld(F.get(anchor(a).bone), [anchor(a).x, anchor(a).y]);

function solveLegs() {
  let lo = -25, hi = 10;
  for (let i = 0; i < 60; i++) {
    const m = (lo + hi) / 2;
    const F = fk(bones, { "thigh-l": { angleDeg: m }, "shank-l": { angleDeg: -m } });
    const y = Math.min(anchorWorld(F, "heel-l")[1], anchorWorld(F, "ball-l")[1]);
    if (y > 0) hi = m;
    else lo = m;
  }
  return (lo + hi) / 2;
}
const THIGH = r4(solveLegs());
for (const m of [0, -8, -16]) {
  const F = fk(bones, { "thigh-l": { angleDeg: m }, "shank-l": { angleDeg: -m } });
  console.error("[DEBUG] thigh", m, "heel", anchorWorld(F, "heel-l").map((v) => r4(v)), "ball", anchorWorld(F, "ball-l").map((v) => r4(v)), "hip", F.get("pelvis").y, "seat", F.get("chair-seat").y);
}

function makePose(spec) {
  const o = {};
  o["hip-shift"] = { dx: spec.shift ?? 0 };
  o["hip-lift"] = { dy: spec.lift ?? 0 };
  o.pelvis = { angleDeg: spec.pelvis ?? 0 };
  o.torso = { angleDeg: spec.torso };
  o.neck = { angleDeg: spec.neck };
  o.head = { angleDeg: spec.head };
  o["chair-backrest"] = { angleDeg: spec.backrest ?? 0 };
  for (const s of LEGS) {
    o[`thigh-${s}`] = { angleDeg: THIGH };
    o[`shank-${s}`] = { angleDeg: -THIGH };
    o[`foot-${s}`] = { angleDeg: 0 };
    o[`toe-${s}`] = { angleDeg: 0 };
  }
  const F0 = fk(bones, o);
  const warn = [];
  for (const [side, tgt] of Object.entries(spec.arms)) {
    const t = typeof tgt === "function" ? tgt(F0) : tgt;
    const I = ids(side);
    const T = F0.get("torso");
    const shoulder = toWorld(T, [bone(I.arm).x, bone(I.arm).y]);
    const sol = ik2(shoulder, t.wrist, bone(I.arm).lengthM, bone(I.fore).lengthM, t.elbow ?? -1);
    o[I.arm] = { angleDeg: r4(wrap(sol.a1 - (T.a + 180))) };
    o[I.fore] = { angleDeg: r4(wrap(sol.a2 - sol.a1)) };
    o[I.hand] = { angleDeg: r4(t.handRel !== undefined ? t.handRel : wrap(t.hand - sol.a2)) };
    o[`__elbow-${side}`] = sol.elbow;
  }
  const F = fk(bones, o);
  const elbows = { l: o["__elbow-l"], r: o["__elbow-r"] };
  delete o["__elbow-l"];
  delete o["__elbow-r"];
  const lim = (id, v) => {
    const j = bone(id).joint;
    if (j.minDeg !== undefined && (v < j.minDeg - 1e-6 || v > j.maxDeg + 1e-6)) warn.push(`${spec.id}:${id}=${v} outside ${j.minDeg}..${j.maxDeg}`);
  };
  for (const id of REV) if (o[id]) lim(id, o[id].angleDeg);
  return { offsets: o, F, warn, elbows };
}

const WRIST_L = [0.505, 0.768], WRIST_R = [0.47, 0.765];
const THIGH_TOP = (F) => toWorld(F.get("thigh-r"), [0.2, 0.0]);
const SPECS = {
  typing: { id: "typing", shift: -0.04, torso: -6, neck: -8, head: -15, arms: { l: { wrist: WRIST_L, hand: -4, elbow: -1 }, r: { wrist: WRIST_R, hand: -4, elbow: -1 } } },
  "reading-screen": { id: "reading-screen", touch: true, torso: 3, neck: -11, head: -9, backrest: 4, arms: { l: { wrist: [0.34, 0.64], hand: -6, elbow: -1 }, r: { wrist: [0.46, 0.768], hand: -6, elbow: -1 } } },
  "leaning-back": {
    id: "leaning-back", touch: true, torso: 12, neck: -4, head: -4, backrest: 12,
    arms: {
      l: (F) => ({ wrist: toWorld(F.get("head"), [0.1 * K, 0.115 * K]), handRel: 40, elbow: -1 }),
      r: { wrist: [0.16, 0.74], hand: -2, elbow: -1 },
    },
  },
};
function backGap(F) {
  const a = anchor("backrest");
  const p = toWorld(F.get("torso"), [a.x, a.y]);
  const br = F.get("chair-backrest");
  const h = (p[1] - br.y) / Math.sin(br.a * (Math.PI / 180));
  const front = toWorld(br, [h, -0.046]);
  return { gap: p[0] - front[0], p, front };
}
const POSES = {};
for (const k of Object.keys(SPECS)) {
  const spec = SPECS[k];
  if (spec.touch) {
    let lo = -0.15, hi = 0.15;
    for (let i = 0; i < 50; i++) {
      const m = (lo + hi) / 2;
      spec.shift = m;
      const { F } = makePose(spec);
      if (backGap(F).gap > 0) hi = m;
      else lo = m;
    }
    spec.shift = r4(spec.shift);
  }
  POSES[k] = makePose(spec);
  if (POSES[k].warn.length) console.error("[DEBUG]", POSES[k].warn.join("; "));
  { const F = POSES[k].F; console.error("[DEBUG]", k, "gap", r4(backGap(F).gap), "shoulder", toWorld(F.get("torso"), [bone("arm-l").x, 0]).map(r4), "head", toWorld(F.get("head"), [0.1 * K, 0.115 * K]).map(r4), "eye", toWorld(F.get("head"), [0.0641 * K, -0.0577 * K]).map(r4), "elbows", JSON.stringify(POSES[k].elbows)); }
}
//#endregion

//#region Thermal
const AREA_F = 1.937 / 1.692;
const regs = clone(reader.thermal.regions);
const REGION_BY = Object.fromEntries(regs.map((r) => [r.id, r]));
const clo = (c, f = 1.144) => r4(c * 0.155 * f);
const vap = (R) => r4(R / (16.5e-3 * 0.38));
const layer = (id, en, de, material, thicknessM, cloV, hc, ref = "iso-9920-2007") => ({ id, title: txt(en, de), kind: "clothing", material, thicknessM, resistanceM2KW: clo(cloV), vapourResistanceM2PaW: vap(clo(cloV)), heatCapacityJM2K: hc, source: "authored-estimate", reference: ref });
const bodyRegions = [
  { ...REGION_BY["face-skin"], title: txt("Face and neck skin", "Haut von Gesicht und Hals"), areaM2: r4(REGION_BY["face-skin"].areaM2 * AREA_F) },
  { ...REGION_BY["hair"], title: txt("Short hair", "Kurzes Haar"), areaM2: r4(0.0348 * AREA_F), layers: [{ ...REGION_BY["hair"].layers[0], title: txt("Short cut scalp hair", "Kurz geschnittenes Kopfhaar"), thicknessM: 0.012, resistanceM2KW: 0.045, vapourResistanceM2PaW: 7.2, heatCapacityJM2K: 250 }], heatCapacityJM2K: 8250 },
  { ...REGION_BY["hand-skin"], areaM2: r4(REGION_BY["hand-skin"].areaM2 * AREA_F) },
  {
    ...REGION_BY["sweater"],
    id: "shirt",
    title: txt("Long-sleeve shirt over a vest on torso, neck and arms", "Langarmhemd über Unterhemd an Rumpf, Hals und Armen"),
    material: "cotton-shirt",
    areaM2: r4(REGION_BY["sweater"].areaM2 * AREA_F),
    layers: [
      layer("vest", "Cotton vest", "Baumwoll-Unterhemd", "cotton-underwear", 0.0006, 0.06, 80),
      layer("shirt", "Cotton long-sleeve shirt", "Langarmhemd aus Baumwolle", "cotton-shirt", 0.0008, 0.25, 200),
    ],
    heatCapacityJM2K: 13280,
  },
  {
    ...REGION_BY["jeans"],
    id: "trousers",
    title: txt("Briefs and wool-blend trousers", "Slip und Hose aus Wollmischung"),
    material: "wool-blend",
    areaM2: r4(REGION_BY["jeans"].areaM2 * AREA_F),
    layers: [
      layer("briefs", "Cotton briefs", "Baumwollslip", "cotton-underwear", 0.0005, 0.04, 70),
      layer("trousers", "Straight wool-blend suit trousers", "Gerade Anzughose aus Wollmischung", "wool-blend", 0.0014, 0.25, 380),
    ],
    heatCapacityJM2K: 13450,
  },
  {
    ...REGION_BY["shoes"],
    title: txt("Socks and leather shoes", "Socken und Lederschuhe"),
    material: "leather-shoe",
    areaM2: r4(REGION_BY["shoes"].areaM2 * AREA_F),
    layers: [layer("socks", "Cotton socks", "Baumwollsocken", "cotton-sock", 0.0015, 0.03, 90), layer("shoe", "Leather office shoe", "Lederschuh fürs Büro", "leather-shoe", 0.004, 0.03, 700)],
    heatCapacityJM2K: 13790,
  },
];
const chairRegions = clone(chairA.thermal.regions);
const surf = (id, en, de, material, areaM2, L, geometry, hc, extra = {}) => ({ id, title: txt(en, de), kind: "surface", model: "passive", material, areaM2, characteristicLengthM: L, geometry, skyViewFactor: 0.5, layers: [], heatCapacityJM2K: hc, source: "authored-estimate", ...extra });
const deskRegion = surf("desk", "Desk top fragment with end panel", "Schreibtischplatte mit Stirnwand", "desk-board", 1.1, 0.5, "horizontal-plate-up", 26000);
const laptopRegion = { ...surf("laptop", "Laptop base and lid", "Laptop-Unterteil und -Deckel", "laptop-aluminium", 0.28, 0.3, "horizontal-plate-up", 4000, { kind: "surface", model: "operating" }), operating: { key: "laptop-load", heatW: 30 } };
const regionIds = [...bodyRegions.map((r) => r.id), "laptop", "desk", ...chairRegions.map((r) => r.id)];

const respiration = clone(reader.thermal.respiration);
respiration.patterns = [
  { id: "desk-work", title: txt("Seated desk work", "Sitzende Büroarbeit"), rateBpm: 15, tidalVolumeL: 0.55, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.13, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
  { id: "reclined-rest", title: txt("Reclined pause", "Zurückgelehnte Pause"), rateBpm: 13, tidalVolumeL: 0.5, inhaleFraction: 0.4, pauseAfterInhaleFraction: 0.03, exhaleFraction: 0.46, pauseAfterExhaleFraction: 0.11, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
];
respiration.exhaledTempC = 34;
respiration.motion = [{ bone: "torso", angleDeg: 0.5, dy: 0.003 }];

const emitters = clone(reader.thermal.emitters);
emitters.find((e) => e.id === "body-plume").region = "shirt";
emitters.push({ id: "laptop-vent", kind: "fan", title: txt("Laptop fan exhaust", "Abluft des Laptop-Lüfters"), anchor: "laptop-vent", directionDeg: 180, diameterM: 0.04, operatingKey: "laptop-load", volumeFlowM3S: 0.0012, temperatureRiseK: 10, source: "authored-estimate", reference: "ashrae-2021" });

const thermal = {
  environment: { airTempC: 22, meanRadiantTempC: 22, skyTempC: 22, groundTempC: 22, relativeHumidity: 0.45, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0.5, pressurePa: 101325 },
  baseMet: 1.2,
  body: { species: "human", massKg: 76, skinAreaM2: 1.937, basalMetabolismW: 82, metWM2: 58.15, coreSetPointC: 36.8, skinSetPointC: 34.1, sweating: true, panting: false, maxSkinBloodFlowLM2H: 90, minSkinBloodFlowLM2H: 1, source: "authored-estimate", reference: "gagge-1971" },
  respiration,
  operating: [{ id: "laptop-load", title: txt("Laptop load", "Laptop-Last"), levels: [{ v: 0, title: txt("Off", "Aus") }, { v: 0.35, title: txt("Idle", "Leerlauf") }, { v: 1, title: txt("Typing and browsing", "Tippen und Surfen") }] }],
  passengers: [],
  regions: [...bodyRegions, laptopRegion, deskRegion, ...chairRegions],
  emitters,
  assumptions: [
    txt("A 1.78 m, 76 kg man: DuBois and DuBois (1916) skin area 1.937 m2; basal metabolism 82 W after Mifflin et al. (1990) for 35 years; segment masses are the seated-reader de Leva fractions scaled to 78.6 kg including 2.6 kg of clothing and shoes.", "Ein 1,78 m großer, 76 kg schwerer Mann: Hautfläche 1,937 m2 nach DuBois und DuBois (1916); Grundumsatz 82 W nach Mifflin et al. (1990) für 35 Jahre; Segmentmassen sind die de-Leva-Anteile des Lesenden auf 78,6 kg einschließlich 2,6 kg Kleidung und Schuhe skaliert."),
    txt("Office clothing: vest 0.06, long-sleeve shirt 0.25, briefs 0.04, suit trousers 0.25, socks 0.03 and leather shoes 0.03 clo (ISO 9920 / ASHRAE tables as authored estimates); resistances use the seated-reader convention (clo x 0.155 x 1.144). The chair adds its own pad as a separate passive region.", "Bürokleidung: Unterhemd 0,06, Langarmhemd 0,25, Slip 0,04, Anzughose 0,25, Socken 0,03 und Lederschuhe 0,03 clo (Tabellen von ISO 9920 / ASHRAE als Schätzwerte); die Widerstände folgen der Konvention des Lesenden (clo x 0,155 x 1,144). Der Stuhl bringt sein Polster als eigene passive Region mit."),
    txt("Seated typing is 1.2 met (70 W/m2; ISO 8996 / ISO 7730 Annex A), seated reading on the screen 1.1 met, reclined pause 1.0 met.", "Sitzendes Tippen sind 1,2 met (70 W/m2; ISO 8996 / ISO 7730 Anhang A), Lesen am Bildschirm 1,1 met, zurückgelehnte Pause 1,0 met."),
    txt("The laptop releases 30 W at full load (authored estimate guided by ASHRAE 2021 office equipment heat gains), 35 % of it at idle; a 1.2 L/s fan carries about 14 W out of the case with a 10 K temperature rise, the rest leaves through the surfaces.", "Der Laptop gibt bei voller Last 30 W ab (Schätzwert nach den Wärmegewinnen von Bürogeräten in ASHRAE 2021), im Leerlauf 35 %; ein Lüfter mit 1,2 L/s trägt bei 10 K Temperaturerhöhung etwa 14 W aus dem Gehäuse, der Rest verlässt es über die Oberflächen."),
    txt("The desk is a fragment (top and end panel) that carries the laptop so the figure is self-contained; the catalog desk 'height-adjustable-desk' is not required.", "Der Schreibtisch ist ein Ausschnitt (Platte und Stirnwand), der den Laptop trägt, damit die Figur in sich geschlossen ist; der Katalogschreibtisch 'height-adjustable-desk' wird nicht benötigt."),
  ],
};
//#endregion

//#region Pose and clip assembly
const CASTOR_CONTACTS = [];
const BASE_CONTACTS = ["seat", "thigh-support", "heel-l", "ball-l", "heel-r", "ball-r"];
const bind = (e, s = 0) => ({ exposedFraction: e, sunlitFraction: s });
function binding(k) {
  const back = k === "leaning-back";
  const r = {
    "face-skin": bind(1),
    hair: bind(1),
    "hand-skin": bind(1),
    shirt: { ...bind(back ? 0.86 : 0.93), contact: { fraction: back ? 0.14 : 0.07, tempC: 24 } },
    trousers: { ...bind(0.84), contact: { fraction: 0.16, tempC: 24 } },
    shoes: bind(1),
    laptop: bind(1),
    desk: bind(1),
    "chair-plastic": bind(1),
    "chair-fabric": bind(0.55),
    "chair-steel": bind(0.9),
    "castor-rubber": bind(0.8),
  };
  const met = { typing: 1.2, "reading-screen": 1.1, "leaning-back": 1.0 }[k];
  const load = { typing: 1, "reading-screen": 0.6, "leaning-back": 0.35 }[k];
  return {
    activityMet: met,
    breathing: back ? "reclined-rest" : "desk-work",
    operating: { "laptop-load": load },
    regions: r,
    emitters: { "body-plume": { typing: 0.7, "reading-screen": 0.65, "leaning-back": 0.55 }[k], "laptop-vent": load },
  };
}
const toPoseBones = (o) => Object.fromEntries(Object.entries(o).map(([k, v]) => [k, v]));
const POSE_TEXT = {
  typing: [txt("Typing", "Tippen"), txt("Upright and slightly forward at the laptop, both forearms level on the keyboard, head bowed to the screen.", "Aufrecht und leicht vorgebeugt am Laptop, beide Unterarme waagerecht auf der Tastatur, der Kopf zum Bildschirm geneigt.")],
  "reading-screen": [txt("Reading the screen", "Bildschirm lesen"), txt("Back resting on the chair, head pushed forward, near hand on the trackpad and far forearm on the thigh.", "Der Rücken ruht am Stuhl, der Kopf ist nach vorn geschoben, die nahe Hand liegt am Trackpad und der ferne Unterarm auf dem Oberschenkel.")],
  "leaning-back": [txt("Leaning back", "Zurücklehnen"), txt("The backrest tilts with the torso, the far hand rests behind the head and the near forearm lies on the armrest side of the lap.", "Die Lehne neigt sich mit dem Rumpf, die ferne Hand ruht hinter dem Kopf und der nahe Unterarm liegt seitlich im Schoß.")],
};
const poses = Object.keys(POSES).map((k) => ({
  id: k,
  title: POSE_TEXT[k][0],
  description: POSE_TEXT[k][1],
  bones: toPoseBones(POSES[k].offsets),
  ground: "authored",
  contacts: [...BASE_CONTACTS, ...(k === "leaning-back" ? ["backrest"] : [])],
  thermal: binding(k),
  loads: [],
}));

const contactSpans = (T, backrest = []) => [...BASE_CONTACTS.map((a) => ({ anchor: a, fromS: 0, toS: T })), ...backrest.map(([a, b]) => ({ anchor: "backrest", fromS: a, toS: b }))];
function typingKey(t, over = {}) {
  const spec = { ...SPECS.typing, ...over, arms: { l: { ...SPECS.typing.arms.l, ...(over.l ?? {}) }, r: { ...SPECS.typing.arms.r, ...(over.r ?? {}) } } };
  const { offsets } = makePose(spec);
  const keep = ["head", "neck", "torso", "arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"];
  return { t, pose: "typing", ease: "smooth", bones: Object.fromEntries(keep.map((id) => [id, offsets[id]])) };
}
const lift = (w, dx, dy) => ({ wrist: [w[0] + dx, w[1] + dy], hand: -4 + dy * 220, elbow: -1 });
const clipTyping = {
  id: "typing",
  title: txt("Typing", "Tippen"),
  description: txt("A looping typing cycle: the hands take turns lifting a finger-width off the keys, the head dips toward the keyboard and rises to the screen, and the back follows the breath by a hair.", "Ein Tippzyklus als Schleife: die Hände heben abwechselnd einen Fingerbreit von den Tasten ab, der Kopf senkt sich zur Tastatur und hebt sich zum Bildschirm, und der Rücken folgt dem Atem um ein Haar."),
  durationS: 4.8,
  fps: 30,
  loop: true,
  rootMotion: "none",
  keys: [
    typingKey(0),
    typingKey(0.6, { r: lift(WRIST_R, -0.012, 0.014), head: -15 }),
    typingKey(1.2, { l: lift(WRIST_L, -0.01, 0.012), head: -16 }),
    typingKey(1.8, { head: -18, torso: -6.8 }),
    typingKey(2.4, { r: lift(WRIST_R, 0.01, 0.016), head: -17 }),
    typingKey(3.0, { l: lift(WRIST_L, 0.012, 0.014), head: -14 }),
    typingKey(3.6, { head: -11, torso: -5.4 }),
    typingKey(4.2, { r: lift(WRIST_R, -0.01, 0.012), head: -13 }),
  ],
  contacts: contactSpans(4.8),
  phases: [
    { id: "burst", fromS: 0, toS: 2.4, title: txt("Typing burst", "Tippphase") },
    { id: "look-up", fromS: 2.4, toS: 3.6, title: txt("Look up and type on", "Aufschauen und weitertippen") },
    { id: "settle", fromS: 3.6, toS: 4.8, title: txt("Back to the keys", "Zurück zu den Tasten") },
  ],
  events: [
    { t: 1.8, kind: "motion", title: txt("Head dips to the keyboard", "Kopf senkt sich zur Tastatur") },
    { t: 3.6, kind: "motion", title: txt("Head lifts to the screen", "Kopf hebt sich zum Bildschirm") },
  ],
  tracks: [
    { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4.8, v: 0 }] },
    { target: "operating:laptop-load", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 3.0, v: 0.8 }, { t: 4.8, v: 1 }] },
  ],
};
const clipSettle = {
  id: "settle-back",
  title: txt("Settling back", "Zurücklehnen"),
  description: txt("From typing she reads the screen with the back on the chair and then leans back, the backrest tilting with the torso while the laptop falls to idle.", "Vom Tippen geht es zum Lesen am Bildschirm mit dem Rücken am Stuhl und dann ins Zurücklehnen, wobei sich die Lehne mit dem Rumpf neigt und der Laptop in den Leerlauf fällt."),
  durationS: 7,
  fps: 30,
  loop: false,
  rootMotion: "none",
  keys: [
    { t: 0, pose: "typing", ease: "smooth" },
    { t: 2.0, pose: "reading-screen", ease: "smooth" },
    { t: 3.0, pose: "reading-screen", ease: "smooth" },
    { t: 5.0, pose: "leaning-back", ease: "smooth" },
    { t: 7.0, pose: "leaning-back", ease: "smooth" },
  ],
  contacts: contactSpans(7, [[4.4, 7]]),
  phases: [
    { id: "read", fromS: 0, toS: 3.0, title: txt("Reading on", "Weiterlesen") },
    { id: "recline", fromS: 3.0, toS: 5.0, title: txt("Leaning back", "Zurücklehnen") },
    { id: "pause", fromS: 5.0, toS: 7.0, title: txt("Pause", "Pause") },
  ],
  events: [{ t: 4.4, kind: "contact-on", title: txt("Back reaches the backrest", "Rücken erreicht die Lehne"), target: "backrest" }],
  tracks: [
    { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4.0, v: 1 }] },
    { target: "operating:laptop-load", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 3.0, v: 0.6 }, { t: 5.0, v: 0.35 }, { t: 7, v: 0.35 }] },
  ],
};
//#endregion

//#region Asset
const refs = clone(reader.provenance.references);
for (const r of chairA.provenance.references) if (!refs.some((x) => x.id === r.id)) refs.push(r);
const mat = (id, en, de, emissivity, abs, source, reference, extra = {}) => ({ id, title: txt(en, de), emissivity, solarAbsorptivity: abs, ...extra, source, ...(reference ? { reference } : {}) });
const materials = [
  ...reader.materials.filter((m) => ["skin", "hair", "cotton-underwear", "cotton-sock"].includes(m.id)),
  mat("cotton-shirt", "Cotton shirt fabric", "Hemdstoff aus Baumwolle", 0.93, 0.45, "literature", "ashrae-2021"),
  mat("wool-blend", "Wool-blend suit cloth", "Anzugstoff aus Wollmischung", 0.9, 0.7, "literature", "ashrae-2021"),
  mat("leather-shoe", "Leather", "Leder", 0.9, 0.75, "authored-estimate"),
  mat("laptop-aluminium", "Anodised aluminium laptop shell", "Eloxierte Aluminiumschale des Laptops", 0.18, 0.5, "authored-estimate"),
  mat("desk-board", "Melamine-faced chipboard", "Melaminbeschichtete Spanplatte", 0.9, 0.5, "literature", "ashrae-2021"),
  ...chairA.materials,
];
const asset = {
  id: "seated-office-worker",
  category: "humans",
  title: txt("Seated Office Worker", "Büroarbeitende Person im Sitzen"),
  summary: txt(
    "A man in shirt and suit trousers working on a laptop at a desk from a swivel office chair: seat bones on the chair, flat feet, a typing loop, a screen-reading pose and a lean-back with the backrest tilting; at 1.2 met with a 30 W laptop beside him for internal-gain studies.",
    "Ein Mann in Hemd und Anzughose arbeitet im Bürodrehstuhl an einem Laptop auf dem Schreibtisch: Sitzbeinhöcker auf dem Stuhl, flache Füße, eine Tippschleife, eine Haltung beim Lesen am Bildschirm und ein Zurücklehnen mit mitkippender Lehne; bei 1,2 met und mit einem 30-W-Laptop neben sich für Studien zu inneren Wärmegewinnen.",
  ),
  tags: ["human", "adult", "man", "seated", "office", "laptop", "desk", "swivel-chair", "internal-gains", "typing"],
  provenance: {
    author: "Semio staffage content worker A1",
    method: "handcrafted",
    created: "2026-11-01",
    references: refs,
    notes: txt(
      "A 1.78 m, 76 kg man seated on the office swivel chair (hand-drawn chair assets embedded as a sub-assembly under a floor datum), working at a desk fragment with an open laptop. The body skeleton and mitten hands are the seated-reader rig scaled to the 1.78 m design; clothing, short hair, collar, chair, desk, laptop, poses, clips and thermal model are new. The laptop is an operating region (30 W, fan exhaust).",
      "Ein 1,78 m großer, 76 kg schwerer Mann im Bürodrehstuhl (die gezeichneten Stuhl-Assets als Unterbaugruppe unter einem Bodenbezug), der an einem Schreibtischausschnitt mit geöffnetem Laptop arbeitet. Skelett und Fäustlingshände sind das auf den 1,78-m-Entwurf skalierte Skelett des Lesenden; Kleidung, Kurzhaar, Kragen, Stuhl, Schreibtisch, Laptop, Haltungen, Clips und Thermikmodell sind neu. Der Laptop ist eine Betriebsregion (30 W, Lüfterabluft).",
    ),
  },
  dimensions: { heightM: 1.411, widthM: 1.291, depthM: 0.6, massKg: totalMass },
  materials,
  rig: { kind: "anatomical", root: "floor", bones },
  anchors,
  representations: reps,
  symbols: reader.symbols.map((s) => ({ ...clone(s), id: "seated-person-marker", title: txt("Seated person marker", "Sitzende Person als Marke"), description: txt("Position dot for crowded overview plans; no direction arrow because a seated person does not travel.", "Positionspunkt für belebte Übersichtspläne; ohne Richtungspfeil, da eine sitzende Person nicht läuft."), geometry: { elevation: s.geometry.elevation.map((g) => ({ ...g, bone: "floor" })) }, dimensionalNote: txt("The dot marks the true position of the chair; its 0.9 mm diameter is a paper size and carries no body dimension.", "Der Punkt markiert die wahre Position des Stuhls; sein Durchmesser von 0,9 mm ist eine Papiergröße und trägt kein Körpermaß.") })),
  poses,
  clips: [clipTyping, clipSettle],
  thermal,
  structure: { gravityMS2: 9.80665, dynamics: "inertial", supports: [], assumptions: reader.structure.assumptions.map((a) => ({ en: a.en.replace("bench", "chair").replace("The chair is rigid and fixed.", "The chair is rigid and fixed."), de: a.de.replace("Bank", "Stuhl") })), clinical: false },
  presentation: { defaultPose: "typing", defaultClip: "typing", defaultScale: 100, viewpoints: ["elevation"] },
  exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"],
  completion: "authored",
};
const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
const outDir = `${S}/🗂️catalog/👤️humans/seated-office-worker`;
mkdirSync(outDir, { recursive: true });
writeFileSync(`${outDir}/🔣️.json`, JSON.stringify(doc, null, 2) + "\n");
console.error(`[DEBUG] wrote worker: thigh offset ${THIGH}, mass ${totalMass}, leaning-back shift ${SPECS["leaning-back"].shift}`);
//#endregion
