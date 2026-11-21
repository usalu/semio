import assert from "node:assert/strict";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { PNG } from "pngjs";
import { parseWindowResourcePage, WINDOW_RESOURCE, WindowResourceReader } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🚚️window-resource/🟦️.ts";

const generated = join(import.meta.dirname, "🗑️generated", "retained-resource");
await mkdir(generated, { recursive: true });
const fixture = JSON.parse(await readFile(join(import.meta.dirname, "large-window-resource.json"), "utf8"));
const { width, height } = fixture;
const pixels = new PNG({ width, height });
let random = fixture.seed;
for (let i = 0; i < pixels.data.length; i++) {
  random ^= random << 13;
  random ^= random >>> 17;
  random ^= random << 5;
  pixels.data[i] = i % 4 === 3 ? 255 : random & 255;
}
const png = PNG.sync.write(pixels, { colorType: 2 });
assert.ok(png.byteLength > 2 * 1024 * 1024);
const text = "data:image/png;base64," + png.toString("base64");
assert.ok(text.length >= fixture.minimumSourceBytes && text.length < WINDOW_RESOURCE.maximumBytes);
const input = join(generated, "source-uri.txt"), output = join(generated, "native-pages.json");
await writeFile(input, text);
const rgba = join(generated, "source.rgba");
await writeFile(rgba, pixels.data);
const root = resolve("🧰️framework/🔨️modules/🖱️ui/🧬️contract/🚚️window-resource/🦀️.rs");
const budget = resolve("🧰️framework/🔨️modules/🖱️ui/🧬️contract/📏️budget/🦀️.rs");
const plugin = await readFile(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🦀️.rs"), "utf8");
function extract(start, end) {
  const from = plugin.indexOf(start), to = plugin.indexOf(end, from + start.length);
  assert.ok(from >= 0 && to >= 0);
  return plugin.slice(from, to);
}
const nodes = extract("pub fn paged_text_carrier_nodes(", "/// 🧩️ Builds ONE");
const publisher = extract("pub struct WindowResourceRequests", "/// 🧩️ Builds one reserved");
const source = join(generated, "witness.rs"), binary = join(generated, process.platform === "win32" ? "native.exe" : "native");
await writeFile(source, [
  "#![allow(dead_code)]",
  "#[path = " + JSON.stringify(root) + "] mod resource;",
  "use resource::*;",
  "use resource as window_resource;",
  "#[path = " + JSON.stringify(budget) + "] mod budget;",
  "use budget::*;",
  "#[path = " + JSON.stringify(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🖼️ui-image-loading/🚚️window-source/🦀️.rs")) + "] mod native_image;",
  await readFile(join(import.meta.dirname,"large-window-resource-native.rs"),"utf8"),
  "mod semio_framework { pub struct TreeWindowRequest { pub body_key:String, pub node_key:String, pub open:Option<bool>, pub offset:u32, pub rows:u32 } }",
  "struct UiText; impl UiText { fn try_from_str(value:&str)->Option<Self>{(value.len()<=UI_TEXT_MAX_BYTES).then_some(Self)} }",
  "type BuiltNode=(String,String); type Label=String; type UiAssemblyResult<T>=Result<T,&'static str>;",
  "fn ui_assembly_error(error:&'static str)->&'static str{error}",
  "fn image_with_source(_id:&str, source:&str, _alt:Label)->UiAssemblyResult<BuiltNode>{Ok((String::new(),source.into()))}",
  "fn image_with_retained_source(_id:&str,key:&str,source:&str,_alt:Label)->UiAssemblyResult<BuiltNode>{Ok((key.into(),source.into()))}",
  nodes,
  publisher,
  "fn main() {",
  "let source = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();",
  "assert_eq!(WINDOW_RESOURCE_PAGE_BYTES, " + WINDOW_RESOURCE.pageBytes + ");",
  "assert_eq!(WINDOW_RESOURCE_MAX_BYTES, " + WINDOW_RESOURCE.maximumBytes + ");",
  "let window = " + JSON.stringify(fixture.window) + "; let question = " + JSON.stringify(fixture.question) + "; let body = " + JSON.stringify(fixture.body) + ";",
  "let resource = WindowResource::new(window,question,&source).unwrap();",
  "assert!(resource.matches_owner(window,question));",
  "verify_native(&source,window,question,resource);",
  "assert!(paged_text_carrier_nodes(&source) > UI_DOCUMENT_NODES);",
  "assert!(WindowResourceRequests::image_body_nodes(&source) < TREE_WINDOW_BODY_NODE_BUDGET / 4);",
  "let term = semio_framework::TreeWindowRequest {body_key:body.into(),node_key:resource.request_key(),open:None,offset:4,rows:1};",
  "let requests = [term];",
  "let owner = WindowResourceRequests {window_id:window,body_key:body,requests:&requests};",
  "let (desc, page) = owner.image(question,&source,String::new()).unwrap();",
  "assert_eq!(WindowResourcePage::parse(&desc).unwrap().index,4);",
  "assert!(page.len() <= WINDOW_RESOURCE_PAGE_BYTES);",
  'let other_body = WindowResourceRequests {window_id:window,body_key:"other",requests:&requests};',
  "assert_eq!(WindowResourcePage::parse(&other_body.image(question,&source,String::new()).unwrap().0).unwrap().index,1);",
  'let small = "data:image/png;base64,ABC=";',
  "assert_eq!(WindowResourceRequests::image_body_nodes(small),0);",
  "assert_eq!(owner.image(question,small,String::new()).unwrap().1,small);",
  "let medium = &source[..1_000_000];",
  "assert_eq!(WindowResourceRequests::image_body_nodes(medium),paged_text_carrier_nodes(medium));",
  "assert_eq!(owner.image(question,medium,String::new()).unwrap().1,medium);",
  "let peer = WindowResource::new(" + JSON.stringify(fixture.wrongWindow) + ", question, &source).unwrap();",
  "let make = |index| WindowResourcePage { resource, index };",
  "let mut reader = WindowResourceReader::new(resource);",
  "assert_eq!(reader.accept(make(2), make(2).text(&source).unwrap()), Err(WindowResourceRefusal::Sequence));",
  "assert_eq!(reader.accept(WindowResourcePage {resource:peer,index:1}, make(1).text(&source).unwrap()), Err(WindowResourceRefusal::Authority));",
  'println!("[");',
  'for index in 1..=resource.pages() { let page = make(index); let key = page.key(); assert_eq!(WindowResourcePage::parse(&key),Some(page)); let text = page.text(&source).unwrap(); if index == resource.pages() { let damaged = format!("{}A",&text[..text.len()-1]); assert!(reader.accept(page,&damaged).is_err()); } reader.accept(page,text).unwrap(); println!("{:?}{}", key, if index == resource.pages() {""} else {","}); }',
  'println!("]");',
  "assert_eq!(reader.complete(),Some(source.as_str()));",
  'let multibyte = "x".repeat(WINDOW_RESOURCE_PAGE_BYTES - 1) + "🧬漢";',
  'let utf8 = WindowResource::new("w","v",&multibyte).unwrap();',
  "let mut reader = WindowResourceReader::new(utf8);",
  "for index in 1..=utf8.pages() { let page = WindowResourcePage {resource:utf8,index}; reader.accept(page,page.text(&multibyte).unwrap()).unwrap(); }",
  "assert_eq!(reader.complete(),Some(multibyte.as_str()));",
  "}"
].join("\n"));
const compile = Bun.spawn(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024",source,"-o",binary], {stdout:"inherit", stderr:"inherit"});
assert.equal(await compile.exited, 0);
const native = Bun.spawn([binary,input,rgba], { stdout:"pipe", stderr:"inherit" });
const contents = await new Response(native.stdout).text();
assert.equal(await native.exited,0);
await writeFile(output, contents);
const descriptors = JSON.parse(contents).map(key => parseWindowResourcePage(key));
assert.equal(descriptors.length, fixture.pages);
const first = descriptors[0];
const reader = new WindowResourceReader(first.resource);
assert.ok(!reader.accept(descriptors[1], text.slice(WINDOW_RESOURCE.pageBytes, 2 * WINDOW_RESOURCE.pageBytes)));
assert.ok(!reader.accept({ ...first, resource: { ...first.resource, key: "different-owner" } }, text.slice(0, WINDOW_RESOURCE.pageBytes)));
for (const descriptor of descriptors) {
  assert.ok(descriptor);
  const start = (descriptor.index - 1) * WINDOW_RESOURCE.pageBytes;
  const page = text.slice(start, start + WINDOW_RESOURCE.pageBytes);
  if (descriptor.index === descriptor.resource.pages) assert.ok(!reader.accept(descriptor, page.slice(0,-1) + "A"));
  assert.ok(reader.accept(descriptor, page));
  assert.ok(!reader.accept(descriptor, page));
}
const restored = reader.complete();
assert.equal(restored, text);
const decoded = PNG.sync.read(Buffer.from(restored.slice(restored.indexOf(",") + 1), "base64"));
assert.equal(decoded.width,width);
assert.equal(decoded.height,height);
assert.deepEqual(decoded.data,pixels.data);
console.log("[DEBUG] Rust and browser-language readers restored", png.byteLength, "PNG bytes over", descriptors.length, "owned pages");
