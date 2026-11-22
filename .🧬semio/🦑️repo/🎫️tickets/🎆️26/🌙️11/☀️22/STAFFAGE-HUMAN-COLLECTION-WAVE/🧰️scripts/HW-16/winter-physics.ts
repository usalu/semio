/** 🌡️ The physical scenario of `person-in-winter-coat`: identity texts, materials, clothing regions after ISO 9920 with the cold-weather ensemble of ISO 11079, the −5 °C reference environment, respiration that condenses in the cold air, poses with the hands in the parka pockets, the cold-weather clips and the provenance.
 *
 * @see ./winter.ts
 */
import { clone, framesOf, reach, renameRegion, t, type Json } from "./lib.ts";

const CLO = 0.155;
const vapour = (r: number, im = 0.38) => Math.round((r / (0.0165 * im)) * 10) / 10;
const layer = (id: string, en: string, de: string, material: string, thicknessM: number, clo: number, heatCapacityJM2K: number, im = 0.38) => ({ id, title: t(en, de), kind: "clothing", material, thicknessM, resistanceM2KW: Math.round(clo * CLO * 1e5) / 1e5, vapourResistanceM2PaW: vapour(clo * CLO, im), heatCapacityJM2K, source: "authored-estimate", reference: "iso-9920-2007" });
const SKIN_NODE = 13000;
const capacity = (layers: Json[]) => SKIN_NODE + layers.reduce((s, l) => s + l.heatCapacityJM2K, 0);

