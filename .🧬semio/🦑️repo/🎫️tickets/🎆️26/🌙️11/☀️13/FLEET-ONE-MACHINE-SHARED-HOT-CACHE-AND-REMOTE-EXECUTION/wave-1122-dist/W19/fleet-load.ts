const {readRemoteExecutionFleetFactsV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚢️status/🟦️.ts");
const facts=await readRemoteExecutionFleetFactsV1({root:"/Users/ueli/Documents/semio",environment:process.env,timeoutMilliseconds:8000});
console.log("[DEBUG] treeCode",facts?.treeCode);
for(const m of facts?.machines??[])console.log("[DEBUG]",m.origin,m.platform,"code",m.code,"worker",m.worker,JSON.stringify(m.why??null).slice(0,300),"hit",m.cacheHit,"miss",m.cacheMiss,"age",m.ageSeconds);
