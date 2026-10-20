import {createHash} from "node:crypto";
import schema from "./schema.json" with {type:"json"};
import {compileJsonSchemaSubset} from "../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {CapturePlan,CaptureControl,InventorySnapshot,StableSource,SourceObservationPort,SourceSeal,SealedSourceRow} from "./contract.ts";

const cap={files:262144,source:4294967296,file:268435456,manifest:134217728,stream:4294967296,chunk:65536};
const encode=new TextEncoder(),decode=new TextDecoder("utf-8",{fatal:true}),hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),json=(value:unknown)=>encode.encode(JSON.stringify(value)),same=(a:unknown,b:unknown)=>JSON.stringify(a)===JSON.stringify(b),check=(control:CaptureControl)=>{control.signal.throwIfAborted();if(control.remainingMilliseconds()<=0)throw Error("Fresh capture deadline exceeded");};
const order=(a:string,b:string)=>Buffer.compare(Buffer.from(a),Buffer.from(b)),validatePlan=compileJsonSchemaSubset(schema),validateInventory=compileJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/inventory"}),excluded=new Set(schema.definitions.payload.const.excludedSegments);
export function sourcePayloadPath(path:string,root:string):string{sourcePath(path,root);if(path.split("/").some(part=>excluded.has(part)))throw Error("Fresh Source payload excludes generated and tool state");return path;}

/** 🛂️ Admits ordinary canonical Source paths on both host and destination. */
export function sourcePath(path:string,root:string):string {
 if(typeof path!=="string"||!path||path!==path.normalize("NFC")||decode.decode(encode.encode(path))!==path||path.startsWith("/")||/[\\:\x00-\x1f\x7f]/u.test(path)||path.split("/").some(part=>!part||part==="."||part==="..")||path.split("/").length>64||encode.encode(path).length>1024||("/repo/"+path).length>256||(root.replace(/[\\/]$/u,"")+"/"+path).length>256)throw Error("Bounded canonical Source path required");return path;
}

export interface FileFacts {kind:"file"|"directory"|"symlink"|"other";identity:string;anchor:string;bytes:number;links:number;executable:boolean}
export interface StableFilePort {
 ancestors:(path:string)=>readonly string[];
 named:(path:string)=>Promise<FileFacts>;
 openNoFollow:(path:string)=>Promise<object>;
 descriptor:(handle:object)=>Promise<FileFacts>;
 read:(handle:object,offset:number,maximum:number)=>Promise<Uint8Array>;
 close:(handle:object)=>Promise<void>;
}

/** 🔏️ Reads one exact ordinary file while retaining descriptor, name and ancestor fences. */
export async function readStableSource(path:string,maximum:number,port:StableFilePort,control:CaptureControl,now:()=>string):Promise<StableSource>{
 check(control);if(!Number.isSafeInteger(maximum)||maximum<0||maximum>cap.file)throw Error("Reducing stable Source bound required");const parents=[];
 for(const name of port.ancestors(path)){check(control);const facts=await port.named(name);if(facts.kind!=="directory")throw Error("Physical Source ancestry required");parents.push({name,anchor:facts.anchor});}if(parents.length>256)throw Error("Bounded Source ancestry required");
 const named=await port.named(path);if(named.kind!=="file"||named.links!==1||!Number.isSafeInteger(named.bytes)||named.bytes<0||named.bytes>maximum)throw Error("Bounded single-link Source file required");const handle=await port.openNoFollow(path);
 try{const verify=async()=>{check(control);const current=await port.named(path),descriptor=await port.descriptor(handle);if(current.kind!=="file"||descriptor.kind!=="file"||current.links!==1||descriptor.links!==1||current.identity!==named.identity||descriptor.identity!==named.identity)throw Error("Source descriptor or name changed");for(const parent of parents){const row=await port.named(parent.name);if(row.kind!=="directory"||row.anchor!==parent.anchor)throw Error("Source ancestor changed");}check(control);};await verify();const bytes=new Uint8Array(named.bytes);let offset=0;while(offset<bytes.length){check(control);const chunk=await port.read(handle,offset,Math.min(cap.chunk,bytes.length-offset));if(!chunk.length||chunk.length>Math.min(cap.chunk,bytes.length-offset))throw Error("Source exact bounded read required");bytes.set(chunk,offset);offset+=chunk.length;check(control);}if((await port.read(handle,offset,1)).length)throw Error("Source grew beyond EOF");await verify();return{bytes,identity:named.identity,executable:named.executable,observedAt:now()};}finally{await port.close(handle);}
}


