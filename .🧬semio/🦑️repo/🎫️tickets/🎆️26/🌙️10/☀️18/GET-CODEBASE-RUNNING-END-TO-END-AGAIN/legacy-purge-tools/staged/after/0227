import { join } from "node:path";
import { existsSync, readFileSync } from "node:fs";
import { type PolicyCrateRef, policyExtractFnBody, policyNewSurfacePriority } from "../../🏛️foundation/🟦️.ts";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { loadTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { taxonomyEcosystemEntryFilenames } from "../../../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";
import { discoverPackages } from "../../../../🔍️discovery/🔭️repository-scan/🟦️.ts";

/**
 * 📏️Anti-inlining tripwire (Single-File-Repo hazard ruling, master ticket): a migrated package's entry
 * configured Rust package entry must stay wiring-only (`#[path]` mod declarations + `plugin_exports!(plugin::plugin)`) — no
 * non-trivial `fn`/`impl` body content beyond `taxonomy.libWiringLineBudget`. Catches the exact
 * regression this repo has hit twice before: an agent following the (now-scoped) "single file repo" goal
 * inlining split `#[path]` modules back into the package entry. Plugin identity lives at the plugin root via
 * `Plugin::builder`, never `semio_plugin!{}`.
 */
const POLICY_FN_OR_IMPL_OPEN_RE = /^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn\s+\w+\s*(?:<[^>]*>)?\s*\([^;{]*\)[^;{]*|impl(?:<[^>]*>)?\s+[^;{]+)\{\s*$/;

export function policyTaxonomyLibShapeBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const lineBudget = loadTaxonomy().libWiringLineBudget;
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    const abs = join(repoRoot, crate.libRelPath);
    if (!existsSync(abs)) continue;
    const content = readFileSync(abs, "utf8");
    const lines = content.split(/\r?\n/);
    const lineStarts: number[] = [];
    for (let offset = 0, li = 0; li < lines.length; li++) {
      lineStarts.push(offset);
      offset += lines[li]!.length + 1;
    }

    let bodyLines = 0;
    let coveredUntilLine = 0; // 🌱️ skip fn/impl bodies nested inside an already-counted outer body — avoids double-counting a fn declared inside an impl block.
    lines.forEach((line, i) => {
      const lineNo = i + 1;
      if (lineNo <= coveredUntilLine || !POLICY_FN_OR_IMPL_OPEN_RE.test(line)) return;
      const body = policyExtractFnBody(content, lineStarts[i]!);
      const bodyLineList = body.split(/\r?\n/);
      bodyLines += bodyLineList.filter((l) => l.trim() && !l.trim().startsWith("//")).length;
      coveredUntilLine = lineNo + bodyLineList.length - 1;
    });
    if (bodyLines <= lineBudget) continue;
    breaches.push({
      id: `taxonomy-lib-shape-${crate.libRelPath}`,
      summary: `"${crate.libRelPath}" has ~${bodyLines} lines of fn/impl body content — a migrated package entry must stay wiring-only`,
      kind: "taxonomy/lib-shape",
      scope: crate.pluginId || crate.ownerRel,
      priority: policyNewSurfacePriority(crate, "medium"),
      reason: "Single-File-Repo hazard ruling: a taxonomy package entry is #[path] mod wiring + plugin_exports!(plugin::plugin) only — real logic lives in taxonomy component files at the plugin root, never inlined back.",
      solution: `Move the non-trivial fn/impl bodies out of ${crate.libRelPath} into their owning taxonomy component file(s) at the plugin root; the package entry should only declare "#[path = \"...\"] mod ...;" and call plugin_exports!(plugin::plugin).`,
    });
  }
  return breaches;
}

/**
 * 📏️TS analogue of `policyTaxonomyLibShapeBreaches`, added by ticket
 * `26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE` (Single-File-Repo hazard ruling, scope note 2): a
 * `"taxonomy"`-area TypeScript package's configured entry file must stay a wiring-only re-export barrel — counts non-import/export/comment/blank lines
 * and flags a breach past `libWiringLineBudget`. Warn-only (`priority: "medium"`) and still vacuous
 * today: the graduated area state is `"clean"` and no area has graduated yet (that flip is the W6
 * activation step of that ticket). The literal it compares against used to be `"taxonomy"`, a value that
 * is not in `taxonomy.areaStates` at all and therefore could never match — fixed here to `"clean"` while
 * mechanism step M4 was in this region (see `🔣️taxonomy.json`'s `_areaStateComment`).
 */
const POLICY_BARREL_GRADUATED_AREA_STATE = "clean";

const POLICY_BARREL_WIRING_LINE_RE = /^\s*(\/\/.*)?$|^\s*export\s+(type\s+)?(\*|\{[^}]*\})\s+from\s+"[^"]+";?\s*(\/\/.*)?$|^\s*import\s+.+\s+from\s+"[^"]+";?\s*(\/\/.*)?$/;

export function policyTaxonomyBarrelShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const entryFilename = taxonomyEcosystemEntryFilenames(taxonomy, "🟦️typescript")[0];
  if (!entryFilename) return [];
  const breaches: BreachRecord[] = [];
  for (const pkg of discoverPackages(repoRoot, taxonomy)) {
    if (pkg.lang !== "🟦️typescript" || pkg.area !== POLICY_BARREL_GRADUATED_AREA_STATE) continue;
    const entryAbs = join(repoRoot, pkg.packageRel, entryFilename);
    if (!existsSync(entryAbs)) continue;
    const lines = readFileSync(entryAbs, "utf8").split(/\r?\n/);
    const nonWiring = lines.filter((l) => l.trim() && !POLICY_BARREL_WIRING_LINE_RE.test(l));
    if (nonWiring.length <= taxonomy.libWiringLineBudget) continue;
    breaches.push({
      id: `taxonomy-barrel-shape-${pkg.manifestPath}`,
      summary: `"${pkg.packageRel}/${entryFilename}" has ~${nonWiring.length} non-wiring lines — a taxonomy package's entry file must stay a re-export barrel`,
      kind: "taxonomy/barrel-shape",
      scope: pkg.id,
      priority: "medium",
      reason: "Single-File-Repo hazard ruling (TS extension, ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE): a taxonomy package's entry barrel is import/export wiring only — real component logic lives in 🧱️elements/<Element>/ leaf files, never inlined back.",
      solution: `Move the non-wiring content out of ${entryFilename} into its owning 🧱️elements/<Element>/ leaf file(s); the entry file should only import/re-export.`,
    });
  }
  return breaches;
}
