/**
 * 🔬 Dumps the dependency-boundary policy for `manifest-batch2-tools/scan.ts`: the live policy, or the policy rebuilt from an overlay COPY of the library files (taxonomy and construction), over the live workspace authority.
 *
 *   bun dump-policy.ts <out.json> [--overlay <dir>]
 */
import { builtinModules, createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = process.env.SEMIO_ROOT ?? resolve(import.meta.dir, "../../../../../../../..");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const direction = `${library}/🕸️dependencies/🧭️direction`;
const out = process.argv[2]!;
const at = process.argv.indexOf("--overlay");
const overlay = at > 0 ? resolve(process.argv[at + 1]!) : undefined;
const require = createRequire(import.meta.url);
const { loadDependencyDirectionPolicy } = require(join(root, direction, "🚀️bootstrap/🟨️.cjs"));
const live = loadDependencyDirectionPolicy(root);
let policy = live.policy;
if (overlay) {
  const { buildDependencyDirectionPolicy } = require(join(overlay, direction, "🏗️construction/🟨️.cjs"));
  const taxonomy = JSON.parse(readFileSync(join(overlay, library, "🔣️taxonomy.json"), "utf8"));
  policy = buildDependencyDirectionPolicy({ taxonomy, plugins: live.plugins, workspacePackages: live.workspacePackages, nodeBuiltins: builtinModules });
}
writeFileSync(out, JSON.stringify({ forbidden: policy.forbidden, options: policy.options }));
console.log(`${overlay ? "overlay" : "live"} policy: ${policy.forbidden.length} rules, ${live.workspacePackages.length} packages -> ${out}`);
