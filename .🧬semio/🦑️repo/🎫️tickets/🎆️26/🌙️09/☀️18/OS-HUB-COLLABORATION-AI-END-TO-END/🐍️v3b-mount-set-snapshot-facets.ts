/** 🧷️ Mounts the authored `📸️set-snapshot` behaviour facets (`🔺️diff`, `↩️inverse`, `🦠️mutation`) from
 * the mutation's own direct leaf, the way `🗿️artifacts/🖊️dwg/🦀️.rs:308-316` already mounts them.
 *
 * The facets are the taxonomy's intended home for a mutation's `diff`/`inverse` bodies
 * (`mutationDirectLeafInlinedBehaviorFacets`, `📚️library/🔍️discovery/🟦️.ts:3877-3894`, forbids
 * re-inlining them into the direct leaf), yet in 28 of the 29 artifacts that carry them nothing
 * mounts them, so they never compile. Children of a `#[path]`-mounted file resolve against that
 * file's own directory — the same rule `🧬️mutations/🦀️.rs` already relies on for its kind leaves.
 *
 * Re-reads each leaf immediately before writing so a peer's concurrent edit survives. Idempotent.
 *
 * Usage: `bun 🐍️v3b-mount-set-snapshot-facets.ts <repoRoot> [--apply]`
 */
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = process.argv[2]!;
const apply = process.argv.includes("--apply");

const FACETS: readonly (readonly [string, string])[] = [
  ["🔺️diff", "diff"],
  ["↩️inverse", "inverse"],
  ["🦠️mutation", "mutation"],
];

function walk(dir: string, out: string[]): void {
  for (const name of readdirSync(dir)) {
    if (name.startsWith(".") || name === "target" || name === "node_modules" || name === "dist") continue;
    const abs = join(dir, name);
    let st;
    try {
      st = statSync(abs);
    } catch {
      continue;
    }
    if (!st.isDirectory()) continue;
    if (name === "📸️set-snapshot") out.push(abs);
    else walk(abs, out);
  }
}

const dirs: string[] = [];
walk(join(repoRoot, "✏️s/🔌️plugins"), dirs);
if (dirs.length === 0) throw new Error("discovery found no 📸️set-snapshot directories — refusing to act on an empty set");
console.log(`📸️set-snapshot directories: ${dirs.length}`);

let changed = 0;
let skippedNoLeaf = 0;
let alreadyMounted = 0;
for (const dir of dirs.sort()) {
  const leaf = join(dir, "🦀️.rs");
  if (!existsSync(leaf)) {
    skippedNoLeaf++;
    continue;
  }
  const present = FACETS.filter(([facetDir]) => existsSync(join(dir, facetDir, "🦀️.rs")));
  if (present.length === 0) continue;
  const source = readFileSync(leaf, "utf8");
  const missing = present.filter(([facetDir]) => !source.includes(`#[path = "${facetDir}/🦀️.rs"]`));
  if (missing.length === 0) {
    alreadyMounted++;
    continue;
  }
  const block = ["//#region 🔖️Facets", ...missing.flatMap(([facetDir, moduleName]) => [`#[path = "${facetDir}/🦀️.rs"]`, `pub mod ${moduleName};`]), "//#endregion 🔖️Facets", ""].join("\n");
  const anchor = source.indexOf("//#region ");
  const next = anchor >= 0 ? source : `${source}\n`;
  const updated = anchor >= 0 ? next.slice(0, anchor) + block + next.slice(anchor) : next + block;
  console.log(`${apply ? "mount" : "would mount"} ${missing.map(([, name]) => name).join(",")}\t${leaf.slice(repoRoot.length + 1)}`);
  if (apply) writeFileSync(leaf, updated);
  changed++;
}
console.log(`leaves changed=${changed} alreadyMounted=${alreadyMounted} noDirectLeaf=${skippedNoLeaf}`);
