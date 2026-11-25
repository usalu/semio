/** 🎼️ `used-harmonium`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-harmonium.ts <asset document> [--force] */
import { type Script, box, ext, main, of } from "./lib.ts";

const cube = [0.075, 0.075, 0.075] as const;

const script: Script = {
  id: "used-harmonium",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: between two 75 mm side panels of their stepped outline the case is one prism of its side outline (a 100 mm back, reservoir, action, stop rail, a 75 mm bottom and a 75 mm key bed that projects over the knee space; the space under the reservoir stays open as in the instrument), the keys with both cheek blocks one 80 mm band on the bed, the kneeboard a 75 mm board hanging from it, lid, crest, desk and both treadles 75 mm boards on their hinges with their outer faces in place; exhausters, straps, fall, knobs and handles are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: zwischen zwei 75-mm-Seitenwänden ihres abgetreppten Umrisses ist das Gehäuse ein Prisma seines Seitenumrisses (eine 100-mm-Rückwand, Magazinbalg, Spielwerk, Registerleiste, ein 75-mm-Boden und ein 75-mm-Tastenbett, das über den Knieraum ragt; der Raum unter dem Magazinbalg bleibt offen wie im Instrument), die Tasten mit beiden Backenklötzen ein 80-mm-Band auf dem Bett, die Knieleiste ein 75-mm-Brett darunter, Deckel, Aufsatz, Notenpult und beide Tretschemel 75-mm-Bretter an ihren Scharnieren mit den Außenflächen am Ort; Schöpfbälge, Riemen, Klappe, Knöpfe und Griffe liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: the body solids in their regions by reference, the bellows as one 75 mm slab under the reservoir where the exhausters hang and two 75 mm exhaust patches.", de: "Thermik: die Körper in ihren Regionen als Verweis, die Bälge als eine 75-mm-Platte unter dem Magazinbalg, wo die Schöpfbälge hängen, und zwei 75-mm-Abluftflächen." },
        { en: "Structure: the members by reference, the case also holding exhausters and straps, the key band as the member of the fall, three 75 mm hinge joints and two mass markers.", de: "Tragwerk: die Stäbe als Verweis, das Gehäuse fasst auch Schöpfbälge und Riemen, das Tastenband als Stab der Klappe, drei 75-mm-Scharniergelenke und zwei Massenmarken." },
      ],
      body: [
        ext(["case", "case", "veneered-softwood", 0.42], "xy", [[0, 0], [0.52, 0], [0.52, 0.075], [0.1, 0.075], [0.1, 0.33], [0.34, 0.33], [0.34, 0.58], [0.525, 0.58], [0.525, 0.655], [0.34, 0.655], [0.34, 0.98], [0, 0.98]], -0.525, 0.525),
        ext(["sides", "case", "veneered-softwood", 0.38], "xy", [[0, 0], [0.52, 0], [0.53, 0.63], [0.53, 0.745], [0.46, 0.765], [0.42, 0.825], [0.42, 0.98], [0, 0.98]], -0.6, -0.525, { repeat: { count: 2, translation: [0, 0, 1.125] } }),
        box(["keys", "case", "celluloid", 0.88], [0.185, 0.08, 1.05], [0.4325, 0.695, 0]),
        box(["kneeboard", "case", "veneered-softwood", 0.4], [0.075, 0.295, 1.05], [0.4075, 0.5075, 0]),
        box(["lid", "lid", "veneered-softwood", 0.4], [0.4, 0.075, 1.2], [0.2, -0.0275, 0]),
        ext(["crest", "case", "veneered-softwood", 0.38], "yz", [[0.98, -0.56], [1.05, -0.56], [1.15, 0], [1.05, 0.56], [0.98, 0.56]], 0, 0.075),
        box(["desk", "desk", "veneered-softwood", 0.44], [0.22, 0.075, 0.7], [0.11, -0.0475, 0]),
        box(["treadle-left", "treadle-l", "beech", 0.5], [0.34, 0.075, 0.16], [0.17, -0.0375, 0.105]),
        box(["treadle-right", "treadle-r", "beech", 0.5], [0.34, 0.075, 0.16], [0.17, -0.0375, -0.105]),
      ],
      thermal: [
        of("th-case", "case", { region: "case" }),
        of("th-sides", "sides", { region: "case" }),
        of("th-keys", "keys", { region: "keys" }),
        of("th-kneeboard", "kneeboard", { region: "case" }),
        of("th-lid", "lid", { region: "case" }),
        of("th-crest", "crest", { region: "case" }),
        of("th-desk", "desk", { region: "case" }),
        of("th-treadle-left", "treadle-left", { region: "treadles" }),
        of("th-treadle-right", "treadle-right", { region: "treadles" }),
        box(["th-reservoir", "case", "rubber-cloth", 0.2], [0.24, 0.075, 1.05], [0.22, 0.2925, 0], { region: "bellows" }),
        box(["th-exhaust-left", "exhauster-l", "rubber-cloth", 0.2, "detail"], cube, [0.11, 0.0375, 0.28], { region: "bellows", tags: ["exhaust-left"] }),
        box(["th-exhaust-right", "exhauster-r", "rubber-cloth", 0.2, "detail"], cube, [0.11, 0.0375, -0.28], { region: "bellows", tags: ["exhaust-right"] }),
      ],
      structure: [
        of("member-case", "case", { member: "case", part: "member", members: ["exhauster-l", "exhauster-r", "strap-l", "strap-r"] }),
        of("member-sides", "sides", { member: "case", part: "member" }),
        of("member-keys", "keys", { member: "fall", part: "member" }),
        of("member-kneeboard", "kneeboard", { member: "case", part: "member" }),
        of("member-lid", "lid", { member: "lid", part: "member" }),
        of("member-crest", "crest", { member: "case", part: "member" }),
        of("member-desk", "desk", { member: "desk", part: "member" }),
        of("member-treadle-left", "treadle-left", { member: "treadle-l", part: "member" }),
        of("member-treadle-right", "treadle-right", { member: "treadle-r", part: "member" }),
        box(["joint-treadle-hinge-left", "treadle-l", "iron", 0.3, "detail"], cube, [0.0375, -0.0375, 0.105], { member: "treadle-l", part: "joint" }),
        box(["joint-treadle-hinge-right", "treadle-r", "iron", 0.3, "detail"], cube, [0.0375, -0.0375, -0.105], { member: "treadle-r", part: "joint" }),
        box(["joint-lid-hinge", "lid", "iron", 0.3, "detail"], [0.075, 0.075, 1.2], [0.0375, -0.0275, 0], { member: "lid", part: "joint" }),
        box(["mass-case", "case", "veneered-softwood", 0.3, "detail"], [0.075, 0.075, 0.075], [0.2, 0.5, 0], { member: "case", part: "mass" }),
        box(["mass-lid", "lid", "veneered-softwood", 0.3, "detail"], [0.075, 0.075, 0.075], [0.2, -0.0275, 0], { member: "lid", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
