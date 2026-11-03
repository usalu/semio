import { ALL_EXPORTS, decisions, level, ref, tx, wrap } from "./lib.mjs";
import { elevation, plan } from "./car-shapes.mjs";

const R = 0.37;
const deg = (m) => Math.round(((-m / R) * 180) / Math.PI * 1e3) / 1e3;

const ASHRAE = ref("ashrae-2021", "ASHRAE Handbook Fundamentals (2021), Chapter 4 Heat Transfer, convection and radiation properties of painted and plated surfaces.");
const INCROPERA = ref("incropera-2011", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley. Ch. 7 and 9 (external and natural convection), Appendix A (thermophysical properties of steel, glass, rubber, nickel).");
const HEYWOOD = ref("heywood-2018", "Heywood, J. B. (2018). Internal Combustion Engine Fundamentals, 2nd ed., McGraw-Hill. Ch. 4 and 13: exhaust gas temperature and mass flow of spark-ignition engines.");
const WONG = ref("wong-2008", "Wong, J. Y. (2008). Theory of Ground Vehicles, 4th ed., Wiley. Ch. 1 and 3: tyre contact patch, rolling radius, rolling resistance heat.");
const LIMPERT = ref("limpert-1999", "Limpert, R. (1999). Brake Design and Safety, 2nd ed., SAE International. Ch. 5: brake drum and disc temperature rise.");
const VOISIN = ref("voisin-c7-c11-class-data", "Class data of the Voisin C7 and C11 saloons (1927 to 1930): about 4.3 to 4.6 m long, 1.7 m wide, 1.7 m high, wheelbase about 3.0 m, track about 1.4 m, 5.00-19 balloon tyres, four-cylinder engines of 1.2 to 2.3 litres, a mass of 1.2 to 1.4 t. Quoted as orders of magnitude from the general literature of the model family, not retrieved for this asset; every number derived from it is tagged authored-estimate.");
const SAVOYE = ref("villa-savoye-photographs", "Contemporary photographs of the Villa Savoye, Poissy (Le Corbusier and Pierre Jeanneret, 1928 to 1931), which show period saloons of this kind on the entrance drive. Context only; no dimension is taken from them.");

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
  mat("paint", "Painted sheet steel", "Lackiertes Stahlblech", 0.9, 0.55, 7850, 50, "literature", "ashrae-2021"),
  mat("fabric-roof", "Fabric-covered wooden roof", "Stoffbespanntes Holzdach", 0.9, 0.6, 600, 0.15, "authored-estimate"),
  mat("glass", "Toughened and laminated glass", "Einscheiben- und Verbundsicherheitsglas", 0.84, 0.15, 2500, 1, "literature", "ashrae-2021"),
  mat("rubber", "Balloon tyre rubber", "Ballonreifengummi", 0.95, 0.92, 1150, 0.2, "literature", "wong-2008"),
  mat("steel-rim", "Painted steel rim and spokes", "Lackierte Stahlfelge und Speichen", 0.85, 0.6, 7850, 50, "authored-estimate", "ashrae-2021"),
  mat("nickel-plate", "Nickel-plated brass", "Vernickeltes Messing", 0.1, 0.4, 8500, 110, "literature", "incropera-2011"),
  mat("cast-iron", "Grey cast iron brake drum", "Graugussbremstrommel", 0.65, 0.7, 7200, 46, "literature", "limpert-1999"),
  mat("exhaust-steel", "Oxidised exhaust steel", "Oxidierter Abgasstahl", 0.7, 0.8, 7800, 25, "literature", "heywood-2018"),
  mat("engine-metal", "Engine casting and manifold", "Motorguss und Krümmer", 0.8, 0.8, 7200, 46, "authored-estimate", "heywood-2018"),
];

const bone = (id, parent, title, x, y, lengthM, joint, mass) => ({ id, parent, title: tx(...title), x, y, angleDeg: 0, lengthM, joint, mass: { ...mass, distribution: "point" } });

