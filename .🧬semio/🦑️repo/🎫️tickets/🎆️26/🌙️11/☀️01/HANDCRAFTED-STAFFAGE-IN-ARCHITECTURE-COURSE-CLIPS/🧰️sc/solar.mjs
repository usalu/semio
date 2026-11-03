import { ASHRAE, SRC, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, un } from "../🧰️g2/g2.mjs";
import { ellipse, poly, rect } from "../🧰️g2/lib.mjs";

const TILT = (35 * Math.PI) / 180;
const C = Math.cos(TILT);
const S = Math.sin(TILT);
const L = 2.0;
const T = 0.1;
const FOOT = 0.18;
const XMIN = -(L * C + T * S) / 2;
const P0 = [XMIN, FOOT + T * C];
const W = (a, d) => [P0[0] + a * C + d * S, P0[1] + a * S - d * C];
const UNDER = (x) => FOOT + (x - (XMIN + T * S)) * Math.tan(TILT);
const TOP = W(L, 0)[1];
const WIDTH = L * C + T * S;
const RISER_X = Array.from({ length: 9 }, (_, i) => -0.4 + i * 0.1);
const HATCH = (id) => (s) => ({ ...s, hatch: id });
const INS = [{ id: "insulation", title: tx("Mineral wool in section", "Mineralwolle im Schnitt"), pattern: "lines", angleDeg: 45, spacingMm: 0.5, penMm: 0.1 }];

const sp = space([
  ["stand", null, 0, 0, { title: tx("Mounting stand with base rails", "Montagegestell mit Fußschienen"), lengthM: 1.5, joint: fixed, mass: m(6.4, 0, 0.35) }],
  ["housing", "stand", 0, 0, { title: tx("Collector housing with insulation", "Kollektorwanne mit Dämmung"), lengthM: L, joint: fixed, mass: m(10, ...W(1, 0.09)) }],
  ["glass", "stand", 0, 0, { title: tx("Solar glass cover", "Solarglasabdeckung"), lengthM: L, joint: fixed, mass: m(14, ...W(1, 0.005)) }],
  ["absorber", "stand", 0, 0, { title: tx("Absorber sheet with risers", "Absorberblech mit Steigrohren"), lengthM: L, joint: fixed, mass: m(8, ...W(1, 0.05)) }],
  ["headers", "stand", 0, 0, { title: tx("Inlet and outlet headers", "Sammelrohre Vor- und Rücklauf"), lengthM: L, joint: fixed, mass: m(1.6, ...W(1, 0.052)) }],
]);

const frontY = (a, d) => W(a, d)[1];
const planX = (a, d) => W(a, d)[0];
const strut = (id, x, w, z, tone) => poly(id, "stand", "body", z, true, tone, [[x - w / 2, 0.04], [x + w / 2, 0.04], [x + w / 2, UNDER(x + w / 2) + 0.012], [x - w / 2, UNDER(x - w / 2) + 0.012]], true, { region: "stand", material: "galvanised-steel" });
const stand = (railH, w, z) => [
  rect("base-rail", "stand", "body", z, true, 0.5, -0.75, 0, 1.5, railH, { region: "stand", material: "galvanised-steel" }),
  strut("front-strut", -0.55, w, z, 0.5),
  strut("rear-strut", 0.55, w, z, 0.5),
];
const tray = (wall, back, z, region = "housing") => poly("tray", "housing", "body", z, true, 0.35, [W(0, 0), W(0, T), W(L, T), W(L, 0), W(L - wall, 0), W(L - wall, T - back), W(wall, T - back), W(wall, 0)], true, { region, material: "anodised-aluminium" });
const box = (z, regions) => poly("collector-box", "housing", "body", z, true, 0.45, [W(0, 0), W(0, T), W(L, T), W(L, 0)], true, { regions, material: "anodised-aluminium" });

