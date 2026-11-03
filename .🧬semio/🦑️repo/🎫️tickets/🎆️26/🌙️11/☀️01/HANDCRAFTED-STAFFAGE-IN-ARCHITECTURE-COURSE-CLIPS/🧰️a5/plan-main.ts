import { writeFileSync, mkdirSync } from "node:fs";
import { appendDecision, capsule, ellipse, insertPlan, line, pathShape, poly, rect, smoothClosed, symmetricOutline, type Pt, type Shape } from "./plan-lib.ts";
import { humanPlan, loadAsset, type HumanSpec } from "./plan-human.ts";

const CATALOG = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/a5";
const SCALES = ["50", "100", "200", "500"] as const;
type Scale = (typeof SCALES)[number];

const commuter: HumanSpec = {
  seated: false, armZ: 3, legZ: 1.05,
  garment: { torso: "coat", sleeve: "coat", legs: "trousers", shoes: "shoes", hair: "hair", face: "face-skin", hand: "hand-skin", neckwear: "scarf", bag: "bag-leather" },
  shoulderY: 0.2, sleeveR: 0.056, forearmR: 0.046, shoulderRx: 0.125, shoulderRy: 0.2, waistRx: 0.12, waistRy: 0.19, connectorR: 0.13,
  headRx: 0.093, headRy: 0.077, legY: 0.085, thighR: [0.085, 0.065], shankR: [0.06, 0.045], heel: 0.02, shoeHeelR: 0.04, shoeBallR: 0.047,
  neckwear: { rx: 0.07, ry: 0.085 }, bag: { x: -0.15, y: 0.25, w: 0.3, h: 0.09 },
};

const adult: HumanSpec = {
  seated: false, armZ: 3, legZ: 1.05,
  garment: { torso: "hoodie", sleeve: "hoodie", legs: "jeans", shoes: "sneakers", hair: "hair", face: "face-skin", hand: "hand-skin", neckwear: "hoodie" },
  shoulderY: 0.195, sleeveR: 0.05, forearmR: 0.042, shoulderRx: 0.115, shoulderRy: 0.195, waistRx: 0.105, waistRy: 0.165, connectorR: 0.115,
  headRx: 0.09, headRy: 0.074, legY: 0.085, thighR: [0.08, 0.062], shankR: [0.057, 0.043], heel: 0.02, shoeHeelR: 0.038, shoeBallR: 0.045,
  neckwear: { rx: 0.065, ry: 0.088 },
};

const worker: HumanSpec = {
  seated: true, armZ: 7, legZ: 2,
  garment: { torso: "shirt", sleeve: "shirt", legs: "trousers", shoes: "shoes", hair: "hair", face: "face-skin", hand: "hand-skin" },
  shoulderY: 0.195, sleeveR: 0.048, forearmR: 0.042, shoulderRx: 0.115, shoulderRy: 0.195, waistRx: 0.11, waistRy: 0.17, connectorR: 0.12,
  headRx: 0.093, headRy: 0.077, legY: 0.095, thighR: [0.085, 0.065], shankR: [0.06, 0.045], heel: 0.02, shoeHeelR: 0.04, shoeBallR: 0.047,
};

