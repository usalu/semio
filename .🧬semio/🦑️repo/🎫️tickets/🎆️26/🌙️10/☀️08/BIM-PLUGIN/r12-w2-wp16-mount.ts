#!/usr/bin/env bun
/**
 * 🪢️ Mounts the coordination leaves (idempotent): the twelve blocks of `🗑️generated/w2-wp16-coordination/mounts.txt` before the `//#endregion 🔖️Leaves` anchor of the artifact root `🦀️.rs`, the test mount lines of
 * the case `cascades-clash-sets-and-rules-scoped-to-it` of `delete-storey` right after the `pub use component::*;` line of its block, and the twelve variants plus kind names in the mutation aggregate.
 * Further sections (inference modules, panels) are added by their own scripts or by hand.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, child, RS, schema } from "./r3-f1-paths.ts";
import { leaves } from "./r12-w2-wp16-leaves.ts";

const root = join(artifact, RS);
const generated = join(import.meta.dir, "🗑️generated", "w2-wp16-coordination");
const notes: string[] = [];

let source = readFileSync(root, "utf8");
if (source.includes("pub mod create_clash_set {")) notes.push("leaves already mounted");
else {
  const mounts = readFileSync(join(generated, "mounts.txt"), "utf8").replace(/\r?\n$/, "");
  const anchor = "                        //#endregion 🔖️Leaves";
  if (!source.includes(anchor)) throw new Error("leaves anchor not found");
  source = source.replace(anchor, `${mounts}\n${anchor}`);
  notes.push("leaves mounted");
}
if (source.includes("mod tests_cascades_clash_sets_and_rules_scoped_to_it;")) notes.push("delete-storey case already mounted");
else {
  const lines = readFileSync(join(generated, "delete-storey-case.txt"), "utf8").replace(/\r?\n$/, "");
  const block = /( {24}pub mod delete_storey \{\n(?: {28}.*\n)*? {28}pub use component::\*;\n)/;
  if (!block.test(source)) throw new Error("delete_storey block not found");
  source = source.replace(block, `$1${lines}\n`);
  notes.push("delete-storey case mounted");
}
writeFileSync(root, source);

const aggregate = join(child(schema, "mutations"), RS);
let text = readFileSync(aggregate, "utf8");
const snake = (kind: string) => kind.replaceAll("-", "_");
if (text.includes("CreateClashSet(")) notes.push("aggregate already lists the leaves");
else {
  const variants = leaves.map((leaf) => `    ${leaf.variant}(super::${snake(leaf.kind)}::${leaf.variant}),`).join("\n");
  const names = leaves.map((leaf) => `    "${leaf.kind}",`).join("\n");
  const variantAnchor = "    DeleteSheetRevision(super::delete_sheet_revision::DeleteSheetRevision),\n";
  const nameAnchor = '    "delete-sheet-revision",\n';
  if (!text.includes(variantAnchor) || !text.includes(nameAnchor)) throw new Error("aggregate anchors not found");
  text = text.replace(variantAnchor, `${variantAnchor}${variants}\n`).replace(nameAnchor, `${nameAnchor}${names}\n`);
  writeFileSync(aggregate, text);
  notes.push("aggregate extended");
}
console.log(notes.join("; "));