const elev50 = [
  ...stand(0.05, 0.04, 3.0),
  tray(0.02, 0.012, 3.2),
  HATCH("insulation")(poly("insulation", "housing", "body", 3.3, true, 0.8, [W(0.019, 0.062), W(0.019, 0.09), W(L - 0.019, 0.09), W(L - 0.019, 0.062)], true, { region: "housing", material: "mineral-wool" })),
  poly("glass", "glass", "glazing", 3.4, true, 0.92, [W(0.019, 0), W(0.019, 0.01), W(L - 0.019, 0.01), W(L - 0.019, 0)], true, { region: "glazing", material: "solar-glass" }),
  poly("absorber-plate", "absorber", "body", 3.5, true, 0.2, [W(0.06, 0.042), W(0.06, 0.05), W(L - 0.06, 0.05), W(L - 0.06, 0.042)], true, { region: "absorber", material: "selective-absorber" }),
  poly("riser-strip", "absorber", "detail", 3.55, true, 0.55, [W(0.06, 0.05), W(0.06, 0.062), W(L - 0.06, 0.062), W(L - 0.06, 0.05)], true, { region: "absorber", material: "copper" }),
  ellipse("header-lower", "headers", "body", 3.6, true, 0.4, ...W(0.06, 0.052), 0.0165, 0.0165, { region: "header-lower", material: "copper" }),
  ellipse("header-upper", "headers", "body", 3.6, true, 0.4, ...W(L - 0.06, 0.052), 0.0165, 0.0165, { region: "header-upper", material: "copper" }),
];
const elev100 = [
  ...stand(0.06, 0.05, 3.0),
  tray(0.025, 0.015, 3.2),
  poly("insulation", "housing", "body", 3.3, true, 0.8, [W(0.024, 0.066), W(0.024, 0.085), W(L - 0.024, 0.085), W(L - 0.024, 0.066)], true, { region: "housing", material: "mineral-wool" }),
  poly("glass", "glass", "glazing", 3.4, true, 0.92, [W(0.024, 0), W(0.024, 0.012), W(L - 0.024, 0.012), W(L - 0.024, 0)], true, { region: "glazing", material: "solar-glass" }),
  poly("absorber-strip", "absorber", "body", 3.5, true, 0.2, [W(0.06, 0.04), W(0.06, 0.062), W(L - 0.06, 0.062), W(L - 0.06, 0.04)], true, { region: "absorber", material: "selective-absorber" }),
  ellipse("header-lower", "headers", "body", 3.6, true, 0.4, ...W(0.06, 0.052), 0.022, 0.022, { region: "header-lower", material: "copper" }),
  ellipse("header-upper", "headers", "body", 3.6, true, 0.4, ...W(L - 0.06, 0.052), 0.022, 0.022, { region: "header-upper", material: "copper" }),
];
const elev200 = [
  ...stand(0.08, 0.08, 3.0),
  box(3.4, ["housing", "glazing", "absorber", "header-lower", "header-upper"]),
];

const FY0 = frontY(0, 0.1);
const FY_LIP0 = frontY(0, 0);
const frontCommon = (lip, side, z, tone, headerH, riserW, withRisers, role) => [
  rect("front-feet-left", "stand", "body", z - 0.5, true, 0.5, -0.4 - 0.025, 0, 0.05, FOOT, { region: "stand", material: "galvanised-steel" }),
  rect("front-feet-right", "stand", "body", z - 0.5, true, 0.5, 0.4 - 0.025, 0, 0.05, FOOT, { region: "stand", material: "galvanised-steel" }),
  rect("front-end-face", "housing", "body", z, true, 0.35, -0.5, FY0, 1.0, FY_LIP0 - FY0 + lip, { region: "housing", material: "anodised-aluminium" }),
  rect("front-side-left", "housing", "body", z, true, 0.35, -0.5, FY_LIP0, side, TOP - FY_LIP0, { region: "housing", material: "anodised-aluminium" }),
  rect("front-side-right", "housing", "body", z, true, 0.35, 0.5 - side, FY_LIP0, side, TOP - FY_LIP0, { region: "housing", material: "anodised-aluminium" }),
  rect("front-top-rail", "housing", "body", z, true, 0.35, -0.5 + side, TOP - lip, 1.0 - 2 * side, lip, { region: "housing", material: "anodised-aluminium" }),
  rect("front-glass", "glass", withRisers ? "glazing" : "body", z + 0.1, true, 0.92, -0.5 + side, FY_LIP0 + lip, 1.0 - 2 * side, TOP - FY_LIP0 - 2 * lip, { region: "glazing", material: "solar-glass" }),
  rect("front-absorber", "absorber", "body", z + 0.2, true, 0.2, -0.5 + side + 0.015, FY_LIP0 + lip + 0.006, 1.0 - 2 * side - 0.03, TOP - FY_LIP0 - 2 * lip - 0.03, { region: "absorber", material: "selective-absorber" }),
  ...(withRisers ? RISER_X.map((x, i) => rect(`front-riser-${i + 1}`, "absorber", role, z + 0.3, true, 0.55, x - riserW / 2, FY_LIP0 + lip + 0.012, riserW, TOP - FY_LIP0 - 2 * lip - 0.042, { region: "absorber", material: "copper" })) : []),
  rect("front-header-upper", "headers", "body", z + 0.4, true, 0.4, -0.5 + side + 0.03, frontY(L - 0.06, 0.052) - headerH / 2, 1.0 - 2 * side - 0.06, headerH, { region: "header-upper", material: "copper" }),
];
const front50 = frontCommon(0.017, 0.03, 3.2, 0.35, 0.033, 0.012, true, "detail");
const front100 = frontCommon(0.02, 0.035, 3.2, 0.35, 0.044, 0.02, true, "detail");
const front200 = frontCommon(0.04, 0.05, 3.2, 0.35, 0.045, 0.02, false, "body");

