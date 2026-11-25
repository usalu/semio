/** 📦️ `used-moving-boxes`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-moving-boxes.ts <asset document> [--force] */
import { type Script, box, main, of } from "./lib.ts";

const script: Script = {
  id: "used-moving-boxes",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: three closed blocks, one per box, so the two joints of the stack still read; the top box takes its tube, double bottom and the four 7 mm flaps into its block (the lid no longer opens at this scale), on the flattened box as its declared 14 mm sheet.", de: "Körper für das 75-mm-Merkmal: drei geschlossene Blöcke, einer je Karton, damit die zwei Fugen des Stapels lesbar bleiben; der oberste Karton nimmt Rohr, Doppelboden und die vier 7-mm-Klappen in seinen Block auf (der Deckel öffnet in diesem Maßstab nicht mehr), auf dem flachgelegten Karton als erklärter 14-mm-Platte." },
        { en: "Thermal: one surface per solid by reference in its region.", de: "Thermik: eine Oberfläche je Körper als Verweis in seiner Region." },
        { en: "Structure: the four solids as members by reference, the top block spanning its four flaps, the glued laps as 75 mm joints and four mass markers.", de: "Tragwerk: die vier Körper als Stäbe per Verweis, der oberste Block überspannt seine vier Klappen, die Klebelaschen als 75-mm-Gelenke und vier Massenmarken." },
      ],
      body: [
        box(["flat-box", "flat-box", "kraft-board-worn", 0.5], [0.96, 0.014, 0.7], [0.48, 0.007, 0], { sheet: true }),
        box(["box-1", "box-1", "kraft-board-worn", 0.6], [0.614, 0.342, 0.344], [0.307, 0.171, 0]),
        box(["box-2", "box-2", "kraft-board", 0.66], [0.614, 0.354, 0.344], [0.307, 0.177, 0]),
        box(["box-3", "box-3", "kraft-board", 0.7], [0.614, 0.354, 0.344], [0.307, 0.177, 0]),
      ],
      thermal: [
        of("th-flat-box", "flat-box", { region: "flat-box" }),
        of("th-box-1", "box-1", { region: "stacked-boxes" }),
        of("th-box-2", "box-2", { region: "stacked-boxes" }),
        of("th-box-3", "box-3", { region: "carried-box" }),
      ],
      structure: [
        of("m-flat-box", "flat-box", { member: "flat-box", part: "member" }),
        of("m-box-1", "box-1", { member: "box-1", part: "member" }),
        of("m-box-2", "box-2", { member: "box-2", part: "member" }),
        of("m-box-3", "box-3", { member: "box-3", part: "member", members: ["flap-r", "flap-l", "flap-f", "flap-b"] }),
        box(["j-lap-box-1", "box-1", "kraft-board-worn", 0.3, "detail"], [0.075, 0.3, 0.075], [0.0445, 0.171, -0.1275], { member: "box-1", part: "joint" }),
        box(["j-lap-box-2", "box-2", "kraft-board", 0.3, "detail"], [0.075, 0.3, 0.075], [0.0445, 0.177, -0.1275], { member: "box-2", part: "joint" }),
        box(["j-lap-box-3", "box-3", "kraft-board", 0.3, "detail"], [0.075, 0.3, 0.075], [0.0445, 0.177, -0.1275], { member: "box-3", part: "joint" }),
        box(["mass-flat-box", "flat-box", "kraft-board-worn", 0.3, "detail"], [0.075, 0.075, 0.075], [0.48, 0.0375, 0], { member: "flat-box", part: "mass" }),
        box(["mass-box-1", "box-1", "kraft-board-worn", 0.3, "detail"], [0.075, 0.075, 0.075], [0.307, 0.16, 0], { member: "box-1", part: "mass" }),
        box(["mass-box-2", "box-2", "kraft-board", 0.3, "detail"], [0.075, 0.075, 0.075], [0.307, 0.166, 0], { member: "box-2", part: "mass" }),
        box(["mass-box-3", "box-3", "kraft-board", 0.3, "detail"], [0.075, 0.075, 0.075], [0.307, 0.112, 0], { member: "box-3", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
