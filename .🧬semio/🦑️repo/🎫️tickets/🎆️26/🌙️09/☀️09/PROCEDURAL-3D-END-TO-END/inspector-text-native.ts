import assert from "node:assert/strict";
import { readFile,mkdir,writeFile } from "node:fs/promises";
import { join,resolve } from "node:path";
import { decode, encode } from "iconv-lite";
import { retainedUiTextInput } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🔤️text-chunks/🟦️.ts";

const ui=resolve("🧰️framework/🔨️modules/🖱️ui");
const procedural=resolve("✏️s/🔌️plugins/🌀️procedural");
const presenter=join(procedural,"🫀️foundation/🖼️semantic-ui");
const generation=join(procedural,"🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any");
const editor=join(generation,"✏️editor");
const output=join(import.meta.dirname,"🗑️generated/inspector-text");
await mkdir(output,{recursive:true});
const fixture=JSON.parse(await readFile(join(import.meta.dirname,"inspector-text-contract.json"),"utf8"));
const materialize=({prefix,unit,repeat,suffix})=>prefix+unit.repeat(repeat)+suffix;
const content=materialize(fixture.display);
const first=materialize(fixture.edit.first);
const next=materialize(fixture.edit.next);
assert.ok(Buffer.byteLength(first)>512);
assert.ok(Buffer.byteLength(next)>512);
const wire=retainedUiTextInput(next).value;
assert.deepEqual(Buffer.concat(wire.flat().map(chunk=>encode(chunk,"utf8"))),encode(next,"utf8"));
assert.equal(decode(encode(content,"utf8"),"utf8",{stripBOM:false}),content);
const input=join(output,"messages.json");
await writeFile(input,JSON.stringify({display:{value:content,integerSpelling:fixture.display.integerSpelling},edit:{first,next,wire}}));
const [inspector,semantic,channels,command,chunks,harness,extra,publisher,tree]=await Promise.all([
 readFile(join(editor,"📌️panels/🔍️inspection/🦀️.rs"),"utf8"),
 readFile(join(presenter,"🦀️.rs"),"utf8"),
 readFile(join(presenter,"📊️channel-values/🦀️.rs"),"utf8"),
 readFile(join(editor,"🎮️commands/🎚️set-widget-input/🦀️.rs"),"utf8"),
 readFile(join(ui,"🧬️contract/🌱️values/🔤️text-chunks/🦀️.rs"),"utf8"),
 readFile(join(import.meta.dirname,"channel-image-native.test.rs"),"utf8"),
 readFile(join(import.meta.dirname,"inspector-text-native.test.rs"),"utf8"),
 readFile(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🦀️.rs"),"utf8"),
 readFile(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🦀️.rs"),"utf8"),
]);
function extract(source,start,end) {
  const first=source.indexOf(start),last=source.indexOf(end,first+start.length);
  assert.ok(first>=0&&last>=0,start);
  return source.slice(first,last);
}
const module=(name,file)=>"#[path = "+JSON.stringify(file)+"] mod "+name+";";
const native=join(output,"actual.rs");
const binary=join(output,process.platform==="win32"?"witness.exe":"witness");
const common=harness.slice(0,harness.indexOf("fn image_with_source")).replace("#[derive(Clone)]\nstruct UiText","#[derive(Clone,Default)]\nstruct UiText").replace("ui_id(text(Label(UiText::clipped(&v.text))),key)","ui_id(text(Label(UiText(v.text.clone()))),key)");
await writeFile(native,[
 "#![allow(dead_code,private_interfaces,private_bounds,unused_imports)]",
 module("budget",join(ui,"🧬️contract/📏️budget/🦀️.rs")),"use budget::*;",
 module("graph_ports",join(generation,"🧬️schema/🔌️graph-ports/🦀️.rs")),"use graph_ports::{GraphTargetKind, graph_port_id};",
 module("window_resource",join(ui,"🧬️contract/🚚️window-resource/🦀️.rs")).replace(" mod "," pub mod "),
 module("text_pages",join(presenter,"📊️channel-values/text-pages.rs")),"use text_pages::TextPages;",
 common,
 extra,
 extract(publisher,"pub fn paged_text_carrier_nodes","/// 🧩️ Builds ONE"),
 "impl TreeWindows<'_> {",extract(tree,"    pub fn try_debit_unreserved","    fn seat"),"}",
 extract(inspector,"fn editable_text_input","/// 📤️ Export format options"),
 extract(inspector,"fn reflected_value","/// 🪪️ Binds the visible inspector"),
 extract(inspector,"fn reflected_output","/// 🥽️ Indexed polygon data"),
 extract(channels,"pub fn windowed_read_only_text","/// 🖼️ Reads the canonical"),
 extract(semantic,"pub fn retained_text_control","/// 🏷️ The generated form"),
 extract(command,"pub const MAX_WIDGET_INPUT_TEXT_BYTES","/// 📃️ Computes one absolute"),
 "impl UiValue {",extract(chunks,"    pub fn join_text_chunks","    /// 📦️ Carries text"),"}",
].join("\n"));
const cache=join(import.meta.dirname,"🗑️generated/mesh-inputs-9f403ac70c06ea23");
const external=(name,id)=>["rlib","rmeta"].flatMap(kind=>["--extern",name+"="+join(cache,"lib"+name+"-"+id+"."+kind)]);
const rustc=Bun.spawn(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024",
 ...external("semio_framework_pack_json","4593048b95aca777"),...external("semio_framework_value","df693cd2ce6ce510"),"-L","dependency="+cache,native,"-o",binary],{stdout:"inherit",stderr:"inherit"});
assert.equal(await rustc.exited,0);
assert.equal(await Bun.spawn([binary,input],{stdout:"inherit",stderr:"inherit"}).exited,0);
console.log("[DEBUG] Exact long output and native integer reflection, complete cross-language Inspector edits, and malformed page refusal");
