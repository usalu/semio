/** 🍳️ `used-electric-cooker`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-electric-cooker.ts <asset document> [--force] */
import { type Script, box, ext, main, of, rect, rev } from "./lib.ts";

const plate = (radius: number) => [[0.782, 0], [0.782, radius], [0.857, radius], [0.857, 0]] as const;

const script: Script = {
  id: "used-electric-cooker",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the pallet as one 144 mm slab; the cooker as one block from the pallet to the hob (casing, drawer, oven, fascia and hob slab merged), the four hotplates as discs of their true 199 and 163 mm that keep their 7 mm above the hob, and the oven door as one 75 mm leaf on the hinge that stands its true 30 mm proud, with the window as a glass block set 10 mm back; knobs, grip, tray and feet are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: die Palette als eine 144-mm-Platte; der Herd als ein Block von der Palette bis zur Kochmulde (Gehäuse, Schublade, Backofen, Blende und Muldenplatte verschmolzen), die vier Kochplatten als Scheiben ihrer wahren 199 und 163 mm, die ihre 7 mm über der Mulde behalten, und die Backofentür als ein 75-mm-Blatt am Scharnier, das seine wahren 30 mm vorsteht, mit dem Fenster als 10 mm zurückgesetztem Glasblock; Knebel, Griff, Blech und Füße liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: pallet, plates, door and door glass by reference, the block as 75 mm region solids (two sides, back, hob top, fascia, drawer front), a 75 mm oven vent and a 75 mm plume patch over the front left plate.", de: "Thermik: Palette, Platten, Tür und Türglas als Verweis, der Block als 75-mm-Regionskörper (zwei Seiten, Rückwand, Mulde, Blende, Schubladenfront), ein 75-mm-Backofenabzug und eine 75-mm-Auftriebsfläche über der Platte vorn links." },
        { en: "Structure: slab, block and door by reference, the block spanning drawer and tray, two 75 mm hinge joints and five mass markers.", de: "Tragwerk: Platte, Block und Tür als Verweis, der Block überspannt Schublade und Blech, zwei 75-mm-Scharniergelenke und fünf Massenmarken." },
      ],
      body: [
        box(["pallet-slab", "pallet", "spruce-pallet", 0.55], [0.6, 0.144, 0.8], [0.3, 0.072, 0]),
        box(["cooker-block", "carcass", "enamel-white-aged", 0.88], [0.57, 0.85, 0.5], [0.285, 0.425, 0]),
        rev(["plates-large", "carcass", "cast-iron-aged", 0.12], [0, 1, 0], [0.41, 0, 0.12], plate(0.0995), { repeat: { count: 2, translation: [-0.25, 0, -0.24] } }),
        rev(["plates-small", "carcass", "cast-iron-aged", 0.12], [0, 1, 0], [0.41, 0, -0.12], plate(0.0815), { repeat: { count: 2, translation: [-0.25, 0, 0.24] } }),
        ext(["door-leaf", "door", "enamel-white-aged", 0.9], "zx", rect(-0.247, 0.247, 0, 0.52), 0, 0.075, { holes: [rect(-0.16, 0.16, 0.16, 0.4)] }),
        box(["door-glass", "door", "door-glass", 0.25, "glazing"], [0.24, 0.065, 0.32], [0.28, 0.0425, 0]),
      ],
      thermal: [
        of("th-pallet-slab", "pallet-slab", { region: "pallet" }),
        of("th-plates-large", "plates-large", { region: "hotplates" }),
        of("th-plates-small", "plates-small", { region: "hotplates" }),
        of("th-door-leaf", "door-leaf", { region: "oven-door" }),
        box(["th-side-r", "carcass", "enamel-white-aged", 0.88], [0.57, 0.775, 0.075], [0.285, 0.3875, -0.2125], { region: "casing" }),
        box(["th-side-l", "carcass", "enamel-white-aged", 0.88], [0.57, 0.775, 0.075], [0.285, 0.3875, 0.2125], { region: "casing" }),
        box(["th-back", "carcass", "enamel-white-aged", 0.88], [0.075, 0.775, 0.35], [0.0375, 0.3875, 0], { region: "casing" }),
        box(["th-hob-top", "carcass", "enamel-hob-stained", 0.82], [0.57, 0.075, 0.5], [0.285, 0.8125, 0], { region: "hob-top" }),
        box(["th-fascia", "carcass", "enamel-white-aged", 0.86], [0.075, 0.075, 0.35], [0.5325, 0.7375, 0], { region: "fascia" }),
        box(["th-drawer", "drawer", "enamel-white-aged", 0.86], [0.075, 0.165, 0.35], [0.5325, 0.1125, 0], { region: "drawer" }),
        of("th-glass", "door-glass", { region: "door-window" }),
        box(["th-oven-vent", "carcass", "enamel-hob-stained", 0.3, "detail"], [0.075, 0.075, 0.3], [0.0375, 0.82, 0], { region: "hob-top", tags: ["oven-vent"] }),
        box(["th-hob-plume", "carcass", "cast-iron-aged", 0.12, "detail"], [0.1, 0.075, 0.1], [0.41, 0.8945, 0.12], { region: "hotplates", tags: ["hob-plume"] }),
      ],
      structure: [
        of("m-pallet-slab", "pallet-slab", { member: "pallet", part: "member", members: ["pallet-deck"] }),
        of("m-cooker-block", "cooker-block", { member: "carcass", part: "member", members: ["drawer", "tray"] }),
        of("m-door-leaf", "door-leaf", { member: "door", part: "member" }),
        of("m-door-glass", "door-glass", { member: "door", part: "member" }),
        box(["j-hinge-r", "door", "chrome-steel", 0.5, "detail"], [0.075, 0.075, 0.075], [0.0375, 0.0375, -0.2095], { member: "door", part: "joint" }),
        box(["j-hinge-l", "door", "chrome-steel", 0.5, "detail"], [0.075, 0.075, 0.075], [0.0375, 0.0375, 0.2095], { member: "door", part: "joint" }),
        box(["mass-carcass", "carcass", "chrome-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0.296, 0.559, 0], { member: "carcass", part: "mass" }),
        box(["mass-door", "door", "chrome-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0.279, 0, 0], { member: "door", part: "mass" }),
        box(["mass-drawer", "drawer", "chrome-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0.352, 0.069, 0], { member: "drawer", part: "mass" }),
        box(["mass-tray", "tray", "chrome-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0.345, 0.453, 0], { member: "tray", part: "mass" }),
        box(["mass-pallet", "pallet", "chrome-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0.3, 0.061, 0], { member: "pallet", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
