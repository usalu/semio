import { execFileSync } from "node:child_process";
import { basename, dirname, join, resolve } from "node:path";
import { createHash } from "node:crypto";
import { cpSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { transpilePluginComponentModulesAsync } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { rewritePreview2ShimImportSource, PREVIEW2_VENDOR_RELATIVE } from "@semio-tech/framework-plugin/browser-bundle/imports";
import { MODULE_PLUGIN_ROUTE } from "@semio-tech/plugin-registry/deployment";

import { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { hostShimSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";
import { pluginComponentBridgeSource, monolithicComponentTable } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";
import { PLUGIN_HOST_SHIM_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";
import { builtPluginModulesRootIn } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { MODULE_BRIDGE_FILE } from "@semio-tech/plugin-registry/deployment";

const source = resolve(process.argv[2]);
const name = basename(source, ".wasm");
const lane = resolve(dirname(source), "../..");
const output = join(lane, "transpiled", name);
const served = join(lane, "browser", name);
const id = /^semio_s_plugin_flow_(math|brep)_component$/.exec(name)?.[1];
if (!id) throw Error("Expected a known Flow extension component");
mkdirSync(output, {recursive:true});
await transpilePluginComponentModulesAsync(source, output, name, {repoRoot:process.cwd(),mode:"prototype",environment:process.env,preview2VendorDir:join(output,PREVIEW2_VENDOR_RELATIVE)});
const js = join(output, name + ".js");
writeFileSync(js, rewritePreview2ShimImportSource(readFileSync(js,"utf8"), "../.." + MODULE_PLUGIN_ROUTE + "/" + PREVIEW2_VENDOR_RELATIVE + "/"));
execFileSync("node", [join(import.meta.dir, "strip-core-debug.mjs"), output, served], {stdio:"inherit"});
const vendor = join(lane, MODULE_PLUGIN_ROUTE.replace(/^\//, ""), PREVIEW2_VENDOR_RELATIVE);
mkdirSync(dirname(vendor), {recursive:true});
cpSync(join(builtPluginModulesRootIn(process.cwd(), "prototype"), PREVIEW2_VENDOR_RELATIVE), vendor, {recursive:true});
writeFileSync(join(served, PLUGIN_HOST_SHIM_FILE), hostShimSource());
const semantic = execFileSync("node", ["--experimental-wasm-jspi","--wasm-num-compilation-tasks=1","--input-type=module","--eval",PLUGIN_DESCRIPTOR_PROBE_SOURCE,join(served,name+".js")], {encoding:"utf8",timeout:300000,maxBuffer:32*1024*1024});
const descriptor = finalizePluginDescriptor(Buffer.from(semantic.trim(),"base64"),"flow-extension-"+id);
writeFileSync(join(served,"🛂️.descriptor.semio"),descriptor.pack);
writeFileSync(join(served,"🔣️.json"),descriptor.json);
writeFileSync(join(served,MODULE_BRIDGE_FILE),pluginComponentBridgeSource(name,monolithicComponentTable(JSON.parse(descriptor.json),name)));
for (const file of readdirSync(served)) {
  console.log(createHash("sha256").update(readFileSync(join(served,file))).digest("hex"), file);
}
