/** bun closure-native.ts [out.json] : writes the cargo script `native` command closure (scheduling stripped) as JSON and prints test-file counts and the non-test importers of test files. */
import {resolve} from "node:path";
import {readFileSync,writeFileSync} from "node:fs";
const root=process.cwd(),lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const inference=await import(resolve(root,lib+"🏗️build/🕸️graph/📥️inference/🟨️.mjs")) as any;await inference.libraryBootstrap;
const scripts=await import(resolve(root,lib+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs")) as any;
const script=resolve(root,lib+"⚡️caching/🦀️cargo/📜️script.ts");
const cache=inference.createScriptInputCache({workspaceRoot:root});
const rows=(await scripts.relativeScriptInputs([script],root,cache,true,[],new Map([[script,["native"]]]))).filter((row:any)=>typeof row==="string");
const raw:string[]=rows.map((row:string)=>row.replace("{workspaceRoot}/",""));if(process.argv[3])writeFileSync(process.argv[3],JSON.stringify(raw));
const files:string[]=scripts.withoutSchedulingV1(rows).map((row:string)=>row.replace("{workspaceRoot}/",""));
if(process.argv[2])writeFileSync(process.argv[2],JSON.stringify(files));
const tests=files.filter(file=>file.includes("/🧪️tests/")||file.includes("/🧪️test/"));
console.log("native closure",files.length,"in tests dirs",tests.length);
const testDirs=new Set(tests.map(file=>file.slice(0,file.indexOf("/🧪️test")+"/🧪️tests".length)));
console.log("test dirs",testDirs.size);
const importers=new Map<string,string[]>();
for(const file of files.filter(file=>!tests.includes(file)&&/\.(ts|mjs|mts)$/.test(file))){
 const text=readFileSync(resolve(root,file),"utf8");
 for(const match of text.matchAll(/from\s*["']([^"']*🧪️test[^"']*)["']|import\(\s*["']([^"']*🧪️test[^"']*)["']\s*\)/gu)){const key=match[1]??match[2]!;importers.set(file,[...(importers.get(file)??[]),key]);}
}
for(const [file,specifiers] of importers)console.log("IMPORTER",file.slice(-110),"->",specifiers.map(item=>item.slice(-70)).join(" | "));
