import { ASHRAE, SRC, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, u500 } from "./g2.mjs";

const sp = space([
  ["hob", null, 0, 0, { title: tx("Hob body with steel plate", "Kochfeldkörper mit Stahlplatte"), lengthM: 0.3, joint: fixed, mass: m(4.1, 0, 0.02) }],
  ["burner", "hob", 0, 0.04, { title: tx("Burner head with cap", "Brennerkopf mit Deckel"), lengthM: 0.02, joint: fixed, mass: m(0.4, 0, 0.047) }],
  ["trivet", "hob", 0, 0.04, { title: tx("Cast-iron pan support", "Pfannenträger aus Gusseisen"), lengthM: 0.05, joint: fixed, mass: m(0.7, 0, 0.07) }],
  ["knob", "hob", 0.105, 0.019, { title: tx("Control knob", "Bedienknebel"), lengthM: 0.012, joint: { type: "revolute", minDeg: -130, maxDeg: 130 }, mass: m(0.03, 0.105, 0.019) }],
  ["flame", "burner", 0, 0.047, { title: tx("Flame lift (hides the flame behind the plate)", "Flammenhub (verbirgt die Flamme hinter der Platte)"), lengthM: 0.02, joint: { type: "prismatic", axisDeg: 90, minM: -0.036, maxM: 0.004 } }],
  ["flame-body", "flame", 0, 0.047, { title: tx("Flame crown, leans in a draught", "Flammenkranz, neigt sich im Luftzug"), lengthM: 0.02, joint: { type: "revolute", minDeg: -20, maxDeg: 20 } }],
  ["flame-tip", "flame-body", 0, 0.062, { title: tx("Central flame tongue, stretches in gusts", "Mittlere Flammenzunge, streckt sich in Böen"), lengthM: 0.01, joint: { type: "prismatic", axisDeg: 90, minM: -0.006, maxM: 0.006 } }],
]);
const { rect, ellipse, poly, path } = sp;

const tongue = (cx, base, h, w) => `M ${cx - w / 2} ${base} C ${cx - w / 2} ${base + h * 0.45} ${cx - w * 0.2} ${base + h * 0.75} ${cx} ${base + h} C ${cx + w * 0.2} ${base + h * 0.75} ${cx + w / 2} ${base + h * 0.45} ${cx + w / 2} ${base} Z`;

