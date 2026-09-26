/** ⏱️ R8 probe: wall time of one `policyMutationStructuralBreaches` call on a minimal git-initialised fixture root (the unit every
 * direct-mutation law pays per vector). Usage: bun mutation-policy-cost-probe.ts <scratch-dir> [calls] */
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { policyMutationStructuralBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts";

const repo = "/Users/ueli/Documents/semio";
const root = mkdtempSync(join(process.argv[2]!, "r8-policy-"));
spawnSync("git", ["init", "--quiet", "--template="], { cwd: root });
for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json"]) {
  mkdirSync(join(root, dirname(path)), { recursive: true });
  copyFileSync(join(repo, path), join(root, path));
}
const mutations = join(root, "✏️s/🔌️plugins/🧪️probe/🗿️artifacts/🧪️artifact/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
mkdirSync(join(mutations, "➕️insert-page"), { recursive: true });
writeFileSync(join(mutations, "🦀️.rs"), "pub enum PageMutation { InsertPage(insert_page::InsertPage) }\n");
writeFileSync(join(mutations, "➕️insert-page", "🦀️.rs"), "pub struct InsertPage;\n");
const rel = "✏️s/🔌️plugins/🧪️probe/🗿️artifacts/🧪️artifact/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
for (let call = 0; call < Number(process.argv[3] ?? 3); call++) {
  const started = performance.now();
  const breaches = policyMutationStructuralBreaches(root, [rel]);
  console.log(`call ${call}: ${(performance.now() - started).toFixed(0)} ms, ${breaches.length} breaches`);
}
