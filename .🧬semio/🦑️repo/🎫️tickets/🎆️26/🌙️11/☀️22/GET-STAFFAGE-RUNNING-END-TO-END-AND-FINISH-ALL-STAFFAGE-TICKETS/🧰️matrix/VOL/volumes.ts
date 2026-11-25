/** ⚖️ `volumeIssues`, `exportIssues`, the geometry findings of the model-catalog oracles over every scale and `checkVolumes` at every authored scale (or --scales 500), on a catalog document or a file: volumes.ts <id | document>… */
import { readFileSync } from "node:fs";
import { assetFile, mod } from "../../🧰️linux/s4/lib.ts";
const args = process.argv.slice(2);
const at = args.indexOf("--scales");
const only = at < 0 ? undefined : args.splice(at, 2)[1]!.split(",").map(Number);
const { volumeIssues, exportIssues } = await mod("🔨️modules/🗂️catalog/🧊️volume/🟦️.ts");
const { auditGeometry } = await mod("🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts");
const { checkVolumes } = await mod("🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
for (const target of args) {
  const asset = JSON.parse(readFileSync(target.endsWith(".json") ? target : assetFile(target), "utf8")).asset;
  const issues = [...volumeIssues(asset), ...exportIssues(asset)];
  const { findings } = await auditGeometry(asset);
  const rules = (only ?? [50, 100, 200, 500]).filter((scale) => asset.volumes[scale].status === "authored").flatMap((scale) => checkVolumes(asset, { scale }).map((f: any) => `1:${scale} ${f.rule} ${f.path}: ${f.message.en}`));
  console.log(`${asset.id} volumes: volumeIssues+exportIssues ${issues.length}, geometry ${findings.length}, checkVolumes ${rules.length}`);
  for (const line of [...issues.map((i: any) => `${i.code} ${i.path}`), ...findings.map((f: any) => `${f.check} ${f.item}: ${f.message}`), ...rules].slice(0, 12)) console.log("  ", line.slice(0, 400));
}
