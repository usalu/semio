import {join} from "node:path";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const observation:any=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const file=join(W,process.argv[2]!);
console.log(JSON.stringify(observation.observedSourceAnalysisV1(inference.prepareSourceAnalysesV1().command,file)).slice(0,1500));
process.exit(0);
