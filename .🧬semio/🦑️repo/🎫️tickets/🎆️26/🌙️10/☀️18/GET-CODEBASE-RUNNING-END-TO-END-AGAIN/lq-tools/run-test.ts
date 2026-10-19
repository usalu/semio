import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const R=join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const {createBuildBootstrapCapabilitiesV1}=await import(join(L,"🏗️build/📥️caller/📤️child/🟦️.ts"));
const {readDefaultBuildCliPolicyV1}=await import(join(L,"🏗️build/📜️script.ts"));
const routing=await import(R);
const {runOwnedCommand}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts"));
const args=["repo","show","projects"];
const capabilities=await createBuildBootstrapCapabilitiesV1(root,args,process.env);
const envelope=routing.createScriptProcessEnvelope(readDefaultBuildCliPolicyV1(),capabilities,Date.now());
await routing.withScriptProcessEnvelope(envelope,async (original:any)=>{
  const env=routing.scriptInvocationEnvironment(original,{...process.env});
  await runOwnedCommand(process.execPath,["test",...process.argv.slice(2)],root,"lq:test",routing.scriptInvocationBudget(original,900000),{env,signal:original.control.signal});
});
