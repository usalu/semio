/** 🌀️ `used-tumble-dryer`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-tumble-dryer.ts <asset document> [--force] */
import { type Script, box, main, of, rev } from "./lib.ts";

const script: Script = {
  id: "used-tumble-dryer",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the dryer is one block from the blanket to the top cover (base, plinth flap, panels, fascia, tank front and cover merged), the porthole door an 80 mm wide ring of 0.46 m around its 0.30 m glass, 75 mm deep and 20 mm proud of the front (the glass 10 mm), on the blanket as its declared 16 mm sheet; cable coil and feet are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: der Trockner ist ein Block von der Decke bis zum Deckel (Boden, Sockelklappe, Bleche, Blende, Tankfront und Deckel verschmolzen), die Bullaugentür ein 80 mm breiter Ring von 0,46 m um ihr 0,30-m-Glas, 75 mm tief und 20 mm vor der Front (das Glas 10 mm), auf der Decke als erklärter 16-mm-Platte; Kabelring und Füße liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: blanket and door by reference, the block as 75 mm region solids (plinth, two sides, back, front, top cover, fascia, tank), the cable as a 75 mm block on the back, the hidden drum as a cylinder and the hidden heat pump as a block inside, a 75 mm vent outlet at the plinth.", de: "Thermik: Decke und Tür als Verweis, der Block als 75-mm-Regionskörper (Sockel, zwei Seiten, Rückwand, Front, Deckel, Blende, Tank), das Kabel als 75-mm-Block an der Rückwand, die verdeckte Trommel als Zylinder und die verdeckte Wärmepumpe als Block im Inneren, ein 75-mm-Luftauslass am Sockel." },
        { en: "Structure: blanket, block and door by reference, the tank as a 75 mm member, four 75 mm foot joints and the hinge, a 75 mm support and four mass markers.", de: "Tragwerk: Decke, Block und Tür als Verweis, der Tank als 75-mm-Stab, vier 75-mm-Fußgelenke und das Scharnier, ein 75-mm-Auflager und vier Massenmarken." },
      ],
      body: [
        box(["blanket", "blanket", "moving-blanket", 0.45], [0.9, 0.016, 0.75], [0.45, 0.008, 0], { sheet: true }),
        box(["unit-block", "unit", "enamel-steel", 0.86], [0.595, 0.85, 0.6], [0.2975, 0.425, 0]),
        rev(["door-ring", "door", "door-abs", 0.72], [0, 1, 0], [0, 0, 0.235], [[-0.02, 0.15], [-0.02, 0.23], [0.055, 0.23], [0.055, 0.15], [-0.02, 0.15]]),
        rev(["door-glass", "door", "door-glass", 0.3], [0, 1, 0], [0, 0, 0.235], [[-0.01, 0], [-0.01, 0.15], [0.065, 0.15], [0.065, 0]]),
      ],
      thermal: [
        of("th-blanket", "blanket", { region: "blanket" }),
        of("th-door-ring", "door-ring", { region: "door" }),
        of("th-door-glass", "door-glass", { region: "door" }),
        box(["th-plinth", "unit", "base-pp", 0.45], [0.605, 0.115, 0.6], [0.3025, 0.0575, 0], { region: "plinth" }),
        box(["th-side-right", "unit", "enamel-steel", 0.86], [0.605, 0.66, 0.075], [0.3025, 0.445, -0.2625], { region: "cabinet" }),
        box(["th-side-left", "unit", "enamel-steel", 0.86], [0.605, 0.66, 0.075], [0.3025, 0.445, 0.2625], { region: "cabinet" }),
        box(["th-back", "unit", "enamel-steel", 0.8], [0.075, 0.66, 0.45], [0.0375, 0.445, 0], { region: "cabinet" }),
        box(["th-front", "unit", "enamel-steel", 0.88], [0.075, 0.585, 0.45], [0.5675, 0.4075, 0], { region: "cabinet" }),
        box(["th-top-cover", "unit", "top-pp", 0.92], [0.605, 0.075, 0.6], [0.3025, 0.8125, 0], { region: "top-cover" }),
        box(["th-fascia", "unit", "fascia-abs", 0.78], [0.075, 0.075, 0.325], [0.5675, 0.7375, -0.0625], { region: "fascia" }),
        box(["th-tank", "tank", "tank-pp", 0.7], [0.075, 0.075, 0.125], [0.2975, -0.0375, -0.0375], { region: "tank" }),
        box(["th-cable", "unit", "pvc-cable", 0.3], [0.075, 0.14, 0.14], [-0.0375, 0.62, -0.15], { region: "cable" }),
        rev(["th-drum", "unit", "stainless-steel", 0.5], [1, 0, 0], [0, 0.455, 0], [[0.075, 0], [0.075, 0.225], [0.53, 0.225], [0.53, 0]], { region: "drum" }),
        box(["th-heat-pump-module", "unit", "aluminium-fin", 0.6], [0.5, 0.13, 0.52], [0.27, 0.11, 0], { region: "heat-pump" }),
        box(["th-plinth-vent", "unit", "base-pp", 0.5, "detail"], [0.075, 0.075, 0.2], [0.6425, 0.075, 0], { region: "plinth", tags: ["plinth-vent"] }),
      ],
      structure: [
        of("m-blanket", "blanket", { member: "blanket", part: "member" }),
        of("m-unit-block", "unit-block", { member: "unit", part: "member" }),
        of("m-door", "door-ring", { member: "door", part: "member" }),
        of("m-door-glass", "door-glass", { member: "door", part: "member" }),
        box(["m-tank", "tank", "tank-pp", 0.7], [0.335, 0.1, 0.2], [0.1675, -0.005, 0], { member: "tank", part: "member" }),
        box(["support-blanket-floor", "blanket", "moving-blanket", 0.3, "ground"], [0.6, 0.075, 0.5], [0.45, 0.0375, 0], { member: "blanket", part: "support", anchor: "blanket-floor" }),
        box(["j-foot-front-right", "unit", "foot-rubber", 0.3], [0.075, 0.075, 0.075], [0.54, 0.0375, -0.25], { member: "unit", part: "joint" }),
        box(["j-foot-front-left", "unit", "foot-rubber", 0.3], [0.075, 0.075, 0.075], [0.54, 0.0375, 0.25], { member: "unit", part: "joint" }),
        box(["j-foot-back-right", "unit", "foot-rubber", 0.3], [0.075, 0.075, 0.075], [0.05, 0.0375, -0.25], { member: "unit", part: "joint" }),
        box(["j-foot-back-left", "unit", "foot-rubber", 0.3], [0.075, 0.075, 0.075], [0.05, 0.0375, 0.25], { member: "unit", part: "joint" }),
        box(["j-door-hinge", "door", "hinge-zinc", 0.3], [0.1, 0.075, 0.075], [0, 0.0025, -0.0225], { member: "door", part: "joint" }),
        box(["mass-unit", "unit", "enamel-steel", 0.5, "detail"], [0.075, 0.075, 0.075], [0.29, 0.311, 0], { member: "unit", part: "mass" }),
        box(["mass-door", "door", "door-abs", 0.5, "detail"], [0.075, 0.075, 0.075], [0, 0.0025, 0.235], { member: "door", part: "mass" }),
        box(["mass-tank", "tank", "tank-pp", 0.5, "detail"], [0.075, 0.075, 0.075], [0.17, 0, 0], { member: "tank", part: "mass" }),
        box(["mass-blanket", "blanket", "moving-blanket", 0.5, "detail"], [0.075, 0.075, 0.075], [0.45, 0.0375, 0], { member: "blanket", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
