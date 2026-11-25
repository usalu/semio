/** 🚿️ `used-wall-hung-gas-boiler`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-wall-hung-gas-boiler.ts <asset document> [--force] */
import { type Script, type Solid, box, main, of, rev } from "./lib.ts";

const strap = (id: string, bone: string, length: number, side: 1 | -1): Solid => box([id, bone, "polyester-webbing", 0.35], [length, 0.002, 0.1], [length / 2, side * 0.001, 0], { sheet: true });
const straps = [strap("strap-back-foot", "strap-back-foot", 0.148, 1), strap("strap-back-leg", "strap-back-leg", 0.733, 1), strap("strap-back-top", "strap-back-top", 0.185, 1), strap("strap-front-foot", "strap-front-foot", 0.148, -1), strap("strap-front-leg", "strap-front-leg", 0.737, -1), strap("strap-front-top", "strap-front-top", 0.175, -1)];
const support = (name: string, u: number, w: number): Solid => box([`support-${name}`, "pallet", "pallet-pine", 0.3, "ground"], [0.1, 0.075, 0.1], [u, 0.0375, w], { member: "pallet", part: "support", anchor: name });

const script: Script = {
  id: "used-wall-hung-gas-boiler",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the pallet as one 144 mm slab, the two bearers widened from 40 to 75 mm, the boiler as its casing box with the flue adapter as one 126 mm cylinder on top and the connection row as one 75 mm block below; the two lashing straps are typified as one 100 mm band on the centre line, six declared 2 mm sheets on the strap bones, so the lashing still reads and still opens in the poses; hooks, rail, trap, fascia and ratchets are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: die Palette als eine 144-mm-Platte, die zwei Lagerhölzer von 40 auf 75 mm verbreitert, die Therme als ihr Gehäusequader mit dem Abgasadapter als einem 126-mm-Zylinder oben und der Anschlussreihe als einem 75-mm-Block unten; die zwei Zurrgurte sind zu einem 100-mm-Band auf der Mittellinie typisiert, sechs erklärte 2-mm-Platten an den Gurtknochen, sodass die Verzurrung lesbar bleibt und sich in den Posen weiter öffnet; Haken, Schiene, Siphon, Blende und Ratschen liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: the body solids in their regions and layers by reference; controls patch, trap and rail as shells of their layer at true size, the hidden burner and heat exchanger as blocks inside the casing and a 75 mm flue-gas outlet on the adapter.", de: "Thermik: die Körper in ihren Regionen und Schichten als Verweis; Bedienfläche, Siphon und Schiene als Schalen ihrer Schicht in wahrer Größe, der verdeckte Brenner und Wärmetauscher als Blöcke im Gehäuse und ein 75-mm-Abgasauslass auf dem Adapter." },
        { en: "Structure: the members by reference, the casing also holding the bracket bone, two 75 mm bearer pads, four 75 mm pallet supports and two mass markers.", de: "Tragwerk: die Stäbe als Verweis, das Gehäuse fasst auch den Konsolenknochen, zwei 75-mm-Auflagen auf den Lagerhölzern, vier 75-mm-Palettenauflager und zwei Massenmarken." },
      ],
      body: [
        box(["pallet-slab", "pallet", "pallet-pine", 0.58], [1.2, 0.144, 0.8], [0.6, 0.072, 0]),
        box(["bearers", "pallet", "spruce", 0.7], [0.4, 0.08, 0.075], [0.6, 0.184, -0.21], { repeat: { count: 2, translation: [0, 0, 0.42] } }),
        box(["casing", "boiler", "painted-steel-aged", 0.86], [0.35, 0.7, 0.44], [0.183, -0.33, 0]),
        rev(["adapter", "boiler", "flue-aluminium", 0.8], [0, 1, 0], [0.1835, 0, 0], [[0.02, 0], [0.02, 0.063], [0.125, 0.063], [0.125, 0]]),
        box(["connections", "boiler", "brass", 0.5], [0.075, 0.075, 0.29], [0.153, -0.7175, 0]),
        ...straps,
      ],
      thermal: [
        of("th-pallet-slab", "pallet-slab", { region: "pallet", layer: "boards" }),
        of("th-bearers", "bearers", { region: "bearers", layer: "timber" }),
        of("th-casing", "casing", { region: "casing", layer: "sheet" }),
        of("th-adapter", "adapter", { region: "flue-adapter", layer: "pipe" }),
        of("th-connections", "connections", { region: "connections", layer: "tube" }),
        ...straps.map((solid) => of(`th-${solid.id as string}`, solid.id as string, { region: "straps", layer: "webbing" })),
        box(["th-controls-patch", "boiler", "polycarbonate", 0.42], [0.03, 0.085, 0.4], [0.373, -0.6175, 0], { region: "controls", layer: "panel" }),
        rev(["th-trap", "boiler", "polypropylene", 0.88], [0, 1, 0], [0.253, 0, 0], [[-0.75, 0], [-0.75, 0.02], [-0.68, 0.02], [-0.68, 0]], { region: "condensate-trap", layer: "body" }),
        box(["th-rail", "bracket", "galv", 0.55], [0.03, 0.045, 0.38], [-0.007, -0.0225, 0], { region: "bracket", layer: "rail" }),
        box(["th-burner", "boiler", "burner-mesh", 0.5], [0.04, 0.08, 0.08], [0.263, -0.21, 0], { region: "burner", layer: "mesh" }),
        box(["th-heat-exchanger", "boiler", "stainless-steel", 0.5], [0.17, 0.23, 0.23], [0.158, -0.21, 0], { region: "heat-exchanger", layer: "wall" }),
        rev(["th-flue-gas", "boiler", "polypropylene", 0.5, "detail"], [0, 1, 0], [0.1835, 0, 0], [[0.125, 0], [0.125, 0.0375], [0.2, 0.0375], [0.2, 0]], { region: "flue-adapter", tags: ["flue-gas"] }),
      ],
      structure: [
        of("member-pallet-slab", "pallet-slab", { member: "pallet", part: "member" }),
        of("member-bearers", "bearers", { member: "pallet", part: "member" }),
        of("member-casing", "casing", { member: "boiler", part: "member", members: ["bracket"] }),
        ...straps.map((solid) => of(`member-${solid.id as string}`, solid.id as string, { member: solid.bone as string, part: "member" })),
        box(["joint-bearer-pads", "boiler", "epdm", 0.3, "detail"], [0.35, 0.075, 0.075], [0.183, -0.7175, -0.21], { repeat: { count: 2, translation: [0, 0, 0.42] }, member: "boiler", part: "joint" }),
        support("pallet-back-right", 0.0725, -0.35),
        support("pallet-back-left", 0.0725, 0.35),
        support("pallet-front-right", 1.1275, -0.35),
        support("pallet-front-left", 1.1275, 0.35),
        box(["mass-boiler", "boiler", "galv", 0.5, "detail"], [0.075, 0.075, 0.075], [0.183, -0.33, 0], { member: "boiler", part: "mass" }),
        box(["mass-pallet", "pallet", "pallet-pine", 0.5, "detail"], [0.075, 0.075, 0.075], [0.6, 0.072, 0], { member: "pallet", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