export const MATERIALS: Json[] = [
  { id: "skin", title: t("Skin", "Haut"), emissivity: 0.98, solarAbsorptivity: 0.6, source: "literature", reference: "ashrae-2021" },
  { id: "hair", title: t("Hair", "Haar"), emissivity: 0.98, solarAbsorptivity: 0.85, source: "authored-estimate" },
  { id: "down-parka", title: t("Down parka, nylon shell", "Daunenparka, Nylonhülle"), emissivity: 0.88, solarAbsorptivity: 0.82, specificHeatJKgK: 1400, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "wool-scarf", title: t("Chunky wool scarf", "Grobstrickschal aus Wolle"), emissivity: 0.92, solarAbsorptivity: 0.62, source: "literature", reference: "ashrae-2021" },
  { id: "knit-wool", title: t("Knitted wool beanie", "Gestrickte Wollmütze"), emissivity: 0.92, solarAbsorptivity: 0.7, specificHeatJKgK: 1360, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "knit-glove", title: t("Knitted wool gloves", "Gestrickte Wollhandschuhe"), emissivity: 0.92, solarAbsorptivity: 0.75, specificHeatJKgK: 1360, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "wool-trouser", title: t("Wool trousers", "Wollhose"), emissivity: 0.92, solarAbsorptivity: 0.7, source: "literature", reference: "ashrae-2021" },
  { id: "nubuck-boot", title: t("Lined nubuck winter boot", "Gefütterter Winterstiefel aus Nubukleder"), emissivity: 0.9, solarAbsorptivity: 0.78, specificHeatJKgK: 1500, source: "authored-estimate" },
  { id: "cotton-underwear", title: t("Cotton undershirt and long johns", "Baumwollunterhemd und lange Unterhose"), emissivity: 0.93, solarAbsorptivity: 0.5, specificHeatJKgK: 1300, source: "authored-estimate" },
  { id: "cotton-shirt", title: t("Flannel shirt", "Flanellhemd"), emissivity: 0.93, solarAbsorptivity: 0.5, specificHeatJKgK: 1300, source: "authored-estimate" },
  { id: "wool-sweater", title: t("Wool sweater", "Wollpullover"), emissivity: 0.92, solarAbsorptivity: 0.75, specificHeatJKgK: 1360, source: "authored-estimate" },
  { id: "wool-sock", title: t("Wool socks", "Wollsocken"), emissivity: 0.93, solarAbsorptivity: 0.6, specificHeatJKgK: 1360, source: "authored-estimate" },
  { id: "bone", title: t("Bone", "Knochen"), emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" },
];

const REFERENCES: Json[] = [
  { id: "iso-11079-2007", citation: "ISO 11079:2007 Ergonomics of the thermal environment, determination and interpretation of cold stress when using required clothing insulation (IREQ) and local cooling effects; Annex tables of cold-weather ensembles (2 to 3 clo for standing or slow walking at −5 to −10 °C)." },
  { id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; codes 17170 (walking, 2.0-2.5 mph, 3.0 MET), 09070 (standing, talking, 1.8 MET), 07040 (standing quietly, 1.3 MET)." },
  { id: "havenith-2002", citation: "Havenith, G. (2002). Interaction of clothing and thermoregulation. Exogenous Dermatology 1(5), 221-230; hand and head heat loss in the cold and the insulation of gloves and caps." },
  { id: "hoeppe-1981", citation: "Höppe, P. (1981). Temperatures of expired air under varying climatic conditions. International Journal of Biometeorology 25(2), 127-132; exhaled air at about 32 °C and saturated when the inhaled air is below 0 °C." },
];

/** 🧮️ DuBois and DuBois (1916) body surface area. */
export const dubois = (kg: number, m: number) => Math.round(0.007184 * kg ** 0.425 * (m * 100) ** 0.725 * 1000) / 1000;

export function winterPhysics(a: Json, base: Json): void {
  renameRegion(a, "coat", "parka");
  renameRegion(a, "hand-skin", "gloves");
  renameRegion(a, "shoes", "boots");
  for (const sc of ["50", "100", "200", "500"]) for (const s of a.volumes[sc].thermal) if (s.layer === "overcoat") s.layer = "parka";
  for (const sc of ["50", "100", "200", "500"]) for (const s of a.volumes[sc].thermal) if (s.layer === "jacket") s.layer = "sweater";
  for (const sc of ["50", "100", "200", "500"]) for (const s of a.volumes[sc].thermal) if (s.layer === "trousers" && s.region === "trousers") s.layer = "trousers";

  a.title = t("Person in Winter Coat", "Person im Wintermantel");
  a.summary = t(
    "A man of 1.78 m bundled up for a frosty day: a knee-length quilted down parka with a stand collar, a chunky wool scarf, a knitted beanie, knitted gloves and lined winter boots with the trousers tucked in. He stands and walks with his hands in the side pockets of the parka, stamps his cold feet, and at −5 °C every breath condenses into a small cloud.",
    "Ein Mann von 1,78 m, warm eingepackt für einen frostigen Tag: knielanger gesteppter Daunenparka mit Stehkragen, grober Wollschal, Strickmütze, Strickhandschuhe und gefütterte Winterstiefel, die Hose in den Schaft gesteckt. Er steht und geht mit den Händen in den Seitentaschen des Parkas, stampft mit den kalten Füßen auf, und bei −5 °C kondensiert jeder Atemzug zu einem kleinen Wölkchen.",
  );
  a.tags = ["human", "adult", "winter", "cold", "parka", "beanie", "gloves", "hands-in-pockets", "gait", "breath-vapour"];
  a.provenance.author = "Semio staffage human collection worker HW-16";
  a.provenance.method = "handcrafted";
  a.provenance.created = "2026-11-22";
  a.provenance.references = [...a.provenance.references.filter((r: Json) => !REFERENCES.some((n) => n.id === r.id)), ...REFERENCES];
  a.provenance.notes = t(
    "Rig, body, gait and the cut of the long coat are derived from the walking commuter; the overcoat is filled out by a 20 mm down layer, the beanie, the boot shafts and the parka details are new and every garment is drawn as the projection of its own solids at each scale. Anthropometry: 1.78 m and 72 kg without clothing (DIN 33402-2 and ANSUR II male medians lie at 1.75 to 1.76 m and 78 to 85 kg; a slender build is chosen), segment masses after de Leva (1996) with the clothing added to the segments it hangs on.",
    "Rig, Körper, Gang und der Schnitt des langen Mantels stammen vom gehenden Pendler; der Mantel ist durch eine 20 mm dicke Daunenschicht aufgefüllt, Mütze, Stiefelschäfte und Parkadetails sind neu und jedes Kleidungsstück ist in jedem Maßstab als Projektion seiner eigenen Körper gezeichnet. Anthropometrie: 1,78 m und 72 kg ohne Kleidung (die Mediane von DIN 33402-2 und ANSUR II für Männer liegen bei 1,75 bis 1,76 m und 78 bis 85 kg; gewählt ist ein schlanker Körperbau), Segmentmassen nach de Leva (1996), die Kleidung den Segmenten zugeschlagen, an denen sie hängt.",
  );

  a.materials = MATERIALS;
  const extra: Record<string, number> = { torso: 0.55, lumbar: 0.35, "arm-l": 0.1, "arm-r": 0.1, "foot-l": 0.36, "foot-r": 0.36, head: 0.08, "hand-l": 0.03, "hand-r": 0.03, "neck": 0.12 };
  for (const b of a.rig.bones) if (b.mass && extra[b.id]) b.mass.kg = Math.round((b.mass.kg + extra[b.id]!) * 1000) / 1000;
  for (const b of a.rig.bones) if (b.id === "torso" && b.title) b.title = t("Torso in the down parka", "Rumpf im Daunenparka");

  const env = { airTempC: -5, meanRadiantTempC: -7, skyTempC: -18, groundTempC: -3, relativeHumidity: 0.82, windMS: 2, windDirectionDeg: 180, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 };
  a.thermal.environment = env;
  a.thermal.baseMet = 1.3;
  a.thermal.body = { ...a.thermal.body, massKg: 72, skinAreaM2: dubois(72, 1.78), basalMetabolismW: 80, source: "authored-estimate", reference: "gagge-1971" };
  a.thermal.respiration.exhaledTempC = 32;
  a.thermal.respiration.exhaledRelativeHumidity = 1;
  a.thermal.respiration.reference = "hoeppe-1981";
  a.thermal.respiration.patterns = [
    { id: "rest-standing", title: t("Standing in the cold", "Stehen in der Kälte"), rateBpm: 14, tidalVolumeL: 0.55, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.13, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
    { id: "walking", title: t("Walking briskly at 3.6 km/h", "Zügiges Gehen mit 3,6 km/h"), rateBpm: 21, tidalVolumeL: 1.25, inhaleFraction: 0.45, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.03, route: "nose-and-mouth", source: "authored-estimate", reference: "icrp-66-1994" },
  ];

  const R = (id: string) => a.thermal.regions.find((r: Json) => r.id === id);
  const tee = layer("tee", "Cotton undershirt", "Baumwollunterhemd", "cotton-underwear", 0.0008, 0.09, 180);
  const shirt = layer("shirt", "Flannel shirt, long sleeve", "Flanellhemd, langarm", "cotton-shirt", 0.0015, 0.3, 220);
  const sweater = layer("sweater", "Wool sweater", "Wollpullover", "wool-sweater", 0.004, 0.42, 520);
  const down = layer("parka", "Quilted down parka, knee length", "Gesteppter Daunenparka, knielang", "down-parka", 0.025, 1.8, 900, 0.34);
  Object.assign(R("parka"), { title: t("Down parka over torso, arms and thighs", "Daunenparka über Rumpf, Armen und Oberschenkeln"), material: "down-parka", layers: [tee, shirt, sweater, down], heatCapacityJM2K: capacity([tee, shirt, sweater, down]), reference: "iso-11079-2007" });
  const scarf = layer("scarf", "Chunky knitted wool scarf, wound twice", "Grob gestrickter Wollschal, zweimal gewickelt", "wool-scarf", 0.02, 0.5, 700);
  Object.assign(R("scarf"), { title: t("Chunky scarf round the neck", "Grober Schal um den Hals"), layers: [scarf], heatCapacityJM2K: capacity([scarf]) });
  const glove = layer("glove", "Knitted wool gloves", "Gestrickte Wollhandschuhe", "knit-glove", 0.004, 0.45, 300);
  Object.assign(R("gloves"), { title: t("Knitted gloves", "Strickhandschuhe"), kind: "clothing", material: "knit-glove", layers: [glove], heatCapacityJM2K: capacity([glove]), reference: "havenith-2002" });
  const johns = layer("long-johns", "Long johns", "Lange Unterhose", "cotton-underwear", 0.001, 0.15, 120);
  const trousers = layer("trousers", "Wool trousers, straight, thick", "Wollhose, gerade, dick", "wool-trouser", 0.0025, 0.26, 480);
  Object.assign(R("trousers"), { title: t("Trousers over long johns", "Hose über langer Unterhose"), layers: [johns, trousers], heatCapacityJM2K: capacity([johns, trousers]) });
  for (const sc of ["50", "100", "200", "500"]) for (const s of a.volumes[sc].thermal) if (s.layer === "briefs") s.layer = "long-johns";
  const socks = layer("socks", "Wool socks, calf length", "Wollsocken, wadenlang", "wool-sock", 0.003, 0.1, 160);
  const boot = layer("shoe", "Lined winter boot", "Gefütterter Winterstiefel", "nubuck-boot", 0.008, 0.35, 1400);
  Object.assign(R("boots"), { title: t("Wool socks and lined winter boots", "Wollsocken und gefütterte Winterstiefel"), material: "nubuck-boot", layers: [socks, boot], heatCapacityJM2K: capacity([socks, boot]) });
  const hair = R("hair");
  hair.body = { segment: "head", metabolicShare: 0.012, skinFraction: 0.006 };
  const beanieLayer = layer("beanie", "Knitted wool beanie with folded cuff", "Strickmütze aus Wolle mit Umschlag", "knit-wool", 0.007, 0.55, 360);
  const beanie = { id: "beanie", title: t("Knitted beanie", "Strickmütze"), kind: "clothing", model: "body", material: "knit-wool", areaM2: 0.05, characteristicLengthM: 0.18, geometry: "sphere", skyViewFactor: 0.8, layers: [beanieLayer], heatCapacityJM2K: capacity([beanieLayer]), body: { segment: "head", metabolicShare: 0.018, skinFraction: 0.018 }, source: "authored-estimate", reference: "havenith-2002" };
  a.thermal.regions.splice(a.thermal.regions.indexOf(hair) + 1, 0, beanie);
  a.thermal.assumptions = [
    t("Skin area 1.894 m2 after DuBois and DuBois (1916) for 72 kg and 1.78 m; the skin fractions are 0.05 face and neck, 0.006 scalp under the hair, 0.018 under the beanie, 0.05 hands, 0.49 trunk and arms, 0.021 neck under the scarf, 0.295 legs and 0.07 feet (rule-of-nines proportions, authored).", "Hautfläche 1,894 m2 nach DuBois und DuBois (1916) für 72 kg und 1,78 m; die Hautanteile sind 0,05 Gesicht und Hals, 0,006 Kopfhaut unter dem Haar, 0,018 unter der Mütze, 0,05 Hände, 0,49 Rumpf und Arme, 0,021 Hals unter dem Schal, 0,295 Beine und 0,07 Füße (Neunerregel-Proportionen, geschätzt)."),
    t("Clothing layers are local insulations in clo after the garment tables of ISO 9920 (1 clo = 0.155 m2K/W, enclosed air included), referred to the outer area; the ensemble (about 2.4 clo over the covered body) matches the required insulation IREQ of ISO 11079 for standing and slow walking at −5 °C. Vapour resistance R/(0.0165 K/Pa x i_m) with i_m 0.38, 0.34 for the nylon-shell parka.", "Kleidungsschichten sind lokale Isolationen in clo nach den Kleidungstabellen von ISO 9920 (1 clo = 0,155 m2K/W, eingeschlossene Luft enthalten), bezogen auf die Außenfläche; das Ensemble (etwa 2,4 clo über dem bedeckten Körper) entspricht der erforderlichen Isolation IREQ nach ISO 11079 für Stehen und langsames Gehen bei −5 °C. Dampfwiderstand R/(0,0165 K/Pa x i_m) mit i_m 0,38, 0,34 für den Parka mit Nylonhülle."),
    t("Region heat capacity = skin node 13 kJ/(m2K) (10 % of the body mass at 3.5 kJ/(kgK) over the skin area, after the Gagge two-node idea) plus the areal heat capacity of the garments.", "Wärmekapazität der Region = Hautknoten 13 kJ/(m2K) (10 % der Körpermasse bei 3,5 kJ/(kgK) über der Hautfläche, nach dem Zweiknotengedanken von Gagge) plus die flächenbezogene Wärmekapazität der Kleidung."),
    t("Standing in the cold with the hands in the pockets 1.4 met, with the arms at the sides 1.3 met, stamping the feet 2.2 to 2.3 met, walking briskly at 3.6 km/h 3.0 met (Ainsworth 2011, codes 07040, 17170; ISO 8996 Annex B). Exhaled air leaves at 32 °C and saturated (Höppe 1981), so at −5 °C and 82 % relative humidity every exhalation crosses the dew point of the mixing line and condenses.", "Stehen in der Kälte mit den Händen in den Taschen 1,4 met, mit hängenden Armen 1,3 met, Aufstampfen 2,2 bis 2,3 met, zügiges Gehen mit 3,6 km/h 3,0 met (Ainsworth 2011, Codes 07040, 17170; ISO 8996 Anhang B). Die Ausatemluft verlässt den Körper mit 32 °C und gesättigt (Höppe 1981), daher kreuzt bei −5 °C und 82 % relativer Feuchte jedes Ausatmen den Taupunkt der Mischungsgeraden und kondensiert."),
  ];

  posesAndClips(a, base);
}

const REGIONS = ["face-skin", "hair", "beanie", "gloves", "parka", "scarf", "trousers", "boots"] as const;
const bind = (exposure: Partial<Record<(typeof REGIONS)[number], number>>) => Object.fromEntries(REGIONS.map((r) => [r, { exposedFraction: exposure[r] ?? 1, sunlitFraction: 0 }]));

/** 🧤️ Arm offsets that put each gloved hand into the side-entry pocket of the parka at the hip, solved by a small IK within the joint limits; the target is the palm 3 cm inside the down layer, ahead of the side seam at hip height (lumbar frame). The arm chain is drawn as solids, so the spread and twist read right in every view. */
function pocketArms(a: Json, fixed: Record<string, Json>): Record<string, Json> {
  let off = clone(fixed);
  for (const side of ["l", "r"] as const) {
    const w = side === "l" ? 1 : -1;
    const res = reach(a, off, [
      { bone: `arm-${side}`, field: "angleDeg", min: -40, max: 25, start: -10 },
      { bone: `arm-${side}`, field: "abductionDeg", min: side === "l" ? 0 : -12, max: side === "l" ? 12 : 0, start: 6 * w },
      { bone: `arm-${side}`, field: "twistDeg", min: -50, max: 50, start: 0 },
      { bone: `forearm-${side}`, field: "angleDeg", min: 0, max: 110, start: 50 },
      { bone: `hand-${side}`, field: "angleDeg", min: -45, max: 45, start: 0 },
    ], { bone: `hand-${side}`, at: [0.06, 0, 0] }, { bone: "lumbar", at: [-0.08, -0.055, 0.155 * w] }, (F) => 3 * Math.max(0, Math.abs(F[`forearm-${side}`].frame.o[2]) - 0.25));
    off = res.offsets;
    if (res.error > 0.02) console.log(`[DEBUG] pocket ${side} residual ${res.error.toFixed(3)} m`);
  }
  return off;
}

function posesAndClips(a: Json, base: Json): void {
  const walkNames = ["contact-left", "loading-left", "passing-left", "contact-right", "loading-right", "passing-right"];
  const hunch = { "clavicle-l": { angleDeg: 6 }, "clavicle-r": { angleDeg: 6 }, neck: { angleDeg: -4 }, head: { angleDeg: -3 }, torso: { angleDeg: -1.5 }, lumbar: { angleDeg: -0.6 } };
  const stand = pocketArms(a, { ...hunch });
  const pocket = Object.fromEntries(Object.entries(stand).filter(([k]) => /^(arm|forearm|hand)-/.test(k)));
  const stance = ["heel-l", "ball-l", "heel-r", "ball-r"];
  const poses: Json[] = [
    { id: "hands-in-pockets", title: t("Standing with the hands in the pockets", "Stehend mit den Händen in den Taschen"), description: t("Upright stance in the cold, the shoulders a little drawn up, the chin in the scarf and both gloved hands deep in the hand-warmer pockets of the parka.", "Aufrechter Stand in der Kälte, die Schultern leicht hochgezogen, das Kinn im Schal und beide behandschuhten Hände tief in den Muff-Taschen des Parkas."), bones: stand, ground: "auto", contacts: stance, thermal: { activityMet: 1.4, breathing: "rest-standing", regions: bind({ gloves: 0.25 }), emitters: { "body-plume": 0.55 } }, loads: [] },
    { id: "arms-at-sides", title: t("Standing with the arms at the sides", "Stehend mit hängenden Armen"), description: t("Relaxed upright stance, the gloved hands hanging beside the long parka, the shoulders drawn up against the cold.", "Entspannter aufrechter Stand, die behandschuhten Hände hängen neben dem langen Parka, die Schultern gegen die Kälte hochgezogen."), bones: { ...hunch, "arm-l": { angleDeg: 2 }, "forearm-l": { angleDeg: 12 }, "hand-l": { angleDeg: 4 }, "arm-r": { angleDeg: 3 }, "forearm-r": { angleDeg: 14 }, "hand-r": { angleDeg: 4 } }, ground: "auto", contacts: stance, thermal: { activityMet: 1.3, breathing: "rest-standing", regions: bind({}), emitters: { "body-plume": 0.55 } }, loads: [] },
    { id: "stamping-feet", title: t("Stamping the cold feet", "Kalte Füße aufstampfen"), description: t("Hands in the pockets, the weight on the left foot while the right knee is lifted to stamp the boot down on the frozen ground.", "Die Hände in den Taschen, das Gewicht auf dem linken Fuß, während das rechte Knie gehoben wird, um den Stiefel auf den gefrorenen Boden zu stampfen."), bones: { ...clone(stand), "thigh-r": { angleDeg: 38 }, "shank-r": { angleDeg: -62 }, "foot-r": { angleDeg: 14 }, "thigh-l": { angleDeg: -1 }, "shank-l": { angleDeg: -2 } }, ground: "auto", contacts: ["heel-l", "ball-l"], thermal: { activityMet: 2.3, breathing: "walking", regions: bind({ gloves: 0.25 }), emitters: { "body-plume": 0.7 } }, loads: [] },
    { id: "stamping-left", key: true, title: t("Stamping with the left foot", "Aufstampfen mit dem linken Fuß"), description: t("The mirror of the right stamp: the left knee lifted, the weight on the right foot.", "Das Spiegelbild des rechten Aufstampfens: das linke Knie gehoben, das Gewicht auf dem rechten Fuß."), bones: { ...clone(stand), "thigh-l": { angleDeg: 38 }, "shank-l": { angleDeg: -62 }, "foot-l": { angleDeg: 14 }, "thigh-r": { angleDeg: -1 }, "shank-r": { angleDeg: -2 } }, ground: "auto", contacts: ["heel-r", "ball-r"], thermal: { activityMet: 2.3, breathing: "walking", regions: bind({ gloves: 0.25 }), emitters: { "body-plume": 0.7 } }, loads: [] },
  ];
  for (const name of walkNames) {
    const p = clone(base.poses.find((q: Json) => q.id === name));
    for (const k of Object.keys(p.bones)) if (/^(arm|forearm|hand|bag)/.test(k) || /^(index|middle|ring|little|thumb)-/.test(k)) delete p.bones[k];
    Object.assign(p.bones, clone(pocket), { "clavicle-l": { angleDeg: 5 }, "clavicle-r": { angleDeg: 5 } });
    p.thermal = { activityMet: 3.0, breathing: "walking", regions: bind({ gloves: 0.25 }), emitters: { "body-plume": 0.75 } };
    p.loads = [];
    p.title = t(p.title.en, p.title.de);
    p.description = t(`${p.description.en.replace(/\.$/, "")}, hands in the parka pockets.`, `${p.description.de.replace(/\.$/, "")}, die Hände in den Parkataschen.`);
    poses.push(p);
  }
  a.poses = poses;

  const walk = clone(base.clips.find((c: Json) => c.id === "walk-cycle"));
  walk.id = "walk-hands-in-pockets";
  walk.title = t("Walking with the hands in the pockets", "Gehen mit den Händen in den Taschen");
  walk.description = t("A brisk heel-to-toe gait at 3.6 km/h with double support; the arms do not swing because both hands stay in the parka pockets, so only the shoulders counter-rotate a little and the long parka swings with the thighs.", "Zügiger Gang von der Ferse zum Ballen mit 3,6 km/h und Doppelstandphase; die Arme schwingen nicht, weil beide Hände in den Parkataschen bleiben, nur die Schultern drehen leicht gegen und der lange Parka schwingt mit den Oberschenkeln.");
  for (const k of walk.keys) {
    for (const b of Object.keys(k.bones ?? {})) if (/^(arm|forearm|hand|bag)/.test(b)) delete k.bones[b];
    Object.assign((k.bones ??= {}), clone(pocket));
  }
  walk.tracks = [{ target: "activityMet", ease: "step", keys: [{ t: 0, v: 3.0 }, { t: walk.durationS, v: 3.0 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: walk.durationS, v: 1 }] }];
  const contactsAll = stance.map((anchor) => ({ anchor, fromS: 0, toS: 0 }));
  const idle = {
    id: "idle-breathing", title: t("Waiting in the frost", "Warten im Frost"), description: t("Quiet standing with the hands in the pockets; the chest rises with every breath and each exhalation leaves the nose as a small cloud of condensed vapour.", "Ruhiges Stehen mit den Händen in den Taschen; der Brustkorb hebt sich mit jedem Atemzug und jedes Ausatmen verlässt die Nase als kleine Wolke kondensierten Dampfes."),
    durationS: 4.2, fps: 30, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "hands-in-pockets", ease: "smooth" }, { t: 2.1, pose: "hands-in-pockets", ease: "smooth", bones: { "clavicle-l": { angleDeg: 7.2 }, "clavicle-r": { angleDeg: 7.2 }, head: { angleDeg: -2 } } }, { t: 4.2, pose: "hands-in-pockets", ease: "smooth" }],
    contacts: contactsAll.map((c) => ({ ...c, toS: 4.2 })), phases: [{ id: "waiting", fromS: 0, toS: 4.2, title: t("Waiting at the tram stop", "Warten an der Haltestelle") }], events: [],
    tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4.2, v: 0 }] }],
  };
  const D = 2.4;
  const stamp = {
    id: "stamp-feet", title: t("Stamping the feet warm", "Füße warm stampfen"), description: t("Waiting in the frost with the hands in the pockets and stamping right, then left, to bring warmth back into the feet: each boot is lifted for 0.5 s and set down hard.", "Warten im Frost mit den Händen in den Taschen und abwechselnd rechts, dann links aufstampfen, damit die Füße wieder warm werden: jeder Stiefel wird 0,5 s gehoben und kräftig aufgesetzt."),
    durationS: D, fps: 30, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "hands-in-pockets", ease: "smooth" }, { t: 0.4, pose: "stamping-feet", ease: "smooth" }, { t: 0.7, pose: "hands-in-pockets", ease: "in" }, { t: 1.2, pose: "hands-in-pockets", ease: "smooth" }, { t: 1.6, pose: "stamping-left", ease: "smooth" }, { t: 1.9, pose: "hands-in-pockets", ease: "in" }, { t: D, pose: "hands-in-pockets", ease: "smooth" }],
    contacts: [{ anchor: "heel-l", fromS: 0, toS: 1.25 }, { anchor: "ball-l", fromS: 0, toS: 1.25 }, { anchor: "heel-l", fromS: 1.85, toS: D }, { anchor: "ball-l", fromS: 1.85, toS: D }, { anchor: "heel-r", fromS: 0, toS: 0.05 }, { anchor: "ball-r", fromS: 0, toS: 0.05 }, { anchor: "heel-r", fromS: 0.65, toS: D }, { anchor: "ball-r", fromS: 0.65, toS: D }],
    phases: [{ id: "stamp-right", fromS: 0, toS: 1.2, title: t("Stamping the right foot", "Rechten Fuß aufstampfen") }, { id: "stamp-left", fromS: 1.2, toS: D, title: t("Stamping the left foot", "Linken Fuß aufstampfen") }],
    events: [{ t: 0.05, kind: "contact-off", title: t("Right boot lifted", "Rechter Stiefel gehoben"), target: "heel-r" }, { t: 0.65, kind: "contact-on", title: t("Right boot stamps down", "Rechter Stiefel stampft auf"), target: "heel-r" }, { t: 1.25, kind: "contact-off", title: t("Left boot lifted", "Linker Stiefel gehoben"), target: "heel-l" }, { t: 1.85, kind: "contact-on", title: t("Left boot stamps down", "Linker Stiefel stampft auf"), target: "heel-l" }],
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 2.2 }, { t: D, v: 2.2 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: D, v: 1 }] }],
  };
  a.clips = [walk, idle, stamp];
  a.presentation = { defaultPose: "hands-in-pockets", defaultClip: "walk-hands-in-pockets", defaultScale: 100 };
  void framesOf;
}
