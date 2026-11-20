import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const root = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained";
const read = path => readFileSync(root + "/" + path, "utf8");
const aggregate = read("../🦀️.rs");
const variantStart = aggregate.indexOf("pub enum LayoutMutation");
const variants = [...aggregate.slice(variantStart, aggregate.indexOf("\n}", variantStart)).matchAll(/^\s+(\w+)\(/gm)].map(row => row[1]).sort();
const ordinary = read("🎚️simple/🦀️.rs").split("pub fn of(")[1].split(" pub fn pool(")[0];
const batch = read("🖼️frame/👥️batch/🏭️prepare/🦀️.rs").split("pub fn selected(")[1].split("\n")[0];
const prepare = read("🏭️prepare/🦀️.rs");
const delegated = ["EditStory", "SetParentPageContent"];
for (const command of delegated) {
  assert.ok(prepare.includes("LayoutMutation::" + command + "(_)"), command);
}
const captured = [...ordinary.matchAll(/LayoutMutation::(\w+)\(_\)=>/g), ...batch.matchAll(/LayoutMutation::(\w+)\(_\)/g)].map(row => row[1]);
assert.deepEqual([...new Set([...captured, ...delegated])].sort(), variants);
const live = readFileSync("✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️layout-play-app/🦀️.rs", "utf8");
assert.match(live, /fn build_artifact_store_one_item_preparation_factory[\s\S]*?Some\(store::mutation_apply_preparation_factory::/);
console.log(variants.length + " current Layout mutations route through the editor's retained producer");
