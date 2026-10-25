import {readFileSync} from "node:fs";
import {fileURLToPath,pathToFileURL} from "node:url";
import {createRequire} from "node:module";
import {runInNewContext} from "node:vm";
import {transform} from "esbuild";
const base=new URL("file:///Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🧪️tests/🟦️.ts");
const root=fileURLToPath(new URL("../../../../../🟨️.mjs",base)),argumentsPath=fileURLToPath(new URL("../../../../../⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs",base)),helper=fileURLToPath(new URL("../../../../📥️caller/📤️orchestration/🟨️.mjs",base)),implementationPath=fileURLToPath(new URL("../../../../../⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🟨️.mjs",base)),schema=fileURLToPath(new URL("../../../../📥️caller/📤️orchestration/🧬️schema/🔣️.json",base)),real=createRequire(base.href);
const virtual=new Map<string,Buffer>();const reads:string[]=[];
const evaluate=async(path:string,dependencies:Record<string,unknown>)=>{const source=readFileSync(path,"utf8"),{code}=await transform(source,{format:"cjs",target:"node22",define:{"import.meta.url":JSON.stringify(pathToFileURL(path).href)}}),module={exports:{}},local=createRequire(path);runInNewContext(code,{module,exports:module.exports,URL,Buffer,process,console,require:(specifier:string)=>specifier==="node:fs"?{...real("node:fs"),readFileSync:(p:string|URL,encoding?:BufferEncoding)=>{const key=p instanceof URL?fileURLToPath(p):p;reads.push(path.slice(-30)+" <- "+String(key).slice(-45));const bytes=virtual.get(key)??readFileSync(key);return encoding?bytes.toString(encoding):bytes;}}:Object.hasOwn(dependencies,specifier)?dependencies[specifier]:local(specifier)},{filename:path,timeout:5000});return module.exports as any;};
for(const mutation of [helper,argumentsPath,implementationPath,schema]){
virtual.clear();
const h=await evaluate(helper,{}),a=await evaluate(argumentsPath,{"../../../../🏗️build/📥️caller/📤️orchestration/🟨️.mjs":h}),i=await evaluate(implementationPath,{}),deps={"./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs":a,"./⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🟨️.mjs":i},before=await evaluate(root,deps);
console.log("[DEBUG] before",mutation.slice(-40),before.nxBuildDependencyImplementationV1().slice(0,8));
virtual.set(mutation,Buffer.concat([readFileSync(mutation),Buffer.from("\n")]));
const later=await evaluate(root,deps);
try{console.log("[DEBUG] later NOTHROW",later.nxBuildDependencyImplementationV1().slice(0,8));}catch(e){console.log("[DEBUG] threw",String(e).slice(0,100));}
}
