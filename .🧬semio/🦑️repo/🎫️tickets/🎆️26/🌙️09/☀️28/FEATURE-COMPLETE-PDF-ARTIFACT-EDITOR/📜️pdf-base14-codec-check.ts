/** 🦀️ Type-checks the staged Base 14 codec with the repository's real WASM PDF dependencies. */
import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";

const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts";
const output = join(import.meta.dir,"🗑️generated","pdf-base14");
const generated = join(output,"metadata");
const cache = resolve(process.argv[2] ?? "");
assert(cache.endsWith("/wasm32-wasip2/prototype/build"),"Pass a WASI repository unit cohort.");
await mkdir(generated,{recursive:true});
const symbolic = process.argv.includes("--symbolic");
const code: [string,string,string][] = await Bun.file(join(output,symbolic ? "symbolic-code-overlay.json" : "font-code-overlay.json")).json();
const packages: [string,string|null,string][] = await Bun.file(join(output,"font-package-overlay.json")).json();
const assets: {path:string;sha256:string;base64:string}[] = await Bun.file(join(output,"font-asset-overlay.json")).json();

async function rust(args: readonly string[]) {
  const work = Bun.spawn(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc",
    "--edition","2024","--target","wasm32-wasip2","-Cpanic=abort","--crate-type","lib","--emit","metadata",...args], {
    stdout:"inherit",stderr:"inherit",
  });
  assert.equal(await work.exited,0);
}

const extra: string[] = [];
for (const [path,before,body] of packages) {
  if (before !== null || !path.endsWith("/Cargo.toml")) continue;
  const manifest = Bun.TOML.parse(body) as {package:{name:string};lib:{path:string}};
  const owner = dirname(path);
  const folder = dirname(dirname(owner));
  const stage = join(generated,manifest.package.name);
  const rustRoot = join(stage,"a","b");
  await mkdir(rustRoot,{recursive:true});
  const source = packages.find(([p])=>p === join(owner,manifest.lib.path));
  assert(source);
  const name = manifest.package.name.replaceAll("-","_");
  const entry = join(rustRoot,manifest.lib.path);
  await writeFile(entry,source[2]);
  for (const asset of assets) {
    if (dirname(asset.path) !== folder) continue;
    await writeFile(join(stage,asset.path.split("/").at(-1)!),Buffer.from(asset.base64,"base64"));
  }
  const meta = join(generated,"lib"+name+".rmeta");
  await rust(["--crate-name",name,entry,"-o",meta]);
  extra.push("--extern",name+"="+meta);
}

const [fontFile, original, patch] = code[0]!;
assert.equal(new Bun.CryptoHasher("sha256").update(await Bun.file(fontFile).bytes()).digest("hex"),original);
const font = join(generated,"fonts.rs");
let absolute = patch.replace(/(#\[path\s*=\s*)"([^"]+)"/gu,(_text,lead,path)=>lead+JSON.stringify(resolve(dirname(fontFile),path)));
if (symbolic) {
  const [source, previous, body] = code.find(([path])=>path.includes("🎚️controlled-opentype/"))!;
  assert.equal(new Bun.CryptoHasher("sha256").update(await Bun.file(source).bytes()).digest("hex"),previous);
  const stagedChild = join(generated,"controlled-opentype.rs");
  await writeFile(stagedChild,body);
  absolute=absolute.replace(JSON.stringify(resolve(source)),JSON.stringify(stagedChild));
}
await writeFile(font,absolute);
const lib = join(generated,"lib.rs");
const main = await Bun.file(join(root,"📦️packages/🦀️rust/🦀️.rs")).text();
assert(main.includes('#[path = "../../🦀️.rs"]'));
await writeFile(lib,main.replace('#[path = "../../🦀️.rs"]','#[path = '+JSON.stringify(font)+']'));
const dependencies = [
  "semio-framework-hash",
  "semio-s-artifact-stdio-pdf-inferences",
  "semio-s-artifact-stdio-pdf-model",
  "semio-s-artifact-stdio-pdf-snapshot-2",
  "semio-s-artifact-stdio-pdf-glyph-list",
  "semio-s-artifact-stdio-pdf-tables",
];
const args = ["--crate-name","semio_s_artifact_stdio_pdf_fonts",lib,"-o",join(generated,"fonts.rmeta"),...extra];
for (const globRoot of [cache,resolve(cache,"../../../prototype/build")]) {
  for await (const path of new Bun.Glob("*/*/out").scan({cwd:globRoot,onlyFiles:false})) {
    args.push("-Ldependency="+join(globRoot,path));
  }
}
for (const dependency of dependencies) {
  const name = dependency.replaceAll("-","_");
  const libs = await Array.fromAsync(new Bun.Glob("*/out/lib"+name+"-*.rmeta").scan({cwd:join(cache,dependency),onlyFiles:true}));
  assert.equal(libs.length,1,dependency);
  args.push("--extern",name+"="+join(cache,dependency,libs[0]!));
}
const response = join(generated,"font-metadata.args");
await writeFile(response,args.join("\n")+"\n");
await rust(["@"+response]);
console.log("[DEBUG] Type-checked the actual staged font codec and six static font resource owners under WASI.");
