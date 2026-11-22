/** 🌰️ Design of the sycamore maple (Acer pseudoplatanus): a mature open-grown tree of about 100 years, 25 m high with a broad, billowing dome of about 20 m on a massive stem of 0.95 m at breast height; four whorls of heavy, wide-spreading limbs; smooth grey bark flaking in irregular plates; leaves of 10 to 16 cm with five coarsely toothed lobes; samaras in hanging clusters, their wings at about a right angle.
 * Every station, limb, ring and pose value below is chosen by hand for this species.
 * @see ./design.ts
 * @see https://www.gbif.org/species/3189870 */
import type { Design } from "./design.ts";
import { REFERENCES, assumptions, drawingTexts, materials, plateMarks, t, volumeTexts, type Voice } from "./design.ts";
import type { LimbSpec, Species } from "./model.ts";

const side = (f: number, daz: number, e0: number, e1: number, L: number, r0: number, r1: number, bend = 0) => ({ f, daz, e0, e1, L, r0, r1, bend });
const limb = (id: string, parent: string, at: number, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend: number, en: string, de: string, sides: LimbSpec["sides"]): LimbSpec => ({ id, parent, title: t(en, de), at, az, e0, e1, L, r0, r1, bend, limit: 10, sides });
const low = (f: number, s: number): LimbSpec["sides"] => [side(0.3, -34 * s, 20, 8, 3.0, 0.085, 0.03, -8 * s), side(0.55, 38 * s, 24, 10, 2.8, 0.075, 0.028, 8 * s), side(0.78, -28 * s, 16, 4, 2.3 * f, 0.06, 0.025)];
const mid = (s: number): LimbSpec["sides"] => [side(0.4, 34 * s, 26, 12, 2.6, 0.07, 0.026), side(0.68, -36 * s, 28, 14, 2.2, 0.06, 0.024)];

