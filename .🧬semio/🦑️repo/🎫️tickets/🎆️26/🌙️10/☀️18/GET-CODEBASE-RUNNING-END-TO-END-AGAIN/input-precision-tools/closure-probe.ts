// 🔬 Root script command closure size with the live production inference module.
import {join} from "node:path";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const {createScriptInputCache,relativeScriptInputs}=inference;
const cache=createScriptInputCache({workspaceRoot:W});
const t0=performance.now();
const files:string[]=await inference.cacheInternals.relativeScriptInputs?.([join(W,process.argv[2]??"📜️script.ts")],W,cache,true)??await relativeScriptInputs([join(W,process.argv[2]??"📜️script.ts")],W,cache,true);
console.log(files.length,Math.round(performance.now()-t0),"ms");
await Bun.write(process.argv[3]??"/private/tmp/ip/closure.txt",files.join("\n"));
process.exit(0);
