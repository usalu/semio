import{readFileSync,lstatSync,mkdirSync,createReadStream,createWriteStream,linkSync,unlinkSync,realpathSync}from"node:fs";
import{open}from"node:fs/promises";
import{join,relative,isAbsolute,resolve,dirname}from"node:path";
import{fileURLToPath}from"node:url";
import{createHash,randomUUID}from"node:crypto";
import{createGunzip}from"node:zlib";
import{Transform}from"node:stream";
import{pipeline}from"node:stream/promises";

const root=new URL("../../../../../../../",import.meta.url),ticket=fileURLToPath(new URL(".",import.meta.url)),schema=JSON.parse(readFileSync(new URL("linux-release-schema.json",import.meta.url),"utf8")),{validateJsonSchemaSubset}=await import(new URL("🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts",root));
const shape=(definition,value)=>{if(validateJsonSchemaSubset(definition,value,schema).length)throw Error("Closed Linux preparation contract required");};

export function linuxSetupStateV1(facts){
 shape(schema.definitions.Facts,facts);
 if(facts.architecture!=="x64")return"linux-x64-required";
 if(facts.platform==="darwin")return"linux-runner-required";
 if(facts.platform==="linux")return facts.debian12!==true?"debian12-required":facts.docker!==true?"docker-setup-required":"ready";
 if(facts.msvcPending)return"privileged-installer-pending";
 if(facts.reboot==="yes")return"reboot-required";
 if(facts.wsl!==true)return facts.elevation==="elevated"?"wsl-install-required":"elevation-required";
 return facts.debian12!==true?"debian-import-required":facts.docker!==true?"docker-setup-required":"ready";
}

export function readLinuxReleasePlanV1(value){
 shape(schema,value);
 if(new Set(value.artifacts.map(row=>row.id)).size!==2||new Set([...value.artifacts.map(row=>row.file),value.rootfs.file]).size!==3)throw Error("Distinct Linux artifact identities required");
 for(const row of value.artifacts)if(row.id==="debian-rootfs"&&row.url!=="https://registry-1.docker.io/v2/library/debian/blobs/sha256:"+row.sha256)throw Error("Rootfs URL must carry its pinned digest");
 return value;
}

export function physicalDirectoryV1(path){
 const actual=realpathSync(path),expected=resolve(path),normalize=value=>process.platform==="win32"?value.toLowerCase():value;
 if(normalize(actual)!==normalize(expected))throw Error("Physical directory ancestry required");
 const stat=lstatSync(path);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error("Physical directory required");return {dev:stat.dev,ino:stat.ino};
}

const directories=new Map();
function checkDirectory(directory){const before=directories.get(directory),after=physicalDirectoryV1(directory);if(!before||before.dev!==after.dev||before.ino!==after.ino)throw Error("Preparation directory identity changed");}

function outputDirectory(){
 const generated=join(ticket,"🗑️generated"),directory=join(generated,"linux-preparation");
 physicalDirectoryV1(ticket);try{mkdirSync(generated);}catch(error){if(error.code!=="EEXIST")throw error;}
 physicalDirectoryV1(generated);try{mkdirSync(directory);}catch(error){if(error.code!=="EEXIST")throw error;}
 physicalDirectoryV1(generated);directories.set(directory,physicalDirectoryV1(directory));
 return directory;
}

function artifactPath(directory,name){
 checkDirectory(directory);const path=join(directory,name),inside=relative(directory,path);
 if(!inside||isAbsolute(inside)||inside.startsWith("..")||path.length>256)throw Error("Bounded artifact path required");
 return path;
}

async function verifyArtifact(path,spec,signal){
 signal.throwIfAborted();let before;try{before=lstatSync(path);}catch(error){if(error.code==="ENOENT")return false;throw error;}
 if(!before.isFile()||before.isSymbolicLink()||before.size>(spec.maximumBytes??spec.bytes)||spec.bytes!==undefined&&before.size!==spec.bytes)throw Error("Occupied artifact does not match its declaration");
 const hash=createHash("sha256");for await(const bytes of createReadStream(path,{signal})){signal.throwIfAborted();hash.update(bytes);}
 const after=lstatSync(path);
 if(before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeMs!==after.mtimeMs||hash.digest("hex")!==spec.sha256)throw Error("Artifact digest or identity mismatch");
 return true;
}

function publishArtifact(stage,target){
 checkDirectory(dirname(target));linkSync(stage,target);unlinkSync(stage);
}

