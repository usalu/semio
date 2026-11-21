/** 📐 Model-versus-reference measurements of a human figure (used for `humans/person-with-rollator` and `humans/pregnant-person`): joint heights and bone lengths of the evaluated rig, extents of each bone's body solids in its own frame, and horizontal body slices (depth, width, convex girth) at named heights, for every authored scale.
 *
 * Usage: `bun r5-person-with-rollator-measure.ts <category/slug> [--pose ID] [--scales 50,100] [name:bone1+bone2:height ...]`
 * @see ./r5-brief.md */
import { cli, listsIn, load, rigAt } from "./r3-lib.ts";
import type { Json, SolidMesh } from "./volumes-lib.ts";

const { spec, flag } = cli(["--pose", "--scales"]);
const { asset } = load(spec, "bun r5-person-with-rollator-measure.ts <category/slug> [--pose ID] [name:bones:height]");
const pose = flag("--pose") ?? asset.presentation.defaultPose;
const scales = (flag("--scales") ?? "50,100,200,500").split(",");
const rig = rigAt(asset, 50, { kind: "pose", pose });
const F = rig.frames;
const f3 = (x: number): string => x.toFixed(3);
const dist = (a: readonly number[], b: readonly number[]): number => Math.hypot(a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!);
console.log(`# ${spec} pose ${pose}`);
const joints: [string, string][] = [["hip", "thigh-l"], ["knee", "shank-l"], ["ankle", "foot-l"], ["ball", "toe-l"], ["shoulder", "arm-l"], ["elbow", "forearm-l"], ["wrist", "hand-l"], ["C7", "neck"], ["head joint", "head"], ["lumbosacral", "torso"]];
for (const [n, b] of joints) {
  const r = b.replace("-l", "-r");
  if (F[b] && F[r]) console.log(`joint ${n.padEnd(12)} y ${f3(F[b]!.o[1])} / ${f3(F[r]!.o[1])}  z ${f3(F[b]!.o[2])} / ${f3(F[r]!.o[2])}`);
}
const tip = (id: string): readonly number[] => rig.bones.find((b) => b.id === id)!.tip;
for (const [n, a, b] of [["thigh", "thigh-l", "shank-l"], ["shank", "shank-l", "foot-l"], ["upper arm", "arm-l", "forearm-l"], ["forearm", "forearm-l", "hand-l"]] as const) console.log(`length ${n.padEnd(10)} ${f3(dist(F[a]!.o, F[b]!.o))}`);
console.log(`length head bone ${f3(dist(F.head!.o, tip("head")))}  shoulder joints apart ${f3(dist(F["arm-l"]!.o, F["arm-r"]!.o))}  hip joints apart ${f3(dist(F["thigh-l"]!.o, F["thigh-r"]!.o))}`);
const slices = process.argv.slice(2).filter((a) => a.includes(":") && !a.startsWith("--"));
function hull(p: [number, number][]): [number, number][] {
  const s = [...p].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cr = (o: readonly number[], a: readonly number[], b: readonly number[]): number => (a[0]! - o[0]!) * (b[1]! - o[1]!) - (a[1]! - o[1]!) * (b[0]! - o[0]!);
  const lo: [number, number][] = [];
  const up: [number, number][] = [];
  for (const q of s) { while (lo.length > 1 && cr(lo[lo.length - 2]!, lo[lo.length - 1]!, q) <= 0) lo.pop(); lo.push(q); }
  for (const q of s.reverse()) { while (up.length > 1 && cr(up[up.length - 2]!, up[up.length - 1]!, q) <= 0) up.pop(); up.push(q); }
  return [...lo.slice(0, -1), ...up.slice(0, -1)];
}
function sliceAt(meshes: readonly SolidMesh[], y: number): [number, number][] {
  const pts: [number, number][] = [];
  for (const m of meshes) for (let i = 0; i < m.indices.length; i += 3) {
    const v = [0, 1, 2].map((k) => { const j = m.indices[i + k]! * 3; return [m.world[j]!, m.world[j + 1]!, m.world[j + 2]!]; });
    for (const [a, b] of [[v[0]!, v[1]!], [v[1]!, v[2]!], [v[2]!, v[0]!]]) if ((a[1]! - y) * (b[1]! - y) < 0) { const t = (y - a[1]!) / (b[1]! - a[1]!); pts.push([a[0]! + t * (b[0]! - a[0]!), a[2]! + t * (b[2]! - a[2]!)]); }
  }
  return pts;
}
for (const scale of scales) {
  if (asset.volumes[scale]?.status !== "authored") continue;
  const lists = listsIn(asset, Number(scale), F);
  const bones = new Map((asset.volumes[scale].body as Json[]).map((s) => [s.id, s.bone as string]));
  let ymax = -1;
  const ex: Record<string, number[][]> = {};
  for (const m of lists.body) {
    const b = bones.get(m.id)!;
    const f = F[b]!;
    for (let i = 0; i < m.world.length; i += 3) {
      const p = [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!];
      ymax = Math.max(ymax, p[1]!);
      const d = [p[0]! - f.o[0], p[1]! - f.o[1], p[2]! - f.o[2]];
      const l = [f.u, f.v, f.w].map((a) => d[0]! * a[0] + d[1]! * a[1] + d[2]! * a[2]);
      const e = (ex[b] ??= [[9, -9], [9, -9], [9, -9]]);
      for (let k = 0; k < 3; k++) { e[k]![0] = Math.min(e[k]![0]!, l[k]!); e[k]![1] = Math.max(e[k]![1]!, l[k]!); }
    }
  }
  console.log(`\n## 1:${scale} body  top ${f3(ymax)}  solids ${lists.body.length}`);
  for (const [b, e] of Object.entries(ex)) console.log(`bone ${b.padEnd(10)} u ${f3(e[0]![1]! - e[0]![0]!)}  v ${f3(e[1]![1]! - e[1]![0]!)}  w ${f3(e[2]![1]! - e[2]![0]!)}`);
  for (const s of slices) {
    const [name, bs, h] = s.split(":");
    const set = new Set(bs!.split("+"));
    const pts = sliceAt(lists.body.filter((m) => set.has(bones.get(m.id)!)), Number(h));
    if (pts.length < 3) { console.log(`slice ${name} at ${h}: empty`); continue; }
    const hl = hull(pts);
    const per = hl.reduce((a, p, i) => a + Math.hypot(p[0] - hl[(i + 1) % hl.length]![0], p[1] - hl[(i + 1) % hl.length]![1]), 0);
    const xs = pts.map((p) => p[0]);
    const zs = pts.map((p) => p[1]);
    console.log(`slice ${name!.padEnd(12)} y ${h}  depth ${f3(Math.max(...xs) - Math.min(...xs))}  width ${f3(Math.max(...zs) - Math.min(...zs))}  convex girth ${f3(per)}`);
  }
}