const rig = {
  kind: "mechanical",
  root: "chassis",
  bones: [
    bone("chassis", null, ["Body, frame, fenders and spare wheels", "Aufbau, Rahmen, Kotflügel und Reserveräder"], 0, 0, 4.39, { type: "fixed" }, { kg: 1196, comX: -0.1, comY: 0.72, source: "authored-estimate", reference: "voisin-c7-c11-class-data" }),
    { ...bone("axle-f", "chassis", ["Front axle", "Vorderachse"], 1.5, R, 0.1, { type: "fixed" }, { kg: 0, comX: 0, comY: 0, source: "authored-estimate" }) },
    { ...bone("axle-r", "chassis", ["Rear axle", "Hinterachse"], -1.5, R, 0.1, { type: "fixed" }, { kg: 0, comX: 0, comY: 0, source: "authored-estimate" }) },
    bone("wheel-f", "axle-f", ["Front wheels", "Vorderräder"], 0, 0, 0.37, { type: "revolute" }, { kg: 52, comX: 0, comY: 0, source: "authored-estimate", reference: "wong-2008" }),
    bone("wheel-r", "axle-r", ["Rear wheels", "Hinterräder"], 0, 0, 0.37, { type: "revolute" }, { kg: 52, comX: 0, comY: 0, source: "authored-estimate", reference: "wong-2008" }),
  ],
};

const anchors = [
  { id: "contact-f", bone: "axle-f", x: 0, y: -R, kind: "contact", title: tx("Front tyre patches", "Vorderreifenaufstandsflächen"), support: "contact", normalDeg: 90, footprintLengthM: 0.14, footprintWidthM: 0.26, frictionCoefficient: 0.7 },
  { id: "contact-r", bone: "axle-r", x: 0, y: -R, kind: "contact", title: tx("Rear tyre patches", "Hinterreifenaufstandsflächen"), support: "contact", normalDeg: 90, footprintLengthM: 0.14, footprintWidthM: 0.26, frictionCoefficient: 0.7 },
  { id: "tailpipe", bone: "chassis", x: -2.08, y: 0.25, kind: "emitter", title: tx("Tailpipe mouth", "Auspuffmündung") },
];

const sheet = (reference) => ({
  id: "sheet",
  title: tx("Steel sheet 1.0 mm", "Stahlblech 1,0 mm"),
  kind: "metal",
  material: "paint",
  thicknessM: 0.001,
  resistanceM2KW: 2e-5,
  heatCapacityJM2K: 3900,
  source: "literature",
  reference,
});
const paintFilm = {
  id: "paint",
  title: tx("Paint film", "Lackschicht"),
  kind: "polymer",
  material: "paint",
  thicknessM: 8e-5,
  resistanceM2KW: 0.00027,
  heatCapacityJM2K: 150,
  source: "authored-estimate",
};

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
  region("body-panels", ["Painted side and end panels", "Lackierte Seiten- und Endbleche"], "surface", "passive", "paint", 6.4, 1.2, "vertical-plate", 0.5, [sheet("incropera-2011"), paintFilm], 6000, "authored-estimate", "ashrae-2021"),
  region("roof", ["Fabric-covered roof", "Stoffbespanntes Dach"], "surface", "passive", "fabric-roof", 2.2, 1.4, "horizontal-plate-up", 0.95, [
    { id: "fabric", title: tx("Fabric on wooden frame 6 mm", "Stoff auf Holzgerippe 6 mm"), kind: "timber", material: "fabric-roof", thicknessM: 0.006, resistanceM2KW: 0.04, heatCapacityJM2K: 2200, source: "authored-estimate" },
  ], 2200, "authored-estimate", "ashrae-2021"),
  region("hood", ["Bonnet, warmed from below", "Motorhaube, von unten erwärmt"], "surface", "operating", "paint", 1.5, 1, "horizontal-plate-up", 0.9, [sheet("incropera-2011"), paintFilm], 6500, "authored-estimate", "heywood-2018", { operating: { key: "engine-load", heatW: 160 } }),
  region("fenders", ["Fenders and running boards", "Kotflügel und Trittbretter"], "surface", "passive", "paint", 2.6, 0.9, "horizontal-plate-up", 0.7, [sheet("incropera-2011"), paintFilm], 6000, "authored-estimate", "ashrae-2021"),
  region("glazing", ["Windscreen and side glass", "Windschutz- und Seitenscheiben"], "glazing", "passive", "glass", 1.6, 0.8, "vertical-plate", 0.6, [
    { id: "glass", title: tx("Toughened glass 5 mm", "Einscheibensicherheitsglas 5 mm"), kind: "glass", material: "glass", thicknessM: 0.005, resistanceM2KW: 0.005, heatCapacityJM2K: 9400, source: "literature", reference: "incropera-2011" },
  ], 9400, "authored-estimate", "ashrae-2021"),
  region("radiator", ["Radiator shell", "Kühlerschale"], "surface", "operating", "nickel-plate", 0.45, 0.7, "vertical-plate", 0.4, [], 3500, "authored-estimate", "heywood-2018", { operating: { key: "engine-load", heatW: 380 } }),
  region("engine-bay", ["Engine casting seen through the louvres", "Motorguss hinter den Luftschlitzen"], "engine", "operating", "engine-metal", 0.8, 0.45, "cylinder", 0.1, [], 9000, "authored-estimate", "heywood-2018", { operating: { key: "engine-load", heatW: 900 } }),
  region("exhaust-system", ["Exhaust pipe and silencer", "Abgasrohr und Schalldämpfer"], "exhaust", "operating", "exhaust-steel", 0.4, 0.08, "cylinder", 0.2, [
    { id: "pipe", title: tx("Steel pipe 1.5 mm", "Stahlrohr 1,5 mm"), kind: "metal", material: "exhaust-steel", thicknessM: 0.0015, resistanceM2KW: 3.3e-5, heatCapacityJM2K: 8500, source: "authored-estimate" },
  ], 8500, "authored-estimate", "heywood-2018", { operating: { key: "engine-load", heatW: 1800 } }),
  region("tyres", ["Balloon tyres and spare tyres", "Ballonreifen und Reservereifen"], "tyre", "operating", "rubber", 2.4, 0.74, "cylinder", 0.4, [
    { id: "tread", title: tx("Rubber tread and carcass", "Gummilauffläche und Karkasse"), kind: "polymer", material: "rubber", thicknessM: 0.01, resistanceM2KW: 0.05, heatCapacityJM2K: 34000, source: "authored-estimate", reference: "wong-2008" },
  ], 34000, "authored-estimate", "wong-2008", { operating: { key: "rolling", heatW: 380 } }),
  region("brakes", ["Brake drums", "Bremstrommeln"], "surface", "operating", "cast-iron", 0.5, 0.34, "cylinder", 0.2, [], 31000, "authored-estimate", "limpert-1999", { operating: { key: "braking", heatW: 1100 } }),
];

