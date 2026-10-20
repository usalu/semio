import {lstatSync,readFileSync,readdirSync,unlinkSync} from "node:fs";
import {cp} from "node:fs/promises";
import {join,relative,sep} from "node:path";
import {writeJavascriptDependencyGenerationFilesV1} from "../../🧾️generation/💾️storage/🟦️.ts";
import {generationSha256V1} from "../../🧾️generation/🟦️.ts";
import {adoptClonedRootsV1,adoptCloneCommandV1,adoptDonorQualifiesV1,adoptProtocolV1 as protocol,installProjectionSha256V1,parseAdoptReceiptV1,sealAdoptReceiptV1,type AdoptFileV1,type AdoptReceiptV1,type AdoptRuntimeV1} from "../🟦️.ts";

const directory=(storeRoot:string)=>join(storeRoot,protocol.directory);
const names=(storeRoot:string,prefix:string):string[]=>{
 try{return readdirSync(directory(storeRoot)).filter(name=>name.startsWith(prefix)&&name.endsWith(protocol.adopt.receiptSuffix)).map(name=>({name,at:lstatSync(join(directory(storeRoot),name)).mtimeMs})).sort((left,right)=>right.at-left.at).map(row=>row.name);}catch{return[];}
};
const read=(storeRoot:string,name:string):AdoptReceiptV1|undefined=>{
 try{const path=join(directory(storeRoot),name),facts=lstatSync(path);if(!facts.isFile()||facts.size>protocol.adopt.maximumReceiptBytes)return undefined;return parseAdoptReceiptV1(readFileSync(path));}catch{return undefined;}
};
const present=(storeRoot:string):Set<string>=>{try{return new Set(readdirSync(join(storeRoot,"j")));}catch{return new Set();}};

/** 🔑️ Computes the install projection digest of a staged generation from its absolute input paths. */
export const adoptProjectionOfV1=(generationDirectory:string,files:readonly AdoptFileV1[]):string=>
 installProjectionSha256V1(files.map(file=>({path:relative(generationDirectory,file.path).split(sep).join("/"),bytes:file.bytes})));
/** 🔎️ Tells whether any receipt already attests this generation. */
export const hasJavascriptDependencyAdoptReceiptV1=(storeRoot:string,generation:string):boolean=>names(storeRoot,generation.slice(0,20)+".").some(name=>read(storeRoot,name)?.generation===generation);
/** 🔏️ Seals the adoption receipt of a complete installed generation and retires the receipts of generations that no longer exist; failure only costs a later installation. */
export async function sealJavascriptDependencyAdoptReceiptV1(storeRoot:string,body:Omit<AdoptReceiptV1,"sha256">,checkpoint:()=>void,yieldContinuation:()=>Promise<void>):Promise<string>{
 const receipt=sealAdoptReceiptV1(body),name=body.generation.slice(0,20)+"."+receipt.sha256+protocol.adopt.receiptSuffix;
 await writeJavascriptDependencyGenerationFilesV1(storeRoot,[{name,bytes:Buffer.from(JSON.stringify(receipt)+"\n")}],checkpoint,yieldContinuation);
 const alive=present(storeRoot);
 for(const other of names(storeRoot,""))if(other!==name&&(other.slice(0,20)===body.generation.slice(0,20)||!alive.has(other.slice(0,20))))try{unlinkSync(join(directory(storeRoot),other));}catch{}
 return receipt.sha256;
}
/** 🔍️ Lists the receipts of earlier generations that qualify as donors, newest first. */
export function findJavascriptDependencyAdoptDonorsV1(storeRoot:string,wanted:Readonly<{generation:string;projectionSha256:string;runtime:AdoptRuntimeV1}>):readonly AdoptReceiptV1[]{
 const alive=present(storeRoot),rows:AdoptReceiptV1[]=[];
 for(const name of names(storeRoot,"")){const receipt=read(storeRoot,name);if(receipt&&alive.has(receipt.generation.slice(0,20))&&adoptDonorQualifiesV1(receipt,wanted))rows.push(receipt);}
 return Object.freeze(rows);
}

/** 🪞️ Carries every effect an adoption needs, so the flow stays testable and every long step stays cancellable. */
export interface AdoptRequestV1{
 readonly storeRoot:string;readonly generation:string;readonly directory:string;readonly files:readonly AdoptFileV1[];readonly members:number;readonly runtime:AdoptRuntimeV1;
 readonly check:()=>void;
 readonly verifyDonor:(donor:string)=>Promise<Readonly<{sha256:string;shape:Readonly<{entries:number;directories:number;files:number;links:number;sha256:string}>}>|undefined>;
 readonly pin:(donor:string)=>Promise<void>;
 readonly run:(argv:readonly string[])=>Promise<boolean>;
 readonly scan:()=>Promise<Readonly<{entries:number;directories:number;files:number;links:number;sha256:string}>>;
 readonly remove:(path:string)=>Promise<void>;
}
/** 🧬️ Gives a fresh generation the installed trees of a verified install-equivalent donor by copy-on-write clone, proves the clone has the donor's exact shape and removes every partial clone before reporting that a real installation is needed. */
export async function adoptJavascriptDependencyInstalledTreeV1(request:AdoptRequestV1):Promise<string|undefined>{
 const projectionSha256=adoptProjectionOfV1(request.directory,request.files);
 for(const donor of findJavascriptDependencyAdoptDonorsV1(request.storeRoot,{generation:request.generation,projectionSha256,runtime:request.runtime})){
  request.check();
  const verified=await request.verifyDonor(donor.generation);if(!verified||verified.sha256!==donor.installed)continue;
  const source=join(request.storeRoot,"j",donor.generation.slice(0,20)),placed:string[]=[];
  try{
   await request.pin(donor.generation);
   for(const root of adoptClonedRootsV1(request.members)){
    request.check();
    const from=join(source,...root.split("/")),to=join(request.directory,...root.split("/"));
    if(!lstatSync(from,{throwIfNoEntry:false}))continue;
    if(lstatSync(to,{throwIfNoEntry:false}))throw Error("Fresh absent clone target required");
    placed.push(to);
    const argv=adoptCloneCommandV1(process.platform,from,to);
    if(argv){if(!await request.run(argv))throw Error("Clone refused");}
    else await cp(from,to,{recursive:true,verbatimSymlinks:true,force:false,errorOnExist:true,filter:()=>{request.check();return true;}});
   }
   const shape=await request.scan(),expected=verified.shape;
   if(shape.entries!==expected.entries||shape.directories!==expected.directories||shape.files!==expected.files||shape.links!==expected.links||shape.sha256!==expected.sha256)throw Error("Adopted tree differs from its donor");
   return donor.generation;
  }catch{
   for(const path of placed.reverse())try{await request.remove(path);}catch{}
   request.check();
  }
 }
 return undefined;
}
/** 🧾️ Builds the receipt body of a complete installed generation. */
export const adoptReceiptBodyV1=(generation:string,projectionSha256:string,runtime:AdoptRuntimeV1,installed:string):Omit<AdoptReceiptV1,"sha256">=>({version:1,purpose:"dependency-adopt-receipt",generation,projectionSha256,runtime,installed});
/** 🪪️ Digests the producer recipe constant a runtime claim is bound to. */
export const adoptRecipeSha256V1=(recipe:unknown):string=>generationSha256V1(JSON.stringify(recipe));
