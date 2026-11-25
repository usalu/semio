/** 💧️ `used-dry-rotor-inline-pump`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-dry-rotor-inline-pump.ts <asset document> [--force] */
import { type Script, box, main, of, rev } from "./lib.ts";

const disc = [[0.19, 0], [0.19, 0.1], [0.265, 0.1], [0.265, 0]] as const;

const script: Script = {
  id: "used-dry-rotor-inline-pump",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the pallet as one 144 mm slab; the pump as a stack of drums on its vertical axis: casing (foot, pedestal, volute and necks merged), lantern, motor with its flange as a 75 mm collar, fan cowl; each flange with its blanking plate is one 75 mm disc on the pipe axis, the terminal box keeps its 120 x 120 x 80 mm; chocks and lugs are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: die Palette als eine 144-mm-Platte; die Pumpe als Stapel von Trommeln auf ihrer senkrechten Achse: Gehäuse (Fuß, Sockel, Spirale und Stutzen verschmolzen), Laterne, Motor mit seinem Flansch als 75-mm-Kragen, Lüfterhaube; jeder Flansch mit seiner Blindplatte ist eine 75-mm-Scheibe auf der Rohrachse, der Klemmenkasten behält seine 120 x 120 x 80 mm; Keile und Ösen liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: the body solids in their regions by reference, the blanking plates as shells of their layer at true 30 mm on the flange discs and a 75 mm fan intake disc over the cowl.", de: "Thermik: die Körper in ihren Regionen als Verweis, die Blindplatten als Schalen ihrer Schicht in wahren 30 mm auf den Flanschscheiben und eine 75-mm-Ansaugscheibe über der Haube." },
        { en: "Structure: the members by reference, the blanking plates as 75 mm discs on their own bone, one bearing pad under the casing and two mass markers.", de: "Tragwerk: die Stäbe als Verweis, die Blindplatten als 75-mm-Scheiben an ihrem eigenen Knochen, eine Lagerplatte unter dem Gehäuse und zwei Massenmarken." },
      ],
      body: [
        box(["pallet-slab", "pallet", "softwood", 0.6], [0.8, 0.144, 0.6], [0.4, 0.072, 0]),
        rev(["casing", "pump", "cast-iron", 0.32], [0, 1, 0], [0.4, 0, 0], [[0, 0], [0, 0.138], [0.248, 0.138], [0.248, 0]]),
        rev(["flanges", "pump", "cast-iron", 0.28], [1, 0, 0], [0, 0.16, 0], disc, { repeat: { count: 2, translation: [0.345, 0, 0] } }),
        rev(["lantern", "pump", "cast-iron", 0.36], [0, 1, 0], [0.4, 0, 0], [[0.248, 0], [0.248, 0.085], [0.362, 0.085], [0.362, 0]]),
        rev(["motor", "pump", "painted-aluminium", 0.5], [0, 1, 0], [0.4, 0, 0], [[0.362, 0], [0.362, 0.125], [0.437, 0.125], [0.437, 0.1], [0.656, 0.1], [0.656, 0]]),
        rev(["fan-cowl", "pump", "steel-sheet", 0.42], [0, 1, 0], [0.4, 0, 0], [[0.656, 0], [0.656, 0.104], [0.776, 0.104], [0.776, 0]]),
        box(["terminal-box", "pump", "painted-aluminium", 0.46], [0.12, 0.12, 0.08], [0.4, 0.516, -0.125]),
      ],
      thermal: [
        of("th-pallet-slab", "pallet-slab", { region: "pallet" }),
        of("th-casing", "casing", { region: "casing" }),
        of("th-flanges", "flanges", { region: "casing" }),
        of("th-lantern", "lantern", { region: "lantern" }),
        of("th-motor", "motor", { region: "motor" }),
        of("th-fan-cowl", "fan-cowl", { region: "fan-cowl" }),
        of("th-terminal-box", "terminal-box", { region: "terminal-box" }),
        rev(["th-blanks", "blanks", "steel", 0.2], [1, 0, 0], [0, 0.16, 0], [[0.19, 0], [0.19, 0.1], [0.22, 0.1], [0.22, 0]], { repeat: { count: 2, translation: [0.39, 0, 0] }, region: "blanking-plates", layer: "plate" }),
        rev(["th-fan-intake", "pump", "steel-sheet", 0.3, "detail"], [0, 1, 0], [0.4, 0, 0], [[0.776, 0], [0.776, 0.09], [0.851, 0.09], [0.851, 0]], { region: "fan-cowl", tags: ["motor-fan"] }),
      ],
      structure: [
        of("member-pallet-slab", "pallet-slab", { member: "pallet", part: "member" }),
        of("member-casing", "casing", { member: "pump", part: "member" }),
        of("member-flanges", "flanges", { member: "pump", part: "member" }),
        of("member-lantern", "lantern", { member: "pump", part: "member" }),
        of("member-motor", "motor", { member: "pump", part: "member" }),
        rev(["member-blanks", "blanks", "steel", 0.2], [1, 0, 0], [0, 0.16, 0], disc, { repeat: { count: 2, translation: [0.345, 0, 0] }, member: "blanks", part: "member" }),
        box(["joint-foot", "pump", "cast-iron", 0.5, "detail"], [0.22, 0.075, 0.18], [0.4, 0.0375, 0], { member: "pump", part: "joint" }),
        box(["mass-pump", "pump", "cast-iron", 0.5, "detail"], [0.075, 0.075, 0.075], [0.4, 0.35, 0], { member: "pump", part: "mass" }),
        box(["mass-pallet", "pallet", "softwood", 0.5, "detail"], [0.075, 0.075, 0.075], [0.4, 0.072, 0], { member: "pallet", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
