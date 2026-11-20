import {constants} from "node:fs";
import {open} from "node:fs/promises";
import {join,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {machinePathV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import type {GenerationHeadCandidateV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts";
import {verifyWholeGenerationAdoptionReceiptV1,wholeGenerationAdoptionReceiptPathV1,type WholeGenerationAdoptionReceiptV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🪪️adoption/🟦️.ts";
import {createFleetRegistryV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import adoptionSchema from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🪪️adoption/🧬️schema/🔣️.json" with {type:"json"};
import fleetSchema from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🧬️schema/🔣️.json" with {type:"json"};
import readySchema from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json" with {type:"json"};
import readyTreeSchema from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧬️schema/🔣️.json" with {type:"json"};
import poolSchema from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧬️schema/🔣️.json" with {type:"json"};

type Input=Readonly<{root:string;store:string;producer:string;destination:string;expectedFile?:string;pinsFile?:string;group?:string;now?:number}>;
type Verdict=Readonly<{status:"verified"|"pending"|"refused";reason:string}>;
type Lookup=(input:Input,context:GenerationHeadCandidateV1)=>Promise<GenerationHeadCandidateV1|undefined>;
type Telemetry=Readonly<{origin:string;updatedMs:number;ready?:boolean;state:string;stage?:string|null;refusal?:unknown;platform?:string;role?:string;totalMegabytes?:number}>;

const origin=new RegExp(adoptionSchema.definitions.Origin.pattern,"u"),limit=1024*1024;
const fresh=(time:number,now:number)=>time<=now&&now-time<=readyTreeSchema.definitions.Policy.const.foregroundFreshMilliseconds;
const evidenceKey=(head:GenerationHeadCandidateV1)=>JSON.stringify([head.origin,head.scope,head.generation,head.manifest,head.identity,head.platform,head.implementation]);

async function readJson(path:string,maximumBytes=limit):Promise<unknown>{
 const file=await open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0)).catch((error:NodeJS.ErrnoException)=>{if(error.code!=="ENOENT")throw error;});
 if(!file)return undefined;
 try{
  const stat=await file.stat();
  if(!stat.isFile()||stat.size>maximumBytes)throw Error("Bounded observation required");
  const data=Buffer.alloc(stat.size+1);
  let size=0;
  while(size<data.length){const part=await file.read(data,size,data.length-size,size);if(!part.bytesRead)break;size+=part.bytesRead;}
  if(size!==stat.size)throw Error("Observation changed");
  return JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(data.subarray(0,size)));
 }finally{await file.close();}
}

async function producerHead(input:Input,context:GenerationHeadCandidateV1):Promise<GenerationHeadCandidateV1|undefined>{
 const [{readLocalFleetCacheFactsV1,resolveRemoteCacheV1,remoteCacheFetchV1},{generationExchangeKeyV1,remoteGenerationLookV1},{httpRemoteExecutionContentV1},policy]=await Promise.all([
  import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts"),
  import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts"),
  import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🗄️content/🟦️.ts"),
  import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧬️schema/🔣️.json",{with:{type:"json"}}),
 ]);
 const copy=readLocalFleetCacheFactsV1(input.root),resolved=copy&&resolveRemoteCacheV1({}, {root:input.root,copy});
 if(resolved?.endpointFrom!=="fleet-facts"||resolved.tokenFrom!=="fleet-identity"||!(copy?.facts.cache.transport==="sealed"||new URL(resolved.contract.endpoint).protocol==="https:"))throw Error("Authenticated fleet cache unavailable");
 const contract=resolved.contract,{probeTimeoutSeconds,probeRetries}=policy.default.definitions.Policy.const;
 const signed=remoteCacheFetchV1(resolved.environment,{root:input.root,base:((request:RequestInfo|URL,options?:RequestInit)=>fetch(request,{...options,redirect:"error"})) as typeof fetch});
 const remote=httpRemoteExecutionContentV1({endpoint:contract.endpoint,token:resolved.token,fetch:signed,timeoutSeconds:probeTimeoutSeconds,retries:probeRetries});
 const key=generationExchangeKeyV1(context.identity,context.platform,input.producer,"all"),deny=async()=>{throw Error("Metadata-only verification required");};
 const content={has:deny,missing:deny,read:deny,stream:deny,write:deny,writeActionResult:deny,readActionResult:async(instance:string,hash:string,signal?:AbortSignal)=>{if(instance!==contract.instance||hash!==key)throw Error("One producer head required");return remote.readActionResult(instance,hash,signal);}};
 const look=await remoteGenerationLookV1({content,instance:contract.instance,key,keys:[key],workspace:input.root,root:input.root,files:{read:async()=>undefined,write:deny,list:async()=>[]},identity:context.identity,platform:context.platform,scope:"all",implementation:context.implementation,requireComplete:true,signal:AbortSignal.timeout(15000)});
 return look.candidate;
}

/** 🧾 Verifies one receiving origin's signed receipt against one independent complete whole head. */
export async function verifyGraphAdoptionV1(input:Input,lookup:Lookup=producerHead):Promise<Verdict>{
 if(!origin.test(input.producer)||!origin.test(input.destination)||input.producer===input.destination)return{status:"refused",reason:"origin-selection"};
 const directory=join(input.store,"v1","origins",input.destination),now=input.now??Date.now();
 try{
  const value=await readJson(wholeGenerationAdoptionReceiptPathV1(input.store,input.destination),adoptionSchema.definitions.Policy.const.maximumBytes);
  if(value===undefined)return{status:"pending",reason:"receipt-missing"};
  if(validateJsonSchemaSubset(adoptionSchema.definitions.Envelope,value,adoptionSchema).length)return{status:"refused",reason:"receipt-shape"};
  const {receipt}=value as {receipt:WholeGenerationAdoptionReceiptV1},claimed=receipt.head;
  if(receipt.destination!==input.destination||claimed.origin!==input.producer||receipt.completedMs>now)return{status:"refused",reason:"receipt-context"};

  const expected=input.expectedFile?await readJson(input.expectedFile,adoptionSchema.definitions.Policy.const.maximumBytes):claimed;
  if(validateJsonSchemaSubset(adoptionSchema.definitions.Head,expected,adoptionSchema).length||evidenceKey(expected as GenerationHeadCandidateV1)!==evidenceKey(claimed))
   return{status:"refused",reason:"expected-head"};
  const [pinFile,ready,pool]=await Promise.all([
   readJson(input.pinsFile??machinePathV1(input.root,".cache","f","cache-server","fleet","pins.json")),
   readJson(join(directory,readySchema.definitions.Policy.const.statusName)),
   readJson(join(directory,poolSchema.definitions.Policy.const.statusName)),
  ]);
  if(pinFile===undefined)return{status:"pending",reason:"pins-missing"};
  if(validateJsonSchemaSubset(fleetSchema.definitions.Pins,pinFile,fleetSchema).length)return{status:"refused",reason:"pins-shape"};
  const {pins}=pinFile as {pins:Record<string,{publicKey:string}>};
  if(!pins[input.destination]?.publicKey||!pins[input.producer]?.publicKey)return{status:"pending",reason:"enrollment-unpinned"};
  const group=input.group??input.producer.split("-",1)[0]!;
  const registry=createFleetRegistryV1({storeRoot:input.store,group,pinned:{[input.destination]:pins[input.destination]!.publicKey,[input.producer]:pins[input.producer]!.publicKey}});
  if(!await verifyWholeGenerationAdoptionReceiptV1(registry,{origin:input.destination,head:claimed},value)||!registry.key(input.producer,"cache-write"))
   return{status:"refused",reason:"enrolled-signature"};

  if(validateJsonSchemaSubset(readySchema.definitions.Status,ready,readySchema).length||validateJsonSchemaSubset(poolSchema.definitions.Status,pool,poolSchema).length)return{status:"refused",reason:"telemetry-shape"};
  const r=ready as Telemetry,p=pool as Telemetry,platform=claimed.platform.startsWith("darwin-")?"macos":claimed.platform.startsWith("win32-")?"windows":claimed.platform.startsWith("linux-")?"linux":undefined;
  if(r.origin!==input.destination||r.platform!==platform||!r.ready||r.state!=="ready"||r.stage!==null||r.refusal!==null||!fresh(r.updatedMs,now))return{status:"refused",reason:"ready-unconfirmed"};
  if(p.origin!==input.destination||p.role!=="puller"||p.totalMegabytes!==16384||!fresh(p.updatedMs,now))return{status:"refused",reason:"physical-memory-unconfirmed"};

  try{
   const head=await lookup(input,expected as GenerationHeadCandidateV1);
   if(!head)return{status:"pending",reason:"producer-head-missing"};
   if(!await verifyWholeGenerationAdoptionReceiptV1(registry,{origin:input.destination,head},value))return{status:"refused",reason:"producer-head-mismatch"};
   return{status:"verified",reason:"exact-head-and-reported-16gib"};
  }catch{return{status:"pending",reason:"producer-cache-unavailable"};}
 }catch{return{status:"refused",reason:"observation-unreadable"};}
}

if(import.meta.main){
 const [store,producer,destination,expected,pinsFile,group,...extra]=process.argv.slice(2);
 if(!store||!producer||!destination||extra.length){
  console.log(JSON.stringify({status:"refused",reason:"usage",arguments:["store","producer","destination","expected-head.json-or--","pins.json","group"]}));
  process.exitCode=1;
 }else{
  const result=await verifyGraphAdoptionV1({root:process.cwd(),store:resolve(store),producer,destination,...expected&&expected!=="-"?{expectedFile:resolve(expected)}:{},...pinsFile?{pinsFile:resolve(pinsFile)}:{},group});
  console.log(JSON.stringify(result));
  process.exitCode=result.status==="verified"?0:result.status==="pending"?2:1;
 }
}
