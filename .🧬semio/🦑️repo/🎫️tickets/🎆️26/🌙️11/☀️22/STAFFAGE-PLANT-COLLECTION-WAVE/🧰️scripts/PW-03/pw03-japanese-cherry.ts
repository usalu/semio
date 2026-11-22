/** 🌸️ japanese-cherry design: per-scale wood, the funnel-and-dome crown of clumps, the hanging double-flower clusters, bark lenticels, outlets and the decision texts of a Prunus serrulata 'Kanzan' in full bloom.
 * @see ./sp_japanese_cherry.py — rig, masses and physics
 * @see ./pw03-engine.ts */
import { D2R, add, mul, type Lobe } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts";
import type { P2, V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";
import { ball, clumps, limbTube, onLobes, sides, stemTube, type ClumpRow, type Context, type Counts, type Extra, type Model, type Outlet, type SideRow, type Species, type Text } from "./pw03-engine.ts";

const TONE = { root: 0.3, stem: 0.34, limb: 0.38, sun: 0.62, shade: 0.46, flower: 0.82 };
const K = [1, 2, 3, 4, 5];
const TIP_AZ: Record<number, number> = { 1: 17, 2: 80, 3: 163, 4: 222, 5: 302 };

/** 🌳️ Crown envelope: a funnel from 1.7 m radius at 3.4 m to the widest ring of 4.05 m at 6.0 m, then a flat dome to 8.0 m. */
const reach = (_az: number, y: number): number => (y >= 6.0 ? 4.05 * Math.max(0.05, 1 - ((y - 6.0) / 2.05) ** 2.2) ** 0.45 : 1.7 + 2.35 * Math.max(0, (y - 3.4) / 2.6) ** 0.8);

const CLUMPS: ClumpRow[] = [
  ...K.flatMap((k): ClumpRow[] => [
    [`w${k}a`, `fol-${k}`, TIP_AZ[k]! - 17, 6.0, 1.0, 1, "12345", 8],
    [`w${k}b`, `fol-${k}`, TIP_AZ[k]! + 19, 5.85, 0.94, 0.99, "1237", 10],
    [`f${k}`, `fol-${k}`, TIP_AZ[k]! + 2, 4.75, 0.78, 1, "134", 18],
  ]),
  ["u1", "fol-1", 40, 6.95, 0.86, 1, "1246", 2], ["u2", "fol-2", 110, 7.0, 0.84, 1, "1256", 2], ["u3", "fol-3", 185, 6.95, 0.86, 1, "1246", 2],
  ["u4", "fol-4", 255, 7.0, 0.84, 1, "1256", 2], ["u5", "fol-5", 325, 6.95, 0.86, 1, "1246", 2],
];
const CAP: [V3, V3] = [[0.45, 7.2, 0.25], [1.35, 0.85, 1.35]];
const CAP_BOSSES: [number, number, number, number][] = [[20, 0.8, 7.68, 0.36], [100, 0.76, 7.66, 0.34], [175, 0.82, 7.68, 0.35], [250, 0.76, 7.66, 0.33], [320, 0.78, 7.68, 0.34]];
const CORES: [string, number, number, number][] = [["core-0", 0, 0, 5.3], ["core-1", 30, 1.15, 5.25], ["core-2", 110, 1.1, 5.45], ["core-3", 185, 1.15, 5.25], ["core-4", 255, 1.1, 5.45], ["core-5", 325, 1.15, 5.3]];

const FLARE: Record<number, [number, number, number, number][]> = {
  50: [[0, 0.36, 0, 0], [0.05, 0.31, 0, 0], [0.12, 0.262, 0.001, 0], [0.22, 0.228, 0.002, 0], [0.3, 0.212, 0.003, 0], [0.4, 0.203, 0.003, 0]],
  100: [[0, 0.36, 0, 0], [0.08, 0.28, 0.001, 0], [0.22, 0.228, 0.002, 0], [0.4, 0.203, 0.003, 0]],
  200: [[0, 0.34, 0, 0], [0.12, 0.25, 0.001, 0], [0.4, 0.204, 0.003, 0]],
  500: [[0, 0.3, 0, 0], [0.4, 0.205, 0.003, 0]],
};
const SIDES_50: SideRow[] = K.flatMap((k): SideRow[] => [
  [`sc-${k}a-s1`, `sc-${k}a`, 0.42, TIP_AZ[k]! + 58, 52, 36, 1.55, 0.034, 0.012, 10],
  [`sc-${k}a-s2`, `sc-${k}a`, 0.74, TIP_AZ[k]! - 52, 46, 30, 1.05, 0.03, 0.011, -10],
  [`sc-${k}b-s1`, `sc-${k}b`, 0.42, TIP_AZ[k]! + 36, 34, 20, 0.62, 0.022, 0.01, 8],
]);
const SIDES_100: SideRow[] = SIDES_50.filter((r) => r[0].endsWith("a-s1") || r[0].endsWith("a-s2")).map((r) => [r[0], r[1], r[2], r[3], r[4], r[5], r[6] * 0.95, 0.032, 0.014, r[9]]);

/** 🍃️ The lobes of one scale: the 1:50 clumps carry every boss; 1:100 keeps the outward, upper and side bosses, 1:200 the outward and upper boss, 1:500 the main masses only, widened and bound to the central upright as the drawing binds them. */
function crown(scale: number): Lobe[] {
  const keep = scale === 50 ? null : scale === 100 ? "123457" : scale === 200 ? "146" : "";
  const grow = scale === 50 ? 1 : scale === 100 ? 1.04 : scale === 200 ? 1.14 : 1.3;
  const bossGrow = scale === 50 ? 1 : scale === 100 ? 1.05 : 1.2;
  const bone = scale === 500 ? "up-c" : undefined;
  const out = clumps(CLUMPS, { reach: (az, y) => reach(az, y) * (scale === 500 ? 1.018 : scale === 200 ? 1.0 : scale === 100 ? 0.99 : 1), axis: [0.2, 0.1], keep, grow, bossGrow, shape: [0.92, 0.72, 0.98], inset: scale === 500 ? 0 : 0.2, material: "cherry-leaf-sun", tone: TONE.sun, bone });
  out.push(ball("cap", bone ?? "fol-top", "cherry-leaf-sun", TONE.sun, "sun", CAP[0], CAP[1].map((r, i) => (i === 1 ? r : r * grow)) as V3));
  if (scale <= 100) CAP_BOSSES.forEach(([az, r, y, rb], i) => out.push(ball(`cap-b${i + 1}`, "fol-top", "cherry-leaf-sun", TONE.sun, "sun", [0.45 + r * Math.cos(az * D2R), y, 0.25 + r * Math.sin(az * D2R)], [rb * bossGrow, rb * 0.82 * bossGrow, rb * bossGrow])));
  const cores = scale === 50 ? CORES : scale === 100 ? CORES.filter((_, i) => i % 2 === 0) : scale === 200 ? [CORES[0]!, CORES[2]!, CORES[4]!] : [CORES[0]!];
  for (const [id, az, r, y] of cores) {
    const c: V3 = scale === 500 ? [0.35, 5.3, 0.2] : [0.35 + r * Math.cos(az * D2R), y, 0.2 + r * Math.sin(az * D2R)];
    out.push(ball(id, bone ?? "fol-core", "cherry-leaf-shade", TONE.shade, "shade", c, id === "core-0" ? [2.6, 0.9, 2.6] : [1.15, 0.85, 1.15]));
  }
  return out;
}

/** 🌸️ Hanging clusters of double flowers: per clump the faces they hang from, fewer and larger at the coarser scales. */
function flowers(scale: number, lobes: Lobe[]): Extra[] {
  const r = scale === 50 ? 0.1 : scale === 100 ? 0.12 : scale === 200 ? 0.15 : 0.16;
  const rows: [string, number, number][] = [];
  for (const [id, , az] of CLUMPS) {
    if (scale === 50) rows.push([`${id}-m`, az + 14, -38], [`${id}-b1`, az - 10, -22], [`${id}-b3`, az + 6, -48], [`${id}-b2`, az - 4, 28], [`${id}-b4`, az + 38, -8], [`${id}-b5`, az - 38, -12]);
    else if (scale === 100) rows.push([`${id}-m`, az + 14, -38], [`${id}-b1`, az - 10, -18], [`${id}-b2`, az, 30]);
    else if (scale === 200) rows.push([`${id}-m`, az + 6, -30], ...(id.startsWith("u") ? [[`${id}-m`, az, 45] as [string, number, number]] : []));
    else if (id.startsWith("w")) rows.push([`${id}-m`, az, -20]);
  }
  if (scale <= 100) rows.push(["cap", 40, 60], ["cap", 200, 62], ["cap", 300, 58]);
  return onLobes(rows, lobes, (id, bone, at, d) => ball(id, bone, "cherry-blossom", TONE.flower, "flower", add(at, mul(d, scale === 500 ? -2.5 * r : -0.25 * r)), [r, r * 0.85, r]), "bloom");
}

function outlets(scale: number): Outlet[] {
  if (scale === 50) return [
    ...K.map((k): Outlet => ["transpiration-sun", `tp-sun-${k}`, `w${k}a-b2`, TIP_AZ[k]! - 17, 50]),
    ["transpiration-sun", "tp-sun-6", "cap", 0, 90],
    ...CORES.slice(1).map(([id, az], i): Outlet => ["transpiration-shade", `tp-shade-${i + 1}`, id, az, 80]),
  ];
  if (scale === 100) return [["transpiration-sun", null, "w1a-b2", 0, 50], ["transpiration-sun", null, "w3a-b2", 146, 50], ["transpiration-sun", null, "cap", 0, 90], ["transpiration-shade", null, "core-2", 110, 80], ["transpiration-shade", null, "core-4", 255, 80]];
  if (scale === 200) return [["transpiration-sun", null, "u1-b6", 40, 80], ["transpiration-sun", null, "cap", 0, 90], ["transpiration-shade", null, "core-2", 110, 80]];
  return [["transpiration-sun", null, "cap", 0, 90], ["transpiration-shade", null, "core-0", 0, 90]];
}

function model(scale: number, ctx: Context): Model {
  const keep = scale === 50 ? null : scale === 100 ? [0, 2, 4, 6, 8] : scale === 200 ? [0, 4, 8] : [0, 8];
  const trunkRows = ctx.skeleton.skeleton.trunk.points.map((p: V3, i: number) => [p[1], ctx.skeleton.skeleton.trunk.radii[i], p[0], p[2]] as [number, number, number, number]);
  const trunk = scale === 50 ? trunkRows : scale === 100 ? trunkRows.filter((_: unknown, i: number) => i !== 2) : [trunkRows[0], trunkRows[3], trunkRows[4], trunkRows[5]];
  const wood = [stemTube(ctx, "root-plate", TONE.root, "root", FLARE[scale]!), stemTube(ctx, "trunk", TONE.stem, "stem", trunk)];
  for (const k of K) wood.push(limbTube(ctx, `sc-${k}a`, keep, TONE.limb), limbTube(ctx, `sc-${k}b`, scale === 500 ? [0, 4, 7] : keep, TONE.limb));
  for (const id of ["up-c", "up-1", "up-3"]) wood.push(limbTube(ctx, id, keep, TONE.limb));
  if (scale === 50) wood.push(...sides(SIDES_50, wood, "cherry-bark-young", TONE.limb, 6));
  if (scale === 100) wood.push(...sides(SIDES_100, wood, "cherry-bark-young", TONE.limb, 4));
  const lobes = crown(scale);
  const extras = flowers(scale, lobes);
  const hidden = scale === 500 ? wood.filter((t) => /b$/.test(t.id) || t.id === "up-1" || t.id === "up-3") : [];
  return { scale, wood: wood.filter((t) => !hidden.includes(t)), twigs: [], hiddenWood: hidden, hiddenTwigs: [], lobes, extras: scale === 500 ? [] : extras, hiddenExtras: scale === 500 ? extras : [], outlets: outlets(scale), ...(scale === 500 ? { crownBone: "up-c" } : {}) };
}

/** 🍃️ An ovate-acuminate cherry leaf of 11 cm with its long drip tip. */
function leaf(c: P2, deg: number): P2[] {
  const half: [number, number][] = [[0, 0], [0.012, 0.016], [0.035, 0.026], [0.062, 0.022], [0.088, 0.01], [0.11, 0]];
  const pts = [...half, ...half.slice(1, -1).reverse().map(([x, y]) => [x, -y] as [number, number])];
  const [cs, sn] = [Math.cos((deg * Math.PI) / 180), Math.sin((deg * Math.PI) / 180)];
  return pts.map(([x, y]) => ({ x: c.x + (x - 0.055) * cs - y * sn, y: c.y + (x - 0.055) * sn + y * cs }));
}

/** 🟫️ Horizontal lenticel bands across the glossy stem and the graft line under the fork. */
function bark(view: "elevation" | "front" | "plan", scale: number, _ctx: Context): { id: string; bone: string; role: string; points: V3[] }[] {
  const step = scale === 50 ? 0.11 : 0.2;
  const out: { id: string; bone: string; role: string; points: V3[] }[] = [];
  const at = (s: number, y: number): V3 => (view === "elevation" ? [0.01 + s, y, 0] : [0, y, s]);
  let i = 0;
  for (let y = 0.5; y < 1.62; y += step, i++) {
    const w = 0.15 * (0.55 + 0.3 * Math.abs(Math.sin(i * 1.9)));
    const off = 0.04 * Math.sin(i * 2.7);
    out.push({ id: `lenticel-${i + 1}`, bone: "trunk", role: "texture", points: [at(off - w / 2, y), at(off, y + 0.004), at(off + w / 2, y)] });
  }
  out.push({ id: "graft-line", bone: "trunk", role: scale === 50 ? "detail" : "texture", points: [at(-0.19, 1.73), at(-0.08, 1.755), at(0.06, 1.75), at(0.19, 1.725)] });
  return out;
}

const n2 = (n: number): string => String(n);
function texts(scale: number, c: Counts): { drawn: Record<string, Text>; volume: Text[] } {
  const mm = { 50: 20, 100: 10, 200: 5, 500: 2 }[scale]!;
  const pen = { 50: ["0,25", "0.25"], 100: ["0,2", "0.2"], 200: ["0,18", "0.18"], 500: ["0,15", "0.15"] }[scale]!;
  const drawn: Record<number, Record<string, Text>> = {
    50: {
      silhouette: { en: `At 1:50 (1 m is ${mm} mm) the three drawings are the true views of the 1:50 body: the root flare, the glossy stem with its lenticel bands and the graft line, the five scaffolds rising in a funnel with their side branches, the central upright, and the vase crown as one outline per foliage mass with leaf-cluster arcs, ovate leaf marks and every flower cluster seen from the viewpoint (${c.seen.elevation} in the side elevation, ${c.seen.front} in the front elevation, ${c.seen.plan} in the plan).`, de: `Bei 1:50 (1 m sind ${mm} mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: der Wurzelanlauf, der glänzende Stamm mit Lentizellenbändern und Veredelungslinie, die fünf trichterförmig aufsteigenden Leitäste mit ihren Seitenästen, der mittlere aufrechte Ast und die Vasenkrone als ein Umriss je Laubmasse mit Blattbüschelbögen, Blattzeichen und jedem aus der Blickrichtung sichtbaren Blütenbüschel (${c.seen.elevation} in der Seitenansicht, ${c.seen.front} in der Vorderansicht, ${c.seen.plan} in der Draufsicht).` },
      retained: { en: "Retained: every woody member of the rig and the fifteen side branches, every foliage mass with its bosses, the shade core, the visible flower clusters at their true size of 20 cm.", de: "Beibehalten: jedes Holzglied des Rigs und die fünfzehn Seitenäste, jede Laubmasse mit ihren Ausbuchtungen, der Schattenkern, die sichtbaren Blütenbüschel in ihrer wahren Größe von 20 cm." },
      merged: { en: "The single flowers (5 to 6 cm) are merged into clusters of three to five; the lenticels into short bands; the leaves into lobed masses.", de: "Die Einzelblüten (5 bis 6 cm) sind zu Büscheln aus drei bis fünf vereint, die Lentizellen zu kurzen Bändern, die Blätter zu gelappten Massen." },
      omitted: { en: "Omitted: single flowers, flower stalks, buds, twigs thinner than 2 cm and the several thousand single leaves.", de: "Weggelassen: Einzelblüten, Blütenstiele, Knospen, Zweige unter 2 cm und die mehreren tausend Einzelblätter." },
      thinElements: { en: "Lenticel bands and leaf-cluster arcs are 0.13 mm texture strokes, leaf marks 0.18 mm outlines; side branch ends of 2 cm are 0.5 mm wide on paper and stay filled.", de: "Lentizellenbänder und Blattbüschelbögen sind 0,13-mm-Texturstriche, Blattzeichen 0,18-mm-Umrisse; Seitenastenden von 2 cm sind auf dem Papier 0,5 mm breit und bleiben gefüllt." },
      hatching: { en: "A thin 45 degree line hatch (0.8 mm spacing) marks the shaded inner crown; bark and leaves are drawn with strokes.", de: "Eine feine 45-Grad-Schraffur (0,8 mm Abstand) kennzeichnet die beschattete Innenkrone; Rinde und Blätter werden mit Strichen gezeichnet." },
    },
    100: {
      silhouette: { en: `At 1:100 (1 m is ${mm} mm) the drawings are the true views of the 1:100 body: root flare, stem with lenticel bands, the funnel of five scaffolds with ten side branches, and the vase crown as foliage masses with broader lobes, leaf-cluster arcs and the visible flower clusters (${c.seen.elevation} in the side elevation, ${c.seen.front} in the front elevation, ${c.seen.plan} in the plan).`, de: `Bei 1:100 (1 m sind ${mm} mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: Wurzelanlauf, Stamm mit Lentizellenbändern, der Trichter aus fünf Leitästen mit zehn Seitenästen und die Vasenkrone als Laubmassen mit breiteren Lappen, Blattbüschelbögen und den sichtbaren Blütenbüscheln (${c.seen.elevation} in der Seitenansicht, ${c.seen.front} in der Vorderansicht, ${c.seen.plan} in der Draufsicht).` },
      retained: { en: "Retained: stem, scaffolds, uprights and the ascending side branches, the foliage masses with their outward and side bosses, three shade cores and the flower clusters of 24 cm.", de: "Beibehalten: Stamm, Leitäste, aufrechte Äste und die aufsteigenden Seitenäste, die Laubmassen mit ihren äußeren und seitlichen Ausbuchtungen, drei Schattenkerne und die Blütenbüschel von 24 cm." },
      merged: { en: "Two neighbouring flower clusters merge into one; the upward bosses merge into the masses; the outer side branches into the foliage.", de: "Zwei benachbarte Blütenbüschel verschmelzen zu einem; die oberen Ausbuchtungen gehen in den Massen auf, die äußeren Seitenäste im Laub." },
      omitted: { en: "Omitted: leaf marks, the graft line as a detail stroke (it is texture here), single flowers and leaves.", de: "Weggelassen: Blattzeichen, die Veredelungslinie als Detailstrich (hier ist sie Textur), Einzelblüten und Blätter." },
      thinElements: { en: "Thin strokes are at least 0.1 mm: lenticel bands use the texture pen, arcs the detail pen; branches keep their real width.", de: "Dünne Striche sind mindestens 0,1 mm: Lentizellenbänder mit dem Texturstift, Bögen mit dem Detailstift; Äste behalten ihre reale Breite." },
      hatching: { en: "Restrained: only the shaded inner crown carries a thin 45 degree hatch (0.6 mm spacing); no other hatching.", de: "Zurückhaltend: nur die beschattete Innenkrone trägt eine feine 45-Grad-Schraffur (0,6 mm Abstand); sonst keine Schraffur." },
    },
    200: {
      silhouette: { en: `At 1:200 (1 m is ${mm} mm) the drawings are the true views of the 1:200 body: stem with root flare, the funnel of scaffolds and uprights, and the vase crown as foliage masses with their outward and upper bosses; the flower clusters are single dots of 0.75 mm (${c.seen.elevation} in the side elevation, ${c.seen.front} in the front elevation, ${c.seen.plan} in the plan).`, de: `Bei 1:200 (1 m sind ${mm} mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: Stamm mit Wurzelanlauf, der Trichter aus Leitästen und aufrechten Ästen und die Vasenkrone als Laubmassen mit ihren äußeren und oberen Ausbuchtungen; die Blütenbüschel sind einzelne Punkte von 0,75 mm (${c.seen.elevation} in der Seitenansicht, ${c.seen.front} in der Vorderansicht, ${c.seen.plan} in der Draufsicht).` },
      retained: { en: "Retained: stem, root flare, the ten scaffold parts with three sections each, the three uprights, the foliage masses, two shade cores and one flower cluster per clump.", de: "Beibehalten: Stamm, Wurzelanlauf, die zehn Leitastteile mit je drei Schnitten, die drei aufrechten Äste, die Laubmassen, zwei Schattenkerne und ein Blütenbüschel je Laubklumpen." },
      merged: { en: "Side branches merge into the foliage masses; the flowers of a clump into one cluster of 30 cm.", de: "Seitenäste gehen in den Laubmassen auf; die Blüten eines Klumpens in einem Büschel von 30 cm." },
      omitted: { en: "Omitted: bark marks, arcs, leaf marks and the cap bosses.", de: "Weggelassen: Rindenzeichen, Bögen, Blattzeichen und die Ausbuchtungen der Kappe." },
      thinElements: { en: "No element is thinner than 0.35 mm on paper; nothing is thickened.", de: "Kein Element ist auf dem Papier dünner als 0,35 mm; nichts wird verdickt." },
      hatching: { en: "No hatching; the masses are separated by their outlines.", de: "Keine Schraffur; die Massen werden durch ihre Umrisse getrennt." },
    },
    500: {
      silhouette: { en: `At 1:500 (1 m is ${mm} mm) the cherry is one closed crown, 16 mm wide, funnel-shaped below and flat-domed above, on the visible funnel of the five lower scaffolds and the stem with its root flare; the drawings are the true views of the 1:500 body.`, de: `Bei 1:500 (1 m sind ${mm} mm) ist die Kirsche eine geschlossene Krone von 16 mm Breite, unten trichterförmig und oben flach gewölbt, über dem sichtbaren Trichter der fünf unteren Leitäste und dem Stamm mit Wurzelanlauf; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers.` },
      retained: { en: "Retained: stem with root flare, the lower scaffold parts and the central upright below the crown, and the crown outline.", de: "Beibehalten: Stamm mit Wurzelanlauf, die unteren Leitastteile und der mittlere aufrechte Ast unter der Krone sowie der Kronenumriss." },
      merged: { en: "Merged into the crown path: every foliage mass, the shade core, the outer scaffold parts, the upright shoots and the flower clusters (all regions of the crown are listed on it).", de: "Im Kronenpfad vereint: jede Laubmasse, der Schattenkern, die äußeren Leitastteile, die aufrechten Triebe und die Blütenbüschel (alle Regionen der Krone sind daran genannt)." },
      omitted: { en: "Omitted: every single flower cluster as a mark and every internal edge.", de: "Weggelassen: jedes einzelne Blütenbüschel als Zeichen und jede innere Kante." },
      thinElements: { en: "The lower scaffolds are 0.4 mm wide on paper and stay filled; nothing is thickened.", de: "Die unteren Leitäste sind auf dem Papier 0,4 mm breit und bleiben gefüllt; nichts wird verdickt." },
      hatching: { en: "No hatching; the pen is 0.15 mm for outlines.", de: "Keine Schraffur; der Stift ist 0,15 mm für Umrisse." },
    },
  };
  const d = drawn[scale]!;
  d.styles = { en: `Light line: outlines of ${pen[1]} mm and the finer pens of this scale. Dark solid: the crown reads in a mid tone, the flower clusters light and the wood darker.`, de: `Helle Linie: Umrisse von ${pen[0]} mm und die feineren Stifte dieses Maßstabs. Dunkle Fläche: die Krone liest sich im Mittelton, die Blütenbüschel hell und das Holz dunkler.` };
  d.overlays = scale === 500
    ? { en: "Overlays map onto trunk-bark (stem, root flare) and, merged in the crown path, limb-bark, leaf-sun, leaf-shade and blossom; merged shapes show the area-weighted value of their regions.", de: "Überlagerungen bilden sich ab auf trunk-bark (Stamm, Wurzelanlauf) sowie, im Kronenpfad vereint, limb-bark, leaf-sun, leaf-shade und blossom; vereinte Formen zeigen den flächengewichteten Wert ihrer Regionen." }
    : { en: "Every thermal region has its own shapes: stem and root flare trunk-bark, scaffolds and uprights limb-bark, outer masses leaf-sun, the core leaf-shade, the flower clusters blossom; marks without a region follow their bone.", de: "Jede thermische Region hat eigene Formen: Stamm und Wurzelanlauf trunk-bark, Leitäste und aufrechte Äste limb-bark, äußere Massen leaf-sun, der Kern leaf-shade, die Blütenbüschel blossom; Zeichen ohne Region folgen ihrem Knochen." };
  const volume: Record<number, Text[]> = {
    50: [
      { en: `Body at 1:50: root flare, stem with the graft swelling under the fork, five scaffolds in two parts (swept lofts, ascending at about 30 degrees from the vertical and arching out), the central upright, two upright shoots, fifteen side branches, and a funnel-and-dome crown of ${n2(c.lobes)} leaf lobes (twenty clumps with their bosses, a flat cap with five bosses, five shade lobes) hung with ${n2(c.extras)} clusters of double flowers of 20 cm.`, de: `Körper bei 1:50: Wurzelanlauf, Stamm mit der Veredelungswulst unter der Verzweigung, fünf Leitäste in zwei Teilen (geführte Lofts, mit etwa 30 Grad gegen die Senkrechte aufsteigend und überbiegend), der mittlere aufrechte Ast, zwei aufrechte Triebe, fünfzehn Seitenäste und eine Trichter- und Kuppelkrone aus ${n2(c.lobes)} Laublappen (zwanzig Klumpen mit ihren Ausbuchtungen, eine flache Kappe mit fünf Ausbuchtungen, fünf Schattenlappen), behängt mit ${n2(c.extras)} Büscheln gefüllter Blüten von 20 cm.` },
      { en: "Thermal at 1:50: trunk bark is the root flare and the stem; limb bark the scaffolds, uprights and side branches; the outer lobes are the sun leaves, the inner lobes the shade leaves and the clusters the blossom, all by reference to the body; eleven transpiration outlets are lens patches in the free faces of the lobes that carry the emitter anchors.", de: "Thermik bei 1:50: Stammrinde sind Wurzelanlauf und Stamm; Astrinde die Leitäste, aufrechten Äste und Seitenäste; die äußeren Lappen sind die Sonnenblätter, die inneren die Schattenblätter und die Büschel die Blüte, alle als Verweis auf den Körper; elf Transpirationsauslässe sind Linsenflächen in den freien Seiten der Lappen, die die Emitteranker tragen." },
      { en: "Structure at 1:50: the wood under the bark of every load-bearing bone along the real limb axes, joints at the rig joints, the round fixed root plate and the masses of the root plate, the stem and the seven foliage masses.", de: "Tragwerk bei 1:50: das Holz unter der Rinde jedes tragenden Knochens entlang der wirklichen Astachsen, Gelenke an den Rig-Gelenken, der runde eingespannte Wurzelteller und die Massen von Wurzelteller, Stamm und den sieben Laubmassen." },
    ],
    100: [
      { en: `Body at 1:100: a simpler root flare, every limb with five sections, ten ascending side branches, ${n2(c.lobes)} broader lobes (the clumps keep their outward and side bosses) and ${n2(c.extras)} flower clusters of 24 cm.`, de: `Körper bei 1:100: ein einfacherer Wurzelanlauf, jeder Ast mit fünf Schnitten, zehn aufsteigende Seitenäste, ${n2(c.lobes)} breitere Lappen (die Klumpen behalten ihre äußeren und seitlichen Ausbuchtungen) und ${n2(c.extras)} Blütenbüschel von 24 cm.` },
      { en: "Thermal at 1:100: the same five regions by reference to this scale's body; five outlets as lens patches.", de: "Thermik bei 1:100: dieselben fünf Regionen als Verweis auf den Körper dieses Maßstabs; fünf Auslässe als Linsenflächen." },
      { en: "Structure at 1:100: the wood of the load-bearing bones with simplified taper, joints, the root plate and the masses.", de: "Tragwerk bei 1:100: das Holz der tragenden Knochen mit vereinfachter Verjüngung, Gelenke, der Wurzelteller und die Massen." },
    ],
    200: [
      { en: `Body at 1:200: root flare, stem, the ten scaffold parts and three uprights with three sections each, no side branches; ${n2(c.lobes)} masses and ${n2(c.extras)} flower clusters of 30 cm.`, de: `Körper bei 1:200: Wurzelanlauf, Stamm, die zehn Leitastteile und drei aufrechten Äste mit je drei Schnitten, keine Seitenäste; ${n2(c.lobes)} Massen und ${n2(c.extras)} Blütenbüschel von 30 cm.` },
      { en: "Thermal at 1:200: bark, leaves and blossom by reference to the body; three outlets.", de: "Thermik bei 1:200: Rinde, Laub und Blüte als Verweis auf den Körper; drei Auslässe." },
      { en: "Structure at 1:200: the wood of the load-bearing bones, joints where they are at least one smallest feature thick, the root plate and the masses.", de: "Tragwerk bei 1:200: das Holz der tragenden Knochen, Gelenke dort, wo sie mindestens ein kleinstes Merkmal dick sind, der Wurzelteller und die Massen." },
    ],
    500: [
      { en: `Body at 1:500: root flare, stem, the lower scaffold parts and the central upright as plain lofts, and the crown as ${n2(c.lobes)} widened masses bound to the central upright as the drawing binds it.`, de: `Körper bei 1:500: Wurzelanlauf, Stamm, die unteren Leitastteile und der mittlere aufrechte Ast als einfache Lofts und die Krone als ${n2(c.lobes)} verbreiterte Massen, an den mittleren aufrechten Ast gebunden wie in der Zeichnung.` },
      { en: "Thermal at 1:500: bark and leaves by reference; the outer scaffold parts, the upright shoots and ten flower clusters are thermal solids of their own inside the crown; two outlets.", de: "Thermik bei 1:500: Rinde und Laub als Verweis; die äußeren Leitastteile, die aufrechten Triebe und zehn Blütenbüschel sind eigene Thermikkörper in der Krone; zwei Auslässe." },
      { en: "Structure at 1:500: the load-bearing bones as members of their declared section, the joint of the stem, the root plate and the masses of root plate and stem; this list is already minimal.", de: "Tragwerk bei 1:500: die tragenden Knochen als Stäbe ihres angegebenen Querschnitts, das Stammgelenk, der Wurzelteller und die Massen von Wurzelteller und Stamm; diese Liste ist bereits minimal." },
    ],
  };
  return { drawn: d, volume: volume[scale]! };
}

export const species: Species = {
  id: "japanese-cherry",
  trunk: "trunk",
  woodMaterial: "cherry-wood",
  extraRegion: "blossom",
  leafGlyph: leaf,
  extraGlyph: "cluster",
  bark,
  model,
  texts,
  areaOverrides: { "trunk-bark": 3.0 },
  scaffolds: ["sc-1a", "sc-2a", "sc-3a", "sc-4a", "sc-5a"],
  symbolTexts: {
    plan: {
      title: { en: "Plan symbol with true crown extent", de: "Grundrisssymbol mit wahrer Kronenausdehnung" },
      description: { en: "A conventional sign for site and planting plans: the true outline of the vase crown in the three views, the stem at its real position and section, and in plan the directions of the five scaffolds. It is a separate symbol and is never used silently for a pictorial scale.", de: "Ein konventionelles Zeichen für Lage- und Pflanzpläne: der wahre Umriss der Vasenkrone in den drei Ansichten, der Stamm an seiner wahren Lage und mit seinem Querschnitt und im Grundriss die Richtungen der fünf Leitäste. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet." },
      dimensionalNote: { en: "The outlines are the true extent of the 1:100 body in model metres (about 8.1 m wide, 7.9 m deep and 8.0 m high); the filled shapes are the stem and root flare at their real size; the five strokes only indicate scaffold directions. Planting distances are not part of the symbol.", de: "Die Umrisse sind die wahre Ausdehnung des 1:100-Körpers in Modellmetern (etwa 8,1 m breit, 7,9 m tief und 8,0 m hoch); die gefüllten Formen sind Stamm und Wurzelanlauf in wahrer Größe; die fünf Striche deuten nur die Leitastrichtungen an. Pflanzabstände gehören nicht zum Symbol." },
    },
    sign: {
      title: { en: "Nominal flowering tree sign", de: "Nominelles Zeichen eines Blütenbaums" },
      description: { en: "A circle with a centre dot and an inner ring as the conventional sign of an ornamental tree in small-scale plans, drawn as plan circle and as side and front lollipop; the size is a fixed paper size.", de: "Ein Kreis mit Mittelpunkt und Innenring als übliches Zeichen eines Zierbaums in kleinmaßstäblichen Plänen, als Grundrisskreis und als seitlicher und vorderer Lolli gezeichnet; die Größe ist eine feste Papiergröße." },
      dimensionalNote: { en: "A fixed 3.2 mm circle with a 1.8 mm inner ring and a 0.7 mm centre dot in paper millimetres; its size carries no dimension. Use the true-extent symbol where the crown width matters.", de: "Ein fester Kreis von 3,2 mm mit 1,8 mm Innenring und 0,7 mm Mittelpunkt in Papiermillimetern; seine Größe trägt kein Maß. Wo die Kronenbreite zählt, ist das Symbol mit wahrer Ausdehnung zu verwenden." },
    },
  },
};
