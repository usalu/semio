/** 🔗 Lists every group of touching solids of one list of machinery/concrete-mixer-truck apart from the largest, as the product rule `volumes-connected` groups them: ids, box and the three nearest solids of other groups with their exact gap.
 *
 * Usage: `bun rework-concrete-mixer-truck-gaps.ts <scale> [body|structure] [file]`
 * @see ./r3-validate.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { partGap, partsOf, touchingGroups } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🔗️connected/🟦️.ts";
import { solidsOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️lists/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const scale = Number(process.argv[2]);
const list = (process.argv[3] ?? "body") as "body" | "structure";
const file = process.argv[4] ?? "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/concrete-mixer-truck/🔣️.json";
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const feature = (7.5 * scale) / 50 / 1000;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: scale } } as never);
const volume = asset.volumes[String(scale)];
const body = worldSolids(asset, state.bones as never, scale, "body");
const worlds = list === "body" ? body : worldSolids(asset, state.bones as never, scale, list, body);
const solids = solidsOf(volume, list);
const kinds = new Map<string, string>(volume.structure.map((s: { id: string; part: string }) => [s.id, s.part]));
const parts = partsOf(worlds, solids.map((solid: { repeat?: { count: number } }) => solid.repeat?.count ?? 1)).filter((p) => list === "body" || kinds.get(p.id) !== "mass");
const groups = touchingGroups(parts, 0.2 * feature);
const f = (v: readonly number[]): string => v.map((x) => x.toFixed(3)).join(",");
for (let g = 1; g < groups.count; g++) {
  const own = parts.filter((_, i) => groups.group[i] === g);
  const near: { id: string; own: string; gap: number }[] = [];
  parts.forEach((other, j) => {
    if (groups.group[j] === g) return;
    let best = { id: other.id, own: "", gap: Infinity };
    for (const part of own) {
      const gap = Math.sqrt(Math.max(0, partGap(part, other, 0.4) ** 2)) ;
      if (gap < best.gap) best = { id: other.id, own: part.id, gap };
    }
    if (best.gap < 0.4) near.push(best);
  });
  near.sort((a, b) => a.gap - b.gap);
  const min = [0, 1, 2].map((k) => Math.min(...own.map((p) => p.min[k]!)));
  const max = [0, 1, 2].map((k) => Math.max(...own.map((p) => p.max[k]!)));
  process.stdout.write(`G${g} [${[...new Set(own.map((p) => p.id))].join(" ")}] box ${f(min)} .. ${f(max)}\n   near: ${near.slice(0, 3).map((n) => `${n.own}->${n.id}(g${groups.group[parts.findIndex((p) => p.id === n.id)]}) ${(n.gap * 1000).toFixed(1)}mm`).join("; ")}\n`);
}
