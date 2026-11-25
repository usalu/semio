/** ⚓️ Puts every landmark anchor (contact, attachment, emitter) of an asset on the drawing of its bone, after `carry.ts` in the chain of a plant: `bun anchors.ts <id|file.json> [--ops ops.json] [--save ops.json] [--out file] [--write] [--steps 6] [--keep 0.6] [--undrawn]`.
 *
 * The matrix asks that such an anchor lies on the drawn shapes of its bone or of a descendant bone, within the tolerance, in every view that draws the bone, at every authored scale and in every subject (pose, clip instant, transition). The run measures that for every anchor; an anchor that fails anywhere is searched a new place:
 * - candidates are the points of a grid of half a tolerance, `steps` steps each way round the rest position of the anchor, bound to its own bone or to any other bone that is drawn in the three views and draws the place itself (a bone with child bones is measured against its own shapes only, since the shapes of its descendants would make every place pass) (cause B of `multiview-diagnosis.md`: an outlet bound to a branch but lying in the foliage of another bone);
 * - a candidate holds when its worst distance over every scale, view and subject is at most `keep` × tolerance; of those the one that moves the anchor least is taken, where a change of the bone counts as half a tolerance of movement, so an anchor stays on its bone when its bone draws a place nearby.
 * With `--undrawn` an anchor whose bone is drawn in no view at any scale (the matrix skips it) is bound the same way to the bone that draws its place.
 * The result is a list of ops `{ "op": "anchor", "id", "set": { "bone", "x", "y", "z" } }` (bone-local metres). `--save` writes it, `--ops` applies a saved list instead of searching (the reproducible step of the chain: absolute values, a second run changes nothing).
 * @see ./carry.ts — the step before
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { subtrees, LANDMARK_KINDS } = await import(join(T, "⚓️landmarks.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));

type V3 = [number, number, number];
type Drawn = { rings: { x: number; y: number }[][]; closed: boolean; box: [number, number, number, number] };
type Candidate = { anchor: string; bone: string; local: V3; cost: number; moved: number; worst: number; where: string };
const VIEWS = ["elevation", "front", "plan"] as const;
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const tolerance: number = engine.toleranceM(asset);
const STEPS = Number(flag("steps") ?? 6);
const KEEP = Number(flag("keep") ?? 0.6);
const STEP = Math.max(0.02, tolerance / 2);
const round = (n: number): number => Math.round(n * 1e4) / 1e4;
const subtreeOf = subtrees(asset.rig.bones);
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);

/** 🖼️ One evaluated subject reduced to what an anchor is measured against: the frames of the bones and, per view and bone, the drawn shapes with their boxes. */
type Scene = { id: string; pose: boolean; frames: Record<string, { o: V3; u: V3; v: V3; w: V3 }>; drawn: Record<string, Map<string, Drawn[]>>; under: Record<string, Map<string, Drawn[]>> };

function sceneOf(key: string, subject: any): Scene {
  const state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
  const vec = (p: any): V3 => [p.x, p.y, p.z];
  const frames = Object.fromEntries(Object.entries(state.bones).map(([id, bone]: [string, any]) => [id, { o: vec(bone.frame.o), u: vec(bone.frame.u), v: vec(bone.frame.v), w: vec(bone.frame.w) }]));
  const drawn: Scene["drawn"] = {};
  for (const view of VIEWS) {
    const map = (drawn[view] = new Map<string, Drawn[]>());
    for (const shape of state.shapes[view]) {
      if (shape.shape.role === "ground") continue;
      const points = shape.rings.flat();
      if (points.length === 0) continue;
      const xs = points.map((p: any) => p.x), ys = points.map((p: any) => p.y);
      (map.get(shape.shape.bone) ?? map.set(shape.shape.bone, []).get(shape.shape.bone)!).push({ rings: shape.rings, closed: shape.closed, box: [Math.min(...xs), Math.min(...ys), Math.max(...xs), Math.max(...ys)] });
    }
  }
  return { id: `1:${key} ${subject.id}`, pose: subject.query.kind === "pose", frames, drawn, under: { elevation: new Map(), front: new Map(), plan: new Map() } };
}

