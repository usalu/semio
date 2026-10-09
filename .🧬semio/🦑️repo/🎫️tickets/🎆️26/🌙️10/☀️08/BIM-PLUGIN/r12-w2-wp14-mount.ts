#!/usr/bin/env bun
/**
 * 🪢️ Mounts the sheet leaves in the artifact root `🦀️.rs` (idempotent): the nine blocks of `🗑️generated/w2-wp14-sheets/mounts.txt` before the `//#endregion 🔖️Leaves` anchor, the test mount lines of the case
 * `cascades-its-viewports` of `delete-view` right after the `pub use component::*;` line of its block, and the `📄️sheet-layout` inference module after `view_linework`.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, child, em, RS, schema } from "./r3-f1-paths.ts";
import { relRoot } from "./r3-f1-gen-leaf.ts";

const root = join(artifact, RS);
const generated = join(import.meta.dir, "🗑️generated", "w2-wp14-sheets");
let source = readFileSync(root, "utf8");
const notes: string[] = [];

if (source.includes("pub mod create_sheet {")) notes.push("leaves already mounted");
else {
  const mounts = readFileSync(join(generated, "mounts.txt"), "utf8").replace(/\r?\n$/, "");
  const anchor = "                        //#endregion 🔖️Leaves";
  if (!source.includes(anchor)) throw new Error("leaves anchor not found");
  source = source.replace(anchor, `${mounts}\n${anchor}`);
  notes.push("leaves mounted");
}

if (source.includes("mod tests_cascades_its_viewports;")) notes.push("delete-view case already mounted");
else {
  const lines = readFileSync(join(generated, "delete-view-case.txt"), "utf8").replace(/\r?\n$/, "");
  const block = /( {24}pub mod delete_view \{\n(?: {28}.*\n)*? {28}pub use component::\*;\n)/;
  if (!block.test(source)) throw new Error("delete_view block not found");
  source = source.replace(block, `$1${lines}\n`);
  notes.push("delete-view case mounted");
}

if (source.includes("pub mod sheet_layout {")) notes.push("sheet-layout already mounted");
else {
  const module = join(child(schema, "inferences"), em(0x1f4c4) + "sheet-layout", RS);
  const block = `                        #[path = "."]
                        pub mod sheet_layout {
                            #[path = "${relRoot(module)}"]
                            mod component;
                            pub use component::*;
                        }
`;
  const anchor = /( {24}pub mod view_linework \{\n(?: {28}.*\n)+ {24}\}\n)/;
  if (!anchor.test(source)) throw new Error("view_linework mount not found");
  source = source.replace(anchor, `$1${block}`);
  notes.push("sheet-layout mounted");
}

writeFileSync(root, source);
console.log(notes.join("; "));
