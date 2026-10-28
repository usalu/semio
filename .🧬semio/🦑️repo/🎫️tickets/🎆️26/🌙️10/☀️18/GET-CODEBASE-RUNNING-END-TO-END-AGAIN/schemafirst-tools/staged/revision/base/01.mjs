import schemaContract from"./🧬️schema/🔣️.json"with{type:"json"};
import{constants,lstatSync,openSync,fstatSync,readSync,closeSync,readFileSync}from"node:fs";
import{AsyncLocalStorage}from"node:async_hooks";
import{createHash}from"node:crypto";
import{resolve,isAbsolute,parse,relative,sep,join}from"node:path";
import{fileURLToPath}from"node:url";
import{churnProtocolV1,concurrentEditV1,retryOnConcurrentEditSyncV1,staleImplementationV1}from"../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import{filesystemShapeErrorV1}from"../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🌊️races/🟦️.ts";
const schema=JSON.parse(JSON.stringify(schemaContract));
const maximumSourceBytes=8388608,maximumTextBytes=65536,maximumChunkBytes=65536,maximumPathUnits=256,maximumAncestors=64;
/** 🧊️ Remembers one settled revision under its file identity, so an unchanged Source is hashed once instead of on every self-check. */
const settledRevisions=new Map(),settleMilliseconds=2000,maximumSettledRevisions=4096;
const stamp=row=>[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");
function capturedPhysicalSources(value,images){
 if(value===undefined)return undefined;
 const contract=schema.definitions.CapturedPhysicalSources;if(!value||typeof value!=="object"||Object.keys(value).sort().join(",")!=="root,sources,version"||value.version!==contract.properties.version.const||typeof value.root!=="string"||!isAbsolute(value.root)||resolve(value.root)!==value.root||value.root.length>maximumPathUnits||!Array.isArray(value.sources)||!value.sources.length||value.sources.length>contract.properties.sources.maxItems)throw Error("Closed compiler-captured physical Source context required");
 const result=new Map();let previous="";for(const row of value.sources){if(!row||typeof row!=="object"||Object.keys(row).sort().join(",")!=="path,sha256"||typeof row.path!=="string"||!row.path||isAbsolute(row.path)||/[:\\\0]/u.test(row.path)||row.path.split("/").some(part=>!part||part==="."||part==="..")||typeof row.sha256!=="string"||!new RegExp(contract.properties.sources.items.properties.sha256.pattern,"u").test(row.sha256)||previous&&Buffer.compare(Buffer.from(previous),Buffer.from(row.path))>=0)throw Error("Canonical captured physical Source paths required");const path=join(value.root,row.path);if(path.length>maximumPathUnits)throw Error("Captured physical Source path capacity exceeded");result.set(path,row.sha256);previous=row.path;}return Object.freeze({root:value.root,sources:result,images:capturedPhysicalSourceImages(images,value.root,result)});
}
/** 🖼️ Validates the compiler-private byte lengths and data texts that let a bundle answer its own reads from the bytes it was compiled from. */
function capturedPhysicalSourceImages(value,root,sources){
 const images=new Map();if(value===undefined)return images;const contract=schema.definitions.CapturedPhysicalSourceImages;if(!value||typeof value!=="object"||Object.keys(value).sort().join(",")!=="images,version"||value.version!==contract.properties.version.const||!Array.isArray(value.images)||value.images.length>contract.properties.images.maxItems)throw Error("Closed compiler-captured physical Source images required");
 const limits=contract.properties.images.items.properties;for(const row of value.images){if(!row||typeof row!=="object"||!["byteLength,path","byteLength,path,text"].includes(Object.keys(row).sort().join(","))||typeof row.path!=="string"||!Number.isSafeInteger(row.byteLength)||row.byteLength<0||row.byteLength>limits.byteLength.maximum)throw Error("Closed compiler-captured physical Source image required");const path=join(root,row.path),expected=sources.get(path);if(!expected||images.has(path))throw Error("Captured physical Source image must name one admitted Source");if(row.text!==undefined){if(typeof row.text!=="string"||row.text.length>limits.text.maxLength||Buffer.byteLength(row.text)!==row.byteLength||createHash("sha256").update(row.text).digest("hex")!==expected)throw Error("Captured physical Source image must match its identity");}images.set(path,Object.freeze({byteLength:row.byteLength,text:row.text}));}
 return images;
}
/** 🧭️ Separates a compiler-owned physical Source origin from the actual emitted module identity. */
export function physicalSourceOriginV1(moduleURL,compiledSourceURL){if(typeof moduleURL!=="string"||compiledSourceURL!==undefined&&(typeof compiledSourceURL!=="string"||!compiledSourceURL.startsWith("file:")||compiledSourceURL.length>schema.definitions.ModuleOrigin.maxLength))throw Error("Closed physical Source module origin required");return compiledSourceURL??moduleURL;}
/** 📄️ Reads reducing UTF8 contract DATA through the same original descriptor and identity checks; a compiler-captured image answers from its own admitted bytes, so a Source edited after the capture can never refuse a bundle that carries its image. */
export function capturePhysicalSourceTextV1(url,maximumBytes){return captureBoundedSourceRevisionV1(fileURLToPath(url),maximumBytes,true).text;}
const capturedSources=capturedPhysicalSources(import.meta.semioCapturedPhysicalSources,import.meta.semioCapturedPhysicalSourceImages);
function capturedReadRefusalV1(captured,path){const distance=relative(captured.root,path),inside=distance!==".."&&!distance.startsWith(".."+sep)&&!isAbsolute(distance);return Object.freeze({version:1,scope:inside?"workspace":"outside",path:inside?distance.split(sep).join("/"):null,sourceCount:captured.sources.size});}
/** 🔁️ Captures one Source revision; a Source another process edits while it is read is captured again as the `read` layer of the churn policy declares. */
const settleWindow=churnProtocolV1().settleMilliseconds,waiter=new Int32Array(new SharedArrayBuffer(4));
/** 🕰️ A Source modified within the settle window of the churn policy may belong to a writer that paused halfway through a rewrite, which no stamp check can see (the file is stable, only incomplete): the capture waits out the rest of the window and captures again, and a second capture that differs is a Source still being written, a concurrent edit of the `read` layer. */
function settledCapture(path,maximumBytes,retainText,first){
 let row;try{row=lstatSync(path,{bigint:true});}catch{return first;}
 const age=Date.now()-Number((row.mtimeNs>row.ctimeNs?row.mtimeNs:row.ctimeNs)/1000000n);
 if(age<0||age>=settleWindow)return first;
 Atomics.wait(waiter,0,0,settleWindow-age);
 const second=captureOnce(path,maximumBytes,retainText);
 if(second.sha256!==first.sha256||second.byteLength!==first.byteLength)throw concurrentEditV1("bytes",path,"Source revision still being written");
 return second;
}
function capture(path,maximumBytes,retainText){globalThis[Symbol.for("semio.sourceTrace.v1")]?.record("p",path);return retryOnConcurrentEditSyncV1("read",()=>{const first=captureOnce(path,maximumBytes,retainText);return capturedSources?first:settledCapture(path,maximumBytes,retainText,first);});}
function captureOnce(path,maximumBytes,retainText){
 const expected=capturedSources?.sources.get(path);if(capturedSources&&!expected)throw Error("Undeclared compiler-captured physical Source read refused: "+JSON.stringify(capturedReadRefusalV1(capturedSources,path)));
 const image=capturedSources?.images.get(path);if(image&&(!retainText||image.text!==undefined)){if(image.byteLength>maximumBytes)throw Error("Compiler-captured admitted Source image identity required");return Object.freeze({sha256:expected,byteLength:image.byteLength,text:retainText?image.text:null});}
 if(capturedSources){const bytes=readFileSync(path);if(bytes.length>maximumBytes||createHash("sha256").update(bytes).digest("hex")!==expected)throw Error("Compiler-captured admitted Source image identity required");return Object.freeze({sha256:expected,byteLength:bytes.length,text:retainText?new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(bytes):null});}
 const base=parse(path).root,parts=relative(base,path).split(sep).filter(Boolean),parents=[];let current=base;
 if(parts.length>maximumAncestors)throw Error("Source revision ancestry capacity exceeded");
 for(const part of["",...parts.slice(0,-1)]){if(part)current=join(current,part);const row=lstatSync(current,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink())throw filesystemShapeErrorV1(row,current,"Physical Source revision ancestry required","ENOTDIR");parents.push({path:current,dev:row.dev,ino:row.ino,mode:row.mode});}
 const before=lstatSync(path,{bigint:true});
 if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<0n||before.size>BigInt(maximumBytes))throw filesystemShapeErrorV1(before,path,"Bounded physical Source revision required");
 const size=Number(before.size),identity=stamp(before),file=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));let failed=false,primary,value,window,retained;
 try{
 const verify=()=>{for(const before of parents){const row=lstatSync(before.path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||row.dev!==before.dev||row.ino!==before.ino||row.mode!==before.mode)throw concurrentEditV1("ancestry",before.path,"Source revision ancestry changed");}const linked=lstatSync(path,{bigint:true}),opened=fstatSync(file,{bigint:true});if(!linked.isFile()||linked.isSymbolicLink()||stamp(linked)!==identity||stamp(opened)!==identity)throw concurrentEditV1("descriptor",path,"Source revision descriptor or path changed");};
 verify();window=Buffer.alloc(Math.min(maximumChunkBytes,Math.max(1,size)));retained=retainText?Buffer.alloc(size):undefined;const hash=createHash("sha256");let offset=0;
 while(offset<size){if(stamp(fstatSync(file,{bigint:true}))!==identity)throw concurrentEditV1("descriptor",path,"Source revision descriptor changed");const length=Math.min(window.length,size-offset),bytes=readSync(file,window,0,length,offset);if(bytes!==length)throw concurrentEditV1("length",path,"Source revision shortened during capture");hash.update(window.subarray(0,length));retained?.set(window.subarray(0,length),offset);offset+=length;}
 verify();if(readSync(file,window,0,1,offset)!==0)throw concurrentEditV1("length",path,"Source revision grew beyond admitted EOF");verify();value=Object.freeze({sha256:hash.digest("hex"),byteLength:size,text:retained===undefined?null:new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(retained)});
 }catch(error){failed=true;primary=error;}
 try{closeSync(file);}catch(error){if(!failed){failed=true;primary=error;}}finally{window?.fill(0);retained?.fill(0);}
 if(failed)throw primary;if(expected!==undefined&&value.sha256!==expected)throw Error("Compiler-captured physical Source byte identity changed");return value;
}
const ownSourceURL=physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL),ownPath=fileURLToPath(ownSourceURL),ownSchemaPath=fileURLToPath(new URL("./🧬️schema/🔣️.json",ownSourceURL));
function ownRow(path){try{const row=lstatSync(path,{bigint:true});return row.isFile()&&!row.isSymbolicLink()?{stamp:stamp(row),settled:Number(row.ctimeNs/1000000n)<Date.now()-settleMilliseconds&&Number(row.mtimeNs/1000000n)<Date.now()-settleMilliseconds}:undefined;}catch{return undefined;}}
const ownBefore=[ownRow(ownPath),capturedSources?ownRow(ownSchemaPath):null],loaded=capture(ownPath,maximumSourceBytes,false),loadedSchema=capturedSources?capture(ownSchemaPath,maximumTextBytes,false):undefined,ownAfter=[ownRow(ownPath),capturedSources?ownRow(ownSchemaPath):null];
const ownTrusted=ownBefore[0]?.settled&&ownBefore[0].stamp===ownAfter[0]?.stamp&&(ownBefore[1]===null||ownBefore[1]?.settled&&ownBefore[1].stamp===ownAfter[1]?.stamp)?[ownBefore[0].stamp,ownBefore[1]?.stamp??null]:undefined;
function ownUnchanged(){return ownTrusted!==undefined&&ownRow(ownPath)?.stamp===ownTrusted[0]&&(ownTrusted[1]===null||ownRow(ownSchemaPath)?.stamp===ownTrusted[1]);}

