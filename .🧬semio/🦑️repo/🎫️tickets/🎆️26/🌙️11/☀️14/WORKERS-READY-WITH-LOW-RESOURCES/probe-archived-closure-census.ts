import {join} from "node:path";
import {pathToFileURL} from "node:url";
import {createHash} from "node:crypto";
import {brotliDecompressSync} from "node:zlib";
import {lstatSync} from "node:fs";

const workspace=process.cwd(),root=join(workspace,".cache","b","g","m","generations");
const source=pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations")+"/");
const {GENERATION_POLICY_V1:policy,createNodeFilePortV1,readGenerationFragmentV1,readGenerationManifestV1,readGenerationPointerV1}=await import(new URL("🟨️.mjs",source).href);
const {recoverClosureRepairV1}=await import(new URL("🩹️closure-repair/🟨️.mjs",source).href);
const {workspaceClosurePortV1}=await import(new URL("🌐️remote-exchange/🟦️.ts",source).href);
const {closureReferencePairsOfTextV1}=await import(new URL("../🩹️closure-repair/🟨️.mjs",source).href);
const fs=createNodeFilePortV1(),pointerPath=join(root,policy.pointer),before=await fs.read(pointerPath);
if(before===undefined)throw Error("Current graph pointer is unavailable");
const pointer=readGenerationPointerV1(before),parent=process.argv[2];
if(pointer.generation!==260||pointer.manifest!==null||pointer.parent!==parent||pointer.reason!=="closure-missing")throw Error("Expected exact generation 260 quarantine");
const manifestBytes=await fs.read(join(root,policy.manifests,parent+".json"));
if(!manifestBytes||createHash("sha256").update(manifestBytes).digest("hex")!==parent)throw Error("The archived manifest does not match its digest");
const archived=readGenerationManifestV1(manifestBytes),{identity,implementation}=archived,port=workspaceClosurePortV1(workspace);
if(!port?.verify)throw Error("The workspace closure verifier is unavailable");
const verify=port.verify,signal=AbortSignal.timeout(120000),started=performance.now();
if(process.argv.includes("--shape")||process.argv.includes("--presence")||process.argv.includes("--remote")){
 const remote=process.argv.includes("--remote"),presence=remote||process.argv.includes("--presence");
 let count=0,packed=0,expanded=0,maximum=0,links=0,firstRefusal,missing=0,other=0,heldBytes=0,globalMissing=false;
 const references=new Map<string,boolean>(),affected=new Set<string>(),documents=[["@global",{fragment:archived.global}],...Object.entries(archived.owners)] as const;
 for(const [name,owner] of documents){
  signal.throwIfAborted();
  const {fragment:digest}=owner as {fragment:string;root?:string;kind?:string};
  const bytes=await fs.read(join(root,policy.fragments,digest+".json.br"));
  if(!bytes||createHash("sha256").update(bytes).digest("hex")!==digest)throw Error("Archived fragment "+digest+" is unavailable");
  const decoded=brotliDecompressSync(bytes,{maxOutputLength:16*1024**2}),text=decoded.toString("utf8");
  if((expanded+=decoded.length)>1024**3)throw Error("Archive expanded-byte diagnostic bound");
  if(name!=="@global"){
   try{const fragment=readGenerationFragmentV1(JSON.parse(text));if(fragment.name!==name||fragment.root!==owner.root||fragment.kind!==owner.kind)throw Error("Owner metadata differs");}
   catch(error){firstRefusal??={name,digest,at:count,reason:error instanceof Error?error.message:String(error)};}
  }
  const found=new Set<string>();
  for(const [key,namespace] of closureReferencePairsOfTextV1(text)){
   const ref=namespace+"/"+key;found.add(ref);
   if(!references.has(ref)){
    let absent=false;
    if(presence)try{const row=lstatSync(join(port.directory(namespace),key+".json"));if(row.isFile())heldBytes+=row.size;else other++;}
    catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"){missing++;absent=true;}else other++;}
    references.set(ref,absent);
   }
   if(references.get(ref)){if(name==="@global")globalMissing=true;else affected.add(name);}
  }
  if(name!=="@global")links+=found.size;
  packed+=bytes.length;maximum=Math.max(maximum,decoded.length);count++;
 }
 const after=await fs.read(pointerPath);
 console.log("[DEBUG] "+JSON.stringify({generation:pointer.generation,parent,count,packed,expanded,maximum,links,references:references.size,firstRefusal,local:presence?{missing,other,heldBytes,globalMissing,affectedOwners:affected.size}:undefined,pointerUnchanged:after!==undefined&&before.equals(after),elapsedMs:Math.round(performance.now()-started)}));
 if(remote&&after!==undefined&&before.equals(after)){
  const {resolveRemoteCacheV1,remoteCacheFetchV1}=await import(new URL("../../🌐️remote-cache/🛰️fleet/🟦️.ts",source).href);
  const {httpRemoteExecutionContentV1}=await import(new URL("../../🌐️remote-execution/🗄️content/🟦️.ts",source).href);
  const config=resolveRemoteCacheV1(process.env,{root:workspace});
  if(!config)throw Error("The workspace remote cache is unconfigured");
  const client=httpRemoteExecutionContentV1({endpoint:config.contract.endpoint,token:config.token,fetch:remoteCacheFetchV1(config.environment,{root:workspace}),timeoutSeconds:5,retries:0});
  const hashes=[...new Set([...references].filter(([,absent])=>absent).map(([reference])=>reference.slice(-64)))];
  if(hashes.length>1024)throw Error("Archive remote digest audit bound");
  const verdict=await client.missing(config.contract.instance,hashes.map(hash=>({hash,size:0})),AbortSignal.timeout(45000));
  const finished=await fs.read(pointerPath);
  console.log("[DEBUG] "+JSON.stringify({remote:{probed:hashes.length,absent:verdict.length,held:hashes.length-verdict.length},pointerUnchanged:finished!==undefined&&before.equals(finished)}));
 }
 process.exit(0);
}
let probed=0,absent=0,unknown=0,bytes=0,fragments=0,peak=process.memoryUsage().rss,timedOut=false;
const answer=await recoverClosureRepairV1({
 root,fs,pointer,identity,implementation,signal,
 progress(done:number){fragments=done;if(done%64===0)peak=Math.max(peak,process.memoryUsage().rss);},
 async verifyClosure({namespace,key}:{namespace:"build-input-closures"|"build-lock-closures";key:string}){
  probed++;
  bytes+=port.size(namespace,key)??0;
  if(bytes>512*1024**2){unknown++;return undefined;}
  const verdict=await verify(namespace,key,signal);
  if(verdict===false)absent++;
  if(verdict===undefined)unknown++;
  return verdict;
 }
 }).catch((error:unknown)=>{if(!signal.aborted)throw error;timedOut=true;return undefined;});
const after=await fs.read(pointerPath);
console.log("[DEBUG] "+JSON.stringify({
 generation:pointer.generation,parent,recordedImplementation:implementation,fragments,probed,absent,unknown,bytes,
 pointerUnchanged:after!==undefined&&before.equals(after),qualified:answer!==undefined,owners:answer?.owners.length,missing:answer?.missing.length,receipt:answer?.receipt,
 elapsedMs:Math.round(performance.now()-started),peakMiB:Math.ceil(Math.max(peak,process.memoryUsage().rss)/1024**2),timedOut
}));
