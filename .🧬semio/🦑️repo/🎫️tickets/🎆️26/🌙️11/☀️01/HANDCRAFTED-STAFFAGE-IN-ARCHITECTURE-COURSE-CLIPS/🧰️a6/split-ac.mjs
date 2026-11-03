import { ALL_EXPORTS, decisions, level, path, rect, ref, tx, unavailable, wrap } from "./lib.mjs";
import { capsule } from "./car-shapes.mjs";

const EN14511 = ref("en-14511", "EN 14511-2:2022 Air conditioners, liquid chilling packages and heat pumps for space heating and cooling, test conditions (indoor 27 C dry bulb and 19 C wet bulb, outdoor 35 C, cooling capacity and electric input).");
const ISO5151 = ref("iso-5151-2017", "ISO 5151:2017 Non-ducted air conditioners and heat pumps, testing and rating for performance (rated cooling capacity, sensible heat ratio, indoor air flow).");
const INCROPERA = ref("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (natural convection at vertical and horizontal plates, radiative properties of plastics).");
const ASHRAE = ref("ashrae-2021", "ASHRAE Handbook Fundamentals (2021), Chapter 1 Psychrometrics and Chapter 4 Heat Transfer (air properties, sensible and latent cooling of an air stream).");
const CLASS = ref("split-ac-class-data", "Manufacturer class data of wall-mounted (high-wall) split indoor units of 2.5 kW cooling capacity: about 0.80 to 0.90 m wide, 0.29 to 0.32 m high, 0.20 to 0.22 m deep, 8 to 10 kg, 400 to 500 m3/h air flow, 0.6 to 0.8 kW electric input of the whole system (EER about 3.5). Quoted as orders of magnitude from the public data-sheet families of the product class, not retrieved for this asset; every derived number is tagged authored-estimate.");

const mat = (id, en, de, emissivity, solarAbsorptivity, densityKgM3, conductivityWmK, source, reference) => ({
  id,
  title: tx(en, de),
  emissivity,
  solarAbsorptivity,
  densityKgM3,
  conductivityWmK,
  source,
  ...(reference ? { reference } : {}),
});

const materials = [
  mat("abs-white", "White ABS casing", "Weißes ABS-Gehäuse", 0.92, 0.25, 1050, 0.17, "literature", "incropera-2018"),
  mat("abs-grey", "Grey ABS grille and vane", "Graues ABS-Gitter und Lamelle", 0.92, 0.6, 1050, 0.17, "literature", "incropera-2018"),
  mat("display", "Dark display window", "Dunkles Anzeigefenster", 0.9, 0.9, 1200, 0.2, "authored-estimate"),
];

const bone = (id, parent, title, x, y, lengthM, joint, mass) => ({ id, parent, title: tx(...title), x, y, angleDeg: 0, lengthM, joint, mass: { ...mass, distribution: "point" } });

const rig = {
  kind: "mechanical",
  root: "unit",
  bones: [
    bone("unit", null, ["Casing with coil, fan and wall plate", "Gehäuse mit Wärmetauscher, Lüfter und Wandplatte"], 0, 0, 0.21, { type: "fixed" }, { kg: 8.55, comX: 0.1, comY: 0.15, source: "authored-estimate", reference: "split-ac-class-data" }),
    { ...bone("vane", "unit", ["Discharge vane", "Ausblaslamelle"], 0.065, 0.015, 0.125, { type: "revolute", minDeg: -55, maxDeg: 0 }, { kg: 0.45, comX: 0.06, comY: 0, source: "authored-estimate", reference: "split-ac-class-data" }) },
  ],
};

const anchors = [
  { id: "wall-hook", bone: "unit", x: 0.012, y: 0.27, kind: "support", title: tx("Upper hook on the wall plate", "Oberer Haken an der Wandplatte"), support: "pin", normalDeg: 0, footprintLengthM: 0.02, footprintWidthM: 0.1 },
  { id: "wall-rest", bone: "unit", x: 0, y: 0.02, kind: "support", title: tx("Lower rest against the wall", "Untere Anlage an der Wand"), support: "roller", normalDeg: 0, footprintLengthM: 0.02, footprintWidthM: 0.1, frictionCoefficient: 0.4 },
  { id: "jet-outlet", bone: "unit", x: 0.13, y: 0.02, kind: "emitter", title: tx("Centre of the discharge slot", "Mitte des Ausblasschlitzes") },
];

