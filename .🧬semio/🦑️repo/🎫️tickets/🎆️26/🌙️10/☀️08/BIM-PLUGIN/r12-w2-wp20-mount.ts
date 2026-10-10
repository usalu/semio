#!/usr/bin/env bun
/**
 * 🪢️ Mounts the energy vocabulary (idempotent): the three leaf blocks of `🗑️generated/w2-wp20-energy/mounts.txt` before the `//#endregion 🔖️Leaves` anchor of the artifact root `🦀️.rs`, the test mount lines of the case
 * `removes-its-conditions` of `delete-space`, the enum variants and kind names in the mutation aggregate, and the `🌡️energy-envelope` inference module after `sheet_layout`.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, child, em, RS, schema } from "./r3-f1-paths.ts";
import { relRoot } from "./r3-f1-gen-leaf.ts";

const root = join(artifact, RS);
const aggregate = join(child(schema, "mutations"), RS);
const generated = join(import.meta.dir, "🗑️generated", "w2-wp20-energy");
let source = readFileSync(root, "utf8");
const notes: string[] = [];

if (source.includes("pub mod set_space_conditions {")) notes.push("leaves already mounted");
else {
  const mounts = readFileSync(join(generated, "mounts.txt"), "utf8").replace(/\r?\n$/, "");
  const anchor = "                        //#endregion 🔖️Leaves";
  if (!source.includes(anchor)) throw new Error("leaves anchor not found");
  source = source.replace(anchor, `${mounts}\n${anchor}`);
  notes.push("leaves mounted");
}

if (source.includes("mod tests_removes_its_conditions;")) notes.push("delete-space case already mounted");
else {
  const lines = readFileSync(join(generated, "delete-space-case.txt"), "utf8").replace(/\r?\n$/, "");
  const block = /( {24}pub mod delete_space \{\n(?: {28}.*\n)*? {28}pub use component::\*;\n)/;
  if (!block.test(source)) throw new Error("delete_space block not found");
  source = source.replace(block, `$1${lines}\n`);
  notes.push("delete-space case mounted");
}

if (source.includes("pub mod energy_envelope {")) notes.push("energy-envelope already mounted");
else {
  const module = join(child(schema, "inferences"), em(0x1f321) + "energy-envelope", RS);
  const block = `                        #[path = "."]
                        pub mod energy_envelope {
                            #[path = "${relRoot(module)}"]
                            mod component;
                            pub use component::*;
                        }
`;
  const anchor = /( {24}pub mod sheet_layout \{\n(?: {28}.*\n)+ {24}\}\n)/;
  if (!anchor.test(source)) throw new Error("sheet_layout mount not found");
  source = source.replace(anchor, `$1${block}`);
  notes.push("energy-envelope mounted");
}
writeFileSync(root, source);

let enumSource = readFileSync(aggregate, "utf8");
if (enumSource.includes("SetSpaceConditions(")) notes.push("aggregate already carries the variants");
else {
  const variants = `    SetSpaceConditions(super::set_space_conditions::SetSpaceConditions),
    RemoveSpaceConditions(super::remove_space_conditions::RemoveSpaceConditions),
    SetTypeThermalData(super::set_type_thermal_data::SetTypeThermalData),
`;
  const variantAnchor = "    DeleteSheetRevision(super::delete_sheet_revision::DeleteSheetRevision),\n";
  const kindAnchor = '    "delete-sheet-revision",\n';
  if (!enumSource.includes(variantAnchor) || !enumSource.includes(kindAnchor)) throw new Error("aggregate anchors not found");
  enumSource = enumSource.replace(variantAnchor, variantAnchor + variants).replace(kindAnchor, kindAnchor + '    "set-space-conditions",\n    "remove-space-conditions",\n    "set-type-thermal-data",\n');
  writeFileSync(aggregate, enumSource);
  notes.push("aggregate extended");
}
console.log(notes.join("; "));
