
const admissionCacheV1=admissionSchema.$defs.admissionCache.const;
const admissionValidatorsV1:{receipt?:{validate(value:unknown):string[]};facts?:{validate(value:unknown):string[]};admission?:{validate(value:unknown):string[]}}={};
const admissionReceiptValidatorV1=()=>admissionValidatorsV1.receipt??=compileJsonSchemaSubset({$defs:admissionSchema.$defs,$ref:"#/$defs/admissionReceipt"});
const ownerFactsValidatorV1=()=>admissionValidatorsV1.facts??=compileJsonSchemaSubset({$defs:admissionSchema.$defs,$ref:"#/$defs/ownerFacts"});
const admissionValidatorV1=()=>admissionValidatorsV1.admission??=compileJsonSchemaSubset(admissionSchema);

/** 🔭️ Names the requester that first admitted an owner: nothing for the selection, `catalog` for a provider, or the owner and optionally its task. */
export type BuildOwnerAdmissionOriginV1=null|"catalog"|{owner:string;target?:string};

/** 🗃️ Locates one owner observation's persisted facts: the leaf is domain separated from project assembly, the key binds implementation, scope, registry, owner and the closure's canonical task and input order. */
export function buildOwnerAdmissionRecordV1(implementation:string,scope:string,registry:string,owner:{name:string;root:string;manifest:string},targets:readonly string[],inputs:readonly string[]):{leaf:string;key:unknown[]}{
 const digest=createHash("sha256").update([admissionCacheV1.recordNamespace,owner.root,JSON.stringify([targets,inputs])].join("\0")).digest("hex").slice(0,admissionCacheV1.recordLeafDigits);
 return {leaf:admissionCacheV1.recordDirectory+"/"+digest+".json",key:[admissionCacheV1.recordVersion,implementation,scope,registry,owner.name,owner.root,owner.manifest,[...targets],[...inputs]]};
}

