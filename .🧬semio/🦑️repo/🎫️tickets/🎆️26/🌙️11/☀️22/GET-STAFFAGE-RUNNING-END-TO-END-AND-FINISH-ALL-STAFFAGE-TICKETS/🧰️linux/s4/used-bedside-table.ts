/** 🛏️ `used-bedside-table`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-bedside-table.ts <asset document> [--force] */
import { type Script, box, ext, main, of, rect } from "./lib.ts";

const pair = { repeat: { count: 2, translation: [0, 0, -0.311] } };

const script: Script = {
  id: "used-bedside-table",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the cabinet is its frame with the open shelf as a through opening (top and drawer merged into the 145 mm above it, the 16 mm boards judged by the extents of the frame) before the back as its declared 3 mm sheet, on four 75 mm posts that keep the outer faces of the 36 mm legs, on the blanket as its declared 18 mm sheet.", de: "Körper für das 75-mm-Merkmal: der Korpus ist sein Rahmen mit dem offenen Fach als Durchbruch (Platte und Schublade in den 145 mm darüber verschmolzen, die 16-mm-Platten nach den Maßen des Rahmens beurteilt) vor der Rückwand als erklärter 3-mm-Platte, auf vier 75-mm-Pfosten, die die Außenflächen der 36-mm-Beine behalten, auf der Decke als erklärter 18-mm-Platte." },
        { en: "Thermal: blanket and posts by reference, the cabinet as region solids: a 75 mm top, the lower carcass as its frame around the open shelf, a 75 mm drawer block and the back as its declared 3 mm sheet.", de: "Thermik: Decke und Pfosten als Verweis, der Korpus als Regionskörper: eine 75-mm-Platte, der untere Korpus als Rahmen um das offene Fach, ein 75-mm-Schubladenblock und die Rückwand als erklärte 3-mm-Platte." },
        { en: "Structure: blanket, frame, back and posts by reference, the drawer as a 75 mm member, two 75 mm leg plates and three mass markers.", de: "Tragwerk: Decke, Rahmen, Rückwand und Pfosten als Verweis, die Schublade als 75-mm-Stab, zwei 75-mm-Beinplatten und drei Massenmarken." },
      ],
      body: [
        box(["blanket", "blanket", "moving-blanket", 0.45], [0.6666, 0.018, 0.75], [0.3333, 0.009, 0], { sheet: true }),
        ext(["cabinet", "carcass", "teak-veneer-chipboard", 0.7], "yz", rect(-0.35, 0, -0.215, 0.215), 0.003, 0.33, { holes: [rect(-0.334, -0.145, -0.199, 0.199)] }),
        box(["back", "carcass", "hardboard", 0.35], [0.003, 0.35, 0.43], [0.0015, -0.175, 0], { sheet: true }),
        box(["legs-rear", "carcass", "beech-stained", 0.55], [0.075, 0.2, 0.075], [0.04, -0.45, 0.1555], pair),
        box(["legs-front", "carcass", "beech-stained", 0.55], [0.075, 0.2, 0.075], [0.29, -0.45, 0.1555], pair),
      ],
      thermal: [
        of("th-blanket", "blanket", { region: "blanket" }),
        of("th-legs-rear", "legs-rear", { region: "legs" }),
        of("th-legs-front", "legs-front", { region: "legs" }),
        box(["th-top", "carcass", "teak-top-faded", 0.84], [0.33, 0.075, 0.43], [0.165, -0.0375, 0], { region: "top" }),
        ext(["th-carcass", "carcass", "teak-veneer-chipboard", 0.7], "yz", rect(-0.35, -0.15, -0.215, 0.215), 0.003, 0.33, { holes: [rect(-0.334, -0.166, -0.199, 0.199)], region: "carcass" }),
        box(["th-drawer", "drawer", "teak-veneer-chipboard", 0.66], [0.315, 0.075, 0.396], [0.1575, 0.0165, 0], { region: "drawer" }),
        box(["th-back", "carcass", "hardboard", 0.35], [0.003, 0.275, 0.43], [0.0015, -0.2125, 0], { region: "back-panel", sheet: true }),
      ],
      structure: [
        of("m-blanket", "blanket", { member: "blanket", part: "member" }),
        of("m-cabinet", "cabinet", { member: "carcass", part: "member" }),
        of("m-back", "back", { member: "carcass", part: "member" }),
        of("m-legs-rear", "legs-rear", { member: "carcass", part: "member" }),
        of("m-legs-front", "legs-front", { member: "carcass", part: "member" }),
        box(["m-drawer", "drawer", "teak-veneer-chipboard", 0.66], [0.315, 0.075, 0.396], [0.1575, 0.0165, 0], { member: "drawer", part: "member" }),
        box(["j-leg-plates", "carcass", "teak-veneer-chipboard", 0.3, "detail"], [0.075, 0.075, 0.386], [0.04, -0.3875, 0], { repeat: { count: 2, translation: [0.25, 0, 0] }, member: "carcass", part: "joint" }),
        box(["mass-carcass", "carcass", "teak-veneer-chipboard", 0.3, "detail"], [0.075, 0.075, 0.075], [0.16, -0.175, 0], { member: "carcass", part: "mass" }),
        box(["mass-drawer", "drawer", "pine-drawer", 0.3, "detail"], [0.075, 0.075, 0.075], [0.188, 0.0165, 0], { member: "drawer", part: "mass" }),
        box(["mass-blanket", "blanket", "moving-blanket", 0.3, "detail"], [0.075, 0.075, 0.075], [0.3333, 0.0375, 0], { member: "blanket", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