const pinKeyV1=Symbol.for("semio.implementationPin.context.v1"),pinContextV1=globalThis[pinKeyV1]??=new AsyncLocalStorage();
/** 🧮️ Counts the pins open in this process: Bun evaluates the top level of a dynamically imported module outside the async context that imported it, so a module loaded by a pinned invocation reads its Sources unpinned unless the process itself remembers the pin. A process that runs exactly one invocation (a command line process) opts in with {@link useProcessWideImplementationPinV1}; a long-lived service that runs invocations side by side never does, so one invocation's pin never covers another. */
const pinCountKeyV1=Symbol.for("semio.implementationPin.count.v1"),pinCountV1=globalThis[pinCountKeyV1]??={open:0,processWide:false};
/** 🧍️ Declares that this process runs one invocation: while its pin is open, module top levels evaluated outside the async context count as pinned too. */
export function useProcessWideImplementationPinV1(){pinCountV1.processWide=true;}
/** 📍️ True while an invocation is committed to the implementation it loaded: every self-check of a loaded Source then lets the invocation finish, and what it publishes names the loaded bytes, so a Source edited meanwhile is left to the next invocation. */
export function implementationPinnedV1(){return pinContextV1.getStore()?.active===true||pinCountV1.processWide===true&&pinCountV1.open>0;}
/** 📌️ Runs `body` committed to the loaded implementation; pins nest and a failing body releases its pin. */
export async function withImplementationPinV1(body){const parent=pinContextV1.getStore(),pin={active:true,authority:parent?.active===true?parent.authority:new Map()};pinCountV1.open++;try{return await pinContextV1.run(pin,body);}finally{pin.active=false;pinCountV1.open--;}}
/** 📸️ Reads authority data (the owner registry, declarations) once per pinned invocation: every later read of the same key returns the first answer, so a run completes consistently with the data it started from; outside a pin it reads every time. */
export function pinnedAuthorityReadV1(key,read){const rows=pinContextV1.getStore()?.active===true?pinContextV1.getStore().authority:undefined;if(!rows)return read();if(!rows.has(key))rows.set(key,read());return rows.get(key);}
/** 🧽️ Forgets the authority snapshot so the next read starts a new consistent view; a graph load calls it before it reads its authority. */
export function clearPinnedAuthorityV1(){pinContextV1.getStore()?.authority?.clear();}
function unchanged(path,row){try{return capture(path,row.byteLength,false).sha256===row.sha256;}catch(error){if(error?.message==="Bounded physical Source revision required"||error?.code==="ENOENT")return false;throw error;}}
function current(){if(implementationPinnedV1())return;if(ownUnchanged())return;if(!unchanged(ownPath,loaded)||loadedSchema&&!unchanged(ownSchemaPath,loadedSchema))throw staleImplementationV1("Stale loaded Source revision reader; restart required");}
/** 🔏️ Streams one finite ordinary Source revision; returned DATA grants no physical resource custody. */
export function captureBoundedSourceRevisionV1(path,maximumBytes,retainText=false){
 if(typeof path!=="string"||!isAbsolute(path)||resolve(path)!==path||path.length>maximumPathUnits||path.includes("\0")||!Number.isSafeInteger(maximumBytes)||maximumBytes<0||maximumBytes>maximumSourceBytes||typeof retainText!=="boolean"||retainText&&maximumBytes>maximumTextBytes)throw Error("Reducing finite Source revision request required");
 const cached=capturedSources?undefined:settledRevisions.get(path);
 if(cached&&cached.maximumBytes===maximumBytes&&cached.retainText===retainText){try{const row=lstatSync(path,{bigint:true});if(row.isFile()&&!row.isSymbolicLink()&&stamp(row)===cached.identity)return cached.value;}catch{}}
 current();const value=capture(path,maximumBytes,retainText),taken=Date.now();current();
 try{const row=lstatSync(path,{bigint:true});if(row.isFile()&&!row.isSymbolicLink()&&Number(row.ctimeNs/1000000n)<taken-settleMilliseconds&&Number(row.mtimeNs/1000000n)<taken-settleMilliseconds){if(settledRevisions.size>=maximumSettledRevisions)settledRevisions.clear();settledRevisions.set(path,{maximumBytes,retainText,identity:stamp(row),value});}}catch{}
 return value;
}
