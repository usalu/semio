/** 🍂️ Design of the field maple (Acer campestre): a small open-grown tree of about 40 years, 10 m high, its short stem forking at 1.9 m into three ascending stems under a dense, rounded and slightly irregular crown of about 8 m; corky bark fissured into small oblong plates; small leaves of 5 to 8 cm with three to five blunt lobes; samaras spread horizontally.
 * Every station, limb, ring and pose value below is chosen by hand for this species.
 * @see ./design.ts
 * @see https://www.gbif.org/species/3189863 */
import type { Design } from "./design.ts";
import { REFERENCES, assumptions, corkMarks, drawingTexts, materials, t, volumeTexts, type Voice } from "./design.ts";
import type { LimbSpec, Species } from "./model.ts";

const side = (f: number, daz: number, e0: number, e1: number, L: number, r0: number, r1: number, bend = 0) => ({ f, daz, e0, e1, L, r0, r1, bend });
const limb = (id: string, parent: string, at: number, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend: number, en: string, de: string, sides: LimbSpec["sides"]): LimbSpec => ({ id, parent, title: t(en, de), at, az, e0, e1, L, r0, r1, bend, limit: 10, sides });

const tree: Species = {
  id: "field-maple",
  seed: 2203,
  title: t("Field Maple", "Feldahorn"),
  summary: t("A small open-grown field maple (Acer campestre) of about 10 m: a short corky stem forks low into three ascending stems that carry a dense, rounded crown of small, blunt-lobed leaves.", "Ein kleiner, frei gewachsener Feldahorn (Acer campestre) von etwa 10 m: Ein kurzer, korkiger Stamm teilt sich tief in drei aufsteigende Stämmlinge, die eine dichte, rundliche Krone aus kleinen, stumpf gelappten Blättern tragen."),
  distinction: t("A small field maple (Acer campestre) of about 10 m whose short stem forks at 1.9 m into three ascending stems; dense, rounded, slightly irregular crown of small blunt-lobed leaves, corky plated bark, horizontally spread samaras.", "Ein kleiner Feldahorn (Acer campestre) von etwa 10 m, dessen kurzer Stamm sich in 1,9 m in drei aufsteigende Stämmlinge teilt; dichte, rundliche, leicht unregelmäßige Krone aus kleinen, stumpf gelappten Blättern, korkige, gefelderte Borke, waagerecht gespreizte Flügelfrüchte."),
  tags: ["tree", "deciduous", "broadleaf", "small-tree", "multi-stem", "maple", "hedgerow", "wind", "transpiration"],
  notes: t("Rig, model and drawings are authored for an open-grown field maple of about 40 years: stem 1.9 m, three stems to 7 to 8.9 m, eight scaffold limbs, crown 10 m high and 8 m wide. Size and habit follow Roloff (2010) and Mitchell (1974) for Acer campestre (usually 8 to 15 m, often low-forked); wood properties, masses and conductances are authored estimates.", "Rig, Modell und Zeichnungen sind für einen frei gewachsenen Feldahorn von etwa 40 Jahren entworfen: Stamm 1,9 m, drei Stämmlinge bis 7 bis 8,9 m, acht Gerüstäste, Krone 10 m hoch und 8 m breit. Größe und Wuchs folgen Roloff (2010) und Mitchell (1974) für Acer campestre (meist 8 bis 15 m, oft tief gezwieselt); Holzeigenschaften, Massen und Leitfähigkeiten sind Autorenschätzungen."),
  flare: [[0, 0.31], [0.08, 0.27], [0.2, 0.225], [0.38, 0.196], [0.6, 0.182], [0.9, 0.173], [1.2, 0.168]],
  buttress: [[30, 1], [118, 0.9], [205, 1.04], [292, 0.94]],
  axes: [
    { id: "stem", parent: "root-plate", title: t("Stem", "Stamm"), limit: 3, region: "trunk-bark", rows: [[0, 0.3, 0, 0.175], [0.012, 0.9, 0.006, 0.17], [0.026, 1.4, 0.008, 0.165], [0.04, 1.9, 0.01, 0.16]] },
    { id: "stem-a", parent: "stem", title: t("Stem ahead left", "Stämmling vorn links"), limit: 5, region: "trunk-bark", rows: [[0.04, 1.9, 0.01, 0.115], [0.409, 3.0, 0.268, 0.1], [0.761, 4.3, 0.515, 0.085], [1.064, 5.8, 0.728, 0.068], [1.269, 7.4, 0.871, 0.05]] },
    { id: "stem-b", parent: "stem", title: t("Stem behind", "Stämmling hinten"), limit: 5, region: "trunk-bark", rows: [[0.04, 1.9, 0.01, 0.11], [-0.443, 2.9, 0.139, 0.095], [-0.878, 4.1, 0.256, 0.08], [-1.235, 5.5, 0.352, 0.064], [-1.457, 7.0, 0.411, 0.046]] },
    { id: "stem-c", parent: "stem", title: t("Stem to the right, leading", "Stämmling rechts, führend"), limit: 5, region: "trunk-bark", rows: [[0.04, 1.9, 0.01, 0.125], [0.087, 3.2, -0.164, 0.11], [0.133, 4.6, -0.338, 0.095], [0.17, 6.0, -0.473, 0.08], [0.19, 7.5, -0.55, 0.064], [0.2, 8.9, -0.589, 0.048]] },
  ],
  limbs: [
    limb("limb-low", "stem", 1.55, 100, 24, 12, 2.7, 0.07, 0.03, 8, "Low limb on the stem", "Tiefer Ast am Stamm", [side(0.42, -36, 14, 6, 1.2, 0.032, 0.013), side(0.72, 32, 16, 6, 1.0, 0.026, 0.012)]),
    limb("limb-a1", "stem-a", 3.3, 10, 30, 16, 2.9, 0.065, 0.028, -6, "Stem ahead left, lower limb", "Stämmling vorn links, unterer Ast", [side(0.42, -34, 18, 8, 1.2, 0.03, 0.013), side(0.72, 36, 20, 8, 1.0, 0.026, 0.012)]),
    limb("limb-a2", "stem-a", 5.0, 75, 38, 26, 2.3, 0.05, 0.025, 6, "Stem ahead left, upper limb", "Stämmling vorn links, oberer Ast", [side(0.55, 34, 26, 14, 1.0, 0.024, 0.012)]),
    limb("limb-b1", "stem-b", 3.1, 200, 28, 15, 3.0, 0.062, 0.028, 6, "Stem behind, lower limb", "Stämmling hinten, unterer Ast", [side(0.42, 36, 18, 8, 1.2, 0.03, 0.013), side(0.72, -34, 20, 8, 1.0, 0.026, 0.012)]),
    limb("limb-b2", "stem-b", 4.8, 130, 40, 28, 2.2, 0.048, 0.024, -6, "Stem behind, upper limb", "Stämmling hinten, oberer Ast", [side(0.55, -34, 26, 14, 1.0, 0.024, 0.012)]),
    limb("limb-c1", "stem-c", 3.7, 245, 30, 18, 2.8, 0.06, 0.027, -6, "Leading stem, lower limb", "Führender Stämmling, unterer Ast", [side(0.42, 34, 18, 8, 1.2, 0.03, 0.013), side(0.72, -36, 20, 8, 1.0, 0.026, 0.012)]),
    limb("limb-c2", "stem-c", 5.9, 320, 42, 30, 2.2, 0.046, 0.023, 6, "Leading stem, middle limb", "Führender Stämmling, mittlerer Ast", [side(0.55, -34, 28, 16, 0.9, 0.022, 0.011)]),
    limb("limb-c3", "stem-c", 7.3, 170, 50, 40, 1.6, 0.036, 0.022, 0, "Leading stem, upper limb", "Führender Stämmling, oberer Ast", []),
  ],
  crown: { base: 1.3, widest: 4.7, top: 10.25, rx: 4.15, rz: 3.95, lowerExp: 1.3, upperP: 1.9, upperQ: 0.68, lumps: [[2, 0.04, 0.9], [5, 0.025, 0.3]], cx: 0 },
  scales: {
    50: {
      rings: [
        { y: 0.03, n: 9, s: 0.12, tilt: 34, k: 1.0 }, { y: 0.1, n: 15, s: 0.14, tilt: 26, k: 1.0 }, { y: 0.19, n: 19, s: 0.15, tilt: 18, k: 1.0 }, { y: 0.29, n: 22, s: 0.155, tilt: 10, k: 1.0 },
        { y: 0.39, n: 23, s: 0.158, tilt: 2, k: 1.0 }, { y: 0.49, n: 22, s: 0.158, tilt: -5, k: 1.0 }, { y: 0.59, n: 21, s: 0.154, tilt: -12, k: 1.0 }, { y: 0.68, n: 18, s: 0.15, tilt: -19, k: 1.0 },
        { y: 0.77, n: 15, s: 0.145, tilt: -26, k: 1.0 }, { y: 0.85, n: 11, s: 0.135, tilt: -34, k: 1.0 }, { y: 0.92, n: 7, s: 0.125, tilt: -43, k: 1.0 },
      ],
      shade: [{ y: 0.15, n: 4, k: 0.5, s: 0.17 }, { y: 0.35, n: 5, k: 0.52, s: 0.19 }, { y: 0.55, n: 5, k: 0.52, s: 0.19 }, { y: 0.74, n: 3, k: 0.42, s: 0.16 }],
      core: { n: 6, from: 0.2, to: 0.78, k: 0.74 },
      cap: [0.2, 0.13], tufts: 0.4, jitter: 1,
    },
    100: {
      rings: [
        { y: 0.04, n: 7, s: 0.135, tilt: 32, k: 1.0, flat: 0.85 }, { y: 0.13, n: 12, s: 0.158, tilt: 22, k: 1.0, flat: 0.85 }, { y: 0.25, n: 15, s: 0.172, tilt: 12, k: 1.0, flat: 0.85 }, { y: 0.38, n: 17, s: 0.177, tilt: 2, k: 1.0, flat: 0.85 },
        { y: 0.51, n: 17, s: 0.177, tilt: -7, k: 1.0, flat: 0.85 }, { y: 0.63, n: 15, s: 0.17, tilt: -15, k: 1.0, flat: 0.85 }, { y: 0.75, n: 12, s: 0.16, tilt: -24, k: 1.0, flat: 0.85 }, { y: 0.86, n: 8, s: 0.148, tilt: -34, k: 1.0, flat: 0.85 }, { y: 0.93, n: 5, s: 0.135, tilt: -44, k: 1.0, flat: 0.85 },
      ],
      shade: [{ y: 0.2, n: 3, k: 0.5, s: 0.2 }, { y: 0.44, n: 4, k: 0.52, s: 0.21 }, { y: 0.67, n: 3, k: 0.45, s: 0.19 }],
      core: { n: 5, from: 0.21, to: 0.77, k: 0.75 },
      cap: [0.21, 0.135], tufts: 0, jitter: 0.9,
    },
    200: {
      rings: [
        { y: 0.05, n: 6, s: 0.18, tilt: 30, k: 1.0, flat: 0.9 }, { y: 0.19, n: 9, s: 0.21, tilt: 16, k: 1.0, flat: 0.9 }, { y: 0.34, n: 11, s: 0.225, tilt: 4, k: 1.0, flat: 0.9 }, { y: 0.49, n: 11, s: 0.225, tilt: -6, k: 1.0, flat: 0.9 }, { y: 0.64, n: 10, s: 0.21, tilt: -16, k: 1.0, flat: 0.9 }, { y: 0.78, n: 7, s: 0.19, tilt: -27, k: 1.0, flat: 0.9 }, { y: 0.9, n: 4, s: 0.16, tilt: -40, k: 1.0, flat: 0.9 },
      ],
      shade: [{ y: 0.2, n: 3, k: 0.45, s: 0.24 }, { y: 0.45, n: 3, k: 0.45, s: 0.25 }, { y: 0.7, n: 3, k: 0.42, s: 0.22 }],
      core: { n: 4, from: 0.22, to: 0.76, k: 0.76 },
      cap: [0.22, 0.14], tufts: 0, jitter: 0.8,
    },
    500: {
      rings: [{ y: 0.06, n: 5, s: 0.24, tilt: 28, k: 1.0, flat: 0.95 }, { y: 0.24, n: 7, s: 0.27, tilt: 12, k: 1.0, flat: 0.95 }, { y: 0.42, n: 8, s: 0.28, tilt: 0, k: 1.0, flat: 0.95 }, { y: 0.6, n: 7, s: 0.27, tilt: -12, k: 1.0, flat: 0.95 }, { y: 0.76, n: 6, s: 0.24, tilt: -24, k: 1.0, flat: 0.95 }, { y: 0.89, n: 4, s: 0.2, tilt: -36, k: 1.0, flat: 0.95 }],
      shade: [{ y: 0.3, n: 4, k: 0.4, s: 0.3 }, { y: 0.6, n: 4, k: 0.4, s: 0.3 }],
      core: { n: 3, from: 0.24, to: 0.74, k: 0.78 },
      cap: [0.25, 0.16], tufts: 0, jitter: 0.6,
    },
  },
  tone: { root: 0.32, stem: 0.36, limb: 0.4, sun: 0.64, shade: 0.47 },
};

