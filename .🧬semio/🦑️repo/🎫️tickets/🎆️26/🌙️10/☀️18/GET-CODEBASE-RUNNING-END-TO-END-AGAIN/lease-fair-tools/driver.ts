import {scriptCompilerAdmissionControlV1} from "./🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "./🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
const suite=process.argv[2]!;const out=process.argv[3]!;
const mod=await import("./🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts");
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"lease-fair-driver",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{
 await mod.testResourceLeases(out,scriptCompilerAdmissionControlV1(invocation,"test:resource-leases",0),invocation);
});
console.log("DRIVER-OK");
