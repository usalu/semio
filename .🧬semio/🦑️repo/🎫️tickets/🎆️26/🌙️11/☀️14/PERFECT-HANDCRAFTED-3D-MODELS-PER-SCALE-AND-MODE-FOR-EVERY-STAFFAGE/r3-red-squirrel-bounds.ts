/** 📦️ Round 3 red squirrel: the world bounding box of every body solid and of the whole body in a pose (default: `presentation.defaultPose`), in millimetres. Usage: bun r3-red-squirrel-bounds.ts <asset|file> --scale N [--pose ID] [--list body] [--all] */
import { cli, listsIn, load, queryOf, rigAt } from "./r3-lib.ts";
const { spec, flag, has } = cli(["--scale", "--pose", "--list"]);
const { asset } = load(spec, "bun r3-red-squirrel-bounds.ts <asset> --scale N");
const scale = Number(flag("--scale") ?? 50);
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), undefined, undefined));
const sm = listsIn(asset, scale, rig.frames);
const list = (flag("--list") ?? "body") as "body";
const total = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
const f = (n: number): string => (n * 1000).toFixed(1).padStart(7);
for (const m of sm[list]) {
  const b = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { b[k] = Math.min(b[k]!, m.world[i + k]!); b[k + 3] = Math.max(b[k + 3]!, m.world[i + k]!); }
  for (let k = 0; k < 3; k++) { total[k] = Math.min(total[k]!, b[k]!); total[k + 3] = Math.max(total[k + 3]!, b[k + 3]!); }
  if (has("--all")) console.log(m.id.padEnd(18), "x", f(b[0]!), f(b[3]!), "y", f(b[1]!), f(b[4]!), "z", f(b[2]!), f(b[5]!));
}
console.log("TOTAL".padEnd(18), "x", f(total[0]!), f(total[3]!), "y", f(total[1]!), f(total[4]!), "z", f(total[2]!), f(total[5]!), "| size", f(total[3]! - total[0]!), f(total[4]! - total[1]!), f(total[5]! - total[2]!));
