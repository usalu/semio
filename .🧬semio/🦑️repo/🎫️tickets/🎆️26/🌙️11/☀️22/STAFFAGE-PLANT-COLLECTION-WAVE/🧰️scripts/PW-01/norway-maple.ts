/** 🍁️ Design of the Norway maple (Acer platanoides): a mature open-grown park tree of about 70 years, 20 m high with a dense, broadly ovoid crown of about 15 m on a straight stem of 0.62 m at breast height; one continuous leader, five whorls of ascending scaffold limbs; regular shallow interlacing bark ridges; leaves 10 to 15 cm with five to seven pointed lobes; samaras spread at an obtuse, almost straight angle.
 * Every station, limb, ring and pose value below is chosen by hand for this species.
 * @see ./design.ts
 * @see https://www.gbif.org/species/3189846 */
import type { Design } from "./design.ts";
import { REFERENCES, assumptions, drawingTexts, materials, ridgeMarks, t, volumeTexts, type Voice } from "./design.ts";
import type { LimbSpec, Species } from "./model.ts";

const side = (f: number, daz: number, e0: number, e1: number, L: number, r0: number, r1: number, bend = 0) => ({ f, daz, e0, e1, L, r0, r1, bend });
const limb = (id: string, parent: string, at: number, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend: number, en: string, de: string, sides: LimbSpec["sides"]): LimbSpec => ({ id, parent, title: t(en, de), at, az, e0, e1, L, r0, r1, bend, limit: 10, sides });