const PX0 = XMIN;
const PX1 = XMIN + WIDTH;
const planCommon = (side, z, headerW, riserW, withRisers, role, lip) => [
  rect("plan-box", "housing", "body", z, true, 0.35, PX0, -0.5, WIDTH, 1.0, { region: "housing", material: "anodised-aluminium" }),
  rect("plan-glass", "glass", withRisers ? "glazing" : "body", z + 0.1, true, 0.92, planX(lip, 0), -0.5 + side, (L - 2 * lip) * C, 1.0 - 2 * side, { region: "glazing", material: "solar-glass" }),
  rect("plan-absorber", "absorber", "body", z + 0.2, true, 0.2, planX(0.06, 0.046), -0.5 + side + 0.015, (L - 0.12) * C, 1.0 - 2 * side - 0.03, { region: "absorber", material: "selective-absorber" }),
  ...(withRisers ? RISER_X.map((y, i) => rect(`plan-riser-${i + 1}`, "absorber", role, z + 0.3, true, 0.55, planX(0.06, 0.056), y - riserW / 2, (L - 0.12) * C, riserW, { region: "absorber", material: "copper" })) : []),
  rect("plan-header-lower", "headers", "body", z + 0.4, true, 0.4, planX(0.06, 0.052) - headerW / 2, -0.5 + side + 0.03, headerW, 1.0 - 2 * side - 0.06, { region: "header-lower", material: "copper" }),
  rect("plan-header-upper", "headers", "body", z + 0.4, true, 0.4, planX(L - 0.06, 0.052) - headerW / 2, -0.5 + side + 0.03, headerW, 1.0 - 2 * side - 0.06, { region: "header-upper", material: "copper" }),
];
const plan50 = planCommon(0.03, 3.2, 0.033, 0.012, true, "detail", 0.02);
const plan100 = planCommon(0.035, 3.2, 0.044, 0.02, true, "detail", 0.025);
const plan200 = planCommon(0.05, 3.2, 0.045, 0.02, false, "body", 0.04);

const withViews = (r, front, plan) => ({ ...r, geometry: { ...r.geometry, front, plan } });

const P50 = { body: 0.18, detail: 0.13, texture: 0.1, glazing: 0.13, ground: 0.35 };
const P100 = { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.25 };
const P200 = { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 };

