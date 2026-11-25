/** 🧱️ `used-panel-radiator-pallet`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-panel-radiator-pallet.ts <asset document> [--force] */
import { type Script, box, ext, main, of, rect, rev } from "./lib.ts";

const ends = { repeat: { count: 2, translation: [0.68, 0, 0] } };
const bands = { repeat: { count: 2, translation: [0.32, 0, 0] } };

const script: Script = {
  id: "used-panel-radiator-pallet",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the pallet as one 144 mm slab (deck and nine feet merged), the two 80 mm battens, each end frame of the rack as one 75 mm portal with its outer face on the pallet edge, the single radiator and the stack as their boxes and the valve with its head as one 75 mm cylinder; brackets, straps, caps and nipples are below the feature and left out (the brackets were the last 36 mm of the depth).", de: "Körper für das 75-mm-Merkmal: die Palette als eine 144-mm-Platte (Deck und neun Füße verschmolzen), die zwei 80-mm-Kanthölzer, jeder Endrahmen des Gestells als ein 75-mm-Portal mit der Außenfläche an der Palettenkante, der einzelne Heizkörper und der Stapel als ihre Quader und das Ventil mit seinem Kopf als ein 75-mm-Zylinder; Konsolen, Bänder, Kappen und Nippel liegen unter dem Merkmal und entfallen (die Konsolen waren die letzten 36 mm der Tiefe)." },
        { en: "Thermal: the body solids in their regions by reference; convector, valve body, brackets and straps as shells of their layer at true thickness and a 75 mm outlet patch over the radiator.", de: "Thermik: die Körper in ihren Regionen als Verweis; Konvektor, Ventilkörper, Konsolen und Bänder als Schalen ihrer Schicht in wahrer Dicke und eine 75-mm-Auslassfläche über dem Heizkörper." },
        { en: "Structure: the members by reference, the radiator spanning its carrier, two 75 mm pads on the battens and three mass markers.", de: "Tragwerk: die Stäbe als Verweis, der Heizkörper überspannt seinen Träger, zwei 75-mm-Auflagen auf den Kanthölzern und drei Massenmarken." },
      ],
      body: [
        box(["pallet-slab", "pallet", "softwood", 0.63], [1.2, 0.144, 0.8], [0.6, -0.061, 0]),
        box(["battens", "pallet", "softwood", 0.7], [0.08, 0.08, 0.8], [0.22, 0.051, 0], ends),
        ext(["rack", "pallet", "rack-steel", 0.3], "yz", rect(0.011, 0.667, -0.4, 0.4), 0, 0.075, { holes: [rect(0.086, 0.592, -0.325, 0.325)], repeat: { count: 2, translation: [1.125, 0, 0] } }),
        box(["radiator", "radiator", "painted-steel", 0.86], [1, 0.6, 0.1], [0.5, 0, 0]),
        box(["stack", "stack", "painted-steel", 0.86], [1, 0.6, 0.7], [0.5, 0, 0]),
        rev(["valve-head", "radiator", "abs-plastic", 0.9], [1, 0, 0], [0, 0.24, 0], [[1, 0], [1, 0.0375], [1.09, 0.0375], [1.09, 0]]),
      ],
      thermal: [
        of("th-pallet-slab", "pallet-slab", { region: "pallet" }),
        of("th-battens", "battens", { region: "pallet" }),
        of("th-rack", "rack", { region: "rack" }),
        of("th-radiator", "radiator", { region: "radiator" }),
        of("th-stack", "stack", { region: "stack" }),
        of("th-valve-head", "valve-head", { region: "thermostat-head" }),
        box(["th-convector", "radiator", "painted-steel", 0.55], [0.997, 0.581, 0.06], [0.5005, -0.0045, 0], { region: "convector", layer: "fins" }),
        rev(["th-valve-body", "radiator", "brass", 0.35], [1, 0, 0], [0, 0.24, 0], [[1, 0], [1, 0.015], [1.03, 0.015], [1.03, 0]], { region: "valve", layer: "brass" }),
        box(["th-brackets", "radiator", "painted-steel", 0.7], [0.04, 0.16, 0.035], [0.15, -0.19, -0.0675], { repeat: { count: 2, translation: [0.7, 0, 0] }, region: "brackets", layer: "sheet" }),
        box(["th-straps-top", "stack", "polyester", 0.2], [0.05, 0.002, 0.704], [0.355, 0.301, 0], { ...bands, region: "straps", layer: "web" }),
        box(["th-straps-near", "stack", "polyester", 0.2], [0.05, 0.6, 0.002], [0.355, 0, -0.351], { ...bands, region: "straps", layer: "web" }),
        box(["th-convection-outlet", "radiator", "painted-steel", 0.8, "detail"], [0.9, 0.075, 0.075], [0.5, 0.3375, 0], { region: "radiator", tags: ["convection-outlet"] }),
      ],
      structure: [
        of("member-pallet-slab", "pallet-slab", { member: "pallet", part: "member" }),
        of("member-battens", "battens", { member: "pallet", part: "member" }),
        of("member-rack", "rack", { member: "pallet", part: "member" }),
        of("member-stack", "stack", { member: "stack", part: "member" }),
        of("member-radiator", "radiator", { member: "radiator", part: "member", members: ["carrier"] }),
        box(["joint-pads", "pallet", "steel", 0.5, "detail"], [0.08, 0.075, 0.8], [0.22, 0.091, 0], { ...ends, member: "pallet", part: "joint" }),
        box(["mass-stack", "stack", "painted-steel", 0.5, "detail"], [0.075, 0.075, 0.075], [0.5, 0, 0], { member: "stack", part: "mass" }),
        box(["mass-radiator", "radiator", "painted-steel", 0.5, "detail"], [0.075, 0.075, 0.075], [0.5, 0, 0], { member: "radiator", part: "mass" }),
        box(["mass-pallet", "pallet", "softwood", 0.5, "detail"], [0.075, 0.075, 0.075], [0.6, -0.061, 0], { member: "pallet", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