const segment = (px: number, py: number, a: { x: number; y: number }, b: { x: number; y: number }): number => {
  const [dx, dy] = [b.x - a.x, b.y - a.y];
  const l2 = dx * dx + dy * dy;
  const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((px - a.x) * dx + (py - a.y) * dy) / l2));
  return Math.hypot(px - (a.x + t * dx), py - (a.y + t * dy));
};

/** 📏️ The distance of a point to drawn shapes as the matrix measures it (0 inside a closed ring), or infinity when it is beyond `limit` for certain. */
function distance(px: number, py: number, shapes: readonly Drawn[], limit: number): number {
  let best = Infinity;
  for (const shape of shapes) {
    if (px < shape.box[0] - limit || px > shape.box[2] + limit || py < shape.box[1] - limit || py > shape.box[3] + limit) continue;
    for (const ring of shape.rings) {
      if (shape.closed && ring.length > 2) {
        let inside = false;
        for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
          const [a, b] = [ring[i]!, ring[j]!];
          if (a.y > py !== b.y > py && px < ((b.x - a.x) * (py - a.y)) / (b.y - a.y) + a.x) inside = !inside;
        }
        if (inside) return 0;
      }
      for (let i = 0; i + 1 < ring.length; i++) best = Math.min(best, segment(px, py, ring[i]!, ring[i + 1]!));
      if (shape.closed && ring.length > 2) best = Math.min(best, segment(px, py, ring[ring.length - 1]!, ring[0]!));
      if (ring.length === 1) best = Math.min(best, Math.hypot(px - ring[0]!.x, py - ring[0]!.y));
    }
  }
  return best;
}

/** 🎯️ The worst distance of a bone-local point in one scene over the views that draw the bone; stops at the first view beyond `limit`. */
function inScene(scene: Scene, bone: string, local: V3, limit: number, strict: boolean): number {
  const f = scene.frames[bone]!;
  const p = [0, 1, 2].map((i) => f.o[i]! + local[0] * f.u[i]! + local[1] * f.v[i]! + local[2] * f.w[i]!);
  let worst = 0;
  for (const view of VIEWS) {
    if (!scene.drawn[view]!.has(bone)) continue;
    let shapes = strict ? scene.drawn[view]!.get(bone)! : scene.under[view]!.get(bone);
    if (shapes === undefined) scene.under[view]!.set(bone, (shapes = [...subtreeOf(bone)].flatMap((id: string) => scene.drawn[view]!.get(id) ?? [])));
    const d = view === "elevation" ? distance(p[0]!, p[1]!, shapes, limit) : view === "front" ? distance(p[2]!, p[1]!, shapes, limit) : distance(p[0]!, p[2]!, shapes, limit);
    worst = Math.max(worst, d);
    if (worst > limit) return worst;
  }
  return worst;
}

const scenes: Scene[] = authored.flatMap((key) => engine.subjects(asset, Number(key)).map((subject: any) => sceneOf(key, subject)));
const ordered = [...scenes.filter((scene) => scene.pose), ...scenes.filter((scene) => !scene.pose)];
const worstOf = (bone: string, local: V3, limit: number, strict = false): { worst: number; where: string } => {
  let worst = 0, where = "";
  for (const scene of ordered) {
    const d = inScene(scene, bone, local, limit, strict);
    if (d > worst) { worst = d; where = scene.id; }
    if (worst > limit) break;
  }
  return { worst, where };
};

