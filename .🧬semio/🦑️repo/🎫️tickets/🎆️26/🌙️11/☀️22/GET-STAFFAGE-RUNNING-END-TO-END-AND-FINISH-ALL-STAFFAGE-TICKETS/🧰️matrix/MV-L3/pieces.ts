/** 🧩️ Which body shows where beside the drawn outlines in one sheet: `bun pieces.ts <id|file.json> <scale> <view> <pose> [--prefix mvb-] [--above]` evaluates the pose and prints, per body, every piece of it outside the union of all other filled shapes (with `--above`: of the filled shapes painted after it only, so a body that covers a hatched see-through of another part is found too) of at least (0.3 mm on paper) squared: its box in metres of the view, its area and mean thickness on paper and its longest straight edge that does not lie on the sheet, in millimetres on paper. The measure of `../MV-3/visible.ts`, kept apart per body so that a cut can be traced to the part that draws it.
 * @see ../MV-3/visible.ts — the totals per sheet
 * @see ../MV-3/body.ts — the bodies */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const clipping: any = (await import(Bun.resolveSync("polygon-clipping", `${join(S, "🧪️tests", "🧊️catalog-silhouette", "🔍️oracles")}/`))).default;

type P = [number, number];
const args = process.argv.slice(2);
const at = args.indexOf("--prefix");
const PREFIX = at < 0 ? "mvb-" : args[at + 1]!;
const [file, key, view, pose] = args as [string, string, string, string];
const paper = Number(key) / 1000;
const areaOf = (ring: readonly P[]): number => Math.abs(ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0)) / 2;
const union = (list: P[][][][]): P[][][] => {
  let level = list;
  while (level.length > 1) level = Array.from({ length: Math.ceil(level.length / 2) }, (_, i) => (level[2 * i + 1] === undefined ? level[2 * i]! : clipping.union(level[2 * i], level[2 * i + 1])));
  return level[0] ?? [];
};
const asset = readDoc(file).doc.asset;
const state: any = evaluate(asset, { kind: "pose", pose }, engine.scaleSelector(Number(key)));
const polygons = (s: any): P[][][][] => s.rings.map((ring: any[]) => ring.map((p) => [Math.round(p.x * 1e5) / 1e5, Math.round(p.y * 1e5) / 1e5] as P)).filter((ring: P[]) => ring.length > 2 && areaOf(ring) > 1e-9).map((ring: P[]) => [[ring]]);
const filled = state.shapes[view].filter((s: any) => s.shape.filled === true && s.shape.role !== "ground" && s.closed === true);
const sheet = union(filled.filter((s: any) => !String(s.shape.id).startsWith(PREFIX)).flatMap(polygons));
const borders = sheet.flat().flatMap((ring) => ring.map((p, i) => [p, ring[(i + 1) % ring.length]!] as [P, P])).filter(([p, q]) => Math.hypot(p[0] - q[0], p[1] - q[1]) > 1e-9);
const onSheet = (m: P): boolean => borders.some(([p, q]) => {
  const l = (q[0] - p[0]) ** 2 + (q[1] - p[1]) ** 2;
  const t = Math.min(1, Math.max(0, ((m[0] - p[0]) * (q[0] - p[0]) + (m[1] - p[1]) * (q[1] - p[1])) / l));
  return Math.hypot(m[0] - p[0] - t * (q[0] - p[0]), m[1] - p[1] - t * (q[1] - p[1])) < 0.02 * paper;
});
const above = args.includes("--above");
for (const body of filled.filter((s: any) => String(s.shape.id).startsWith(PREFIX))) {
  const own = union(polygons(body));
  if (own.length === 0) continue;
  const cover = above ? union(filled.slice(filled.indexOf(body) + 1).filter((s: any) => !String(s.shape.id).startsWith(PREFIX)).flatMap(polygons)) : sheet;
  for (const polygon of clipping.difference(own, cover) as P[][][]) {
    const ring = polygon[0]!;
    const a = areaOf(ring);
    if (a < (0.3 * paper) ** 2) continue;
    const around = ring.reduce((sum, p, i) => sum + Math.hypot(ring[(i + 1) % ring.length]![0] - p[0], ring[(i + 1) % ring.length]![1] - p[1]), 0);
    let edge = 0, where = "";
    for (let i = 0; i < ring.length; i++) {
      const [p, q] = [ring[i]!, ring[(i + 1) % ring.length]!];
      const length = Math.hypot(q[0] - p[0], q[1] - p[1]) / paper;
      if (length > edge && !onSheet([(p[0] + q[0]) / 2, (p[1] + q[1]) / 2])) { edge = length; where = `(${p[0].toFixed(3)}, ${p[1].toFixed(3)}) to (${q[0].toFixed(3)}, ${q[1].toFixed(3)})`; }
    }
    const xs = ring.map((p) => p[0]), ys = ring.map((p) => p[1]);
    console.log(`${body.shape.id}: x ${Math.min(...xs).toFixed(2)} … ${Math.max(...xs).toFixed(2)}, y ${Math.min(...ys).toFixed(2)} … ${Math.max(...ys).toFixed(2)} m; ${(a / paper ** 2).toFixed(1)} mm², thickness ${((2 * a) / around / paper).toFixed(2)} mm, longest free straight edge ${edge.toFixed(1)} mm ${where}`);
  }
}
