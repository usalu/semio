#!/usr/bin/env bun
/** @emoji 📁️ Second pass: rewrite references missed due to multi-dot extensions. */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

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

const SKIP_PATH_PARTS = ["/node_modules/", "/.git/", "/target/", "/dist/", "/.venv/", "/temp/", "/.nx/", "/storybook-static/", "/.🦑️repo/🎫️tickets/"];

const PACKAGE_REPLACEMENTS: [string, string][] = [
  ["@semio-tech/puzzle-asset", "@semio-tech/puzzle-assets"],
  ["@semio-tech/remodel-image", "@semio-tech/remodel-images"],
  ["@semio-tech/asset", "@semio-tech/assets"],
  ["@semio-tech/icon", "@semio-tech/icons"],
  ["@semio-tech/image", "@semio-tech/images"],
  ["@semio-tech/logo", "@semio-tech/logos"],
];

function shouldSkipPath(path: string): boolean {
  return SKIP_PATH_PARTS.some((part) => path.includes(part));
}

function isTextFile(name: string): boolean {
  if (name === "launch.json" || name === ".gitignore" || name === ".dependency-cruiser.cjs") return true;
  if (name.startsWith("post-") || name === "prepare-commit-msg") return true;
  return /\.(ts|tsx|js|jsx|json|md|mdx|go|py|cs|rs|toml|yaml|yml|html|css|scss|svg|mjs|cjs|sh|ps1|mod|work|cypher|wit|launch)$/i.test(name);
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
    if (isTextFile(entry) || entry === "Cargo.toml" || entry === "go.work") out.push(full);
  }
}

function escapeRegExp(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
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

function rewriteReferences(): number {
  const files: string[] = [];
  collectFiles(root, files);
  const extra = [join(root, ".dependency-cruiser.cjs"), join(root, "combos.txt")];
  for (const f of extra) if (existsSync(f)) files.push(f);
  let changed = 0;
  const seen = new Set<string>();
  for (const file of files) {
    if (seen.has(file)) continue;
    seen.add(file);
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
    next = next.replaceAll("✏️s\\🔌️plugin\\", "✏️s\\🔌️plugins\\");
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

const updated = rewriteReferences();
console.log(`[DEBUG] second pass updated ${updated} files`);
