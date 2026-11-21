/** 📏 Round-5 measurement of `humans/parent-with-stroller` and `humans/parent-carrying-toddler` against the reference tables: rig segment lengths and, per stated group of body solids of one scale, the extents in the frame of a chosen bone (along the bone u, sagittal v, lateral w) in the default pose; printed as markdown rows for the report.
 * Usage: `bun r5-parent-measure.ts <category/slug> [--scale 50]`
 * @see ./r3-lib.ts */
import { cli, listsIn, load, rigAt, type Frame } from "./r3-lib.ts";

const GROUPS: Record<string, [string, string, RegExp][]> = {
  "parent-with-stroller": [["head (skin, chin to vertex)", "head", /^(head|neck)$/], ["head with hair", "head", /^(head|hair-cap|hair-fall)$/], ["hand (palm, fingers, thumb)", "hand-r", /^(palm-r|finger-r-\d|thumb-r)$/], ["shoe with sole", "foot-r", /^(shoe-r|sole-r|shoe-toe-r|sole-toe-r)$/], ["jacket at the shoulders", "torso", /^(jacket-body|sleeve-upper-[lr])$/], ["jeans seat (hip breadth)", "pelvis", /^jeans-seat$/], ["jeans thigh", "thigh-r", /^jeans-thigh-r$/], ["jeans shank", "shank-r", /^jeans-shank-r$/], ["toddler", "child", /^child-/]],
  "parent-carrying-toddler": [["head (skin)", "head", /^head$/], ["hand (left glove, hanging)", "hand-l", /^glove-l(-|$)/], ["boot", "foot-r", /^boot-(.*-)?r$/], ["thigh (jeans)", "thigh-r", /^thigh-r$/], ["shank (jeans)", "shank-r", /^shank-r$/], ["toddler boot", "toddler-foot-l", /^toddler-boot-(sole-)?l$/], ["coat at the shoulders", "torso", /^(coat|sleeve-[lr])$/], ["jeans seat", "pelvis", /^jeans-seat$/], ["toddler head (skin)", "toddler-head", /^toddler-head$/], ["toddler body", "toddler-torso", /^toddler-/]],
};
const { spec, flag } = cli(["--scale"]);
const { entry, asset } = load(spec, "bun r5-parent-measure.ts <category/slug> [--scale 50]");
const scale = Number(flag("--scale") ?? "50");
const rig = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose });
const sm = listsIn(asset, scale, rig.frames);
console.log(`## ${entry.category}/${entry.slug} 1:${scale}, pose ${asset.presentation.defaultPose}\n`);
console.log("| bone | lengthM |\n|---|---|");
for (const b of asset.rig.bones) console.log(`| ${b.id} | ${b.lengthM} |`);
let lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
for (const m of sm.body) for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, m.world[i + k]!); hi[k] = Math.max(hi[k]!, m.world[i + k]!); }
console.log(`\nbody bounds x ${lo[0]!.toFixed(3)}..${hi[0]!.toFixed(3)}, y ${lo[1]!.toFixed(3)}..${hi[1]!.toFixed(3)}, z ${lo[2]!.toFixed(3)}..${hi[2]!.toFixed(3)} m\n`);
console.log("| group | frame | along u | sagittal v | lateral w |\n|---|---|---|---|---|");
for (const [name, bone, re] of GROUPS[entry.slug] ?? []) {
  const f: Frame = rig.frames[bone]!;
  const a = [1e9, 1e9, 1e9], b = [-1e9, -1e9, -1e9];
  let n = 0;
  for (const m of sm.body) {
    if (!re.test(m.id)) continue;
    n++;
    for (let i = 0; i < m.world.length; i += 3) {
      const d = [m.world[i]! - f.o[0], m.world[i + 1]! - f.o[1], m.world[i + 2]! - f.o[2]];
      const l = [f.u, f.v, f.w].map((ax) => d[0]! * ax[0] + d[1]! * ax[1] + d[2]! * ax[2]);
      for (let k = 0; k < 3; k++) { a[k] = Math.min(a[k]!, l[k]!); b[k] = Math.max(b[k]!, l[k]!); }
    }
  }
  console.log(n === 0 ? `| ${name} | ${bone} | no solid | | |` : `| ${name} (${n}) | ${bone} | ${(b[0]! - a[0]!).toFixed(3)} | ${(b[1]! - a[1]!).toFixed(3)} | ${(b[2]! - a[2]!).toFixed(3)} |`);
}
