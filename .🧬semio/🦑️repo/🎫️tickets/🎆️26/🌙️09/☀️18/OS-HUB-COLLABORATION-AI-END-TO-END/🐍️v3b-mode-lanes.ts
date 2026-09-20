#!/usr/bin/env bun
/** 🎭️ V3b: gives every surface mode the four lanes the taxonomy requires of it.
 *
 * `validateTaxonomyTree` states the law in its own comment: "a mode declares its windows plus its own
 * 🎚️config / 👥️presence / 🫧️transient lanes; an empty lane is valid (it carries only the tracked
 * marker), an absent lane is not". The marker body is not invented here — it comes from
 * `scaffoldEmptyFacetMarkdown`, the same emitter `bun ./📜️script.ts new surface` uses, so a lane this
 * script writes is byte-identical to one the scaffolder would have written.
 *
 * Usage: `bun 🐍️v3b-mode-lanes.ts [--apply]`
 */
import { existsSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { TAXONOMY_MODE_CHILDREN, WINDOW_EMPTY_FACET_FILENAME } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts";
import { scaffoldEmptyFacetMarkdown } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const apply = process.argv.includes("--apply");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
const modesDirName = "🎭️modes";
const windowsDirName = "🪟️windows";

function listDirs(dir: string): string[] {
  if (!existsSync(dir)) return [];
  return readdirSync(dir, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name);
}

const modeDirs: string[] = [];
function walk(dir: string, depth: number): void {
  if (depth > 12) return;
  for (const name of listDirs(dir)) {
    const path = join(dir, name);
    if (name === modesDirName) {
      for (const mode of listDirs(path)) modeDirs.push(join(path, mode));
      continue;
    }
    if (name === "📦️packages" || name === "🗑️generated" || name === "🤖️generated" || name === "node_modules" || name === "target") continue;
    walk(path, depth + 1);
  }
}
walk(pluginsRoot, 0);
if (modeDirs.length === 0) throw new Error("no 🎭️modes/<mode> directory found — the repair would silently do nothing");

const lanes = TAXONOMY_MODE_CHILDREN.filter((child) => child !== windowsDirName);
if (lanes.length === 0) throw new Error("🔣️taxonomy.json declares no mode child lanes — refusing to guess");

let written = 0;
const touched = new Set<string>();
for (const modeDir of modeDirs.sort()) {
  for (const lane of lanes) {
    const laneDir = join(modeDir, lane);
    if (existsSync(laneDir)) continue;
    const marker = join(laneDir, WINDOW_EMPTY_FACET_FILENAME);
    written += 1;
    touched.add(relative(repoRoot, modeDir).replaceAll("\\", "/"));
    console.log(`${apply ? "create" : "would create"}\t${relative(repoRoot, marker).replaceAll("\\", "/")}`);
    if (!apply) continue;
    mkdirSync(laneDir, { recursive: true });
    writeFileSync(marker, scaffoldEmptyFacetMarkdown(`Mode ${lane}`));
  }
}
console.log(`modes=${modeDirs.length} lanes=${lanes.join(" ")} ${apply ? "created" : "missing"}=${written} across ${touched.size} mode(s)`);
