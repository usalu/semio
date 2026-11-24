/** 🌾️ Gives straight culms their lean as a solid: `bun lean.ts <id> --bones <regex> [--views plan] [--scales 50,100] [--straight 0.004]` writes `set` ops into `ops/<id>.json`. A culm of an ornamental grass stands in the sagittal plane of its bone and leans sideways; the plan (and front) drew it as a flat line or sliver in world coordinates, which is carried at one depth and keeps its direction when the wind swings the bone (cause D). For every flat shape of a matching bone that has a twin of the same id in the side elevation spans at least 0.6 of the bone length and is straight (its points lie within `straight` metres of the line through its two ends, apart from its own width), the shape takes the outline of the elevation twin and becomes an `extruded` solid, `sweep: "w"`, whose lateral span runs linearly from the place of the drawn shape (read from the front of an upright bone, from the plan of a level one) at the bone origin (`z0`, `z1`) to its place at the bone tip (`zTip`), at least 1.5 mm wide: at rest it is the drawn culm, in every pose the bone carries it whole.
 * @see ./apply.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const id = args[0]!;
const asset = readDoc(id).doc.asset;
const bones = new RegExp(flag("bones")!);
const views = (flag("views") ?? "plan").split(",");
const scales = flag("scales")?.split(",");
const STRAIGHT = Number(flag("straight") ?? 0.004);
const rest = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[] } }>;
const lengthOf = (bone: string): number => asset.rig.bones.find((candidate: any) => candidate.id === bone).lengthM;
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
const ops: any[] = [];
let skipped = 0;
for (const scale of Object.keys(asset.representations)) {
  const geometry = asset.representations[scale].geometry;
  if (geometry === undefined || (scales !== undefined && !scales.includes(scale))) continue;
  for (const view of views)
    for (const shape of geometry[view]) {
      if (!bones.test(shape.bone) || shape.solid !== undefined || shape.kind === "member") continue;
      const twin = geometry.elevation.find((candidate: any) => candidate.id === shape.id && candidate.bone === shape.bone && candidate.solid === undefined);
      if (twin === undefined) continue;
      const { o, u } = rest[shape.bone]!.frame;
      const from = Math.abs(u[1]!) >= Math.abs(u[0]!) ? "front" : "plan";
      const drawn = geometry[from].find((candidate: any) => candidate.id === shape.id && candidate.bone === shape.bone && candidate.kind !== "member" && (candidate.solid === undefined || from === view));
      if (drawn === undefined || (from !== view && drawn.solid !== undefined)) { skipped++; continue; }
      const points = flattenGeometry(drawn).rings.flat().map((p: any) => (from === "plan" ? { s: (p.x - o[0]!) / u[0]!, w: p.y } : { s: (p.y - o[1]!) / u[1]!, w: p.x }));
      const [s0, s1] = [Math.min(...points.map((p: any) => p.s)), Math.max(...points.map((p: any) => p.s))];
      if (s1 - s0 < 0.6 * lengthOf(shape.bone)) { skipped++; continue; }
      const near = (s: number): number[] => points.filter((p: any) => Math.abs(p.s - s) <= 0.05 * (s1 - s0)).map((p: any) => p.w);
      const [a, b] = [near(s0), near(s1)];
      const [wa, wb] = [(Math.min(...a) + Math.max(...a)) / 2, (Math.min(...b) + Math.max(...b)) / 2];
      const half = Math.max(0.00075, (Math.max(...a) - Math.min(...a)) / 2, (Math.max(...b) - Math.min(...b)) / 2);
      const slope = (wb - wa) / (s1 - s0);
      const at = (s: number): number => wa + slope * (s - s0);
      if (points.some((p: any) => Math.abs(p.w - at(p.s)) > half + STRAIGHT)) { skipped++; continue; }
      const tip = at(lengthOf(shape.bone));
      const set: any = { kind: twin.kind, solid: { kind: "extruded", sweep: "w", z0: round(at(0) - half), z1: round(at(0) + half), zTip: [round(tip - half), round(tip + half)] } };
      for (const key of ["points", "closed", "d"]) set[key] = twin[key] ?? null;
      ops.push({ op: "set", scale, view, id: shape.id, bone: shape.bone, set });
    }
}
const file = flag("ops") ?? join(import.meta.dir, "ops", `${id}.json`);
const mine = (op: any): string => `${op.op} ${op.scale} ${op.view} ${op.id} ${op.bone}`;
const taken = new Set(ops.map(mine));
const known: any[] = existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : [];
const next = [...known.filter((op) => !taken.has(mine(op))), ...ops];
writeFileSync(file, `[\n${next.map((op) => `  ${JSON.stringify(op)}`).join(",\n")}\n]\n`);
console.log(`${id}: ${ops.length} culm shapes become solids, ${skipped} skipped (not straight or along the view), ${next.length} ops in ${file}`);