const thermal = {
  environment: { airTempC: 20, meanRadiantTempC: 20, skyTempC: 8, groundTempC: 28, relativeHumidity: 0.5, windMS: 0.5, windDirectionDeg: 180, solarWM2: 400, diffuseFraction: 0.2, pressurePa: 101325 },
  passengers: [],
  operating: [
    { id: "engine-load", title: tx("Engine load", "Motorlast"), levels: [level(0, "Engine off", "Motor aus"), level(0.2, "Idle", "Leerlauf"), level(0.5, "Part load (cruising)", "Teillast (Reisefahrt)"), level(1, "Full load", "Volllast")] },
    { id: "rolling", title: tx("Rolling speed", "Rollgeschwindigkeit"), levels: [level(0, "Standstill", "Stillstand"), level(0.5, "About 20 km/h", "Etwa 20 km/h"), level(1, "About 40 km/h", "Etwa 40 km/h")] },
    { id: "braking", title: tx("Drum braking", "Trommelbremsen"), levels: [level(0, "Not braking", "Nicht gebremst"), level(0.3, "Light braking", "Leichte Bremsung"), level(1, "Full braking from 40 km/h", "Vollbremsung aus 40 km/h")] },
  ],
  regions,
  emitters: [
    { id: "tailpipe", kind: "exhaust", title: tx("Exhaust jet", "Abgasstrahl"), anchor: "tailpipe", directionDeg: 190, diameterM: 0.045, operatingKey: "engine-load", volumeFlowM3S: 0.06, temperatureRiseK: 260, vapourKgPerKg: 0.075, source: "authored-estimate", reference: "heywood-2018" },
  ],
  assumptions: [
    tx(
      "Heat release at level 1 is the part of the waste heat that leaves through the visible surface: engine louvres 0.9 kW, radiator shell 0.38 kW, exhaust system 1.8 kW, bonnet 0.16 kW, tyres 0.38 kW (about 40 km/h) and brake drums 1.1 kW (a stop from 40 km/h is about 60 kJ in 2.2 s on four drums, named poses use level 0.05). Heat capacities are the thermally active shell masses on a minutes timescale.",
      "Die Wärmefreisetzung bei Stufe 1 ist der Teil der Abwärme, der durch die sichtbare Oberfläche austritt: Motorluftschlitze 0,9 kW, Kühlerschale 0,38 kW, Abgasanlage 1,8 kW, Motorhaube 0,16 kW, Reifen 0,38 kW (etwa 40 km/h) und Bremstrommeln 1,1 kW (ein Halt aus 40 km/h sind rund 60 kJ in 2,2 s auf vier Trommeln; die benannten Posen nutzen Stufe 0,05). Die Wärmekapazitäten sind die thermisch wirksamen Hüllmassen auf der Minutenskala.",
    ),
    tx(
      "The small four-cylinder engine (about 40 kW) idles with a low exhaust volume flow (0.06 m3/s at full load, +260 K at the mouth); the 1929 saloon has no catalyst, so the exhaust jet is hot and humid (0.075 kg/kg).",
      "Der kleine Vierzylindermotor (etwa 40 kW) hat einen geringen Abgasvolumenstrom (0,06 m3/s bei Volllast, +260 K an der Mündung); der Wagen von 1929 hat keinen Katalysator, der Abgasstrahl ist daher heiß und feucht (0,075 kg/kg).",
    ),
  ],
};

