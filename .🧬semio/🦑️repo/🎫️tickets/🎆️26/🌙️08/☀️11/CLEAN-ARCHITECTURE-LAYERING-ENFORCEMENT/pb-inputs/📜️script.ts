import {readFileSync,readdirSync,lstatSync,openSync,readSync,closeSync,mkdirSync,writeFileSync,writeSync} from "node:fs";
import {resolve,join,extname,relative,sep} from "node:path";
import {createHash} from "node:crypto";
import {spawn} from "node:child_process";
import {isDeepStrictEqual} from "node:util";
import {terminateOwnedChildTree} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";

const started=performance.now(),epochStarted=Date.now(),ticket=resolve(import.meta.dir,".."),root=resolve(ticket,"../../../../../../.."),[command,epoch,expectedProducer,originalDefining,expectedDefining]=process.argv.slice(2);
const signal=new AbortController();let child:ReturnType<typeof spawn>|undefined,reason="running",spawnError:string|undefined;
const stop=(cause="cancelled")=>{reason=cause;signal.abort();if(child)terminateOwnedChildTree(child);};
process.once("SIGINT",stop);process.once("SIGTERM",stop);
let timer:ReturnType<typeof setTimeout>|undefined,observedBytes=0,physicalReads=0;
if(command!=="receive"||epoch!=="pb4"||process.argv.length!==7||![expectedProducer,originalDefining,expectedDefining].every(value=>/^[0-9a-f]{64}$/.test(value)))throw Error("Literal producer and defining SHA arguments required");
const producerBefore=lstatSync(import.meta.filename);
if(!producerBefore.isFile()||producerBefore.isSymbolicLink()||producerBefore.size>16777216||[...import.meta.filename].length>256||import.meta.filename.length>256)throw Error("Bounded original producer source required");
const producerBytes=readFileSync(import.meta.filename),producerAfter=lstatSync(import.meta.filename);
observedBytes+=producerBytes.length;physicalReads++;
if(producerBefore.ino!==producerAfter.ino||producerBefore.size!==producerAfter.size||producerBefore.mtimeMs!==producerAfter.mtimeMs||createHash("sha256").update(producerBytes).digest("hex")!==expectedProducer)throw Error("Literal external producer SHA refused");
const metadata=(path:string)=>{const before=lstatSync(path);if(!before.isFile()||before.isSymbolicLink()||before.size>16777216)throw Error("Metadata input refused");const bytes=readFileSync(path),after=lstatSync(path);observedBytes+=bytes.length;physicalReads++;if(observedBytes>268435456||before.ino!==after.ino||before.size!==after.size||before.mtimeMs!==after.mtimeMs)throw Error("Metadata observation refused");return JSON.parse(bytes.toString("utf8"));};
const expand=(value:unknown):any=>JSON.parse(JSON.stringify(value).replaceAll("${workspaceFolder}",JSON.stringify(root).slice(1,-1)));
const input=(name:string)=>metadata(join(import.meta.dir,name));
const policy=expand(input("parent-portable.json")),scope=input("scope-portable.json");
if(policy.version!==1||policy.maximumElapsedMilliseconds!==60000||policy.typecheckMilliseconds!==30000||policy.testMilliseconds!==15000||policy.capabilities.repositoryRoot!==root)throw Error("Exact authored Pack parent declaration required");
const deadline=epochStarted+policy.maximumElapsedMilliseconds,remaining=()=>Math.min(deadline-Date.now(),policy.maximumElapsedMilliseconds-(performance.now()-started));
const check=()=>{signal.signal.throwIfAborted();if(remaining()<=0)throw Error("Original Pack parent deadline exhausted");};
const bounded=(path:string)=>{if([...path].length>policy.observation.maximumPathCodePoints||path.length>policy.observation.maximumPathUtf16Units)throw Error("Source path ceiling");};
const observe=(path:string)=>{check();bounded(path);const before=lstatSync(path);if(!before.isFile()||before.isSymbolicLink())throw Error("Original source file required");const fd=openSync(path,"r"),digest=createHash("sha256"),buffer=Buffer.alloc(policy.observation.chunkBytes);let bytes=0;try{for(;;){check();const count=readSync(fd,buffer,0,buffer.length,null);if(!count)break;observedBytes+=count;physicalReads++;if(observedBytes>policy.observation.maximumCumulativeBytes)throw Error("Cumulative source observation exhausted");bytes+=count;digest.update(buffer.subarray(0,count));}}finally{closeSync(fd);}const after=lstatSync(path);if(before.ino!==after.ino||before.size!==after.size||before.mtimeMs!==after.mtimeMs||bytes!==before.size)throw Error("Source advanced during physical read");return{path,bytes,sha256:digest.digest("hex"),inode:before.ino,mtimeMs:before.mtimeMs};};
const output=join(ticket,"🗑️generated",epoch,"e");mkdirSync(resolve(output,".."),{recursive:true});mkdirSync(output);
const emit=(name:string,value:unknown)=>{const bytes=Buffer.from(JSON.stringify(value));if(bytes.length>policy.observation.maximumMetadataBytes)throw Error("Metadata ceiling");writeFileSync(join(output,name),bytes);};
const progress=async(stage:string)=>{check();console.log(`[DEBUG] Pack parent ${stage} remaining=${remaining()}ms observed=${observedBytes}B`);await new Promise<void>(done=>setImmediate(done));check();};
const verify=(stage:string)=>{const producer=observe(import.meta.filename);if(producer.sha256!==expectedProducer)throw Error("External producer SHA advanced");const original=observe(join(import.meta.dir,"defining.json"));if(original.sha256!==originalDefining)throw Error("Original defining SHA advanced");const manifest=join(import.meta.dir,"defining-portable.json"),actual=observe(manifest);if(actual.sha256!==expectedDefining)throw Error("Literal defining manifest advanced");const defining=metadata(manifest),reread=observe(manifest);if(reread.sha256!==expectedDefining||defining.originalDefiningSha256!==originalDefining)throw Error("Defining manifest advanced during parse");const rows=defining.pins.map((pin:{path:string;bytes:number;sha256:string})=>{const row=observe(join(root,pin.path));if(row.bytes!==pin.bytes||row.sha256!==pin.sha256)throw Error(`Literal defining input advanced: ${pin.path}`);return row;});emit(`pins-${stage}.json`,{producer:producer.sha256,original:original.sha256,manifest:actual.sha256,rows,exact:true});};
const census=()=>{const paths:string[]=[];const walk=(path:string)=>{check();bounded(path);const state=lstatSync(path);if(state.isSymbolicLink())throw Error("Selected source symlink refused");if(state.isDirectory()){for(const name of readdirSync(path).sort())if(!scope.excludeDirectories.includes(name))walk(join(path,name));}else if(state.isFile()&&scope.extensions.includes(extname(path)))paths.push(path);};for(const path of scope.roots)walk(join(root,path));for(const path of scope.files)paths.push(join(root,path));const unique=[...new Set(paths)].sort();if(unique.length>policy.observation.maximumClaims)throw Error("Source claim ceiling");return unique.map(observe);};
let status:number|null=null,physicalClosed=false;
try{
 timer=setTimeout(()=>stop("timeout"),Math.max(1,remaining()));verify("before-gui");
 const expected=expand(input("launch-portable.json")[0]),readGui=(path:string)=>metadata(path).configurations;
 const producerRelative=relative(root,import.meta.filename).split(sep).join("/"),exactCommand=`bun "${root}/node_modules/nx/dist/bin/nx.js" exec --projects=ticket-fixed-slots --excludeTaskDependencies -- bun "${root}/${producerRelative}" receive pb4 ${expectedProducer} ${originalDefining} ${expectedDefining}`;
 const parentWorkspace=join(ticket,"root-launch-seed-inputs/🧪️nx");
 if(expected.command!==exactCommand||resolve(expected.cwd)!==parentWorkspace||resolve(expected.env.NX_WORKSPACE_ROOT_PATH)!==parentWorkspace)throw Error("Exact external SHA argv and parent Nx workspace required");
 for(const file of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const all=readGui(join(root,file)),matches=all.filter((row:{name:string})=>row.name===expected.name);if(matches.length!==1||!isDeepStrictEqual(expand(matches[0]),expected))throw Error("Exact registered GUI parent row required");}
 if(resolve(expected.env.SEMIO_TEST_ARTIFACT_DIR)!==resolve(policy.capabilities.artifactDirectory))throw Error("Exact original artifact owner required");
 emit("gui.json",{row:expected,exact:true});mkdirSync(policy.capabilities.artifactDirectory);await progress("admission");const before=census();emit("before.json",{claims:before,qualification:scope.qualification});verify("before-child");
 const env={...process.env,...expected.env,NX_WORKSPACE_ROOT_PATH:root,SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify({version:1,policy:{version:1,owner:policy.owner,maximumElapsedMilliseconds:policy.maximumElapsedMilliseconds},deadlineEpochMilliseconds:deadline,capabilities:policy.capabilities})};
 emit("invocation.json",{startedEpochMilliseconds:epochStarted,deadlineEpochMilliseconds:deadline,policy,underlyingCommand:"bun nx run @semio-tech/framework-pack-record-rs:test-refusals --skip-nx-cache",parentWorkspace,childWorkspace:root,dynamicEnvironment:["SEMIO_SCRIPT_PROCESS_INVOCATION","NX_WORKSPACE_ROOT_PATH"],literalEnvironment:expected.env});
 await progress("child-admission");let outputBytes=0,outputLines=0;const stdout=openSync(join(output,"out.txt"),"wx"),stderr=openSync(join(output,"err.txt"),"wx");
 try{
  child=spawn(process.execPath,[join(root,"node_modules/nx/dist/bin/nx.js"),"run","@semio-tech/framework-pack-record-rs:test-refusals","--skip-nx-cache"],{cwd:root,env,stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32",windowsHide:true});
  const append=(fd:number,bytes:Buffer)=>{outputBytes+=bytes.length;for(const byte of bytes)if(byte===10)outputLines++;if(outputBytes>policy.maximumBytes||outputLines>policy.maximumLines){stop("output-limit");return;}writeSync(fd,bytes);};child.stdout!.on("data",bytes=>append(stdout,bytes));child.stderr!.on("data",bytes=>append(stderr,bytes));
  status=await new Promise<number|null>(done=>{child!.once("error",error=>{spawnError=String(error);reason="spawn-error";});child!.once("close",(code:number|null)=>{physicalClosed=true;done(code);});});
 }finally{closeSync(stdout);closeSync(stderr);}
 if(reason==="running")reason="exit";emit("terminal.json",{status,reason,spawnError,physicalClosed,cancelled:signal.signal.aborted,elapsedMilliseconds:performance.now()-started});if(spawnError)throw Error(spawnError);verify("after-child");await progress("closure-observation");
 const after=census(),index=new Map(after.map((row:any)=>[row.path,row])),prior=new Set(before.map((row:any)=>row.path)),advances=before.filter((row:any)=>!isDeepStrictEqual(row,index.get(row.path))),births=after.filter((row:any)=>!prior.has(row.path));
 emit("closure.json",{status,physicalClosed,claims:before.length,exact:before.length-advances.length,advances,births,observedBytes,physicalReads,qualification:scope.qualification});process.exitCode=status??1;
}catch(error){emit("refusal.json",{error:String(error),status,physicalClosed,observedBytes,physicalReads,elapsedMilliseconds:performance.now()-started});console.error(`[DEBUG] Pack parent refused: ${String(error)}`);process.exitCode=1;}
finally{if(timer)clearTimeout(timer);process.removeListener("SIGINT",stop);process.removeListener("SIGTERM",stop);}
