/** 📏 Round 5 measurement of a figure against its reference (used for humans/skateboarder and humans/child-on-kick-scooter): per rig bone its length and world joint height in a pose, and the extents of the body solids bound to it in the bone's own frame (u along the bone, v sagittal, w lateral), plus the whole body's world extents.
 *
 * Usage: `bun r5-skateboarder-measure.ts <category/slug> [--scale N] [--pose ID] [--list body|thermal|structure]`
 *
 * @see ./r3-lib.ts */
import { cli, listsIn, load, queryOf, rigAt } from "./r3-lib.ts";

const { spec, flag } = cli(["--scale", "--pose", "--list"]);
const { asset } = load(spec, "bun r5-skateboarder-measure.ts <category/slug> [--scale N] [--pose ID] [--list body|thermal|structure]");
const scale = Number(flag("--scale") ?? 50);
const list = (flag("--list") ?? "body") as "body" | "thermal" | "structure";
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), undefined, undefined));
const lists = listsIn(asset, scale, rig.frames);
const solids = lists.solids[list];
const meshes = lists[list];
const dot = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
const f3 = (x: number): string => x.toFixed(3).padStart(7);
let lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) {
  lo[k] = Math.min(lo[k]!, m.world[i + k]!);
  hi[k] = Math.max(hi[k]!, m.world[i + k]!);
}
process.stdout.write(`${spec} 1:${scale} ${list} pose ${rig.query.kind === "pose" ? rig.query.pose : ""}: world x ${f3(lo[0]!)}..${f3(hi[0]!)}  y ${f3(lo[1]!)}..${f3(hi[1]!)}  z ${f3(lo[2]!)}..${f3(hi[2]!)}\n`);
process.stdout.write(`bone                length  joint-y   u-range           v-range           w-range         solids\n`);
for (const b of rig.bones) {
  const f = rig.frames[b.id]!;
  const len = Math.hypot(b.tip[0] - b.o[0], b.tip[1] - b.o[1], b.tip[2] - b.o[2]);
  const r = [[Infinity, -Infinity], [Infinity, -Infinity], [Infinity, -Infinity]];
  let n = 0;
  solids.forEach((s, i) => {
    if (s.bone !== b.id || s.texture === true || s.slender === true) return;
    n++;
    const m = meshes[i]!;
    for (let j = 0; j < m.world.length; j += 3) {
      const d = [m.world[j]! - f.o[0], m.world[j + 1]! - f.o[1], m.world[j + 2]! - f.o[2]];
      [f.u, f.v, f.w].forEach((ax, k) => {
        const q = dot(d, ax);
        r[k]![0] = Math.min(r[k]![0]!, q);
        r[k]![1] = Math.max(r[k]![1]!, q);
      });
    }
  });
  const rng = (k: number): string => (n ? `${f3(r[k]![0]!)}..${f3(r[k]![1]!)}` : "        -        ");
  process.stdout.write(`${b.id.padEnd(18)} ${f3(len)}  ${f3(b.o[1])}   ${rng(0)} ${rng(1)} ${rng(2)}  ${n}\n`);
}
