/** 📚️ Shared texts, references and region templates of the three PW-02 park trees (horse chestnut, European beech, common ash).
 * @see ./kit.ts */
import { tx, type Text } from "./kit.ts";

export const COMMON_REFERENCES = [
  { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapters 4 and 5 (density, moisture and mechanical properties of green wood); used as a guide, species values are authored estimates." },
  { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, power-law profile used as a simplification of the logarithmic profile." },
  { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet-snow accretion on foliage." },
  { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
  { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance." },
  { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  { id: "roloff-2010", citation: "Roloff, A. (2010). Bäume Mitteleuropas: Von Aspe bis Zirbelkiefer. Wiley-VCH, Weinheim. Habit, crown form, bark, leaves, flowers and fruit of the native and planted tree species; used for dimensions and habit as orders of magnitude." },
  { id: "breda-2003", citation: "Bréda, N. J. J. (2003). Ground-based measurements of leaf area index: a review of methods, instruments and current controversies. Journal of Experimental Botany 54(392), 2403-2417. Leaf area indices of temperate broadleaf stands." },
  { id: "koerner-1979", citation: "Körner, C., Scheel, J. A. and Bauer, H. (1979). Maximum leaf diffusive conductance in vascular plants. Photosynthetica 13(1), 45-82. Ranges of maximum leaf conductance of deciduous trees." },
];

/** 🍃️ A foliage region with exchange area and leaf model. */
export const leafRegion = (id: string, title: Text, material: string, size: number, sky: number, heat: number, gs: number) => ({
  id, title, kind: "foliage", model: "leaf", material, areaM2: 0, areaBasis: "exchange", characteristicLengthM: size, geometry: "vertical-plate", skyViewFactor: sky, layers: [], heatCapacityJM2K: heat,
  leaf: { stomatalConductanceMaxMS: gs, leafAreaIndex: 0, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014",
});

/** 🪵️ A passive bark region; its area is the outer surface of its 1:50 solids. */
export const barkRegion = (id: string, title: Text, material: string, length: number, sky: number, heat: number) => ({
  id, title, kind: "bark", model: "passive", material, areaM2: 0, characteristicLengthM: length, geometry: "cylinder", skyViewFactor: sky, layers: [], heatCapacityJM2K: heat, source: "authored-estimate", reference: "wood-handbook-2021",
});

export const broadleafAssumptions = (lai: string, gsSun: string, gsShade: string): Text[] => [
  tx(`Leaf regions use one-sided leaf area; the leaf area index (sun + shade ${lai}) is that area over the crown ground projection, within the range Bréda (2003) gives for closed temperate broadleaf canopies. The maximum stomatal conductances (${gsSun} and ${gsShade} m/s) are authored estimates within the ranges of Körner et al. (1979) for deciduous trees; no published species maximum is claimed. Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.`, `Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex (Sonne + Schatten ${lai.replace(/\./g, ",")}) ist diese Fläche bezogen auf die Kronenprojektion am Boden, im Bereich, den Bréda (2003) für geschlossene Laubwaldbestände angibt. Die maximalen Spaltöffnungsleitfähigkeiten (${gsSun.replace(".", ",")} und ${gsShade.replace(".", ",")} m/s) sind Autorenschätzungen innerhalb der Bereiche von Körner et al. (1979) für sommergrüne Bäume; es wird kein veröffentlichtes Artmaximum behauptet. Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben.`),
  tx("Bark, limbs and twigs are passive. Their areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1.5 to 3 cm, an authored estimate guided by the Wood Handbook; their area is the outer surface of the 1:50 solids. No surface temperature is authored.", "Rinde, Äste und Triebe sind passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1,5 bis 3 cm, eine Autorenschätzung angelehnt an das Wood Handbook; ihre Fläche ist die Außenfläche der Körper 1:50. Es werden keine Oberflächentemperaturen vorgegeben."),
  tx("Transpiration emitters release the latent flux of their foliage region as vapour, spread over nine (sun) and five (shade) anchors on the tops of the foliage lobes across the crown; the direction is the vertical measured in the frame of the first anchor's bone, since the release is diffuse. Sun and shade foliage are separate regions: the sun leaves form the outer lobes, the shade leaves the inner core round the stem, and each pose authors the sunlit share of each region.", "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf neun (Sonne) und fünf (Schatten) Anker auf den Oberseiten der Laubkörper über die Krone; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist. Sonnen- und Schattenlaub sind getrennte Regionen: Die Sonnenblätter bilden die äußeren Laubkörper, die Schattenblätter den inneren Kern um den Stamm, und jede Pose gibt den besonnten Anteil jeder Region vor."),
  tx("Leaf colour and leaf fall are not geometric: a pose cannot hide shapes, so seasonal poses change the thermal binding (stomatal closure, wetness, sun) and the loads only.", "Laubfarbe und Laubfall sind nicht geometrisch: Eine Pose kann keine Formen ausblenden, saisonale Posen ändern daher nur die thermische Bindung (Stomataschluss, Nässe, Sonne) und die Lasten."),
];

export const structureAssumptions = (density: string): Text[] => [
  tx("The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centre of the foliage of each carrying bone (its share of the porous frontal crown area) and on the clear stem as a line load, gravity on every bone mass. The base moment follows from statics at the posed geometry; root-soil failure and dynamic amplification are not modelled. Pose lean angles are authored estimates scaled with the drag force, not a computed deflection.", "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck wirkt im Schwerpunkt des Laubs jedes tragenden Knochens (sein Anteil an der porösen Kronenstirnfläche) und am Schaft als Linienlast, Schwerkraft auf jede Knochenmasse. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen und dynamische Überhöhung werden nicht modelliert. Die Neigungswinkel der Posen sind Autorenschätzungen proportional zur Widerstandskraft, keine berechnete Durchbiegung."),
  tx(`Bone masses are the volumes of the 1:50 wood lofts times a green wood density of ${density} kg/m3 plus the fresh foliage of the lobes the bone carries; section properties are solid circles of green wood (authored estimates guided by the Wood Handbook); hollow or decayed stems are not represented.`, `Knochenmassen sind die Volumina der Holzlofts 1:50 mal einer Frischholzdichte von ${density} kg/m3 plus das frische Laub der Laubkörper, die der Knochen trägt; Querschnittswerte sind Vollkreise aus frischem Holz (Autorenschätzungen angelehnt an das Wood Handbook); hohle oder faule Stämme werden nicht abgebildet.`),
];

/** ☀️ Sunlit share tracks over a day: night 0, rising to the noon value, a little lower in the afternoon. */
export function diurnalSunlit(noon: Record<string, number>): Record<string, [number, number][]> {
  const shape: [number, number][] = [[0, 0], [5.5, 0], [7, 0.35], [9, 0.8], [12, 1], [15, 0.85], [17, 0.55], [19, 0.15], [20.5, 0], [24, 0]];
  return Object.fromEntries(Object.entries(noon).map(([id, v]) => [id, shape.map(([t, k]) => [t, Math.round(v * k * 1000) / 1000] as [number, number])]));
}
