/** 🍽️ `used-dishwasher`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-dishwasher.ts <asset document> [--force] */
import { type Script, box, ext, main, of, rect } from "./lib.ts";

const script: Script = {
  id: "used-dishwasher",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the pallet as one 144 mm slab; the machine as one block from the pallet to the top of the worktop (base, plinth, panels, tub and worktop merged) with the plinth step under the door and the worktop as a 75 mm lip over it, and the door with its control panel as one 75 mm leaf on the hinge; baskets, hoses, plug and every sheet edge are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: die Palette als eine 144-mm-Platte; die Maschine als ein Block von der Palette bis zur Oberkante der Arbeitsplatte (Bodengruppe, Sockel, Bleche, Bottich und Arbeitsplatte verschmolzen) mit der Sockelstufe unter der Tür und der Arbeitsplatte als 75-mm-Lippe darüber, und die Tür mit ihrer Bedienblende als ein 75-mm-Blatt am Scharnier; Körbe, Schläuche, Stecker und jede Blechkante liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: pallet and door by reference, the block split into 75 mm region solids (base, plinth, two sides, back, worktop), the hidden tub as a 75 mm tube between the sides with the two racks as blocks in it, the hoses as a 75 mm block on the back and a 75 mm steam outlet at the door top.", de: "Thermik: Palette und Tür als Verweis, der Block in 75-mm-Regionskörper geteilt (Bodengruppe, Sockel, zwei Seiten, Rückwand, Arbeitsplatte), der verdeckte Bottich als 75-mm-Rohr zwischen den Seiten mit den zwei Körben als Blöcken darin, die Schläuche als 75-mm-Block an der Rückwand und ein 75-mm-Dampfauslass an der Türoberkante." },
        { en: "Structure: slab, block and door by reference, the block spanning the bones of the cabinet, two 75 mm hinge joints, a 75 mm pallet support and the mass marker.", de: "Tragwerk: Platte, Block und Tür als Verweis, der Block überspannt die Knochen des Gehäuses, zwei 75-mm-Scharniergelenke, ein 75-mm-Auflager der Palette und die Massenmarke." },
      ],
      body: [
        box(["pallet-slab", "pallet", "pallet-spruce", 0.55], [0.8, 0.144, 0.6], [0.4, 0.072, 0]),
        ext(["machine-block", "body", "painted-steel", 0.88], "xy", [[0, 0], [0.6, 0], [0.6, 0.125], [0.525, 0.125], [0.525, 0.775], [0.6, 0.775], [0.6, 0.85], [0, 0.85]], -0.3, 0.3),
        box(["door-leaf", "door", "door-composite", 0.92], [0.65, 0.075, 0.6], [0.325, -0.0175, 0]),
      ],
      thermal: [
        of("th-pallet-slab", "pallet-slab", { region: "pallet" }),
        of("th-door-leaf", "door-leaf", { region: "door" }),
        box(["th-base", "base", "base-unit", 0.4], [0.525, 0.11, 0.6], [0.2625, 0.055, 0], { region: "base" }),
        box(["th-plinth", "plinth", "pp-plinth", 0.85], [0.075, 0.115, 0.6], [0.0375, 0.0575, 0], { region: "plinth" }),
        box(["th-side-l", "side-l", "painted-steel", 0.88], [0.665, 0.525, 0.075], [0.4225, -0.2625, -0.0375], { region: "cabinet" }),
        box(["th-side-r", "side-r", "painted-steel", 0.88], [0.665, 0.525, 0.075], [0.4225, -0.2625, 0.0375], { region: "cabinet" }),
        box(["th-back", "back", "painted-steel", 0.8], [0.665, 0.075, 0.45], [0.4225, -0.0375, 0], { region: "cabinet" }),
        box(["th-worktop", "worktop", "laminate-chipboard", 0.75], [0.6, 0.075, 0.6], [0.3, -0.0075, 0], { region: "worktop" }),
        ext(["th-tub", "tub", "stainless-steel", 0.55], "xy", rect(0.03, 0.48, -0.015, 0.65), -0.225, 0.225, { holes: [rect(0.105, 0.405, 0.06, 0.575)], region: "tub" }),
        box(["th-rack-lower", "basket-lower", "coated-wire", 0.45, "detail"], [0.26, 0.19, 0.4], [0.235, 0.17, 0], { region: "racks" }),
        box(["th-rack-upper", "basket-upper", "coated-wire", 0.45, "detail"], [0.26, 0.17, 0.4], [0.235, 0.1, 0], { region: "racks" }),
        box(["th-hoses", "hoses", "pvc-hose", 0.45], [0.075, 0.5, 0.45], [-0.0375, 0.3, 0], { region: "hoses" }),
        box(["th-door-steam", "door", "door-composite", 0.5, "detail"], [0.075, 0.075, 0.4], [0.6125, -0.0175, 0], { region: "door", tags: ["door-steam"] }),
      ],
      structure: [
        of("m-pallet-slab", "pallet-slab", { member: "pallet", part: "member" }),
        of("m-machine-block", "machine-block", { member: "base", part: "member", members: ["body", "side-l", "side-r", "back", "worktop", "tub", "basket-lower", "basket-upper"] }),
        of("m-door-leaf", "door-leaf", { member: "door", part: "member" }),
        box(["j-hinge-r", "door", "painted-steel", 0.4, "detail"], [0.075, 0.075, 0.075], [0.0375, -0.0175, -0.2625], { member: "door", part: "joint" }),
        box(["j-hinge-l", "door", "painted-steel", 0.4, "detail"], [0.075, 0.075, 0.075], [0.0375, -0.0175, 0.2625], { member: "door", part: "joint" }),
        box(["support-pallet-floor", "pallet", "pallet-spruce", 0.2, "ground"], [0.6, 0.075, 0.5], [0.4, 0.0375, 0], { member: "pallet", part: "support", anchor: "pallet-floor" }),
        box(["mass-machine", "body", "base-unit", 0.3, "detail"], [0.075, 0.075, 0.075], [0.3, 0.36, 0], { member: "base", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
