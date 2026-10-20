// 🔬 Runs the live inference plugin's projectWithDefaults on the root manifest and dumps the inferred target inputs (baseline before input precision).
import {join} from "node:path";
import {readFileSync} from "node:fs";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const {cacheInternals,createScriptInputCache,prepareSourceAnalysesV1}=inference;
const json=JSON.parse(readFileSync(join(W,"📋️project.json"),"utf8"));
const tax=join(L,"🔣️taxonomy.json");
const t0=performance.now();
const contracts={};
const scripts=createScriptInputCache({workspaceRoot:W});
const out=await cacheInternals.projectWithDefaults(json,".",W,W,contracts,new Map(),undefined,scripts,undefined,"source");
console.error("ms",Math.round(performance.now()-t0));
await Bun.write(process.argv[2]??"/private/tmp/ip/root-source.json",JSON.stringify(out));
process.exit(0);
