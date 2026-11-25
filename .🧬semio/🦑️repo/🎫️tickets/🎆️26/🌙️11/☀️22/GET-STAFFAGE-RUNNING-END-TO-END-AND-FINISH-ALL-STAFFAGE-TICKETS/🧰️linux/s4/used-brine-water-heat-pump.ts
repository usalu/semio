/** 🌡️ `used-brine-water-heat-pump`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-brine-water-heat-pump.ts <asset document> [--force] */
import { type Script, box, main, of, rev } from "./lib.ts";

const both = (step: number) => ({ repeat: { count: 2, translation: [0, 0, step] } });
const cube = [0.075, 0.075, 0.075] as const;

const script: Script = {
  id: "used-brine-water-heat-pump",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the pallet as one 144 mm slab, the unit as its 85 mm plinth under the open cabinet, whose panels stay declared 21 mm sheets (two sides, back, top), so its inside shows: the compressor drum on the plinth, evaporator and condenser as boxes that stand on it and the control box under the top; the two hose bundles are 75 mm stubs on the top that keep the height; chocks, brackets, risers and pumps are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: die Palette als eine 144-mm-Platte, das Gerät als sein 85-mm-Sockel unter dem offenen Schrank, dessen Bleche erklärte 21-mm-Platten bleiben (zwei Seiten, Rückwand, Deckel), sodass sein Inneres sich zeigt: die Verdichtertrommel auf dem Sockel, Verdampfer und Verflüssiger als Kästen, die auf ihm stehen, und der Schaltkasten unter dem Deckel; die zwei Schlauchbündel sind 75-mm-Stutzen auf dem Deckel, die die Höhe halten; Keile, Halter, Steigleitungen und Pumpen liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: pallet, plinth and hose stubs by reference; the hidden contents as solids of their own; the casing as its 1 mm steel sheets on its 20 mm linings, all declared; the two pumps as drums of their own and a 75 mm plume patch on the compressor.", de: "Thermik: Palette, Sockel und Schlauchstutzen als Verweis; der verdeckte Inhalt als eigene Körper; das Gehäuse als seine 1-mm-Stahlbleche auf seinen 20-mm-Auskleidungen, alle erklärt; die zwei Pumpen als eigene Trommeln und eine 75-mm-Auftriebsfläche auf dem Verdichter." },
        { en: "Structure: slab, plinth and panels by reference, three 100 mm runner supports, four 75 mm foot pads and two mass markers.", de: "Tragwerk: Platte, Sockel und Bleche als Verweis, drei 100-mm-Kufenauflager, vier 75-mm-Fußplatten und zwei Massenmarken." },
      ],
      body: [
        box(["pallet-slab", "pallet", "softwood", 0.55], [0.8, 0.144, 0.6], [0, 0.072, 0]),
        box(["plinth", "unit", "galvanised-steel", 0.7], [0.7, 0.085, 0.6], [0, 0.0425, 0]),
        box(["sides", "unit", "casing-panel", 0.86], [0.7, 1.065, 0.021], [0, 0.6175, -0.2895], { sheet: true, ...both(0.579) }),
        box(["back", "unit", "casing-panel", 0.8], [0.021, 1.044, 0.558], [-0.3395, 0.607, 0], { sheet: true }),
        box(["top", "unit", "casing-panel", 0.9], [0.7, 0.021, 0.558], [0, 1.1395, 0], { sheet: true }),
        rev(["compressor", "unit", "compressor-steel", 0.12], [0, 1, 0], [0.1, 0, -0.129], [[0.085, 0], [0.085, 0.12], [0.521, 0.12], [0.521, 0]]),
        box(["evaporator", "unit", "armaflex", 0.15], [0.2, 0.581, 0.1], [-0.2, 0.3755, 0.199]),
        box(["condenser", "unit", "stainless-steel", 0.65], [0.16, 0.561, 0.1], [-0.2, 0.3655, -0.199]),
        box(["control-box", "unit", "polymer", 0.8], [0.1, 0.143, 0.498], [0.27, 1.0575, 0]),
        box(["hoses-heating", "unit", "stainless-hose", 0.5], [0.17, 0.126, 0.075], [-0.2, 1.213, -0.199]),
        box(["hoses-brine", "unit", "armaflex", 0.2], [0.205, 0.126, 0.075], [-0.2005, 1.213, 0.1995]),
      ],
      thermal: [
        of("th-pallet-slab", "pallet-slab", { region: "pallet" }),
        of("th-plinth", "plinth", { region: "base-frame" }),
        box(["th-side-sheets", "unit", "painted-steel", 0.86], [0.7, 1.065, 0.001], [0, 0.6175, -0.2995], { sheet: true, ...both(0.599), region: "casing" }),
        box(["th-back-sheet", "unit", "painted-steel", 0.8], [0.001, 1.065, 0.598], [-0.3495, 0.6175, 0], { sheet: true, region: "casing" }),
        box(["th-top-sheet", "unit", "painted-steel", 0.9], [0.7, 0.001, 0.598], [0, 1.1495, 0], { sheet: true, region: "casing" }),
        box(["th-side-linings", "unit", "acoustic-foam", 0.3], [0.7, 1.065, 0.02], [0, 0.6175, -0.289], { sheet: true, ...both(0.578), region: "lining" }),
        box(["th-back-lining", "unit", "acoustic-foam", 0.3], [0.02, 1.044, 0.558], [-0.339, 0.607, 0], { sheet: true, region: "lining" }),
        box(["th-top-lining", "unit", "acoustic-foam", 0.3], [0.7, 0.02, 0.558], [0, 1.139, 0], { sheet: true, region: "lining" }),
        of("th-hoses-heating", "hoses-heating", { region: "heating-lines" }),
        of("th-hoses-brine", "hoses-brine", { region: "brine-lines" }),
        rev(["th-compressor", "unit", "compressor-steel", 0.12], [0, 1, 0], [0.1, 0, -0.129], [[0.121, 0], [0.121, 0.12], [0.521, 0.12], [0.521, 0]], { region: "compressor" }),
        box(["th-evaporator", "unit", "armaflex", 0.15], [0.2, 0.52, 0.1], [-0.2, 0.406, 0.199], { region: "evaporator" }),
        box(["th-condenser", "unit", "stainless-steel", 0.65], [0.16, 0.46, 0.1], [-0.2, 0.416, -0.199], { region: "condenser" }),
        rev(["th-pumps", "unit", "cast-iron", 0.35], [1, 0, 0], [0, 0.846, -0.199], [[-0.185, 0], [-0.185, 0.05], [0.06, 0.05], [0.06, 0]], { repeat: { count: 2, translation: [0, 0, 0.398] }, region: "pumps" }),
        box(["th-control-box", "unit", "polymer", 0.8], [0.1, 0.143, 0.498], [0.27, 1.0575, 0], { region: "controls" }),
        rev(["th-compressor-plume", "unit", "compressor-steel", 0.5, "detail"], [0, 1, 0], [0.1, 0, -0.129], [[0.521, 0], [0.521, 0.06], [0.596, 0.06], [0.596, 0]], { region: "compressor", tags: ["compressor-plume"] }),
      ],
      structure: [
        of("member-pallet-slab", "pallet-slab", { member: "pallet", part: "member" }),
        of("member-frame", "plinth", { member: "unit", part: "member" }),
        of("member-sides", "sides", { member: "unit", part: "member" }),
        of("member-back", "back", { member: "unit", part: "member" }),
        of("member-top", "top", { member: "unit", part: "member" }),
        box(["support-runner-0", "pallet", "softwood", 0.3, "ground"], [0.1, 0.1, 0.1], [0, 0.05, -0.25], { member: "pallet", part: "support", anchor: "pallet-runner-0" }),
        box(["support-runner-1", "pallet", "softwood", 0.3, "ground"], [0.1, 0.1, 0.1], [0, 0.05, 0], { member: "pallet", part: "support", anchor: "pallet-runner-1" }),
        box(["support-runner-2", "pallet", "softwood", 0.3, "ground"], [0.1, 0.1, 0.1], [0, 0.05, 0.25], { member: "pallet", part: "support", anchor: "pallet-runner-2" }),
        box(["joint-feet-right", "unit", "rubber", 0.3, "detail"], cube, [-0.3125, 0.0375, -0.2625], { repeat: { count: 2, translation: [0.625, 0, 0] }, member: "unit", part: "joint" }),
        box(["joint-feet-left", "unit", "rubber", 0.3, "detail"], cube, [-0.3125, 0.0375, 0.2625], { repeat: { count: 2, translation: [0.625, 0, 0] }, member: "unit", part: "joint" }),
        box(["mass-unit", "unit", "galvanised-steel", 0.3, "detail"], [0.075, 0.075, 0.075], [0, 0.45, 0], { member: "unit", part: "mass" }),
        box(["mass-pallet", "pallet", "softwood", 0.3, "detail"], [0.075, 0.075, 0.075], [0, 0.072, 0], { member: "pallet", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
