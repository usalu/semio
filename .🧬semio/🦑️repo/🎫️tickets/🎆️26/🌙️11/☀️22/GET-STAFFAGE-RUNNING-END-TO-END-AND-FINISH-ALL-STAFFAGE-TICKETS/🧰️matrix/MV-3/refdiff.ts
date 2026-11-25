/** 🪞️ Where the silhouette comparison of a refitted file differs from the one of the file before, in the default pose: `bun refdiff.ts <before.json> <after.json> <view> <scale>` prints (1) the groups of the reference of the body (`referenceOf` with the drawn shapes, as `🧊️catalog-silhouette` builds it) that only one of the two files has, with their boxes, and (2) the raster cells (20 mm) the filled shapes of only one of the two drawings cover, with their box. A repair that must not change the rest drawing shows nothing in (2); a reference that moves in (1) names the body solid whose drawing convention reacts to the new shapes (a slender solid follows the 2D shape on its bone).
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { referenceOf, drawnFor, worldSolids, Mask, strokesOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [beforeArg, afterArg, view, key] = process.argv.slice(2);
const scale = Number(key);
type Q = { x: number; y: number };
const build = (arg: string) => {
  const asset = readDoc(arg).doc.asset;
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const solids = (worldSolids(asset, state.bones, scale) as any[]).filter((w) => w.texture !== true);
  const drawn = drawnFor(asset, state, view, scale);
  const reference = referenceOf(view, solids, undefined, drawn).cut as Q[][][];
  const strokes = new Map<string, string>(solids.filter((w) => w.slender === true).map((w) => [w.id as string, boxOf((strokesOf(view, w, drawn) as Q[][]).flat())]));
  return { asset, state, reference, strokes };
};
const boxOf = (points: Q[]): string => (points.length === 0 ? "nothing" : `x [${Math.min(...points.map((p) => p.x)).toFixed(3)}, ${Math.max(...points.map((p) => p.x)).toFixed(3)}] y [${Math.min(...points.map((p) => p.y)).toFixed(3)}, ${Math.max(...points.map((p) => p.y)).toFixed(3)}]`);
const [a, b] = [build(beforeArg!), build(afterArg!)];
const keyOf = (group: Q[][]): string => group.map((ring) => ring.map((p) => `${p.x.toFixed(4)},${p.y.toFixed(4)}`).join(" ")).join("|");
const [ka, kb] = [new Set(a.reference.map(keyOf)), new Set(b.reference.map(keyOf))];
const onlyA = a.reference.filter((g) => !kb.has(keyOf(g)));
const onlyB = b.reference.filter((g) => !ka.has(keyOf(g)));
console.log(`reference 1:${key} ${view}: ${a.reference.length} groups before, ${b.reference.length} after; only before ${onlyA.length} (${boxOf(onlyA.flat(2))}), only after ${onlyB.length} (${boxOf(onlyB.flat(2))})`);
for (const [id, box] of b.strokes) if (a.strokes.get(id) !== box) console.log(`  slender solid "${id}": drawn as ${a.strokes.get(id)} before, ${box} after`);
const all = [...a.state.shapes[view], ...b.state.shapes[view]].flatMap((s: any) => s.rings.flat()) as Q[];
const box = { minX: Math.min(...all.map((p) => p.x)) - 0.1, minY: Math.min(...all.map((p) => p.y)) - 0.1, maxX: Math.max(...all.map((p) => p.x)) + 0.1, maxY: Math.max(...all.map((p) => p.y)) + 0.1 };
const maskOf = (state: any) => { const mask = new Mask(box, 0.02); for (const s of state.shapes[view]) if (s.closed && s.shape.filled !== false && s.shape.role !== "ground") mask.fill(s.rings); return mask; };
const [ma, mb] = [maskOf(a.state), maskOf(b.state)];
for (const [name, from, to] of [["only before", ma, mb], ["only after", mb, ma]] as const) {
  const cells: Q[] = [];
  for (let k = 0; k < from.cells.length; k++) if (from.cells[k] && !to.cells[k]) cells.push({ x: box.minX + ((k % from.width) + 0.5) * 0.02, y: box.minY + (Math.floor(k / from.width) + 0.5) * 0.02 });
  console.log(`drawn 1:${key} ${view} ${name}: ${cells.length} cells of 20 mm (${boxOf(cells)})`);
}
