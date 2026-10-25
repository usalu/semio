// usage: bun derive-fragment.mjs <projectRoot> <target> <out.json> [representation]  -> fresh project node (inputs, namedInputs) of one project from the CURRENT policy/engine, no graph refresh
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const ws="/Users/ueli/Documents/semio";
const L=ws+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inference=await import(L+"/🏗️build/🕸️graph/📥️inference/🟨️.mjs");
await inference.libraryBootstrap;
const ci=inference.cacheInternals;
const [root,target,out,rep="hash"]=process.argv.slice(2);
const json=JSON.parse(readFileSync(join(ws,root,"📋️project.json"),"utf8"));
const contracts=JSON.parse(readFileSync(L+"/🔣️taxonomy.json","utf8")).generatorContracts;
const scripts=inference.createScriptInputCache({workspaceRoot:ws});
const t0=performance.now();
const project=await ci.projectWithDefaults(json,root,join(ws,root),ws,contracts,new Map(),undefined,scripts,undefined,rep,[target]);
console.log("derived in",Math.round(performance.now()-t0),"ms",root,Object.keys(project.targets));
writeFileSync(out,JSON.stringify({root,name:json.name,node:project}));