/** 🕳️ Proves a negative Source observation through stable ordinary ancestry and exact ENOENT fences. */
export async function observeStableAbsence(path:string,port:StableFilePort,control:CaptureControl):Promise<boolean>{
 check(control);const parents=port.ancestors(path);if(parents.length>256)throw Error("Bounded absence ancestry required");const retained:{name:string;anchor:string}[]=[];let missing:string|undefined;
 const absent=async(name:string)=>{check(control);try{await port.named(name);return false;}catch(error){if((error as any).code!=="ENOENT")throw error;return true;}finally{check(control);}};
 for(const name of parents){check(control);try{const row=await port.named(name);if(row.kind!=="directory")throw Error("Ordinary absence ancestry required");retained.push({name,anchor:row.anchor});}catch(error){if((error as any).code!=="ENOENT")throw error;missing=name;break;}}
 const result=await absent(path);for(const parent of retained){check(control);const row=await port.named(parent.name);if(row.kind!=="directory"||row.anchor!==parent.anchor)throw Error("Absence ancestor changed");}if(missing&&!await absent(missing))throw Error("Absent ancestor changed");if(result&&!await absent(path))throw Error("Source absence changed");check(control);return result;
}

function header(name:string,bytes:number,type:string,executable=false):Uint8Array{
 const out=Buffer.alloc(512),octal=(value:number,width:number)=>value.toString(8).padStart(width-1,"0")+"\0";out.write(name,0,100,"ascii");out.write(octal(executable?0o555:0o444,8),100);out.write(octal(0,8),108);out.write(octal(0,8),116);out.write(octal(bytes,12),124);out.write(octal(0,12),136);out.fill(32,148,156);out.write(type,156);out.write("ustar\0",257);out.write("00",263);out.write(out.reduce((sum,value)=>sum+value,0).toString(8).padStart(6,"0")+"\0 ",148);return out;
}

/** 📦️ Projects one file as finite Unicode PAX records without retaining an archive. */
export function* paxFile(path:string,bytes:Uint8Array,index:number,executable=false):Generator<Uint8Array>{
 sourcePath(path,"/repo");if(!Number.isSafeInteger(index)||index<0||index>cap.files||bytes.length>cap.manifest&&path===".semio-private/source.json"||bytes.length>cap.file)throw Error("Bounded PAX entry required");const text=" path="+path+"\n";let size=encode.encode(text).length+1;while(String(size).length+encode.encode(text).length!==size)size=String(size).length+encode.encode(text).length;const metadata=encode.encode(String(size)+text);yield header("PaxHeaders/"+index,metadata.length,"x");yield metadata;if(metadata.length%512)yield new Uint8Array(512-metadata.length%512);yield header("Source/"+index,bytes.length,"0",executable);for(let offset=0;offset<bytes.length;offset+=cap.chunk)yield bytes.subarray(offset,offset+cap.chunk);if(bytes.length%512)yield new Uint8Array(512-bytes.length%512);
}

export interface CaptureSink {
 write:(chunk:Uint8Array)=>Promise<void>;
 finish:(seal:SourceSeal)=>Promise<void>;
 abortAndJoin:(reason:string)=>Promise<void>;
}
export interface FreshCorePorts {
 source:SourceObservationPort;
 sink:CaptureSink;
 metadata:(name:"manifest"|"divergence",captureId:string,bytes:Uint8Array)=>Promise<void>;
 now:()=>string;
}