const voice: Voice = {
  name: t("Field Maple", "Feldahorn"), latin: "Acer campestre",
  habit: t("a short stem forking into three stems", "ein kurzer Stamm, der sich in drei Stämmlinge teilt"),
  leaf: t("small leaves with three to five blunt lobes", "kleine Blätter mit drei bis fünf stumpfen Lappen"),
  bark: t("corky bark in small oblong plates", "korkige Borke in kleinen länglichen Feldern"),
  fruit: t("the horizontally spread samaras are only drawn at 1:50.", "die waagerecht gespreizten Flügelfrüchte sind nur bei 1:50 gezeichnet."),
  crown: t("a dense, rounded, slightly irregular crown of many small masses", "eine dichte, rundliche, leicht unregelmäßige Krone aus vielen kleinen Massen"),
};

const gs: [number, number] = [0.0042, 0.0024];
const a = assumptions(voice, gs);
const sunCalm: [number, number, number, number] = [0.28, 0.32, 0.86, 0.09];

export const design: Design = {
  tree,
  core: { top: "stem-c", low: ["stem-c", 3.4], high: ["stem-c", 6.3] },
  wood: { densityKgM3: 930, trunkHeatCapacity: 80000, limbHeatCapacity: 30000, limbLengthM: 0.1 },
  leaf: { lai: 4.5, sunShare: 0.55, massPerAreaKgM2: 0.25, lengthM: 0.06, gsSun: gs[0], gsShade: gs[1], size: 0.075, spacingMm: 3.2, most: 14, lobes: 5, sharp: 0.05, sinus: 0.42 },
  samara: { share: 0.14, span: 0.065, angleDeg: 180, most: 5 },
  bark: { marks: corkMarks, closed: false },
  wind: { cd: 0.35, porosity: 0.3, periodS: 2.2, gustPeak: 13 },
  rootPlateM: 2.6,
  environment: { airTempC: 23, meanRadiantTempC: 27, relativeHumidity: 0.55, windMS: 2, windDirectionDeg: 0, solarWM2: 750, pressurePa: 101325, diffuseFraction: 0.2 },
  poses: [
    { id: "summer-calm", title: t("Summer, calm", "Sommer, windstill"), description: t("Light air of 1.5 m/s at 10 m on a clear summer day in open country: the crown stands at rest, the outer leaves are lit, transpiration at 80 % of the daytime maximum.", "Leiser Zug von 1,5 m/s in 10 m Höhe an einem klaren Sommertag im offenen Land: Die Krone steht ruhig, das äußere Laub ist besonnt, Transpiration bei 80 % des Tagesmaximums."), v10: 1.5, lean: [0, 0, 0, 0], sun: sunCalm, transp: [0.8, 0.56] },
    { id: "summer-breeze", title: t("Summer breeze", "Sommerbrise"), description: t("Moderate breeze of 6 m/s at 10 m (Beaufort 4): the three stems and their limbs lean downwind; drag with cd 0.35 on the crown face less 30 % porosity.", "Mäßige Brise von 6 m/s in 10 m Höhe (Beaufort 4): Die drei Stämmlinge und ihre Äste neigen sich in Windrichtung; Winddruck mit cd 0,35 auf die Kronenfläche abzüglich 30 % Durchlässigkeit."), v10: 6, lean: [-0.1, -0.25, -0.5, -0.8], sun: sunCalm, transp: [0.95, 0.66] },
    { id: "storm", title: t("Storm", "Sturm"), description: t("Beaufort 9, 22 m/s at 10 m with rain: strong downwind lean, drag reduced by reconfiguration (cd 0.35 (v/8)^-0.6 above 8 m/s, Vogel 1989), wet bark and leaves, overcast sky.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen: starke Neigung in Windrichtung, Winddruck durch Umformung gemindert (cd 0,35 (v/8)^-0,6 über 8 m/s, Vogel 1989), nasse Rinde und Blätter, bedeckter Himmel."), v10: 22, lean: [-0.8, -1.6, -3, -5.5], wet: 0.6, sun: [0.04, 0.05, 0.12, 0.01], transp: [0.4, 0.28] },
    { id: "summer-downpour", title: t("Summer downpour", "Sommerlicher Platzregen"), description: t("A heavy summer shower: the crown holds about 1 mm of intercepted water (1 kg per m2 of crown plan area, the canopy storage of a broadleaf in leaf after Crockford and Richardson 2000), the small leaves and thin limbs sag a little; wet surfaces, no sun, transpiration almost stopped.", "Ein kräftiger Sommerschauer: Die Krone hält etwa 1 mm Interzeptionswasser (1 kg je m2 Kronengrundfläche, der Kronenspeicher eines belaubten Laubbaums nach Crockford und Richardson 2000), die kleinen Blätter und dünnen Äste hängen etwas durch; nasse Oberflächen, keine Sonne, Transpiration fast eingestellt."), v10: 4, lean: [-0.05, -0.12, -0.25, -0.4], droop: [0.6, 1.5], wet: 1, sun: [0.03, 0.03, 0.06, 0.01], transp: [0.06, 0.04], extra: { kind: "rain", load: "imposed", kgPerM2: 1, basis: "plan", title: t("Intercepted rain", "Interzeptionswasser"), source: "literature", reference: "crockford-richardson-2000" } },
    { id: "autumn-colour", title: t("Autumn colour", "Herbstfärbung"), description: t("A dry late-October day: the leaves have turned butter-yellow and the stomata close, transpiration at 20 %; the low sun lights less of the crown. Colour is not geometric, so only the thermal binding changes.", "Ein trockener Tag Ende Oktober: Die Blätter sind buttergelb gefärbt und die Spaltöffnungen schließen, Transpiration bei 20 %; die tiefe Sonne beleuchtet weniger von der Krone. Die Farbe ist nicht geometrisch, daher ändert sich nur die thermische Bindung."), v10: 2, lean: [0, 0, 0, 0], sun: [0.2, 0.24, 0.6, 0.06], transp: [0.2, 0.14] },
  ],
  materials: materials(t("Field maple bark, corky, in small oblong plates", "Feldahornborke, korkig, in kleinen länglichen Feldern"), 0.7, 930, 0.48),
  references: [...REFERENCES, { id: "crockford-richardson-2000", citation: "Crockford, R. H. and Richardson, D. P. (2000). Partitioning of rainfall into throughfall, stemflow and interception: effect of forest type, ground cover and climate. Hydrological Processes 14, 2903-2920. Canopy storage capacity of broadleaf crowns in leaf." }],
  texts: {
    volume: (s, n) => volumeTexts(voice, s, n),
    drawing: (s, n) => drawingTexts(voice, s, n),
    symbol: t("A conventional plan sign for site, hedgerow and landscape plans: the true outline of the crown, the stem at breast height and the directions of the stems and lower limbs. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Lage-, Hecken- und Freiraumpläne: der wahre Kronenumriss, der Stamm in Brusthöhe und die Richtungen der Stämmlinge und unteren Äste. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
    sway: t("A deterministic 2.2 s sway cycle about the breeze lean (6 m/s at 10 m); the first bending mode of a small open-grown 10 m tree lies near 0.45 Hz (James et al. 2006, authored estimate). The three stems sway together, limbs and foliage with growing amplitude and a phase lag.", "Ein deterministischer Schwingungszyklus von 2,2 s um die Neigung bei Brise (6 m/s in 10 m Höhe); die erste Biegeeigenform eines kleinen frei stehenden 10 m hohen Baums liegt bei etwa 0,45 Hz (James et al. 2006, Autorenschätzung). Die drei Stämmlinge schwingen gemeinsam, Äste und Laub mit wachsender Amplitude und phasenverzögert."),
    gust: t("A single gust over 8 s from 6 to 13 m/s at 10 m: the wind-load tracks follow the reconfigured drag law, the foliage leans first and the stem last.", "Eine einzelne Böe über 8 s von 6 auf 13 m/s in 10 m Höhe: Die Windlast-Spuren folgen dem umgeformten Widerstandsgesetz, das Laub neigt sich zuerst, der Stamm zuletzt."),
    diurnal: t("A 24 h summer day compressed to 24 s (1 s = 1 h), calm weather: the sunlit share of every region and the transpiration follow the sun, with stomatal closure at night and a mid-afternoon depression; leaf and bark temperatures follow from the thermal solver.", "Ein 24-h-Sommertag auf 24 s gerafft (1 s = 1 h) bei Windstille: Der besonnte Anteil jeder Region und die Transpiration folgen der Sonne, nachts sind die Spaltöffnungen geschlossen, am Nachmittag gibt es eine Mittagsdepression; Blatt- und Rindentemperaturen folgen aus dem thermischen Löser."),
    regions: { trunk: t("Corky bark of the stem and the three stems", "Korkige Borke von Stamm und den drei Stämmlingen"), limb: t("Bark of the limbs and branches", "Rinde der Äste und Zweige"), sun: t("Sun leaves of the outer crown", "Sonnenblätter der äußeren Krone"), shade: t("Shade leaves of the inner crown", "Schattenblätter der Innenkrone") },
    thermalAssumptions: a.thermal,
    structureAssumptions: a.structure,
  },
};
