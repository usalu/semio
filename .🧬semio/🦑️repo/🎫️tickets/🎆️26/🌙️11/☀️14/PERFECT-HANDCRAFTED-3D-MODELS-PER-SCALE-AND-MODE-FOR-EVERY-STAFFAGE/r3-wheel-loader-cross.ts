/** ⚖️ Product measurements of the wheel loader candidate with groups of solids removed, to find what drives a distance: `bun r3-wheel-loader-cross.ts <rule> <scale> [<finer scale>] <id-prefix,...|-> ...`. */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [rule, ...rest] = process.argv.slice(2);
const scales = rest.filter((x) => /^\d+$/.test(x));
const groups = rest.filter((x) => !/^\d+$/.test(x));
const base = JSON.parse(readFileSync("🗑️generated/r3-wheel-loader/🔣️.json", "utf8")).asset;
for (const group of groups) {
  const drop = group === "-" ? [] : group.split(",");
  const asset = structuredClone(base);
  for (const d of scales) {
    const v = asset.volumes[d];
    const keep = (s: { id: string; of?: string }) => !drop.some((p) => s.id.startsWith(p) || (s.of ?? "").startsWith(p));
    v.body = v.body.filter(keep);
    v.thermal = v.thermal.filter(keep);
    v.structure = v.structure.filter(keep);
  }
  const m = measureVolumes(asset, { scale: Number(scales[0]) as never, rules: [rule as never] });
  console.log(`[DEBUG] drop ${group}: ` + m.filter((x) => /^(p95M|outsideBodyM|iou:)/.test(x.name)).map((x) => `${x.name}=${x.value.toFixed(4)}`).join(" "));
}