const abs = (reference) => ({
  id: "shell",
  title: tx("ABS shell 2 mm", "ABS-Schale 2 mm"),
  kind: "polymer",
  material: "abs-white",
  thicknessM: 0.002,
  resistanceM2KW: 0.0118,
  heatCapacityJM2K: 2900,
  source: "literature",
  reference,
});

const region = (id, title, kind, model, material, areaM2, lengthM, geometry, sky, layers, capacity, source, reference, extra = {}) => ({
  id,
  title: tx(...title),
  kind,
  model,
  material,
  areaM2,
  characteristicLengthM: lengthM,
  geometry,
  skyViewFactor: sky,
  layers,
  heatCapacityJM2K: capacity,
  ...extra,
  source,
  ...(reference ? { reference } : {}),
});

const regions = [
  region("casing", ["Front panel and end caps", "Frontblende und Endkappen"], "surface", "operating", "abs-white", 0.62, 0.29, "vertical-plate", 0.3, [abs("incropera-2018")], 2900, "authored-estimate", "en-14511", { operating: { key: "cooling", heatW: -15 } }),
  region("intake-grille", ["Air intake grille on top", "Lufteinlassgitter oben"], "surface", "passive", "abs-grey", 0.1, 0.12, "horizontal-plate-up", 0.4, [{ ...abs("incropera-2018"), material: "abs-grey" }], 2900, "authored-estimate", "incropera-2018"),
  region("discharge-vane", ["Discharge vane in the cold jet", "Ausblaslamelle im kalten Strahl"], "surface", "operating", "abs-grey", 0.1, 0.12, "vertical-plate", 0.3, [{ ...abs("incropera-2018"), material: "abs-grey" }], 2900, "authored-estimate", "en-14511", { operating: { key: "cooling", heatW: -8 } }),
];