const rep50 = { ...withViews(rep(50, [20, 75], P50, elev50, dec(
  ["A 2.0 m by 1.0 m flat-plate collector tilted by 35 degrees on a steel stand, 34 mm wide and 28 mm tall on paper: the section through the slope shows the aluminium tray with insulation, the glass cover, the absorber sheet with its riser strip and a header circle at each end.", "Ein 2,0 m mal 1,0 m großer Flachkollektor, 35 Grad geneigt auf einem Stahlgestell, auf Papier 34 mm breit und 28 mm hoch: der Schnitt durch die Neigungsebene zeigt die Aluminiumwanne mit Dämmung, die Glasabdeckung, das Absorberblech mit Steigrohrstreifen und je einen Sammlerkreis an den Enden."],
  ["Tray with end walls, mineral wool with hatch, glass, absorber sheet, riser strip, both headers, base rail and two struts. Front and plan add the frame ring, nine risers and the headers.", "Wanne mit Stirnwänden, Mineralwolle mit Schraffur, Glas, Absorberblech, Steigrohrstreifen, beide Sammler, Fußschiene und zwei Stützen. Vorderansicht und Grundriss ergänzen den Rahmenring, neun Steigrohre und die Sammler."],
  ["Nothing is merged; the nine risers share the strip of the section.", "Nichts ist zusammengeführt; die neun Steigrohre teilen sich den Streifen im Schnitt."],
  ["The 3 mm tray sheet joints, the sealing gasket, the back rail brackets and the absorber coating.", "Die Blechstöße der 3 mm Wanne, die Dichtung, die Halterungen der Rückschiene und die Absorberbeschichtung."],
  ["The 4 mm glass is drawn 10 mm thick, the 0.5 mm absorber sheet 8 mm, the 1.5 mm tray sheet 12 to 20 mm, the 10 mm risers 12 mm; the 40 mm struts are real size.", "Das 4 mm Glas ist 10 mm dick gezeichnet, das 0,5 mm Absorberblech 8 mm, das 1,5 mm Wannenblech 12 bis 20 mm, die 10 mm Steigrohre 12 mm; die 40 mm Stützen haben reale Größe."],
  ["Housing, glazing, absorber, two headers and stand are six regions: in the thermal view the absorber and the upper header glow hot behind the cooler glass, the lower header stays cooler while the pump runs.", "Wanne, Glas, Absorber, zwei Sammler und Gestell sind sechs Regionen: in der Thermalansicht glühen Absorber und oberer Sammler heiß hinter dem kühleren Glas, der untere Sammler bleibt bei laufender Pumpe kühler."],
  P50,
)), front50, plan50), hatches: INS };

const rep100 = { ...withViews(rep(100, [75, 150], P100, elev100, dec(
  ["A 17 mm by 14 mm collector: tray, glass, one absorber strip and a header circle at each end on two struts.", "Ein 17 mm mal 14 mm großer Kollektor: Wanne, Glas, ein Absorberstreifen und je ein Sammlerkreis an den Enden auf zwei Stützen."],
  ["Tray, plain insulation block, glass, absorber strip, both headers, base rail and struts.", "Wanne, einfacher Dämmblock, Glas, Absorberstreifen, beide Sammler, Fußschiene und Stützen."],
  ["Absorber sheet and risers are one strip of 22 mm.", "Absorberblech und Steigrohre sind ein Streifen von 22 mm."],
  ["The insulation hatch, gasket and brackets.", "Die Dämmschraffur, die Dichtung und die Halterungen."],
  ["Glass 12 mm, tray walls 25 mm and back 15 mm, headers 44 mm across, struts 50 mm.", "Glas 12 mm, Wannenwände 25 mm und Rücken 15 mm, Sammler 44 mm weit, Stützen 50 mm."],
  ["Same six regions as at 1:50.", "Dieselben sechs Regionen wie bei 1:50."],
  P100,
)), front100, plan100), hatches: [] };

const rep200 = { ...withViews(rep(200, [150, 350], P200, elev200, dec(
  ["An 8 mm by 7 mm collector: one tilted slab on two struts and a base rail.", "Ein 8 mm mal 7 mm großer Kollektor: eine geneigte Platte auf zwei Stützen und einer Fußschiene."],
  ["Slab, base rail and two struts; front and plan add the glass field, absorber and upper header.", "Platte, Fußschiene und zwei Stützen; Vorderansicht und Grundriss ergänzen Glasfeld, Absorber und oberen Sammler."],
  ["Housing, glass, absorber and both headers are one slab (merged regions).", "Wanne, Glas, Absorber und beide Sammler sind eine Platte (zusammengeführte Regionen)."],
  ["Insulation, risers, glass edge and all section detail.", "Dämmung, Steigrohre, Glaskante und alle Schnittdetails."],
  ["Struts are 80 mm, the rail 80 mm high.", "Die Stützen sind 80 mm, die Schiene 80 mm hoch."],
  ["The slab lists five regions as one merged form.", "Die Platte führt fünf Regionen als eine zusammengeführte Form."],
  P200,
)), front200, plan200), hatches: [] };

