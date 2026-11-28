/** 🧿️ The geometry findings of `🧊️model-catalog` without the file audit: per asset the findings of `auditGeometry` (or the error it throws), the triangles of every scene and, with `--report <substring>`, the weld report of every solid whose id contains the substring (triangles, boundary and overlapped edges, signed volume, whether the oracle expects it closed). geo.ts [--report <substring>] <asset id | file.json>… */
import { readFileSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { auditGeometry } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";

const args = process.argv.slice(2);
const report = args.includes("--report") ? args[args.indexOf("--report") + 1] : undefined;
const targets = args.filter((a, i) => !a.startsWith("--") && args[i - 1] !== "--report");
const entries = await assetPathsFromFiles();
let bad = 0;
for (const target of targets) {
  const asset = target.endsWith(".json") ? JSON.parse(readFileSync(target, "utf8")).asset : (await readStreamedAsset(entries.find((e) => e.folder === target)!)).asset;
  try {
    const audit = await auditGeometry(asset);
    bad += audit.findings.length;
    console.log(`${asset.id} findings ${audit.findings.length}: ${audit.findings.map((f) => `${f.check} ${f.item}: ${f.message}`).join(" | ")}`);
    console.log(`${asset.id} scenes ${audit.facts.scenes.map((s) => `${s.scene}=${s.triangles}`).join(" ")}`);
    if (report !== undefined) for (const r of audit.solids.filter((s) => s.id.includes(report))) console.log(`  ${r.id} ${r.kind}: ${r.triangles} triangles, ${r.degenerate} degenerate, ${r.boundary} boundary, ${r.overlapped} overlapped, volume ${(r.volume * 1e6).toFixed(3)} cm3, closed expected ${r.closedExpected}`);
  } catch (error) {
    bad += 1;
    console.log(`${asset.id} ERROR ${String(error).slice(0, 600)}`);
  }
}
process.exit(bad > 0 ? 1 : 0);
