import { writeFileSync, mkdirSync } from "node:fs";
import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { authoredScales, catalog, consistency, sweep } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const mine = new Set(process.argv.length > 2 ? process.argv.slice(2) : ["mallard-duck", "pig", "pigeon", "red-fox", "red-squirrel", "roe-deer", "sheep"]);
const out: string[] = [];
const all = catalog();
const known = new Set(all.map((a: any) => a.id));
for (const asset of all) {
  if (!mine.has(asset.id)) continue;
  const report = checkAssetDetailed(asset, { samples: 6, profile: DEFAULT_PRINT_PROFILE, assets: known, physics: true });
  out.push(`${report.problems.length ? "FAIL" : "OK"} ${asset.id}`);
  for (const l of report.lines) out.push("  " + l);
  for (const p of report.problems.slice(0, 40)) out.push("  PROBLEM " + p);
  for (const scale of authoredScales(asset)) {
    const findings = [...sweep(asset, scale), ...consistency(asset, scale)];
    out.push(`  three-views 1:${scale}: ${findings.length === 0 ? "ok" : findings.map((f: any) => f.check + ": " + f.message).join(" | ")}`);
  }
}
const dir = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/A2";
mkdirSync(dir, { recursive: true });
writeFileSync(dir + "/all-output.txt", out.join("\n"), "utf8");
console.log("done", out.length);
