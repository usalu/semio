import { readFileSync, writeFileSync } from "node:fs";
import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const [, , out, physics, ...ids] = process.argv;
const lines: string[] = [];
for (const id of ids) {
  const asset = JSON.parse(readFileSync(`${S}/🗂️catalog/🪑️objects/${id}/🔣️.json`, "utf8")).asset;
  const report = checkAssetDetailed(asset, { physics: physics === "physics", samples: 5 });
  lines.push(`== ${id} ${report.problems.length === 0 ? "OK" : "PROBLEMS"}`);
  lines.push(...report.lines.map((l) => `  ${l}`));
  lines.push(...report.problems.map((p) => `  ! ${p}`));
}
writeFileSync(out!, lines.join("\n") + "\n");
console.log("[DEBUG] check written");
