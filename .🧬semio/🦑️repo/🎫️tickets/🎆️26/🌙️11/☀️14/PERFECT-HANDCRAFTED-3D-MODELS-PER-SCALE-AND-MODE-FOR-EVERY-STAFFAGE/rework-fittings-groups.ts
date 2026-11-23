/** 🧲 Touching groups of one list of one scale in the default pose, and exact gaps between named solids.
 *
 * Usage: `bun rework-fittings-groups.ts <category>/<slug> <scale> body|structure [--near <id> [mm]] [--pose <id>]`: prints every group apart from the largest with its ids, its gap and the nearest solid; `--near` lists the solids within the cutoff (default 40 mm) of one solid with their exact distances. Reads the catalog asset with the volumes source merged over it.
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🔗️connected/🟦️.ts */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf, solidsOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { groupGap, partGap, partsOf, touchingGroups } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🔗️connected/🟦️.ts";
import { SEAM_FEATURES } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { catalogEntries, loadAsset, loadSource, merged, type Json } from "./volumes-lib.ts";

const [name, scaleText, list, ...rest] = process.argv.slice(2);
const option = (flag: string): string[] => (rest.includes(flag) ? rest.slice(rest.indexOf(flag) + 1) : []);
const entry = catalogEntries().find((e) => `${e.category}/${e.slug}` === name)!;
const source = loadSource(entry);
const catalog = loadAsset(entry.path);
const asset = source === null ? catalog : merged(catalog, source);
const d = Number(scaleText);
const feature = featureOf(d);
const tolerance = SEAM_FEATURES * feature;
const pose = option("--pose")[0] ?? asset.presentation.defaultPose;
const state = evaluate(asset as never, { kind: "pose", pose }, asset.representations[`${d}`]?.status === "authored" ? ({ kind: "scale", scale: { denominator: d } } as never) : undefined);
const body = worldSolids(asset as never, state.bones as never, d, "body");
const solids = solidsOf(asset.volumes[`${d}`] as never, list as never) as readonly Json[];
const worlds = list === "body" ? body : worldSolids(asset as never, state.bones as never, d, list as never, body);
const parts = partsOf(worlds, solids.map((s) => s.repeat?.count ?? 1));
const mm = (m: number): string => (m * 1000).toFixed(1);
const near = option("--near");
if (near.length > 0) {
  const cutoff = Number(near[1] ?? 40) / 1000;
  const own = parts.filter((p) => p.id === near[0]);
  const seen = new Map<string, number>();
  for (const other of parts) if (other.id !== near[0]) for (const part of own) { const gap = partGap(part, other, cutoff); if (gap <= cutoff) seen.set(other.id, Math.min(seen.get(other.id) ?? Infinity, gap)); }
  console.log(`${near[0]} box y ${own.map((p) => `${mm(p.min[1])}..${mm(p.max[1])}`).join(", ")} x ${own.map((p) => `${mm(p.min[0])}..${mm(p.max[0])}`).join(", ")} z ${own.map((p) => `${mm(p.min[2])}..${mm(p.max[2])}`).join(", ")}`);
  for (const [id, gap] of [...seen].sort((a, b) => a[1] - b[1])) console.log(`  ${mm(gap)} mm  ${id}`);
} else {
  const groups = touchingGroups(parts, tolerance);
  const sizes = Array.from({ length: groups.count }, (_, g) => parts.filter((_, i) => groups.group[i] === g).length);
  const largest = sizes.indexOf(Math.max(...sizes));
  console.log(`1:${d} ${list} pose ${pose}: ${parts.length} parts, ${groups.count} groups, tolerance ${mm(tolerance)} mm`);
  for (let g = 0; g < groups.count; g++) {
    if (g === largest) continue;
    const own = parts.filter((_, i) => groups.group[i] === g);
    const gap = groupGap(parts, groups, g, 6 * feature);
    console.log(`  [${[...new Set(own.map((p) => p.id))].join(" ")}] lowest y ${mm(Math.min(...own.map((p) => p.min[1])))} mm, gap ${mm(gap.gapM)} mm to ${gap.nearest < 0 ? "-" : parts[gap.nearest]!.id}`);
  }
}
