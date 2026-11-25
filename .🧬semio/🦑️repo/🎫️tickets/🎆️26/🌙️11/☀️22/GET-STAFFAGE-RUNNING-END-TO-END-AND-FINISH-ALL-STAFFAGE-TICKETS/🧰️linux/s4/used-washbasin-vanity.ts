/** 🚰️ `used-washbasin-vanity`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-washbasin-vanity.ts <asset document> [--force] */
import { type Script, box, ext, main, of, rect } from "./lib.ts";

const bowl = [[0.0479, 0.4014], [0.1308, 0.3662], [0.1787, 0.3052], [0.1787, 0.2348], [0.1308, 0.1738], [0.0479, 0.1386], [-0.0479, 0.1386], [-0.1308, 0.1738], [-0.1787, 0.2348], [-0.1787, 0.3052], [-0.1308, 0.3662], [-0.0479, 0.4014]] as const;
const cube = [0.075, 0.075, 0.075] as const;

const script: Script = {
  id: "used-washbasin-vanity",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the pallet as one 144 mm slab; the mirror cabinet as one block under its light bar raised from 60 to 75 mm with the top in place; the vanity as one block that takes carcass, back, drawer box and drawer front in; the basin as one 75 mm slab on it (deck and apron) with the oval mouth of the bowl as its opening down to a floor declared as a 10 mm sheet (the depth of the bowl below stays inside the block) and the tap with spout and lever as one 75 mm post that keeps the height of the lever.", de: "Körper für das 75-mm-Merkmal: die Palette als eine 144-mm-Platte; der Spiegelschrank als ein Block unter seiner von 60 auf 75 mm erhöhten Lichtleiste mit der Oberkante am Ort; der Unterschrank als ein Block, der Korpus, Rückwand, Schubkasten und Schubladenfront aufnimmt; das Becken als eine 75-mm-Platte darauf (Deck und Schürze) mit dem ovalen Mund der Mulde als Öffnung bis auf einen als 10-mm-Platte erklärten Boden (die Tiefe der Mulde darunter bleibt im Block) und die Armatur mit Auslauf und Hebel als ein 75-mm-Pfosten, der die Höhe des Hebels hält." },
        { en: "Thermal: the body solids in their regions by reference; the mirror doors as their declared 8 mm sheet before the block, the drawer front as a 75 mm shell of its layer in the vanity front, the hidden water as a shell of its layer in the basin, a 75 mm light plume patch in the light bar and a 75 mm mist patch in the basin.", de: "Thermik: die Körper in ihren Regionen als Verweis; die Spiegeltüren als erklärte 8-mm-Platte vor dem Block, die Schubladenfront als 75-mm-Schale ihrer Schicht in der Unterschrankfront, das verdeckte Wasser als Schale seiner Schicht im Becken, eine 75-mm-Auftriebsfläche in der Lichtleiste und eine 75-mm-Nebelfläche im Becken." },
        { en: "Structure: the members by reference, the vanity also holding the drawer bone and the tap the lever bone, the lever pivot as a 75 mm joint and five mass markers.", de: "Tragwerk: die Stäbe als Verweis, der Unterschrank fasst auch den Schubladenknochen und die Armatur den Hebelknochen, das Hebellager als 75-mm-Gelenk und fünf Massenmarken." },
      ],
      body: [
        box(["pallet-slab", "pallet", "pallet-spruce", 0.48], [1.2, 0.144, 0.8], [0.6, 0.072, 0]),
        box(["mirror-block", "mirror", "melamine-chipboard", 0.82], [0.148, 0.685, 0.6], [0.074, 0.3425, 0]),
        box(["light-bar", "mirror", "opal-acrylic", 0.85], [0.17, 0.075, 0.6], [0.085, 0.7225, 0]),
        box(["vanity-block", "vanity", "melamine-chipboard", 0.8], [0.459, 0.5, 0.58], [0.2295, 0.25, 0]),
        ext(["basin-slab", "basin", "vitreous-china", 0.92], "zx", rect(-0.3, 0.3, 0, 0.48), -0.005, 0.07, { holes: [bowl] }),
        ext(["bowl-floor", "basin", "vitreous-china", 0.86], "zx", bowl, -0.005, 0.005, { sheet: true }),
        box(["tap", "basin", "chrome-brass", 0.55], [0.075, 0.12, 0.075], [0.07, 0.13, 0]),
      ],
      thermal: [
        of("th-pallet-slab", "pallet-slab", { region: "pallet" }),
        of("th-mirror-block", "mirror-block", { region: "mirror-carcass" }),
        of("th-light-bar", "light-bar", { region: "light-bar" }),
        of("th-vanity-block", "vanity-block", { region: "vanity-carcass" }),
        of("th-basin-slab", "basin-slab", { region: "basin" }),
        of("th-bowl-floor", "bowl-floor", { region: "basin" }),
        of("th-tap", "tap", { region: "tap" }),
        box(["th-mirror-doors", "mirror", "mirror-glass", 0.68], [0.008, 0.685, 0.6], [0.152, 0.3425, 0], { region: "mirror-doors", layer: "mirror", sheet: true }),
        box(["th-drawer-front", "drawer", "mdf-gloss", 0.9], [0.075, 0.496, 0.568], [0.3815, 0.221, 0], { region: "drawer", layer: "d-board" }),
        box(["th-water-stream", "basin", "water", 0.5, "detail"], [0.046, 0.196, 0.03], [0.202, 0.013, 0], { region: "water", layer: "stream" }),
        box(["th-light-plume", "mirror", "opal-acrylic", 0.4, "detail"], [0.075, 0.075, 0.4], [0.13, 0.7225, 0], { region: "light-bar", tags: ["light-plume"] }),
        box(["th-basin-mist", "basin", "water", 0.4, "detail"], cube, [0.22, 0.0325, 0], { region: "water", tags: ["basin-mist"] }),
      ],
      structure: [
        of("m-pallet-slab", "pallet-slab", { member: "pallet", part: "member" }),
        of("m-mirror-block", "mirror-block", { member: "mirror", part: "member" }),
        of("m-light-bar", "light-bar", { member: "mirror", part: "member" }),
        of("m-vanity-block", "vanity-block", { member: "vanity", part: "member", members: ["drawer"] }),
        of("m-basin-slab", "basin-slab", { member: "basin", part: "member" }),
        of("m-bowl-floor", "bowl-floor", { member: "basin", part: "member" }),
        of("m-tap", "tap", { member: "basin", part: "member", members: ["lever"] }),
        box(["j-lever-pivot", "lever", "chrome-brass", 0.3, "detail"], cube, [0, 0, 0], { member: "lever", part: "joint" }),
        box(["mass-pallet", "pallet", "pallet-spruce", 0.3, "detail"], [0.075, 0.075, 0.075], [0.6, 0.072, 0], { member: "pallet", part: "mass" }),
        box(["mass-mirror", "mirror", "melamine-chipboard", 0.3, "detail"], [0.075, 0.075, 0.075], [0.074, 0.35, 0], { member: "mirror", part: "mass" }),
        box(["mass-vanity", "vanity", "melamine-chipboard", 0.3, "detail"], [0.075, 0.075, 0.075], [0.22, 0.25, 0], { member: "vanity", part: "mass" }),
        box(["mass-drawer", "drawer", "mdf-gloss", 0.3, "detail"], [0.075, 0.075, 0.075], [0.2, 0.1, 0], { member: "drawer", part: "mass" }),
        box(["mass-basin", "basin", "vitreous-china", 0.3, "detail"], [0.075, 0.075, 0.075], [0.24, 0.0325, 0], { member: "basin", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
