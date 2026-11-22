/** 🪟 Stages the exact PDF inspector's own action declarations using the canvas definitions. */
import assert from "node:assert/strict";
import {mkdir} from "node:fs/promises";
import {join} from "node:path";
import law from "./pdf-inspector-action-ownership.json";

const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor";
const paths = [join(root,"🖼️page/🪟️windows/🦀️.rs"),join(root,"🦀️.rs")];
const [windows,editor] = await Promise.all(paths.map(path=>Bun.file(path).text()));
const replace = (source:string,old:string,next:string) => {
  assert.equal(source.split(old).length,2,old.slice(0,120));
  return source.replace(old,next);
};
const definition = windows.indexOf("pub fn inspector_window_definition()");
assert(definition>=0);
const scope = windows.slice(definition);
const records = [
  ['pub fn inspector_window_definition() -> WindowKindDefinition {','pub fn inspector_window_definition(page_actions: &[ActionDefinition]) -> WindowKindDefinition {'],
  ['        actions: Vec::new(),','        actions: page_actions.iter().filter(|action| INSPECTOR_ACTION_IDS.contains(&action.id.as_str())).map(|action| action.clone().in_palette(false)).collect(),'],
];
let tail = scope;
for (const [old,next] of records) tail=replace(tail,old,next);
const lines=[];
for(let i=0;i<law.active.length;i+=4)lines.push("    "+law.active.slice(i,i+4).map(id=>JSON.stringify(id)).join(", ")+",");
const marker="/// 🪟 Fields for the object selected on the page.";
const updatedWindows=replace(windows.slice(0,definition),marker,"const INSPECTOR_ACTION_IDS: &[&str] = &[\n"+lines.join("\n")+"\n];\n\n"+marker)+tail;
const updatedEditor=replace(editor,[
  "    let builder = builder.window_kind_def(main::definition());",
  "    let builder = builder.window_kind_def(crate::editor::page::inspector_window_definition());",
].join("\n"),[
  "    let page = main::definition();",
  "    let inspector = crate::editor::page::inspector_window_definition(&page.actions);",
  "    let builder = builder.window_kind_def(page);",
  "    let builder = builder.window_kind_def(inspector);",
].join("\n"));
const generated=join(import.meta.dir,"🗑️generated","pdf-inspector-owner");
await mkdir(generated,{recursive:true});
const digest=(source:string)=>new Bun.CryptoHasher("sha256").update(source).digest("hex");
await Promise.all([
  Bun.write(join(generated,"windows.rs"),updatedWindows),
  Bun.write(join(generated,"overlay.json"),JSON.stringify([[paths[0],digest(windows),updatedWindows],[paths[1],digest(editor),updatedEditor]])),
]);
console.log("[DEBUG] Staged the bounded PDF inspector's exact view-owned action roster");
