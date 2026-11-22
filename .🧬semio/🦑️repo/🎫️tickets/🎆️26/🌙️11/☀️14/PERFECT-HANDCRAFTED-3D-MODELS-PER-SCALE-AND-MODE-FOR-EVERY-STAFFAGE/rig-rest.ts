/** 🦴️ The rest frames of a rig and the rest placement that gives a bone a wanted world frame (see `rig-lateral-rest.md`).
 *
 * Usage (from the ticket folder):
 *   `bun rig-rest.ts <category>/<slug> | --file <asset document json> [--bone ID]...`
 *       one line per bone: origin, tip and the axes u (along the bone), v (normal), w (joint axis), each `[ahead, up, left]` in metres with the root at its authored rest placement.
 *   `... --solve BONE [--parent ID] [--at "x,y,z"] [--u "x,y,z"] [--w "x,y,z"]`
 *       the `parent`, `x`, `y`, `z`, `angleDeg`, `abductionDeg`, `twistDeg` that place BONE under `--parent` (default: its parent) with its origin at the world point `--at` (default: where it is), its axis along `--u` and its joint axis along `--w`
 *       (defaults: as they are; `--w` is made perpendicular to `--u`). With only `--parent` this re-parents a bone without moving it; every child of BONE keeps its own fields.
 *       BONE need not exist yet: for a new bone give `--parent`, `--at` and `--u` (`--w` defaults to the joint axis of the parent, made perpendicular to `--u`); the printed fields are those of the new bone.
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts — `restFrames`
 */
import { readFileSync } from "node:fs";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { load } from "./r3-lib.ts";

type V = readonly [number, number, number];
const args = process.argv.slice(2);
const valued = ["--file", "--bone", "--solve", "--parent", "--at", "--u", "--w"];
const flag = (name: string): string | undefined => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
const all = (name: string): string[] => args.flatMap((a, i) => (a === name && args[i + 1] !== undefined ? [args[i + 1]!] : []));
const spec = args.find((a, i) => !a.startsWith("--") && !valued.includes(args[i - 1] ?? ""));
const usage = 'rig-rest.ts <category>/<slug> | --file <asset document json> [--bone ID]... [--solve BONE [--parent ID] [--at "x,y,z"] [--u "x,y,z"] [--w "x,y,z"]]';
const file = flag("--file");
const asset = file !== undefined ? ((document) => document.asset ?? document)(JSON.parse(readFileSync(file, "utf8"))) : load(spec, usage).asset;
const rest = restFrames(asset.rig);
const bones = new Map<string, { id: string; parent: string | null; lengthM: number }>(asset.rig.bones.map((bone: { id: string }) => [bone.id, bone]));
const dot = (a: V, b: V): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const add = (a: V, s: number, b: V, t: number): V => [a[0] * s + b[0] * t, a[1] * s + b[1] * t, a[2] * s + b[2] * t];
const unit = (a: V): V => ((l) => [a[0] / l, a[1] / l, a[2] / l] as const)(Math.hypot(...a));
const cross = (a: V, b: V): V => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const vector = (text: string): V => text.split(",").map(Number) as unknown as V;
const show = (v: readonly number[], digits = 4): string => `[${v.map((x) => (Math.abs(x) < 5e-7 ? 0 : x).toFixed(digits)).join(", ")}]`;
const round = (x: number, digits: number): number => Number((Math.abs(x) < 0.5 * 10 ** -digits ? 0 : x).toFixed(digits));

const solve = flag("--solve");
if (solve === undefined) {
  const only = all("--bone");
  for (const [id, solved] of Object.entries(rest)) {
    if (only.length > 0 && !only.includes(id)) continue;
    const f = solved.frame;
    const length = bones.get(id)!.lengthM;
    process.stdout.write(`${id} (parent ${bones.get(id)!.parent ?? "-"}, ${length} m): o ${show(f.o)} tip ${show(add(f.o, 1, f.u, length))} u ${show(f.u)} v ${show(f.v)} w ${show(f.w)}\n`);
  }
} else {
  const bone = bones.get(solve);
  const parentId = flag("--parent") ?? bone?.parent ?? null;
  if (parentId === null || !bones.has(parentId)) throw new Error(bone === undefined ? `"${solve}" is not a bone of the rig yet: name its parent with --parent ID (and give --at and --u)` : `bone "${solve}" needs a parent that is a bone`);
  const parent = rest[parentId]!.frame;
  const own = bone === undefined ? undefined : rest[solve]!.frame;
  if (own === undefined && (flag("--at") === undefined || flag("--u") === undefined)) throw new Error(`"${solve}" is not a bone of the rig yet: give its joint with --at "x,y,z" and its direction with --u "x,y,z" (--w defaults to the joint axis of the parent)`);
  const at = flag("--at") === undefined ? own!.o : vector(flag("--at")!);
  const u = unit(flag("--u") === undefined ? own!.u : vector(flag("--u")!));
  const wanted = flag("--w") === undefined ? (own?.w ?? parent.w) : vector(flag("--w")!);
  if (Math.hypot(...add(wanted, 1, u, -dot(wanted, u))) < 1e-9) throw new Error(`the joint axis ${show(wanted)} lies along the bone: give another with --w "x,y,z"`);
  const w = unit(add(wanted, 1, u, -dot(wanted, u)));
  const v = cross(u, w).map((x) => -x) as unknown as V;
  const d: V = [at[0] - parent.o[0], at[1] - parent.o[1], at[2] - parent.o[2]];
  const deg = 180 / Math.PI;
  const lateral = Math.min(1, Math.max(-1, dot(u, parent.w)));
  const abductionDeg = Math.asin(lateral) * deg;
  const flat = Math.hypot(dot(u, parent.u), dot(u, parent.v)) < 1e-9;
  const angleDeg = flat ? 0 : Math.atan2(dot(u, parent.v), dot(u, parent.u)) * deg;
  const [c, s] = [Math.cos(angleDeg / deg), Math.sin(angleDeg / deg)];
  const u1 = add(parent.u, c, parent.v, s);
  const v1 = add(parent.u, -s, parent.v, c);
  const w2 = add(u1, -Math.sin(abductionDeg / deg), parent.w, Math.cos(abductionDeg / deg));
  const twistDeg = Math.atan2(-dot(v, w2), dot(v, v1)) * deg;
  const fields = { parent: parentId, x: round(dot(d, parent.u), 5), y: round(dot(d, parent.v), 5), z: round(dot(d, parent.w), 5), angleDeg: round(angleDeg, 4), abductionDeg: round(abductionDeg, 4), twistDeg: round(twistDeg, 4) };
  process.stdout.write(`${solve}: world o ${show(at)} u ${show(u)} v ${show(v)} w ${show(w)}\n${JSON.stringify(fields)}\n`);
}
