/** 🪢 Every group of touching solids of one list of the excavator with the nearest solid of another group, so that each loose member can be joined by hand.
 * @see ./rework-10-connected.ts */
import { groupGap, partGap, partsOf, touchingGroups } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { cli, load } from "./r3-lib.ts";

const { spec, flag } = cli(["--scale", "--list", "--pose", "--masses"]);
const { asset } = load(spec, "rework-excavator-connected.ts <asset> [--scale N] [--list body|structure] [--pose ID]") as { asset: any };
const list = (flag("--list") ?? "structure") as "body" | "structure";
const pose = flag("--pose") ?? asset.presentation.defaultPose;
for (const d of flag("--scale") === undefined ? [50, 100, 200, 500] : [Number(flag("--scale"))]) {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: d } } as never);
  const body = worldSolids(asset, state.bones as never, d, "body");
  const worlds = list === "body" ? body : worldSolids(asset, state.bones as never, d, list, body);
  const solids = asset.volumes[`${d}`][list] as readonly any[];
  const masses = new Set(solids.filter((s) => s.part === "mass").map((s) => s.id as string));
  const parts = partsOf(worlds, solids.map((s) => s.repeat?.count ?? 1)).filter((p) => flag("--masses") !== "off" || !masses.has(p.id));
  const tolerance = (0.2 * 0.15 * d) / 1000;
  const groups = touchingGroups(parts, tolerance);
  const cap = (6 * 0.15 * d) / 1000;
  console.log(`1:${d} ${list} pose ${pose}: ${parts.length} parts, ${groups.count} groups, tolerance ${(tolerance * 1000).toFixed(1)} mm`);
  for (let g = 0; g < groups.count; g++) {
    const own = parts.filter((_, i) => groups.group[i] === g);
    const ids = [...new Set(own.map((p) => p.id))];
    const { gapM, nearest } = groupGap(parts, groups, g, cap);
    let from = "";
    if (nearest >= 0) {
      let best = Infinity;
      for (const p of own) { const x = partGap(p, parts[nearest]!, cap); if (x < best) [best, from] = [x, p.id]; }
    }
    const minY = Math.min(...own.map((p) => p.min[1]!));
    console.log(`  g${g} n=${own.length} minY=${minY.toFixed(3)} gap ${(gapM * 1000).toFixed(1)} mm ${from} -> ${nearest < 0 ? "-" : parts[nearest]!.id}: ${ids.slice(0, 10).join(" ")}${ids.length > 10 ? ` … (${ids.length})` : ""}`);
  }
}
