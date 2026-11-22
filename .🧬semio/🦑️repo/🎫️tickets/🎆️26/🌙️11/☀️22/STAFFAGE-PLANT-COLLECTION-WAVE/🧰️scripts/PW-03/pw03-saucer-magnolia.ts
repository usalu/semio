/** 🌷️ saucer-magnolia design: per-scale wood of the four stems with their low branches and shoots, the broad low crown of loose clumps, the upright goblet flowers, the bark folds, outlets and the decision texts of a Magnolia × soulangeana in late bloom.
 * @see ./sp_saucer_magnolia.py — rig, masses and physics
 * @see ./pw03-engine.ts */
import { D2R, add, mul, sub, unit, type Lobe } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts";
import type { P2, V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";
import { ball, clumps, limbTube, onLobes, sides, stemTube, type ClumpRow, type Context, type Counts, type Extra, type Goblet, type Model, type Outlet, type SideRow, type Species, type Text } from "./pw03-engine.ts";

const TONE = { root: 0.4, stem: 0.44, limb: 0.46, sun: 0.6, shade: 0.45, flower: 0.85 };
const K = [1, 2, 3, 4];
const STEM_AZ: Record<number, number> = { 1: 25, 2: 118, 3: 205, 4: 295 };
const LOW_AZ: Record<number, number> = { 1: 57, 2: 150, 3: 237, 4: 327 };

/** 🌳️ Crown envelope: low and broad, from 1.3 m up to the widest ring of 3.35 m at 3.0 m and a rounded top at 6.0 m. */
const reach = (_az: number, y: number): number => (y >= 3.0 ? 3.35 * Math.max(0.05, 1 - ((y - 3.0) / 3.0) ** 2.4) ** 0.45 : 3.35 * Math.sqrt(Math.max(0.08, 1 - ((3.0 - y) / 2.0) ** 2)));

const CLUMPS: ClumpRow[] = [
  ...K.flatMap((k): ClumpRow[] => [
    [`l${k}a`, `fol-l${k}`, LOW_AZ[k]! - 14, 2.25, 0.92, 1, "1345", 12],
    [`l${k}b`, `fol-l${k}`, LOW_AZ[k]! + 20, 2.05, 0.82, 0.98, "134", 16],
    [`m${k}`, `fol-${k}`, STEM_AZ[k]! - 8, 3.35, 1.0, 1, "12345", 6],
    [`u${k}`, `fol-${k}`, STEM_AZ[k]! + 12, 4.65, 0.92, 1, "1246", 4],
  ]),
];
const CAP: [V3, V3] = [[0.05, 5.15, 0.0], [1.35, 0.82, 1.35]];
const CAP_BOSSES: [number, number, number, number][] = [[30, 0.7, 5.62, 0.34], [120, 0.72, 5.6, 0.33], [210, 0.7, 5.62, 0.34], [300, 0.72, 5.6, 0.33]];
const CORES: [string, number, number, number][] = [["core-0", 0, 0, 3.5], ["core-1", 70, 1.05, 3.3], ["core-2", 160, 1.0, 3.6], ["core-3", 250, 1.05, 3.3], ["core-4", 340, 1.0, 3.6]];
const FLARE: Record<number, [number, number, number, number][]> = {
  50: [[0, 0.56, 0, 0], [0.05, 0.48, 0, 0], [0.12, 0.41, 0, 0], [0.2, 0.36, 0, 0], [0.28, 0.33, 0, 0], [0.36, 0.3, 0, 0]],
  100: [[0, 0.56, 0, 0], [0.1, 0.43, 0, 0], [0.22, 0.35, 0, 0], [0.36, 0.3, 0, 0]],
  200: [[0, 0.54, 0, 0], [0.15, 0.39, 0, 0], [0.36, 0.3, 0, 0]],
  500: [[0, 0.5, 0, 0], [0.36, 0.3, 0, 0]],
};
const SIDES_50: SideRow[] = K.flatMap((k): SideRow[] => [
  [`s${k}b-sh`, `s${k}b`, 0.72, STEM_AZ[k]! + 20, 80, 72, 1.15, 0.026, 0.01, 6],
  [`s${k}a-s1`, `s${k}a`, 0.82, STEM_AZ[k]! - 42, 42, 26, 1.4, 0.036, 0.012, -8],
  [`br-${k}-sh`, `br-${k}`, 0.62, LOW_AZ[k]! - 10, 58, 70, 0.95, 0.022, 0.009, 4],
]);
const SIDES_100: SideRow[] = SIDES_50.filter((r) => !r[0].startsWith("br")).map((r) => [r[0], r[1], r[2], r[3], r[4], r[5], r[6] * 0.95, 0.03, 0.014, r[9]]);

function crown(scale: number): Lobe[] {
  const keep = scale === 50 ? null : scale === 100 ? "123457" : scale === 200 ? "146" : "";
  const grow = scale === 50 ? 1 : scale === 100 ? 1.04 : scale === 200 ? 1.14 : 1.3;
  const bossGrow = scale === 50 ? 1 : scale === 100 ? 1.05 : 1.2;
  const bone = scale === 500 ? "s1a" : undefined;
  const out = clumps(CLUMPS, { reach: (az, y) => reach(az, y) * (scale === 500 ? 0.985 : scale === 200 ? 0.99 : scale === 100 ? 0.995 : 1), axis: [0.05, 0.0], keep, grow, bossGrow, shape: [0.86, 0.66, 0.95], inset: scale === 500 ? 0 : 0.18, material: "magnolia-leaf-sun", tone: TONE.sun, bone });
  out.push(ball("cap", bone ?? "fol-1", "magnolia-leaf-sun", TONE.sun, "sun", CAP[0], CAP[1].map((r, i) => (i === 1 ? r : r * grow)) as V3));
  if (scale <= 100) CAP_BOSSES.forEach(([az, r, y, rb], i) => out.push(ball(`cap-b${i + 1}`, "fol-1", "magnolia-leaf-sun", TONE.sun, "sun", [0.05 + r * Math.cos(az * D2R), y, r * Math.sin(az * D2R)], [rb * bossGrow, rb * 0.82 * bossGrow, rb * bossGrow])));
  const cores = scale === 50 ? CORES : scale === 100 ? CORES.filter((_, i) => i % 2 === 0) : scale === 200 ? [CORES[0]!, CORES[2]!, CORES[4]!] : [CORES[0]!];
  for (const [id, az, r, y] of cores) {
    const c: V3 = scale === 500 ? [0.15, 3.5, 0.1] : [0.15 + r * Math.cos(az * D2R), y, 0.1 + r * Math.sin(az * D2R)];
    out.push(ball(id, bone ?? "fol-core", "magnolia-leaf-shade", TONE.shade, "shade", c, id === "core-0" ? [2.3, 0.9, 2.3] : [1.05, 0.8, 1.05]));
  }
  return out;
}

const PROFILE: [number, number][] = [[0, 0], [0.006, 0.016], [0.03, 0.042], [0.065, 0.056], [0.1, 0.054], [0.128, 0.04], [0.14, 0.022], [0.142, 0]];

/** 🌷️ Upright goblets standing on the lobe faces; the coarser scales merge neighbouring flowers into fewer, larger goblets, 1:500 keeps them inside the crown. */
function flowers(scale: number, lobes: Lobe[]): Extra[] {
  const k = scale === 50 ? 1 : scale === 100 ? 1.2 : scale === 200 ? 1.5 : 1.5;
  const rows: [string, number, number][] = [];
  for (const [id, , az] of CLUMPS) {
    if (scale === 50) rows.push([`${id}-m`, az + 10, 48], [`${id}-b2`, az - 6, 62], [`${id}-b6`, az + 4, 82], [`${id}-b4`, az + 40, 26], [`${id}-b5`, az - 40, 26], [`${id}-b1`, az + 2, 14]);
    else if (scale === 100) rows.push([`${id}-m`, az + 10, 48], [`${id}-b4`, az + 40, 24], [`${id}-b5`, az - 40, 24]);
    else if (scale === 200) rows.push([`${id}-m`, az, 45], ...(id.startsWith("u") ? [[`${id}-b6`, az, 80] as [string, number, number]] : []));
    else if (id.startsWith("m")) rows.push([`${id}-m`, az, 40]);
  }
  if (scale <= 100) rows.push(["cap", 40, 75], ["cap", 160, 72], ["cap", 280, 74], ["cap", 0, 90]);
  return onLobes(rows, lobes, (id, bone, at, d): Goblet => ({ kind: "goblet", id, bone, material: "magnolia-tepal", tone: TONE.flower, group: "flower", base: scale === 500 ? sub(at, mul(d, 0.45)) : sub(at, [0, 0.07 * k, 0]), profile: PROFILE.map(([a, r]) => [a * k, r * k]) }), "bloom");
}

function outlets(scale: number): Outlet[] {
  if (scale === 50) return [
    ...K.map((k): Outlet => ["transpiration-sun", `tp-sun-${2 * k - 1}`, `u${k}-b2`, STEM_AZ[k]! + 12, 55]),
    ...K.map((k): Outlet => ["transpiration-sun", `tp-sun-${2 * k}`, `l${k}a-b4`, LOW_AZ[k]! + 20, 35]),
    ...CORES.map(([id, az], i): Outlet => ["transpiration-shade", `tp-shade-${i + 1}`, id, az, 80]),
  ];
  if (scale === 100) return [["transpiration-sun", null, "u1-b2", 37, 55], ["transpiration-sun", null, "u3-b2", 217, 55], ["transpiration-sun", null, "cap", 0, 90], ["transpiration-shade", null, "core-2", 160, 80], ["transpiration-shade", null, "core-4", 340, 80]];
  if (scale === 200) return [["transpiration-sun", null, "u1-b6", 37, 80], ["transpiration-sun", null, "cap", 0, 90], ["transpiration-shade", null, "core-2", 160, 80]];
  return [["transpiration-sun", null, "cap", 0, 90], ["transpiration-shade", null, "core-0", 0, 90]];
}

function model(scale: number, ctx: Context): Model {
  const keep = scale === 50 ? null : scale === 100 ? [0, 2, 4, 6, 8] : scale === 200 ? [0, 4, 8] : [0, 8];
  const wood = [stemTube(ctx, "root-plate", TONE.root, "root", FLARE[scale]!)];
  for (const k of K) wood.push(limbTube(ctx, `s${k}a`, keep, TONE.stem, 0.08, "stem"), limbTube(ctx, `s${k}b`, scale === 500 ? [0, 4, 7] : keep, TONE.limb), limbTube(ctx, `br-${k}`, scale === 500 ? [0, 4, 7] : keep, TONE.limb));
  if (scale === 50) wood.push(...sides(SIDES_50, wood, "magnolia-bark-young", TONE.limb, 6));
  if (scale === 100) wood.push(...sides(SIDES_100, wood, "magnolia-bark-young", TONE.limb, 4));
  const lobes = crown(scale);
  const extras = flowers(scale, lobes);
  const hidden = scale === 500 ? wood.filter((t) => /b$/.test(t.id) || t.id.startsWith("br-")) : [];
  return { scale, wood: wood.filter((t) => !hidden.includes(t)), twigs: [], hiddenWood: hidden, hiddenTwigs: [], lobes, extras: scale === 500 ? [] : extras, hiddenExtras: scale === 500 ? extras : [], outlets: outlets(scale), ...(scale === 500 ? { crownBone: "s1a" } : {}) };
}

/** 🍃️ An obovate magnolia leaf of 15 cm, widest beyond the middle, with a short point. */
function leaf(c: P2, deg: number): P2[] {
  const half: [number, number][] = [[0, 0], [0.02, 0.014], [0.055, 0.028], [0.095, 0.036], [0.128, 0.03], [0.146, 0.012], [0.152, 0]];
  const pts = [...half, ...half.slice(1, -1).reverse().map(([x, y]) => [x, -y] as [number, number])];
  const [cs, sn] = [Math.cos((deg * Math.PI) / 180), Math.sin((deg * Math.PI) / 180)];
  return pts.map(([x, y]) => ({ x: c.x + (x - 0.076) * cs - y * sn, y: c.y + (x - 0.076) * sn + y * cs }));
}

/** 🟫️ The smooth grey stems carry only soft folds across their axis near the root crown. */
function bark(view: "elevation" | "front" | "plan", scale: number, ctx: Context): { id: string; bone: string; role: string; points: V3[] }[] {
  const out: { id: string; bone: string; role: string; points: V3[] }[] = [];
  const stems = view === "elevation" ? [1, 3] : [2, 4];
  for (const k of stems) {
    const pts: V3[] = ctx.skeleton.skeleton[`s${k}a`].points;
    const fs = scale === 50 ? [0.12, 0.2, 0.3, 0.55] : [0.15, 0.4];
    fs.forEach((f, i) => {
      const at = Math.min(pts.length - 2, Math.floor(f * (pts.length - 1)));
      const p = pts[at]!, d = unit(sub(pts[at + 1]!, p));
      const perp: V3 = view === "elevation" ? unit([-d[1], d[0], 0]) : unit([0, d[2], -d[1]]);
      const w = 0.055 * (1 - 0.2 * i);
      out.push({ id: `fold-${k}-${i + 1}`, bone: `s${k}a`, role: "texture", points: [add(p, mul(perp, -w)), add(add(p, mul(perp, 0)), mul(d, 0.012)), add(p, mul(perp, w))] });
    });
  }
  return out;
}

function texts(scale: number, c: Counts): { drawn: Record<string, Text>; volume: Text[] } {
  const mm = { 50: 20, 100: 10, 200: 5, 500: 2 }[scale]!;
  const pen = { 50: ["0,25", "0.25"], 100: ["0,2", "0.2"], 200: ["0,18", "0.18"], 500: ["0,15", "0.15"] }[scale]!;
  const seen = (en: boolean): string => en ? `${c.seen.elevation} in the side elevation, ${c.seen.front} in the front elevation, ${c.seen.plan} in the plan` : `${c.seen.elevation} in der Seitenansicht, ${c.seen.front} in der Vorderansicht, ${c.seen.plan} in der Draufsicht`;
  const drawn: Record<number, Record<string, Text>> = {
    50: {
      silhouette: { en: `At 1:50 (1 m is ${mm} mm) the three drawings are the true views of the 1:50 body: the broad root crown, four smooth stems leaning out with soft bark folds, their low spreading branches, upright shoots and side branches, and the low, broad crown as one outline per loose foliage mass with leaf-cluster arcs, obovate leaf marks and every goblet flower seen from the viewpoint (${seen(true)}).`, de: `Bei 1:50 (1 m sind ${mm} mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: der breite Wurzelstock, vier glatte, nach außen geneigte Stämme mit weichen Rindenfalten, ihre tiefen ausladenden Äste, aufrechten Triebe und Seitenäste und die niedrige, breite Krone als ein Umriss je lockerer Laubmasse mit Blattbüschelbögen, verkehrt-eiförmigen Blattzeichen und jeder aus der Blickrichtung sichtbaren Kelchblüte (${seen(false)}).` },
      retained: { en: "Retained: every woody member of the rig and the twelve shoots and side branches, every foliage mass with its bosses, the shade core and the visible goblets at their true size of 14 cm.", de: "Beibehalten: jedes Holzglied des Rigs und die zwölf Triebe und Seitenäste, jede Laubmasse mit ihren Ausbuchtungen, der Schattenkern und die sichtbaren Kelchblüten in ihrer wahren Größe von 14 cm." },
      merged: { en: "The nine tepals of a flower are one goblet; the unfolding leaves are merged into loose lobed masses.", de: "Die neun Hüllblätter einer Blüte sind ein Kelch; die sich entfaltenden Blätter sind zu lockeren, gelappten Massen vereint." },
      omitted: { en: "Omitted: flower stalks, the furry bud scales, twigs thinner than 2 cm and the single leaves.", de: "Weggelassen: Blütenstiele, die pelzigen Knospenschuppen, Zweige unter 2 cm und die Einzelblätter." },
      thinElements: { en: "Bark folds and arcs are 0.13 mm texture strokes, leaf marks 0.18 mm outlines; shoot ends of 2 cm are 0.4 mm wide on paper and stay filled.", de: "Rindenfalten und Bögen sind 0,13-mm-Texturstriche, Blattzeichen 0,18-mm-Umrisse; Triebenden von 2 cm sind auf dem Papier 0,4 mm breit und bleiben gefüllt." },
      hatching: { en: "A thin 45 degree line hatch (0.8 mm spacing) marks the shaded inner crown; the smooth bark carries no hatch.", de: "Eine feine 45-Grad-Schraffur (0,8 mm Abstand) kennzeichnet die beschattete Innenkrone; die glatte Rinde trägt keine Schraffur." },
    },
    100: {
      silhouette: { en: `At 1:100 (1 m is ${mm} mm) the drawings are the true views of the 1:100 body: root crown, the four stems with their low branches and upright shoots, and the broad crown as foliage masses with broader lobes, leaf-cluster arcs and the visible goblets of 17 cm (${seen(true)}).`, de: `Bei 1:100 (1 m sind ${mm} mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: Wurzelstock, die vier Stämme mit ihren tiefen Ästen und aufrechten Trieben und die breite Krone als Laubmassen mit breiteren Lappen, Blattbüschelbögen und den sichtbaren Kelchblüten von 17 cm (${seen(false)}).` },
      retained: { en: "Retained: stems, low branches, the upright shoots and the inner side branches, the foliage masses with their outward and side bosses, three shade cores and three goblets per clump.", de: "Beibehalten: Stämme, tiefe Äste, die aufrechten Triebe und die inneren Seitenäste, die Laubmassen mit ihren äußeren und seitlichen Ausbuchtungen, drei Schattenkerne und drei Kelchblüten je Klumpen." },
      merged: { en: "Two neighbouring flowers merge into one larger goblet; the upward bosses merge into the masses.", de: "Zwei benachbarte Blüten verschmelzen zu einem größeren Kelch; die oberen Ausbuchtungen gehen in den Massen auf." },
      omitted: { en: "Omitted: leaf marks, the shoots on the low branches, single leaves and flower stalks.", de: "Weggelassen: Blattzeichen, die Triebe auf den tiefen Ästen, Einzelblätter und Blütenstiele." },
      thinElements: { en: "Thin strokes are at least 0.1 mm: bark folds use the texture pen, arcs the detail pen; branches keep their real width.", de: "Dünne Striche sind mindestens 0,1 mm: Rindenfalten mit dem Texturstift, Bögen mit dem Detailstift; Äste behalten ihre reale Breite." },
      hatching: { en: "Restrained: only the shaded inner crown carries a thin 45 degree hatch (0.6 mm spacing).", de: "Zurückhaltend: nur die beschattete Innenkrone trägt eine feine 45-Grad-Schraffur (0,6 mm Abstand)." },
    },
    200: {
      silhouette: { en: `At 1:200 (1 m is ${mm} mm) the drawings are the true views of the 1:200 body: root crown, the four stems and their low branches under a broad, low crown of foliage masses; the flowers are goblets of 21 cm, 1 mm on paper (${seen(true)}).`, de: `Bei 1:200 (1 m sind ${mm} mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: Wurzelstock, die vier Stämme und ihre tiefen Äste unter einer breiten, niedrigen Krone aus Laubmassen; die Blüten sind Kelche von 21 cm, 1 mm auf dem Papier (${seen(false)}).` },
      retained: { en: "Retained: root crown, the twelve stem and branch parts with three sections each, the foliage masses with their outward, side and upper bosses, two shade cores and one or two goblets per clump.", de: "Beibehalten: Wurzelstock, die zwölf Stamm- und Astteile mit je drei Schnitten, die Laubmassen mit ihren äußeren, seitlichen und oberen Ausbuchtungen, zwei Schattenkerne und ein oder zwei Kelche je Klumpen." },
      merged: { en: "Shoots and side branches merge into the foliage masses; the flowers of a clump into one or two goblets.", de: "Triebe und Seitenäste gehen in den Laubmassen auf; die Blüten eines Klumpens in einem oder zwei Kelchen." },
      omitted: { en: "Omitted: bark folds, arcs, leaf marks and the cap bosses.", de: "Weggelassen: Rindenfalten, Bögen, Blattzeichen und die Ausbuchtungen der Kappe." },
      thinElements: { en: "No element is thinner than 0.35 mm on paper; nothing is thickened.", de: "Kein Element ist auf dem Papier dünner als 0,35 mm; nichts wird verdickt." },
      hatching: { en: "No hatching; the masses are separated by their outlines.", de: "Keine Schraffur; die Massen werden durch ihre Umrisse getrennt." },
    },
    500: {
      silhouette: { en: `At 1:500 (1 m is ${mm} mm) the magnolia is one closed, low and broad crown, 13 mm wide and 12 mm high, on the short visible stems and the root crown; the drawings are the true views of the 1:500 body.`, de: `Bei 1:500 (1 m sind ${mm} mm) ist die Magnolie eine geschlossene, niedrige und breite Krone von 13 mm Breite und 12 mm Höhe über den kurzen sichtbaren Stämmen und dem Wurzelstock; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers.` },
      retained: { en: "Retained: root crown, the lower parts of the four stems and the crown outline.", de: "Beibehalten: Wurzelstock, die unteren Teile der vier Stämme und der Kronenumriss." },
      merged: { en: "Merged into the crown path: every foliage mass, the shade core, the upper stems, the low branches and the flowers (all regions of the crown are listed on it).", de: "Im Kronenpfad vereint: jede Laubmasse, der Schattenkern, die oberen Stämme, die tiefen Äste und die Blüten (alle Regionen der Krone sind daran genannt)." },
      omitted: { en: "Omitted: every flower as a mark and every internal edge.", de: "Weggelassen: jede Blüte als Zeichen und jede innere Kante." },
      thinElements: { en: "The stems are 0.4 mm wide on paper and stay filled; nothing is thickened.", de: "Die Stämme sind auf dem Papier 0,4 mm breit und bleiben gefüllt; nichts wird verdickt." },
      hatching: { en: "No hatching; the pen is 0.15 mm for outlines.", de: "Keine Schraffur; der Stift ist 0,15 mm für Umrisse." },
    },
  };
  const d = drawn[scale]!;
  d.styles = { en: `Light line: outlines of ${pen[1]} mm and the finer pens of this scale. Dark solid: the crown reads in a mid tone, the goblets light and the smooth stems a lighter grey than the crown.`, de: `Helle Linie: Umrisse von ${pen[0]} mm und die feineren Stifte dieses Maßstabs. Dunkle Fläche: die Krone liest sich im Mittelton, die Kelche hell und die glatten Stämme in einem helleren Grau als die Krone.` };
  d.overlays = scale === 500
    ? { en: "Overlays map onto stem-bark (root crown, lower stems) and, merged in the crown path, limb-bark, leaf-sun, leaf-shade and flower; merged shapes show the area-weighted value of their regions.", de: "Überlagerungen bilden sich ab auf stem-bark (Wurzelstock, untere Stämme) sowie, im Kronenpfad vereint, limb-bark, leaf-sun, leaf-shade und flower; vereinte Formen zeigen den flächengewichteten Wert ihrer Regionen." }
    : { en: "Every thermal region has its own shapes: root crown and lower stems stem-bark, upper stems and branches limb-bark, outer masses leaf-sun, the core leaf-shade, the goblets flower; marks without a region follow their bone.", de: "Jede thermische Region hat eigene Formen: Wurzelstock und untere Stämme stem-bark, obere Stämme und Äste limb-bark, äußere Massen leaf-sun, der Kern leaf-shade, die Kelche flower; Zeichen ohne Region folgen ihrem Knochen." };
  const volume: Record<number, Text[]> = {
    50: [
      { en: `Body at 1:50: the broad root crown, four stems in two parts leaning out (swept lofts), four low spreading branches, twelve shoots and side branches, a low broad crown of ${c.lobes} leaf lobes (sixteen loose clumps with their bosses, a cap with four bosses, five shade lobes) and ${c.extras} upright goblet flowers of 14 cm as revolved tepal profiles.`, de: `Körper bei 1:50: der breite Wurzelstock, vier nach außen geneigte Stämme in zwei Teilen (geführte Lofts), vier tiefe ausladende Äste, zwölf Triebe und Seitenäste, eine niedrige, breite Krone aus ${c.lobes} Laublappen (sechzehn lockere Klumpen mit ihren Ausbuchtungen, eine Kappe mit vier Ausbuchtungen, fünf Schattenlappen) und ${c.extras} aufrechte Kelchblüten von 14 cm als gedrehte Hüllblattprofile.` },
      { en: "Thermal at 1:50: stem bark is the root crown and the lower stems; limb bark the upper stems, branches and shoots; the outer lobes are the sun leaves, the inner lobes the shade leaves and the goblets the flower region, all by reference to the body; thirteen transpiration outlets are lens patches in the free faces of the lobes that carry the emitter anchors.", de: "Thermik bei 1:50: Stammrinde sind Wurzelstock und untere Stämme; Astrinde die oberen Stämme, Äste und Triebe; die äußeren Lappen sind die Sonnenblätter, die inneren die Schattenblätter und die Kelche die Blütenregion, alle als Verweis auf den Körper; dreizehn Transpirationsauslässe sind Linsenflächen in den freien Seiten der Lappen, die die Emitteranker tragen." },
      { en: "Structure at 1:50: the wood under the bark of the four stems and their branches along the real axes, joints at the rig joints, the round fixed root plate under the shared root crown and the masses of the root crown, the first stem and the nine foliage masses.", de: "Tragwerk bei 1:50: das Holz unter der Rinde der vier Stämme und ihrer Äste entlang der wirklichen Achsen, Gelenke an den Rig-Gelenken, der runde eingespannte Wurzelteller unter dem gemeinsamen Wurzelstock und die Massen von Wurzelstock, erstem Stamm und den neun Laubmassen." },
    ],
    100: [
      { en: `Body at 1:100: a simpler root crown, every stem and branch with five sections, eight shoots and side branches, ${c.lobes} broader lobes and ${c.extras} goblets of 17 cm.`, de: `Körper bei 1:100: ein einfacherer Wurzelstock, jeder Stamm und Ast mit fünf Schnitten, acht Triebe und Seitenäste, ${c.lobes} breitere Lappen und ${c.extras} Kelche von 17 cm.` },
      { en: "Thermal at 1:100: the same five regions by reference to this scale's body; five outlets as lens patches.", de: "Thermik bei 1:100: dieselben fünf Regionen als Verweis auf den Körper dieses Maßstabs; fünf Auslässe als Linsenflächen." },
      { en: "Structure at 1:100: the wood of the load-bearing bones with simplified taper, joints, the root plate and the masses.", de: "Tragwerk bei 1:100: das Holz der tragenden Knochen mit vereinfachter Verjüngung, Gelenke, der Wurzelteller und die Massen." },
    ],
    200: [
      { en: `Body at 1:200: root crown, the twelve stem and branch parts with three sections each, no shoots; ${c.lobes} masses and ${c.extras} goblets of 21 cm.`, de: `Körper bei 1:200: Wurzelstock, die zwölf Stamm- und Astteile mit je drei Schnitten, keine Triebe; ${c.lobes} Massen und ${c.extras} Kelche von 21 cm.` },
      { en: "Thermal at 1:200: bark, leaves and flowers by reference to the body; three outlets.", de: "Thermik bei 1:200: Rinde, Laub und Blüten als Verweis auf den Körper; drei Auslässe." },
      { en: "Structure at 1:200: the wood of the load-bearing bones, joints where they are at least one smallest feature thick, the root plate and the masses.", de: "Tragwerk bei 1:200: das Holz der tragenden Knochen, Gelenke dort, wo sie mindestens ein kleinstes Merkmal dick sind, der Wurzelteller und die Massen." },
    ],
    500: [
      { en: `Body at 1:500: root crown and the lower stems as plain lofts, and the crown as ${c.lobes} widened masses bound to the first stem as the drawing binds it.`, de: `Körper bei 1:500: Wurzelstock und die unteren Stämme als einfache Lofts und die Krone als ${c.lobes} verbreiterte Massen, an den ersten Stamm gebunden wie in der Zeichnung.` },
      { en: "Thermal at 1:500: bark and leaves by reference; the upper stems, the low branches and four goblets are thermal solids of their own inside the crown; two outlets.", de: "Thermik bei 1:500: Rinde und Laub als Verweis; die oberen Stämme, die tiefen Äste und vier Kelche sind eigene Thermikkörper in der Krone; zwei Auslässe." },
      { en: "Structure at 1:500: the load-bearing bones as members of their declared section, the joint of the first stem, the root plate and the masses of root crown and first stem; this list is already minimal.", de: "Tragwerk bei 1:500: die tragenden Knochen als Stäbe ihres angegebenen Querschnitts, das Gelenk des ersten Stamms, der Wurzelteller und die Massen von Wurzelstock und erstem Stamm; diese Liste ist bereits minimal." },
    ],
  };
  return { drawn: d, volume: volume[scale]! };
}

export const species: Species = {
  id: "saucer-magnolia",
  trunk: "s1a",
  stems: ["s1a", "s2a", "s3a", "s4a"],
  woodMaterial: "magnolia-wood",
  extraRegion: "flower",
  leafGlyph: leaf,
  extraGlyph: "goblet",
  bark,
  model,
  texts,
  scaffolds: ["s1a", "s2a", "s3a", "s4a"],
  symbolTexts: {
    plan: {
      title: { en: "Plan symbol with true crown extent", de: "Grundrisssymbol mit wahrer Kronenausdehnung" },
      description: { en: "A conventional sign for garden and planting plans: the true outline of the low, broad crown in the three views, the root crown and the first stem at their real position and size, and in plan the directions of the four stems. It is a separate symbol and is never used silently for a pictorial scale.", de: "Ein konventionelles Zeichen für Garten- und Pflanzpläne: der wahre Umriss der niedrigen, breiten Krone in den drei Ansichten, Wurzelstock und erster Stamm an ihrer wahren Lage und in wahrer Größe und im Grundriss die Richtungen der vier Stämme. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet." },
      dimensionalNote: { en: "The outlines are the true extent of the 1:100 body in model metres (about 6.5 m wide, 6.4 m deep and 6.05 m high); the filled shapes are the root crown and the first stem at their real size; the four strokes only indicate stem directions. Planting distances are not part of the symbol.", de: "Die Umrisse sind die wahre Ausdehnung des 1:100-Körpers in Modellmetern (etwa 6,5 m breit, 6,4 m tief und 6,05 m hoch); die gefüllten Formen sind Wurzelstock und erster Stamm in wahrer Größe; die vier Striche deuten nur die Stammrichtungen an. Pflanzabstände gehören nicht zum Symbol." },
    },
    sign: {
      title: { en: "Nominal flowering tree sign", de: "Nominelles Zeichen eines Blütenbaums" },
      description: { en: "A circle with a centre dot and an inner ring as the conventional sign of an ornamental tree or large shrub in small-scale plans, drawn as plan circle and as side and front lollipop; the size is a fixed paper size.", de: "Ein Kreis mit Mittelpunkt und Innenring als übliches Zeichen eines Zierbaums oder Großstrauchs in kleinmaßstäblichen Plänen, als Grundrisskreis und als seitlicher und vorderer Lolli gezeichnet; die Größe ist eine feste Papiergröße." },
      dimensionalNote: { en: "A fixed 3.2 mm circle with a 1.8 mm inner ring and a 0.7 mm centre dot in paper millimetres; its size carries no dimension. Use the true-extent symbol where the crown width matters.", de: "Ein fester Kreis von 3,2 mm mit 1,8 mm Innenring und 0,7 mm Mittelpunkt in Papiermillimetern; seine Größe trägt kein Maß. Wo die Kronenbreite zählt, ist das Symbol mit wahrer Ausdehnung zu verwenden." },
    },
  },
};
