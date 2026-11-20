/** ⚖️ Cross-scale front/plan/elevation agreement of the forklift candidate with groups of solids removed from both scales, to find what drives a cross-scale distance: `bun r3-forklift-cross.ts <coarse> <fine> <id-prefix,...|-> ...`. */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [coarse, fine, ...groups] = process.argv.slice(2);
const base = JSON.parse(readFileSync("🗑️generated/r3-forklift/forklift-r3/🔣️.json", "utf8")).asset;
for (const group of groups) {
  const drop = group === "-" ? [] : group.split(",");
  const asset = structuredClone(base);
  for (const d of [coarse, fine]) {
    const v = asset.volumes[d!];
    const keep = (s: { id: string; of?: string }) => !drop.some((p) => s.id.startsWith(p) || (s.of ?? "").startsWith(p));
    v.body = v.body.filter(keep);
    v.thermal = v.thermal.filter(keep);
    v.structure = v.structure.filter(keep);
  }
  const m = measureVolumes(asset, { scale: Number(coarse) as never, rules: ["volumes-cross-scale"] });
  console.log(`[DEBUG] drop ${group}: ` + m.filter((x) => x.name.startsWith("p95M")).map((x) => `${x.name}=${x.value.toFixed(4)}`).join(" "));
}
