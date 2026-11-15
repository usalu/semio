import {readFileSync} from "node:fs";
const root="/Users/Shared/semio",lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inf=await import(root+"/"+lib+"/⚡️caching/📥️inference/🟨️.mjs");
const f=root+"/"+lib+"/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";const t=readFileSync(f,"utf8");
const edits:Record<string,string>={same:t,comment:t+"\n// harmless comment\n",body:t.replace(/const /,"const zz = 1; const "),str:t+"\nconst q = 'x/y.ts';\n",blank:"\n\n"+t};
for(const [k,v] of Object.entries(edits)){const o={cmd:inf.commandSourceImports(f,v),url:inf.commandUrlReferences(f,v),rrefs:inf.runtimeModuleSourceReferences(f,v),mod:[inf.moduleSourceImports(f,v),inf.runtimeModuleSourceImports(f,v),inf.productionModuleSourceImports(f,v)]};console.log(k,Bun.hash(JSON.stringify(o)).toString(16),JSON.stringify(o).length);}
const o=inf.runtimeModuleSourceReferences(f,t);console.log(JSON.stringify(o).slice(0,600));console.log(JSON.stringify(inf.commandSourceImports(f,t)).slice(0,400));
