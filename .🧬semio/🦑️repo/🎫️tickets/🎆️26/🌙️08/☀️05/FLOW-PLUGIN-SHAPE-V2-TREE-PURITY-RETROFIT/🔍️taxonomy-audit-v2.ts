#!/usr/bin/env bun
/** 🔍️ Shape V2 tree-purity mirror of `📇️registry/📜️script.ts`'s `validateTaxonomyTree`, adapted from the
 * W1 pilot's V1 audit (`26/08/05/FLOW-PLUGIN-PILOT-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🔍️taxonomy-audit.ts`)
 * for the amended contract: entry file lives at `<owner>/📦️packages/<lang>/📦️lib.rs`, not the owner root,
 * and nothing besides `component.<ext>` files / `📦️packages` / plain folders may exist below the owner
 * root. Delete once the registrar/root `check` reports this natively. Ticket
 * 26/08/05/FLOW-PLUGIN-SHAPE-V2-TREE-PURITY-RETROFIT. */
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";

const TAXONOMY_ARTIFACT_COMPONENTS = ["🔺️diff", "🗣️dsl", "🎒️pack", "🔧️op", "📡️spr"] as const;
const TAXONOMY_WINDOW_CHILDREN = new Set(["🍱️panes", "🪀️widgets", "🪛️utilities", "🎬️actions", "🎚️options"]);
const TAXONOMY_LEAF_FILENAME = "🦀️component.rs";
const APPS_DIRNAME = "🎛️apps";
const PACKAGES_DIRNAME = "📦️packages";
const OUT_OF_SCOPE_DIRS = new Set(["🧩️extensions", "🔨️modules"]); // untouched by design, not V2 violations
const ROOT_DATA_DIR_NAMES = new Set(["📚️examples", "🧫️fixtures", "🤖️generated", "🧫️examples", "🖼️assets", "📇️registry"]);
const ROOT_DATA_FILE_NAMES = new Set(["🛂️manifest.json"]);
const ROOT_DOC_FILE_NAMES = new Set(["AGENTS.md", "README.md"]);

function listDirs(dir: string): string[] {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((name) => statSync(join(dir, name)).isDirectory());
}

