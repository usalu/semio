// compares a derived-walk question on the real repository: direct deriveClosureFactV1 vs the closure settler (cold, then cached, then durable)
import{mkdtempSync,rmSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";
const G="/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph";
const inference=await import(G+"/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const observation=await import("/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs"),scriptInputs=await import(G+"/📥️inference/🧭️script-inputs/🟨️.mjs");
const {createClosureSettlerV1}=await import(G+"/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs"),{createNodeStoreV1}=await import(G+"/🧮️derived-nodes/💾️store/🟨️.mjs");
const root="/home/deck/semio",skip=JSON.parse(process.argv[2]??'{"paths":[],"names":[".git","node_modules","target",".cache",".venv",".r"]}');
const row={kind:"derived",path:JSON.stringify({derive:1,workspaceRoot:root,recipe:{fn:"crate-index",skip}}),fact:"none"};
const derive=asked=>observation.observeInferenceSnapshotV1(async()=>scriptInputs.deriveClosureFactV1(asked,inference.createScriptInputCache({workspaceRoot:root})));
const hold=rows=>observation.observeInferenceSnapshotV1(async()=>observation.observationRowsHoldV1(rows));
const time=async work=>{const at=performance.now(),value=await work();return[value,Math.round(performance.now()-at)];};
const dir=mkdtempSync(join(tmpdir(),"real-closures-"));
try{
 const [direct,directMs]=await time(()=>derive(row));
 const settler=createClosureSettlerV1({store:createNodeStoreV1({root:dir})}),ask=(s,fact)=>s.settle({pseudoRows:new Map([["q",{...row,fact}]]),derive,hold});
 const [cold,coldMs]=await time(()=>ask(settler,"none"));
 const [warm,warmMs]=await time(()=>ask(settler,direct.fact));
 const [durable,durableMs]=await time(()=>ask(createClosureSettlerV1({store:createNodeStoreV1({root:dir})}),direct.fact));
 console.log(JSON.stringify({directFact:direct.fact,directRows:direct.rows.length,directMs,coldFact:cold.answers.get("q").fact,coldEqual:cold.answers.get("q").fact===direct.fact,coldMs,warmEqual:warm.held.has("q"),warmReused:warm.reused,warmMs,durableEqual:durable.held.has("q"),durableReused:durable.reused,durableMs}));
}finally{rmSync(dir,{recursive:true,force:true});}