function seatedExtras(asset: ReturnType<typeof loadAsset>, scale: Scale): Shape[] {
  const out: Shape[] = [];
  const regions = new Set(asset.thermal.regions.map((r) => r.id));
  const reg = (id: string): { region?: string } => (regions.has(id) ? { region: id } : {});
  const hub = [0.04, 0] as const;
  const arms: { bone: string; castor: string; angle: number }[] = [
    { bone: "chair-arm-front", castor: "chair-castor-front", angle: 0 },
    { bone: "chair-arm-side-a", castor: "chair-castor-side-a", angle: 72 },
    { bone: "chair-arm-side-b", castor: "chair-castor-side-b", angle: -72 },
    { bone: "chair-arm-back-a", castor: "chair-castor-back-a", angle: 144 },
    { bone: "chair-arm-back-b", castor: "chair-castor-back-b", angle: -144 },
  ];
  const fine = scale === "50" || scale === "100";
  if (fine) {
    for (const a of arms) {
      const ex = hub[0] + 0.3 * Math.cos((a.angle * Math.PI) / 180);
      const ey = 0.3 * Math.sin((a.angle * Math.PI) / 180);
      out.push(capsule(`${a.bone}-plan`, a.bone, "body", 1.0, 0.45, hub[0], 0, ex, ey, 0.024, 0.017, reg("chair-steel")));
      out.push(ellipse(`${a.castor}-plan`, a.castor, "body", 1.02, 0.1, ex, ey, 0.027, 0.027, reg("castor-rubber")));
    }
    out.push(ellipse("chair-hub-plan", "chair-base", "body", 1.05, 0.45, hub[0], 0, 0.05, 0.05, reg("chair-steel")));
  }
  out.push(rect("chair-seat-plan", "chair-seat", "body", 1.5, 0.35, -0.16, -0.235, 0.43, 0.47, 0.06, reg("chair-fabric")));
  out.push(capsule("chair-backrest-plan", "chair-backrest", "body", 1.55, 0.3, -0.21, -0.215, -0.21, 0.215, 0.032, 0.032, { ...reg("chair-fabric"), followTip: false }));
  if (fine) {
    for (const [s, sign] of [["l", 1], ["r", -1]] as const) out.push(capsule(`chair-armrest-${s}-plan`, "chair-seat", "body", 1.6, 0.3, -0.14, sign * 0.268, 0.14, sign * 0.268, 0.021, 0.021, reg("chair-plastic")));
  }
  out.push(rect("desk-top-plan", "desk-top", "body", 5, 0.55, 0.28, -0.7, 0.78, 1.4, 0.01, reg("desk")));
  if (scale === "50") out.push(line("desk-edge-plan", "desk-top", "detail", 5.1, 0.4, [[0.292, -0.688], [1.048, -0.688], [1.048, 0.688], [0.292, 0.688], [0.292, -0.688]], {}));
  if (scale !== "500") {
    out.push(rect("laptop-base-plan", "laptop", "body", 5.5, 0.4, 0.42, -0.17, 0.32, 0.34, 0.012, reg("laptop")));
    out.push(rect("laptop-lid-plan", "screen", "body", 5.6, 0.45, 0.74, -0.17, 0.075, 0.34, 0.006, reg("laptop")));
    if (scale === "50") {
      out.push(line("laptop-keys-plan", "laptop", "detail", 5.55, 0.7, [[0.45, -0.145], [0.71, -0.145], [0.71, 0.145], [0.45, 0.145], [0.45, -0.145]], {}));
      out.push(line("laptop-pad-plan", "laptop", "detail", 5.55, 0.7, [[0.43, -0.05], [0.43, 0.05]], {}));
    }
  }
  return out;
}

function carPlan(asset: ReturnType<typeof loadAsset>, scale: Scale): Shape[] {
  const regions = new Set(asset.thermal.regions.map((r) => r.id));
  const reg = (id: string): { region?: string } => (regions.has(id) ? { region: id } : {});
  const outline: Pt[] = symmetricOutline([[1.95, 0], [1.945, 0.22], [1.915, 0.45], [1.84, 0.64], [1.66, 0.76], [1.35, 0.815], [0.95, 0.835], [0.3, 0.84], [-0.5, 0.84], [-1.1, 0.835], [-1.55, 0.82], [-1.82, 0.78], [-1.925, 0.66], [-1.95, 0.4], [-1.95, 0]]);
  const roof: Pt[] = symmetricOutline([[0.31, 0], [0.3, 0.55], [0.08, 0.63], [-0.5, 0.655], [-1.1, 0.65], [-1.44, 0.58], [-1.45, 0]]);
  const windscreen: Pt[] = symmetricOutline([[0.9, 0], [0.9, 0.7], [0.31, 0.6], [0.31, 0]]);
  const rear: Pt[] = symmetricOutline([[-1.52, 0], [-1.52, 0.58], [-1.74, 0.53], [-1.74, 0]]);
  const bonnet: Pt[] = symmetricOutline([[1.9, 0], [1.88, 0.3], [1.8, 0.58], [1.6, 0.7], [1.25, 0.75], [0.95, 0.76], [0.95, 0]]);
  const gate: Pt[] = symmetricOutline([[-1.5, 0], [-1.5, 0.74], [-1.78, 0.76], [-1.9, 0.68], [-1.93, 0.4], [-1.93, 0]]);
  const out: Shape[] = [];
  out.push(pathShape("body-shell-plan", "chassis", "body", 2, 0.62, smoothClosed(outline, 0.55), reg("body-panels")));
  if (scale === "500") {
    out.push(poly("greenhouse-plan", "chassis", "glazing", 6, 0.85, symmetricOutline([[0.9, 0], [0.9, 0.68], [0.31, 0.6], [-1.45, 0.6], [-1.74, 0.53], [-1.74, 0]]), reg("glazing")));
    return out;
  }
  out.push(poly("bonnet-panel-plan", "bonnet", "body", 2.3, 0.62, bonnet, reg("bonnet")));
  out.push(poly("tailgate-panel-plan", "hatch", "body", 2.3, 0.62, gate, reg("tailgate")));
  out.push(pathShape("roof-plan", "chassis", "body", 2.4, 0.5, smoothClosed(roof, 0.4), reg("roof")));
  out.push(poly("windscreen-plan", "chassis", "glazing", 6, 0.85, windscreen, reg("glazing")));
  out.push(poly("rear-glass-plan", "hatch", "glazing", 6, 0.85, rear, reg("glazing")));
  if (scale === "200") {
    for (const sign of [1, -1]) out.push(ellipse(`mirror-${sign > 0 ? "l" : "r"}-plan`, "chassis", "detail", 7, 0.35, 0.82, sign * 0.9, 0.065, 0.04, {}));
    return out;
  }
  for (const [s, sign] of [["l", 1], ["r", -1]] as const) {
    out.push(ellipse(`mirror-${s}-plan`, "chassis", "detail", 7, 0.35, 0.82, sign * 0.9, 0.065, 0.04, {}));
    out.push(poly(`headlamp-${s}-plan`, "chassis", "glazing", 7, 0.9, [[1.93, sign * 0.46], [1.9, sign * 0.66], [1.78, sign * 0.7], [1.76, sign * 0.55]], {}));
    out.push(line(`door-front-${s}-plan`, "chassis", "detail", 7, 0.5, [[0.72, sign * 0.7], [0.72, sign * 0.835]], {}));
    out.push(line(`door-b-${s}-plan`, "chassis", "detail", 7, 0.5, [[-0.08, sign * 0.68], [-0.08, sign * 0.84]], {}));
    out.push(line(`door-rear-${s}-plan`, "chassis", "detail", 7, 0.5, [[-1.38, sign * 0.62], [-1.36, sign * 0.83]], {}));
    out.push(rect(`tail-lamp-${s}-plan`, "chassis", "detail", 7, 0.7, -1.96, sign > 0 ? 0.5 : -0.72, 0.07, 0.22, 0.01, {}));
  }
  if (scale === "50") {
    for (const [s, sign] of [["l", 1], ["r", -1]] as const) out.push(line(`bonnet-crease-${s}-plan`, "bonnet", "detail", 7, 0.4, [[1.0, sign * 0.2], [1.5, sign * 0.22], [1.85, sign * 0.27]], {}));
  }
  return out;
}

