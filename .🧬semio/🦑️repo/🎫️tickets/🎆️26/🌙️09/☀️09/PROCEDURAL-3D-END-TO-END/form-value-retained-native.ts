import { readFile, mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";

const root = "🧰️framework/🔨️modules/🖱️ui";
const os = "🧰️framework/🛍️products/💻️os/🔨️modules";
const paths = [
  join(root, "🌳️retained/🌳️tree/🔀️reconcile/🧾️record-routing-and-actions/🦀️.rs"),
  join(root, "🎬️scene/🎬️scenes/📜️doc/🦀️.rs"),
  join(os, "🔌️plugin/🪟️window/🦀️.rs"),
  join(os, "📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs"),
  join(root, "🎬️action/🧱️bounded-action-model/🦀️.rs"),
];
const [reader, hash, producer, generation, action] = await Promise.all(paths.map(path => readFile(path, "utf8")));
function extract(source, start, end) {
  const a = source.indexOf(start), b = source.indexOf(end, a + start.length);
  if (a === -1 || b === -1) throw Error(start);
  return source.slice(a, b);
}
const fixture = await Bun.file(join(import.meta.dirname,"long-generate-text.json")).json();
const asRust = value => 'r####"' + value + '"####';
const occurrences = fixture.cases.map(row => "(" + asRust(row.prefix + row.unit.repeat(row.repeat) + row.suffix) + "," + (row.bytes ?? Buffer.byteLength(row.prefix + row.unit.repeat(row.repeat) + row.suffix)) + ")");
const generated = join(import.meta.dirname, "🗑️generated", "form-value-retained-native");
await mkdir(generated,{recursive:true});
const entry = join(generated, "actual.rs"), binary = join(generated, process.platform === "win32" ? "proof.exe" : "proof");
const stubs = [
  "#![allow(dead_code)]",
  "const UI_DOCUMENT_NODES: usize = 128;",
  "type UiNodeId = u64;",
  "mod ui_contract { pub enum Component { Text(Text), Container } pub struct Text(pub String); impl Text { pub fn packed_payload(&self) -> String { self.0.clone() } } }",
  "struct UiNodeRecord { key: String, component: ui_contract::Component, children: Vec<UiNodeId> }",
  "struct UiDocumentTree(std::collections::HashMap<UiNodeId, UiNodeRecord>); impl UiDocumentTree { fn record(&self,id: UiNodeId)->Option<&UiNodeRecord>{self.0.get(&id)} }",
  "#[derive(Clone,Debug,PartialEq)] enum DslValue { Null, String(String), Array(Vec<DslValue>), Object(Vec<(String,DslValue)>), Number(f64), Bool(bool) }",
  "impl DslValue { fn as_f64(&self)->Option<f64>{if let Self::Number(x)=self {Some(*x)} else {None}} fn as_array(&self)->Option<&[Self]>{if let Self::Array(x)=self {Some(x)} else {None}} fn as_str(&self)->Option<&str>{if let Self::String(x)=self {Some(x)} else {None}} fn as_bool(&self)->Option<bool>{if let Self::Bool(x)=self {Some(*x)} else {None}} }",
  "struct PlaybookBlock { kind: String, fields: Option<Vec<()>>, options: Option<Vec<Choice>> } struct Choice { value: String }",
  "fn default_value_for_block(_: &PlaybookBlock)->DslValue { DslValue::Null }",
  "#[derive(Debug)] pub struct ActionDescriptor { controller_id:String, action:String, args:Option<DslValue> }",
  "#[derive(Debug,PartialEq,Eq,Copy,Clone)] pub struct ActionQueueReceipt(u64);",
  "#[derive(Debug)] pub enum BoundedActionFault { Structure, ByteCredits }",
  "const RETAINED_ACTION_STRING_PAGE_BYTES: usize = 4096; const RETAINED_ACTION_STRING_BYTE_CAPACITY: usize = 16 * 1024 * 1024;",
  "fn checked_action_string_bytes(parts:&[&str])->Result<usize,BoundedActionFault>{Ok(parts.iter().map(|x|x.len()).sum())}",
];
const tests = [
  "#[test] fn stored_body_and_commit_are_complete_in_utf8() {",
  "for (source, bytes) in [" + occurrences.join(",") + "] {",
  'let key = input_value_body_key(source); assert!(key.starts_with(&format!("framework.input.value.{bytes}.")));',
  "let root = UiNodeRecord { key:String::new(),component:ui_contract::Component::Container,children:vec![1] };",
  "let mut document=UiDocumentTree(std::collections::HashMap::from([(1, UiNodeRecord {key:key.clone(),component:ui_contract::Component::Container,children:vec![2]})]));",
  "assert!(retained_text_body(&document,&root,&key,bytes).is_none());",
  "document.0.insert(2,UiNodeRecord {key:String::new(),component:ui_contract::Component::Text(ui_contract::Text(source.to_owned())),children:vec![]});",
  "assert_eq!(retained_text_body(&document,&root,&key,bytes).as_deref(),Some(source));",
  "assert!(retained_text_body(&document,&root,&key,bytes+1).is_none());",
  'let field=PlaybookBlock{kind:"longText".into(),fields:None,options:None};',
  "let pages=DslValue::Array(vec![DslValue::Array(source.chars().map(|c|DslValue::String(c.to_string())).collect())]);",
  "assert_eq!(form::generation_answer(&field,None,&pages,None).unwrap().as_str(),Some(source));",
  'let action = ActionDescriptor {controller_id:"generate".into(),action:"updateGenerationValues".into(),args:Some(DslValue::Object(vec![("generationId".into(),DslValue::String("active".into())),("questionId".into(),DslValue::String("caption".into()))]))};',
  'let mut transfer=RetainedStringAction::new(action,"value".into(),bytes).unwrap();',
  "for scalar in source.chars() { transfer.push_page(scalar.encode_utf8(&mut [0;4])).unwrap(); }",
  "let commit=transfer.finish(Some(ActionQueueReceipt(17))).unwrap();",
  "assert_eq!(commit.receipt,Some(ActionQueueReceipt(17)));",
  "let Some(DslValue::Object(entries))=commit.descriptor.args else { panic!() };",
  'assert_eq!(entries.iter().find(|(key,_)|key=="value").unwrap().1.as_str(),Some(source));',
  'assert_eq!(entries.iter().find(|(key,_)|key=="generationId").unwrap().1.as_str(),Some("active"));',
  'assert_eq!(entries.iter().find(|(key,_)|key=="questionId").unwrap().1.as_str(),Some("caption"));',
  "}}",
];
await writeFile(entry, [
  ...stubs,
  extract(reader,"fn lane_payload(","/// 🔀️ Reattaches").replaceAll("pub(super)","pub(crate)"),
  "mod semio_ui_scene {",extract(hash,"pub fn scene_lane_hash(","pub fn deserialize_non_empty_interaction_domain_text"),"}",
  extract(producer,"pub fn input_value_body_key(","/// 📏️ Upper bound"),
  "mod form { use super::*;",
  extract(generation,"pub fn generation_field_index(","/// 🎛️ Maps a Generate-mode").replace("    fn generation_answer","    pub fn generation_answer"),
  "}",
  extract(action,"#[derive(Debug)]\npub struct QueuedActionDescriptor","#[derive(Debug)]\npub struct BoundedAction").replaceAll("pub(super)","pub(crate)"),
  ...tests,
].join("\n"));
for (const argv of [
  ["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024","--test",entry,"-o",binary],
  [binary],
]) {
  const status = await Bun.spawn(argv, {stdout:"inherit",stderr:"inherit"}).exited;
  if (status) process.exit(status);
}
