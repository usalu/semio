/** 🍏️ crab-apple design: per-scale wood of the short leaning stem, four spreading limbs, the leader and the thorny spurs, the dense rounded crown of clumps, the small hanging crab apples, bark plates, outlets and the decision texts of a Malus sylvestris with fruit.
 * @see ./sp_crab_apple.py — rig, masses and physics
 * @see ./pw03-engine.ts */
import { D2R, add, mul, type Lobe, type Tube } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts";
import type { P2, V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";
import { ball, clumps, limbTube, onLobes, sides, stemTube, type ClumpRow, type Context, type Counts, type Extra, type Model, type Outlet, type SideRow, type Species, type Text } from "./pw03-engine.ts";

const TONE = { root: 0.3, stem: 0.34, limb: 0.38, twig: 0.22, sun: 0.58, shade: 0.44, fruit: 0.7 };
const K = [1, 2, 3, 4];
const LIMB_AZ: Record<number, number> = { 1: 25, 2: 100, 3: 198, 4: 275 };

/** 🌳️ Crown envelope: a dense, broad dome from 1.8 m with a slightly drooping skirt, widest at 3.9 m (3.75 m, a little more towards the lean), top at 7.0 m. */
const reach = (az: number, y: number): number => (3.75 + 0.12 * Math.cos((az - 20) * D2R)) * (y >= 3.9 ? Math.max(0.05, 1 - ((y - 3.9) / 3.1) ** 2.3) ** 0.45 : Math.sqrt(Math.max(0.08, 1 - ((3.9 - y) / 2.1) ** 2)));

const CLUMPS: ClumpRow[] = [
  ...K.flatMap((k): ClumpRow[] => [
    [`s${k}`, `fol-${k}`, LIMB_AZ[k]! + 8, 2.75, 0.82, 0.98, "13", 18],
    [`w${k}a`, `fol-${k}`, LIMB_AZ[k]! - 14, 3.95, 1.05, 1, "12345", 6],
    [`w${k}b`, `fol-${k}`, LIMB_AZ[k]! + 30, 3.8, 0.95, 0.99, "1234", 8],
    [`u${k}`, `fol-${k}`, LIMB_AZ[k]! + 42, 5.05, 0.95, 1, "12456", 4],
  ]),
  ["t1", "fol-top", 60, 6.0, 0.85, 0.98, "1246", 2], ["t2", "fol-top", 240, 6.0, 0.85, 0.98, "1256", 2], ["t3", "fol-top", 150, 6.05, 0.8, 0.98, "1246", 2], ["t4", "fol-top", 330, 6.05, 0.8, 0.98, "1256", 2],
];
const CAP: [V3, V3] = [[-0.1, 6.15, -0.05], [1.45, 0.82, 1.45]];
const CAP_BOSSES: [number, number, number, number][] = [[10, 0.72, 6.6, 0.34], [100, 0.7, 6.58, 0.33], [190, 0.72, 6.6, 0.34], [280, 0.7, 6.58, 0.33]];
const CORES: [string, number, number, number][] = [["core-0", 0, 0, 4.1], ["core-1", 60, 1.2, 3.95], ["core-2", 150, 1.15, 4.35], ["core-3", 240, 1.2, 3.95], ["core-4", 330, 1.15, 4.35]];
const FLARE: Record<number, [number, number, number, number][]> = {
  50: [[0, 0.38, 0, 0], [0.05, 0.32, 0, 0], [0.12, 0.27, 0.001, 0], [0.22, 0.232, 0.002, -0.001], [0.3, 0.214, 0.003, -0.002], [0.4, 0.203, 0.005, -0.002]],
  100: [[0, 0.38, 0, 0], [0.08, 0.29, 0, 0], [0.22, 0.232, 0.002, -0.001], [0.4, 0.203, 0.005, -0.002]],
  200: [[0, 0.36, 0, 0], [0.12, 0.26, 0.001, 0], [0.4, 0.204, 0.005, -0.002]],
  500: [[0, 0.32, 0, 0], [0.4, 0.205, 0.005, -0.002]],
};
const SIDES_50: SideRow[] = [
  ...K.flatMap((k): SideRow[] => [
    [`sc-${k}a-s1`, `sc-${k}a`, 0.5, LIMB_AZ[k]! + 60, 35, 20, 1.4, 0.035, 0.012, 10],
    [`sc-${k}a-s2`, `sc-${k}a`, 0.8, LIMB_AZ[k]! - 55, 30, 12, 1.0, 0.03, 0.011, -10],
    [`sc-${k}b-s1`, `sc-${k}b`, 0.35, LIMB_AZ[k]! + 35, 25, 10, 1.0, 0.024, 0.009, 8],
  ]),
  ["ld-s1", "ld", 0.45, 30, 40, 25, 1.4, 0.03, 0.012, 10], ["ld-s2", "ld", 0.68, 255, 42, 28, 1.3, 0.028, 0.011, -10],
];
const SIDES_100: SideRow[] = SIDES_50.filter((r) => !r[0].includes("b-s")).map((r) => [r[0], r[1], r[2], r[3], r[4], r[5], r[6] * 0.95, 0.03, 0.014, r[9]]);

function crown(scale: number): Lobe[] {
  const keep = scale === 50 ? null : scale === 100 ? "123457" : scale === 200 ? "146" : "";
  const grow = scale === 50 ? 1 : scale === 100 ? 1.04 : scale === 200 ? 1.14 : 1.3;
  const bossGrow = scale === 50 ? 1 : scale === 100 ? 1.05 : 1.2;
  const bone = scale === 500 ? "ld" : undefined;
  const out = clumps(CLUMPS, { reach: (az, y) => reach(az, y) * (scale === 500 ? 1.018 : scale === 100 ? 0.99 : 1), axis: [0.1, -0.03], keep, grow, bossGrow, shape: [0.9, 0.72, 0.96], inset: scale === 500 ? 0 : 0.2, material: "crab-leaf-sun", tone: TONE.sun, bone });
  out.push(ball("cap", bone ?? "fol-top", "crab-leaf-sun", TONE.sun, "sun", CAP[0], CAP[1].map((r, i) => (i === 1 ? r : r * grow)) as V3));
  if (scale <= 100) CAP_BOSSES.forEach(([az, r, y, rb], i) => out.push(ball(`cap-b${i + 1}`, "fol-top", "crab-leaf-sun", TONE.sun, "sun", [-0.1 + r * Math.cos(az * D2R), y, -0.05 + r * Math.sin(az * D2R)], [rb * bossGrow, rb * 0.82 * bossGrow, rb * bossGrow])));
  const cores = scale === 50 ? CORES : scale === 100 ? CORES.filter((_, i) => i % 2 === 0) : scale === 200 ? [CORES[0]!, CORES[2]!, CORES[4]!] : [CORES[0]!];
  for (const [id, az, r, y] of cores) {
    const c: V3 = scale === 500 ? [0.05, 4.1, -0.05] : [0.05 + r * Math.cos(az * D2R), y, -0.05 + r * Math.sin(az * D2R)];
    out.push(ball(id, bone ?? "fol-core", "crab-leaf-shade", TONE.shade, "shade", c, id === "core-0" ? [2.6, 0.9, 2.6] : [1.2, 0.85, 1.2]));
  }
  return out;
}

/** 🍏️ Crab apples of 3 cm hanging under the clumps at 1:50 and 1:100; at 1:200 each clump carries one typified fruit cluster of 9 cm, at 1:500 four clusters lie inside the crown. */
function fruit(scale: number, lobes: Lobe[]): Extra[] {
  const r = scale <= 100 ? 0.015 : 0.045;
  const rows: [string, number, number][] = [];
  for (const [id, , az] of CLUMPS) {
    if (scale === 50) rows.push([`${id}-m`, az + 12, -42], [`${id}-m`, az - 16, -30], [`${id}-b1`, az - 6, -35], [`${id}-b1`, az + 10, -12], [`${id}-b3`, az + 4, -50], [`${id}-b4`, az + 40, -20], [`${id}-b5`, az - 40, -18], [`${id}-b2`, az + 6, 25]);
    else if (scale === 100) rows.push([`${id}-m`, az + 12, -42], [`${id}-b1`, az - 6, -30], [`${id}-b4`, az + 40, -18], [`${id}-b5`, az - 40, -18]);
    else if (scale === 200) rows.push([`${id}-m`, az + 6, -30]);
    else if (id.startsWith("w") && id.endsWith("a")) rows.push([`${id}-m`, az, -20]);
  }
  if (scale <= 100) rows.push(["cap", 40, 50], ["cap", 220, 52]);
  return onLobes(rows, lobes, (id, bone, at, d) => ball(id, bone, "crab-fruit", TONE.fruit, "fruit", add(at, mul(d, scale === 500 ? -3 * r : scale === 200 ? -0.3 * r : r * 0.2)), [r, r, r]), "fruit");
}

function outlets(scale: number): Outlet[] {
  if (scale === 50) return [
    ...K.map((k): Outlet => ["transpiration-sun", `tp-sun-${k}`, `w${k}a-b2`, LIMB_AZ[k]! - 14, 50]),
    ["transpiration-sun", "tp-sun-5", "cap", 0, 90],
    ...CORES.map(([id, az], i): Outlet => ["transpiration-shade", `tp-shade-${i + 1}`, id, az, 80]),
  ];
  if (scale === 100) return [["transpiration-sun", null, "w1a-b2", 11, 50], ["transpiration-sun", null, "w3a-b2", 184, 50], ["transpiration-sun", null, "cap", 0, 90], ["transpiration-shade", null, "core-2", 150, 80], ["transpiration-shade", null, "core-4", 330, 80]];
  if (scale === 200) return [["transpiration-sun", null, "u1-b6", 67, 80], ["transpiration-sun", null, "cap", 0, 90], ["transpiration-shade", null, "core-2", 150, 80]];
  return [["transpiration-sun", null, "cap", 0, 90], ["transpiration-shade", null, "core-0", 0, 90]];
}

/** 🌵️ A thorny spur shoot: the skeleton twig with three short thorns at 1:50. */
function spurs(ctx: Context, scale: number): Tube[] {
  const out: Tube[] = [];
  for (const k of K) {
    const t = limbTube(ctx, `spur-${k}`, scale === 50 ? null : [0, 2, 4], TONE.twig, 0.03, "twig");
    t.flags = { slender: true };
    out.push(t);
    if (scale !== 50) continue;
    [[1, 1, 0.05], [2, -1, 0.045], [3, 1, 0.04]].forEach(([i, s, L], j) => {
      const p = t.points[i! + 1]!.p;
      const tip: V3 = [p[0] + s! * L! * 0.7, p[1] + L! * 0.5, p[2] + s! * L! * 0.5];
      out.push({ kind: "tube", id: `spur-${k}-thorn${j + 1}`, bone: `spur-${k}`, material: "crab-twig", tone: TONE.twig, group: "twig", points: [{ p, r: 0.0055 }, { p: [(p[0] + tip[0]) / 2, (p[1] + tip[1]) / 2, (p[2] + tip[2]) / 2], r: 0.0045 }, { p: tip, r: 0.004 }] });
    });
  }
  return out;
}

function model(scale: number, ctx: Context): Model {
  const keep = scale === 50 ? null : scale === 100 ? [0, 2, 4, 6, 8] : scale === 200 ? [0, 4, 8] : [0, 8];
  const tr = ctx.skeleton.skeleton.trunk;
  const trunkRows = tr.points.map((p: V3, i: number) => [p[1], tr.radii[i], p[0], p[2]] as [number, number, number, number]);
  const trunk = scale <= 100 ? trunkRows : [trunkRows[0], trunkRows[2], trunkRows[4]];
  const wood = [stemTube(ctx, "root-plate", TONE.root, "root", FLARE[scale]!), stemTube(ctx, "trunk", TONE.stem, "stem", trunk)];
  for (const k of K) wood.push(limbTube(ctx, `sc-${k}a`, keep, TONE.limb), limbTube(ctx, `sc-${k}b`, scale === 500 ? [0, 4, 7] : keep, TONE.limb));
  wood.push(limbTube(ctx, "ld", keep, TONE.limb));
  if (scale === 50) wood.push(...sides(SIDES_50, wood, "crab-bark-young", TONE.limb, 6));
  if (scale === 100) wood.push(...sides(SIDES_100, wood, "crab-bark-young", TONE.limb, 4));
  const twigs = spurs(ctx, scale);
  const lobes = crown(scale);
  const extras = fruit(scale, lobes);
  const hidden = scale === 500 ? wood.filter((t) => /b$/.test(t.id)) : [];
  return { scale, wood: wood.filter((t) => !hidden.includes(t)), twigs: scale <= 100 ? twigs : [], hiddenWood: hidden, hiddenTwigs: scale <= 100 ? [] : twigs, lobes, extras: scale === 500 ? [] : extras, hiddenExtras: scale === 500 ? extras : [], outlets: outlets(scale), ...(scale === 500 ? { crownBone: "ld" } : {}) };
}

/** 🍃️ A small ovate crab-apple leaf of 6 cm with a short point. */
function leaf(c: P2, deg: number): P2[] {
  const half: [number, number][] = [[0, 0], [0.008, 0.012], [0.022, 0.02], [0.04, 0.019], [0.054, 0.01], [0.062, 0]];
  const pts = [...half, ...half.slice(1, -1).reverse().map(([x, y]) => [x, -y] as [number, number])];
  const [cs, sn] = [Math.cos((deg * Math.PI) / 180), Math.sin((deg * Math.PI) / 180)];
  return pts.map(([x, y]) => ({ x: c.x + (x - 0.031) * cs - y * sn, y: c.y + (x - 0.031) * sn + y * cs }));
}

/** 🟫️ The grey-brown stem bark cracks into small plates: vertical fissures crossed by short cracks. */
function bark(view: "elevation" | "front" | "plan", scale: number, _ctx: Context): { id: string; bone: string; role: string; points: V3[] }[] {
  const out: { id: string; bone: string; role: string; points: V3[] }[] = [];
  const lean = (y: number): number => 0.06 * Math.max(0, y - 0.35);
  const at = (s: number, y: number): V3 => (view === "elevation" ? [lean(y) + s, y, 0] : [0, y, s - lean(y) * 0.4]);
  const fissures = scale === 50 ? [-0.11, -0.04, 0.03, 0.1] : [-0.08, 0.02, 0.1];
  fissures.forEach((s, i) => {
    const y0 = 0.42 + 0.06 * (i % 2), y1 = 1.2 - 0.05 * (i % 3);
    out.push({ id: `fissure-${i + 1}`, bone: "trunk", role: "texture", points: [0, 1, 2, 3, 4].map((j) => at(s + 0.012 * Math.sin(j * 1.7 + i), y0 + ((y1 - y0) * j) / 4)) });
  });
  if (scale === 50) [0.62, 0.84, 1.05].forEach((y, i) => out.push({ id: `plate-crack-${i + 1}`, bone: "trunk", role: "texture", points: [at(-0.07 + 0.07 * (i % 2), y), at(-0.035 + 0.07 * (i % 2), y + 0.01)] }));
  return out;
}

function texts(scale: number, c: Counts): { drawn: Record<string, Text>; volume: Text[] } {
  const mm = { 50: 20, 100: 10, 200: 5, 500: 2 }[scale]!;
  const pen = { 50: ["0,25", "0.25"], 100: ["0,2", "0.2"], 200: ["0,18", "0.18"], 500: ["0,15", "0.15"] }[scale]!;
  const seen = (en: boolean): string => en ? `${c.seen.elevation} in the side elevation, ${c.seen.front} in the front elevation, ${c.seen.plan} in the plan` : `${c.seen.elevation} in der Seitenansicht, ${c.seen.front} in der Vorderansicht, ${c.seen.plan} in der Draufsicht`;
  const drawn: Record<number, Record<string, Text>> = {
    50: {
      silhouette: { en: `At 1:50 (1 m is ${mm} mm) the three drawings are the true views of the 1:50 body: the root flare, the short leaning stem with its plated bark, the four limbs spreading almost horizontally with their side branches, the central leader, the thorny spur shoots, and the dense rounded crown as one outline per foliage mass with leaf-cluster arcs, small ovate leaf marks and every crab apple seen from the viewpoint at its real size of 3 cm (${seen(true)}).`, de: `Bei 1:50 (1 m sind ${mm} mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: der Wurzelanlauf, der kurze geneigte Stamm mit seiner schuppigen Rinde, die vier fast waagerecht ausladenden Leitäste mit ihren Seitenästen, der Mitteltrieb, die dornigen Kurztriebe und die dichte, runde Krone als ein Umriss je Laubmasse mit Blattbüschelbögen, kleinen eiförmigen Blattzeichen und jedem aus der Blickrichtung sichtbaren Holzapfel in wahrer Größe von 3 cm (${seen(false)}).` },
      retained: { en: "Retained: every woody member of the rig and the fourteen side branches, the four spur shoots with three thorns each, every foliage mass with its bosses, the shade core and the visible crab apples.", de: "Beibehalten: jedes Holzglied des Rigs und die vierzehn Seitenäste, die vier Kurztriebe mit je drei Dornen, jede Laubmasse mit ihren Ausbuchtungen, der Schattenkern und die sichtbaren Holzäpfel." },
      merged: { en: "Bark plates are merged into four fissures and three cracks; the leaves into lobed masses; the fruit are a sample of the load, not a count.", de: "Rindenplatten sind in vier Risse und drei Querrisse zusammengefasst, die Blätter in gelappte Massen; die Früchte sind eine Auswahl der Last, keine Zählung." },
      omitted: { en: "Omitted: the several thousand single leaves, fruit stalks, buds and the remaining fruit (about 2700 hang on the tree).", de: "Weggelassen: die mehreren tausend Einzelblätter, Fruchtstiele, Knospen und die übrigen Früchte (etwa 2700 hängen am Baum)." },
      thinElements: { en: "Spur shoots of 2.6 cm are 0.13 mm texture strokes over their filled bodies, thorns of 8 mm are 0.13 mm strokes; fissures and arcs 0.13 mm, leaf marks 0.18 mm.", de: "Kurztriebe von 2,6 cm sind 0,13-mm-Texturstriche über ihren gefüllten Körpern, Dornen von 8 mm 0,13-mm-Striche; Risse und Bögen 0,13 mm, Blattzeichen 0,18 mm." },
      hatching: { en: "A thin 45 degree line hatch (0.8 mm spacing) marks the shaded inner crown; bark and leaves are drawn with strokes.", de: "Eine feine 45-Grad-Schraffur (0,8 mm Abstand) kennzeichnet die beschattete Innenkrone; Rinde und Blätter werden mit Strichen gezeichnet." },
    },
    100: {
      silhouette: { en: `At 1:100 (1 m is ${mm} mm) the drawings are the true views of the 1:100 body: root flare, the leaning stem with its fissures, the four spreading limbs, the leader and their inner side branches, the spur shoots, and the dense crown as foliage masses with broader lobes, leaf-cluster arcs and the visible crab apples as 0.3 mm dots (${seen(true)}).`, de: `Bei 1:100 (1 m sind ${mm} mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: Wurzelanlauf, der geneigte Stamm mit seinen Rissen, die vier ausladenden Leitäste, der Mitteltrieb und ihre inneren Seitenäste, die Kurztriebe und die dichte Krone als Laubmassen mit breiteren Lappen, Blattbüschelbögen und den sichtbaren Holzäpfeln als Punkte von 0,3 mm (${seen(false)}).` },
      retained: { en: "Retained: stem, limbs, leader, ten inner side branches, the spur shoots as single strokes, the foliage masses with their outward and side bosses, three shade cores and four fruit per clump.", de: "Beibehalten: Stamm, Leitäste, Mitteltrieb, zehn innere Seitenäste, die Kurztriebe als einzelne Striche, die Laubmassen mit ihren äußeren und seitlichen Ausbuchtungen, drei Schattenkerne und vier Früchte je Klumpen." },
      merged: { en: "The outer side branches merge into the foliage; the upward bosses into the masses.", de: "Die äußeren Seitenäste gehen im Laub auf, die oberen Ausbuchtungen in den Massen." },
      omitted: { en: "Omitted: thorns, plate cracks, leaf marks and most fruit.", de: "Weggelassen: Dornen, Querrisse, Blattzeichen und die meisten Früchte." },
      thinElements: { en: "Thin strokes are at least 0.1 mm: fissures use the texture pen, arcs and spurs the detail pen; branches keep their real width.", de: "Dünne Striche sind mindestens 0,1 mm: Risse mit dem Texturstift, Bögen und Kurztriebe mit dem Detailstift; Äste behalten ihre reale Breite." },
      hatching: { en: "Restrained: only the shaded inner crown carries a thin 45 degree hatch (0.6 mm spacing).", de: "Zurückhaltend: nur die beschattete Innenkrone trägt eine feine 45-Grad-Schraffur (0,6 mm Abstand)." },
    },
    200: {
      silhouette: { en: `At 1:200 (1 m is ${mm} mm) the drawings are the true views of the 1:200 body: root flare, the short stem, the spreading limbs and the leader under a dense rounded crown of foliage masses; each clump carries one typified fruit cluster of 9 cm, a 0.45 mm dot (${seen(true)}).`, de: `Bei 1:200 (1 m sind ${mm} mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: Wurzelanlauf, der kurze Stamm, die ausladenden Leitäste und der Mitteltrieb unter einer dichten, runden Krone aus Laubmassen; jeder Klumpen trägt ein typisiertes Fruchtbüschel von 9 cm, einen Punkt von 0,45 mm (${seen(false)}).` },
      retained: { en: "Retained: root flare, stem, the eight limb parts and the leader with three sections each, the foliage masses with their outward, side and upper bosses, two shade cores and one fruit cluster per clump.", de: "Beibehalten: Wurzelanlauf, Stamm, die acht Leitastteile und der Mitteltrieb mit je drei Schnitten, die Laubmassen mit ihren äußeren, seitlichen und oberen Ausbuchtungen, zwei Schattenkerne und ein Fruchtbüschel je Klumpen." },
      merged: { en: "The spur shoots merge into the foliage masses (spur-twig is a merged region of the sun masses); the single fruit of a clump into one typified cluster.", de: "Die Kurztriebe gehen in den Laubmassen auf (spur-twig ist eine vereinte Region der Sonnenmassen); die einzelnen Früchte eines Klumpens in einem typisierten Büschel." },
      omitted: { en: "Omitted: side branches, bark marks, arcs, leaf marks and the cap bosses.", de: "Weggelassen: Seitenäste, Rindenzeichen, Bögen, Blattzeichen und die Ausbuchtungen der Kappe." },
      thinElements: { en: "No element is thinner than 0.3 mm on paper; the spur shoots of 2.6 cm (0.13 mm) are not thickened but merged.", de: "Kein Element ist auf dem Papier dünner als 0,3 mm; die Kurztriebe von 2,6 cm (0,13 mm) werden nicht verdickt, sondern eingeschmolzen." },
      hatching: { en: "No hatching; the masses are separated by their outlines.", de: "Keine Schraffur; die Massen werden durch ihre Umrisse getrennt." },
    },
    500: {
      silhouette: { en: `At 1:500 (1 m is ${mm} mm) the crab apple is one closed, broad and dense crown, 15 mm wide, on a short leaning stem with its root flare and the visible bases of the four limbs; the drawings are the true views of the 1:500 body.`, de: `Bei 1:500 (1 m sind ${mm} mm) ist der Holzapfel eine geschlossene, breite und dichte Krone von 15 mm Breite über einem kurzen geneigten Stamm mit Wurzelanlauf und den sichtbaren Ansätzen der vier Leitäste; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers.` },
      retained: { en: "Retained: stem with root flare, the inner limb parts and the leader below the crown, and the crown outline.", de: "Beibehalten: Stamm mit Wurzelanlauf, die inneren Leitastteile und der Mitteltrieb unter der Krone sowie der Kronenumriss." },
      merged: { en: "Merged into the crown path: every foliage mass, the shade core, the outer limb parts, the spur shoots and the fruit (all regions of the crown are listed on it).", de: "Im Kronenpfad vereint: jede Laubmasse, der Schattenkern, die äußeren Leitastteile, die Kurztriebe und die Früchte (alle Regionen der Krone sind daran genannt)." },
      omitted: { en: "Omitted: every fruit as a mark and every internal edge.", de: "Weggelassen: jede Frucht als Zeichen und jede innere Kante." },
      thinElements: { en: "The inner limb parts are 0.3 to 0.4 mm wide on paper and stay filled; nothing is thickened.", de: "Die inneren Leitastteile sind auf dem Papier 0,3 bis 0,4 mm breit und bleiben gefüllt; nichts wird verdickt." },
      hatching: { en: "No hatching; the pen is 0.15 mm for outlines.", de: "Keine Schraffur; der Stift ist 0,15 mm für Umrisse." },
    },
  };
  const d = drawn[scale]!;
  d.styles = { en: `Light line: outlines of ${pen[1]} mm and the finer pens of this scale. Dark solid: the dense crown reads in a mid tone, the fruit lighter and the wood darker.`, de: `Helle Linie: Umrisse von ${pen[0]} mm und die feineren Stifte dieses Maßstabs. Dunkle Fläche: die dichte Krone liest sich im Mittelton, die Früchte heller und das Holz dunkler.` };
  d.overlays = scale === 500
    ? { en: "Overlays map onto trunk-bark (stem, root flare) and, merged in the crown path, limb-bark, spur-twig, leaf-sun, leaf-shade and fruit; merged shapes show the area-weighted value of their regions.", de: "Überlagerungen bilden sich ab auf trunk-bark (Stamm, Wurzelanlauf) sowie, im Kronenpfad vereint, limb-bark, spur-twig, leaf-sun, leaf-shade und fruit; vereinte Formen zeigen den flächengewichteten Wert ihrer Regionen." }
    : { en: "Every thermal region has its shapes: stem and root flare trunk-bark, limbs and leader limb-bark, spur shoots spur-twig (merged into the sun masses at 1:200), outer masses leaf-sun, the core leaf-shade, the crab apples fruit; marks without a region follow their bone.", de: "Jede thermische Region hat ihre Formen: Stamm und Wurzelanlauf trunk-bark, Leitäste und Mitteltrieb limb-bark, Kurztriebe spur-twig (bei 1:200 in die Sonnenmassen vereint), äußere Massen leaf-sun, der Kern leaf-shade, die Holzäpfel fruit; Zeichen ohne Region folgen ihrem Knochen." };
  const volume: Record<number, Text[]> = {
    50: [
      { en: `Body at 1:50: root flare, the short leaning stem, four limbs in two parts spreading almost horizontally (swept lofts), the central leader, fourteen side branches, four spur shoots with three thorns each, a dense dome of ${c.lobes} leaf lobes (eighteen clumps with their bosses including a drooping skirt, a cap with four bosses, five shade lobes) and ${c.extras} crab apples of 3 cm hanging under the clumps.`, de: `Körper bei 1:50: Wurzelanlauf, der kurze geneigte Stamm, vier Leitäste in zwei Teilen, fast waagerecht ausladend (geführte Lofts), der Mitteltrieb, vierzehn Seitenäste, vier Kurztriebe mit je drei Dornen, eine dichte Kuppel aus ${c.lobes} Laublappen (achtzehn Klumpen mit ihren Ausbuchtungen samt einem hängenden Saum, eine Kappe mit vier Ausbuchtungen, fünf Schattenlappen) und ${c.extras} Holzäpfel von 3 cm unter den Klumpen.` },
      { en: "Thermal at 1:50: trunk bark is the root flare and the stem; limb bark the limbs, leader and side branches; the spur shoots with their thorns are the twig region; outer lobes sun leaves, inner lobes shade leaves, the crab apples the fruit region, all by reference to the body; ten transpiration outlets are lens patches in the free faces of the lobes that carry the emitter anchors.", de: "Thermik bei 1:50: Stammrinde sind Wurzelanlauf und Stamm; Astrinde die Leitäste, der Mitteltrieb und die Seitenäste; die Kurztriebe mit ihren Dornen sind die Zweigregion; äußere Lappen Sonnenblätter, innere Lappen Schattenblätter, die Holzäpfel die Fruchtregion, alle als Verweis auf den Körper; zehn Transpirationsauslässe sind Linsenflächen in den freien Seiten der Lappen, die die Emitteranker tragen." },
      { en: "Structure at 1:50: the wood under the bark of every load-bearing bone along the real limb axes (root flare, stem, eight limb parts, leader, four spurs), joints at the rig joints, the round fixed root plate and the masses of the root plate, the stem and the six foliage masses.", de: "Tragwerk bei 1:50: das Holz unter der Rinde jedes tragenden Knochens entlang der wirklichen Astachsen (Wurzelanlauf, Stamm, acht Leitastteile, Mitteltrieb, vier Kurztriebe), Gelenke an den Rig-Gelenken, der runde eingespannte Wurzelteller und die Massen von Wurzelteller, Stamm und den sechs Laubmassen." },
    ],
    100: [
      { en: `Body at 1:100: a simpler root flare, every limb with five sections, ten inner side branches, the spur shoots without thorns, ${c.lobes} broader lobes and ${c.extras} crab apples.`, de: `Körper bei 1:100: ein einfacherer Wurzelanlauf, jeder Ast mit fünf Schnitten, zehn innere Seitenäste, die Kurztriebe ohne Dornen, ${c.lobes} breitere Lappen und ${c.extras} Holzäpfel.` },
      { en: "Thermal at 1:100: the same six regions by reference to this scale's body; five outlets as lens patches.", de: "Thermik bei 1:100: dieselben sechs Regionen als Verweis auf den Körper dieses Maßstabs; fünf Auslässe als Linsenflächen." },
      { en: "Structure at 1:100: the wood of the load-bearing bones with simplified taper, joints, the root plate and the masses.", de: "Tragwerk bei 1:100: das Holz der tragenden Knochen mit vereinfachter Verjüngung, Gelenke, der Wurzelteller und die Massen." },
    ],
    200: [
      { en: `Body at 1:200: root flare, stem, the eight limb parts and the leader with three sections each, no side branches and no spurs; ${c.lobes} masses and ${c.extras} typified fruit clusters of 9 cm.`, de: `Körper bei 1:200: Wurzelanlauf, Stamm, die acht Leitastteile und der Mitteltrieb mit je drei Schnitten, keine Seitenäste und keine Kurztriebe; ${c.lobes} Massen und ${c.extras} typisierte Fruchtbüschel von 9 cm.` },
      { en: "Thermal at 1:200: bark, leaves and fruit by reference to the body; the spur shoots stay as slender twigs in the thermal list only; three outlets.", de: "Thermik bei 1:200: Rinde, Laub und Früchte als Verweis auf den Körper; die Kurztriebe bleiben als schlanke Zweige nur in der Thermikliste; drei Auslässe." },
      { en: "Structure at 1:200: the wood of the load-bearing bones, joints where they are at least one smallest feature thick, the root plate and the masses.", de: "Tragwerk bei 1:200: das Holz der tragenden Knochen, Gelenke dort, wo sie mindestens ein kleinstes Merkmal dick sind, der Wurzelteller und die Massen." },
    ],
    500: [
      { en: `Body at 1:500: root flare, stem, the inner limb parts and the leader as plain lofts, and the crown as ${c.lobes} widened masses bound to the leader as the drawing binds it.`, de: `Körper bei 1:500: Wurzelanlauf, Stamm, die inneren Leitastteile und der Mitteltrieb als einfache Lofts und die Krone als ${c.lobes} verbreiterte Massen, an den Mitteltrieb gebunden wie in der Zeichnung.` },
      { en: "Thermal at 1:500: bark and leaves by reference; the outer limb parts, the spur shoots and four fruit clusters are thermal solids of their own inside the crown; two outlets.", de: "Thermik bei 1:500: Rinde und Laub als Verweis; die äußeren Leitastteile, die Kurztriebe und vier Fruchtbüschel sind eigene Thermikkörper in der Krone; zwei Auslässe." },
      { en: "Structure at 1:500: the load-bearing bones as members of their declared section, the joint of the stem, the root plate and the masses of root plate and stem; this list is already minimal.", de: "Tragwerk bei 1:500: die tragenden Knochen als Stäbe ihres angegebenen Querschnitts, das Stammgelenk, der Wurzelteller und die Massen von Wurzelteller und Stamm; diese Liste ist bereits minimal." },
    ],
  };
  return { drawn: d, volume: volume[scale]! };
}

export const species: Species = {
  id: "crab-apple",
  trunk: "trunk",
  woodMaterial: "crab-wood",
  twigRegion: "spur-twig",
  extraRegion: "fruit",
  leafGlyph: leaf,
  extraGlyph: "fruit",
  bark,
  model,
  texts,
  areaOverrides: { "trunk-bark": 2.5 },
  scaffolds: ["sc-1a", "sc-2a", "sc-3a", "sc-4a"],
  symbolTexts: {
    plan: {
      title: { en: "Plan symbol with true crown extent", de: "Grundrisssymbol mit wahrer Kronenausdehnung" },
      description: { en: "A conventional sign for site, hedgerow and orchard plans: the true outline of the dense crown in the three views, the stem at its real position and section, and in plan the directions of the four main limbs. It is a separate symbol and is never used silently for a pictorial scale.", de: "Ein konventionelles Zeichen für Lage-, Hecken- und Obstwiesenpläne: der wahre Umriss der dichten Krone in den drei Ansichten, der Stamm an seiner wahren Lage und mit seinem Querschnitt und im Grundriss die Richtungen der vier Leitäste. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet." },
      dimensionalNote: { en: "The outlines are the true extent of the 1:100 body in model metres (about 7.7 m wide, 7.5 m deep and 7.0 m high); the filled shapes are the stem and root flare at their real size; the four strokes only indicate limb directions. Planting distances are not part of the symbol.", de: "Die Umrisse sind die wahre Ausdehnung des 1:100-Körpers in Modellmetern (etwa 7,7 m breit, 7,5 m tief und 7,0 m hoch); die gefüllten Formen sind Stamm und Wurzelanlauf in wahrer Größe; die vier Striche deuten nur die Leitastrichtungen an. Pflanzabstände gehören nicht zum Symbol." },
    },
    sign: {
      title: { en: "Nominal fruit tree sign", de: "Nominelles Obstbaumzeichen" },
      description: { en: "A circle with a centre dot and an inner ring as the conventional sign of a fruit tree in small-scale plans, drawn as plan circle and as side and front lollipop; the size is a fixed paper size.", de: "Ein Kreis mit Mittelpunkt und Innenring als übliches Zeichen eines Obstbaums in kleinmaßstäblichen Plänen, als Grundrisskreis und als seitlicher und vorderer Lolli gezeichnet; die Größe ist eine feste Papiergröße." },
      dimensionalNote: { en: "A fixed 3.2 mm circle with a 1.8 mm inner ring and a 0.7 mm centre dot in paper millimetres; its size carries no dimension. Use the true-extent symbol where the crown width matters.", de: "Ein fester Kreis von 3,2 mm mit 1,8 mm Innenring und 0,7 mm Mittelpunkt in Papiermillimetern; seine Größe trägt kein Maß. Wo die Kronenbreite zählt, ist das Symbol mit wahrer Ausdehnung zu verwenden." },
    },
  },
};