const rep50 = rep(50, [20, 75], { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 }, [
  path("flame-left", "flame-body", "body", 3.4, true, 0.92, tongue(-0.03, 0.047, 0.02, 0.016), { region: "flame", material: "flame" }),
  path("flame-right", "flame-body", "body", 3.4, true, 0.92, tongue(0.03, 0.047, 0.02, 0.016), { region: "flame", material: "flame" }),
  path("flame-centre", "flame-tip", "body", 3.4, true, 0.97, tongue(0, 0.047, 0.027, 0.018), { region: "flame", material: "flame" }),
  poly("burner-cap", "burner", "body", 3.3, true, 0.55, [[-0.02, 0.047], [0.02, 0.047], [0.016, 0.058], [-0.016, 0.058]], true, { region: "burner", material: "brass" }),
  poly("burner-ring", "burner", "body", 3.5, true, 0.5, [[-0.052, 0.04], [0.052, 0.04], [0.046, 0.047], [-0.046, 0.047]], true, { region: "burner", material: "brass" }),
  rect("plate", "hob", "body", 3.6, true, 0.3, -0.15, 0, 0.3, 0.04, { region: "plate", material: "steel-enamel" }),
  poly("plate-lip", "hob", "detail", 3.7, false, 0.8, [[-0.15, 0.03], [0.15, 0.03]], false, { region: "plate", material: "steel-enamel" }),
  ellipse("knob", "knob", "body", 3.8, true, 0.6, 0.105, 0.019, 0.011, 0.011, { region: "knob", material: "bakelite" }),
  poly("knob-pointer", "knob", "detail", 3.9, false, 0.1, [[0.105, 0.019], [0.105, 0.03]], false, { region: "knob", material: "bakelite" }),
  rect("trivet-leg-left", "trivet", "body", 3.2, true, 0.45, -0.127, 0.04, 0.014, 0.04, { region: "trivet", material: "cast-iron" }),
  rect("trivet-leg-right", "trivet", "body", 3.2, true, 0.45, 0.113, 0.04, 0.014, 0.04, { region: "trivet", material: "cast-iron" }),
  rect("trivet-rail", "trivet", "body", 3.2, true, 0.45, -0.135, 0.078, 0.27, 0.01, { region: "trivet", material: "cast-iron" }),
], dec(
  ["A 0.30 m domino hob seen from the front, 6 mm by 1.8 mm tall on paper: a flat plate with the burner standing on it, the flame crown above and the cast-iron pan support spanning over it.", "Ein 0,30 m breites Domino-Kochfeld in der Frontansicht, auf Papier 15 mm breit und 4,4 mm hoch: eine flache Platte mit dem Brenner darauf, dem Flammenkranz darüber und dem Pfannenträger aus Gusseisen darüber hinweg."],
  ["The plate with its front lip, the control knob with pointer, the burner ring and cap, three flame tongues (the middle one taller) and the pan support with its two legs and top rail.", "Die Platte mit ihrer Frontkante, der Bedienknebel mit Zeiger, Brennerring und -deckel, drei Flammenzungen (die mittlere höher) und der Pfannenträger mit seinen zwei Beinen und der oberen Schiene."],
  ["Nothing is merged: every region has its own shape.", "Nichts ist zusammengeführt: jede Region hat ihre eigene Form."],
  ["The second burner, the grate fingers towards the centre, the burner flame ports and the plate seam.", "Der zweite Brenner, die Roststäbe zur Mitte hin, die Flammenöffnungen des Brenners und die Plattenfuge."],
  ["The flame ring is drawn as three tongues 16 mm wide with 11 mm gaps so that they stay separate on paper; the trivet legs are 14 mm wide.", "Der Flammenkranz ist als drei Zungen von 16 mm Breite mit 11 mm Lücken gezeichnet, damit sie auf Papier getrennt bleiben; die Beine des Pfannenträgers sind 14 mm breit."],
  ["Plate, knob, burner, flame and pan support are five regions; in the thermal view the flame and the pan support glow, the burner head warms and the plate stays cool except around the burner.", "Platte, Knebel, Brenner, Flamme und Pfannenträger sind fünf Regionen; in der Thermalansicht glühen Flamme und Pfannenträger, der Brennerkopf erwärmt sich, und die Platte bleibt außer um den Brenner kühl."],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
));

