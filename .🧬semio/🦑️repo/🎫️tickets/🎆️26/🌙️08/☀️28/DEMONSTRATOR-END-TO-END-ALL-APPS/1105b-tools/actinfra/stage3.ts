import {copyFileSync,existsSync,mkdirSync,readdirSync,writeFileSync,rmSync,statSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {hostShimSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";
import {ensurePreview2ShimVendorAt} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🩹️preview2-shim-patches/🟦️.ts";
import {rewritePreview2ShimImports} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import {PLUGIN_HOST_SHIM_FILE} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";
import {PLUGIN_DESCRIPTOR_PROBE_SOURCE,finalizePluginDescriptor} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import {pluginComponentBridgeSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";
const repoRoot="/Users/ueli/Documents/semio",out=process.argv[2]!,base="semio_s_plugin_flow_list_component",transpiled=join(out,"transpiled"),optimized=join(out,"optimized"),module=join(out,"module"),vendor=join(out,"vendor");
rmSync(module,{recursive:true,force:true});mkdirSync(module,{recursive:true});
const walk=(dir:string,prefix=""):string[]=>readdirSync(dir,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(join(dir,e.name),prefix+e.name+"/"):[prefix+e.name]);
for(const name of walk(transpiled)){const src=existsSync(join(optimized,name))?join(optimized,name):join(transpiled,name);mkdirSync(join(module,name,".."),{recursive:true});copyFileSync(src,join(module,name));}
writeFileSync(join(module,PLUGIN_HOST_SHIM_FILE),hostShimSource());
ensurePreview2ShimVendorAt(vendor,repoRoot);
rewritePreview2ShimImports(join(module,base+".js"),vendor);
console.log("STAGE assemble OK",walk(module).length,"files",(walk(module).reduce((n,f)=>n+statSync(join(module,f)).size,0)/1048576).toFixed(1)+" MiB");
const probe=spawnSync("node",["--experimental-wasm-jspi","--input-type=module","--eval",PLUGIN_DESCRIPTOR_PROBE_SOURCE,join(module,base+".js")],{cwd:repoRoot,encoding:"utf8",maxBuffer:64<<20,timeout:300000});
if(probe.status!==0){console.log("STAGE probe FAILED",probe.status,(probe.stderr||"").slice(-1500));process.exit(1);}
console.log("STAGE probe OK describe() answered",probe.stdout.length,"chars");
const semantic=Buffer.from(probe.stdout.trim(),"base64");
const {decodePackValue}=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts");const decoded=decodePackValue(new Uint8Array(semantic)) as {manifest?:{pluginId?:string}};const pluginId=decoded.manifest?.pluginId??"";console.log("  descriptor pluginId",pluginId);const descriptor=finalizePluginDescriptor(new Uint8Array(semantic),pluginId);
console.log("STAGE finalize OK pack",descriptor.pack.byteLength,"json",descriptor.json.length);
writeFileSync(join(module,"🛂️.descriptor.semio"),descriptor.pack);writeFileSync(join(module,"🔣️.json"),descriptor.json);writeFileSync(join(module,"bridge.js"),pluginComponentBridgeSource(base,base+".wasm"));
console.log("STAGE publish-to-scratch OK",walk(module).length,"files");
import {createHash} from "node:crypto";import {readFileSync} from "node:fs";
const committed=process.argv[3];if(committed){const live=createHash("sha256").update(descriptor.pack).digest("hex"),disk=createHash("sha256").update(readFileSync(committed)).digest("hex");console.log("CHECK committed descriptor pack equals finalized probe pack:",live===disk,live.slice(0,12),disk.slice(0,12));}
