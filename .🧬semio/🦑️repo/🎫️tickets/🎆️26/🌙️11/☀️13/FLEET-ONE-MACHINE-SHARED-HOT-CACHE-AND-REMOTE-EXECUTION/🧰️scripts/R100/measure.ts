/** 📏️ R100: how many rows of the build library one real owner closure carries: the late closure of three real targets, resolved the way a project record holds it. usage: bun measure.ts */
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference:any=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const observation:any=await import(B+"👁️observation/🟨️.mjs"),scripts:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const targets:[string,any][]=[
 ["🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript",{options:{command:"repo ./📜️script.ts prepare stdio-deflate wgpu prototype",cwd:"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"}}],
 ["🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust",{options:{command:"repo ./📜️script.ts test",cwd:"🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust"}}],
 ["🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",{options:{command:"repo ./📜️script.ts test-session-return source",cwd:"🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"}}]];
for(const [root,target] of targets){
 const out=await observation.observeInferenceSnapshotV1(async()=>{
  const cache=scripts.createScriptInputCache({workspaceRoot:R,lateClosures:true}),api=scripts.closureApiV1(cache,R);
  const handle=scripts.closureHandleOfV1(await api.nativeTargetCommandInputs(target,await api.nativeCommandInputs(),root)),result=await api.resolve([[handle.key,handle.recipe]]);
  const all=[...result.rows,...result.derived.flatMap((item:any)=>item.support)];
  const lib=all.filter((row:any)=>["analysis","file","field"].includes(row.kind)&&String(row.path).includes("📚️library/"));
  return{rows:result.rows.length,derived:result.derived.length,support:result.derived.reduce((sum:number,item:any)=>sum+item.support.length,0),libraryContentRows:lib.length,files:(result.values.get(handle.key)as any[]).filter(x=>typeof x==="string").length};
 });
 console.log(JSON.stringify({root:root.slice(-40),...out}));
}
process.exit(0);
