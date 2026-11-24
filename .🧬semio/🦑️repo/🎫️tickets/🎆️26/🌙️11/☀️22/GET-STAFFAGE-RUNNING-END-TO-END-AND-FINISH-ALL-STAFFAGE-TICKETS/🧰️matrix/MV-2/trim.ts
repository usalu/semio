/** ✂️ Cuts hidden length off flat front shapes: `bun trim.ts <id> [--scales 200] [--limit 0.5]` writes `set` ops into `ops/<id>.json`. Where a bone draws one flat filled polygon in the front and its twin of the same id in the side elevation, and the front polygon is taller at rest than the twin by more than `limit` × tolerance (cause G: the front part runs on behind its neighbour, the elevation part ends where the neighbour begins), the front polygon is cut at the lowest and highest height of the elevation twin. Only hidden length goes; the rest of the outline stays where it was.
 * @see ./apply.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const id = args[0]!;
const asset = readDoc(id).doc.asset;
const scales = flag("scales")?.split(",");
const LIMIT = Number(flag("limit") ?? 0.5);
const tolerance: number = engine.toleranceM(asset);
const rest = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[] } }>;
const round = (n: number): number => Math.round(n * 1e4) / 1e4;
type P = [number, number];
const cut = (ring: P[], y: number, keepBelow: boolean): P[] => {
  const out: P[] = [];
  const inside = (p: P): boolean => (keepBelow ? p[1] <= y : p[1] >= y);
  ring.forEach((p, i) => {
    const q = ring[(i + 1) % ring.length]!;
    if (inside(p)) out.push(p);
    if (inside(p) !== inside(q)) out.push([round(p[0] + ((q[0] - p[0]) * (y - p[1])) / (q[1] - p[1])), round(y)]);
  });
  return out;
};
const ops: any[] = [];
for (const scale of Object.keys(asset.representations)) {
  const geometry = asset.representations[scale].geometry;
  if (geometry === undefined || (scales !== undefined && !scales.includes(scale))) continue;
  const count = (view: string, bone: string): number => geometry[view].filter((shape: any) => shape.bone === bone && shape.role !== "ground").length;
  for (const shape of geometry.front) {
    if (shape.kind !== "poly" || shape.filled !== true || shape.solid !== undefined || count("front", shape.bone) !== 1 || count("elevation", shape.bone) !== 1) continue;
    const twin = geometry.elevation.find((candidate: any) => candidate.id === shape.id && candidate.bone === shape.bone && candidate.kind === "poly");
    if (twin === undefined) continue;
    const { o, u, v } = rest[shape.bone]!.frame;
    const heights = twin.points.map(([x, y]: P) => o[1]! + x * u[1]! + y * v[1]!);
    const [lo, hi] = [Math.min(...heights), Math.max(...heights)];
    const ys = shape.points.map((p: P) => p[1]);
    const over = Math.max(lo - Math.min(...ys), Math.max(...ys) - hi);
    if (over <= LIMIT * tolerance) continue;
    const points = cut(cut(shape.points, hi, true), lo, false);
    console.log(`1:${scale} front ${shape.id} on ${shape.bone}: [${Math.min(...ys).toFixed(3)}, ${Math.max(...ys).toFixed(3)}] cut to the elevation twin [${lo.toFixed(3)}, ${hi.toFixed(3)}], ${shape.points.length} -> ${points.length} points`);
    ops.push({ op: "set", scale, view: "front", id: shape.id, bone: shape.bone, set: { points } });
  }
}
const file = flag("ops") ?? join(import.meta.dir, "ops", `${id}.json`);
const mine = (op: any): string => `${op.op} ${op.scale} ${op.view} ${op.id} ${op.bone}`;
const taken = new Set(ops.map(mine));
const known: any[] = existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : [];
const next = [...known.filter((op) => !taken.has(mine(op))), ...ops];
writeFileSync(file, `[\n${next.map((op) => `  ${JSON.stringify(op)}`).join(",\n")}\n]\n`);
console.log(`${id}: ${ops.length} front shapes cut, ${next.length} ops in ${file}`);