const tree: Species = {
  id: "norway-maple",
  seed: 1101,
  title: t("Norway Maple", "Spitzahorn"),
  summary: t("A mature park Norway maple (Acer platanoides) of about 20 m with a straight stem, one continuous leader and a dense, broadly ovoid crown of large five-lobed, pointed leaves; the bark is finely and regularly ridged.", "Ein ausgewachsener Spitzahorn (Acer platanoides) im Park, etwa 20 m hoch, mit geradem Stamm, durchgehender Stammverlängerung und dichter, breit eiförmiger Krone aus großen, fünflappigen, spitzen Blättern; die Borke ist fein und regelmäßig gerippt."),
  distinction: t("A mature Norway maple (Acer platanoides) of about 20 m: a straight stem with one leader and five whorls of ascending limbs under a dense, rounded-ovoid crown that is closed to the ground of the crown; obtuse samaras.", "Ein ausgewachsener Spitzahorn (Acer platanoides) von etwa 20 m: ein gerader Stamm mit einer Stammverlängerung und fünf Quirlen aufsteigender Äste unter einer dichten, rundlich-eiförmigen, geschlossenen Krone; stumpfwinklige Flügelfrüchte."),
  tags: ["tree", "deciduous", "broadleaf", "dense-crown", "maple", "park-tree", "wind", "transpiration"],
  notes: t("Rig, model and drawings are authored for an open-grown Norway maple of about 70 years: stem 3.2 m, leader to 17.5 m, five limb whorls at 3.4 to 15 m. Size and crown form follow Roloff (2010) and Mitchell (1974) for Acer platanoides in parks (20 to 25 m, crown 12 to 16 m); wood properties, masses and conductances are authored estimates.", "Rig, Modell und Zeichnungen sind für einen frei gewachsenen Spitzahorn von etwa 70 Jahren entworfen: Stamm 3,2 m, Stammverlängerung bis 17,5 m, fünf Astquirle zwischen 3,4 und 15 m. Größe und Kronenform folgen Roloff (2010) und Mitchell (1974) für Acer platanoides in Parks (20 bis 25 m, Krone 12 bis 16 m); Holzeigenschaften, Massen und Leitfähigkeiten sind Autorenschätzungen."),
  flare: [[0, 0.58], [0.1, 0.5], [0.25, 0.42], [0.45, 0.37], [0.7, 0.345], [1.0, 0.33], [1.35, 0.318]],
  buttress: [[18, 1], [86, 0.92], [152, 1.05], [214, 0.9], [283, 1.02], [334, 0.94]],
  axes: [
    { id: "stem", parent: "root-plate", title: t("Stem", "Stamm"), limit: 3, region: "trunk-bark", rows: [[0, 0.3, 0, 0.33], [0.005, 0.9, 0.0, 0.322], [0.012, 1.6, 0.004, 0.31], [0.02, 2.4, 0.006, 0.3], [0.026, 3.2, 0.0, 0.29]] },
    { id: "leader-1", parent: "stem", title: t("Leader, lower", "Stammverlängerung, unten"), limit: 4, region: "trunk-bark", rows: [[0.026, 3.2, 0, 0.27], [0.04, 4.3, -0.012, 0.255], [0.052, 5.5, -0.02, 0.24], [0.048, 6.8, -0.01, 0.222], [0.03, 8.0, 0.012, 0.205], [0.012, 9.0, 0.02, 0.19]] },
    { id: "leader-2", parent: "leader-1", title: t("Leader, middle", "Stammverlängerung, Mitte"), limit: 5, region: "limb-bark", rows: [[0.012, 9.0, 0.02, 0.175], [-0.01, 10.4, 0.012, 0.158], [-0.026, 11.8, -0.008, 0.14], [-0.02, 13.2, -0.02, 0.122]] },
    { id: "leader-3", parent: "leader-2", title: t("Leader, top", "Stammverlängerung, Spitze"), limit: 6, region: "limb-bark", rows: [[-0.02, 13.2, -0.02, 0.11], [-0.006, 14.6, -0.012, 0.092], [0.01, 16.0, 0.004, 0.072], [0.016, 17.5, 0.01, 0.05]] },
  ],
  limbs: [
    limb("limb-1a", "leader-1", 3.45, 16, 40, 26, 5.4, 0.17, 0.055, 8, "Lowest whorl, limb ahead", "Unterster Quirl, Ast vorn", [side(0.34, -38, 22, 10, 1.9, 0.06, 0.022, -10), side(0.58, 40, 26, 12, 1.7, 0.05, 0.02, 8), side(0.8, -30, 18, 4, 1.4, 0.04, 0.018)]),
    limb("limb-1b", "leader-1", 3.6, 106, 38, 25, 5.2, 0.165, 0.055, -6, "Lowest whorl, limb to the left", "Unterster Quirl, Ast links", [side(0.36, 36, 22, 10, 1.8, 0.058, 0.022), side(0.6, -42, 24, 12, 1.7, 0.05, 0.02, -8), side(0.82, 28, 16, 4, 1.3, 0.04, 0.018)]),
    limb("limb-1c", "leader-1", 3.5, 197, 41, 27, 5.3, 0.168, 0.055, 7, "Lowest whorl, limb behind", "Unterster Quirl, Ast hinten", [side(0.35, -40, 22, 10, 1.9, 0.06, 0.022), side(0.6, 38, 26, 12, 1.6, 0.05, 0.02), side(0.8, -26, 18, 4, 1.4, 0.04, 0.018, 6)]),
    limb("limb-1d", "leader-1", 3.7, 287, 39, 26, 5.1, 0.16, 0.055, -8, "Lowest whorl, limb to the right", "Unterster Quirl, Ast rechts", [side(0.35, 38, 22, 10, 1.8, 0.058, 0.022), side(0.6, -36, 24, 12, 1.7, 0.05, 0.02), side(0.82, 30, 16, 4, 1.3, 0.04, 0.018)]),
    limb("limb-2a", "leader-1", 5.7, 62, 44, 30, 5.4, 0.14, 0.05, 6, "Second whorl, limb ahead left", "Zweiter Quirl, Ast vorn links", [side(0.4, -34, 24, 12, 1.7, 0.05, 0.02), side(0.68, 36, 28, 14, 1.5, 0.044, 0.018)]),
    limb("limb-2b", "leader-1", 5.9, 152, 45, 31, 5.3, 0.138, 0.05, -6, "Second whorl, limb behind left", "Zweiter Quirl, Ast hinten links", [side(0.4, 34, 24, 12, 1.7, 0.05, 0.02), side(0.68, -36, 28, 14, 1.5, 0.044, 0.018)]),
    limb("limb-2c", "leader-1", 5.8, 243, 43, 30, 5.5, 0.142, 0.05, 6, "Second whorl, limb behind right", "Zweiter Quirl, Ast hinten rechts", [side(0.4, -36, 24, 12, 1.7, 0.05, 0.02), side(0.68, 34, 28, 14, 1.5, 0.044, 0.018)]),
    limb("limb-2d", "leader-1", 6.0, 333, 46, 31, 5.2, 0.136, 0.05, -6, "Second whorl, limb ahead right", "Zweiter Quirl, Ast vorn rechts", [side(0.4, 36, 24, 12, 1.7, 0.05, 0.02), side(0.68, -34, 28, 14, 1.5, 0.044, 0.018)]),
    limb("limb-3a", "leader-2", 9.6, 4, 50, 38, 4.6, 0.11, 0.045, 6, "Third whorl, limb ahead", "Dritter Quirl, Ast vorn", [side(0.45, -36, 30, 18, 1.4, 0.04, 0.018), side(0.72, 34, 30, 18, 1.2, 0.036, 0.016)]),
    limb("limb-3b", "leader-2", 9.9, 124, 52, 40, 4.5, 0.108, 0.045, -6, "Third whorl, limb behind left", "Dritter Quirl, Ast hinten links", [side(0.45, 36, 30, 18, 1.4, 0.04, 0.018), side(0.72, -34, 30, 18, 1.2, 0.036, 0.016)]),
    limb("limb-3c", "leader-2", 9.7, 244, 51, 39, 4.6, 0.11, 0.045, 6, "Third whorl, limb behind right", "Dritter Quirl, Ast hinten rechts", [side(0.45, -36, 30, 18, 1.4, 0.04, 0.018), side(0.72, 34, 30, 18, 1.2, 0.036, 0.016)]),
    limb("limb-4a", "leader-2", 12.4, 68, 58, 46, 3.7, 0.085, 0.04, 6, "Fourth whorl, limb to the left", "Vierter Quirl, Ast links", [side(0.55, -32, 36, 24, 1.1, 0.034, 0.016)]),
    limb("limb-4b", "leader-2", 12.6, 188, 59, 47, 3.6, 0.083, 0.04, -6, "Fourth whorl, limb behind", "Vierter Quirl, Ast hinten", [side(0.55, 32, 36, 24, 1.1, 0.034, 0.016)]),
    limb("limb-4c", "leader-2", 12.5, 308, 58, 46, 3.7, 0.085, 0.04, 6, "Fourth whorl, limb to the right", "Vierter Quirl, Ast rechts", [side(0.55, -32, 36, 24, 1.1, 0.034, 0.016)]),
    limb("limb-5a", "leader-3", 15.0, 28, 64, 56, 2.8, 0.06, 0.035, 4, "Top whorl, limb ahead", "Oberster Quirl, Ast vorn", []),
    limb("limb-5b", "leader-3", 15.2, 208, 65, 57, 2.7, 0.058, 0.035, -4, "Top whorl, limb behind", "Oberster Quirl, Ast hinten", []),
  ],
  crown: { base: 3.1, widest: 9.6, top: 20.05, rx: 7.65, rz: 7.35, lowerExp: 2.0, upperP: 2.15, upperQ: 0.62, lumps: [[3, 0.025, 0.4], [5, 0.018, 1.9]], cx: 0.03 },
  scales: {
    50: {
      rings: [
        { y: 0.03, n: 10, s: 0.125, tilt: 34, k: 1.0 }, { y: 0.1, n: 16, s: 0.15, tilt: 26, k: 1.0 }, { y: 0.19, n: 20, s: 0.162, tilt: 18, k: 1.0 }, { y: 0.29, n: 22, s: 0.168, tilt: 10, k: 1.0 },
        { y: 0.39, n: 24, s: 0.17, tilt: 2, k: 1.0 }, { y: 0.49, n: 24, s: 0.17, tilt: -5, k: 1.0 }, { y: 0.59, n: 22, s: 0.166, tilt: -12, k: 1.0 }, { y: 0.68, n: 19, s: 0.16, tilt: -18, k: 1.0 },
        { y: 0.77, n: 16, s: 0.152, tilt: -25, k: 1.0 }, { y: 0.85, n: 12, s: 0.142, tilt: -33, k: 1.0 }, { y: 0.92, n: 8, s: 0.13, tilt: -42, k: 1.0 },
      ],
      shade: [{ y: 0.14, n: 5, k: 0.5, s: 0.17 }, { y: 0.33, n: 6, k: 0.55, s: 0.2 }, { y: 0.52, n: 6, k: 0.55, s: 0.2 }, { y: 0.72, n: 4, k: 0.45, s: 0.17 }],
      core: { n: 6, from: 0.2, to: 0.78, k: 0.74 },
      cap: [0.205, 0.125], tufts: 0.42, jitter: 1,
    },
    100: {
      rings: [
        { y: 0.04, n: 8, s: 0.14, tilt: 32, k: 1.0, flat: 0.85 }, { y: 0.13, n: 13, s: 0.168, tilt: 22, k: 1.0, flat: 0.85 }, { y: 0.25, n: 16, s: 0.185, tilt: 12, k: 1.0, flat: 0.85 }, { y: 0.38, n: 18, s: 0.19, tilt: 2, k: 1.0, flat: 0.85 },
        { y: 0.51, n: 18, s: 0.19, tilt: -7, k: 1.0, flat: 0.85 }, { y: 0.63, n: 16, s: 0.182, tilt: -15, k: 1.0, flat: 0.85 }, { y: 0.75, n: 13, s: 0.17, tilt: -24, k: 1.0, flat: 0.85 }, { y: 0.86, n: 9, s: 0.155, tilt: -34, k: 1.0, flat: 0.85 }, { y: 0.93, n: 6, s: 0.14, tilt: -44, k: 1.0, flat: 0.85 },
      ],
      shade: [{ y: 0.18, n: 4, k: 0.5, s: 0.2 }, { y: 0.42, n: 5, k: 0.55, s: 0.22 }, { y: 0.66, n: 4, k: 0.48, s: 0.2 }],
      core: { n: 5, from: 0.21, to: 0.77, k: 0.75 },
      cap: [0.21, 0.13], tufts: 0, jitter: 0.9,
    },
    200: {
      rings: [
        { y: 0.05, n: 7, s: 0.18, tilt: 30, k: 1.0, flat: 0.9 }, { y: 0.19, n: 10, s: 0.21, tilt: 16, k: 1.0, flat: 0.9 }, { y: 0.34, n: 12, s: 0.225, tilt: 4, k: 1.0, flat: 0.9 }, { y: 0.49, n: 12, s: 0.225, tilt: -6, k: 1.0, flat: 0.9 }, { y: 0.64, n: 11, s: 0.21, tilt: -16, k: 1.0, flat: 0.9 }, { y: 0.78, n: 8, s: 0.19, tilt: -27, k: 1.0, flat: 0.9 }, { y: 0.9, n: 5, s: 0.16, tilt: -40, k: 1.0, flat: 0.9 },
      ],
      shade: [{ y: 0.2, n: 3, k: 0.45, s: 0.24 }, { y: 0.45, n: 3, k: 0.45, s: 0.25 }, { y: 0.7, n: 3, k: 0.42, s: 0.22 }],
      core: { n: 4, from: 0.22, to: 0.76, k: 0.76 },
      cap: [0.22, 0.14], tufts: 0, jitter: 0.8,
    },
    500: {
      rings: [{ y: 0.06, n: 6, s: 0.24, tilt: 28, k: 1.0, flat: 0.95 }, { y: 0.24, n: 8, s: 0.27, tilt: 12, k: 1.0, flat: 0.95 }, { y: 0.42, n: 9, s: 0.28, tilt: 0, k: 1.0, flat: 0.95 }, { y: 0.6, n: 8, s: 0.27, tilt: -12, k: 1.0, flat: 0.95 }, { y: 0.76, n: 7, s: 0.24, tilt: -24, k: 1.0, flat: 0.95 }, { y: 0.89, n: 5, s: 0.2, tilt: -36, k: 1.0, flat: 0.95 }],
      shade: [{ y: 0.3, n: 4, k: 0.4, s: 0.3 }, { y: 0.6, n: 4, k: 0.4, s: 0.3 }],
      core: { n: 3, from: 0.24, to: 0.74, k: 0.78 },
      cap: [0.25, 0.16], tufts: 0, jitter: 0.6,
    },
  },
  tone: { root: 0.3, stem: 0.34, limb: 0.38, sun: 0.6, shade: 0.44 },
};

