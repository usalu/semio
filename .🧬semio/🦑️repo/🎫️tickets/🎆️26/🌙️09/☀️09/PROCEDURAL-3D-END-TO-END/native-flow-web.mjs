import { mkdir, readFile, writeFile } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { resolve, join } from "node:path";
import { platform } from "node:os";

const [wasm, destination] = process.argv.slice(2).map(resolvePath => resolve(resolvePath));
const tools = process.env.CARGO_HOME ?? resolve(".cache/b/r/m/cargo/home");
const bindgen = join(tools, "bin", platform() === "win32" ? "wasm-bindgen.exe" : "wasm-bindgen");
await mkdir(destination,{recursive:true});
const build = spawnSync(bindgen, [wasm,"--remove-name-section","--target","web","--out-name","flow_core","--out-dir",destination],{stdio:"inherit"});
if(build.status!==0) throw new Error("Flow wasm-bindgen failed", {cause:build.error});
const source = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm");
for(const role of ["🌐️browser","🖥️host"]){
  const input = join(source,role,"🏃️runtime","🟨️.js");
  const code = (await readFile(input,"utf8")).replace('import("../../../🎁️browser-package/🕸️bindings/flow_core.js")','import("../flow_core.js")').replace('"../../🖥️host/🏃️runtime/🟨️.js"','"../🖥️host/🟨️.js"');
  const build = await Bun.build({entrypoints:["flow:entry"],target:"browser",external:["../flow_core.js","../🖥️host/🟨️.js"],plugins:[{name:"flow-source",setup(builder){
    builder.onResolve({filter:/^flow:entry$/},()=>({path:"entry",namespace:"flow"}));
    builder.onLoad({filter:/.*/,namespace:"flow"},()=>({contents:code,loader:"js"}));
  }}]});
  if(!build.success) throw new AggregateError(build.logs,"Flow JS bundle failed");
  await mkdir(join(destination,role),{recursive:true});
  await writeFile(join(destination,role,"🟨️.js"), await build.outputs[0].text());
}

const publication = join(source, "🌐️browser", "📦️publication");
const { flowBrowserPackageManifestV1 } = await import(join(publication, "🌱️bootstrap", "🟦️.ts"));
const { flowBrowserDeclaration } = await import(join(source, "🌐️browser", "📝️declaration", "📤️projection", "🟦️.ts"));
const protocol = JSON.parse(await readFile(join(publication, "🧬️schema", "🔣️.json"), "utf8")).definitions.Protocol.const;
const family = resolve(protocol.sourceOwner, "Cargo.toml");
const version = (await readFile(family, "utf8")).match(/^version\s*=\s*"([^"]+)"/m)?.[1];
const manifest = { ...flowBrowserPackageManifestV1(version), semioWasmOwner: {version:1, sourceOwner:protocol.sourceOwner, mode:"prototype"} };
await writeFile(join(destination,protocol.declaration),flowBrowserDeclaration());
await writeFile(join(destination,"package.json"),JSON.stringify(manifest,null,2)+"\n");