const thermal = {
  environment: { airTempC: 27, meanRadiantTempC: 27, relativeHumidity: 0.47, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
  passengers: [],
  operating: [
    { id: "cooling", title: tx("Cooling duty of the evaporator", "Kühlbetrieb des Verdampfers"), levels: [level(0, "Off", "Aus"), level(0.4, "Part load (1.0 kW)", "Teillast (1,0 kW)"), level(1, "Rated (2.5 kW cooling, 0.7 kW electric)", "Nennlast (2,5 kW Kühlung, 0,7 kW elektrisch)")] },
    { id: "fan", title: tx("Indoor fan speed", "Drehzahl des Innenlüfters"), levels: [level(0, "Stopped", "Stillstand"), level(0.5, "Low (225 m3/h)", "Niedrig (225 m3/h)"), level(1, "High (450 m3/h)", "Hoch (450 m3/h)")] },
  ],
  regions,
  emitters: [
    { id: "cold-jet", kind: "fan", title: tx("Cold air jet from the discharge slot", "Kalter Luftstrahl aus dem Ausblasschlitz"), anchor: "jet-outlet", directionDeg: -35, diameterM: 0.15, operatingKey: "fan", volumeFlowM3S: 0.125, temperatureRiseK: -13.3, vapourKgPerKg: 0, source: "authored-estimate", reference: "iso-5151-2017" },
  ],
  assumptions: [
    tx(
      "Rated cooling per EN 14511 and ISO 5151 (27 C / 19 C wet bulb indoor, 35 C outdoor): 2.5 kW total cooling capacity with a sensible heat ratio of 0.8 is 2.0 kW sensible and 0.5 kW latent, at 0.7 kW electric input of the whole system (EER 3.6). The indoor unit extracts the 2.5 kW from the room air at the evaporator: 450 m3/h = 0.125 m3/s with rho cp V = 151 W/K cools the air by 13.3 K (2.0 kW).",
      "Nennkühlleistung nach EN 14511 und ISO 5151 (innen 27 C / 19 C Feuchtkugel, außen 35 C): 2,5 kW Gesamtkühlleistung bei einem Sensiblen-Wärme-Verhältnis von 0,8 sind 2,0 kW sensibel und 0,5 kW latent, bei 0,7 kW elektrischer Aufnahme des gesamten Systems (EER 3,6). Das Innengerät entzieht der Raumluft am Verdampfer die 2,5 kW: 450 m3/h = 0,125 m3/s mit rho cp V = 151 W/K kühlen die Luft um 13,3 K ab (2,0 kW).",
    ),
    tx(
      "The 0.5 kW latent share condenses at the coil and drains outdoors; the emitter schema has no negative moisture change, so the jet carries the sensible cooling only (vapour change 0). The visible plastic surfaces are cooled only slightly by the cold air behind them: front panel -15 W and vane -8 W at level 1. The 0.67 kW compressor power is released outdoors, the 0.03 kW fan power is part of the air stream.",
      "Der latente Anteil von 0,5 kW kondensiert am Verdampfer und wird nach außen abgeleitet; das Quellenschema kennt keine negative Feuchteänderung, der Strahl trägt daher nur die sensible Kühlung (Feuchteänderung 0). Die sichtbaren Kunststoffflächen werden von der kalten Luft dahinter nur leicht gekühlt: Frontblende -15 W und Lamelle -8 W bei Stufe 1. Die Verdichterleistung von 0,67 kW wird im Freien abgegeben, die Lüfterleistung von 0,03 kW steckt im Luftstrom.",
    ),
  ],
};

const reg = (exposed) => ({ exposedFraction: exposed, sunlitFraction: 0 });
const binding = (cooling, fan, jet) => ({
  operating: { cooling, fan },
  regions: { casing: reg(1), "intake-grille": reg(1), "discharge-vane": reg(1) },
  emitters: { "cold-jet": jet },
});

const pose = (id, title, description, bones, key, thermalBinding) => ({
  id,
  ...(key ? { key: true } : {}),
  title: tx(...title),
  description: tx(...description),
  bones,
  ground: "authored",
  contacts: ["wall-hook", "wall-rest"],
  thermal: thermalBinding,
  loads: [],
});

const poses = [
  pose("off", ["Off", "Aus"], ["Switched off, the discharge vane closed flush with the underside.", "Ausgeschaltet, die Ausblaslamelle bündig mit der Unterseite geschlossen."], {}, false, binding(0, 0, 0)),
  pose("cooling", ["Cooling", "Kühlen"], ["Rated cooling at high fan speed, the vane open 35 degrees: the cold jet leaves forward and down.", "Nennkühlung bei hoher Lüfterdrehzahl, die Lamelle 35 Grad geöffnet: Der kalte Strahl tritt nach vorn und unten aus."], { vane: { angleDeg: -35 } }, false, binding(1, 1, 1)),
  pose("cooling-low", ["Cooling, low fan", "Kühlen, niedrige Lüfterdrehzahl"], ["Part load at low fan speed, the vane open 15 degrees: a gentle, flat jet.", "Teillast bei niedriger Lüfterdrehzahl, die Lamelle 15 Grad geöffnet: ein sanfter, flacher Strahl."], { vane: { angleDeg: -15 } }, false, binding(0.4, 0.5, 0.5)),
  pose("swing-high", ["Swing, high", "Pendeln, hoch"], ["Key pose: vane open 20 degrees.", "Zwischenpose: Lamelle 20 Grad geöffnet."], { vane: { angleDeg: -20 } }, true, binding(1, 1, 1)),
  pose("swing-low", ["Swing, low", "Pendeln, tief"], ["Key pose: vane open 50 degrees.", "Zwischenpose: Lamelle 50 Grad geöffnet."], { vane: { angleDeg: -50 } }, true, binding(1, 1, 1)),
];

const contacts = (d) => [
  { anchor: "wall-hook", fromS: 0, toS: d },
  { anchor: "wall-rest", fromS: 0, toS: d },
];
const track = (target, ease, keys) => ({ target, ease, keys: keys.map(([t, v]) => ({ t, v })) });

const clips = [
  {
    id: "start-cooling",
    title: tx("Switch on and start cooling", "Einschalten und Kühlung starten"),
    description: tx(
      "The off unit is switched on after four seconds: the vane opens to 35 degrees, the fan starts and the evaporator ramps up to rated cooling within six seconds; the cold jet runs on for two minutes.",
      "Das ausgeschaltete Gerät wird nach vier Sekunden eingeschaltet: Die Lamelle öffnet auf 35 Grad, der Lüfter läuft an und der Verdampfer fährt binnen sechs Sekunden auf Nennkühlung hoch; der kalte Strahl läuft zwei Minuten weiter.",
    ),
    durationS: 120,
    fps: 1,
    loop: false,
    rootMotion: "none",
    keys: [
      { t: 0, pose: "off", ease: "smooth" },
      { t: 4, pose: "off", ease: "smooth" },
      { t: 10, pose: "cooling", ease: "linear" },
      { t: 120, pose: "cooling", ease: "linear" },
    ],
    contacts: contacts(120),
    phases: [
      { id: "off", fromS: 0, toS: 4, title: tx("Off", "Aus") },
      { id: "start", fromS: 4, toS: 10, title: tx("Vane opens, fan and coil ramp up", "Lamelle öffnet, Lüfter und Tauscher laufen hoch") },
      { id: "cooling", fromS: 10, toS: 120, title: tx("Rated cooling", "Nennkühlung") },
    ],
    events: [
      { t: 4, kind: "operating", title: tx("Switched on", "Eingeschaltet"), target: "cooling" },
      { t: 6, kind: "operating", title: tx("Fan starts", "Lüfter läuft an"), target: "fan" },
    ],
    tracks: [
      track("operating:cooling", "smooth", [[0, 0], [4, 0], [10, 1]]),
      track("operating:fan", "smooth", [[0, 0], [6, 0], [10, 1]]),
      track("emitter:cold-jet", "smooth", [[0, 0], [6, 0], [10, 1]]),
    ],
  },
  {
    id: "vane-swing",
    title: tx("Vane swing", "Lamellenpendeln"),
    description: tx(
      "At rated cooling the vane sweeps between 20 and 50 degrees every six seconds and fans the cold jet over the room.",
      "Bei Nennkühlung pendelt die Lamelle alle sechs Sekunden zwischen 20 und 50 Grad und fächert den kalten Strahl durch den Raum.",
    ),
    durationS: 6,
    fps: 12,
    loop: true,
    rootMotion: "none",
    keys: [
      { t: 0, pose: "swing-high", ease: "smooth" },
      { t: 3, pose: "swing-low", ease: "smooth" },
      { t: 6, pose: "swing-high", ease: "smooth" },
    ],
    contacts: contacts(6),
    phases: [
      { id: "down", fromS: 0, toS: 3, title: tx("Vane sweeps down", "Lamelle schwenkt nach unten") },
      { id: "up", fromS: 3, toS: 6, title: tx("Vane sweeps up", "Lamelle schwenkt nach oben") },
    ],
    events: [{ t: 3, kind: "motion", title: tx("Lowest vane angle", "Tiefster Lamellenwinkel") }],
    tracks: [],
  },
];

const pens = {
  50: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  100: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  200: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
};
const ranges = { 50: [20, 75], 100: [75, 150], 200: [150, 350] };

const CASING = "M 0,0 L 0.065,0 L 0.065,0.03 L 0.185,0.03 C 0.2,0.045 0.21,0.09 0.21,0.15 L 0.21,0.245 C 0.21,0.275 0.195,0.29 0.165,0.29 L 0,0.29 Z";
const VANE = "M 0,-0.015 L 0.11,-0.015 C 0.12,-0.015 0.125,-0.005 0.125,0.015 L 0.12,0.015 L 0,0.015 Z";

const VANE_100 = "M 0,-0.017 L 0.11,-0.017 C 0.12,-0.017 0.127,-0.005 0.127,0.017 L 0.12,0.017 L 0,0.017 Z";

const elevation = (sc) => {
  const u = "unit";
  const s = [];
  s.push(path("casing", u, "body", 1, true, 0.88, CASING, { region: "casing", material: "abs-white" }));
  if (sc === 200) {
    s.push(capsule("vane", "vane", "body", 2, true, 0.55, 0, 0, 0.12, 0, 0.02, 0.02, { region: "discharge-vane", material: "abs-grey" }));
    s.push(rect("intake-grille", u, "body", 2, true, 0.3, 0.03, 0.272, 0.11, 0.018, { region: "intake-grille", material: "abs-grey" }));
    return s;
  }
  s.push(path("vane", "vane", "body", 2, true, 0.6, sc === 100 ? VANE_100 : VANE, { region: "discharge-vane", material: "abs-grey" }));
  s.push(rect("intake-grille", u, "detail", 2, true, 0.3, sc === 100 ? 0.025 : 0.03, sc === 100 ? 0.264 : 0.268, sc === 100 ? 0.12 : 0.11, sc === 100 ? 0.026 : 0.022, { region: "intake-grille", material: "abs-grey" }));
  s.push(path("panel-seam", u, "detail", 1.5, false, 0.5, "M 0.02,0.255 L 0.165,0.255 C 0.185,0.255 0.195,0.245 0.197,0.225"));
  if (sc === 50) {
    for (let i = 0; i < 4; i++) s.push(capsule(`grille-slat-${i + 1}`, u, "detail", 2.2, true, 0.7, 0.047 + 0.025 * i, 0.272, 0.047 + 0.025 * i, 0.286, 0.006, 0.006, { material: "abs-grey" }));
    s.push(rect("display-window", u, "detail", 1.6, true, 0.15, 0.198, 0.13, 0.012, 0.05, { r: 0.004, material: "display" }));
    s.push(path("front-split", u, "detail", 1.5, false, 0.5, "M 0.005,0.225 L 0.17,0.225"));
  }
  return s;
};

const plan = (sc) => {
  const u = "unit";
  const s = [];
  s.push(rect("plan-casing", u, "body", 1, true, 0.88, 0, -0.425, 0.21, 0.85, { r: 0.02, region: "casing", material: "abs-white" }));
  s.push(rect("plan-grille", u, sc === 200 ? "body" : "detail", 2, true, 0.3, 0.03, -0.37, 0.11, 0.74, { region: "intake-grille", material: "abs-grey" }));
  if (sc === 200) return s;
  s.push(path("plan-panel-seam", u, "detail", 1.5, false, 0.5, "M 0.18,-0.4 L 0.18,0.4"));
  if (sc === 50) {
    for (let i = 0; i < 9; i++) {
      const y = -0.32 + 0.08 * i;
      s.push(capsule(`plan-slat-${i + 1}`, u, "detail", 2.2, true, 0.7, 0.045, y, 0.125, y, 0.007, 0.007, { material: "abs-grey" }));
    }
    s.push(rect("plan-display", u, "detail", 1.6, true, 0.15, 0.185, 0.3, 0.02, 0.08, { r: 0.004, material: "display" }));
  }
  return s;
};

const dec = {
  50: {
    silhouette: ["A rounded high-wall casing 5.8 mm by 4.2 mm on paper: flat back on the wall, flat top, curved front, the lower front curve folding into the discharge slot with its vane.", "Ein abgerundetes Hochwandgehäuse von 5,8 mm mal 4,2 mm auf Papier: ebene Rückseite an der Wand, ebene Oberseite, gewölbte Front, die untere Frontrundung geht in den Ausblasschlitz mit Lamelle über."],
    retained: ["Intake grille with four slats, front panel seam and split line, the display window and the vane as a separate, rotating slab; plan: nine slats, seam and display.", "Einlassgitter mit vier Lamellen, Blendenfuge und Teilungslinie, Anzeigefenster und die Lamelle als eigene, drehende Platte; Grundriss: neun Lamellen, Fuge und Anzeige."],
    merged: ["End caps and front panel are one outline; the evaporator, fan and wall plate behind them are not drawn.", "Endkappen und Frontblende sind eine Kontur; Verdampfer, Lüfter und Wandplatte dahinter werden nicht gezeichnet."],
    omitted: ["Filter, louvres, refrigerant pipes and drain hose (below 0.01 m = 0.2 mm; the pipes would exceed the 0.29 m height).", "Filter, Querlamellen, Kältemittelleitungen und Kondensatschlauch (unter 0,01 m = 0,2 mm; die Leitungen würden die Höhe von 0,29 m überschreiten)."],
    thinElements: ["The vane is 0.03 m (0.6 mm) at real thickness; grille slats 0.012 m (0.24 mm) at real width.", "Die Lamelle ist 0,03 m (0,6 mm) in realer Dicke; Gitterlamellen 0,012 m (0,24 mm) in realer Breite."],
    hatching: ["None; tone only: casing light, grille and vane mid-grey, display dark.", "Keine; nur Ton: Gehäuse hell, Gitter und Lamelle mittelgrau, Anzeige dunkel."],
    styles: ["Outlines 0.18 mm, detail 0.13 mm.", "Konturen 0,18 mm, Details 0,13 mm."],
    overlays: ["Regions map one to one onto shapes; the cold jet is drawn by the transport view at the slot.", "Regionen bilden sich eins zu eins auf Formen ab; der kalte Strahl wird von der Strömungsansicht am Schlitz gezeichnet."],
  },
  100: {
    silhouette: ["The same casing at 2.9 mm by 2.1 mm: a rounded D with the vane as a separate slab under the slot.", "Dasselbe Gehäuse mit 2,9 mm mal 2,1 mm: ein abgerundetes D mit der Lamelle als eigene Platte unter dem Schlitz."],
    retained: ["Intake grille as one dark bar, the front seam and the vane; plan: casing, grille and seam.", "Einlassgitter als ein dunkler Balken, die Blendenfuge und die Lamelle; Grundriss: Gehäuse, Gitter und Fuge."],
    merged: ["Grille slats merge into one bar; the display window merges into the front.", "Gitterlamellen verschmelzen zu einem Balken; das Anzeigefenster geht in der Front auf."],
    omitted: ["Display window, split line and slats (below 0.02 m = 0.2 mm).", "Anzeigefenster, Teilungslinie und Lamellen (unter 0,02 m = 0,2 mm)."],
    thinElements: ["Vane 0.03 m (0.3 mm) at real thickness.", "Lamelle 0,03 m (0,3 mm) in realer Dicke."],
    hatching: ["None.", "Keine."],
    styles: ["Outlines 0.18 mm, detail 0.13 mm.", "Konturen 0,18 mm, Details 0,13 mm."],
    overlays: ["Regions map onto shapes.", "Regionen bilden sich auf Formen ab."],
  },
  200: {
    silhouette: ["The casing as a 1.45 mm by 1.05 mm rounded D, the grille bar on top and the vane as a thickened line under the slot.", "Das Gehäuse als abgerundetes D von 1,45 mm mal 1,05 mm, der Gitterbalken oben und die Lamelle als verstärkte Linie unter dem Schlitz."],
    retained: ["Casing, grille bar and vane.", "Gehäuse, Gitterbalken und Lamelle."],
    merged: ["Front seam and panel line merge into the casing outline.", "Blendenfuge und Plattenlinie gehen in der Gehäusekontur auf."],
    omitted: ["Seam, display and slats (below 0.04 m = 0.2 mm).", "Fuge, Anzeige und Lamellen (unter 0,04 m = 0,2 mm)."],
    thinElements: ["The vane is drawn 0.04 m (0.2 mm) thick instead of 0.03 m; the grille bar 0.018 m (0.09 mm) is thickened by its own outline.", "Die Lamelle wird 0,04 m (0,2 mm) dick gezeichnet statt 0,03 m; der Gitterbalken von 0,018 m (0,09 mm) wird durch seine eigene Kontur verstärkt."],
    hatching: ["None.", "Keine."],
    styles: ["Outline 0.15 mm.", "Kontur 0,15 mm."],
    overlays: ["Regions map onto shapes.", "Regionen bilden sich auf Formen ab."],
  },
};

const authoredRep = (sc) => ({
  status: "authored",
  denominator: sc,
  validRange: { minDenominator: ranges[sc][0], maxDenominator: ranges[sc][1] },
  pens: pens[sc],
  hatches: [],
  geometry: { elevation: elevation(sc), plan: plan(sc) },
  decisions: decisions(dec[sc]),
});

export function splitAc() {
  return wrap({
    id: "split-ac-indoor-unit",
    category: "appliances",
    title: tx("Split Air Conditioner Indoor Unit", "Split-Klimagerät Innenteil"),
    summary: tx(
      "A wall-mounted split air-conditioner indoor unit (0.85 m wide, 0.29 m high, 0.21 m deep, 9 kg) with an intake grille, a rotating discharge vane and a cold air jet: rated cooling of 2.5 kW at 0.7 kW electric input, a jet of 450 m3/h cooled by 13 K.",
      "Ein wandhängendes Innenteil eines Split-Klimageräts (0,85 m breit, 0,29 m hoch, 0,21 m tief, 9 kg) mit Einlassgitter, drehender Ausblaslamelle und kaltem Luftstrahl: Nennkühlleistung 2,5 kW bei 0,7 kW elektrischer Aufnahme, ein Strahl von 450 m3/h, um 13 K abgekühlt.",
    ),
    tags: ["split-ac", "air-conditioner", "indoor-unit", "cooling", "wall-mounted", "appliance", "air-jet"],
    provenance: {
      author: "Semio staffage content worker (appliances)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [CLASS, EN14511, ISO5151, ASHRAE, INCROPERA],
      notes: tx(
        "Class data of 2.5 kW high-wall split indoor units; the profile is an original drawing of that type. The origin is the bottom of the casing on the wall plane, the hook sits 0.27 m above it: place the origin 0.29 m below the mounting height. Rated cooling follows the EN 14511 and ISO 5151 test point (35 C outdoor, 27 C / 19 C indoor); every number is an authored estimate of that class.",
        "Klassendaten von 2,5-kW-Hochwand-Innengeräten; das Profil ist eine eigene Zeichnung dieses Typs. Der Ursprung liegt an der Unterkante des Gehäuses in der Wandebene, der Haken sitzt 0,27 m darüber: den Ursprung 0,29 m unter die Montagehöhe setzen. Die Nennkühlung folgt dem Prüfpunkt von EN 14511 und ISO 5151 (außen 35 C, innen 27 C / 19 C); jede Zahl ist eine Schätzung dieser Klasse.",
      ),
    },
    dimensions: { heightM: 0.29, widthM: 0.21, depthM: 0.85, massKg: 9 },
    materials,
    rig,
    anchors,
    representations: {
      50: authoredRep(50),
      100: authoredRep(100),
      200: authoredRep(200),
      500: unavailable(
        500,
        ["At 1:500 the unit is 0.58 mm tall and 0.42 mm deep, its 0.03 m vane 0.06 mm thick, below every print profile.", "Bei 1:500 ist das Gerät 0,58 mm hoch und 0,42 mm tief, seine Lamelle von 0,03 m nur 0,06 mm dick, unter jedem Druckprofil."],
        ["Discharge vane thickness of 0.03 m", "Dicke der Ausblaslamelle von 0,03 m"],
        0.03,
        [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
      ),
    },
    symbols: [],
    poses,
    clips,
    thermal,
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: ["wall-hook", "wall-rest"],
      assumptions: [
        tx("The 9 kg unit hangs on the upper hook (pin) and leans against the wall at the lower edge (roller): the hook carries the weight, the pair resists the overturning moment of the centre of mass 0.1 m from the wall.", "Das Gerät von 9 kg hängt am oberen Haken (Festlager) und lehnt unten an der Wand (Rollenlager): der Haken trägt das Gewicht, das Paar nimmt das Kippmoment des Schwerpunkts 0,1 m vor der Wand auf."),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "off", defaultClip: "start-cooling", defaultScale: 100, viewpoints: ["elevation", "plan"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
}