const voice: Voice = {
  name: t("Norway Maple", "Spitzahorn"), latin: "Acer platanoides",
  habit: t("a straight stem with one leader", "ein gerader Stamm mit einer Stammverlängerung"),
  leaf: t("five-lobed leaves with long pointed lobe tips", "fünflappige Blätter mit lang zugespitzten Lappen"),
  bark: t("long shallow interlacing ridges", "lange, flache, netzartige Rippen"),
  fruit: t("the samaras of the fruit pose spread at an almost straight angle and are only a load.", "die Flügelfrüchte stehen fast gestreckt auseinander und sind nur eine Last."),
  crown: t("a dense, broadly ovoid crown of overlapping rounded masses", "eine dichte, breit eiförmige Krone aus sich überlappenden rundlichen Massen"),
};

const gs: [number, number] = [0.0045, 0.0025];
const a = assumptions(voice, gs);
const sunCalm: [number, number, number, number] = [0.3, 0.34, 0.85, 0.08];

export const design: Design = {
  tree,
  core: { top: "leader-3", low: ["leader-1", 6.4], high: ["leader-2", 11.6] },
  wood: { densityKgM3: 920, trunkHeatCapacity: 120000, limbHeatCapacity: 40000, limbLengthM: 0.2 },
  leaf: { lai: 5.5, sunShare: 0.55, massPerAreaKgM2: 0.28, lengthM: 0.11, gsSun: gs[0], gsShade: gs[1], size: 0.13, spacingMm: 4.2, most: 14, lobes: 5, sharp: 0.95, sinus: 0.34 },
  samara: { share: 0.16, span: 0.1, angleDeg: 165, most: 5 },
  bark: { marks: ridgeMarks, closed: false },
  wind: { cd: 0.35, porosity: 0.2, periodS: 3.2, gustPeak: 14 },
  rootPlateM: 4.0,
  environment: { airTempC: 24, meanRadiantTempC: 28, relativeHumidity: 0.5, windMS: 1.5, windDirectionDeg: 0, solarWM2: 750, pressurePa: 101325, diffuseFraction: 0.2 },
  poses: [
    { id: "summer-calm", title: t("Summer, calm", "Sommer, windstill"), description: t("Light air of 1.5 m/s at 10 m on a clear summer day: the crown stands at rest (the drag of a few newtons per foliage bone bends the limbs by less than the drawing shows), the outer leaves are lit, transpiration at 80 % of the daytime maximum.", "Leiser Zug von 1,5 m/s in 10 m Höhe an einem klaren Sommertag: Die Krone steht ruhig (die wenigen Newton Winddruck je Laubknochen biegen die Äste weniger, als die Zeichnung zeigt), das äußere Laub ist besonnt, Transpiration bei 80 % des Tagesmaximums."), v10: 1.5, lean: [0, 0, 0, 0], sun: sunCalm, transp: [0.8, 0.56] },
    { id: "summer-breeze", title: t("Summer breeze", "Sommerbrise"), description: t("Moderate breeze of 6 m/s at 10 m (Beaufort 4) in an open park: limbs and foliage lean downwind; drag with cd 0.35 on the crown face less 20 % porosity.", "Mäßige Brise von 6 m/s in 10 m Höhe (Beaufort 4) in einem offenen Park: Äste und Laub neigen sich in Windrichtung; Winddruck mit cd 0,35 auf die Kronenfläche abzüglich 20 % Durchlässigkeit."), v10: 6, lean: [-0.12, -0.25, -0.5, -0.8], sun: sunCalm, transp: [0.95, 0.66] },
    { id: "storm", title: t("Storm", "Sturm"), description: t("Beaufort 9, 22 m/s at 10 m with rain: strong downwind lean, drag reduced by reconfiguration (cd 0.35 (v/8)^-0.6 above 8 m/s, Vogel 1989), wet bark and leaves, overcast sky.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen: starke Neigung in Windrichtung, Winddruck durch Umformung gemindert (cd 0,35 (v/8)^-0,6 über 8 m/s, Vogel 1989), nasse Rinde und Blätter, bedeckter Himmel."), v10: 22, lean: [-0.9, -1.8, -3.4, -6], wet: 0.6, sun: [0.04, 0.05, 0.12, 0.01], transp: [0.4, 0.28] },
    { id: "autumn-colour", title: t("Autumn colour", "Herbstfärbung"), description: t("A dry October day: the leaves have turned clear yellow to orange and the stomata close, transpiration at 25 %; the shorter, lower sun lights less of the crown. Colour is not geometric, so only the thermal binding changes.", "Ein trockener Oktobertag: Die Blätter sind leuchtend gelb bis orange gefärbt und die Spaltöffnungen schließen, Transpiration bei 25 %; die kürzere, tiefere Sonne beleuchtet weniger von der Krone. Die Farbe ist nicht geometrisch, daher ändert sich nur die thermische Bindung."), v10: 2, lean: [0, 0, 0, 0], sun: [0.22, 0.25, 0.62, 0.06], transp: [0.25, 0.18] },
    { id: "early-wet-snow", title: t("Early wet snow", "Früher Nassschnee"), description: t("An early snowfall while the crown is still in leaf, the classic break event of large maples: about 12 kg of wet snow per m2 of crown plan area (order of magnitude after EN 1991-1-3) bends limbs and foliage down; calm, no sun, stomata closed.", "Ein früher Schneefall, solange die Krone noch belaubt ist, das typische Bruchereignis großer Ahorne: etwa 12 kg Nassschnee je m2 Kronengrundfläche (Größenordnung nach EN 1991-1-3) biegen Äste und Laub nach unten; windstill, keine Sonne, Spaltöffnungen geschlossen."), v10: 1, lean: [0, 0, 0, 0], droop: [1.2, 2.6], wet: 1, sun: [0.02, 0.02, 0.04, 0.005], transp: [0.05, 0.035], extra: { kind: "snow", load: "snow", kgPerM2: 12, basis: "plan", title: t("Wet snow", "Nassschnee"), source: "authored-estimate", reference: "en-1991-1-3" } },
  ],
  materials: materials(t("Norway maple bark, finely ridged, grey-brown", "Spitzahornborke, fein gerippt, graubraun"), 0.68, 920, 0.5),
  references: REFERENCES,
  texts: {
    volume: (s, n) => volumeTexts(voice, s, n),
    drawing: (s, n) => drawingTexts(voice, s, n),
    symbol: t("A conventional plan sign for site and landscape plans: the true outline of the crown, the stem at breast height and the directions of the lower limbs. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Lage- und Freiraumpläne: der wahre Kronenumriss, der Stamm in Brusthöhe und die Richtungen der unteren Äste. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
    sway: t("A deterministic 3.2 s sway cycle about the breeze lean (6 m/s at 10 m); the first bending mode of an open-grown 20 m broadleaf lies near 0.3 Hz (James et al. 2006, authored estimate). Limbs and foliage swing with growing amplitude and a phase lag behind the stem.", "Ein deterministischer Schwingungszyklus von 3,2 s um die Neigung bei Brise (6 m/s in 10 m Höhe); die erste Biegeeigenform eines frei stehenden 20 m hohen Laubbaums liegt bei etwa 0,3 Hz (James et al. 2006, Autorenschätzung). Äste und Laub schwingen mit wachsender Amplitude und phasenverzögert hinter dem Stamm."),
    gust: t("A single gust over 8 s from 6 to 14 m/s at 10 m: the wind-load tracks follow the reconfigured drag law, the foliage leans first and the stem last.", "Eine einzelne Böe über 8 s von 6 auf 14 m/s in 10 m Höhe: Die Windlast-Spuren folgen dem umgeformten Widerstandsgesetz, das Laub neigt sich zuerst, der Stamm zuletzt."),
    diurnal: t("A 24 h summer day compressed to 24 s (1 s = 1 h), calm weather: the sunlit share of every region and the transpiration follow the sun, with stomatal closure at night and a mid-afternoon depression; leaf and bark temperatures follow from the thermal solver.", "Ein 24-h-Sommertag auf 24 s gerafft (1 s = 1 h) bei Windstille: Der besonnte Anteil jeder Region und die Transpiration folgen der Sonne, nachts sind die Spaltöffnungen geschlossen, am Nachmittag gibt es eine Mittagsdepression; Blatt- und Rindentemperaturen folgen aus dem thermischen Löser."),
    regions: { trunk: t("Ridged bark of the stem and the lower leader", "Gerippte Borke von Stamm und unterer Stammverlängerung"), limb: t("Smoother bark of limbs, branches and the upper leader", "Glattere Rinde von Ästen, Zweigen und oberer Stammverlängerung"), sun: t("Sun leaves of the outer crown", "Sonnenblätter der äußeren Krone"), shade: t("Shade leaves of the inner crown", "Schattenblätter der Innenkrone") },
    thermalAssumptions: a.thermal,
    structureAssumptions: a.structure,
  },
};
