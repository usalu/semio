import { readFileSync, writeFileSync, appendFileSync, mkdirSync } from "node:fs";
import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { authoredScales, catalog, consistency, sweep } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

// usage: all.ts [--no-check] [--no-views] [--only-pose id] [asset ...]
const args = process.argv.slice(2);
console.log("[DEBUG] args", JSON.stringify(args));
const noCheck = args.includes("--no-check");
const noViews = args.includes("--no-views");
const ids = args.filter((a) => !a.startsWith("--"));
const mine = new Set(ids.length ? ids : ["mallard-duck", "pig", "pigeon", "red-fox", "red-squirrel", "roe-deer", "sheep"]);
const CAT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals";
const T = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE";
const dir = T + "/🗑️generated/QA8";
mkdirSync(dir + "/sheets", { recursive: true });
const out: string[] = [];
writeFileSync(dir + "/all-output.txt", "", "utf8");
const flush = (): void => { const t = out.splice(0).join("\n") + "\n"; console.log(t); appendFileSync(dir + "/all-output.txt", t, "utf8"); };
const all = catalog();
const known = new Set(all.map((a: any) => a.id));
for (const asset of all) {
  if (!mine.has(asset.id)) continue;
  if (!noCheck) {
    const report = checkAssetDetailed(asset, { samples: 6, profile: DEFAULT_PRINT_PROFILE, assets: known, physics: true });
    out.push(`${report.problems.length ? "FAIL" : "OK"} ${asset.id}`);
    for (const l of report.lines) out.push("  " + l);
    for (const p of report.problems.slice(0, 40)) out.push("  PROBLEM " + p);
    for (const scale of authoredScales(asset)) {
      const findings = [...sweep(asset, scale), ...consistency(asset, scale)];
      out.push(`  three-views 1:${scale}: ${findings.length === 0 ? "ok" : findings.map((f: any) => f.check + ": " + f.message).join(" | ")}`);
    }
  }
  flush();
  if (noViews) continue;
  const file = `${CAT}/${asset.id}/🔣️.json`;
  const doc = JSON.parse(readFileSync(file, "utf8")).asset;
  const scales = authoredScales(asset);
  for (const scale of scales) {
    const jobs: string[][] = [];
    for (const p of doc.poses.filter((p: any) => !p.key)) jobs.push(["--pose", p.id]);
    for (const c of doc.clips) for (const f of [0.1, 0.45, 0.8]) jobs.push(["--clip", c.id, "--time", (c.durationS * f).toFixed(2)]);
    for (const j of jobs) {
      try {
        await runPreview(["--file", file, "--scale", String(scale), "--views", "--no-png", "--out", `${dir}/sheets/${asset.id}-${scale}`, ...j]);
      } catch (e: any) {
        out.push(`  PREVIEW FAIL ${asset.id} 1:${scale} ${j.join(" ")}: ${e?.message}`);
      }
    }
    out.push(`  previews ${asset.id} 1:${scale}: ${jobs.length}`);
  }
}
writeFileSync(dir + "/all-output.txt", out.join("\n"), "utf8");
console.log("done", out.length);