const rep100 = rep(100, [75, 150], { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  path("flame-crown", "flame-body", "body", 3.4, true, 0.92, "M -0.042 0.047 C -0.044 0.058 -0.036 0.066 -0.028 0.067 C -0.022 0.064 -0.02 0.062 -0.012 0.07 C -0.006 0.076 -0.003 0.08 0 0.081 C 0.003 0.08 0.006 0.076 0.012 0.07 C 0.02 0.062 0.022 0.064 0.028 0.067 C 0.036 0.066 0.044 0.058 0.042 0.047 Z", { region: "flame", material: "flame" }),
  poly("burner", "burner", "body", 3.5, true, 0.5, [[-0.052, 0.04], [0.052, 0.04], [0.044, 0.047], [0.02, 0.047], [0.016, 0.058], [-0.016, 0.058], [-0.02, 0.047], [-0.044, 0.047]], true, { region: "burner", material: "brass" }),
  rect("plate", "hob", "body", 3.6, true, 0.3, -0.15, 0, 0.3, 0.04, { regions: ["plate", "knob"], material: "steel-enamel" }),
  rect("trivet-leg-left", "trivet", "body", 3.2, true, 0.45, -0.128, 0.04, 0.016, 0.04, { region: "trivet", material: "cast-iron" }),
  rect("trivet-leg-right", "trivet", "body", 3.2, true, 0.45, 0.112, 0.04, 0.016, 0.04, { region: "trivet", material: "cast-iron" }),
  rect("trivet-rail", "trivet", "body", 3.2, true, 0.45, -0.136, 0.078, 0.272, 0.012, { region: "trivet", material: "cast-iron" }),
], dec(
  ["A 0.30 m hob 3 mm wide on paper: a flat plate, a burner wedge with its flame crown and the pan support bridge above.", "Ein 0,30 m breites Kochfeld, auf Papier 3 mm breit: eine flache Platte, ein Brennerkeil mit Flammenkranz und die Pfannenträger-Brücke darüber."],
  ["Plate, burner wedge, one scalloped flame crown with a taller middle tongue, and the pan support with legs and rail.", "Platte, Brennerkeil, ein gezackter Flammenkranz mit höherer mittlerer Zunge sowie der Pfannenträger mit Beinen und Schiene."],
  ["Plate and knob share one shape (regions plate and knob); burner ring and cap are one wedge; the three tongues are one crown.", "Platte und Knebel teilen eine Form (Regionen Platte und Knebel); Brennerring und -deckel sind ein Keil; die drei Zungen sind ein Kranz."],
  ["The knob pointer, the plate lip and the single tongues.", "Der Knebelzeiger, die Plattenkante und die einzelnen Zungen."],
  ["The flame crown is thickened from 48 to 84 mm wide; the legs are 16 mm wide (0.16 mm).", "Der Flammenkranz ist von 48 auf 84 mm Breite verdickt; die Beine sind 16 mm breit (0,16 mm)."],
  ["The plate shape carries plate and knob as merged regions; burner and crown stay separate, so the flame and the pan support still glow on their own.", "Die Plattenform trägt Platte und Knebel als zusammengeführte Regionen; Brenner und Kranz bleiben getrennt, sodass Flamme und Pfannenträger weiter für sich glühen."],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const rep200 = rep(200, [150, 350], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  path("flame-crown", "flame-body", "body", 3.4, true, 0.92, "M -0.05 0.047 C -0.052 0.06 -0.038 0.072 -0.02 0.074 C -0.01 0.078 -0.004 0.084 0 0.088 C 0.004 0.084 0.01 0.078 0.02 0.074 C 0.038 0.072 0.052 0.06 0.05 0.047 Z", { region: "flame", material: "flame" }),
  poly("burner", "burner", "body", 3.5, true, 0.5, [[-0.056, 0.04], [0.056, 0.04], [0.046, 0.047], [0.02, 0.047], [0.016, 0.058], [-0.016, 0.058], [-0.02, 0.047], [-0.046, 0.047]], true, { region: "burner", material: "brass" }),
  rect("plate", "hob", "body", 3.6, true, 0.3, -0.15, 0, 0.3, 0.04, { regions: ["plate", "knob"], material: "steel-enamel" }),
  rect("trivet", "trivet", "body", 3.2, true, 0.45, -0.138, 0.044, 0.276, 0.044, { region: "trivet", material: "cast-iron" }),
], dec(
  ["A 1.5 mm wide hob: a plate strip, a burner wedge with a flame drop and the pan support as one block above.", "Ein 1,5 mm breites Kochfeld: ein Plattenstreifen, ein Brennerkeil mit Flammentropfen und der Pfannenträger als ein Block darüber."],
  ["Plate strip, burner wedge, flame drop and the pan support as a closed bar.", "Plattenstreifen, Brennerkeil, Flammentropfen und der Pfannenträger als geschlossener Balken."],
  ["Legs and rail of the pan support are one block (the opening below is 0.2 mm tall, so it is closed); plate and knob share the plate strip.", "Beine und Schiene des Pfannenträgers sind ein Block (die Öffnung darunter ist 0,2 mm hoch und deshalb geschlossen); Platte und Knebel teilen den Plattenstreifen."],
  ["Knob, pointer, plate lip and every single flame tongue.", "Knebel, Zeiger, Plattenkante und jede einzelne Flammenzunge."],
  ["The pan support is thickened from 10 mm to 44 mm and lies 4 mm higher than at 1:50; the flame is thickened to 100 mm.", "Der Pfannenträger ist von 10 mm auf 44 mm verdickt und liegt 4 mm höher als bei 1:50; die Flamme ist auf 100 mm verdickt."],
  ["Four shapes carry the five regions; the flame and the pan support stay their own regions.", "Vier Formen tragen die fünf Regionen; Flamme und Pfannenträger bleiben eigene Regionen."],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const reps = {
  500: u500("the 10 mm pan support rail", "die 10 mm breite Schiene des Pfannenträgers", 0.01, 0.6),
  200: rep200,
  100: rep100,
  50: rep50,
};

const regions = [
  reg("plate", ["Steel plate", "Stahlplatte"], "surface", "operating", "steel-enamel", 0.14, 0.3, "horizontal-plate-up", 0.0008, 0.00002, 3500, { operating: { key: "burner", heatW: 25 } }),
  reg("knob", ["Control knob", "Bedienknebel"], "surface", "passive", "bakelite", 0.0025, 0.025, "vertical-plate", 0.005, 0.035, 1500, {}, { layerKind: "polymer" }),
  reg("burner", ["Burner head and cap", "Brennerkopf und -deckel"], "heater", "operating", "brass", 0.012, 0.1, "cylinder", 0.003, 0.00002, 1200, { operating: { key: "burner", heatW: 90 } }),
  reg("flame", ["Flame crown", "Flammenkranz"], "heater", "operating", "flame", 0.008, 0.025, "cylinder", 0.001, 0.04, 150, { operating: { key: "burner", heatW: 1500 } }, { layerKind: "polymer" }),
  reg("trivet", ["Cast-iron pan support", "Pfannenträger aus Gusseisen"], "heater", "operating", "cast-iron", 0.05, 0.27, "cylinder", 0.008, 0.0001, 4500, { operating: { key: "burner", heatW: 260 } }),
];

const bind = bindOf({
  plate: { exposedFraction: 0.9, contact: { fraction: 0.1, tempC: 21 } },
  knob: { exposedFraction: 1 },
  burner: { exposedFraction: 1 },
  flame: { exposedFraction: 1 },
  trivet: { exposedFraction: 1 },
});
const sup = ["base-contact"];
const poses = [
  pose("off", ["Off", "Aus"], ["The burner is closed, the flame is gone and everything is at room temperature.", "Der Brenner ist geschlossen, die Flamme fehlt und alles hat Raumtemperatur."], { flame: { dy: -0.036 }, knob: { angleDeg: 0 } }, sup, "auto", bind({ burner: 0 }, { "flame-plume": 0 })),
  pose("simmer", ["Simmer", "Köcheln"], ["The knob stands at a third and the flame burns small: about 1 kW.", "Der Knebel steht bei einem Drittel und die Flamme brennt klein: rund 1 kW."], { flame: { dy: -0.006 }, knob: { angleDeg: 45 } }, sup, "auto", bind({ burner: 0.4 }, { "flame-plume": 0.4 })),
  pose("high", ["High", "Volle Flamme"], ["The knob is fully open and the flame stands upright and releases about 2.5 kW.", "Der Knebel ist ganz geöffnet und die Flamme steht aufrecht und gibt rund 2,5 kW ab."], { knob: { angleDeg: 110 } }, sup, "auto", bind({ burner: 1 }, { "flame-plume": 1 })),
];

const clips = [
  clip(
    "flicker",
    ["Flicker", "Flackern"],
    ["The flames of the running burner lean left and right and stretch their middle tongue every 2 seconds.", "Die Flammen des laufenden Brenners neigen sich nach links und rechts und strecken alle 2 Sekunden ihre mittlere Zunge."],
    2, 15,
    [
      { t: 0, pose: "high", ease: "smooth" },
      { t: 0.5, pose: "high", ease: "smooth", bones: { "flame-body": { angleDeg: -8 }, "flame-tip": { dy: 0.005 } } },
      { t: 1, pose: "high", ease: "smooth", bones: { "flame-body": { angleDeg: 7 }, "flame-tip": { dy: -0.003 } } },
      { t: 1.5, pose: "high", ease: "smooth", bones: { "flame-body": { angleDeg: -4 }, "flame-tip": { dy: 0.004 } } },
      { t: 2, pose: "high", ease: "smooth" },
    ],
    contacts(sup, 2),
    [["rest", 0, 0.4, ["Upright", "Aufrecht"]], ["gusts", 0.4, 1.6, ["Gusts", "Böen"]], ["settle", 1.6, 2, ["Settling", "Beruhigen"]]],
    [[0.5, "thermal", ["Flame peak", "Flammenspitze"], "burner"]],
    [["operating:burner", "smooth", [[0, 0.95], [0.5, 1], [1, 0.9], [1.5, 1], [2, 0.95]]], ["emitter:flame-plume", "smooth", [[0, 0.95], [0.5, 1], [1, 0.9], [1.5, 1], [2, 0.95]]]],
  ),
  clip(
    "light-and-burn",
    ["Light and burn", "Anzünden und brennen"],
    ["The knob is turned and the burner is lit after two seconds: the flame grows out of the plate and burns on at full power while the pan support and the burner head glow up.", "Der Knebel wird gedreht und der Brenner nach zwei Sekunden gezündet: die Flamme wächst aus der Platte und brennt mit voller Leistung weiter, während Pfannenträger und Brennerkopf aufglühen."],
    300, 1,
    [
      { t: 0, pose: "off", ease: "step" },
      { t: 2, pose: "off", ease: "smooth" },
      { t: 3, pose: "high", ease: "step" },
      { t: 300, pose: "high", ease: "step" },
    ],
    contacts(sup, 300),
    [["cold", 0, 2, ["Off", "Aus"]], ["lighting", 2, 3, ["Lighting", "Zünden"]], ["burn", 3, 300, ["Burning, heating up", "Brennt, heizt auf"]]],
    [[2, "ignition", ["Burner lit", "Brenner gezündet"], "burner"]],
    [["operating:burner", "smooth", [[0, 0], [2, 0], [3, 1]]], ["emitter:flame-plume", "smooth", [[0, 0], [2, 0], [3, 1]]]],
  ),
];

export const hob = () =>
  mkAsset({
    id: "gas-hob",
    category: "appliances",
    title: ["Domino Gas Hob", "Domino-Gaskochfeld"],
    summary: [
      "A 0.30 m domino gas hob seen from the front: a steel plate with one burner, a flame crown of about 2.5 kW, a control knob and a cast-iron pan support that glows once the flame burns.",
      "Ein 0,30 m breites Domino-Gaskochfeld in der Frontansicht: eine Stahlplatte mit einem Brenner, einem Flammenkranz von rund 2,5 kW, einem Bedienknebel und einem gusseisernen Pfannenträger, der aufglüht, sobald die Flamme brennt.",
    ],
    tags: ["hob", "gas", "cooking", "flame", "burner", "kitchen", "internal-gain", "plume"],
    references: [
      ref("din-en-30-1-1-2008", "DIN EN 30-1-1:2008 Domestic cooking appliances burning gas - Safety (rated heat input of hotplate burners, typically 1 to 3 kW for a rapid burner)."),
      ref("quintiere-2006", "Quintiere, J. G. (2006). Fundamentals of Fire Phenomena, Wiley (premixed and diffusion flames, convective share of the combustion heat)."),
      ASHRAE,
    ],
    notes: [
      "A domino gas hob of 0.30 m by 0.52 m with one rapid burner of 2.5 kW rated heat input: a 0.8 mm enamelled steel plate on a pressed housing, a brass burner with a ring of flame ports 0.1 m across, a cast-iron pan support whose rail stands 48 mm above the plate (the pan sits on it, 48 mm above the burner head) and a bakelite knob at the right front. The mass of 5.2 kg is the sum of the housing (4.1 kg), burner (0.4 kg), pan support (0.7 kg) and knob (0.03 kg). Natural gas releases 10 kWh per cubic metre; the thermal model puts 1500 W into the flame region (about 60 percent of the rated input leaves the flame directly as plume and radiation, the rest reaches the pan) and 260 W into the pan support, 120 W into the burner head and 55 W into the plate (authored estimates). The origin is the centre of the burner on the plate base line; the view is the front elevation.",
      "Ein Domino-Gaskochfeld von 0,30 m mal 0,52 m mit einem Schnellkochbrenner von 2,5 kW Nennwärmebelastung: eine 0,8 mm starke emaillierte Stahlplatte auf einem gepressten Gehäuse, ein Messingbrenner mit einem Kranz von Flammenöffnungen von 0,1 m Durchmesser, ein gusseiserner Pfannenträger, dessen Schiene 48 mm über der Platte liegt (der Topf steht darauf, 48 mm über dem Brennerkopf) und ein Bakelit-Knebel vorn rechts. Die Masse von 5,2 kg ist die Summe aus Gehäuse (4,1 kg), Brenner (0,4 kg), Pfannenträger (0,7 kg) und Knebel (0,03 kg). Erdgas setzt 10 kWh je Kubikmeter frei; das Wärmemodell legt 1500 W in die Flammenregion (rund 60 Prozent der Nennbelastung verlassen die Flamme direkt als Fahne und Strahlung, der Rest erreicht den Topf) und 260 W in den Pfannenträger, 120 W in den Brennerkopf und 55 W in die Platte (Schätzwerte). Der Ursprung ist die Mitte des Brenners auf der Plattengrundlinie; die Ansicht ist die Vorderansicht.",
    ],
    dims: [0.088, 0.3, 0.52],
    materials: [
      materialOf("steel-enamel", "Enamelled steel", "Emaillierter Stahl", 0.88, 0.7, 7800, 50, 480, SRC, "incropera-2018"),
      materialOf("brass", "Brass", "Messing", 0.3, 0.5, 8500, 110, 380, "literature", "incropera-2018"),
      materialOf("cast-iron", "Cast iron", "Gusseisen", 0.85, 0.8, 7200, 50, 540, "literature", "incropera-2018"),
      materialOf("bakelite", "Phenolic resin", "Phenolharz", 0.9, 0.7, 1300, 0.2, 1500, SRC),
      materialOf("flame", "Gas flame", "Gasflamme", 0.2, 0.05, 0.3, 0.07, 1100, SRC, "quintiere-2006"),
    ],
    root: "hob",
    bones: sp.bones,
    anchors: [
      baseContact("hob", 0, 0, 0.3, 0.5),
      sp.anchor("flame-top", "flame-tip", 0, 0.08, "emitter", ["Tip of the flame crown", "Spitze des Flammenkranzes"]),
    ],
    reps,
    poses,
    clips,
    operating: [{ id: "burner", title: tx("Burner", "Brenner"), levels: [level(0, "Off", "Aus"), level(0.4, "Simmer (1 kW)", "Köcheln (1 kW)"), level(1, "High (2.5 kW)", "Volle Flamme (2,5 kW)")] }],
    regions,
    emitters: [{ id: "flame-plume", kind: "plume", title: tx("Plume above the burner", "Fahne über dem Brenner"), anchor: "flame-top", directionDeg: 90, diameterM: 0.1, region: "flame", source: SRC, reference: "quintiere-2006" }],
    assumptions: [
      ["The 2.5 kW of gas leave in three parts: 1500 W into the flame region (plume and radiation), 435 W into plate, burner head and pan support and the rest into a pan above; combustion products (carbon dioxide, 0.4 g/s of water vapour) are neglected.", "Die 2,5 kW Gas verteilen sich in drei Teile: 1500 W in die Flammenregion (Fahne und Strahlung), 435 W in Platte, Brennerkopf und Pfannenträger und der Rest in einen Topf darüber; die Verbrennungsprodukte (Kohlendioxid, 0,4 g/s Wasserdampf) bleiben unberücksichtigt."],
      ["The unlit pose hides the flame by lowering the flame bones behind the plate, so one asset shows all states.", "Die Pose Aus verbirgt die Flamme, indem sie die Flammenknochen hinter die Platte absenkt, sodass ein Asset alle Zustände zeigt."],
    ],
    structure: [["The 5.2 kg hob stands on its housing: a unilateral contact of 0.30 m by 0.50 m carries the weight.", "Das 5,2 kg schwere Kochfeld steht auf seinem Gehäuse: ein einseitiger Kontakt von 0,30 m mal 0,50 m trägt das Gewicht."]],
    defaultPose: "high",
    defaultClip: "flicker",
  });