const NOTES: Record<string, Record<Scale, { en: string; de: string }>> = {
  "walking-commuter": {
    "50": { en: "Plan: shoulders, waist, scarf, head with face, ears, nose and parting, upper arms, forearms and hands, thighs, shanks and shoes from above, the shoulder bag on the left hip with its strap; limbs stretch with the pose.", de: "Grundriss: Schultern, Taille, Schal, Kopf mit Gesicht, Ohren, Nase und Scheitel, Oberarme, Unterarme und Hände, Oberschenkel, Unterschenkel und Schuhe von oben, die Umhängetasche an der linken Hüfte mit Riemen; die Glieder dehnen sich mit der Pose." },
    "100": { en: "Plan: shoulders, waist, scarf, head with face, arms with hands, legs, shoes and the bag; no ears, nose, parting, laces or strap.", de: "Grundriss: Schultern, Taille, Schal, Kopf mit Gesicht, Arme mit Händen, Beine, Schuhe und Tasche; ohne Ohren, Nase, Scheitel, Schnürung und Riemen." },
    "200": { en: "Plan: trunk, shoulders, two arm links per side, head and shoes; legs, hands and bag are merged into these.", de: "Grundriss: Rumpf, Schultern, je Seite zwei Armglieder, Kopf und Schuhe; Beine, Hände und Tasche gehen darin auf." },
    "500": { en: "Plan: one shoulder ellipse of the coat width and the head circle.", de: "Grundriss: eine Schulterellipse in Manteltiefe und der Kopfkreis." },
  },
  "standing-adult": {
    "50": { en: "Plan: shoulders, waist, hood, head with face, ears, nose and parting, upper arms, forearms and hands, thighs, shanks and sneakers from above; limbs stretch with the pose.", de: "Grundriss: Schultern, Taille, Kapuze, Kopf mit Gesicht, Ohren, Nase und Scheitel, Oberarme, Unterarme und Hände, Oberschenkel, Unterschenkel und Turnschuhe von oben; die Glieder dehnen sich mit der Pose." },
    "100": { en: "Plan: shoulders, waist, hood, head with face, arms with hands, legs and sneakers; no ears, nose, parting or laces.", de: "Grundriss: Schultern, Taille, Kapuze, Kopf mit Gesicht, Arme mit Händen, Beine und Turnschuhe; ohne Ohren, Nase, Scheitel und Schnürung." },
    "200": { en: "Plan: trunk, shoulders, two arm links per side, head and sneakers; legs and hands are merged into these.", de: "Grundriss: Rumpf, Schultern, je Seite zwei Armglieder, Kopf und Turnschuhe; Beine und Hände gehen darin auf." },
    "500": { en: "Plan: one shoulder ellipse of the hoodie width and the head circle.", de: "Grundriss: eine Schulterellipse in Kapuzenpullover-Breite und der Kopfkreis." },
  },
  "seated-office-worker": {
    "50": { en: "Plan: shoulders, waist, head with face, ears, nose and parting, arms reaching over the desk to the keyboard, thighs on the seat, the five-star chair base with castors, seat, armrests and backrest, the desk top and the laptop with keyboard and lid; the shanks and shoes stay hidden under the desk.", de: "Grundriss: Schultern, Taille, Kopf mit Gesicht, Ohren, Nase und Scheitel, Arme über den Tisch zur Tastatur, Oberschenkel auf dem Sitz, der fünfstrahlige Stuhlfuß mit Rollen, Sitz, Armlehnen und Rückenlehne, die Tischplatte und der Laptop mit Tastatur und Deckel; Unterschenkel und Schuhe bleiben unter dem Tisch verdeckt." },
    "100": { en: "Plan: shoulders, waist, head with face, arms with hands, thighs, chair base, seat, armrests, backrest, desk top and laptop; no ears, nose, parting or keys.", de: "Grundriss: Schultern, Taille, Kopf mit Gesicht, Arme mit Händen, Oberschenkel, Stuhlfuß, Sitz, Armlehnen, Rückenlehne, Tischplatte und Laptop; ohne Ohren, Nase, Scheitel und Tasten." },
    "200": { en: "Plan: trunk, shoulders, two arm links per side, head, seat, backrest, desk top and laptop; chair base, armrests and legs are omitted.", de: "Grundriss: Rumpf, Schultern, je Seite zwei Armglieder, Kopf, Sitz, Rückenlehne, Tischplatte und Laptop; Stuhlfuß, Armlehnen und Beine entfallen." },
    "500": { en: "Plan: shoulder ellipse, head circle, seat and desk top.", de: "Grundriss: Schulterellipse, Kopfkreis, Sitz und Tischplatte." },
  },
  "city-car": {
    "50": { en: "Plan: body outline with the bonnet, roof and tailgate panels, windscreen and rear glass, both mirrors, headlamps, tail lamps, door gaps and bonnet creases; the wheels are hidden under the body.", de: "Grundriss: Aufbaukontur mit Haube, Dach und Heckklappe, Frontscheibe und Heckscheibe, beiden Spiegeln, Scheinwerfern, Rückleuchten, Türfugen und Hauben-Sicken; die Räder bleiben unter dem Aufbau verdeckt." },
    "100": { en: "Plan: body outline, bonnet, roof, tailgate, windscreen, rear glass, mirrors, headlamps, tail lamps and door gaps; no bonnet creases.", de: "Grundriss: Aufbaukontur, Haube, Dach, Heckklappe, Frontscheibe, Heckscheibe, Spiegel, Scheinwerfer, Rückleuchten und Türfugen; ohne Hauben-Sicken." },
    "200": { en: "Plan: body outline, bonnet, roof, tailgate, windscreen, rear glass and mirrors; lamps and door gaps are omitted.", de: "Grundriss: Aufbaukontur, Haube, Dach, Heckklappe, Frontscheibe, Heckscheibe und Spiegel; Leuchten und Türfugen entfallen." },
    "500": { en: "Plan: body outline and one glass area for windscreen, roof opening and rear glass.", de: "Grundriss: Aufbaukontur und eine Glasfläche für Frontscheibe, Dachbereich und Heckscheibe." },
  },
};

