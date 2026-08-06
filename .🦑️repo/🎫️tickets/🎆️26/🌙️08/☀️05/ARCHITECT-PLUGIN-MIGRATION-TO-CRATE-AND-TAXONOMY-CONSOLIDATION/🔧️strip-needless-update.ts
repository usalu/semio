#!/usr/bin/env bun
/** 🧹 Drops the `..Default::default()` struct-update lines clippy's `needless_update` flags in the
 *  ported register patch tests (every field is already listed, so the update has no effect). Line
 *  numbers come straight from `cargo clippy --message-format=short`. Scratch tool for ticket
 *  `26/08/05/ARCHITECT-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const target = join(repoRoot, "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️registers.rs");
const lineNumbers = process.argv.slice(2).map(Number);

const lines = readFileSync(target, "utf8").split("\n");
const drop = new Set(lineNumbers.map((n) => n - 1));
for (const index of drop) {
  const text = lines[index]?.trim();
  if (text !== "..Default::default()") throw new Error(`line ${index + 1} is ${JSON.stringify(text)}, not a struct-update line`);
}
writeFileSync(target, lines.filter((_, index) => !drop.has(index)).join("\n"));
console.log(`dropped ${drop.size} struct-update lines`);