const tree: Species = {
  id: "sycamore-maple",
  seed: 3307,
  title: t("Sycamore Maple", "Bergahorn"),
  summary: t("A mature open-grown sycamore maple (Acer pseudoplatanus) of about 25 m with a massive stem and a broad, billowing dome of dark, coarsely toothed five-lobed leaves on heavy spreading limbs; the grey bark flakes in plates.", "Ein ausgewachsener, frei stehender Bergahorn (Acer pseudoplatanus) von etwa 25 m mit mächtigem Stamm und breiter, wolkiger Kuppel aus dunklen, grob gezähnten, fünflappigen Blättern an schweren, weit ausladenden Ästen; die graue Borke schuppt in Platten ab."),
  distinction: t("A mature sycamore maple (Acer pseudoplatanus) of about 25 m: a massive stem with four whorls of heavy, wide-spreading limbs under a broad, billowing dome; grey bark flaking in plates, hanging clusters of right-angled samaras.", "Ein ausgewachsener Bergahorn (Acer pseudoplatanus) von etwa 25 m: ein mächtiger Stamm mit vier Quirlen schwerer, weit ausladender Äste unter einer breiten, wolkigen Kuppel; graue, in Platten schuppende Borke, hängende Trauben rechtwinkliger Flügelfrüchte."),
  tags: ["tree", "deciduous", "broadleaf", "broad-crown", "maple", "park-tree", "samaras", "wind", "transpiration"],
  notes: t("Rig, model and drawings are authored for an open-grown sycamore of about 100 years: stem 4.6 m, leader to 22 m, four limb whorls at 4.8 to 18.5 m, crown 25 m high and 20 m wide. Size and crown form follow Roloff (2010) and Mitchell (1974) for Acer pseudoplatanus (25 to 35 m, broad domed crown); wood properties, masses, the samara crop and conductances are authored estimates.", "Rig, Modell und Zeichnungen sind für einen frei stehenden Bergahorn von etwa 100 Jahren entworfen: Stamm 4,6 m, Stammverlängerung bis 22 m, vier Astquirle zwischen 4,8 und 18,5 m, Krone 25 m hoch und 20 m breit. Größe und Kronenform folgen Roloff (2010) und Mitchell (1974) für Acer pseudoplatanus (25 bis 35 m, breit gewölbte Krone); Holzeigenschaften, Massen, der Fruchtbehang und die Leitfähigkeiten sind Autorenschätzungen."),
  flare: [[0, 0.92], [0.12, 0.78], [0.3, 0.66], [0.55, 0.58], [0.85, 0.535], [1.2, 0.51], [1.5, 0.5]],
  buttress: [[12, 1], [70, 0.9], [128, 1.06], [190, 0.94], [247, 1.02], [312, 0.92]],
  axes: [
    { id: "stem", parent: "root-plate", title: t("Stem", "Stamm"), limit: 2.5, region: "trunk-bark", rows: [[0, 0.3, 0, 0.53], [0.01, 1.3, 0.005, 0.5], [0.03, 2.5, 0.01, 0.48], [0.05, 3.6, 0.006, 0.465], [0.06, 4.6, 0, 0.45]] },
    { id: "leader-1", parent: "stem", title: t("Leader, lower", "Stammverlängerung, unten"), limit: 3.5, region: "trunk-bark", rows: [[0.06, 4.6, 0, 0.42], [0.08, 6.2, -0.02, 0.39], [0.09, 7.8, -0.03, 0.36], [0.07, 9.4, -0.01, 0.33], [0.04, 11.0, 0.02, 0.3]] },
    { id: "leader-2", parent: "leader-1", title: t("Leader, middle", "Stammverlängerung, Mitte"), limit: 4.5, region: "limb-bark", rows: [[0.04, 11.0, 0.02, 0.27], [0.01, 13.0, 0.03, 0.24], [-0.02, 15.0, 0.01, 0.2], [-0.03, 17.0, -0.02, 0.16]] },
    { id: "leader-3", parent: "leader-2", title: t("Leader, top", "Stammverlängerung, Spitze"), limit: 5.5, region: "limb-bark", rows: [[-0.03, 17.0, -0.02, 0.15], [-0.02, 18.8, -0.02, 0.12], [0, 20.5, 0, 0.09], [0.01, 22.2, 0.01, 0.06]] },
  ],
  limbs: [
    limb("limb-1a", "leader-1", 4.8, 8, 36, 20, 8.4, 0.26, 0.07, 8, "Lowest whorl, limb ahead", "Unterster Quirl, Ast vorn", low(1, 1)),
    limb("limb-1b", "leader-1", 5.1, 80, 35, 19, 8.2, 0.25, 0.07, -8, "Lowest whorl, limb ahead left", "Unterster Quirl, Ast vorn links", low(0.95, -1)),
    limb("limb-1c", "leader-1", 4.9, 152, 37, 21, 8.5, 0.26, 0.07, 8, "Lowest whorl, limb behind left", "Unterster Quirl, Ast hinten links", low(1.05, 1)),
    limb("limb-1d", "leader-1", 5.3, 224, 34, 19, 8.1, 0.245, 0.07, -8, "Lowest whorl, limb behind right", "Unterster Quirl, Ast hinten rechts", low(0.95, -1)),
    limb("limb-1e", "leader-1", 5.0, 296, 36, 20, 8.3, 0.25, 0.07, 8, "Lowest whorl, limb ahead right", "Unterster Quirl, Ast vorn rechts", low(1, 1)),
    limb("limb-2a", "leader-1", 8.7, 44, 40, 26, 8.4, 0.2, 0.06, -6, "Second whorl, limb ahead left", "Zweiter Quirl, Ast vorn links", mid(1)),
    limb("limb-2b", "leader-1", 9.0, 134, 41, 27, 8.3, 0.2, 0.06, 6, "Second whorl, limb behind left", "Zweiter Quirl, Ast hinten links", mid(-1)),
    limb("limb-2c", "leader-1", 8.8, 226, 40, 26, 8.5, 0.205, 0.06, -6, "Second whorl, limb behind right", "Zweiter Quirl, Ast hinten rechts", mid(1)),
    limb("limb-2d", "leader-1", 9.3, 316, 42, 27, 8.2, 0.195, 0.06, 6, "Second whorl, limb ahead right", "Zweiter Quirl, Ast vorn rechts", mid(-1)),
    limb("limb-3a", "leader-2", 13.3, 0, 46, 34, 7.2, 0.15, 0.05, 6, "Third whorl, limb ahead", "Dritter Quirl, Ast vorn", [side(0.5, -32, 32, 20, 2.0, 0.05, 0.022)]),
    limb("limb-3b", "leader-2", 13.6, 92, 47, 35, 7.0, 0.148, 0.05, -6, "Third whorl, limb to the left", "Dritter Quirl, Ast links", [side(0.5, 32, 32, 20, 2.0, 0.05, 0.022)]),
    limb("limb-3c", "leader-2", 13.4, 182, 46, 34, 7.2, 0.15, 0.05, 6, "Third whorl, limb behind", "Dritter Quirl, Ast hinten", [side(0.5, -32, 32, 20, 2.0, 0.05, 0.022)]),
    limb("limb-3d", "leader-2", 13.8, 272, 47, 35, 7.0, 0.146, 0.05, -6, "Third whorl, limb to the right", "Dritter Quirl, Ast rechts", [side(0.5, 32, 32, 20, 2.0, 0.05, 0.022)]),
    limb("limb-4a", "leader-3", 18.3, 58, 50, 40, 5.2, 0.1, 0.045, 4, "Top whorl, limb to the left", "Oberster Quirl, Ast links", []),
    limb("limb-4b", "leader-3", 18.5, 178, 51, 41, 5.1, 0.098, 0.045, -4, "Top whorl, limb behind", "Oberster Quirl, Ast hinten", []),
    limb("limb-4c", "leader-3", 18.4, 298, 50, 40, 5.2, 0.1, 0.045, 4, "Top whorl, limb to the right", "Oberster Quirl, Ast rechts", []),
  ],
  crown: { base: 4.2, widest: 11.8, top: 25.1, rx: 10.3, rz: 9.9, lowerExp: 1.75, upperP: 1.75, upperQ: 0.52, lumps: [[3, 0.035, 1.1], [4, 0.022, 0.2], [7, 0.012, 2.6]], cx: 0.05 },
  scales: {
    50: {
      rings: [
        { y: 0.03, n: 12, s: 0.12, tilt: 36, k: 1.0 }, { y: 0.1, n: 17, s: 0.14, tilt: 28, k: 1.0 }, { y: 0.18, n: 21, s: 0.155, tilt: 20, k: 1.0 }, { y: 0.27, n: 23, s: 0.162, tilt: 12, k: 1.0 },
        { y: 0.36, n: 24, s: 0.165, tilt: 4, k: 1.0 }, { y: 0.45, n: 24, s: 0.166, tilt: -3, k: 1.0 }, { y: 0.54, n: 23, s: 0.165, tilt: -9, k: 1.0 }, { y: 0.63, n: 21, s: 0.16, tilt: -15, k: 1.0 },
        { y: 0.72, n: 18, s: 0.155, tilt: -22, k: 1.0 }, { y: 0.8, n: 15, s: 0.15, tilt: -29, k: 1.0 }, { y: 0.87, n: 11, s: 0.14, tilt: -37, k: 1.0 }, { y: 0.93, n: 7, s: 0.125, tilt: -45, k: 1.0 },
      ],
      shade: [{ y: 0.12, n: 6, k: 0.5, s: 0.16 }, { y: 0.3, n: 7, k: 0.55, s: 0.19 }, { y: 0.5, n: 7, k: 0.55, s: 0.19 }, { y: 0.7, n: 5, k: 0.48, s: 0.17 }],
      core: { n: 6, from: 0.2, to: 0.78, k: 0.74 },
      cap: [0.19, 0.11], tufts: 0.35, jitter: 1,
    },
    100: {
      rings: [
        { y: 0.04, n: 9, s: 0.135, tilt: 34, k: 1.0, flat: 0.85 }, { y: 0.13, n: 14, s: 0.158, tilt: 24, k: 1.0, flat: 0.85 }, { y: 0.24, n: 17, s: 0.175, tilt: 14, k: 1.0, flat: 0.85 }, { y: 0.36, n: 18, s: 0.182, tilt: 4, k: 1.0, flat: 0.85 },
        { y: 0.48, n: 18, s: 0.183, tilt: -5, k: 1.0, flat: 0.85 }, { y: 0.6, n: 16, s: 0.177, tilt: -13, k: 1.0, flat: 0.85 }, { y: 0.71, n: 14, s: 0.17, tilt: -22, k: 1.0, flat: 0.85 }, { y: 0.81, n: 11, s: 0.16, tilt: -31, k: 1.0, flat: 0.85 }, { y: 0.9, n: 7, s: 0.145, tilt: -41, k: 1.0, flat: 0.85 },
      ],
      shade: [{ y: 0.16, n: 4, k: 0.5, s: 0.19 }, { y: 0.4, n: 5, k: 0.55, s: 0.21 }, { y: 0.64, n: 4, k: 0.5, s: 0.19 }],
      core: { n: 5, from: 0.21, to: 0.77, k: 0.75 },
      cap: [0.2, 0.115], tufts: 0, jitter: 0.9,
    },
    200: {
      rings: [
        { y: 0.05, n: 8, s: 0.175, tilt: 30, k: 1.0, flat: 0.9 }, { y: 0.19, n: 11, s: 0.204, tilt: 16, k: 1.0, flat: 0.9 }, { y: 0.34, n: 13, s: 0.218, tilt: 4, k: 1.0, flat: 0.9 }, { y: 0.49, n: 13, s: 0.218, tilt: -6, k: 1.0, flat: 0.9 }, { y: 0.64, n: 12, s: 0.204, tilt: -16, k: 1.0, flat: 0.9 }, { y: 0.78, n: 9, s: 0.184, tilt: -27, k: 1.0, flat: 0.9 }, { y: 0.9, n: 6, s: 0.155, tilt: -40, k: 1.0, flat: 0.9 },
      ],
      shade: [{ y: 0.2, n: 3, k: 0.45, s: 0.24 }, { y: 0.45, n: 3, k: 0.45, s: 0.25 }, { y: 0.7, n: 3, k: 0.42, s: 0.22 }],
      core: { n: 4, from: 0.22, to: 0.76, k: 0.76 },
      cap: [0.22, 0.14], tufts: 0, jitter: 0.8,
    },
    500: {
      rings: [{ y: 0.06, n: 7, s: 0.233, tilt: 28, k: 1.0, flat: 0.95 }, { y: 0.24, n: 9, s: 0.262, tilt: 12, k: 1.0, flat: 0.95 }, { y: 0.42, n: 10, s: 0.272, tilt: 0, k: 1.0, flat: 0.95 }, { y: 0.6, n: 9, s: 0.262, tilt: -12, k: 1.0, flat: 0.95 }, { y: 0.76, n: 8, s: 0.233, tilt: -24, k: 1.0, flat: 0.95 }, { y: 0.89, n: 6, s: 0.194, tilt: -36, k: 1.0, flat: 0.95 }],
      shade: [{ y: 0.3, n: 4, k: 0.4, s: 0.3 }, { y: 0.6, n: 4, k: 0.4, s: 0.3 }],
      core: { n: 3, from: 0.24, to: 0.74, k: 0.78 },
      cap: [0.25, 0.16], tufts: 0, jitter: 0.6,
    },
  },
  tone: { root: 0.34, stem: 0.4, limb: 0.42, sun: 0.54, shade: 0.4 },
};

