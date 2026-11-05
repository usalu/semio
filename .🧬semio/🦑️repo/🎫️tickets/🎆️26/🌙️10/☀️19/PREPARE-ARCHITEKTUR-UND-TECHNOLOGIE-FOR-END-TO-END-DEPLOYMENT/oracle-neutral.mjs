import assert from "node:assert/strict";
import {readFileSync} from "node:fs";

const root=new URL("../../../../../../../",import.meta.url),read=path=>JSON.parse(readFileSync(new URL(path,root),"utf8"));
const process="🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/",suite="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/";
const{resolveTestToolV1}=await import(new URL(process+"🟦️.ts",root)),{pinnedTestToolVariableV1}=await import(new URL(process+"📌️pinned/🟦️.ts",root)),{readWorkspaceTestPythonOutputV1}=await import(new URL(suite+"🐍️python/🟦️.ts",root));
const variable=pinnedTestToolVariableV1("python"),bound=read(process+"🧪️tests/🧫️fixtures/🏷️bound.json"),maximum=read(process+"🔣️.json").binding.maximumPathUnits;
for(const row of bound){
 let probes=0;const result=resolveTestToolV1("python",{platform:row.platform,environment:{[variable]:row.binding},which:()=>{throw Error("Bound tool consulted PATH");},run:(command,args)=>{assert.equal(command,row.binding);assert.deepEqual(args,["--version"]);probes++;return{status:row.status,stdout:row.stdout};},write:()=>{}});
 assert.equal(result.path,row.expected,row.name);assert.equal(probes,row.probes,row.name);assert.deepEqual(result.tried,[variable]);
}
const results=read(suite+"🧪️tests/🧫️fixtures/🐍️python.json");
for(const row of results)if(row.expected===null)assert.throws(()=>readWorkspaceTestPythonOutputV1(row.output,row.directory,row.platform));else assert.equal(readWorkspaceTestPythonOutputV1(row.output,row.directory,row.platform),row.expected,row.name);
for(const glyph of["x","😀","🐍️"]){
 const budget=maximum-1,exact="/"+glyph.repeat(Math.floor(budget/glyph.length))+"x".repeat(budget%glyph.length);
 for(const[path,valid]of[[exact,true],[exact+glyph,false]]){let probes=0;const result=resolveTestToolV1("python",{platform:"linux",environment:{[variable]:path},which:()=>{throw Error("Bound tool consulted PATH");},run:()=>{probes++;return{status:0,stdout:"Python 3.14.0"};},write:()=>{}});assert.equal(result.path,valid?path:null);assert.equal(probes,valid?1:0);}
 const prefix="/managed/",room=read(suite+"🐍️python/🔣️.json").maximumPathUnits-prefix.length,path=prefix+glyph.repeat(Math.floor(room/glyph.length))+"x".repeat(room%glyph.length);
 assert.equal(readWorkspaceTestPythonOutputV1(path+"\n","/managed","linux"),path);assert.throws(()=>readWorkspaceTestPythonOutputV1(path+glyph,"/managed","linux"));
}
const expected=read(suite+"🧪️tests/🧫️fixtures/🩺️preflight.json"),table=read(suite+"🔣️.json"),project=read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json"),owners=table.suites.filter(row=>row.trees?.includes(expected.tree)),owner=owners[0];
assert.equal(owners.length,1);assert.deepEqual({suite:owner.id,tree:owner.trees.join(","),platforms:owner.platforms,tools:owner.tools??[],target:project.targets[expected.target]?.options.command.endsWith(expected.suite)?expected.target:null},expected);assert.deepEqual(project.targets[expected.target].dependsOn,["workspace:deps-python"]);
console.log("[DEBUG] Portable gate, bound-tool8, managed-result9 and UTF-16 boundaries pass; Ajv remains a separate pending check");