const ASSETS: Record<string, { file: string; build: (asset: ReturnType<typeof loadAsset>, scale: Scale) => Shape[] }> = {
  "walking-commuter": { file: "👤️humans/walking-commuter", build: (a, s) => humanPlan(a as never, commuter, s) },
  "standing-adult": { file: "👤️humans/standing-adult", build: (a, s) => humanPlan(a as never, adult, s) },
  "seated-office-worker": { file: "👤️humans/seated-office-worker", build: (a, s) => [...humanPlan(a as never, worker, s), ...seatedExtras(a, s)] },
  "city-car": { file: "🚜️vehicles/city-car", build: (a, s) => carPlan(a, s) },
};

const [mode, ...ids] = process.argv.slice(2);
mkdirSync(OUT, { recursive: true });
for (const id of ids.length > 0 ? ids : Object.keys(ASSETS)) {
  const spec = ASSETS[id]!;
  const path = `${CATALOG}/${spec.file}/🔣️.json`;
  const asset = loadAsset(path);
  const plans = Object.fromEntries(SCALES.map((s) => [s, spec.build(asset, s)]));
  writeFileSync(`${OUT}/${id}-plan.json`, JSON.stringify(plans, null, 1));
  console.log(id, SCALES.map((s) => `${s}:${plans[s]!.length}`).join(" "));
  if (mode === "write") {
    insertPlan(path, plans, true);
    for (const s of SCALES) appendDecision(path, s, "silhouette", NOTES[id]![s].en, NOTES[id]![s].de);
  }
}
