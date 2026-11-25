/** 🔥️ `used-cast-iron-sectional-boiler`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-cast-iron-sectional-boiler.ts <asset document> [--force] */
import { type Script, box, main, of, rev } from "./lib.ts";

const along = { repeat: { count: 2, translation: [0.68, 0, 0] } };

const script: Script = {
  id: "used-cast-iron-sectional-boiler",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the two bearers at their true 100 mm section, the boiler as one block from the bearers to the jacket top (base band and jacket merged), the control panel as its 100 mm box, burner door and blanking plate as one 75 mm boss on the front, the cleaning door below it as a 75 mm block that stands its true 30 mm proud, and the flue spigot as a solid 150 mm cylinder at the back; stubs, eye bolts and lugs are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: die zwei Kanthölzer im wahren 100-mm-Querschnitt, der Kessel als ein Block von den Hölzern bis zur Manteloberkante (Grundrahmen und Mantel verschmolzen), das Schaltfeld als sein 100-mm-Kasten, Brennertür und Blindplatte als ein 75-mm-Vorsprung an der Front, die Putztür darunter als 75-mm-Block, der seine wahren 30 mm vorsteht, und der Abgasstutzen als voller 150-mm-Zylinder an der Rückseite; Stutzen, Ringschrauben und Laschen liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: bearers, jacket, panel, door boss and spigot by reference; blanking plate, cleaning cover, stubs and base band as shells of their layer at true thickness; the hidden block as one casting inside; a 75 mm flue outlet at the spigot mouth and a 75 mm plume patch on the jacket top.", de: "Thermik: Hölzer, Mantel, Schaltfeld, Türvorsprung und Stutzen als Verweis; Blindplatte, Putzdeckel, Stutzen und Grundrahmen als Schalen ihrer Schicht in wahrer Dicke; der verdeckte Block als ein Gussteil im Inneren; ein 75-mm-Abgasauslass an der Stutzenmündung und eine 75-mm-Auftriebsfläche auf dem Mantel." },
        { en: "Structure: the bearers by reference, the cast block as the member of the boiler, four 75 mm bearing pads and two mass markers.", de: "Tragwerk: die Hölzer als Verweis, der Gussblock als Stab des Kessels, vier 75-mm-Lagerplatten und zwei Massenmarken." },
      ],
      body: [
        box(["bearers", "bearers", "softwood", 0.62], [1, 0.1, 0.1], [0.48, 0.05, -0.22], { repeat: { count: 2, translation: [0, 0, 0.44] } }),
        box(["boiler-block", "boiler", "painted-steel", 0.74], [0.8, 0.85, 0.62], [0.5, 0.425, 0]),
        box(["control-panel", "boiler", "painted-steel", 0.5], [0.2, 0.1, 0.6], [0.8, 0.9, 0]),
        box(["door-boss", "boiler", "cast-iron", 0.3], [0.075, 0.36, 0.4], [0.9375, 0.6, 0]),
        box(["cleaning-door", "boiler", "cast-iron", 0.3], [0.075, 0.14, 0.4], [0.8925, 0.17, 0]),
        rev(["flue-spigot", "boiler", "flue-steel", 0.34], [1, 0, 0], [0, 0.68, 0], [[-0.03, 0], [-0.03, 0.075], [0.1, 0.075], [0.1, 0]]),
      ],
      thermal: [
        of("th-bearers", "bearers", { region: "bearers" }),
        of("th-jacket", "boiler-block", { region: "jacket" }),
        of("th-control-panel", "control-panel", { region: "control-panel" }),
        of("th-door-boss", "door-boss", { region: "doors" }),
        of("th-flue-spigot", "flue-spigot", { region: "flue-spigot" }),
        box(["th-blanking-plate", "boiler", "steel", 0.2], [0.03, 0.24, 0.24], [0.99, 0.6, 0], { region: "blanking-plate", layer: "plate" }),
        box(["th-cleaning-cover", "boiler", "cast-iron", 0.3], [0.03, 0.14, 0.4], [0.915, 0.17, 0], { region: "doors", layer: "casting" }),
        rev(["th-flow-stub", "boiler", "steel", 0.4], [0, 1, 0], [0.22, 0, 0.15], [[0.85, 0], [0.85, 0.026], [0.925, 0.026], [0.925, 0]], { region: "stubs", layer: "tube" }),
        rev(["th-return-stub", "boiler", "steel", 0.4], [1, 0, 0], [0, 0.2, -0.15], [[0.025, 0], [0.025, 0.026], [0.1, 0.026], [0.1, 0]], { region: "stubs", layer: "tube" }),
        box(["th-base-frame", "boiler", "steel", 0.28], [0.76, 0.05, 0.48], [0.5, 0.025, 0], { region: "steelwork", layer: "steel" }),
        box(["th-block", "boiler", "cast-iron", 0.25], [0.74, 0.73, 0.52], [0.5, 0.415, 0], { region: "block" }),
        rev(["th-flue-outlet", "boiler", "flue-steel", 0.2, "detail"], [1, 0, 0], [0, 0.68, 0], [[-0.105, 0], [-0.105, 0.06], [-0.03, 0.06], [-0.03, 0]], { region: "flue-spigot", tags: ["flue-gas"] }),
        box(["th-jacket-plume", "boiler", "painted-steel", 0.74, "detail"], [0.36, 0.075, 0.4], [0.45, 0.8875, 0], { region: "jacket", tags: ["jacket-plume"] }),
      ],
      structure: [
        of("member-bearers", "bearers", { member: "bearers", part: "member" }),
        box(["member-block", "boiler", "cast-iron", 0.3], [0.74, 0.73, 0.52], [0.5, 0.415, 0], { member: "boiler", part: "member" }),
        box(["joint-bearing-right", "boiler", "steel", 0.5, "detail"], [0.075, 0.075, 0.075], [0.16, 0.0375, -0.221], { ...along, member: "boiler", part: "joint" }),
        box(["joint-bearing-left", "boiler", "steel", 0.5, "detail"], [0.075, 0.075, 0.075], [0.16, 0.0375, 0.221], { ...along, member: "boiler", part: "joint" }),
        box(["mass-boiler", "boiler", "cast-iron", 0.5, "detail"], [0.075, 0.075, 0.075], [0.549, 0.423, 0], { member: "boiler", part: "mass" }),
        box(["mass-bearers", "bearers", "softwood", 0.5, "detail"], [0.075, 0.075, 0.075], [0.48, 0.05, -0.22], { member: "bearers", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
