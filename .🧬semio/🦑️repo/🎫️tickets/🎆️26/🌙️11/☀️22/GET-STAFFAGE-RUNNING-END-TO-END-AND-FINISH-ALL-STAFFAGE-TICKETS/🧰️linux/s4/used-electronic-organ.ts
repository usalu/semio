/** 🎹️ `used-electronic-organ`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-electronic-organ.ts <asset document> [--force] */
import { type Script, box, ext, main, of } from "./lib.ts";

const cube = [0.075, 0.075, 0.075] as const;
const pair = { repeat: { count: 2, translation: [0, 0, 0.685] } };
const faces = ["+u", "-u", "+v", "-v", "+w", "-w"];

const script: Script = {
  id: "used-electronic-organ",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: between two 75 mm cheeks of their true side outline the cabinet is one prism (lid, console, the closed key cover as the slope from the lid to the console front, knee panel and bottom merged), so the console still overhangs the knee space; the pedalboard is one block in the knee space, the swell shoe a 75 mm block beside it, the dolly one slab from the floor to the cabinet and the bench a top on four 75 mm legs that keep their outer faces; manuals, panels and castors are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: zwischen zwei 75-mm-Wangen ihres wahren Seitenumrisses ist das Gehäuse ein Prisma (Deckel, Spieltisch, die geschlossene Tastenklappe als Schräge vom Deckel zur Spieltischfront, Knieblende und Boden verschmolzen), sodass der Spieltisch weiter über den Knieraum ragt; das Pedal ist ein Block im Knieraum, der Schweller ein 75-mm-Block daneben, der Rollwagen eine Platte vom Boden bis zum Gehäuse und die Bank eine Platte auf vier 75-mm-Beinen, die ihre Außenflächen behalten; Manuale, Blenden und Rollen liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: the body solids in their regions by reference, the back as its declared 3 mm sheet, the knee grille, the keys and the two control panels as shells of their layer on the faces of the prism, the hidden amplifier as a block inside and the plume patch on the back.", de: "Thermik: die Körper in ihren Regionen als Verweis, die Rückwand als erklärte 3-mm-Platte, das Kniegitter, die Tasten und die zwei Bedienfelder als Schalen ihrer Schicht auf den Flächen des Prismas, der verdeckte Verstärker als Block im Inneren und die Auftriebsfläche an der Rückwand." },
        { en: "Structure: the members by reference, the cabinet spanning key cover and key, four 75 mm joints, eight 75 mm supports under dolly and bench and three mass markers.", de: "Tragwerk: die Stäbe als Verweis, das Gehäuse überspannt Tastenklappe und Taste, vier 75-mm-Gelenke, acht 75-mm-Auflager unter Rollwagen und Bank und drei Massenmarken." },
      ],
      body: [
        ext(["cabinet", "cabinet", "veneer-aged", 0.45], "xy", [[0, 0.275], [0.983, 0.275], [0.983, 0.03], [0.735, -0.275], [0.6, -0.275], [0.6, 0.031], [0, 0.031]], -0.475, 0.475),
        ext(["cheeks", "cabinet", "veneer-aged", 0.4], "xy", [[0, 0.275], [0, -0.255], [0.05, -0.255], [0.09, -0.215], [0.6, -0.215], [0.6, -0.275], [0.735, -0.275], [0.983, 0.03], [0.983, 0.275]], -0.55, -0.475, { repeat: { count: 2, translation: [0, 0, 1.025] } }),
        box(["pedalboard", "pedalboard", "pedal-beech", 0.5], [0.25, 0.085, 0.5], [0.125, 0.0425, -0.17]),
        box(["swell-shoe", "swell-pedal", "pedal-sharp", 0.2], [0.29, 0.075, 0.095], [0.015, 0.0375, 0.2025]),
        box(["dolly", "dolly", "dolly-beech", 0.6], [0.45, 0.125, 0.6], [0.225, -0.0485, 0]),
        box(["bench-legs-back", "bench", "bench-beech", 0.4], [0.45, 0.075, 0.075], [0.225, 0.1075, -0.3725], pair),
        box(["bench-legs-front", "bench", "bench-beech", 0.4], [0.45, 0.075, 0.075], [0.225, -0.1075, -0.3725], pair),
        box(["bench-top", "bench", "bench-vinyl", 0.4], [0.125, 0.32, 0.8], [0.5125, 0, -0.03]),
      ],
      thermal: [
        of("th-cabinet", "cabinet", { region: "cabinet-veneer" }),
        of("th-pedalboard", "pedalboard", { region: "pedals" }),
        of("th-swell-shoe", "swell-shoe", { region: "pedals" }),
        of("th-dolly", "dolly", { region: "dolly" }),
        of("th-cheeks", "cheeks", { region: "cabinet-veneer" }),
        of("th-bench-legs-back", "bench-legs-back", { region: "bench" }),
        of("th-bench-legs-front", "bench-legs-front", { region: "bench" }),
        of("th-bench-top", "bench-top", { region: "bench" }),
        box(["th-back-panel", "cabinet", "hardboard-aged", 0.3], [0.983, 0.003, 1.1], [0.4915, 0.2765, 0], { region: "back-panel", sheet: true }),
        box(["th-knee-panel", "cabinet", "grille-cloth", 0.3], [0.582, 0.03, 1.04], [0.309, 0.046, 0], { region: "grille-panel", layer: "cloth-on-board" }),
        box(["th-keys", "cabinet", "key-acrylic-yellowed", 0.85, "detail"], [0.085, 0.075, 0.611], [0.6625, -0.2375, -0.0825], { region: "keys", layer: "key-shell" }),
        box(["th-panel-right", "cabinet", "panel-faded", 0.62], [0.07, 0.075, 0.132], [0.655, -0.2375, -0.454], { region: "control-panels", layer: "printed-aluminium" }),
        box(["th-panel-left", "cabinet", "panel-faded", 0.62], [0.07, 0.075, 0.297], [0.655, -0.2375, 0.3715], { region: "control-panels", layer: "printed-aluminium" }),
        box(["th-amplifier", "cabinet", "amp-aluminium", 0.15, "detail"], [0.18, 0.17, 0.39], [0.48, 0.175, 0], { region: "amplifier" }),
        box(["th-rear-plume", "cabinet", "hardboard-aged", 0.3, "texture"], [0.14, 0.003, 0.51], [0.56, 0.2795, 0], { region: "back-panel", tags: ["rear-plume"], sectionFaces: faces }),
      ],
      structure: [
        of("member-cabinet", "cabinet", { member: "cabinet", part: "member", members: ["key-cover", "lower-key"] }),
        of("member-pedalboard", "pedalboard", { member: "pedalboard", part: "member" }),
        of("member-swell-shoe", "swell-shoe", { member: "swell-pedal", part: "member" }),
        of("member-dolly", "dolly", { member: "dolly", part: "member" }),
        of("member-cheeks", "cheeks", { member: "cabinet", part: "member" }),
        of("member-bench-legs-back", "bench-legs-back", { member: "bench", part: "member" }),
        of("member-bench-legs-front", "bench-legs-front", { member: "bench", part: "member" }),
        of("member-bench-top", "bench-top", { member: "bench", part: "member" }),
        box(["joint-cover-hinge", "key-cover", "castor-steel", 0.4, "detail"], [0.075, 0.075, 1.1], [0.0375, -0.0375, 0], { member: "key-cover", part: "joint" }),
        box(["joint-pedal-hinge", "pedalboard", "castor-steel", 0.4, "detail"], [0.075, 0.085, 0.5], [0.0375, 0.0425, -0.17], { member: "pedalboard", part: "joint" }),
        box(["joint-swell-pivot", "swell-pedal", "castor-steel", 0.4, "detail"], cube, [0, 0.0375, 0.2025], { member: "swell-pedal", part: "joint" }),
        box(["joint-key-pivot", "lower-key", "castor-steel", 0.4, "detail"], cube, [0.0375, -0.0375, -0.118], { member: "lower-key", part: "joint" }),
        box(["support-castor-back-right", "dolly", "castor-rubber", 0.2, "ground"], cube, [0.045, -0.0735, -0.25], { member: "dolly", part: "support", anchor: "castor-back-right" }),
        box(["support-castor-back-left", "dolly", "castor-rubber", 0.2, "ground"], cube, [0.045, -0.0735, 0.25], { member: "dolly", part: "support", anchor: "castor-back-left" }),
        box(["support-castor-front-right", "dolly", "castor-rubber", 0.2, "ground"], cube, [0.405, -0.0735, -0.25], { member: "dolly", part: "support", anchor: "castor-front-right" }),
        box(["support-castor-front-left", "dolly", "castor-rubber", 0.2, "ground"], cube, [0.405, -0.0735, 0.25], { member: "dolly", part: "support", anchor: "castor-front-left" }),
        box(["support-bench-foot-back-right", "bench", "bench-beech", 0.2, "ground"], cube, [0.0375, 0.1075, -0.3725], { member: "bench", part: "support", anchor: "bench-foot-back-right" }),
        box(["support-bench-foot-back-left", "bench", "bench-beech", 0.2, "ground"], cube, [0.0375, 0.1075, 0.3125], { member: "bench", part: "support", anchor: "bench-foot-back-left" }),
        box(["support-bench-foot-front-right", "bench", "bench-beech", 0.2, "ground"], cube, [0.0375, -0.1075, -0.3725], { member: "bench", part: "support", anchor: "bench-foot-front-right" }),
        box(["support-bench-foot-front-left", "bench", "bench-beech", 0.2, "ground"], cube, [0.0375, -0.1075, 0.3125], { member: "bench", part: "support", anchor: "bench-foot-front-left" }),
        box(["mass-cabinet", "cabinet", "veneer-aged", 0.3, "detail"], [0.075, 0.075, 0.075], [0.45, 0.1, 0], { member: "cabinet", part: "mass" }),
        box(["mass-dolly", "dolly", "dolly-beech", 0.3, "detail"], [0.075, 0.075, 0.075], [0.225, -0.05, 0], { member: "dolly", part: "mass" }),
        box(["mass-bench", "bench", "bench-beech", 0.3, "detail"], [0.075, 0.075, 0.075], [0.45, 0, -0.03], { member: "bench", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
