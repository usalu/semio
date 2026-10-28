import{join,resolve}from"node:path";
import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{nxAuthorizerMemoBaseV1,nxAuthorizerMemoFileV1,readNxAuthorizerMemoV1}from"../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧊️memo/🟦️.ts";
import{buildSourceClosureStreamLoaderV1}from"../../📥️source/🔏️closure/🟦️.ts";
import closureSchema from"../../📥️source/🔏️closure/🧬️schema/🔣️.json"with{type:"json"};
import{sealedEntryOfV1,sealedPolicyV1}from"../🟦️.ts";
import{resolveSealedImplementationV1,type SealedCandidateV1,type SealedResolutionV1}from"../📤️promotion/🟦️.ts";

/** 🔁️ Promotes the live tree of a workspace to the sealed snapshot of the one entry, from LIVE code and without an invocation envelope: the front-door trampoline calls this when the tree drifted, so a bug or a half-finished edit in the promoter can only stop a promotion (the last good snapshot keeps running and the failure is reported) and can never stop a command, and a promoter that is itself fixed in the live tree is used the moment it is fixed (a sealed promoter could not heal itself). It captures like the compiler capture does (same closure, same file rows, same memo), then lets the store decide: re-check after the lock, parse, boot, publish.
 * @see ../🟦️.ts the store, ../📤️promotion/🟦️.ts the protocol, ../🚦️trampoline/🟦️.ts the caller */
export async function promoteLiveTreeV1(root:string,entry:string,environment:Readonly<Record<string,string|undefined>>,write:(text:string)=>void,signal?:AbortSignal):Promise<SealedResolutionV1>{
 const workspace=resolve(root),store=sealedEntryOfV1(workspace,entry),memo=nxAuthorizerMemoFileV1(nxAuthorizerMemoBaseV1(environment),workspace,entry,true);
 const capture=async(stop:AbortSignal):Promise<SealedCandidateV1>=>{
  const captured=await captureNxAuthorizerSourcesV1(workspace,entry,stop,{installed:true,memo:{file:memo}}),bytes=nxAuthorizerCapturedBytesV1(captured),imports=nxAuthorizerCapturedImportsV1(workspace,captured),policy=closureSchema.definitions.Policy.const;
  const files:SealedCandidateV1["snapshot"]["files"][number][]=[];let total=0;
  for(const row of captured.manifest.sources){
   const value=bytes.get(row.path);
   if(!value||join(workspace,row.path).length>sealedPolicyV1.maximumPathCharacters||value.byteLength>policy.maximumFileBytes)throw Error("Bounded admitted compiler image required");
   total+=value.byteLength;if(total>policy.maximumSourceBytes||files.length>=policy.maximumFiles)throw Error("Bounded admitted compiler closure required");
   files.push(Object.freeze({...row,base64:Buffer.from(value).toString("base64"),imports:imports.get(row.path)!}));
  }
  const stored=await readNxAuthorizerMemoV1(memo);
  return{snapshot:Object.freeze({version:1 as const,root:workspace,entry,files:Object.freeze(files)}),...(stored?{structure:{directories:stored.directories,observations:stored.observations}}:{})};
 };
 return resolveSealedImplementationV1(workspace,store,{capture,loader:buildSourceClosureStreamLoaderV1,environment,...(signal?{signal}:{}),write});
}
