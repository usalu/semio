/** 🔗️ Lists every group of touching solids of one list per scale in the default pose (the product's `touchingGroups` at the tolerance of `volumes-connected`), with its lowest point and its gap to the nearest other group: `bun rework-tour-guide-groups.ts <asset> [--list structure|body] [--scale N]`. */
import { groupGap, touchingGroups, type Part } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { cli, listsIn, load, queryOf, rigAt } from "./r3-lib.ts";

const args = cli(["--list", "--scale"]);
const { asset } = load(args.spec, "rework-tour-guide-groups.ts <asset>");
const list = (args.flag("--list") ?? "structure") as "structure" | "body";
for (const scale of args.flag("--scale") ? [Number(args.flag("--scale"))] : [50, 100, 200, 500]) {
  const feature = (scale * 0.15) / 1000;
  const rig = rigAt(asset, scale, queryOf(asset, undefined, undefined, undefined));
  const lists = listsIn(asset, scale, rig.frames);
  const kinds = new Map<string, string>(lists.solids.structure.map((s: { id: string; part: string }) => [s.id, s.part]));
  const parts: Part[] = lists[list].map((mesh: any, solid: number) => {
    const positions = Float64Array.from(mesh.world);
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 3) for (let k = 0; k < 3; k++) { min[k] = Math.min(min[k]!, positions[i + k]!); max[k] = Math.max(max[k]!, positions[i + k]!); }
    return { solid, copy: 0, id: mesh.id ?? lists.solids[list][solid].id, positions, indices: Uint32Array.from(mesh.indices), min, max } as unknown as Part;
  }).filter((p: Part) => list !== "structure" || kinds.get(p.id) !== "mass");
  const groups = touchingGroups(parts, feature / 5);
  process.stdout.write(`== 1:${scale} ${list}: ${parts.length} parts, ${groups.count} groups (tolerance ${(feature / 5) * 1000} mm) ${lists.errors.join("; ")}\n`);
  for (let g = 0; g < groups.count; g++) {
    const own = parts.filter((_, i) => groups.group[i] === g);
    const low = Math.min(...own.map((p) => p.min[1]!));
    const gap = groups.count > 1 ? groupGap(parts, groups, g, 0.2) : { gapM: 0, nearest: -1 };
    const ids = own.map((p) => p.id);
    process.stdout.write(`  g${g} n=${own.length} low=${(low * 1000).toFixed(0)}mm gap=${(gap.gapM * 1000).toFixed(1)}mm to ${gap.nearest >= 0 ? parts[gap.nearest]!.id : "-"}: ${ids.length > 14 ? ids.slice(0, 14).join(" ") + " …" : ids.join(" ")}\n`);
  }
}
