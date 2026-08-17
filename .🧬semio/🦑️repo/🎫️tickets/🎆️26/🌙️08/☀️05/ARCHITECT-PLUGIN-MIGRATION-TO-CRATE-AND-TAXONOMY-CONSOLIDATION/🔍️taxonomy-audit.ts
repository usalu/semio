#!/usr/bin/env bun
// 🌳️ Updated to Shape V2 (ticket 26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST): the entry file lives at
// <plugin>/📦️packages/🦀️rust/📦️lib.rs and its #[path] leaves resolve relative to THAT dir, not the plugin root.
/** 🔍️ Standalone mirror of `📇️registry/📜️script.ts`'s `validateTaxonomyTree`, so the pilot can audit
 * 🌊️flow's tree while the real `check` still bails early on registry staleness (regeneration is
 * registrar work, not the migrating agent's). Delete once the registrar has regenerated the registry
 * and `bun …/📇️registry/📜️script.ts check` reports the taxonomy audit itself. */
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";

const TAXONOMY_ARTIFACT_COMPONENTS = ["🔺️diff", "🗣️dsl", "🎒️pack", "🔧️op", "📡️spr"] as const;
const TAXONOMY_WINDOW_CHILDREN = new Set(["🍱️panes", "🪀️widgets", "🪛️utilities", "🎬️actions", "🎚️options"]);
const TAXONOMY_LEAF_FILENAME = "🦀️component.rs";
const APPS_DIRNAME = "🎛️apps";

function listDirs(dir: string): string[] {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((name) => statSync(join(dir, name)).isDirectory());
}

function validateTaxonomyTree(pluginRoot: string, pluginId: string): string[] {
  const findings: string[] = [];

  const artifactsDir = join(pluginRoot, "🗿️artifacts");
  for (const artifact of listDirs(artifactsDir)) {
    for (const component of TAXONOMY_ARTIFACT_COMPONENTS) {
      if (!existsSync(join(artifactsDir, artifact, component, TAXONOMY_LEAF_FILENAME))) {
        findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
      }
    }
  }

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

  const componentFiles: string[] = [];
  const taxonomyLeafParents = new Set<string>([...TAXONOMY_ARTIFACT_COMPONENTS, ...TAXONOMY_WINDOW_CHILDREN]);
  function walkPluginTree(dir: string): void {
    for (const name of readdirSync(dir)) {
      if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
      const path = join(dir, name);
      if (statSync(path).isDirectory()) {
        walkPluginTree(path);
        continue;
      }
      if (!name.endsWith(".rs")) continue;
      if (name === TAXONOMY_LEAF_FILENAME) componentFiles.push(path);
      else if (taxonomyLeafParents.has(dir.split("/").pop() ?? "")) findings.push(`${pluginId}: taxonomy leaf file must be named ${TAXONOMY_LEAF_FILENAME}, found ${relative(pluginRoot, path)}`);
    }
  }
  walkPluginTree(pluginRoot);

  const libRsDir = join(pluginRoot, "📦️packages", "🦀️rust");
  const libRsPath = join(libRsDir, "📦️lib.rs");
  if (existsSync(libRsPath)) {
    const libText = readFileSync(libRsPath, "utf8");
    const declaredPaths = [...libText.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)].map((m) => m[1]);
    const declaredAbs = new Set(declaredPaths.map((p) => join(libRsDir, p)));
    for (const file of componentFiles) if (!declaredAbs.has(file)) findings.push(`${pluginId}: ${relative(pluginRoot, file)} is not declared by any #[path] in 📦️lib.rs`);
    for (const p of declaredPaths) if (p.endsWith(".rs") && !existsSync(join(libRsDir, p))) findings.push(`${pluginId}: 📦️lib.rs declares #[path = "${p}"] but the file does not exist on disk`);
  } else {
    findings.push(`${pluginId}: missing 📦️lib.rs at 📦️packages/🦀️rust (Shape V2 entryLocation="packages")`);
  }

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

  return findings;
}

const repoRoot = process.argv[2] ?? process.cwd();
const pluginId = process.argv[3] ?? "🌊️flow";
const findings = validateTaxonomyTree(join(repoRoot, "✏️s", "🔌️plugins", pluginId), pluginId);
if (findings.length === 0) console.log(`taxonomy tree clean: ${pluginId}`);
else {
  for (const finding of findings) console.error(`  - ${finding}`);
  process.exit(1);
}
