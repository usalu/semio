/** 🏭️ Generate exact IFC2x3 declarations through the shared first-party API. */
import fs from "node:fs/promises";
import path from "node:path";
import {stripTypeScriptTypes} from "node:module";
import {pathToFileURL} from "node:url";
export async function generate(root,ticket){
 const ifc=path.join(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"),owner=path.join(ifc,"🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog"),output=path.join(ticket,"🗑️generated/catalog2x3-generation-"+Date.now()),cache=new Map();await fs.mkdir(output,{recursive:true});
 async function load(file){file=path.resolve(file);if(cache.has(file))return cache.get(file);const target=path.join(output,"module"+cache.size+".mjs");cache.set(file,target);let source=stripTypeScriptTypes(await fs.readFile(file,"utf8"),{mode:"transform"});for(const match of [...source.matchAll(/from\s*["']([^"']+)["']/g)]){if(!match[1].startsWith("."))continue;const dependency=await load(path.resolve(path.dirname(file),match[1]));source=source.replace(match[0],()=>"from "+JSON.stringify(pathToFileURL(dependency).href));}await fs.writeFile(target,source);return target;}
 const {generateCatalog}=await import(pathToFileURL(await load(path.join(ifc,"📚️catalog/🛠️extract/📁️generate.ts"))).href),files={source:path.join(owner,"IFC2X3_TC1.exp"),manifest:path.join(owner,"source.json"),json:path.join(output,"catalog.json"),rust:path.join(output,"data.rs")},result=await generateCatalog(files);
 if(result.schema!=="IFC2X3"||result.entities!==653||result.types!==327||result.functions!==38||result.rules!==2)throw Error("Unexpected normative IFC2x3 declarations");
 await fs.writeFile(path.join(owner,"🔣️.json"),await fs.readFile(files.json));await fs.writeFile(path.join(owner,"data.rs"),await fs.readFile(files.rust));return {result,output};
}
