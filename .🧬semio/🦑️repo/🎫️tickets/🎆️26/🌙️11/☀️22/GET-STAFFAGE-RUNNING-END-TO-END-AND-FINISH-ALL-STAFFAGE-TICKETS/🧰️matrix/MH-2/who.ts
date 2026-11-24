/** 🕵️ Which shapes make the extent of each view in one subject: `bun who.ts <id|file.json> <scale> "<subject id>" [bone-regex]` evaluates the subject with that id (as `gate.ts` prints it; a clip instant is taken at the printed time, to the millisecond, e.g. `transition a>b @0.5`, `pose x`, `clip c key k @1.000 s`) and prints, per shared axis (height: elevation y and front y; forward: elevation x and plan x; lateral: front x and plan y), the two shapes that reach furthest at each end in both views; with a bone regex it prints instead the extent of every shape of the matching bones in the three views and the anchors of those bones. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [arg, scale, id, pattern] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const pose = /^pose (\S+)$/.exec(id!);
const transition = /^transition (\S+)>(\S+) @([\d.]+)$/.exec(id!);
const clip = /^clip (\S+) .*@([\d.]+) s$/.exec(id!);
const query = pose ? { kind: "pose", pose: pose[1] } : transition ? { kind: "transition", from: transition[1], to: transition[2], durationS: 1, timeS: Number(transition[3]) } : clip ? { kind: "clip", clip: clip[1], timeS: Number(clip[2]) } : undefined;
if (query === undefined) throw new Error(`cannot read the subject "${id}"`);
const state = evaluate(asset, query, engine.scaleSelector(Number(scale)));
const range = (shape: any, axis: "x" | "y"): [number, number] => {
  const values = shape.rings.flat().map((p: any) => p[axis] as number);
  return [Math.min(...values), Math.max(...values)];
};
const f = (v: number): string => v.toFixed(3);
if (pattern !== undefined) {
  const rx = new RegExp(pattern);
  for (const view of ["elevation", "front", "plan"])
    for (const shape of state.shapes[view]) if (rx.test(shape.shape.bone) && shape.rings.flat().length > 0) console.log(`${view.padEnd(9)} ${shape.shape.bone.padEnd(14)} ${shape.shape.id.padEnd(24)} x [${range(shape, "x").map(f)}] y [${range(shape, "y").map(f)}]`);
  for (const [name, { anchor, at, z }] of Object.entries(state.anchors) as any) if (rx.test(anchor.bone)) console.log(`anchor ${name} on ${anchor.bone} (${anchor.kind}) ahead ${f(at.x)} up ${f(at.y)} left ${f(z)}`);
} else
  for (const [name, a, aAxis, b, bAxis] of [["height", "elevation", "y", "front", "y"], ["forward", "elevation", "x", "plan", "x"], ["lateral", "front", "x", "plan", "y"]] as const)
    for (const [view, axis] of [[a, aAxis], [b, bAxis]] as const) {
      const shapes = state.shapes[view].filter((shape: any) => shape.shape.role !== "ground" && shape.rings.flat().length > 0).map((shape: any) => ({ key: `${shape.shape.bone}/${shape.shape.id}`, r: range(shape, axis) }));
      const low = [...shapes].sort((x, y) => x.r[0] - y.r[0]).slice(0, 2).map((s) => `${s.key} ${f(s.r[0])}`);
      const high = [...shapes].sort((x, y) => y.r[1] - x.r[1]).slice(0, 2).map((s) => `${s.key} ${f(s.r[1])}`);
      console.log(`${name.padEnd(8)} ${view.padEnd(9)} min: ${low.join(", ")}   max: ${high.join(", ")}`);
    }