const regions = [
  reg("housing", ["Housing with insulation", "Wanne mit Dämmung"], "surface", "passive", "anodised-aluminium", 0.8, 1.0, "vertical-plate", 0.0015, 0.0001, 5000, {}, { sky: 0.6 }),
  reg("glazing", ["Glass cover", "Glasabdeckung"], "glazing", "passive", "solar-glass", 1.8, 1.9, "horizontal-plate-up", 0.004, 0.004, 7500, {}, { sky: 0.9, layerKind: "glass", layerTitle: ["Low-iron solar glass 4 mm", "Eisenarmes Solarglas 4 mm"] }),
  reg("absorber", ["Absorber with risers", "Absorber mit Steigrohren"], "heater", "operating", "selective-absorber", 1.8, 1.9, "horizontal-plate-up", 0.0005, 0.00001, 6000, { operating: { key: "flow", heatW: -620 } }, { sky: 0.9 }),
  reg("header-lower", ["Inlet header", "Vorlauf-Sammler"], "surface", "operating", "copper", 0.066, 0.033, "cylinder", 0.001, 0.00003, 12000, { operating: { key: "fluid", heatW: 8 } }, { sky: 0.5 }),
  reg("header-upper", ["Outlet header", "Rücklauf-Sammler"], "surface", "operating", "copper", 0.066, 0.033, "cylinder", 0.001, 0.00003, 12000, { operating: { key: "fluid", heatW: 22 } }, { sky: 0.5 }),
  reg("stand", ["Mounting stand", "Montagegestell"], "surface", "passive", "galvanised-steel", 0.45, 0.6, "vertical-plate", 0.002, 0.00004, 4000, {}, { sky: 0.5 }),
];
const bind = bindOf({
  housing: { exposedFraction: 0.7 },
  glazing: { exposedFraction: 1 },
  absorber: { exposedFraction: 1 },
  "header-lower": { exposedFraction: 0.5 },
  "header-upper": { exposedFraction: 0.5 },
  stand: { exposedFraction: 1 },
});
const SUN = { housing: { sunlitFraction: 0.6 }, glazing: { sunlitFraction: 1 }, absorber: { sunlitFraction: 1 }, "header-lower": { sunlitFraction: 0.6 }, "header-upper": { sunlitFraction: 0.6 }, stand: { sunlitFraction: 0.3 } };
const sup = ["base-contact"];
const poses = [
  pose("night-idle", ["Night, idle", "Nacht, ruhend"], ["No sun and no flow: the whole collector sits at the air temperature.", "Keine Sonne und kein Durchfluss: der ganze Kollektor hat Lufttemperatur."], {}, sup, "auto", bind({ flow: 0, fluid: 0 }, { "absorber-plume": 0 })),
  pose("operating", ["Operating", "Betrieb"], ["The sun shines and the pump runs: the fluid carries the heat away, the absorber stays near 75 degC and the outlet header runs warmer than the inlet header.", "Die Sonne scheint und die Pumpe läuft: das Fluid führt die Wärme ab, der Absorber bleibt bei etwa 75 degC und der Rücklauf-Sammler ist wärmer als der Vorlauf-Sammler."], {}, sup, "auto", bind({ flow: 1, fluid: 1 }, { "absorber-plume": 0.5 }, SUN)),
  pose("stagnation", ["Stagnation", "Stillstand"], ["The sun shines but the pump has stopped: nothing carries the heat away, the absorber heats towards its stagnation temperature and the fluid in the headers turns to vapour.", "Die Sonne scheint, aber die Pumpe steht: nichts führt die Wärme ab, der Absorber erwärmt sich auf seine Stillstandstemperatur und das Fluid in den Sammlern verdampft."], {}, sup, "auto", bind({ flow: 0, fluid: 3 }, { "absorber-plume": 1 }, SUN)),
];
const SUN_TRACKS = (a, b) => Object.keys(SUN).map((id) => [`region:${id}:sunlitFraction`, "smooth", [[0, a], [5, a], [6, b]]]);
const clips = [
  clip("stagnation", ["Stagnation", "Stillstand"], ["The sun comes up with the pump off: the cold collector heats towards its stagnation temperature within five minutes.", "Die Sonne geht auf bei stehender Pumpe: der kalte Kollektor erwärmt sich binnen fünf Minuten auf seine Stillstandstemperatur."],
    300, 1,
    [{ t: 0, pose: "night-idle", ease: "step" }, { t: 5, pose: "stagnation", ease: "step" }, { t: 300, pose: "stagnation", ease: "step" }],
    contacts(sup, 300), [["dark", 0, 5, ["Dark", "Dunkel"]], ["stagnating", 5, 300, ["Stagnating", "Stillstand"]]], [[5, "thermal", ["Sun on the glass", "Sonne auf dem Glas"], "flow"]],
    [...SUN_TRACKS(0, 1), ["operating:flow", "smooth", [[0, 0], [300, 0]]], ["operating:fluid", "smooth", [[0, 0], [5, 0], [200, 3]]], ["emitter:absorber-plume", "smooth", [[0, 0], [5, 0], [200, 1]]]]),
  clip("start-up", ["Start-up", "Anlaufen"], ["The sun comes up, the pump starts after a few seconds and the collector settles at its operating temperature while the outlet header runs warmer than the inlet header.", "Die Sonne geht auf, die Pumpe startet nach wenigen Sekunden und der Kollektor stellt sich auf seine Betriebstemperatur ein, während der Rücklauf-Sammler wärmer läuft als der Vorlauf-Sammler."],
    300, 1,
    [{ t: 0, pose: "night-idle", ease: "step" }, { t: 5, pose: "operating", ease: "step" }, { t: 300, pose: "operating", ease: "step" }],
    contacts(sup, 300), [["dark", 0, 5, ["Dark", "Dunkel"]], ["pumping", 5, 300, ["Pumping", "Pumpen"]]], [[5, "operating", ["Pump starts", "Pumpe startet"], "flow"]],
    [...SUN_TRACKS(0, 1), ["operating:flow", "smooth", [[0, 0], [5, 0], [6, 1]]], ["operating:fluid", "smooth", [[0, 0], [5, 0], [30, 1]]], ["emitter:absorber-plume", "smooth", [[0, 0], [5, 0], [120, 0.5]]]]),
  clip("operating", ["Pump starts", "Pumpe startet"], ["The hot, stagnating collector gets its flow back: the absorber cools from the stagnation temperature to the operating temperature as the fluid carries the heat away.", "Der heiße, stillstehende Kollektor bekommt seinen Durchfluss zurück: der Absorber kühlt von der Stillstandstemperatur auf die Betriebstemperatur ab, weil das Fluid die Wärme abführt."],
    300, 1,
    [{ t: 0, pose: "stagnation", ease: "step" }, { t: 10, pose: "operating", ease: "step" }, { t: 300, pose: "operating", ease: "step" }],
    contacts(sup, 300), [["stagnating", 0, 10, ["Stagnating", "Stillstand"]], ["pumping", 10, 300, ["Pumping", "Pumpen"]]], [[10, "operating", ["Pump starts", "Pumpe startet"], "flow"]],
    [["operating:flow", "smooth", [[0, 0], [10, 0], [11, 1]]], ["operating:fluid", "smooth", [[0, 3], [10, 3], [60, 1]]], ["emitter:absorber-plume", "smooth", [[0, 1], [10, 1], [120, 0.5]]]]),
];

