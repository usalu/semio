import{createIssuedResourceControlV1}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{statfs}from"node:fs/promises";
import{receiveScriptProcessInvocation}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{readNativeCallerAuthorityV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import{withRepositoryToolingStoreV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import{retireManagedToolingGenerationsV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts";
import schema from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧬️schema/🔣️.json"with{type:"json"};
const command=process.argv[2];if(process.argv.length!==3||!["preview","prune"].includes(command!))throw Error("Expected exact tooling maintenance command");
await receiveScriptProcessInvocation(process.env,async original=>{
 const root=readNativeCallerAuthorityV1(original.capabilities).repositoryRoot;
 return withRepositoryToolingStoreV1(root,original,createIssuedResourceControlV1(original,"cache:entry",0),async store=>{
  await store.recheck();const facts=await statfs(store.root,{bigint:true}),request={version:1 as const,minimumAvailableBytes:schema.definitions.Protocol.const.minimumInstallerAvailableBytes};
  const result=command==="prune"?await retireManagedToolingGenerationsV1(root,store,request):{retired:[],availableBytes:Number(facts.bavail*facts.bsize)};
  await store.recheck();console.log("[DEBUG] Actual held tools8 maintenance completed "+JSON.stringify({command,root:store.root,minimumAvailableBytes:request.minimumAvailableBytes,...result}));
 });
});