const reg = (exposed, sun, extra = {}) => ({ exposedFraction: exposed, sunlitFraction: sun, ...extra });
const binding = (engine, rolling, braking, tail) => ({
  operating: { "engine-load": engine, rolling, braking },
  regions: {
    "body-panels": reg(1, 0.5),
    roof: reg(1, 1),
    hood: reg(1, 1),
    fenders: reg(1, 0.7),
    glazing: reg(1, 0.5),
    radiator: reg(1, 0.6),
    "engine-bay": reg(0.2, 0),
    "exhaust-system": reg(0.6, 0),
    tyres: reg(1, 0.15, { contact: { fraction: 0.05, tempC: 30 } }),
    brakes: reg(0.4, 0),
  },
  emitters: { tailpipe: tail },
});

const pose = (id, title, description, bones, root, key, thermalBinding) => ({
  id,
  ...(key ? { key: true } : {}),
  title: tx(...title),
  description: tx(...description),
  bones,
  ...(root ? { root } : {}),
  ground: "auto",
  contacts: ["contact-f", "contact-r"],
  thermal: thermalBinding,
  loads: [],
});

const rollAt = (x) => ({ "wheel-f": { angleDeg: deg(x) }, "wheel-r": { angleDeg: deg(x) } });

const poses = [
  pose("parked", ["Parked", "Geparkt"], ["Engine off and cold, standing on level ground as at the Villa Savoye.", "Motor aus und kalt, auf ebenem Grund abgestellt wie an der Villa Savoye."], {}, { x: 0, y: 0, angleDeg: 0 }, false, binding(0, 0, 0, 0)),
  pose("driving", ["Driving", "Fahrt"], ["Cruising at about 30 km/h: the engine at part load, the tyres warm, the exhaust jet on.", "Reisefahrt mit etwa 30 km/h: Motor in Teillast, Reifen warm, Abgasstrahl an."], {}, undefined, false, binding(0.5, 0.8, 0, 1)),
  pose("braking", ["Braking", "Bremsen"], ["Light drum braking at walking pace into the parking space, engine idling.", "Leichte Trommelbremsung im Schritttempo beim Einparken, Motor im Leerlauf."], {}, undefined, false, binding(0.2, 0.4, 0.05, 1)),
  pose("lane-start", ["Start of the lane", "Spuranfang"], ["Key pose: idling 12 m before the origin.", "Zwischenpose: Leerlauf 12 m vor dem Ursprung."], rollAt(-12), { x: -12, y: 0, angleDeg: 0 }, true, binding(0.2, 0, 0, 1)),
  pose("lane-cruise-entry", ["Cruise entry", "Beginn der Reisefahrt"], ["Key pose: end of the acceleration, 4 m before the origin at 8 m/s.", "Zwischenpose: Ende der Beschleunigung, 4 m vor dem Ursprung bei 8 m/s."], rollAt(-4), { x: -4, y: 0, angleDeg: 0 }, true, binding(0.6, 0.8, 0, 1)),
  pose("lane-exit", ["End of the lane", "Spurende"], ["Key pose: cruising 12 m behind the origin.", "Zwischenpose: Reisefahrt 12 m hinter dem Ursprung."], rollAt(12), { x: 12, y: 0, angleDeg: 0 }, true, binding(0.5, 0.8, 0, 1)),
  pose("crawl-in", ["Crawl-in", "Einfahrt im Schritttempo"], ["Key pose: 3 m/s, 6 m before the parking space, light braking.", "Zwischenpose: 3 m/s, 6 m vor dem Parkplatz, leichte Bremsung."], rollAt(-6), { x: -6, y: 0, angleDeg: 0 }, true, binding(0.2, 0.2, 0.3, 1)),
];

const contactsAll = (d) => [
  { anchor: "contact-f", fromS: 0, toS: d },
  { anchor: "contact-r", fromS: 0, toS: d },
];