function validateTaxonomyTreeV2(pluginRoot: string, pluginId: string): string[] {
  const findings: string[] = [];

  //#region 🗿️ArtifactCompleteness
  const artifactsDir = join(pluginRoot, "🗿️artifacts");
  for (const artifact of listDirs(artifactsDir)) {
    for (const component of TAXONOMY_ARTIFACT_COMPONENTS) {
      if (!existsSync(join(artifactsDir, artifact, component, TAXONOMY_LEAF_FILENAME))) {
        findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
      }
    }
  }
  //#endregion 🗿️ArtifactCompleteness

  //#region 🪟️WindowChildren
  const appsDir = join(pluginRoot, APPS_DIRNAME);
  for (const app of listDirs(appsDir)) {
    const modesDir = join(appsDir, app, "🎭️modes");
    for (const mode of listDirs(modesDir)) {
      const windowsDir = join(modesDir, mode, "🪟️windows");
      for (const w of listDirs(windowsDir)) {
        for (const child of listDirs(join(windowsDir, w))) {
          if (!TAXONOMY_WINDOW_CHILDREN.has(child)) {
            findings.push(`${pluginId}: window "${app}/${mode}/${w}" has unexpected child "${child}"`);
          }
        }
      }
    }
  }
  //#endregion 🪟️WindowChildren

  //#region 🌳️TreePurityBelowOwnerRoot
  // Every entry directly under pluginRoot must be: 📦️packages, an out-of-scope dir (extensions/modules),
  // a root data dir/file, a root doc file, or a plain taxonomy folder (🗿️artifacts/🎛️apps/…).
  const componentFiles: string[] = [];
  const taxonomyLeafParents = new Set<string>([...TAXONOMY_ARTIFACT_COMPONENTS, ...TAXONOMY_WINDOW_CHILDREN, "🎚️config", "🗣️terminology"]);
  function walkPluginTree(dir: string, isRoot: boolean): void {
    for (const name of readdirSync(dir)) {
      if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
      const path = join(dir, name);
      const isDir = statSync(path).isDirectory();
      if (isRoot) {
        if (isDir && (name === PACKAGES_DIRNAME || OUT_OF_SCOPE_DIRS.has(name))) continue; // skip packages/out-of-scope subtrees entirely
        if (!isDir && ROOT_DATA_FILE_NAMES.has(name)) continue;
        if (!isDir && ROOT_DOC_FILE_NAMES.has(name)) continue;
        if (isDir && ROOT_DATA_DIR_NAMES.has(name)) continue; // valid at root, don't walk for .rs leaf checks
      }
      if (isDir) {
        walkPluginTree(path, false);
        continue;
      }
      if (!name.endsWith(".rs")) {
        if (isRoot) findings.push(`${pluginId}: unexpected file at owner root: ${relative(pluginRoot, path)}`);
        continue;
      }
      if (name === TAXONOMY_LEAF_FILENAME) componentFiles.push(path);
      else if (taxonomyLeafParents.has(dir.split("/").pop() ?? "") || true) {
        // any non-component.rs .rs file below the owner root (outside packages/out-of-scope) is a V2 violation
        findings.push(`${pluginId}: V2 violation — sibling variant / stray .rs file (must fold into <folder>/${TAXONOMY_LEAF_FILENAME}): ${relative(pluginRoot, path)}`);
      }
    }
  }
  walkPluginTree(pluginRoot, true);
  //#endregion 🌳️TreePurityBelowOwnerRoot

  //#region 📦️EntryFileLocationAndPathWiring
  const libRsPath = join(pluginRoot, PACKAGES_DIRNAME, "🦀️rust", "📦️lib.rs");
  const libDir = join(pluginRoot, PACKAGES_DIRNAME, "🦀️rust");
  if (existsSync(libRsPath)) {
    const libText = readFileSync(libRsPath, "utf8");
    const declaredPaths = [...libText.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)].map((m) => m[1]).filter((p) => p !== ".");
    const declaredAbs = new Set(declaredPaths.map((p) => join(libDir, p)));
    for (const file of componentFiles) if (!declaredAbs.has(file)) findings.push(`${pluginId}: ${relative(pluginRoot, file)} is not declared by any #[path] in 📦️packages/🦀️rust/📦️lib.rs`);
    for (const p of declaredPaths) if (p.endsWith(".rs") && !existsSync(join(libDir, p))) findings.push(`${pluginId}: 📦️lib.rs declares #[path = "${p}"] but the file does not exist on disk (resolved: ${join(libDir, p)})`);
    for (const p of declaredPaths) if (!p.startsWith("../../")) findings.push(`${pluginId}: 📦️lib.rs leaf #[path = "${p}"] is missing the required ../../ prefix (V2: entry file now sits 2 levels deeper)`);
  } else {
    findings.push(`${pluginId}: missing 📦️lib.rs at 📦️packages/🦀️rust/ (V2 entry location)`);
  }
  if (existsSync(join(pluginRoot, "📦️lib.rs"))) findings.push(`${pluginId}: 📦️lib.rs still present at the OWNER ROOT — should have moved into 📦️packages/🦀️rust/`);
  //#endregion 📦️EntryFileLocationAndPathWiring

  //#region 📡️ProtocolRename
  function containsProtocolSegment(dir: string): boolean {
    for (const name of readdirSync(dir)) {
      if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
      const path = join(dir, name);
      if (!statSync(path).isDirectory()) continue;
      if (name === "📡️protocol" || containsProtocolSegment(path)) return true;
    }
    return false;
  }
  if (containsProtocolSegment(pluginRoot)) findings.push(`${pluginId}: found a "📡️protocol" path segment under the plugin dir (renamed to 📡️spr)`);
  //#endregion 📡️ProtocolRename

  return findings;
}

const repoRoot = process.argv[2] ?? process.cwd();
const pluginId = process.argv[3] ?? "🌊️flow";
const findings = validateTaxonomyTreeV2(join(repoRoot, "✏️s", "🔌️plugins", pluginId), pluginId);
if (findings.length === 0) console.log(`Shape V2 tree purity clean: ${pluginId}`);
else {
  for (const finding of findings) console.error(`  - ${finding}`);
  process.exit(1);
}