/** 🧲️ Supplies the caller-owned storage, observation, progress and cancellation ports of the per-owner admission records. */
export interface BuildOwnerAdmissionPortsV1{directory:string;implementation:string;scope:string;registry:string;check():void;advance(stage:string):Promise<void>;observe<T>(options:{cachePath:string;key:unknown;owner:string;reusable:(value:unknown)=>boolean},compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>}

/** 🪶️ Keeps exactly what the closure reads of an owner's facts, as the plain JSON a persisted record returns, so a hit equals a fresh observation and no record carries generated target bodies. */
export function slimBuildOwnerAdmissionFactsV1(facts:BuildOwnerAdmissionFactsV1):BuildOwnerAdmissionFactsV1{
 return JSON.parse(JSON.stringify({...facts,project:{name:facts.project.name,targets:Object.fromEntries(Object.entries(facts.project.targets).map(([target,row])=>[target,row.dependsOn===undefined?{}:{dependsOn:row.dependsOn}]))}})) as BuildOwnerAdmissionFactsV1;
}

/** 🧲️ Reuses a persisted owner observation while every source fact it consumed holds; a miss computes and persists it, so a cancelled closure resumes with its finished owners. */
export async function observeBuildOwnerAdmissionV1(ports:BuildOwnerAdmissionPortsV1,owner:{name:string;root:string;manifest:string},targets:readonly string[],inputs:readonly string[],compute:()=>Promise<BuildOwnerAdmissionFactsV1>):Promise<BuildOwnerAdmissionFactsV1>{
 const record=buildOwnerAdmissionRecordV1(ports.implementation,ports.scope,ports.registry,owner,targets,inputs);
 ports.check();await ports.advance(admissionCacheV1.progressStage);
 const facts=await ports.observe({cachePath:join(ports.directory,record.leaf),key:record.key,owner:admissionCacheV1.recordNamespace+":"+owner.name,reusable:value=>ownerFactsValidatorV1().validate(value).length===0},async()=>slimBuildOwnerAdmissionFactsV1(await compute()));
 ports.check();return facts;
}

/** 🧾️ Locates the scope's persisted admission receipt next to its graph publication. */
export function buildAdmissionReceiptPathV1(directory:string,scope:string):string{
 if(!/^[a-f0-9]{64}$/u.test(scope))throw Error("Closed build graph scope required");return join(directory,scope+admissionCacheV1.receiptSuffix);
}

/** 🧾️ A closed receipt: the closure and every original fact it was derived from. */
export interface BuildAdmissionReceiptV1{version:1;scope:string;module:{implementation:string;publication:string};startedAt:number;admission:BuildOwnerAdmissionV1;observations:readonly unknown[]}

/** 📖️ Serializes only a closed receipt whose closure is itself a closed admission. */
export function createBuildAdmissionReceiptV1(value:BuildAdmissionReceiptV1):string{
 if(admissionReceiptValidatorV1().validate(value).length||admissionValidatorV1().validate(value.admission).length)throw Error("Closed build admission receipt required");
 const text=JSON.stringify(value);if(Buffer.byteLength(text)>admissionCacheV1.maximumReceiptBytes)throw Error("Build admission receipt exceeds its finite publication limit");return text;
}

/** 📖️ Admits a receipt only when it is closed and was issued for this scope by this exact loaded producer. */
export function readBuildAdmissionReceiptV1(text:string,expected:{scope:string;module:{implementation:string;publication:string}}):BuildAdmissionReceiptV1|undefined{
 let value:unknown;try{value=JSON.parse(text);}catch{return undefined;}
 if(admissionReceiptValidatorV1().validate(value).length)return undefined;const receipt=value as BuildAdmissionReceiptV1;
 if(receipt.scope!==expected.scope||receipt.module.implementation!==expected.module.implementation||receipt.module.publication!==expected.module.publication||admissionValidatorV1().validate(receipt.admission).length)return undefined;
 return receipt;
}

/** ⏳️ Withholds a receipt whose facts could still be half-written: no consumed file may be younger than the racy window before the closure started. */
export function buildAdmissionReceiptPublishableV1(observations:readonly unknown[],startedAt:number):boolean{
 const limit=startedAt-admissionCacheV1.racyMilliseconds;
 return !observations.some(row=>{const {kind,fact}=row as {kind:string;fact:{mtimeMs?:number;ctimeMs?:number;source?:{mtimeMs?:number;ctimeMs?:number}}|undefined},stamp=kind==="file"?fact:kind==="analysis"?fact?.source:undefined;return stamp!==undefined&&((stamp.mtimeMs??0)>=limit||(stamp.ctimeMs??0)>=limit);});
}

/** 🔭️ Lists the requesters from an owner back to the selection, so a failing owner names the task that pulled it into the closure. */
export function buildOwnerAdmissionChainV1(requestedBy:ReadonlyMap<string,BuildOwnerAdmissionOriginV1>,name:string):string[]{
 const chain=[name],seen=new Set([name]);let owner=name;
 for(let hop=0;;hop++){
  const origin=requestedBy.get(owner)??null;
  if(origin===null){chain.push("selection");return chain;}
  if(origin==="catalog"){chain.push("catalog provider");return chain;}
  chain.push(origin.target===undefined?origin.owner:origin.owner+":"+origin.target);
  if(seen.has(origin.owner))return chain;
  if(hop+1>=admissionCacheV1.chainLimit){chain.push("...");return chain;}
  seen.add(origin.owner);owner=origin.owner;
 }
}

/** 🛡️ Appends the admission chain to a failing owner observation's message once, keeping its identity, class and code, and leaving cancellation untouched so it stays what it is. */
export function annotateBuildOwnerAdmissionFailureV1(error:unknown,name:string,requestedBy:ReadonlyMap<string,BuildOwnerAdmissionOriginV1>):unknown{
 if(!(error instanceof Error)||Object.hasOwn(error,"admissionChain")||error.name==="AbortError"||error.name==="TimeoutError"||/abort|cancel/iu.test(error.message))return error;
 const chain=buildOwnerAdmissionChainV1(requestedBy,name);
 try{Object.defineProperty(error,"admissionChain",{value:chain,enumerable:false});error.message+="\nBuild owner admission chain: "+chain.join(" <- ");}catch{}
 return error;
}
