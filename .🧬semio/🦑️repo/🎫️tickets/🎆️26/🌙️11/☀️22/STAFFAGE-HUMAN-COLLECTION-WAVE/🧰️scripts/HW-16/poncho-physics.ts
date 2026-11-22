/** 🌡️ The physical scenario of `person-in-rain-poncho`: identity texts, materials, clothing regions after ISO 9920 with a vapour-tight PVC poncho, the cool rain reference environment with wetted outer surfaces, the anthropometry of a 1.67 m adult, poses under the poncho, the rain walk and the waiting clip, and the provenance.
 *
 * @see ./poncho.ts
 */
import { clone, renameRegion, t, type Json } from "./lib.ts";
import { dubois } from "./winter-physics.ts";

const CLO = 0.155;
const vapour = (r: number, im = 0.38) => Math.round((r / (0.0165 * im)) * 10) / 10;
const layer = (id: string, en: string, de: string, material: string, thicknessM: number, clo: number, heatCapacityJM2K: number, vapourResistanceM2PaW?: number) => ({ id, title: t(en, de), kind: "clothing", material, thicknessM, resistanceM2KW: Math.round(clo * CLO * 1e5) / 1e5, vapourResistanceM2PaW: vapourResistanceM2PaW ?? vapour(clo * CLO), heatCapacityJM2K, source: "authored-estimate", reference: "iso-9920-2007" });
const SKIN_NODE = 11000;
const capacity = (layers: Json[]) => SKIN_NODE + layers.reduce((s, l) => s + l.heatCapacityJM2K, 0);

