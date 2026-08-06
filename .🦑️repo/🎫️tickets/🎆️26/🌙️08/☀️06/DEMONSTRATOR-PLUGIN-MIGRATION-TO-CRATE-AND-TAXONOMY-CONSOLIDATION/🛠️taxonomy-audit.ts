#!/usr/bin/env bun
/**
 * 🗿️ Ticket-local mirror of the registry script's `validateTaxonomyTree` + the Shape V2 tree-purity
 * rule, for 🎪️demonstrator only. Exists because the real `registry check` bails on generated-catalog
 * staleness until the registrar regenerates (TEMPLATE.md §9 step 4c).
 *
 * `bun 🛠️taxonomy-audit.ts` from the repo root.
 */
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join, relative, dirname } from "node:path";

const PLUGIN_ROOT = "✏️s/🔌️plugins/🎪️demonstrator";
const LEAF = "🦀️component.rs";
const ENTRY = join(PLUGIN_ROOT, "📦️packages", "🦀️rust", "📦️lib.rs");
const PACKAGES = "📦️packages";
const findings: string[] = [];

const componentFiles: string[] = [];
function walk(dir: string) {
  for (const name of readdirSync(dir)) {
    if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
    const path = join(dir, name);
    if (statSync(path).isDirectory()) {
      walk(path);
      continue;
    }
    if (path.includes(`/${PACKAGES}/`)) continue;
    if (name === LEAF) componentFiles.push(path);
    else findings.push(`tree purity: ${relative(PLUGIN_ROOT, path)} is neither a ${LEAF} leaf nor inside ${PACKAGES}`);
  }
}
walk(PLUGIN_ROOT);

if (!existsSync(ENTRY)) findings.push(`entry file missing at ${ENTRY} (Shape V2 requires it inside ${PACKAGES}/🦀️rust)`);
if (existsSync(join(PLUGIN_ROOT, "📦️lib.rs"))) findings.push("V1 entry file still present at the owner root");

const declared = new Set<string>();
const baseStack = [dirname(ENTRY)];
let pending: string | null = null;
for (const raw of readFileSync(ENTRY, "utf8").split("\n")) {
  const line = raw.trim();
  const p = line.match(/#\[path\s*=\s*"([^"]+)"\]/);
  if (p) {
    pending = p[1]!;
    continue;
  }
  const m = line.match(/^(?:pub\s+)?mod\s+(\w+)\s*(\{|;)/);
  if (m) {
    const target = pending ?? m[1]!;
    const resolved = join(baseStack[baseStack.length - 1]!, target);
    pending = null;
    if (m[2] === ";") {
      if (target.endsWith(".rs")) {
        declared.add(resolved);
        if (!existsSync(resolved)) findings.push(`📦️lib.rs declares #[path = "${target}"] but the file does not exist`);
      }
    } else baseStack.push(resolved);
    continue;
  }
  const net = (line.match(/\}/g) ?? []).length - (line.match(/\{/g) ?? []).length;
  for (let i = 0; i < net; i++) if (baseStack.length > 1) baseStack.pop();
}
for (const file of componentFiles) if (!declared.has(file)) findings.push(`${relative(PLUGIN_ROOT, file)} is not declared by any #[path] in 📦️lib.rs`);

function hasProtocol(dir: string): boolean {
  for (const name of readdirSync(dir)) {
    if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
    const path = join(dir, name);
    if (!statSync(path).isDirectory()) continue;
    if (name === "📡️protocol" || hasProtocol(path)) return true;
  }
  return false;
}
if (hasProtocol(PLUGIN_ROOT)) findings.push('a "📡️protocol" path segment survives (renamed to 📡️spr)');
if (existsSync(join(PLUGIN_ROOT, "⚡️implementations"))) findings.push("forbidden ⚡️implementations segment at the owner root");

console.log(`components on disk: ${componentFiles.length}, declared in 📦️lib.rs: ${declared.size}`);
console.log(findings.length === 0 ? "✅ taxonomy audit clean" : `❌ ${findings.length} finding(s):\n${findings.map((f) => `  - ${f}`).join("\n")}`);
process.exit(findings.length === 0 ? 0 : 1);
