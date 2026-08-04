#!/usr/bin/env bun
/** @emoji 📁️ Pluralizes taxonomy container directory names and rewrites active path references. */
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  renameSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { basename, dirname, join, relative } from "node:path";

const root = "/Users/ueli/Documents/semio";

const SINGULAR_TO_PLURAL: Record<string, string> = {
  "⚡️implementation": "⚡️implementations",
  "🔨️module": "🔨️modules",
  "📚️example": "📚️examples",
  "🎛️app": "🎛️apps",
  "🖼️asset": "🖼️assets",
  "🧩️extension": "🧩️extensions",
  "🗂️typology": "🗂️typologies",
  "🎬️action": "🎬️actions",
  "🎬️interaction": "🎬️interactions",
  "🔀️transformation": "🔀️transformations",
  "🔣️icon": "🔣️icons",
  "🧫️fixture": "🧫️fixtures",
  "🪧️logo": "🪧️logos",
  "🖼️image": "🖼️images",
  "🏷️attributeDefinition": "🏷️attributeDefinitions",
  "📊️statDefinition": "📊️statDefinitions",
  "🔤️font": "🔤️fonts",
  "🔌️port": "🔌️ports",
  "🛍️product": "🛍️products",
  "🏗️modelDefinition": "🏗️modelDefinitions",
  "🔧️propertyDefinition": "🔧️propertyDefinitions",
  "🏷️propertyKind": "🏷️propertyKinds",
  "🪝️hook": "🪝️hooks",
};

const SKIP_DIR_NAMES = new Set([
  "node_modules",
  ".git",
  "target",
  "dist",
  "pkg",
  "out",
  "deps",
  "debug",
  "release",
  "storybook-static",
  "temp",
  ".nx",
  ".venv",
  "🤖️generated",
]);

const SKIP_PATH_PARTS = [
  "/node_modules/",
  "/.git/",
  "/target/",
  "/dist/",
  "/.venv/",
  "/temp/",
  "/.nx/",
  "/storybook-static/",
  "/.🦑️repo/🎫️tickets/",
];

const SKIP_RENAME_UNDER_PREFIXES = ["compose/", "♻️mit-bestand/"];

const TEXT_EXTENSIONS = new Set([
  ".ts",
  ".tsx",
  ".js",
  ".jsx",
  ".json",
  ".md",
  ".mdx",
  ".go",
  ".py",
  ".cs",
  ".rs",
  ".toml",
  ".yaml",
  ".yml",
  ".html",
  ".css",
  ".scss",
  ".svg",
  ".mjs",
  ".cjs",
  ".sh",
  ".ps1",
  ".mod",
  ".work",
  ".gitignore",
  ".cypher",
  ".wit",
  ".launch",
]);

const PACKAGE_REPLACEMENTS: [string, string][] = [
  ["@semio-tech/puzzle-asset", "@semio-tech/puzzle-assets"],
  ["@semio-tech/remodel-image", "@semio-tech/remodel-images"],
  ["@semio-tech/asset", "@semio-tech/assets"],
  ["@semio-tech/icon", "@semio-tech/icons"],
  ["@semio-tech/image", "@semio-tech/images"],
  ["@semio-tech/logo", "@semio-tech/logos"],
];

function shouldSkipPath(path: string): boolean {
  const rel = path.startsWith(root) ? path.slice(root.length + 1) : path;
  if (SKIP_PATH_PARTS.some((part) => path.includes(part))) return true;
  if (SKIP_RENAME_UNDER_PREFIXES.some((p) => rel.startsWith(p))) return true;
  return false;
}

function consolidateRepoHooks(): void {
  const repoRoot = join(root, "🧰️framework/🛍️product/🦑️repo");
  const hook = join(repoRoot, "🪝️hook");
  const hooks = join(repoRoot, "🪝️hooks");
  if (!existsSync(hook)) return;
  mkdirSync(hooks, { recursive: true });
  for (const name of readdirSync(hook)) {
    copyFileSync(join(hook, name), join(hooks, name));
  }
  rmSync(hook, { recursive: true, force: true });
  console.log("[DEBUG] consolidated 🪝️hook -> 🪝️hooks");
}

function shouldRenameDir(dir: string, base: string): string | null {
  if (base === "🔌️plugin") {
    const parent = basename(dirname(dir));
    if (parent !== "✏️s") return null;
    return "🔌️plugins";
  }
  const plural = SINGULAR_TO_PLURAL[base];
  if (!plural || plural === base) return null;
  if (base === "🪝️hook") return null;
  return plural;
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
}