const MATERIALS: Json[] = [
  { id: "skin", title: t("Skin", "Haut"), emissivity: 0.98, solarAbsorptivity: 0.6, source: "literature", reference: "ashrae-2021" },
  { id: "hair", title: t("Hair of the eyebrows", "Haar der Augenbrauen"), emissivity: 0.98, solarAbsorptivity: 0.85, source: "authored-estimate" },
  { id: "pvc-poncho", title: t("PVC-coated rain poncho, yellow", "PVC-beschichteter Regenponcho, gelb"), emissivity: 0.9, solarAbsorptivity: 0.35, densityKgM3: 1300, specificHeatJKgK: 1000, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "fleece-jacket", title: t("Fleece and softshell jacket", "Fleece- und Softshelljacke"), emissivity: 0.92, solarAbsorptivity: 0.7, specificHeatJKgK: 1350, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "wool-trouser", title: t("Cotton twill trousers", "Hose aus Baumwollköper"), emissivity: 0.93, solarAbsorptivity: 0.7, specificHeatJKgK: 1300, source: "literature", reference: "ashrae-2021" },
  { id: "rubber-boot", title: t("Natural rubber wellington boot", "Gummistiefel aus Naturkautschuk"), emissivity: 0.94, solarAbsorptivity: 0.85, densityKgM3: 1100, specificHeatJKgK: 1900, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "cotton-underwear", title: t("Cotton undershirt and briefs", "Baumwollunterwäsche"), emissivity: 0.93, solarAbsorptivity: 0.5, specificHeatJKgK: 1300, source: "authored-estimate" },
  { id: "cotton-shirt", title: t("Cotton long-sleeve shirt", "Langarmshirt aus Baumwolle"), emissivity: 0.93, solarAbsorptivity: 0.5, specificHeatJKgK: 1300, source: "authored-estimate" },
  { id: "wool-sock", title: t("Wool-blend socks", "Socken aus Wollmischung"), emissivity: 0.93, solarAbsorptivity: 0.6, specificHeatJKgK: 1360, source: "authored-estimate" },
  { id: "bone", title: t("Bone", "Knochen"), emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" },
];

const REFERENCES: Json[] = [
  { id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; codes 17170 (walking, 2.0-2.5 mph, 3.0 MET) and 07040 (standing quietly, 1.3 MET)." },
  { id: "din-33402-2-2020", citation: "DIN 33402-2:2020-12 Ergonomie, Körpermaße des Menschen, Teil 2: Werte; body heights of German adults 18-65 years (women P50 1.66 m, men P50 1.79 m)." },
  { id: "ansur-ii-2012", citation: "Gordon, C. C. et al. (2014). 2012 Anthropometric Survey of U.S. Army Personnel: Methods and Summary Statistics (ANSUR II). Natick/TR-15/007; stature and mass percentiles." },
  { id: "havenith-1999", citation: "Havenith, G., Holmér, I. and Parsons, K. (2002). Personal factors in thermal comfort assessment: clothing properties and metabolic heat production. Energy and Buildings 34(6), 581-591; vapour resistance of impermeable layers and the effect of wetting." },
];

export function ponchoPhysics(a: Json, base: Json): void {
  renameRegion(a, "coat", "poncho");
  renameRegion(a, "shoes", "wellies");
  for (const sc of ["50", "100", "200", "500"]) for (const s of a.volumes[sc].thermal) s.layer = ({ overcoat: "softshell", jacket: "fleece" } as Record<string, string>)[s.layer] ?? s.layer;

  a.title = t("Person in Rain Poncho", "Person im Regenponcho");
  a.summary = t(
    "A man of 1.83 m walking through steady rain under a hooded yellow PVC poncho that hangs from the shoulders to just above the knees and hides the arms, over a fleece and softshell jacket, with trousers and wellington boots. The poncho and hood stay wet, so their outer surface cools by evaporation, and the vapour-tight film keeps the moisture of the body inside.",
    "Ein Mann von 1,83 m geht durch Dauerregen unter einem gelben PVC-Regenponcho mit Kapuze, der von den Schultern bis knapp über die Knie fällt und die Arme verdeckt, darunter Fleece- und Softshelljacke, Hose und Gummistiefel. Poncho und Kapuze bleiben nass, ihre Außenfläche kühlt durch Verdunstung, und die dampfdichte Folie hält die Feuchte des Körpers innen.",
  );
  a.tags = ["human", "adult", "rain", "poncho", "hood", "wellington-boots", "gait", "wet-surface"];
  a.provenance.author = "Semio staffage human collection worker HW-16";
  a.provenance.method = "handcrafted";
  a.provenance.created = "2026-11-22";
  a.provenance.references = [...a.provenance.references.filter((r: Json) => !REFERENCES.some((n) => n.id === r.id)), ...REFERENCES];
  a.provenance.notes = t(
    "Rig, body and gait are derived from the walking commuter and rescaled by 1.03 to a stature of 1.83 m (1.84 m in boots, 1.88 m to the top of the hood), about the 75th percentile of the men of DIN 33402-2 and ANSUR II, with 78.7 kg (the commuter's de Leva male segment masses scaled by 1.03³, BMI 23.4). The poncho is a 6 mm revolved shell (declared as a sheet) about the torso, the hood a loft about the head that leaves the face free, the wellington shafts lofts about the shanks; every new part is drawn as the projection of its own solids at each scale.",
    "Rig, Körper und Gang stammen vom gehenden Pendler und sind mit 1,03 auf eine Körpergröße von 1,83 m (1,84 m in Stiefeln, 1,88 m bis zum Scheitel der Kapuze) skaliert, etwa das 75. Perzentil der Männer nach DIN 33402-2 und ANSUR II, mit 78,7 kg (die männlichen Segmentmassen des Pendlers nach de Leva mit 1,03³ skaliert, BMI 23,4). Der Poncho ist eine 6 mm dünne Rotationsschale (als Fläche deklariert) um den Rumpf, die Kapuze ein Loft um den Kopf, der das Gesicht freilässt, die Stiefelschäfte Lofts um die Unterschenkel; jedes neue Teil ist in jedem Maßstab als Projektion seiner eigenen Körper gezeichnet.",
  );
  a.materials = MATERIALS;
  const extra: Record<string, number> = { torso: 0.35, lumbar: 0.15, head: 0.08, "shank-l": 0.55, "shank-r": 0.55, "foot-l": 0.2, "foot-r": 0.2 };
  for (const b of a.rig.bones) if (b.mass && extra[b.id]) b.mass.kg = Math.round((b.mass.kg + extra[b.id]!) * 1000) / 1000;
  for (const b of a.rig.bones) if (b.id === "torso") b.title = t("Torso under the poncho", "Rumpf unter dem Poncho");

  a.thermal.environment = { airTempC: 9, meanRadiantTempC: 8.5, skyTempC: 7, groundTempC: 9, relativeHumidity: 0.95, windMS: 3, windDirectionDeg: 180, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 };
  a.thermal.baseMet = 1.3;
  const kg = Math.round(72 * 1.03 ** 3 * 10) / 10;
  a.thermal.body = { ...a.thermal.body, massKg: kg, skinAreaM2: dubois(kg, 1.833), basalMetabolismW: 85, source: "authored-estimate", reference: "gagge-1971" };
  a.thermal.respiration.exhaledTempC = 34;
  a.thermal.respiration.patterns = [
    { id: "rest-standing", title: t("Quiet standing", "Ruhiges Stehen"), rateBpm: 13, tidalVolumeL: 0.55, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.13, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
    { id: "walking", title: t("Walking at 3.7 km/h", "Gehen mit 3,7 km/h"), rateBpm: 19, tidalVolumeL: 1.3, inhaleFraction: 0.45, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.03, route: "nose-and-mouth", source: "authored-estimate", reference: "icrp-66-1994" },
  ];
  const R = (id: string) => a.thermal.regions.find((r: Json) => r.id === id);
  const tee = layer("tee", "Cotton undershirt", "Baumwollunterhemd", "cotton-underwear", 0.0008, 0.09, 180);
  const shirt = layer("shirt", "Cotton long-sleeve shirt", "Langarmshirt aus Baumwolle", "cotton-shirt", 0.0012, 0.2, 160);
  const fleece = layer("fleece", "Fleece pullover", "Fleecepullover", "fleece-jacket", 0.005, 0.36, 300);
  const soft = layer("softshell", "Softshell jacket", "Softshelljacke", "fleece-jacket", 0.003, 0.25, 420);
  const film = layer("poncho", "PVC rain poncho with the enclosed air", "PVC-Regenponcho mit eingeschlossener Luft", "pvc-poncho", 0.006, 0.12, 380, 500);
  Object.assign(R("poncho"), { title: t("Poncho over torso and arms", "Poncho über Rumpf und Armen"), material: "pvc-poncho", layers: [tee, shirt, fleece, soft, film], heatCapacityJM2K: capacity([tee, shirt, fleece, soft, film]), skyViewFactor: 0.5, reference: "havenith-1999", body: { segment: "trunk", metabolicShare: 0.56, skinFraction: 0.49 } });
  const hoodFilm = layer("hood", "PVC hood over the hair", "PVC-Kapuze über dem Haar", "pvc-poncho", 0.006, 0.3, 300, 500);
  const hood = { id: "hood", title: t("Hood", "Kapuze"), kind: "clothing", model: "body", material: "pvc-poncho", areaM2: 0.09, characteristicLengthM: 0.17, geometry: "sphere", skyViewFactor: 0.8, layers: [hoodFilm], heatCapacityJM2K: capacity([hoodFilm]), body: { segment: "head", metabolicShare: 0.04, skinFraction: 0.045 }, source: "authored-estimate", reference: "havenith-1999" };
  a.thermal.regions.splice(1, 0, hood);
  Object.assign(R("face-skin"), { title: t("Face in the hood opening", "Gesicht in der Kapuzenöffnung"), body: { segment: "head", metabolicShare: 0.06, skinFraction: 0.05 }, heatCapacityJM2K: SKIN_NODE });
  Object.assign(R("hand-skin"), { title: t("Bare hands under the poncho", "Bloße Hände unter dem Poncho"), heatCapacityJM2K: SKIN_NODE });
  const briefs = layer("briefs", "Cotton briefs", "Baumwollunterhose", "cotton-underwear", 0.0008, 0.04, 100);
  const trousers = layer("trousers", "Cotton twill trousers", "Hose aus Baumwollköper", "wool-trouser", 0.0015, 0.2, 360);
  Object.assign(R("trousers"), { title: t("Trousers between poncho and boots", "Hose zwischen Poncho und Stiefeln"), layers: [briefs, trousers], heatCapacityJM2K: capacity([briefs, trousers]), body: { segment: "leg", metabolicShare: 0.27, skinFraction: 0.295 } });
  const socks = layer("socks", "Wool-blend socks", "Socken aus Wollmischung", "wool-sock", 0.003, 0.08, 150);
  const welly = layer("shoe", "Rubber wellington boot, unlined", "Gummistiefel, ungefüttert", "rubber-boot", 0.004, 0.15, 2100, 2000);
  Object.assign(R("wellies"), { title: t("Socks and wellington boots", "Socken und Gummistiefel"), material: "rubber-boot", layers: [socks, welly], heatCapacityJM2K: capacity([socks, welly]), body: { segment: "foot", metabolicShare: 0.03, skinFraction: 0.07 } });
  R("hand-skin").body = { segment: "hand", metabolicShare: 0.04, skinFraction: 0.05 };
  a.thermal.assumptions = [
    t("Skin area after DuBois and DuBois (1916) for 78.7 kg and 1.83 m; skin fractions 0.05 face, 0.045 head under the hood, 0.05 hands, 0.49 trunk and arms under the poncho, 0.295 legs, 0.07 feet (rule-of-nines proportions, authored).", "Hautfläche nach DuBois und DuBois (1916) für 78,7 kg und 1,83 m; Hautanteile 0,05 Gesicht, 0,045 Kopf unter der Kapuze, 0,05 Hände, 0,49 Rumpf und Arme unter dem Poncho, 0,295 Beine, 0,07 Füße (Neunerregel-Proportionen, geschätzt)."),
    t("Clothing layers are local insulations in clo after ISO 9920 (1 clo = 0.155 m2K/W, enclosed air included), referred to the outer area. The poncho layer carries the air it encloses (0.12 clo) and, as a vapour-tight PVC film, a vapour resistance of 500 m2Pa/W instead of R/(0.0165 x i_m); the rubber boot 2000 m2Pa/W (Havenith 2002).", "Kleidungsschichten sind lokale Isolationen in clo nach ISO 9920 (1 clo = 0,155 m2K/W, eingeschlossene Luft enthalten), bezogen auf die Außenfläche. Die Ponchoschicht trägt die eingeschlossene Luft (0,12 clo) und als dampfdichte PVC-Folie einen Dampfwiderstand von 500 m2Pa/W statt R/(0,0165 x i_m); der Gummistiefel 2000 m2Pa/W (Havenith 2002)."),
    t("Rain wets the outer surfaces: poncho, hood and boots carry a wettedness of 0.85 to 0.9 and evaporate from their outer face, the trousers below the poncho 0.4, the face 0.2; in 95 % relative humidity at 9 °C the evaporation is weak and the wet film mostly couples the surface to the wet-bulb temperature.", "Regen benetzt die Außenflächen: Poncho, Kapuze und Stiefel tragen eine Benetzung von 0,85 bis 0,9 und verdunsten an ihrer Außenseite, die Hose unter dem Poncho 0,4, das Gesicht 0,2; bei 95 % relativer Feuchte und 9 °C ist die Verdunstung schwach und der Wasserfilm koppelt die Oberfläche vor allem an die Feuchtkugeltemperatur."),
    t("Standing in the rain 1.3 met, walking at 3.7 km/h 3.0 met (Ainsworth 2011, codes 07040 and 17170; ISO 8996 Annex B); region heat capacity = skin node 11 kJ/(m2K) plus the areal heat capacity of the garments.", "Stehen im Regen 1,3 met, Gehen mit 3,7 km/h 3,0 met (Ainsworth 2011, Codes 07040 und 17170; ISO 8996 Anhang B); Wärmekapazität der Region = Hautknoten 11 kJ/(m2K) plus die flächenbezogene Wärmekapazität der Kleidung."),
  ];
  posesAndClips(a, base);
}

const REGIONS = ["face-skin", "hood", "hand-skin", "poncho", "trousers", "wellies"] as const;
const wet = { "face-skin": 0.2, hood: 0.9, "hand-skin": 0, poncho: 0.85, trousers: 0.4, wellies: 0.9 } as Record<string, number>;
const bind = (exposure: Partial<Record<(typeof REGIONS)[number], number>>) => Object.fromEntries(REGIONS.map((r) => [r, { exposedFraction: exposure[r] ?? 1, sunlitFraction: 0, ...(wet[r] ? { wettedness: wet[r] } : {}) }]));

function posesAndClips(a: Json, base: Json): void {
  const stance = ["heel-l", "ball-l", "heel-r", "ball-r"];
  const hang = { "arm-l": { angleDeg: 3 }, "forearm-l": { angleDeg: 16 }, "hand-l": { angleDeg: 4 }, "arm-r": { angleDeg: 4 }, "forearm-r": { angleDeg: 18 }, "hand-r": { angleDeg: 4 } };
  const under = bind({ "hand-skin": 0.3 });
  const mirror: Record<string, string> = { "contact-left": "contact-right", "loading-left": "loading-right", "passing-left": "passing-right", "contact-right": "contact-left", "loading-right": "loading-left", "passing-right": "passing-left" };
  const swing = (v: number | undefined, rest: number) => Math.round((rest + 0.4 * ((v ?? rest) - rest)) * 100) / 100;
  const poses: Json[] = [
    { id: "standing", title: t("Standing in the rain", "Stehend im Regen"), description: t("Upright and still under the poncho, hood up, the arms hanging inside the cape, the boots side by side.", "Aufrecht und ruhig unter dem Poncho, die Kapuze auf, die Arme hängen im Umhang, die Stiefel nebeneinander."), bones: { ...hang, neck: { angleDeg: 2 }, head: { angleDeg: 1 }, torso: { angleDeg: -0.6 }, lumbar: { angleDeg: -0.4 } }, ground: "auto", contacts: stance, thermal: { activityMet: 1.3, breathing: "rest-standing", regions: under, emitters: { "body-plume": 0.6 } }, loads: [] },
    { id: "head-bowed", title: t("Head bowed against the rain", "Kopf gegen den Regen gesenkt"), description: t("Standing with the chin tucked and the upper body leaning a little forward, so the hood brim shields the face from the driving rain.", "Stehend mit eingezogenem Kinn und leicht vorgeneigtem Oberkörper, damit der Kapuzenrand das Gesicht vor dem Schlagregen schützt."), bones: { ...hang, neck: { angleDeg: -16 }, head: { angleDeg: -12 }, torso: { angleDeg: -6 }, lumbar: { angleDeg: -4 }, "clavicle-l": { angleDeg: 4 }, "clavicle-r": { angleDeg: 4 } }, ground: "auto", contacts: stance, thermal: { activityMet: 1.3, breathing: "rest-standing", regions: bind({ "hand-skin": 0.3, "face-skin": 0.8 }), emitters: { "body-plume": 0.6 } }, loads: [] },
  ];
  for (const name of Object.keys(mirror)) {
    const p = clone(base.poses.find((q: Json) => q.id === name));
    const other = base.poses.find((q: Json) => q.id === mirror[name]);
    for (const k of Object.keys(p.bones)) if (/^(arm|forearm|hand|bag)/.test(k) || /^(index|middle|ring|little|thumb)-/.test(k)) delete p.bones[k];
    Object.assign(p.bones, { "arm-l": { angleDeg: swing(base.poses.find((q: Json) => q.id === name).bones["arm-l"]?.angleDeg, 3) }, "forearm-l": { angleDeg: swing(base.poses.find((q: Json) => q.id === name).bones["forearm-l"]?.angleDeg, 16) }, "hand-l": { angleDeg: 4 }, "arm-r": { angleDeg: swing(other.bones["arm-l"]?.angleDeg, 4) }, "forearm-r": { angleDeg: swing(other.bones["forearm-l"]?.angleDeg, 18) }, "hand-r": { angleDeg: 4 } });
    p.thermal = { activityMet: 3.0, breathing: "walking", regions: under, emitters: { "body-plume": 0.75 } };
    p.loads = [];
    p.description = t(`${p.description.en.replace(/\.$/, "")}; the arms swing only a little inside the poncho.`, `${p.description.de.replace(/\.$/, "")}; die Arme schwingen im Poncho nur wenig.`);
    poses.push(p);
  }
  a.poses = poses;

  const walk = clone(a.clips.find((c: Json) => c.id === "walk-cycle"));
  walk.id = "walk-in-rain";
  walk.title = t("Walking through the rain", "Gehen durch den Regen");
  walk.description = t("Heel-to-toe gait at 3.7 km/h with double support in wellington boots; the arms swing only a little inside the poncho and the stride is a touch shorter than in shoes.", "Gang von der Ferse zum Ballen mit 3,7 km/h und Doppelstandphase in Gummistiefeln; die Arme schwingen im Poncho nur wenig und der Schritt ist etwas kürzer als in Schuhen.");
  const half = walk.durationS / 2;
  const source = clone(walk.keys);
  walk.keys = source.map((k: Json) => {
    const bones = { ...(k.bones ?? {}) };
    for (const b of Object.keys(bones)) if (/^(arm|forearm|hand|bag)/.test(b)) delete bones[b];
    const arm = (time: number, bone: string) => {
      const tt = ((time % walk.durationS) + walk.durationS) % walk.durationS;
      return source.reduce((best: Json, key: Json) => (Math.abs(key.t - tt) < Math.abs(best.t - tt) ? key : best)).bones?.[bone]?.angleDeg;
    };
    Object.assign(bones, { "arm-l": { angleDeg: swing(arm(k.t, "arm-l"), 3) }, "forearm-l": { angleDeg: swing(arm(k.t, "forearm-l"), 16) }, "hand-l": { angleDeg: 4 }, "arm-r": { angleDeg: swing(arm(k.t + half, "arm-l"), 4) }, "forearm-r": { angleDeg: swing(arm(k.t + half, "forearm-l"), 18) }, "hand-r": { angleDeg: 4 } });
    return { ...k, bones };
  });
  walk.tracks = [{ target: "activityMet", ease: "step", keys: [{ t: 0, v: 3.0 }, { t: walk.durationS, v: 3.0 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: walk.durationS, v: 1 }] }];
  const D = 4.4;
  const idle = {
    id: "idle-breathing", title: t("Waiting in the rain", "Warten im Regen"), description: t("Quiet standing under the poncho at a bus stop; the chest rises with every breath and the head turns a little to look down the street.", "Ruhiges Stehen unter dem Poncho an einer Haltestelle; der Brustkorb hebt sich mit jedem Atemzug und der Kopf dreht sich ein wenig die Straße hinunter."),
    durationS: D, fps: 30, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "standing", ease: "smooth" }, { t: D / 2, pose: "standing", ease: "smooth", bones: { torso: { angleDeg: 0.4 }, neck: { angleDeg: 1 }, head: { angleDeg: 3 }, "clavicle-l": { angleDeg: 1.5 }, "clavicle-r": { angleDeg: 1.5 } } }, { t: D, pose: "standing", ease: "smooth" }],
    contacts: stance.map((anchor) => ({ anchor, fromS: 0, toS: D })), phases: [{ id: "waiting", fromS: 0, toS: D, title: t("Waiting at the bus stop", "Warten an der Haltestelle") }], events: [],
    tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: D, v: 0 }] }],
  };
  a.clips = [walk, idle];
  a.presentation = { defaultPose: "standing", defaultClip: "walk-in-rain", defaultScale: 100 };
}
