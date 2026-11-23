import {createHash,randomBytes} from "node:crypto";
import {chmod,mkdir,open,readdir,readFile,rename,rm,stat,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {codeDirectoryV1,companionsCurrentV1,executableBitV1,sealCodeV1,verifyCodeSnapshotV1,type SealedCodeV1} from "../../🛡️resilience/🟦️.ts";
import type {ReplicaSinkV1} from "../../🔁️replica/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧾️ The receipt `published` of the kit: when it was written, the code digest it was made from, the module directory of the sealed paths, how many files it holds and the sha256 of its `SHA256SUMS`. */
export type KitPublishedV1=Readonly<{version:1;publishedAt:string;code:string;moduleRoot:string;files:number;sums:string}>;

/** 📄️ One file of the kit: its path below the kit directory, the path in the sealed code, its hash and its execute bit (the kit file carries it on disk, `SHA256SUMS` stays the plain `sha256sum -c` format). */
export type KitFileV1=Readonly<{path:string;source:string;bytes:number;sha256:string;executable:boolean}>;

/** 📦️ What one publication did: `published` (a new kit), `unchanged` (the kit already is the sealed code of the digest and intact) or `skipped` with the reason. */
export type KitResultV1=Readonly<{outcome:"published"|"unchanged"|"skipped";reason:string|null;code:string|null}>;

const validators=new Map<string,CompiledJsonSchemaSubset>(),token=():string=>randomBytes(8).toString("hex"),sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex"),bytewise=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right)),stale=3_600_000;
const topologyName="🗺️topology";

/** 🛡️ Validates a value against one definition of the provisioning schema. */
export function requireKitShapeV1(definition:"KitPublished",value:unknown,message:string):void{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🗂️ The directory of the kit below a store root: `<store>/v1/bootstrap/kit`, holding `topology/`, `SHA256SUMS` and `published`. */
export function kitDirectoryV1(store:string):string{return join(store,"v1","bootstrap","kit");}

/** 📚️ The files of the kit that a seal yields: every sealed file below the topology directory of the module, as `topology/<path below it>`, sorted bytewise. */
export function kitFilesV1(seal:SealedCodeV1):readonly KitFileV1[]{
 const prefix=`${seal.moduleRoot}/${topologyName}/`;
 return seal.files.filter(file=>file.path.startsWith(prefix)).map(file=>({path:`topology/${file.path.slice(prefix.length)}`,source:file.path,bytes:file.bytes,sha256:file.sha256,executable:file.executable})).sort((left,right)=>bytewise(left.path,right.path));
}

/** 🧮️ The text of `SHA256SUMS`: one `<sha256>  <path>` line per kit file, which `sha256sum -c` and `shasum -a 256 -c` verify. */
export function kitSumsV1(files:readonly KitFileV1[]):string{return files.map(file=>`${file.sha256}  ${file.path}\n`).join("");}

async function filesBelow(directory:string,prefix=""):Promise<string[]>{
 const found:string[]=[];
 for(const entry of await readdir(directory,{withFileTypes:true})){
  const path=prefix===""?entry.name:`${prefix}/${entry.name}`;
  if(entry.isDirectory())found.push(...await filesBelow(join(directory,entry.name),path));else found.push(path);
 }
 return found;
}

/** 🔎️ Checks a kit directory: `published` is well formed, `SHA256SUMS` hashes to what it says and lists exactly the files below `topology/`, every file has its listed hash, and (when `snapshot` is given, a directory with `sealed.json` and `files/`) the kit is exactly the topology files of that seal with the same bytes. Returns the first problem or `null`. */
export async function verifyKitV1(kit:string,snapshot?:string):Promise<string|null>{
 try{
  const published=JSON.parse(await readFile(join(kit,"published"),"utf8")) as KitPublishedV1;
  requireKitShapeV1("KitPublished",published,"Closed kit receipt required");
  const sums=await readFile(join(kit,"SHA256SUMS"));
  if(sha(sums)!==published.sums)return "SHA256SUMS differs from the receipt";
  const listed=new Map<string,string>();
  for(const line of sums.toString("utf8").split("\n")){
   if(line==="")continue;
   const match=/^([0-9a-f]{64}) {2}(topology\/.+)$/u.exec(line);
   if(match===null)return `bad line in SHA256SUMS: ${line}`;
   listed.set(match[2]!,match[1]!);
  }
  if(listed.size!==published.files)return "file count differs from the receipt";
  const present=(await filesBelow(kit)).filter(path=>path!=="SHA256SUMS"&&path!=="published").sort(bytewise);
  if(present.join("\n")!==[...listed.keys()].sort(bytewise).join("\n"))return "the files differ from SHA256SUMS";
  for(const [path,hash] of listed)if(sha(await readFile(join(kit,...path.split("/"))))!==hash)return `changed ${path}`;
  if(snapshot!==undefined&&process.platform!=="win32"){
   const seal=JSON.parse(await readFile(join(snapshot,"sealed.json"),"utf8")) as SealedCodeV1;
   for(const file of kitFilesV1(seal))if(await executableBitV1(join(kit,...file.path.split("/")))!==file.executable)return `execute bit differs at ${file.path}`;
  }
  if(snapshot!==undefined){
   const seal=JSON.parse(await readFile(join(snapshot,"sealed.json"),"utf8")) as SealedCodeV1;
   if(seal.digest!==published.code)return "the receipt names another code digest than the snapshot";
   const expected=kitFilesV1(seal);
   if(expected.length!==listed.size||expected.some(file=>listed.get(file.path)!==file.sha256))return "the kit differs from the sealed files";
   for(const file of expected)if(sha(await readFile(join(snapshot,"files",...file.source.split("/"))))!==file.sha256)return `snapshot differs at ${file.source}`;
  }
  return null;
 }catch(error){return error instanceof Error?error.message:String(error);}
}

/** 📤️ Writes the kit of a verified sealed snapshot into the store. The snapshot (`sealed.json` and `files/`) is re-hashed first; a kit that already is that code and intact is left alone (`unchanged`); otherwise the kit is built in a private `.tmp-kit-` directory next to it, `published` written last, and swapped in by two renames (the old kit is moved aside first and removed afterwards), so a reader sees the old kit or the new one, never half of each, and no second copy stays. */
export async function publishKitV1(options:Readonly<{store:string;originId:string;snapshot:string;now?:()=>number}>):Promise<KitResultV1>{
 const kit=kitDirectoryV1(options.store),base=dirname(kit),now=options.now??Date.now;
 const broken=await verifyCodeSnapshotV1(options.snapshot);
 if(broken!==null)return {outcome:"skipped",reason:`snapshot: ${broken}`,code:null};
 const seal=JSON.parse(await readFile(join(options.snapshot,"sealed.json"),"utf8")) as SealedCodeV1,files=kitFilesV1(seal);
 if(files.length===0)return {outcome:"skipped",reason:"the seal holds no topology files",code:seal.digest};
 if(await verifyKitV1(kit,options.snapshot)===null)return {outcome:"unchanged",reason:null,code:seal.digest};
 await mkdir(base,{recursive:true});
 const temporary=join(base,`.tmp-kit-${options.originId}-${token()}`),aside=join(base,`.old-kit-${token()}`);
 try{
  const sums=kitSumsV1(files);
  for(const file of files){
   const destination=join(temporary,...file.path.split("/"));
   await mkdir(dirname(destination),{recursive:true});
   await writeFile(destination,await readFile(join(options.snapshot,"files",...file.source.split("/"))),{mode:file.executable?0o755:0o644});
   if(file.executable)await chmod(destination,0o755);
  }
  await writeFile(join(temporary,"SHA256SUMS"),sums);
  const receipt:KitPublishedV1={version:1,publishedAt:new Date(now()).toISOString().replace(/\.\d+Z$/u,"Z"),code:seal.digest,moduleRoot:seal.moduleRoot,files:files.length,sums:sha(sums)};
  requireKitShapeV1("KitPublished",receipt,"Closed kit receipt required");
  const handle=await open(join(temporary,"published"),"w");
  try{await handle.writeFile(`${JSON.stringify(receipt)}\n`);await handle.sync();}finally{await handle.close();}
  if(await verifyKitV1(temporary,options.snapshot)!==null)throw Error("the new kit does not verify");
  const had=await stat(kit).then(()=>true,()=>false);
  if(had)await rename(kit,aside);
  try{await rename(temporary,kit);}catch(error){if(had)await rename(aside,kit).catch(()=>{});throw error;}
  if(had)await rm(aside,{recursive:true,force:true});
  return {outcome:"published",reason:null,code:seal.digest};
 }catch(error){
  await rm(temporary,{recursive:true,force:true}).catch(()=>{});
  return {outcome:"skipped",reason:(error instanceof Error?error.message:String(error)).slice(0,256),code:seal.digest};
 }
}

/** ✂️ Removes the unfinished `.tmp-kit-` and `.old-kit-` directories that a dead writer left in the bootstrap directory (silent for an hour). */
export async function pruneKitLitterV1(store:string,now:number=Date.now()):Promise<number>{
 const base=dirname(kitDirectoryV1(store));
 let removed=0;
 for(const name of await readdir(base).catch(()=>[] as string[])){
  if(!name.startsWith(".tmp-kit-")&&!name.startsWith(".old-kit-"))continue;
  const info=await stat(join(base,name)).catch(()=>undefined);
  if(info!==undefined&&now-info.mtimeMs>stale){await rm(join(base,name),{recursive:true,force:true});removed++;}
 }
 return removed;
}

/** 🔄️ Keeps the kit of the store equal to the sealed code that runs on the group server: reads the local sealed snapshot of `digest` (`.cache/f/code/<digest>`), seals it again when it is missing, damaged or carries other companions than the disk (a companion, a provision script, can change without a new digest), and publishes the kit from it. Never throws; a failure comes back as `skipped` with its reason. */
export async function maintainKitV1(options:Readonly<{root:string;moduleRoot:string;store:string;originId:string;digest:string;digestOf:(moduleRoot:string)=>Promise<string>;signal?:AbortSignal;now?:()=>number}>):Promise<KitResultV1>{
 try{
  const snapshot=join(codeDirectoryV1(options.root),options.digest);
  let current=false;
  try{const seal=JSON.parse(await readFile(join(snapshot,"sealed.json"),"utf8")) as SealedCodeV1;current=seal.digest===options.digest&&await companionsCurrentV1(options.root,options.moduleRoot,seal)&&await verifyCodeSnapshotV1(snapshot)===null;}catch{}
  if(!current){
   const sealed=await sealCodeV1({root:options.root,moduleRoot:options.moduleRoot,digest:options.digest,digestOf:options.digestOf,signal:options.signal,now:options.now});
   if(sealed.directory===null)return {outcome:"skipped",reason:`seal: ${sealed.reason??"none"}`,code:options.digest};
  }
  await pruneKitLitterV1(options.store,(options.now??Date.now)());
  return await publishKitV1({store:options.store,originId:options.originId,snapshot,now:options.now});
 }catch(error){return {outcome:"skipped",reason:(error instanceof Error?error.message:String(error)).slice(0,256),code:options.digest};}
}

/** 🔄️ The kit loop of a group server: every minute it compares the attested digest with the receipt of the kit and publishes when they differ; every `minutes` it also re-verifies the kit and the companions (a provision script edited without a new digest) and publishes again when anything differs. It runs until `signal` aborts and reports a failure as an `error` event. */
export async function refreshKitV1(options:Readonly<{root:string;moduleRoot:string;store:string;originId:string;digestOf:(moduleRoot:string)=>Promise<string>;attested:()=>Promise<string|undefined>;minutes:number;pollMilliseconds?:number;sink?:ReplicaSinkV1}>,signal:AbortSignal):Promise<void>{
 const poll=options.pollMilliseconds??60_000,every=options.minutes*60_000,sink=options.sink??(()=>{});
 let last=-Infinity,known="",retry=0;
 while(!signal.aborted){
  const at=Date.now();
  try{
   const digest=await options.attested(),published=await readFile(join(kitDirectoryV1(options.store),"published"),"utf8").then(text=>(JSON.parse(text) as {code?:string}).code,()=>undefined);
   if(digest!==undefined&&at>=retry&&(digest!==published||digest!==known||at-last>=every)){
    const result=await maintainKitV1({root:options.root,moduleRoot:options.moduleRoot,store:options.store,originId:options.originId,digest,digestOf:options.digestOf,signal});
    if(result.outcome==="skipped"){retry=at+5*poll;sink({version:1,event:"error",stage:"kit",message:result.reason??"skipped"});}
    else{known=digest;last=at;retry=0;}
   }
  }catch(error){if(signal.aborted)break;retry=at+5*poll;sink({version:1,event:"error",stage:"kit",message:error instanceof Error?error.message:String(error)});}
  await new Promise<void>(done=>{
   const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();};
   const timer=setTimeout(finish,poll);signal.addEventListener("abort",finish,{once:true});
  });
 }
}