function renameDirs(): string[] {
  const allDirs: string[] = [];
  collectDirs(root, allDirs);
  const renames: { from: string; to: string }[] = [];
  for (const dir of allDirs) {
    const base = basename(dir);
    const plural = shouldRenameDir(dir, base);
    if (!plural) continue;
    const parent = dir.slice(0, dir.length - base.length);
    const target = `${parent}${plural}`;
    if (existsSync(target) && plural !== "🪝️hooks") {
      const entries = readdirSync(target);
      if (entries.length === 0) rmSync(target, { recursive: true });
    }
    if (existsSync(target) && plural !== "🧫️fixtures" && plural !== "🪝️hooks") continue;
    renames.push({ from: dir, to: target });
  }
  renames.sort((a, b) => b.from.split("/").length - a.from.split("/").length);
  const done: string[] = [];
  for (const { from, to } of renames) {
    if (!existsSync(from)) continue;
    if (existsSync(to)) {
      console.log(`[DEBUG] skip rename conflict ${relative(root, from)}`);
      continue;
    }
    renameSync(from, to);
    done.push(`${relative(root, from)} -> ${relative(root, to)}`);
  }
  const pluginsFrom = join(root, "✏️s/🔌️plugin");
  const pluginsTo = join(root, "✏️s/🔌️plugins");
  if (existsSync(pluginsFrom) && !existsSync(pluginsTo)) {
    renameSync(pluginsFrom, pluginsTo);
    done.push("✏️s/🔌️plugin -> ✏️s/🔌️plugins");
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
    const dot = entry.indexOf(".");
    const ext = dot >= 0 ? entry.slice(dot) : "";
    if (TEXT_EXTENSIONS.has(ext) || entry === "launch.json" || entry === "prepare-commit-msg" || entry === "post-commit") {
      out.push(full);
    }
  }
}

function pluralizeTokenInText(text: string, singular: string, plural: string): string {
  const forms = [
    [singular, plural],
    [singular.replaceAll("/", "\\/"), plural.replaceAll("/", "\\/")],
    [singular.replaceAll("/", "\\\\"), plural.replaceAll("/", "\\\\")],
  ] as const;
  let next = text;
  for (const [from, to] of forms) {
    const re = new RegExp(`${escapeRegExp(from)}(?!s)(?![A-Za-z0-9_-])`, "g");
    next = next.replace(re, to);
  }
  return next;
}

function escapeRegExp(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function rewriteReferences(): number {
  const files: string[] = [];
  collectFiles(root, files);
  let changed = 0;
  for (const file of files) {
    let original: string;
    try {
      original = readFileSync(file, "utf8");
    } catch {
      continue;
    }
    let next = original;
    next = next.replaceAll("✏️s/🔌️plugin/", "✏️s/🔌️plugins/");
    next = next.replaceAll("✏️s\\/🔌️plugin\\/", "✏️s\\/🔌️plugins\\/");
    next = next.replaceAll('✏️s", "🔌️plugin"', '✏️s", "🔌️plugins"');
    next = next.replaceAll('✏️s\\", \\"🔌️plugin\\"', '✏️s\\", \\"🔌️plugins\\"');
    next = next.replaceAll("✏️s\\🔌️plugin\\", "✏️s\\🔌️plugins\\");
    next = next.replaceAll("✏️s\\\\🔌️plugin\\\\", "✏️s\\\\🔌️plugins\\\\");
    for (const [singular, plural] of Object.entries(SINGULAR_TO_PLURAL)) {
      next = pluralizeTokenInText(next, singular, plural);
    }
    for (const [from, to] of PACKAGE_REPLACEMENTS) {
      next = next.split(from).join(to);
    }
    if (next !== original) {
      writeFileSync(file, next);
      changed++;
    }
  }
  return changed;
}

consolidateRepoHooks();
const renamed = renameDirs();
const updated = rewriteReferences();
console.log(`[DEBUG] renamed ${renamed.length} directories`);
for (const line of renamed.slice(0, 30)) console.log(`  ${line}`);
if (renamed.length > 30) console.log(`  ... and ${renamed.length - 30} more`);
console.log(`[DEBUG] updated ${updated} files`);