const landmarks: any[] = asset.anchors.filter((anchor: any) => LANDMARK_KINDS.includes(anchor.kind));
const localOf = (anchor: any): V3 => [anchor.x, anchor.y, anchor.z ?? 0];
let ops: any[];
if (flag("ops") !== undefined) ops = JSON.parse(readFileSync(flag("ops")!, "utf8"));
else {
  ops = [];
  const rest = restFrames(asset.rig) as Record<string, { frame: { o: V3; u: V3; v: V3; w: V3 } }>;
  const everywhere = asset.rig.bones.map((bone: any) => bone.id).filter((bone: string) => scenes.some((scene) => VIEWS.every((view) => scene.drawn[view]!.has(bone))));
  const parents = new Set<string>(asset.rig.bones.map((bone: any) => bone.parent).filter((bone: any) => typeof bone === "string"));
  const drawnSomewhere = (bone: string): boolean => scenes.some((scene) => VIEWS.some((view) => scene.drawn[view]!.has(bone)));
  for (const anchor of landmarks) {
    const now = worstOf(anchor.bone, localOf(anchor), Infinity);
    const hidden = args.includes("--undrawn") && !drawnSomewhere(anchor.bone);
    if (now.worst <= tolerance && !hidden) continue;
    const own = rest[anchor.bone]!.frame;
    const at = [0, 1, 2].map((i) => own.o[i]! + anchor.x * own.u[i]! + anchor.y * own.v[i]! + (anchor.z ?? 0) * own.w[i]!);
    const found: Candidate[] = [];
    for (const bone of [...(hidden ? [] : [anchor.bone]), ...everywhere.filter((id: string) => id !== anchor.bone)]) {
      const f = rest[bone]!.frame;
      for (let i = -STEPS; i <= STEPS; i++)
        for (let j = -STEPS; j <= STEPS; j++)
          for (let k = -STEPS; k <= STEPS; k++) {
            const d = [at[0]! + i * STEP - f.o[0], at[1]! + j * STEP - f.o[1], at[2]! + k * STEP - f.o[2]];
            const local: V3 = [round(d[0]! * f.u[0] + d[1]! * f.u[1] + d[2]! * f.u[2]), round(d[0]! * f.v[0] + d[1]! * f.v[1] + d[2]! * f.v[2]), round(d[0]! * f.w[0] + d[1]! * f.w[1] + d[2]! * f.w[2])];
            const moved = Math.hypot(i, j, k) * STEP;
            found.push({ anchor: anchor.id, bone, local, moved, cost: moved + (bone === anchor.bone ? 0 : tolerance / 2), worst: 0, where: "" });
          }
    }
    found.sort((p, q) => p.cost - q.cost);
    let pick: Candidate | undefined;
    for (const candidate of found) {
      Object.assign(candidate, worstOf(candidate.bone, candidate.local, KEEP * tolerance, candidate.bone !== anchor.bone && parents.has(candidate.bone)));
      if (candidate.worst <= KEEP * tolerance) { pick = candidate; break; }
    }
    if (pick === undefined) { console.log(`${asset.id} ${anchor.id} on ${anchor.bone}: ${(now.worst * 1000).toFixed(0)} mm @ ${now.where}; NO PLACE within ${(STEPS * STEP).toFixed(2)} m holds`); continue; }
    console.log(`${asset.id} ${anchor.id} on ${anchor.bone}: ${hidden ? "its bone is drawn in no view" : `${(now.worst * 1000).toFixed(0)} mm @ ${now.where}`} -> ${pick.bone === anchor.bone ? "stays on its bone" : `bound to ${pick.bone}`}, moved ${(pick.moved * 1000).toFixed(0)} mm, worst ${(pick.worst * 1000).toFixed(0)} mm${pick.where ? ` @ ${pick.where}` : ""}`);
    ops.push({ op: "anchor", id: anchor.id, set: { bone: pick.bone, x: pick.local[0], y: pick.local[1], z: pick.local[2] } });
  }
}
for (const op of ops) {
  const anchor = asset.anchors.find((candidate: any) => candidate.id === op.id);
  if (anchor === undefined) throw new Error(`${asset.id}: no anchor "${op.id}"`);
  Object.assign(anchor, op.set);
}
let beyond = 0;
for (const anchor of landmarks) {
  const { worst, where } = worstOf(anchor.bone, localOf(anchor), Infinity);
  if (worst > tolerance) { beyond++; console.log(`${asset.id} ${anchor.id} on ${anchor.bone}: STILL ${(worst * 1000).toFixed(0)} mm @ ${where}`); }
}
console.log(`${asset.id}: tolerance ${(tolerance * 1000).toFixed(0)} mm, ${landmarks.length} landmark anchors, ${ops.length} ops, ${beyond} beyond the tolerance, ${scenes.length} subjects`);
if (flag("save") !== undefined) writeFileSync(flag("save")!, `[\n${ops.map((op) => `  ${JSON.stringify(op)}`).join(",\n")}\n]\n`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  const written = rewrite(text, previous, next);
  if (written === text && out === undefined) console.log(`${asset.id}: unchanged`);
  else { writeFileSync(out ?? file, written); console.log(`${asset.id}: written to ${out ?? file}`); }
}
