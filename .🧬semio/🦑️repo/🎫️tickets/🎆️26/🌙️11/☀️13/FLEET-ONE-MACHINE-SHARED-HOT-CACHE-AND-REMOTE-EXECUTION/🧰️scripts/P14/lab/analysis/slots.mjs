const R="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state";
const w=await import(R+"/🙋️wanted/🟦️.ts"),x=await import(R+"/🛰️exchange/🟦️.ts");
for(const [req,plat] of [["g1-m04","linux-x64"],["g1-m04","darwin-arm64"],["g1-m03","darwin-arm64"],["g1-m06","darwin-arm64"],["g1-m02","darwin-arm64"]]){
 console.log(req,plat,x.graphEntryKeyV1(w.wantedStateV1(req,plat)));
}
