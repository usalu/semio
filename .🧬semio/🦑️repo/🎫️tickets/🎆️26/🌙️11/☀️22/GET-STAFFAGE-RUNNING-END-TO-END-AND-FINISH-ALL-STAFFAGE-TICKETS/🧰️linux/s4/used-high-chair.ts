/** 🪑️ `used-high-chair`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-high-chair.ts <asset document> [--force] */
import { type Script, box, ext, main, of } from "./lib.ts";

const pair = { repeat: { count: 2, translation: [0, 0, 0.389] } };
const cube = [0.075, 0.075, 0.075] as const;
const upright = [[0.04, -0.038], [0.8181, -0.038], [0.8457, 0.038], [0.04, 0.038]] as const;

const script: Script = {
  id: "used-high-chair",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: each side frame keeps its runner and its leaning upright with the level top cut, thickened inwards from 30 to 75 mm with the outer faces in place and the runner raised to 75 mm; backrest band, seat plate, footplate and stretcher are 75 mm bars between the frames with their top faces at the true heights; the bow is its rear bar and the tray one 75 mm slab in front of it; the tile is its declared 12 mm sheet.", de: "Körper für das 75-mm-Merkmal: jeder Seitenrahmen behält Kufe und geneigten Holm mit dem waagrechten Kopfschnitt, von 30 auf 75 mm nach innen verdickt mit den Außenflächen am Ort und der Kufe auf 75 mm erhöht; Rückenband, Sitzplatte, Fußplatte und Querstab sind 75-mm-Balken zwischen den Rahmen mit ihren Oberseiten auf wahrer Höhe; der Bügel ist sein hinterer Stab und das Tablett eine 75-mm-Platte davor; die Fliese ist ihre erklärte 12-mm-Platte." },
        { en: "Thermal: one surface per body solid by reference in its region.", de: "Thermik: eine Oberfläche je Körper als Verweis in seiner Region." },
        { en: "Structure: the members by reference, the twelve joints as 75 mm blocks in the frames and ten mass markers.", de: "Tragwerk: die Stäbe als Verweis, die zwölf Verbindungen als 75-mm-Blöcke in den Rahmen und zehn Massenmarken." },
      ],
      body: [
        box(["tile", "mat", "eva-foam", 0.42], [0.6, 0.012, 0.6], [0.306, 0.006, 0.006], { sheet: true }),
        box(["runner-r", "runner-r", "beech-lacquered", 0.68], [0.49, 0.075, 0.075], [0.245, 0.0295, -0.1945]),
        box(["runner-l", "runner-l", "beech-lacquered", 0.68], [0.49, 0.075, 0.075], [0.245, 0.0295, 0.1945]),
        ext(["upright-r", "upright-r", "beech-lacquered", 0.74], "xy", upright, -0.232, -0.157),
        ext(["upright-l", "upright-l", "beech-lacquered", 0.74], "xy", upright, 0.157, 0.232),
        box(["backrest-band", "backrest", "beech-lacquered", 0.72], [0.236, 0.075, 0.314], [0.11, -0.0075, 0]),
        box(["stretcher", "stretcher", "beech-lacquered", 0.66], [0.075, 0.075, 0.314], [0.0125, 0, 0]),
        box(["seat-plate", "seat", "beech-plywood", 0.62], [0.2, 0.075, 0.314], [0.1, -0.0195, 0]),
        box(["footplate", "footplate", "beech-plywood", 0.6], [0.22, 0.075, 0.314], [0.11, -0.0195, 0]),
        box(["bow", "bow", "beech-lacquered", 0.7], [0.09, 0.075, 0.38], [0.045, 0.0225, 0]),
        box(["tray", "bow", "polypropylene", 0.9], [0.24, 0.075, 0.38], [0.21, 0.0225, 0]),
      ],
      thermal: [
        of("th-tile", "tile", { region: "mat" }),
        of("th-runner-r", "runner-r", { region: "frame" }),
        of("th-runner-l", "runner-l", { region: "frame" }),
        of("th-upright-r", "upright-r", { region: "frame" }),
        of("th-upright-l", "upright-l", { region: "frame" }),
        of("th-backrest-band", "backrest-band", { region: "frame" }),
        of("th-stretcher", "stretcher", { region: "frame" }),
        of("th-seat-plate", "seat-plate", { region: "seat-plate" }),
        of("th-footplate", "footplate", { region: "footplate" }),
        of("th-bow", "bow", { region: "bow" }),
        of("th-tray", "tray", { region: "tray" }),
      ],
      structure: [
        of("m-tile", "tile", { member: "mat", part: "member" }),
        of("m-runner-r", "runner-r", { member: "runner-r", part: "member" }),
        of("m-runner-l", "runner-l", { member: "runner-l", part: "member" }),
        of("m-upright-r", "upright-r", { member: "upright-r", part: "member" }),
        of("m-upright-l", "upright-l", { member: "upright-l", part: "member" }),
        of("m-backrest-band", "backrest-band", { member: "backrest", part: "member" }),
        of("m-stretcher", "stretcher", { member: "stretcher", part: "member" }),
        of("m-seat-plate", "seat-plate", { member: "seat", part: "member" }),
        of("m-footplate", "footplate", { member: "footplate", part: "member" }),
        of("m-bow", "bow", { member: "bow", part: "member" }),
        box(["j-foot-r", "upright-r", "beech-lacquered", 0.3, "detail"], cube, [0.0775, 0, -0.1945], { member: "upright-r", part: "joint" }),
        box(["j-foot-l", "upright-l", "beech-lacquered", 0.3, "detail"], cube, [0.0775, 0, 0.1945], { member: "upright-l", part: "joint" }),
        box(["j-seat", "seat", "beech-lacquered", 0.3, "detail"], cube, [0.0375, -0.0195, -0.1945], { ...pair, member: "seat", part: "joint" }),
        box(["j-foot-plate", "footplate", "beech-lacquered", 0.3, "detail"], cube, [0.047, -0.0195, -0.1945], { ...pair, member: "footplate", part: "joint" }),
        box(["j-back", "backrest", "beech-lacquered", 0.3, "detail"], [0.2, 0.075, 0.075], [0.111, -0.0075, -0.1945], { ...pair, member: "backrest", part: "joint" }),
        box(["j-bow", "bow", "beech-lacquered", 0.3, "detail"], cube, [0.0375, 0.0225, -0.1945], { ...pair, member: "bow", part: "joint" }),
        box(["j-rod", "stretcher", "beech-lacquered", 0.3, "detail"], cube, [0.0125, 0, -0.1945], { ...pair, member: "stretcher", part: "joint" }),
        box(["mass-runner-r", "runner-r", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.243, 0.0295, -0.1945], { member: "runner-r", part: "mass" }),
        box(["mass-upright-r", "upright-r", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.441, 0, -0.1945], { member: "upright-r", part: "mass" }),
        box(["mass-runner-l", "runner-l", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.247, 0.0295, 0.1945], { member: "runner-l", part: "mass" }),
        box(["mass-upright-l", "upright-l", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.441, 0, 0.1945], { member: "upright-l", part: "mass" }),
        box(["mass-backrest", "backrest", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.112, -0.0075, 0], { member: "backrest", part: "mass" }),
        box(["mass-stretcher", "stretcher", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.0125, 0, 0], { member: "stretcher", part: "mass" }),
        box(["mass-seat", "seat", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.1, -0.0195, 0], { member: "seat", part: "mass" }),
        box(["mass-footplate", "footplate", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.11, -0.0195, 0], { member: "footplate", part: "mass" }),
        box(["mass-bow", "bow", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.045, 0.0225, 0], { member: "bow", part: "mass" }),
        box(["mass-mat", "mat", "beech-lacquered", 0.4, "detail"], [0.075, 0.075, 0.075], [0.306, 0.0375, 0.006], { member: "mat", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