const clips = [
  {
    id: "drive-by",
    title: tx("Drive-by from standstill", "Vorbeifahrt aus dem Stand"),
    description: tx(
      "The engine is started and idles, the saloon accelerates uniformly over 8 m to 8 m/s (29 km/h) and cruises past the origin; the 0.37 m wheels roll with theta = -x / r.",
      "Der Motor wird gestartet und läuft im Leerlauf, der Wagen beschleunigt gleichmäßig auf 8 m Strecke auf 8 m/s (29 km/h) und fährt am Ursprung vorbei; die Räder von 0,37 m Radius rollen mit theta = -x / r.",
    ),
    durationS: 4.8,
    fps: 30,
    loop: false,
    rootMotion: "none",
    keys: [
      { t: 0, pose: "lane-start", ease: "linear" },
      { t: 0.8, pose: "lane-start", ease: "in" },
      { t: 2.8, pose: "lane-cruise-entry", ease: "linear" },
      { t: 4.8, pose: "lane-exit", ease: "linear" },
    ],
    contacts: contactsAll(4.8),
    phases: [
      { id: "idle", fromS: 0, toS: 0.8, title: tx("Start and idle", "Start und Leerlauf") },
      { id: "accelerate", fromS: 0.8, toS: 2.8, title: tx("Uniform acceleration", "Gleichmäßige Beschleunigung") },
      { id: "cruise", fromS: 2.8, toS: 4.8, title: tx("Cruise", "Reisefahrt") },
    ],
    events: [
      { t: 0.3, kind: "ignition", title: tx("Ignition", "Zündung"), target: "tailpipe" },
      { t: 0.8, kind: "motion", title: tx("Pulls away", "Anfahren") },
      { t: 2.8, kind: "thermal", title: tx("Cruise speed 8 m/s", "Reisegeschwindigkeit 8 m/s") },
    ],
    tracks: [
      { target: "operating:engine-load", ease: "smooth", keys: [[0, 0], [0.3, 0.45], [0.6, 0.2], [0.8, 0.2], [2, 0.95], [2.8, 0.6], [4.8, 0.5]].map(([t, v]) => ({ t, v })) },
      { target: "operating:rolling", ease: "smooth", keys: [[0, 0], [0.8, 0], [2.8, 0.8], [4.8, 0.8]].map(([t, v]) => ({ t, v })) },
      { target: "emitter:tailpipe", ease: "step", keys: [[0, 0], [0.3, 1]].map(([t, v]) => ({ t, v })) },
    ],
  },
  {
    id: "arrive-and-park",
    title: tx("Arrive and park", "Ankommen und parken"),
    description: tx(
      "The saloon brakes uniformly from 3 m/s to rest in 4 s (6 m), the engine is switched off and the drums cool.",
      "Der Wagen bremst gleichmäßig in 4 s von 3 m/s bis zum Stillstand (6 m), der Motor wird abgestellt und die Trommeln kühlen aus.",
    ),
    durationS: 8,
    fps: 30,
    loop: false,
    rootMotion: "none",
    keys: [
      { t: 0, pose: "crawl-in", ease: "out" },
      { t: 4, pose: "parked", ease: "linear" },
      { t: 8, pose: "parked", ease: "linear" },
    ],
    contacts: contactsAll(8),
    phases: [
      { id: "brake", fromS: 0, toS: 4, title: tx("Braking to rest", "Bremsen bis zum Stillstand") },
      { id: "shutdown", fromS: 4, toS: 5, title: tx("Engine off", "Motor aus") },
      { id: "cool", fromS: 5, toS: 8, title: tx("Standing, cooling", "Stehend, kühlt aus") },
    ],
    events: [
      { t: 4, kind: "motion", title: tx("Standstill", "Stillstand") },
      { t: 4.4, kind: "shutdown", title: tx("Ignition off", "Zündung aus"), target: "tailpipe" },
    ],
    tracks: [
      { target: "operating:engine-load", ease: "smooth", keys: [[0, 0.2], [4.4, 0.2], [4.7, 0], [8, 0]].map(([t, v]) => ({ t, v })) },
      { target: "operating:braking", ease: "smooth", keys: [[0, 0.7], [4, 0.6], [8, 0.1]].map(([t, v]) => ({ t, v })) },
      { target: "emitter:tailpipe", ease: "step", keys: [[0, 1], [4.4, 0]].map(([t, v]) => ({ t, v })) },
    ],
  },
];

const pens = {
  50: { body: 0.25, detail: 0.18, texture: null, glazing: 0.13, ground: 0.5 },
  100: { body: 0.18, detail: 0.13, texture: null, glazing: 0.1, ground: 0.35 },
  200: { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.25 },
  500: { body: 0.15, detail: null, texture: null, glazing: 0.13, ground: 0.25 },
};
const ranges = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };

const dec = {
  50: {
    silhouette: ["Late-1920s saloon in side view: separate crescent fenders with 0.05 m gap over each tyre, running board, long bonnet with louvres, upright cabin with visor and fabric roof slab, nickel radiator with cap.", "Salonwagen der späten 1920er in der Seitenansicht: getrennte Sichelkotflügel mit 0,05 m Abstand über jedem Reifen, Trittbrett, lange Haube mit Luftschlitzen, aufrechte Kabine mit Sonnenblende und Stoffdachplatte, vernickelter Kühler mit Verschluss."],
    retained: ["Door gaps, handles, three side windows, seats and steering column through the glass, headlamp bowl with stalk, tail lamp, plate, bumper brackets, exhaust pipe, silencer and tail pipe, spare wheel with cover ring, 12 wire spokes, hub cap, brake drum and tyre valve per wheel.", "Türfugen, Griffe, drei Seitenfenster, Sitze und Lenksäule hinter dem Glas, Scheinwerfertopf mit Stütze, Rücklicht, Nummernschild, Stoßstangenhalter, Abgasrohr, Schalldämpfer und Endrohr, Reserverad mit Abdeckring, 12 Drahtspeichen, Radkappe, Bremstrommel und Reifenventil je Rad."],
    merged: ["Sill, belt and window frames merge into the tub and cabin outlines; wheel arches merge into the fender crescents.", "Schweller, Gürtellinie und Fensterrahmen gehen in den Kontur von Wanne und Kabine auf; Radläufe in den Kotflügelsicheln."],
    omitted: ["Door hinges, hood latches, mascot, tyre tread, wire-wheel nipples and the rear-axle housing (below 0.02 m = 0.4 mm).", "Türscharniere, Haubenverschlüsse, Kühlerfigur, Reifenprofil, Speichennippel und Hinterachsgehäuse (unter 0,02 m = 0,4 mm)."],
    thinElements: ["Spokes are 0.022 m wide (0.44 mm) at real width; the windscreen edge is a 0.3 mm line; crescent fenders 0.05 m (real 0.002 m sheet).", "Speichen sind 0,022 m breit (0,44 mm) in Realbreite; die Windschutzscheibenkante ist eine 0,3-mm-Linie; Sichelkotflügel 0,05 m (real 0,002 m Blech)."],
    hatching: ["None; materials are shown by tone only: tyres near black, glass lightest, nickel parts light, fabric roof lighter than the painted body.", "Keine; Materialien zeigen sich nur über den Ton: Reifen fast schwarz, Glas am hellsten, vernickelte Teile hell, Stoffdach heller als der lackierte Aufbau."],
    styles: ["Body mid-tone, fenders darker, outlines 0.25 mm, detail 0.18 mm, glazing 0.13 mm; plan view uses the same pens.", "Aufbau mittlerer Ton, Kotflügel dunkler, Konturen 0,25 mm, Details 0,18 mm, Verglasung 0,13 mm; der Grundriss nutzt dieselben Stifte."],
    overlays: ["Regions map one to one onto shapes; the valve and the asymmetric spoke pattern make the rolling wheel legible in clips.", "Regionen bilden sich eins zu eins auf Formen ab; Ventil und unsymmetrisches Speichenbild machen das rollende Rad in Clips lesbar."],
  },
  100: {
    silhouette: ["The same saloon with the fenders as separate crescents at 0.06 m (0.6 mm) clearance, running board, bonnet, cabin with roof slab and three windows.", "Derselbe Salonwagen mit den Kotflügeln als getrennte Sicheln bei 0,06 m (0,6 mm) Abstand, Trittbrett, Haube, Kabine mit Dachplatte und drei Fenstern."],
    retained: ["Three door gaps, belt line, six bonnet louvres, headlamp bowl and stalk, bumpers and brackets, exhaust pipe, silencer and tail pipe, spare with ring, eight spokes, hub and valve per wheel.", "Drei Türfugen, Gürtellinie, sechs Haubenschlitze, Scheinwerfertopf und Stütze, Stoßstangen und Halter, Abgasrohr, Schalldämpfer und Endrohr, Reserverad mit Ring, acht Speichen, Nabe und Ventil je Rad."],
    merged: ["Handles, seats, plate and fender lamp are dropped into the body tone; the tyre bead ring merges into the rim.", "Griffe, Sitze, Nummernschild und Kotflügellampe gehen im Aufbauton auf; der Reifenwulstring in der Felge."],
    omitted: ["Interior, steering column, tyre bead line, wheel nuts and brackets below 0.03 m (0.3 mm).", "Innenraum, Lenksäule, Reifenwulstlinie, Radmuttern und Halter unter 0,03 m (0,3 mm)."],
    thinElements: ["Spokes 0.032 m (0.32 mm) at real width; louvres 0.04 m; windscreen edge 0.3 mm line.", "Speichen 0,032 m (0,32 mm) in Realbreite; Luftschlitze 0,04 m; Windschutzscheibenkante als 0,3-mm-Linie."],
    hatching: ["None; tone only.", "Keine; nur Ton."],
    styles: ["Outlines 0.18 mm, detail 0.13 mm, glazing 0.10 mm.", "Konturen 0,18 mm, Details 0,13 mm, Verglasung 0,10 mm."],
    overlays: ["Regions map onto shapes; the eight spokes and the valve keep the rolling readable.", "Regionen bilden sich auf Formen ab; acht Speichen und das Ventil halten das Rollen lesbar."],
  },
  200: {
    silhouette: ["Main masses separated: tub and cabin, bonnet, two fenders, roof slab, three windows, bumpers, wheels as tyre disc with rim.", "Hauptmassen getrennt: Wanne und Kabine, Haube, zwei Kotflügel, Dachplatte, drei Fenster, Stoßstangen, Räder als Reifenscheibe mit Felge."],
    retained: ["Radiator shell and cap, headlamp bowl, spare wheel, silencer and tail pipe, a two-bar spoke cross and a hub per wheel.", "Kühlerschale und Verschluss, Scheinwerfertopf, Reserverad, Schalldämpfer und Endrohr, ein Speichenkreuz aus zwei Balken und eine Nabe je Rad."],
    merged: ["The louvres merge into one dark band, the exhaust pipe into the silencer line, the brake drum into the rim disc.", "Die Luftschlitze verschmelzen zu einem dunklen Band, das Abgasrohr zur Schalldämpferlinie, die Bremstrommel zur Felgenscheibe."],
    omitted: ["Door gaps, belt line, handles, seats, brackets, spokes beyond the cross and the valve (below 0.06 m = 0.3 mm).", "Türfugen, Gürtellinie, Griffe, Sitze, Halter, Speichen jenseits des Kreuzes und das Ventil (unter 0,06 m = 0,3 mm)."],
    thinElements: ["The spoke cross is 0.06 m wide (0.3 mm) instead of 0.022 m; the exhaust pipe is merged because 0.06 m would be 0.3 mm.", "Das Speichenkreuz ist 0,06 m breit (0,3 mm) statt 0,022 m; das Abgasrohr ist verschmolzen, da 0,06 m 0,3 mm ergäben."],
    hatching: ["None.", "Keine."],
    styles: ["Outlines 0.18 mm, detail 0.13 mm, glazing 0.13 mm.", "Konturen 0,18 mm, Details 0,13 mm, Verglasung 0,13 mm."],
    overlays: ["Regions map onto shapes; the cross shows the rotation.", "Regionen bilden sich auf Formen ab; das Kreuz zeigt die Drehung."],
  },
  500: {
    silhouette: ["One composed silhouette of the saloon (body, fenders, bonnet, radiator, roof), one window band, an underbody bar and two wheel discs.", "Eine zusammengesetzte Silhouette des Salonwagens (Aufbau, Kotflügel, Haube, Kühler, Dach), ein Fensterband, ein Unterbalken und zwei Radscheiben."],
    retained: ["The long bonnet, upright cabin and the two wheels with a single bar for rotation.", "Die lange Haube, die aufrechte Kabine und die zwei Räder mit einem einzigen Balken für die Drehung."],
    merged: ["Fenders, bonnet, radiator, tub, cabin and roof into one outline; exhaust and underbody into one bar; the brake drum into the tyre disc.", "Kotflügel, Haube, Kühler, Wanne, Kabine und Dach zu einer Kontur; Abgasanlage und Unterboden zu einem Balken; die Bremstrommel in die Reifenscheibe."],
    omitted: ["Spare wheel, lamps, bumpers, louvres, door gaps, windows beyond the band (below 0.1 m = 0.2 mm).", "Reserverad, Lampen, Stoßstangen, Luftschlitze, Türfugen, Fenster jenseits des Bandes (unter 0,1 m = 0,2 mm)."],
    thinElements: ["The wheel bar is 0.18 m wide (0.36 mm); the window band 0.33 m tall (0.66 mm).", "Der Radbalken ist 0,18 m breit (0,36 mm); das Fensterband 0,33 m hoch (0,66 mm)."],
    hatching: ["None.", "Keine."],
    styles: ["Outlines 0.15 mm, glazing 0.13 mm.", "Konturen 0,15 mm, Verglasung 0,13 mm."],
    overlays: ["Merged shapes carry the regions they merge, painted with area-weighted values.", "Verschmolzene Formen tragen die Regionen, die sie vereinen, mit flächengewichteten Werten."],
  },
};