export const solar = () => {
  const doc = mkAsset({
    id: "solar-thermal-collector",
    category: "machinery",
    title: ["Flat-Plate Solar Thermal Collector", "Flachkollektor (Solarthermie)"],
    summary: [
      "A 2.0 m by 1.0 m glazed flat-plate solar thermal collector of 40 kg, tilted by 35 degrees on a steel stand: aluminium tray with mineral wool, solar glass, selective absorber with nine risers, inlet and outlet headers.",
      "Ein verglaster Flachkollektor von 2,0 m mal 1,0 m und 40 kg, 35 Grad geneigt auf einem Stahlgestell: Aluminiumwanne mit Mineralwolle, Solarglas, selektiver Absorber mit neun Steigrohren, Vor- und Rücklauf-Sammler.",
    ],
    tags: ["solar-thermal", "flat-plate-collector", "collector", "absorber", "stagnation", "dec", "solar-heat", "machinery"],
    references: [
      ref("iso-9806-2017", "ISO 9806:2017 Solar energy - Solar thermal collectors - Test methods (peak efficiency, loss coefficients a1 and a2, stagnation temperature)."),
      ref("duffie-beckman-2013", "Duffie, J. A., Beckman, W. A. (2013). Solar Engineering of Thermal Processes, 4th ed., Wiley (flat-plate collector construction, loss coefficients, stagnation)."),
      ASHRAE,
    ],
    notes: [
      "A typical glazed flat-plate collector of 2.0 m by 1.0 m by 0.1 m (gross area 2.0 m2, absorber area 1.8 m2) and 40 kg: a 1.5 mm anodised aluminium tray with 26 mm of mineral wool, a 4 mm low-iron solar glass cover, a selective absorber sheet of 0.5 mm with nine 10 mm copper risers at 0.1 m pitch and two 22 mm copper headers (inlet below, outlet above), mounted at 35 degrees on a 6.4 kg steel stand. The masses are tray 10 kg, glass 14 kg, absorber 8 kg, headers 1.6 kg and stand 6.4 kg. The optical efficiency is 0.78 and the loss coefficients a1 = 3.6 W/(m2 K) and a2 = 0.015 W/(m2 K2), so the stagnation temperature at 1000 W/m2 is about 150 degC and the useful gain at 60 degC mean fluid temperature about 1 kW (ISO 9806 style). In the thermal model the absorber takes the solar share from the environment (effective absorptivity 0.86 behind the glass), and the operating key flow removes 620 W at the pumped state, which is the mean useful gain under the model's solar geometry factor; the key fluid heats the headers (inlet 8 W, outlet 22 W, three times that when the fluid stands and vapour fills them). Reference environment for the thermal view: summer-heatwave-calm. The elevation is the section through the slope, the front view looks at the low edge, the plan from above. The origin is the centre of the base rails on the roof.",
      "Ein typischer verglaster Flachkollektor von 2,0 m mal 1,0 m mal 0,1 m (Bruttofläche 2,0 m2, Absorberfläche 1,8 m2) und 40 kg: eine 1,5 mm dicke eloxierte Aluminiumwanne mit 26 mm Mineralwolle, eine 4 mm dicke eisenarme Solarglasabdeckung, ein selektives Absorberblech von 0,5 mm mit neun 10 mm dicken Kupfer-Steigrohren im Abstand von 0,1 m und zwei 22 mm dicken Kupfer-Sammelrohren (Vorlauf unten, Rücklauf oben), mit 35 Grad Neigung auf einem 6,4 kg schweren Stahlgestell montiert. Die Massen sind Wanne 10 kg, Glas 14 kg, Absorber 8 kg, Sammler 1,6 kg und Gestell 6,4 kg. Der optische Wirkungsgrad beträgt 0,78 und die Verlustkoeffizienten a1 = 3,6 W/(m2 K) und a2 = 0,015 W/(m2 K2), sodass die Stillstandstemperatur bei 1000 W/m2 etwa 150 degC und der Nutzgewinn bei 60 degC mittlerer Fluidtemperatur etwa 1 kW beträgt (Auslegung nach ISO 9806). Im Thermalmodell nimmt der Absorber den Solaranteil aus der Umgebung auf (wirksamer Absorptionsgrad 0,86 hinter dem Glas), und der Betriebsschlüssel flow entzieht 620 W im gepumpten Zustand, dem mittleren Nutzgewinn bei dem Solargeometriefaktor des Modells; der Schlüssel fluid heizt die Sammler (Vorlauf 8 W, Rücklauf 22 W, das Dreifache bei stehendem Fluid und Dampf in den Sammlern). Bezugsumgebung für die Thermalansicht: summer-heatwave-calm. Die Ansicht ist der Schnitt durch die Neigungsebene, die Vorderansicht blickt auf die untere Kante, der Grundriss von oben. Der Ursprung ist die Mitte der Fußschienen auf dem Dach.",
    ],
    dims: [Math.round(TOP * 1e4) / 1e4, Math.round(WIDTH * 1e4) / 1e4, 1.0],
    materials: [
      materialOf("anodised-aluminium", "Anodised aluminium", "Eloxiertes Aluminium", 0.8, 0.7, 2700, 237, 900, "literature", "incropera-2018"),
      materialOf("solar-glass", "Low-iron solar glass", "Eisenarmes Solarglas", 0.9, 0.05, 2500, 1.0, 750, "literature", "incropera-2018"),
      materialOf("selective-absorber", "Selective absorber coating behind glass", "Selektive Absorberbeschichtung hinter Glas", 0.12, 0.86, 2700, 237, 900, "authored-estimate"),
      materialOf("copper", "Copper tube", "Kupferrohr", 0.3, 0.4, 8900, 390, 385, "literature", "incropera-2018"),
      materialOf("mineral-wool", "Mineral wool", "Mineralwolle", 0.9, 0.6, 50, 0.035, 840, "literature", "incropera-2018"),
      materialOf("galvanised-steel", "Galvanised steel", "Verzinkter Stahl", 0.25, 0.5, 7800, 50, 480, "literature", "incropera-2018"),
    ],
    root: "stand",
    bones: sp.bones,
    anchors: [baseContact("stand", 0, 0, 1.5, 0.9), sp.anchor("glass-top", "glass", ...W(1.9, 0), "emitter", ["Upper edge of the glass", "Oberkante des Glases"])],
    reps: { 500: un(500, "the 100 mm box and the 40 mm struts", "der 100 mm dicke Kasten und die 40 mm Stützen", 0.04, 3.4, [200, 100]), 200: rep200, 100: rep100, 50: rep50 },
    poses,
    clips,
    operating: [
      { id: "flow", title: tx("Pump flow", "Pumpendurchfluss"), levels: [level(0, "Pump off (stagnation)", "Pumpe aus (Stillstand)"), level(1, "Pump running", "Pumpe läuft")] },
      { id: "fluid", title: tx("Fluid state in the headers", "Fluidzustand in den Sammlern"), levels: [level(0, "Cold", "Kalt"), level(1, "Pumped liquid", "Gepumpte Flüssigkeit"), level(3, "Stagnation vapour", "Stillstandsdampf")] },
    ],
    regions,
    emitters: [{ id: "absorber-plume", kind: "plume", title: tx("Plume above the glass", "Fahne über dem Glas"), anchor: "glass-top", directionDeg: 90, diameterM: 0.5, region: "absorber", source: SRC, reference: "incropera-2018" }],
    assumptions: [
      ["The glass cover, the 25 mm air gap and the radiation shield are not solved as layers: the absorber takes the sun and the convective loss of an open surface, so the stagnation temperature is lower than the 150 degC of a tested collector unless the environment is calm and hot.", "Glasabdeckung, 25 mm Luftspalt und Strahlungsschirm sind nicht als Schichten gelöst: der Absorber nimmt Sonne und konvektiven Verlust einer offenen Fläche auf, sodass die Stillstandstemperatur unter den 150 degC eines geprüften Kollektors liegt, außer die Umgebung ist ruhig und heiß."],
      ["The heat carrier is not an emitter: the model carries air and vapour only, so the pump flow shows as the cooler absorber and the cooler inlet header against the warmer outlet header.", "Der Wärmeträger ist kein Emitter: das Modell führt nur Luft und Dampf, daher zeigt sich der Pumpendurchfluss als kühlerer Absorber und kühlerer Vorlauf-Sammler gegen den wärmeren Rücklauf-Sammler."],
    ],
    structure: [["The 40 kg collector with its stand rests on two base rails: a unilateral contact of 1.5 m by 0.9 m carries the weight; wind and snow loads are not part of the asset.", "Der 40 kg schwere Kollektor mit Gestell ruht auf zwei Fußschienen: ein einseitiger Kontakt von 1,5 m mal 0,9 m trägt das Gewicht; Wind- und Schneelasten gehören nicht zum Asset."]],
    defaultPose: "operating",
    defaultClip: "operating",
  });
  doc.asset.presentation.viewpoints = ["elevation", "front", "plan"];
  return doc;
};