const voice: Voice = {
  name: t("Sycamore Maple", "Bergahorn"), latin: "Acer pseudoplatanus",
  habit: t("a massive stem with heavy spreading limbs", "ein mächtiger Stamm mit schweren, ausladenden Ästen"),
  leaf: t("large five-lobed, coarsely toothed leaves", "große, fünflappige, grob gezähnte Blätter"),
  bark: t("grey bark flaking in irregular plates", "graue, in unregelmäßigen Platten schuppende Borke"),
  fruit: t("the hanging samara clusters are drawn at 1:50 and are a load in the fruit pose.", "die hängenden Fruchttrauben sind bei 1:50 gezeichnet und in der Fruchtpose eine Last."),
  crown: t("a broad, billowing dome of large overlapping masses", "eine breite, wolkige Kuppel aus großen, sich überlappenden Massen"),
};

const gs: [number, number] = [0.005, 0.0028];
const a = assumptions(voice, gs);
const sunCalm: [number, number, number, number] = [0.3, 0.34, 0.84, 0.07];

export const design: Design = {
  tree,
  core: { top: "leader-3", low: ["leader-1", 8.0], high: ["leader-2", 14.8] },
  wood: { densityKgM3: 900, trunkHeatCapacity: 150000, limbHeatCapacity: 50000, limbLengthM: 0.28 },
  leaf: { lai: 5.0, sunShare: 0.55, massPerAreaKgM2: 0.3, lengthM: 0.13, gsSun: gs[0], gsShade: gs[1], size: 0.15, spacingMm: 4.8, most: 12, lobes: 5, sharp: 0.35, sinus: 0.45 },
  samara: { share: 0.22, span: 0.08, angleDeg: 85, most: 6 },
  bark: { marks: plateMarks, closed: true },
  wind: { cd: 0.35, porosity: 0.25, periodS: 3.9, gustPeak: 15 },
  rootPlateM: 5.5,
  environment: { airTempC: 24, meanRadiantTempC: 28, relativeHumidity: 0.5, windMS: 1.5, windDirectionDeg: 0, solarWM2: 750, pressurePa: 101325, diffuseFraction: 0.2 },
  poses: [
    { id: "summer-calm", title: t("Summer, calm", "Sommer, windstill"), description: t("Light air of 1.5 m/s at 10 m on a clear summer day: the dome stands at rest, its outer leaves are lit and the dense dark foliage shades the inner crown; transpiration at 80 % of the daytime maximum.", "Leiser Zug von 1,5 m/s in 10 m Höhe an einem klaren Sommertag: Die Kuppel steht ruhig, ihr äußeres Laub ist besonnt und das dichte dunkle Laub beschattet die Innenkrone; Transpiration bei 80 % des Tagesmaximums."), v10: 1.5, lean: [0, 0, 0, 0], sun: sunCalm, transp: [0.8, 0.56] },
    { id: "summer-breeze", title: t("Summer breeze", "Sommerbrise"), description: t("Moderate breeze of 6 m/s at 10 m (Beaufort 4): the heavy limbs barely move, the foliage leans downwind; drag with cd 0.35 on the crown face less 25 % porosity.", "Mäßige Brise von 6 m/s in 10 m Höhe (Beaufort 4): Die schweren Äste bewegen sich kaum, das Laub neigt sich in Windrichtung; Winddruck mit cd 0,35 auf die Kronenfläche abzüglich 25 % Durchlässigkeit."), v10: 6, lean: [-0.08, -0.18, -0.4, -0.75], sun: sunCalm, transp: [0.95, 0.66] },
    { id: "storm", title: t("Storm", "Sturm"), description: t("Beaufort 9, 22 m/s at 10 m with rain: strong downwind lean of the broad dome, drag reduced by reconfiguration (cd 0.35 (v/8)^-0.6 above 8 m/s, Vogel 1989), wet bark and leaves, overcast sky.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen: starke Neigung der breiten Kuppel in Windrichtung, Winddruck durch Umformung gemindert (cd 0,35 (v/8)^-0,6 über 8 m/s, Vogel 1989), nasse Rinde und Blätter, bedeckter Himmel."), v10: 22, lean: [-0.7, -1.5, -3, -5.6], wet: 0.6, sun: [0.04, 0.05, 0.12, 0.01], transp: [0.4, 0.28] },
    { id: "samara-laden", title: t("Laden with samaras", "Mit Flügelfrüchten behangen"), description: t("Early September of a heavy mast year: hanging clusters of paired samaras with wings at about a right angle all over the outer crown, some 0.12 kg of green fruit per m2 of crown plan area (authored estimate of a crop of the order of 50 000 fruit pairs); the limbs droop a little, transpiration at 70 % of the summer level.", "Anfang September eines starken Mastjahres: hängende Trauben gepaarter Flügelfrüchte mit etwa rechtwinkligen Flügeln über die ganze äußere Krone, rund 0,12 kg grüne Früchte je m2 Kronengrundfläche (Autorenschätzung für einen Behang in der Größenordnung von 50 000 Fruchtpaaren); die Äste hängen etwas durch, Transpiration bei 70 % des Sommerwerts."), v10: 3, lean: [-0.02, -0.05, -0.1, -0.18], droop: [0.15, 0.45], sun: [0.27, 0.31, 0.78, 0.07], transp: [0.7, 0.49], extra: { kind: "samaras", load: "imposed", kgPerM2: 0.12, basis: "plan", title: t("Samara crop", "Fruchtbehang"), source: "authored-estimate", reference: "roloff-2010" } },
    { id: "midsummer-drought", title: t("Midsummer drought", "Hochsommerliche Trockenheit"), description: t("A hot, dry spell: the shallow-rooted sycamore closes its stomata in the afternoon, transpiration at 30 %, the large leaves lose turgor and hang; full sun on the dome.", "Eine heiße Trockenperiode: Der flach wurzelnde Bergahorn schließt nachmittags die Spaltöffnungen, Transpiration bei 30 %, die großen Blätter verlieren Spannung und hängen; volle Sonne auf der Kuppel."), v10: 1.5, lean: [0, 0, 0, 0], droop: [0.25, 0.9], sun: [0.33, 0.37, 0.9, 0.09], transp: [0.3, 0.2] },
  ],
  materials: materials(t("Sycamore bark, smooth grey, flaking in plates", "Bergahornborke, glatt grau, in Platten schuppend"), 0.62, 900, 0.52),
  references: REFERENCES,
  texts: {
    volume: (s, n) => volumeTexts(voice, s, n),
    drawing: (s, n) => drawingTexts(voice, s, n),
    symbol: t("A conventional plan sign for site and landscape plans: the true outline of the broad crown, the stem at breast height and the directions of the lower limbs. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Lage- und Freiraumpläne: der wahre Umriss der breiten Krone, der Stamm in Brusthöhe und die Richtungen der unteren Äste. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
    sway: t("A deterministic 3.9 s sway cycle about the breeze lean (6 m/s at 10 m); the first bending mode of a heavy open-grown 25 m broadleaf lies near 0.25 Hz (James et al. 2006, authored estimate). The heavy limbs follow the stem closely, the foliage swings with a growing phase lag.", "Ein deterministischer Schwingungszyklus von 3,9 s um die Neigung bei Brise (6 m/s in 10 m Höhe); die erste Biegeeigenform eines schweren, frei stehenden 25 m hohen Laubbaums liegt bei etwa 0,25 Hz (James et al. 2006, Autorenschätzung). Die schweren Äste folgen dem Stamm dicht, das Laub schwingt mit wachsender Phasenverzögerung."),
    gust: t("A single gust over 8 s from 6 to 15 m/s at 10 m: the wind-load tracks follow the reconfigured drag law, the foliage leans first and the stem last.", "Eine einzelne Böe über 8 s von 6 auf 15 m/s in 10 m Höhe: Die Windlast-Spuren folgen dem umgeformten Widerstandsgesetz, das Laub neigt sich zuerst, der Stamm zuletzt."),
    diurnal: t("A 24 h summer day compressed to 24 s (1 s = 1 h), calm weather: the sunlit share of every region and the transpiration follow the sun, with stomatal closure at night and a mid-afternoon depression; leaf and bark temperatures follow from the thermal solver.", "Ein 24-h-Sommertag auf 24 s gerafft (1 s = 1 h) bei Windstille: Der besonnte Anteil jeder Region und die Transpiration folgen der Sonne, nachts sind die Spaltöffnungen geschlossen, am Nachmittag gibt es eine Mittagsdepression; Blatt- und Rindentemperaturen folgen aus dem thermischen Löser."),
    regions: { trunk: t("Flaking grey bark of the stem and the lower leader", "Schuppende graue Borke von Stamm und unterer Stammverlängerung"), limb: t("Smoother bark of limbs, branches and the upper leader", "Glattere Rinde von Ästen, Zweigen und oberer Stammverlängerung"), sun: t("Sun leaves of the outer dome", "Sonnenblätter der äußeren Kuppel"), shade: t("Shade leaves of the inner crown", "Schattenblätter der Innenkrone") },
    thermalAssumptions: a.thermal,
    structureAssumptions: a.structure,
  },
};
