#!/usr/bin/env bun
/** 🌳️ Shape V2 tree-purity retrofit (TEMPLATE.md §14) for 🏛️architect: folds every sibling variant
 *  `🦀️<topic>.rs` into its own `<emoji-folder>/🦀️component.rs`, moves the crate entry file into
 *  `📦️packages/🦀️rust/`, and rewrites the entry's `#[path]` leaves with the `../../` prefix the deeper
 *  location needs. Pure code motion — module idents are unchanged, so no `use crate::…` site moves.
 *  Scratch tool for ticket `26/08/05/ARCHITECT-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`. */
import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginRoot = join(repoRoot, "✏️s/🔌️plugins/🏛️architect");
const ARTIFACT = "🗿️artifacts/🏛️program";
const APP = "🎛️apps/🏛️architect";

/** 🗺️ old owner-relative sibling file → new owner-relative component-folder file. */
const MOVES: ReadonlyArray<readonly [string, string]> = [
  [`${ARTIFACT}/🦀️kernel.rs`, `${ARTIFACT}/🧱️kernel/🦀️component.rs`],
  [`${ARTIFACT}/🦀️registers.rs`, `${ARTIFACT}/🗄️registers/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️adjacency.rs`, `${ARTIFACT}/⚙️engine/↔️adjacency/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️analyze.rs`, `${ARTIFACT}/⚙️engine/🔬️analyze/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️exchange.rs`, `${ARTIFACT}/⚙️engine/📤️exchange/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️outputs.rs`, `${ARTIFACT}/⚙️engine/🎁️outputs/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️report.rs`, `${ARTIFACT}/⚙️engine/📄️report/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️search.rs`, `${ARTIFACT}/⚙️engine/🔍️search/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️status_summary.rs`, `${ARTIFACT}/⚙️engine/📊️status-summary/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️template.rs`, `${ARTIFACT}/⚙️engine/📐️template/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️trace.rs`, `${ARTIFACT}/⚙️engine/🧭️trace/🦀️component.rs`],
  [`${ARTIFACT}/⚙️engine/🦀️validate.rs`, `${ARTIFACT}/⚙️engine/✅️validate/🦀️component.rs`],
  [`${APP}/🦀️config.rs`, `${APP}/🎚️config/🦀️component.rs`],
  [`${APP}/🦀️chrome.rs`, `${APP}/🎨️chrome/🦀️component.rs`],
  [`${APP}/🦀️catalog.rs`, `${APP}/🗂️catalog/🦀️component.rs`],
];

for (const [from, to] of MOVES) {
  const source = join(pluginRoot, from);
  const target = join(pluginRoot, to);
  if (!existsSync(source)) throw new Error(`missing source ${from}`);
  mkdirSync(dirname(target), { recursive: true });
  renameSync(source, target);
  console.log(`${from} -> ${to}`);
}

// 📦️ The entry file moves into the packages dir; its leaf `#[path]`s need two more levels back out.
const entryFrom = join(pluginRoot, "📦️lib.rs");
const entryTo = join(pluginRoot, "📦️packages/🦀️rust/📦️lib.rs");
const rename = new Map(MOVES);
let entry = readFileSync(entryFrom, "utf8");
entry = entry.replace(/#\[path = "([^"]+)"\]/g, (all, target: string) => {
  if (target === ".") return all; // 🧭️ base-reset marker — stays relative to the entry file's own dir.
  const moved = rename.get(target) ?? target;
  return `#[path = "../../${moved}"]`;
});
mkdirSync(dirname(entryTo), { recursive: true });
writeFileSync(entryTo, entry);
rmSync(entryFrom);
console.log("📦️lib.rs -> 📦️packages/🦀️rust/📦️lib.rs");

// 🧾️ `[lib] path` is now relative to the manifest's own directory.
const manifestPath = join(pluginRoot, "📦️packages/🦀️rust/Cargo.toml");
const manifest = readFileSync(manifestPath, "utf8").replace('path = "../../📦️lib.rs"', 'path = "📦️lib.rs"');
writeFileSync(manifestPath, manifest);
console.log('Cargo.toml [lib] path -> "📦️lib.rs"');
