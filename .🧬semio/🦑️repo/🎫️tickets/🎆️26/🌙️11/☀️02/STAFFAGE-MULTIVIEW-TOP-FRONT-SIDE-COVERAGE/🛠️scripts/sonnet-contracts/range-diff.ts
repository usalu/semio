import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const root = "C:/git/semio";
const catalog = `${root}/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog`;
type Range = Record<string, { status: string; min?: number; max?: number }>;
const rangesOf = (asset: any): Range => Object.fromEntries(Object.entries(asset.representations as Record<string, any>).map(([key, rep]) => [key, rep.status === "authored" ? { status: "authored", min: rep.validRange.minDenominator, max: rep.validRange.maxDenominator } : { status: "unavailable" }]));
const rows: string[] = [];
for (const category of readdirSync(catalog, { withFileTypes: true }).filter((entry) => entry.isDirectory())) {
  for (const slug of readdirSync(join(catalog, category.name), { withFileTypes: true }).filter((entry) => entry.isDirectory())) {
    const file = join(catalog, category.name, slug.name, "🔣️.json");
    let before: any;
    try {
      before = JSON.parse(execFileSync("git", ["-C", root, "show", `HEAD:🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/${category.name}/${slug.name}/🔣️.json`], { encoding: "utf8", maxBuffer: 1 << 28 })).asset;
    } catch {
      continue;
    }
    const after = JSON.parse(readFileSync(file, "utf8")).asset;
    const a = rangesOf(before);
    const b = rangesOf(after);
    if (JSON.stringify(a) !== JSON.stringify(b)) rows.push(`${category.name}/${slug.name}\n  before ${JSON.stringify(a)}\n  after  ${JSON.stringify(b)}`);
    const presets = [50, 100, 200, 500];
    presets.forEach((d, i) => {
      const x = b[`${d}`];
      const y = b[`${presets[i + 1]}`];
      if (x?.status === "authored" && y?.status === "authored" && y.min! - x.max! > 1) rows.push(`GAP ${category.name}/${slug.name} ${d}->${presets[i + 1]}: ${x.max} ${y.min}`);
      if (x?.status === "authored" && y?.status === "authored" && x.max! > y.min!) rows.push(`OVERLAP ${category.name}/${slug.name} ${d}->${presets[i + 1]}`);
    });
  }
}
console.log(rows.join("\n") || "no range changes");
