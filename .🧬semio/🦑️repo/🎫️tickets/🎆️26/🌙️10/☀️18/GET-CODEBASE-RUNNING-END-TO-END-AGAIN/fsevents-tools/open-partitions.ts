import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const R=join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const {createBuildBootstrapCapabilitiesV1}=await import(join(L,"🏗️build/📥️caller/📤️child/🟦️.ts"));
const {readDefaultBuildCliPolicyV1}=await import(join(L,"🏗️build/📜️script.ts"));
const {openRepositoryCachePartitionV1}=await import(join(L,"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"));
const routing=await import(R);
const ids=process.argv.slice(2);
const capabilities=await createBuildBootstrapCapabilitiesV1(root,["repo","tools","setup","all"],process.env);
const envelope=routing.createScriptProcessEnvelope(readDefaultBuildCliPolicyV1(),capabilities,Date.now());
await routing.withScriptProcessEnvelope(envelope,async (original:any)=>{
  const control=routing.createIssuedResourceControlV1(original,"fsevents:open",0);
  for(const id of ids){const started=Date.now();const image=await openRepositoryCachePartitionV1(root,id,control);console.log("[fsevents] opened",id,Date.now()-started,"ms");await image.close();}
});