function snapshot(value:InventorySnapshot,plan:CapturePlan):InventorySnapshot{
 const names=new Set<string>();if(validateInventory.validate(value).length||!value||!Array.isArray(value.paths)||value.paths.length>cap.files||!Array.isArray(value.criticalPaths)||value.criticalPaths.length>cap.files||!Array.isArray(value.absent)||value.absent.length>cap.files||!value.control||Object.keys(value.control).sort().join()!=="membership,providers,registry,tracked,untracked"||Object.values(value.control).some(value=>!/^[a-f0-9]{64}$/u.test(value)))throw Error("Closed bounded Source inventory required");
 for(const row of value.paths){sourcePayloadPath(row.path,plan.workspaceRoot);if(names.has(row.path)||typeof row.owner!=="string"||!row.owner||row.owner.length>256)throw Error("Unique canonical Source owner required");names.add(row.path);}for(const path of[...value.criticalPaths,...value.absent])sourcePath(path,plan.workspaceRoot);return{paths:[...value.paths].sort((a,b)=>order(a.path,b.path)),criticalPaths:[...new Set(value.criticalPaths)].sort(order),absent:[...new Set(value.absent)].sort(order),control:{...value.control},observedAt:value.observedAt};
}

/** 🚚️ Captures and streams one stable file at a time, then seals honest epoch provenance. */
export async function captureFreshSource(plan:CapturePlan,ports:FreshCorePorts,control:CaptureControl):Promise<SourceSeal>{
 check(control);if(validatePlan.validate(plan).length||!plan||plan.version!==1||plan.kind!=="fresh-source-capture-plan"||!/^[a-f0-9]{32}$/u.test(plan.captureId)||typeof plan.workspaceRoot!=="string"||!plan.workspaceRoot||!Array.isArray(plan.critical)||!plan.critical.length||plan.critical.length>1024)throw Error("Closed fresh capture plan required");const expected=new Map<string,string|null>();for(const row of plan.critical){sourcePath(row.path,plan.workspaceRoot);if(expected.has(row.path)||row.sha256!==null&&!/^[a-f0-9]{64}$/u.test(row.sha256))throw Error("Unique reviewed critical Source required");expected.set(row.path,row.sha256);}const planSha256=hash(json(plan)),before=snapshot(await ports.source.inventory(control),plan),catalog=new Map(before.paths.map(row=>[row.path,row])),critical=new Set([...before.criticalPaths,...expected.keys()]),rows:SealedSourceRow[]=[],epochs:any[]=[],absent:any[]=[],captured=new Map<string,SealedSourceRow>();let sourceBytes=0,streamBytes=0,metadataBytes=0,index=0,streamStarted=false;
 const write=async(chunk:Uint8Array)=>{check(control);if(chunk.length>cap.chunk&&chunk.length!==512&&chunk.length>2048)throw Error("Bounded archive chunk required");if(streamBytes+chunk.length>cap.stream)throw Error("Capture stream bound exceeded");streamStarted=true;await ports.sink.write(chunk);streamBytes+=chunk.length;check(control);},emit=async(path:string,body:Uint8Array,executable=false)=>{for(const chunk of paxFile(path,body,index++,executable))await write(chunk);},read=async(path:string)=>{check(control);const value=await ports.source.readStable(path,cap.file,control);if(value.bytes.length>cap.file||!value.identity||value.identity.length>1024)throw Error("Bounded stable Source observation required");return{...value,sha256:hash(value.bytes)};},record=async(path:string,epoch:number)=>{const value=await read(path),owner=catalog.get(path)?.owner;if(!owner)throw Error("Captured Source must belong to refreshed roster");if(expected.has(path)&&value.sha256!==expected.get(path))throw Error("Reviewed critical Source hash differs");const row={path,owner,bytes:value.bytes.length,sha256:value.sha256,identity:value.identity,executable:value.executable,epoch};if(sourceBytes+row.bytes>cap.source)throw Error("Source capture byte bound exceeded");metadataBytes+=json(row).length;if(metadataBytes>cap.manifest)throw Error("Capture metadata bound exceeded");await emit(path,value.bytes,value.executable);rows.push(row);captured.set(path,row);sourceBytes+=row.bytes;return row;},criticalCheck=async()=>{for(const path of [...critical].sort(order)){check(control);const prior=captured.get(path);if(prior){const current=await read(path);if(current.identity!==prior.identity||current.sha256!==prior.sha256||current.executable!==prior.executable)throw Error("Critical Source epoch changed");}else if(!await ports.source.observeAbsent(path,control))throw Error("Critical Source absence changed");}};
 try{
  await control.progress("capture:critical",{paths:critical.size});const openedAt=ports.now();for(const path of[...critical].sort(order)){if(expected.get(path)===null){if(catalog.has(path)||!await ports.source.observeAbsent(path,control))throw Error("Reviewed absent Source is present");absent.push({path,epoch:0,observedAt:ports.now()});continue;}await record(path,0);}await criticalCheck();const afterCritical=snapshot(await ports.source.inventory(control),plan);if(!same(before.control,afterCritical.control)||!same(before.criticalPaths,afterCritical.criticalPaths))throw Error("Critical provider or membership epoch changed");epochs.push({epoch:0,owner:"critical-controls",openedAt,sealedAt:ports.now(),rowsSha256:hash(json(rows)),files:rows.length,bytes:sourceBytes});
  let chunk:SealedSourceRow[]=[],chunkBytes=0,owner="",opened="";const closeEpoch=()=>{if(!chunk.length)return;epochs.push({epoch:epochs.length,owner,openedAt:opened,sealedAt:ports.now(),rowsSha256:hash(json(chunk)),files:chunk.length,bytes:chunkBytes});chunk=[];chunkBytes=0;};
  for(const item of before.paths){if(critical.has(item.path))continue;check(control);if(chunk.length&&(chunk.length>=128||chunkBytes>=33554432||owner!==item.owner))closeEpoch();if(!chunk.length){owner=item.owner;opened=ports.now();}const row=await record(item.path,epochs.length);chunk.push(row);chunkBytes+=row.bytes;if(rows.length%128===0)await control.progress("capture:source",{files:rows.length,bytes:sourceBytes,streamBytes});}closeEpoch();
  await criticalCheck();const after=snapshot(await ports.source.inventory(control),plan);if(!same(before.control,after.control)||!same(before.criticalPaths,after.criticalPaths))throw Error("Final critical provider or membership epoch changed");const finalPaths=new Set(after.paths.map(row=>row.path)),changedIdentity:string[]=[];for(const row of rows){check(control);if(finalPaths.has(row.path)&&await ports.source.observeIdentity(row.path,control)!==row.identity)changedIdentity.push(row.path);}const divergence={added:after.paths.filter(row=>!catalog.has(row.path)).map(row=>row.path),missing:rows.filter(row=>!finalPaths.has(row.path)).map(row=>row.path),changedIdentity,controlChanged:false};absent.push(...before.absent.filter(path=>!absent.some(row=>row.path===path)).map(path=>({path,epoch:0,observedAt:before.observedAt})));const manifest={version:1 as const,kind:"fresh-source-manifest" as const,captureId:plan.captureId,planSha256,atomicHostSnapshot:false as const,model:"stable-file-critical-control-and-owner-epochs" as const,controlBefore:before.control,controlAfter:after.control,criticalPaths:[...critical].sort(order),rows,absent,epochs,divergence,bytes:sourceBytes},body=json(manifest);if(body.length>cap.manifest)throw Error("Final Source manifest bound exceeded");const seal={manifest,manifestSha256:hash(body)};await ports.metadata("divergence",plan.captureId,json(divergence));await ports.metadata("manifest",plan.captureId,body);await emit(".semio-private/source.json",body);await write(new Uint8Array(1024));await ports.sink.finish(seal);check(control);await control.progress("capture:sealed",{files:rows.length,bytes:sourceBytes,manifestSha256:seal.manifestSha256,atomic:false});return seal;
 }catch(error){if(streamStarted)try{await ports.sink.abortAndJoin(String((error as Error).message).slice(0,256));}catch(join){throw new AggregateError([error,join],"Capture failed and receiver terminal custody is unconfirmed");}throw error;}
}
