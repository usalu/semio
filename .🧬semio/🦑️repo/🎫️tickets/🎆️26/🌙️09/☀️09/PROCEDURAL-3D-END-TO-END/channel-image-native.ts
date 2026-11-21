import assert from "node:assert/strict";
import { mkdir, readFile, readdir, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { PNG } from "pngjs";

const checkout = process.cwd();
const output = join(import.meta.dirname,"🗑️generated/channel-image");
await mkdir(output,{ recursive:true });
const ui = resolve(checkout, "🧰️framework/🔨️modules/🖱️ui");
const plugin = resolve(checkout, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
const channel = resolve(checkout, "✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📊️channel-values");
const [values, publisher, kit, harness] = await Promise.all([
  readFile(join(channel,"🦀️.rs"),"utf8"),
  readFile(join(plugin,"🪟️window/🦀️.rs"),"utf8"),
  readFile(join(plugin,"🪟️window-kits/🌳️tree/🦀️.rs"),"utf8"),
  readFile(join(import.meta.dirname,"channel-image-native.test.rs"),"utf8"),
]);
function extract(source, start, end) {
  const begin = source.indexOf(start);
  const stop = source.indexOf(end, begin + start.length);
  assert.ok(begin >= 0 && stop >= 0);
  return source.slice(begin, stop);
}
const portable = join(channel,"🧪️tests/🖼️images.json");
const fixture = JSON.parse(await readFile(portable,"utf8"));
const verified = PNG.sync.read(Buffer.from(fixture.image.dataUrl.split(",")[1],"base64"));
assert.equal(verified.width,fixture.image.width);
assert.equal(verified.height,fixture.image.height);
assert.deepEqual(Array.from(verified.data),fixture.pixels);
const pixels = new PNG({ width:256, height:256 });
let random = 0x12345678;
for (let i=0; i<pixels.data.length; i++) {
  random ^= random << 13;
  random ^= random >>> 17;
  random ^= random << 5;
  pixels.data[i]=i%4===3 ? 255 : random&255;
}
const data = "data:image/png;base64," + PNG.sync.write(pixels,{colorType:2}).toString("base64");
const source = join(output,"source-uri.txt");
await writeFile(source,data);
const native = join(output,"actual.rs");
const binary = join(output,process.platform==="win32"?"witness.exe":"witness");
const module = (name,file) => "#[path = "+JSON.stringify(file)+"] mod "+name+";";
await writeFile(native, [
  "#![allow(dead_code,private_interfaces)]",
  module("budget",join(ui,"🧬️contract/📏️budget/🦀️.rs")),
  "use budget::*;",
  module("window_resource",join(ui,"🧬️contract/🚚️window-resource/🦀️.rs")).replace(" mod ", " pub mod "),
  "mod channel_values { use super::*; use semio_framework_pack_json::Value;",
  module("text_pages",join(channel,"text-pages.rs")),
  "use self::text_pages::TextPages;",
  values.slice(values.indexOf("/// 🔌️ One evaluated direction")).replace("fn channel_image_source","pub(super) fn channel_image_source"),
  "}",
  extract(publisher,"pub fn paged_text_carrier_nodes","/// 🧩️ Builds ONE"),
  extract(publisher,"pub struct WindowResourceRequests","/// 🧩️ Builds one reserved"),
  "impl TreeWindows<'_> {",
  extract(kit,"    /// 🖼️ Builds one image child","    /// 🧾️ Whether the render currently stands"),
  "}",
  harness.replace("CHANNEL_IMAGE_FIXTURE",JSON.stringify(portable)),
].join("\n"));
const cache=resolve(checkout,".cache/b/r/m/u/l/p1c4772/prototype/build");
const libraries=(await Promise.all((await readdir(cache,{withFileTypes:true})).filter(entry=>entry.isDirectory()).map(async({name}) =>
 (await readdir(join(cache,name),{withFileTypes:true})).filter(entry=>entry.isDirectory()).map(({name:id})=>join(cache,name,id,"out"))))).flat();
const pack=join(cache,"semio-framework-pack-json/5262b28961bbf9e7/out/libsemio_framework_pack_json-5262b28961bbf9e7.rlib");
const compile=Bun.spawn(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024","--extern","semio_framework_pack_json="+pack,"--extern","semio_framework_pack_json="+pack.replace(/rlib$/,"rmeta"),...libraries.flatMap(path=>["-L","dependency="+path]),native,"-o",binary],{stdout:"inherit",stderr:"inherit"});
assert.equal(await compile.exited,0);
const witness=Bun.spawn([binary,source],{stdout:"pipe",stderr:"inherit"});
const result=JSON.parse(await new Response(witness.stdout).text());
assert.equal(await witness.exited,0);
assert.equal(result.pages.length,3);
await writeFile(join(output,"native-pages.json"),JSON.stringify(result));
console.log("[DEBUG] Flow Image schema, original directional hover, exact source pages, metadata, node cost and PNGJS pixels");
