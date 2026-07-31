#!/usr/bin/env bun
/** @emoji 📁️ Renames plural directory names to singular and updates path references. */
import { existsSync, readdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";

const root = "/Users/ueli/Documents/compose";

const PLURAL_TO_SINGULAR: Record<string, string> = {
  agents: "agent",
  algorithms: "algorithm",
  assets: "asset",
  badges: "badge",
  benchmarks: "benchmark",
  breaches: "breach",
  cursors: "cursor",
  designs: "design",
  diagrams: "diagram",
  docs: "doc",
  events: "event",
  examples: "example",
  fixtures: "fixture",
  fonts: "font",
  hooks: "hook",
  icons: "icon",
  images: "image",
  integrations: "integration",
  iterations: "iteration",
  lists: "list",
  manuals: "manual",
  meshes: "mesh",
  modules: "module",
  pages: "page",
  plans: "plan",
  representations: "representation",
  runs: "run",
  scopes: "scope",
  settings: "setting",
  showcases: "showcase",
  skills: "skill",
  starters: "starter",
  stores: "store",
  stories: "story",
  targets: "target",
  textures: "texture",
  tickets: "ticket",
  tools: "tool",
  tutorials: "tutorial",
  types: "type",
  voxels: "voxel",
  warnings: "warning",
  widgets: "widget",
};

const SKIP_DIR_NAMES = new Set(["node_modules", ".git", ".repo", ".venv", "temp", ".nx", "dist", "target", "pkg", "deps", "generated", "debug", "release"]);

const SKIP_PATH_PARTS = ["/.repo/", "/node_modules/", "/.venv/", "/temp/", "/.nx/", "/dist/", "/target/"];

const KEEP_PLURAL_PATHS = [
  "/.github/workflows",
  "/.cursor/plans",
  "/.cursor/agents",
  "/.copilot/plans",
  "/.codex/plans",
  "/.github/agents",
  "/.github/hooks",
  "/.kiro/agents",
  "/.kiro/settings",
  "/.agents/skills",
  "/.storybook/stories",
  "/coda/.agent/skills",
];

const TEXT_EXTENSIONS = new Set([".ts", ".tsx", ".js", ".jsx", ".json", ".md", ".mdx", ".go", ".py", ".cs", ".rs", ".toml", ".yaml", ".yml", ".html", ".css", ".scss", ".svg", ".launch"]);

const TEXT_REPLACEMENTS: [string, string][] = [
  ["@semio-tech/semio-assets", "@semio-tech/semio-asset"],
  ["@semio-tech/compose-fixtures", "@semio-tech/compose-fixture"],
  ["@semio-tech/puzzle-assets", "@semio-tech/puzzle-asset"],
  ["@semio-tech/ui-assets", "@semio-tech/ui-asset"],
  ['"bundleKind": "assets"', '"bundleKind": "asset"'],
  ['"bundleKind": "fixtures"', '"bundleKind": "fixture"'],
  ["  kind: assets", "  kind: asset"],
  ["  kind: fixtures", "  kind: fixture"],
  ["  name: assets", "  name: asset"],
  ["  name: fixtures", "  name: fixture"],
  ["/representations/", "/representation/"],
  ["/integrations/", "/integration/"],
  ["/algorithms/", "/algorithm/"],
  ["/benchmarks/", "/benchmark/"],
  ["/fixtures/", "/fixture/"],
  ["/assets/", "/asset/"],
  ["/icons/", "/icon/"],
  ["/images/", "/image/"],
  ["/lists/", "/list/"],
  ["/badges/", "/badge/"],
  ["/fonts/", "/font/"],
  ["/cursors/", "/cursor/"],
  ["/types/", "/type/"],
  ["/designs/", "/design/"],
  ["/docs/", "/doc/"],
  ["/examples/", "/example/"],
  ["/stores/", "/store/"],
  ["/diagrams/", "/diagram/"],
  ["/textures/", "/texture/"],
  ["/stories/", "/story/"],
  ["/pages/", "/page/"],
  ["/hooks/", "/hook/"],
  ["/modules/", "/module/"],
  ["/widgets/", "/widget/"],
  ["/tutorials/", "/tutorial/"],
  ["/manuals/", "/manual/"],
  ["/showcases/", "/showcase/"],
  ["/starters/", "/starter/"],
  ["/voxels/", "/voxel/"],
  ["/tools/", "/tool/"],
  ["/standards/", "/standard/"],
  ["/events/", "/event/"],
  ["/warnings/", "/warning/"],
  ["/tickets/", "/ticket/"],
  ["/scopes/", "/scope/"],
  ["/breaches/", "/breach/"],
  ["/targets/", "/target/"],
  ["/iterations/", "/iteration/"],
  ["/runs/", "/run/"],
  ["/agents/", "/agent/"],
  ["/settings/", "/setting/"],
  ["/plans/", "/plan/"],
  ["/skills/", "/skill/"],
  ["/meshes/", "/mesh/"],
  ["compose/fixtures", "compose/fixture"],
  ["assets", "asset"],
  ["puzzle/assets", "puzzle/asset"],
  ["cad/assets", "cad/asset"],
  ["ui/assets", "ui/asset"],
  ["repo/assets", "repo/asset"],
  ["flow/modules", "flow/module"],
  [".storybook/fixtures", ".storybook/fixture"],
  [".storybook/stories", ".storybook/story"],
  ["workspace:ui/assets", "workspace:ui/asset"],
  ["workspace:assets", "workspace:asset"],
  ["workspace:compose/fixtures", "workspace:compose/fixture"],
  ["workspace:puzzle/assets", "workspace:puzzle/asset"],
];

function shouldSkipPath(path: string): boolean {
  if (KEEP_PLURAL_PATHS.some((keep) => path.includes(keep))) return true;
  return SKIP_PATH_PARTS.some((part) => path.includes(part));
}

function collectDirs(dir: string, out: string[]): void {
  if (shouldSkipPath(dir)) return;
  let entries: string[];
  try {
    entries = readdirSync(dir);
  } catch {
    return;
  }
  for (const entry of entries) {
    if (SKIP_DIR_NAMES.has(entry)) continue;
    const full = join(dir, entry);
    if (shouldSkipPath(full)) continue;
    try {
      if (!statSync(full).isDirectory()) continue;
    } catch {
      continue;
    }
    out.push(full);
    collectDirs(full, out);
  }
  return;
}

function renameDirs(): string[] {
  const allDirs: string[] = [];
  collectDirs(root, allDirs);
  const renames: { from: string; to: string }[] = [];
  for (const dir of allDirs) {
    const base = dir.split("/").pop() ?? "";
    const singular = PLURAL_TO_SINGULAR[base];
    if (!singular || singular === base) continue;
    const parent = dir.slice(0, dir.length - base.length);
    const target = `${parent}${singular}`;
    if (existsSync(target)) {
      const targetEmpty = readdirSync(target).length === 0;
      if (targetEmpty) rmSync(target, { recursive: true });
    }
    if (existsSync(target)) continue;
    renames.push({ from: dir, to: target });
  }
  renames.sort((a, b) => b.from.split("/").length - a.from.split("/").length);
  const done: string[] = [];
  for (const { from, to } of renames) {
    if (!existsSync(from)) continue;
    if (existsSync(to)) {
      console.log(`[DEBUG] skip conflict ${relative(root, from)} -> ${relative(root, to)}`);
      continue;
    }
    renameSync(from, to);
    done.push(`${relative(root, from)} -> ${relative(root, to)}`);
  }
  return done;
}

function collectFiles(dir: string, out: string[]): void {
  if (shouldSkipPath(dir)) return;
  let entries: string[];
  try {
    entries = readdirSync(dir);
  } catch {
    return;
  }
  for (const entry of entries) {
    if (SKIP_DIR_NAMES.has(entry)) continue;
    const full = join(dir, entry);
    if (shouldSkipPath(full)) continue;
    let st;
    try {
      st = statSync(full);
    } catch {
      continue;
    }
    if (st.isDirectory()) {
      collectFiles(full, out);
      continue;
    }
    const ext = entry.includes(".") ? entry.slice(entry.lastIndexOf(".")) : "";
    if (TEXT_EXTENSIONS.has(ext) || entry === "launch.json") out.push(full);
  }
}

function updateReferences(): number {
  const files: string[] = [];
  collectFiles(root, files);
  let changed = 0;
  for (const file of files) {
    const original = readFileSync(file, "utf8");
    let next = original;
    for (const [from, to] of TEXT_REPLACEMENTS) next = next.split(from).join(to);
    if (next !== original) {
      writeFileSync(file, next);
      changed++;
    }
  }
  return changed;
}

const renamed = renameDirs();
const updated = updateReferences();
console.log(`[DEBUG] renamed ${renamed.length} directories`);
for (const line of renamed) console.log(`  ${line}`);
console.log(`[DEBUG] updated ${updated} files`);
