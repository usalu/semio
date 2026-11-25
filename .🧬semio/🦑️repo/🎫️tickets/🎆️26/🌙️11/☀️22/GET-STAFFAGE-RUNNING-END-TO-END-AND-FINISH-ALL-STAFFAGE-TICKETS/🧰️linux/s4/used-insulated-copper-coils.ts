/** 🌀️ `used-insulated-copper-coils`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-insulated-copper-coils.ts <asset document> [--force] */
import { type Script, type Solid, box, main, of, rev } from "./lib.ts";

const centre = [0.085, 0.332, 0] as const;
const ring = (id: string, tone: number, from: number, to: number, binding: Solid): Solid => rev([id, "seg-00", "elastomer-foam", tone], [0, 0, 1], centre, [[from, 0.2], [from, 0.345], [to, 0.345], [to, 0.2], [from, 0.2]], binding);
const segments = Array.from({ length: 38 }, (_, index) => `seg-${String(index + 1).padStart(2, "0")}`);

const script: Script = {
  id: "used-insulated-copper-coils",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the pallet as one 144 mm slab, the two battens as one 75 mm cradle block, and the coil of the two insulated lines as one upright ring of 0.69 m outer and 0.40 m inner diameter and 75 mm width: the three turns of 32 and 36 mm tube lie 45 mm apart, under the feature, so the ring stands for the wound pair and no longer unrolls; the bare tails are left out.", de: "Körper für das 75-mm-Merkmal: die Palette als eine 144-mm-Platte, die zwei Kanthölzer als ein 75-mm-Lagerblock und der Ring der zwei gedämmten Leitungen als ein stehender Ring von 0,69 m Außen- und 0,40 m Innendurchmesser und 75 mm Breite: die drei Windungen aus 32- und 36-mm-Rohr liegen 45 mm auseinander, unter dem Merkmal, daher steht der Ring für das gewickelte Paar und rollt sich nicht mehr ab; die blanken Enden entfallen." },
        { en: "Thermal: pallet and cradle by reference, each line as its half of the ring as a shell of its foam layer, the copper ends as a texture face on the ring.", de: "Thermik: Palette und Lagerblock als Verweis, jede Leitung als ihre Hälfte des Rings als Schale ihrer Schaumschicht, die Kupferenden als Texturfläche am Ring." },
        { en: "Structure: slab, cradle and ring by reference, the ring spanning all 39 segment bones, and two mass markers.", de: "Tragwerk: Platte, Lagerblock und Ring als Verweis, der Ring überspannt alle 39 Segmentknochen, und zwei Massenmarken." },
      ],
      body: [
        box(["pallet-slab", "pallet", "pallet-spruce", 0.6], [0.8, 0.144, 0.6], [0.4, 0.072, 0]),
        box(["cradle", "pallet", "pallet-spruce", 0.66], [0.3, 0.075, 0.148], [0.4, 0.1815, -0.002]),
        ring("coil", 0.18, -0.0375, 0.0375, {}),
      ],
      thermal: [
        of("th-pallet-slab", "pallet-slab", { region: "pallet" }),
        of("th-cradle", "cradle", { region: "pallet" }),
        ring("th-liquid-line", 0.2, 0, 0.0375, { region: "liquid-line", layer: "liquid-foam" }),
        ring("th-suction-line", 0.16, -0.0375, 0, { region: "suction-line", layer: "suction-foam" }),
        box(["th-copper-ends", "seg-00", "copper-oxidised", 0.5, "texture"], [0.04, 0.001, 0.075], [0.02, -0.0135, 0], { texture: true, region: "copper-ends" }),
      ],
      structure: [
        of("member-pallet-slab", "pallet-slab", { member: "pallet", part: "member" }),
        of("member-cradle", "cradle", { member: "pallet", part: "member" }),
        of("member-coil", "coil", { member: "seg-00", part: "member", members: segments }),
        box(["mass-coil", "seg-00", "copper-tube", 0.5, "detail"], [0.075, 0.075, 0.075], [0.085, 0.06, 0], { member: "seg-00", part: "mass" }),
        box(["mass-pallet", "pallet", "pallet-spruce", 0.5, "detail"], [0.075, 0.075, 0.075], [0.4, 0.072, 0], { member: "pallet", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