const planDec = {
  50: ["The plan adds the top view: hood ridge, roof seam and the B-pillar gaps.", "Der Grundriss ergänzt die Draufsicht: Haubenfirst, Dachnaht und die B-Säulenfugen."],
};

const representations = Object.fromEntries(
  [50, 100, 200, 500].map((sc) => [
    `${sc}`,
    {
      status: "authored",
      denominator: sc,
      validRange: { minDenominator: ranges[sc][0], maxDenominator: ranges[sc][1] },
      pens: pens[sc],
      hatches: [],
      geometry: { elevation: elevation(sc), plan: plan(sc) },
      decisions: decisions(dec[sc]),
    },
  ]),
);

export function vintageCar() {
  return wrap({
    id: "vintage-car-1929",
    category: "vehicles",
    title: tx("Vintage Saloon 1929", "Salonwagen von 1929"),
    summary: tx(
      "A late-1920s European saloon of the Voisin C7/C11 family (4.4 m, 1.7 m wide, 1.7 m high, about 1.3 t) as parked at the Villa Savoye: separate crescent fenders, running boards, side-mounted spare wheels, a long louvred bonnet, a fabric-covered cabin and rolling spoked wheels, in elevation and in plan, with exhaust, tyre, drum and engine thermal regions.",
      "Ein europäischer Salonwagen der späten 1920er aus der Familie Voisin C7/C11 (4,4 m, 1,7 m breit, 1,7 m hoch, rund 1,3 t), wie er an der Villa Savoye stand: getrennte Sichelkotflügel, Trittbretter, seitlich montierte Reserveräder, eine lange Haube mit Luftschlitzen, eine stoffbespannte Kabine und rollende Speichenräder, in Ansicht und Grundriss, mit thermischen Regionen für Abgas, Reifen, Trommeln und Motor.",
    ),
    tags: ["vehicle", "car", "saloon", "vintage", "1929", "petrol", "villa-savoye", "wheeled"],
    provenance: {
      author: "Semio staffage content worker (vehicles, vintage)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [VOISIN, SAVOYE, ASHRAE, INCROPERA, HEYWOOD, WONG, LIMPERT],
      notes: tx(
        "Outer dimensions, wheelbase (3.0 m), track (1.4 m) and tyre size (5.00-19, overall radius 0.37 m) follow the class data of the Voisin C7/C11 saloons; the body lines are an original drawing of that type, not a copy of a particular car. Wheels roll consistently: the clip poses satisfy theta = -x / r with r = 0.37 m. The plan view has no wheels because fenders and running boards cover them from above. Thermal numbers are scaled-down authored estimates of a 40 kW petrol engine, tagged authored-estimate.",
        "Außenmaße, Radstand (3,0 m), Spurweite (1,4 m) und Reifengröße (5.00-19, Außenradius 0,37 m) folgen den Klassendaten der Voisin-C7/C11-Salonwagen; die Karosserielinien sind eine eigene Zeichnung dieses Typs, keine Kopie eines bestimmten Wagens. Die Räder rollen stimmig: die Clip-Posen erfüllen theta = -x / r mit r = 0,37 m. Der Grundriss zeigt keine Räder, da Kotflügel und Trittbretter sie von oben verdecken. Die thermischen Zahlen sind verkleinerte Schätzungen eines Benzinmotors von 40 kW und als authored-estimate gekennzeichnet.",
      ),
    },
    dimensions: { heightM: 1.7, widthM: 4.38, depthM: 1.7, massKg: 1300 },
    materials,
    rig,
    anchors,
    representations,
    symbols: [],
    poses,
    clips,
    thermal,
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: [],
      assumptions: [
        tx("Frame, body, fenders and spare wheels are one rigid point mass; the two axles carry it through four tyre patches (static front share about 48 %). No suspension is modelled.", "Rahmen, Aufbau, Kotflügel und Reserveräder sind eine starre Punktmasse; die zwei Achsen tragen sie über vier Reifenaufstandsflächen (statischer Vorderachsanteil etwa 48 %). Eine Federung ist nicht modelliert."),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "parked", defaultClip: "drive-by", defaultScale: 100, viewpoints: ["elevation", "plan"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
}
