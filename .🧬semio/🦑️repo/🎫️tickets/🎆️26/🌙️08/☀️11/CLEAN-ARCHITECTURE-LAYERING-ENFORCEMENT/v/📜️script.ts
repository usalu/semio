/** 🧪️ Runs current repository sources through an isolated ticket-owned Nx fixture workspace. */
import {mkdirSync,readFileSync} from "node:fs";
import {isAbsolute,resolve,relative,join} from "node:path";

const workspace=resolve(import.meta.dir,"../../../../../../../..");
const [command,...arguments_]=process.argv.slice(2);
const options=new Map(arguments_.map(value=>{const at=value.indexOf("=");if(!value.startsWith("--")||at<0)throw Error("Named verification arguments required");return [value.slice(2,at),value.slice(at+1)] as const;}));
for(const key of options.keys())if(!["files","testNamePattern","group","arguments","budget","capabilities-file","libraries"].includes(key))throw Error("Unknown verification option "+key);
const files=(options.get("files")??"").split(",").filter(Boolean).map(path=>{if(isAbsolute(path)||path.split(/[\\/]/u).some(part=>part===".."||part==="."))throw Error("Repository-relative source required");const absolute=resolve(workspace,path);if(relative(workspace,absolute).startsWith(".."))throw Error("Source escapes workspace");return absolute;});
if(!files.length)throw Error("At least one actual source required");
const group=options.get("group")??"sources";
if(!/^[a-z0-9-]{1,40}$/u.test(group))throw Error("Canonical verification group required");
const shortStorage:Readonly<Record<string,string>>={"general-native":"n","native-catalog":"c"};
const artifacts=resolve(import.meta.dir,"../🗑️generated",...(shortStorage[group]?[shortStorage[group]!]:["v",group]));
mkdirSync(artifacts,{recursive:true});
const budget=Number(options.get("budget")??180_000);
if(!Number.isSafeInteger(budget)||budget<=0||budget>1_200_000)throw Error("Finite source verification budget required");
const suppliedArguments=JSON.parse(options.get("arguments")??"[]") as unknown;
if(!Array.isArray(suppliedArguments)||suppliedArguments.some(arg=>typeof arg!=="string"||arg.includes("\0")))throw Error("Explicit command arguments required");
if(command==="run"&&files.length!==1)throw Error("One actual command router required");
const args=command==="test"?["test",...files,...(options.has("testNamePattern")?["--test-name-pattern",options.get("testNamePattern")!]:[])]:command==="strict"?[Bun.resolveSync("typescript/bin/tsc",workspace),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun,node",...(options.has("libraries")?["--lib",options.get("libraries")!]:[]),...files]:command==="run"?[files[0]!,...suppliedArguments]:undefined;
if(!args)throw Error("Expected test, strict or run");
let environment:Record<string,string|undefined>={...process.env,NX_WORKSPACE_ROOT:workspace,SEMIO_TEST_ARTIFACT_DIR:artifacts};
if(command==="run"){
  const routing=await import(join(workspace,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
  let capabilities:Record<string,unknown>={};
  if(options.has("capabilities-file")){
    const path=resolve(import.meta.dir,options.get("capabilities-file")!);
    if(relative(resolve(import.meta.dir,".."),path).startsWith(".."))throw Error("Capabilities input must belong to the ticket");
    const authority=await import(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts"));
    const value=JSON.parse(readFileSync(path,"utf8"));
    const original=authority.readNativeOwnerCapabilities(value);
    if(resolve(original.repositoryRoot)!==workspace)throw Error("Native capabilities must address the actual repository");
    capabilities=value;
    environment={...environment,SEMIO_BUILD_MODE:original.buildMode,CARGO_TARGET_DIR:original.artifactDirectory,CARGO_NET_OFFLINE:String(original.network.offline)};
  }
  environment=routing.scriptProcessEnvironment(routing.createScriptProcessEnvelope({version:1,owner:"ticket:architecture-source-verification",maximumElapsedMilliseconds:budget},capabilities,Date.now()),environment);
}
const child=Bun.spawn([process.execPath,...args],{cwd:workspace,env:environment,stdout:"inherit",stderr:"inherit"});
const started=Date.now(),progress=setInterval(()=>console.log("[DEBUG] Current source verification progress",{command,group,elapsedMilliseconds:Date.now()-started}),10_000);
const deadline=setTimeout(()=>child.kill("SIGTERM"),budget);
const cancel=()=>child.kill("SIGTERM");
process.once("SIGTERM",cancel);process.once("SIGINT",cancel);
try{process.exitCode=await child.exited;}finally{clearInterval(progress);clearTimeout(deadline);process.removeListener("SIGTERM",cancel);process.removeListener("SIGINT",cancel);}
