import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { authoredScales, catalog, consistency, sweep } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/O2";
const [, , round, previewsArg, ...ids] = process.argv;
const dir = `${OUT}/${round}`;
mkdirSync(dir, { recursive: true });
const previews: Record<string, { scales: string[]; whens: string[] }> = previewsArg === "none" ? {} : JSON.parse(previewsArg!);

const lines: string[] = [];
for (const id of ids) {
  const asset = JSON.parse(readFileSync(`${S}/🗂️catalog/🪑️objects/${id}/🔣️.json`, "utf8")).asset;
  const report = checkAssetDetailed(asset, { physics: false, samples: 5 });
  lines.push(`== ${id} ${report.problems.length === 0 ? "OK" : "PROBLEMS"}`, ...report.lines.map((l) => `  ${l}`), ...report.problems.map((p) => `  ! ${p}`));
}
writeFileSync(`${dir}/check.txt`, lines.join("\n") + "\n");

const cons: string[] = [];
for (const asset of catalog()) {
  if (!ids.includes(asset.id)) continue;
  for (const scale of authoredScales(asset)) {
    const findings = [...sweep(asset, scale), ...consistency(asset, scale)];
    cons.push(`${asset.id} 1:${scale} ${findings.length === 0 ? "ok" : findings.map((f) => `[${f.check}] ${f.message}`).join(" | ")}`);
  }
}
writeFileSync(`${dir}/consistency.txt`, cons.join("\n") + "\n");

for (const [id, job] of Object.entries(previews)) {
  for (const scale of job.scales) {
    for (const when of job.whens) {
      const [kind, name, time] = when.split(":");
      const args = ["--file", `${S}/🗂️catalog/🪑️objects/${id}/🔣️.json`, "--scale", scale, "--views", "--no-png", "--out", `${dir}/${id}`];
      if (kind === "clip") args.push("--clip", name!, "--time", time ?? "0");
      else args.push("--pose", name!);
      try {
        await runPreview(args);
      } catch (error) {
        cons.push(`[render failed] ${id} ${scale} ${when} ${(error as Error).message}`);
      }
    }
  }
}
writeFileSync(`${dir}/consistency.txt`, cons.join("\n") + "\n");
writeFileSync(`${dir}/DONE`, "done\n");
console.log("[DEBUG] all done", round);
