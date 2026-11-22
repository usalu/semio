/** 🌱️ Shared hand tools of the PW-05 species designs: a limb from its target cluster position and three hand-chosen elevations, the common references, the wind and snow pose texts.
 * @see ./tree-kit.ts */
import type { Limb, LobeT, Text } from "./tree-kit.ts";

const D2R = Math.PI / 180;

/** 🌿️ A limb whose foliage cluster centre lies `rc` metres out from the stem: the path is split 50 / 35 / 15 % over the inner part, the outer part and the end shoot, at the elevations `els` (degrees above the horizontal); `rr` are the bark radii at base and at the two joints. */
export function limb(id: string, title: Text, parent: string, at: number, az: number, rc: number, els: [number, number, number], size: [number, number, number], r0: number, extra: Partial<Limb> = {}, split: [number, number, number] = [0.5, 0.35, 0.15]): Limb {
  const [a, b, f] = els.map((e) => e * D2R) as [number, number, number];
  const L = rc / (split[0] * Math.cos(a) + split[1] * Math.cos(b) + 0.55 * split[2] * Math.cos(f));
  return { id, title, parent, at, az, r0, segs: [{ len: round(split[0] * L), el: els[0], r1: round(0.62 * r0) }, { len: round(split[1] * L), el: els[1], r1: round(0.32 * r0) }], fol: { len: round(split[2] * L), el: els[2], r: Math.max(0.03, round(0.16 * r0)) }, size, ...extra };
}

/** 🧭️ Cluster height of a limb from the stem point at `h`: for checking a design against its envelope. */
export function clusterHeight(h: number, l: Limb): number {
  return h + l.segs.reduce((s, g) => s + g.len * Math.sin(g.el * D2R), 0) + 0.55 * l.fol.len * Math.sin(l.fol.el * D2R);
}

const round = (x: number): number => Math.round(x * 1000) / 1000;

/** 🧭️ Compass words of an azimuth (degrees from +x, the asset's forward, towards +z, its left), named as if forward were east. */
export function compass(az: number): Text {
  const en = ["east", "north-east", "north", "north-west", "west", "south-west", "south", "south-east"];
  const de = ["Ost", "Nordost", "Nord", "Nordwest", "West", "Südwest", "Süd", "Südost"];
  const k = Math.round((((az % 360) + 360) % 360) / 45) % 8;
  return { en: en[k]!, de: de[k]! };
}

export const REFERENCES = {
  woodHandbook: { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5, Table 5-3a (strength properties of green wood: modulus of elasticity, modulus of rupture, specific gravity)." },
  en199114: { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m, power-law profile." },
  en199113: { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet-snow accretion on foliage." },
  vogel: { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
  jones: { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance." },
  monteith: { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  silvics: { id: "silvics-1990", citation: "Burns, R. M. and Honkala, B. H. (tech. coords.) (1990). Silvics of North America, Vol. 2: Hardwoods. Agriculture Handbook 654, USDA Forest Service, Washington DC. Species chapters on Liriodendron tulipifera, Quercus rubra and Robinia pseudoacacia (heights, diameters, crown form, bark, flowering)." },
  dirr: { id: "dirr-2009", citation: "Dirr, M. A. (2009). Manual of Woody Landscape Plants, 6th ed., Stipes Publishing, Champaign IL. Landscape sizes, habit and leaf, flower and bark characters of cultivated trees." },
  roloff: { id: "roloff-2010", citation: "Roloff, A. (2010). Bäume Mitteleuropas: Von Aspe bis Zirbelkiefer. Mit den Porträts aller Bäume des Jahres von 1989 bis 2010. Wiley-VCH, Weinheim. Habit, size and wood of the trees planted in Central Europe." },
};

/** 🌬️ The four standard pose texts of a broadleaf tree with its own name in the descriptions. */
export function windPoseTexts(name: Text, porosity: number, snowKg: number): Record<string, { title: Text; description: Text }> {
  const p = porosity.toFixed(2);
  return {
    calm: { title: { en: "Calm summer day", de: "Windstiller Sommertag" }, description: { en: `Light air of 1.5 m/s at 10 m: the ${name.en} stands at rest in full leaf; drag on every foliage cluster is only a few newtons.`, de: `Leiser Zug von 1,5 m/s in 10 m Höhe: Der ${name.de} steht in vollem Laub in Ruhe; der Widerstand jedes Laubbüschels beträgt nur wenige Newton.` } },
    windy: { title: { en: "Fresh breeze", de: "Frische Brise" }, description: { en: `Beaufort 5, 8 m/s at 10 m (power-law profile, exponent 0.22). Drag F = 0.5 rho cd A v^2 with rho 1.25 kg/m3, cd 0.35 (reconfiguration of the leaves at moderate speed, Vogel 1989) and the frontal area of every foliage cluster times an optical solidity of ${p}. The stem leans downwind, limbs facing the wind lift and limbs facing away sink, most at the end shoots.`, de: `Beaufort 5, 8 m/s in 10 m Höhe (Potenzprofil, Exponent 0,22). Widerstand F = 0,5 rho cd A v^2 mit rho 1,25 kg/m3, cd 0,35 (Umformung der Blätter bei mäßiger Geschwindigkeit, Vogel 1989) und der Anströmfläche jedes Laubbüschels mal einer optischen Dichte von ${p.replace(".", ",")}. Der Stamm neigt sich in Windrichtung, dem Wind zugewandte Äste heben sich, abgewandte senken sich, am stärksten an den Endtrieben.` } },
    storm: { title: { en: "Gale", de: "Sturm" }, description: { en: "Beaufort 9, 22 m/s at 10 m. Drag with cd 0.22 (strong reconfiguration of leaves and twigs, Vogel 1989) on the same areas; all joints approach their limits, the end shoots sweep downwind, rain wets bark and leaves and transpiration is damped.", de: "Beaufort 9, 22 m/s in 10 m Höhe. Widerstand mit cd 0,22 (starke Umformung von Blättern und Zweigen, Vogel 1989) auf dieselben Flächen; alle Gelenke nähern sich ihren Grenzen, die Endtriebe schwenken in Windrichtung, Regen nässt Rinde und Blätter und die Transpiration ist gedämpft." } },
    "wet-snow": { title: { en: "Wet autumn snow on the foliage", de: "Nasser Herbstschnee im Laub" }, description: { en: `An early snowfall on a tree still in leaf: ${snowKg} kg/m2 of wet snow on the plan area of every foliage cluster bends limbs and end shoots down; wind is calm. The load follows the order of magnitude of EN 1991-1-3, not a code value.`, de: `Früher Schneefall auf einen noch belaubten Baum: ${snowKg} kg/m2 Nassschnee auf der Grundrissfläche jedes Laubbüschels biegen Äste und Endtriebe nach unten; Windstille. Die Lastannahme folgt der Größenordnung von EN 1991-1-3, ist aber kein Normwert.` } },
  };
}

/** 🍃️ A template helper: offsets and radius factors of one lobe. */
export const lobe = (dr: number, dy: number, dt: number, sa: number, sb = sa, sc = sa): LobeT => [dr, dy, dt, sa, sb, sc];