async function downloadArtifact(directory,spec,signal){
 const target=artifactPath(directory,spec.file);if(await verifyArtifact(target,spec,signal)){console.log("[DEBUG] Verified existing",spec.id);return target;}
 const stage=artifactPath(directory,spec.file+"."+randomUUID()+".partial"),headers=spec.id==="wsl"?{"User-Agent":"semio-repo","Accept":"application/octet-stream"}:{};
 if(spec.id==="debian-rootfs"){
  const response=await fetch("https://auth.docker.io/token?service=registry.docker.io&scope=repository:library/debian:pull",{signal});if(!response.ok)throw Error("Public Debian registry token request failed");const token=await response.json();if(typeof token.token!=="string")throw Error("Public registry token required");headers.Authorization="Bearer "+token.token;
 }
 let file,identity,published=false;
 try{
  const response=await fetch(spec.url,{headers,signal});if(!response.ok||!response.body)throw Error("Pinned artifact download failed: "+response.status);
  file=await open(stage,"wx");identity=await file.stat();let count=0,last=0;const hash=createHash("sha256");
  for await(const bytes of response.body){signal.throwIfAborted();count+=bytes.length;if(count>spec.bytes)throw Error("Pinned artifact exceeded its declared bytes");hash.update(bytes);let offset=0;while(offset<bytes.length){const write=await file.write(bytes,offset,bytes.length-offset);if(!write.bytesWritten)throw Error("Artifact write made no progress");offset+=write.bytesWritten;}if(Date.now()-last>=5000){console.log("[DEBUG] Download",spec.id,count,"/",spec.bytes);last=Date.now();}}
  if(count!==spec.bytes||hash.digest("hex")!==spec.sha256)throw Error("Pinned artifact byte count or checksum mismatch");
  await file.sync();await file.close();file=undefined;signal.throwIfAborted();publishArtifact(stage,target);published=true;console.log("[DEBUG] Download verified",spec.id,count);return target;
 }finally{
  await file?.close();if(!published&&identity){try{const current=lstatSync(stage);if(current.dev===identity.dev&&current.ino===identity.ino)unlinkSync(stage);}catch(error){if(error.code!=="ENOENT")throw error;}}
 }
}

async function unpackRootfs(directory,source,spec,signal){
 const target=artifactPath(directory,spec.file);if(await verifyArtifact(target,spec,signal)){console.log("[DEBUG] Verified existing uncompressed rootfs");return;}
 const stage=artifactPath(directory,spec.file+"."+randomUUID()+".partial"),hash=createHash("sha256");let count=0,created=false,identity,last=0;
 const output=createWriteStream(stage,{flags:"wx"});output.once("open",()=>{created=true;identity=lstatSync(stage);});
 try{
  await pipeline(createReadStream(source),createGunzip(),new Transform({transform(bytes,encoding,done){count+=bytes.length;if(count>spec.maximumBytes)return done(Error("Rootfs exceeded its admitted bytes"));hash.update(bytes);if(Date.now()-last>=5000){console.log("[DEBUG] Unpack rootfs",count,"/",spec.maximumBytes);last=Date.now();}done(null,bytes);}}),output,{signal});
  if(hash.digest("hex")!==spec.sha256)throw Error("Uncompressed rootfs checksum mismatch");signal.throwIfAborted();publishArtifact(stage,target);created=false;console.log("[DEBUG] Uncompressed rootfs verified",count);
 }finally{if(created){try{const current=lstatSync(stage);if(current.dev===identity.dev&&current.ino===identity.ino)unlinkSync(stage);}catch(error){if(error.code!=="ENOENT")throw error;}}}
}

async function prepare(){
 if(process.arch!=="x64")throw Error("Prepared Linux release requires an x64 execution environment");
 const plan=readLinuxReleasePlanV1(JSON.parse(readFileSync(new URL("linux-release.json",import.meta.url),"utf8"))),directory=outputDirectory(),controller=new AbortController(),abort=()=>controller.abort(Error("Linux preparation cancelled"));process.once("SIGINT",abort);process.once("SIGTERM",abort);
 try{const rootfs=plan.artifacts.find(row=>row.id==="debian-rootfs");await downloadArtifact(directory,rootfs,controller.signal);await unpackRootfs(directory,artifactPath(directory,rootfs.file),plan.rootfs,controller.signal);await downloadArtifact(directory,plan.artifacts.find(row=>row.id==="wsl"),controller.signal);console.log("[DEBUG] Pinned artifacts prepared; no installer, distribution or daemon started");}finally{process.removeListener("SIGINT",abort);process.removeListener("SIGTERM",abort);}
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
 if(process.argv.length!==3||process.argv[2]!=="prepare")throw Error("Use only the artifact-only prepare command");
 await prepare();
}
