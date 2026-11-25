/** 🚽️ `used-wc-pan-cistern`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-wc-pan-cistern.ts <asset document> [--force] */
import { type Script, box, ext, main, of, rev } from "./lib.ts";

const leaf = [[0, 0], [0.0774, 0.0237], [0.1396, 0.09], [0.174, 0.1858], [0.174, 0.2922], [0.1396, 0.388], [0.0774, 0.4543], [0, 0.478], [-0.0774, 0.4543], [-0.1396, 0.388], [-0.174, 0.2922], [-0.174, 0.1858], [-0.1396, 0.09], [-0.0774, 0.0237]] as const;
const cube = [0.075, 0.075, 0.075] as const;

const script: Script = {
  id: "used-wc-pan-cistern",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the pan is its foot as one tapered pedestal prism under the bowl as one filled turned body of its true oval outline, with rear body, neck and deck as one block under the cistern; seat ring and lid are one 75 mm leaf of their oval in fourteen points on the hinge, the outlet with its cap one 116 mm cylinder, the cistern with lid and button one block that keeps the height; the pallet is one 144 mm slab under the cardboard as its declared 5 mm sheet.", de: "Körper für das 75-mm-Merkmal: das Becken ist sein Fuß als ein verjüngtes Sockelprisma unter der Schüssel als einem vollen Drehkörper ihres wahren ovalen Umrisses, mit Hinterteil, Hals und Deck als einem Block unter dem Spülkasten; Sitzring und Deckel sind ein 75-mm-Blatt ihres Ovals in vierzehn Punkten am Scharnier, der Abgang mit seiner Kappe ein 116-mm-Zylinder, der Spülkasten mit Deckel und Taste ein Block, der die Höhe hält; die Palette ist eine 144-mm-Platte unter dem Karton als erklärter 5-mm-Platte." },
        { en: "Thermal: one surface per body solid by reference in its region.", de: "Thermik: eine Oberfläche je Körper als Verweis in seiner Region." },
        { en: "Structure: the members by reference, the bowl also holding the seat bone and the cistern its lid and button, the cistern bolts and the hinge as 75 mm joints and four mass markers.", de: "Tragwerk: die Stäbe als Verweis, die Schüssel fasst auch den Sitzknochen und der Spülkasten Deckel und Taste, die Spülkastenschrauben und das Scharnier als 75-mm-Gelenke und vier Massenmarken." },
      ],
      body: [
        box(["pallet-slab", "pallet", "spruce-pallet", 0.65], [0.8, 0.144, 0.6], [0.4, 0.072, 0]),
        box(["cardboard", "pallet", "corrugated-board", 0.5], [0.52, 0.005, 0.4], [0.39, 0.1465, 0], { sheet: true }),
        ext(["pedestal", "pan", "vitreous-china-pan", 0.86], "zx", [[-0.08, 0], [0.08, 0], [0.105, 0.07], [0.105, 0.3], [0.06, 0.42], [0, 0.44], [-0.06, 0.42], [-0.105, 0.3], [-0.105, 0.07]], 0, 0.235, { taperTo: [1, 0.88] }),
        rev(["bowl", "pan", "vitreous-china-pan", 0.92], [0, 1, 0], [0.32, 0, 0], [[0.2, 0], [0.2, 0.044], [0.24, 0.099], [0.3, 0.1455], [0.36, 0.1785], [0.4, 0.18], [0.4, 0]], { lateralScale: 1.361111 }),
        box(["rear-body", "pan", "vitreous-china-pan", 0.9], [0.215, 0.165, 0.25], [0.0125, 0.3175, 0]),
        ext(["lid", "lid", "duroplast-seat", 0.86], "zx", leaf, -0.031, 0.044),
        rev(["outlet", "pan", "pe-cap", 0.3], [1, 0, 0], [0, 0.18, 0], [[-0.095, 0], [-0.095, 0.058], [0.03, 0.058], [0.03, 0]]),
        box(["cistern", "cistern", "vitreous-china-cistern", 0.9], [0.165, 0.395, 0.375], [0.08, 0.1975, 0]),
      ],
      thermal: [
        of("th-pallet-slab", "pallet-slab", { region: "pallet" }),
        of("th-cardboard", "cardboard", { region: "cardboard" }),
        of("th-pedestal", "pedestal", { region: "pan" }),
        of("th-bowl", "bowl", { region: "pan" }),
        of("th-rear-body", "rear-body", { region: "pan" }),
        of("th-lid", "lid", { region: "seat-set" }),
        of("th-outlet", "outlet", { region: "outlet-cap" }),
        of("th-cistern", "cistern", { region: "cistern" }),
      ],
      structure: [
        of("m-pallet-slab", "pallet-slab", { member: "pallet", part: "member" }),
        of("m-cardboard", "cardboard", { member: "pallet", part: "member" }),
        of("m-pedestal", "pedestal", { member: "pan", part: "member" }),
        of("m-bowl", "bowl", { member: "pan", part: "member", members: ["seat"] }),
        of("m-rear-body", "rear-body", { member: "pan", part: "member" }),
        of("m-lid", "lid", { member: "lid", part: "member" }),
        of("m-outlet", "outlet", { member: "pan", part: "member" }),
        of("m-cistern", "cistern", { member: "cistern", part: "member", members: ["cistern-lid", "button"] }),
        box(["j-cistern-bolts", "cistern", "chromed-zinc", 0.3, "detail"], cube, [0.08, 0.0375, -0.08], { repeat: { count: 2, translation: [0, 0, 0.16] }, member: "cistern", part: "joint" }),
        box(["j-hinge", "lid", "chromed-zinc", 0.3, "detail"], cube, [0.0375, 0.0065, 0], { member: "lid", part: "joint" }),
        box(["mass-pan", "pan", "vitreous-china-pan", 0.3, "detail"], [0.075, 0.075, 0.075], [0.25, 0.2, 0], { member: "pan", part: "mass" }),
        box(["mass-cistern", "cistern", "vitreous-china-cistern", 0.3, "detail"], [0.075, 0.075, 0.075], [0.08, 0.18, 0], { member: "cistern", part: "mass" }),
        box(["mass-lid", "lid", "duroplast-seat", 0.3, "detail"], [0.075, 0.075, 0.075], [0.24, 0.0065, 0], { member: "lid", part: "mass" }),
        box(["mass-pallet", "pallet", "spruce-pallet", 0.3, "detail"], [0.075, 0.075, 0.075], [0.4, 0.072, 0], { member: "pallet", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
