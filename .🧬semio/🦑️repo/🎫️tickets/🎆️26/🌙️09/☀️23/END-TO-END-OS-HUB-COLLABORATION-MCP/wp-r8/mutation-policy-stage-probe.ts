/** ⏱️ R8 probe: per-stage wall time of the mutation policy pipeline on a minimal fixture root. Usage: bun mutation-policy-stage-probe.ts <scratch-dir> */
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation";
const { mutationTaxonomySourceAdmission, mutationTaxonomyStructuralView } = await import(`${L}/📸️captured-source/🟦️.ts`);
const { mutationTaxonomySourceIndex } = await import(`${L}/📇️index/🟦️.ts`);
const reach = await import(`${L}/📐️structural-reachability/🟦️.ts`);
const repo = "/Users/ueli/Documents/semio";
const root = mkdtempSync(join(process.argv[2]!, "r8-stage-"));
spawnSync("git", ["init", "--quiet", "--template="], { cwd: root });
for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json"]) {
  mkdirSync(join(root, dirname(path)), { recursive: true });
  copyFileSync(join(repo, path), join(root, path));
}
const rel = "✏️s/🔌️plugins/🧪️probe/🗿️artifacts/🧪️artifact/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
mkdirSync(join(root, rel, "➕️insert-page"), { recursive: true });
writeFileSync(join(root, rel, "🦀️.rs"), "pub enum PageMutation { InsertPage(insert_page::InsertPage) }\n");
writeFileSync(join(root, rel, "➕️insert-page", "🦀️.rs"), "pub struct InsertPage;\n");
for (let round = 0; round < 3; round++) {
  let t = performance.now();
  const lap = (label: string) => { const now = performance.now(); console.log(`  ${label}: ${(now - t).toFixed(0)} ms`); t = now; };
  let last = performance.now(); const admission = mutationTaxonomySourceAdmission(root, { progress: (e: any) => { const now = performance.now(); console.log(`    +${(now - last).toFixed(0)} ms ${e.phase} ${e.current}/${e.total}`); last = now; } }); lap("admission");
  const index = mutationTaxonomySourceIndex(root, {}, admission); lap("index");
  const view = mutationTaxonomyStructuralView(index); lap("view");
  reach.policyMutationStructuralBreachesView(view, [rel]); lap("policy");
}
