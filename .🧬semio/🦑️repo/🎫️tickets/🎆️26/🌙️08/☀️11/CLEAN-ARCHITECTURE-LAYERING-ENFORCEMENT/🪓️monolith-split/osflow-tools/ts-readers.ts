// 🧪️ Calls the Rust-source-reading Flow TS contracts directly, outside the repo launcher.
const F="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/";
const runs:[string,(m:any)=>unknown][]=[
 ["📔️registry/🧪️tests/🪪️manifest-admission/🟦️.ts",m=>m.flowExtensionManifestAdmissionSelfTests()],
 ["🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts",_=>0],
 ["🕸️wasm/🧪️tests/🎟️retained-receipt/🟦️.ts",m=>m.testFlowRetainedReceipt()],
 ["🕸️wasm/🧪️tests/🎟️retained-grant/🟦️.ts",m=>m.testFlowRetainedGrant()],
 ["🖥️host/🧹️retirement/🧪️tests/🎟️host-source/🟦️.ts",m=>m.testHostSourceCustodyContract()],
 ["🌿️vcs/♻️retirement/🧪️tests/🟦️.ts",m=>m.testOriginalVcsClosure()],
 ["🌐️geometry/🧪️tests/🎟️caller/🟦️.ts",_=>0],
];
const only=process.argv[2];
for(const [f,call] of runs){
 if(only&&!f.includes(only))continue;
 try{const m=await import(F+f);await call(m);console.log("OK  ",f)}catch(e){console.log("FAIL",f,String(e).slice(0,100))}
}
