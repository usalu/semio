/** 🪢️ Binds the side-elevation shape of a part to the bone that carries the part in the front and in the plan, a step of the chain of a plant before `carry.ts`: `bun rebind.ts <id|file.json> [--out file] [--write] [--scales 50,100] [--skip <regex of shape ids>] [--stem]`.
 *
 * Where the elevation draws a limb or a foliage mass on a stem bone while front and plan draw the shape of the same id on the bone of the limb itself, the wind poses move the three views of one part differently (the stem leans 1 degree, the limb 5 to 9): the whole-figure extents of elevation and plan part (cause B of `multiview-diagnosis.md` for a shape instead of an anchor). Every flat elevation shape whose front twin (else plan twin) of the same id sits on another bone is bound to that bone, drawn at the same place in the rest pose:
 * - on a bone that lies in the drawing plane the outline is re-expressed in the frame of the new bone and stays flat;
 * - on a bone that stands askew to the paper (spread or twisted at rest) a flat shape would be drawn wrong when the bone turns (`lateral-ambiguous` of the validator), so the outline becomes a plate: an `extruded` solid 0.004 mm thick, swept along the lateral axis of the bone, at the lateral place the front twin (else the plan twin) draws the part, whose profile is solved so that the plate is seen exactly as the outline in the rest pose.
 * With `--stem` (a plant that draws its crown on the stem bones and has only single patches on askew limb bones) every flat shape left on an askew bone, in any view, is bound to the nearest ancestor bone that lies in the drawing plane instead, at the same place in the rest pose: the patch then moves with the crown it belongs to in all three views.
 * A shape of several rings (holes) on an askew bone, and a bone whose plane stands nearly edge-on to the paper, are left as they are and named. The run is idempotent: a bound shape has no twin on another bone.
 * @see ./carry.ts — the step after
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const scales = flag("scales")?.split(",");
const skip = flag("skip") === undefined ? undefined : new RegExp(flag("skip")!, "u");
const frames = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
const THICK = 2e-6;
const GEOMETRY_KEYS = ["points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d", "followTip"];
const round = (n: number, digits = 5): number => Number(n.toFixed(digits));
let flat = 0, plates = 0;
const left: string[] = [];
for (const [key, representation] of Object.entries(asset.representations) as [string, any][]) {
  const geometry = representation.geometry;
  if (geometry === undefined || (scales !== undefined && !scales.includes(key))) continue;
  for (const shape of geometry.elevation as any[]) {
    if (shape.solid !== undefined || shape.kind === "member" || skip?.test(shape.id) === true) continue;
    const front = geometry.front.find((twin: any) => twin.id === shape.id);
    const plan = geometry.plan.find((twin: any) => twin.id === shape.id);
    const twin = front ?? plan;
    if (twin === undefined || twin.bone === shape.bone) continue;
    const from = frames[shape.bone]!.frame;
    const to = frames[twin.bone]!.frame;
    const { rings, closed } = flattenGeometry(shape) as { rings: { x: number; y: number }[][]; closed: boolean };
    const world = rings.map((ring) => ring.map((p) => [from.o[0]! + p.x * from.u[0]! + p.y * from.v[0]!, from.o[1]! + p.x * from.u[1]! + p.y * from.v[1]!] as const));
    const det = to.u[0]! * to.v[1]! - to.u[1]! * to.v[0]!;
    const askew = Math.abs(to.w[2]!) < 0.999;
    if (Math.abs(det) < 0.25) { left.push(`1:${key} ${shape.id} stays on ${shape.bone}: the plane of ${twin.bone} stands edge-on to the paper (${det.toFixed(2)})`); continue; }
    if (askew && (rings.length !== 1 || !closed)) { left.push(`1:${key} ${shape.id} stays on ${shape.bone}: ${closed ? `${rings.length} rings` : "an open stroke"} cannot be a plate on the askew bone ${twin.bone}`); continue; }
    const lateral = front !== undefined ? flattenGeometry(front).rings.flat().map((p: any) => p.x) : flattenGeometry(plan).rings.flat().map((p: any) => p.y);
    const depth = askew ? (Math.min(...lateral) + Math.max(...lateral)) / 2 - to.o[2]! : 0;
    const local = world.map((ring) => ring.map(([x, y]) => {
      const [dx, dy] = [x - to.o[0]! - depth * to.w[0]!, y - to.o[1]! - depth * to.w[1]!];
      return [round((dx * to.v[1]! - dy * to.v[0]!) / det), round((dy * to.u[0]! - dx * to.u[1]!) / det)] as const;
    }));
    for (const name of GEOMETRY_KEYS) delete shape[name];
    shape.bone = twin.bone;
    if (shape.kind === "path" || local.length > 1) Object.assign(shape, { kind: "path", d: local.map((ring) => `M${ring.map((p) => `${p[0]} ${p[1]}`).join("L")}${closed ? "Z" : ""}`).join("") });
    else Object.assign(shape, { kind: "poly", points: local[0], closed });
    if (askew) { shape.solid = { kind: "extruded", sweep: "w", z0: round(depth + to.o[2]! - THICK, 6), z1: round(depth + to.o[2]! + THICK, 6) }; plates++; } else flat++;
  }
}
let stemmed = 0;
if (args.includes("--stem")) {
  const parent = new Map<string, string | null>(asset.rig.bones.map((bone: any) => [bone.id, bone.parent ?? null]));
  const level = (bone: string): boolean => Math.abs(frames[bone]!.frame.w[2]!) >= 0.999;
  for (const [key, representation] of Object.entries(asset.representations) as [string, any][]) {
    const geometry = representation.geometry;
    if (geometry === undefined || (scales !== undefined && !scales.includes(key))) continue;
    for (const view of ["elevation", "front", "plan"] as const)
      for (const shape of geometry[view] as any[]) {
        if (shape.solid !== undefined || shape.kind === "member" || level(shape.bone) || skip?.test(shape.id) === true) continue;
        let stem: string | null = parent.get(shape.bone) ?? null;
        while (stem !== null && !level(stem)) stem = parent.get(stem) ?? null;
        if (stem === null) continue;
        if (view === "elevation") {
          const [from, to] = [frames[shape.bone]!.frame, frames[stem]!.frame];
          const { rings, closed } = flattenGeometry(shape) as { rings: { x: number; y: number }[][]; closed: boolean };
          const det = to.u[0]! * to.v[1]! - to.u[1]! * to.v[0]!;
          const local = rings.map((ring) => ring.map((p) => {
            const [dx, dy] = [from.o[0]! + p.x * from.u[0]! + p.y * from.v[0]! - to.o[0]!, from.o[1]! + p.x * from.u[1]! + p.y * from.v[1]! - to.o[1]!];
            return [round((dx * to.v[1]! - dy * to.v[0]!) / det), round((dy * to.u[0]! - dx * to.u[1]!) / det)] as const;
          }));
          for (const name of GEOMETRY_KEYS) delete shape[name];
          if (shape.kind === "path" || local.length > 1) Object.assign(shape, { kind: "path", d: local.map((ring) => `M${ring.map((p) => `${p[0]} ${p[1]}`).join("L")}${closed ? "Z" : ""}`).join("") });
          else Object.assign(shape, { kind: "poly", points: local[0], closed });
        }
        shape.bone = stem;
        stemmed++;
      }
  }
}
for (const line of left) console.log(line);
const written = rewrite(text, previous, next);
console.log(`${asset.id}: ${flat} elevation shapes bound flat, ${plates} as plates, ${left.length} left${args.includes("--stem") ? `, ${stemmed} flat shapes on askew bones bound to their stem` : ""}; ${written === text ? "unchanged" : "changed"}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || out !== undefined) writeFileSync(out ?? file, written);
}
