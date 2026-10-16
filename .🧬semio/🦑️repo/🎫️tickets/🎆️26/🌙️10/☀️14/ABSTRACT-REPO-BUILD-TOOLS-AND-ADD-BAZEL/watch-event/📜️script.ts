import{constants,openSync,closeSync,fstatSync,lstatSync,readSync,realpathSync,type BigIntStats}from"node:fs";
import{mkdir,writeFile,rm,readFile}from"node:fs/promises";
import{join,resolve,dirname,parse}from"node:path";
import{createHash,randomBytes}from"node:crypto";
import assert from"node:assert/strict";

const root=process.cwd(),ticket=resolve(import.meta.dir,".."),output=join(ticket,"🗑️generated"),mode=process.argv[2],candidate=join(root,"🌎️hub/Dockerfile"),planPath=join(output,"watch-source-event-plan.json"),resultPath=join(output,"watch-source-event-result.json"),processRoot=join(root,"🧰️framework/🔨️modules/🏃️process");
if(process.argv.length!==3||!["--plan","--event","--self-test","--strict"].includes(mode!)||root!==realpathSync(root)||output!==realpathSync(output)||["SEMIO_SCRIPT_PROCESS_INVOCATION","SEMIO_SCRIPT_CAPABILITIES","SEMIO_BUILD_TOOL","SEMIO_BUILD_GRAPH_PATH","SEMIO_BUILD_GRAPH_DIGEST","SEMIO_BUILD_REUSE_GRAPH"].some(key=>process.env[key]!==undefined))throw Error("Closed standalone watch event helper required");
const routing:typeof import("../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")=await import(join(processRoot,"📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),{runOwnedCommand}=await import(join(processRoot,"🎛️owned-execution/🟦️.ts")),{checkResourceLeaseControl}=await import(join(processRoot,"🔒️leases/🟦️.ts"));
type Stamp=Readonly<Record<"dev"|"ino"|"mode"|"nlink"|"size"|"mtimeNs"|"ctimeNs"|"atimeNs",string>>;
type Plan=Readonly<{version:1;target:string;sha256:string;stamp:Stamp;parents:readonly Readonly<{path:string;dev:string;ino:string}>[]}>;
const stamp=(value:BigIntStats):Stamp=>Object.fromEntries(["dev","ino","mode","nlink","size","mtimeNs","ctimeNs","atimeNs"].map(key=>[key,String(value[key as keyof BigIntStats])]))as Stamp;
const python=String.raw`
import os,sys,json,stat,hashlib,time,signal
plan_path,target,result_path=sys.argv[1:]
plan_fd=os.open(plan_path,os.O_RDONLY|getattr(os,'O_NOFOLLOW',0))
try:
 plan_before=os.fstat(plan_fd)
 if not stat.S_ISREG(plan_before.st_mode) or plan_before.st_nlink!=1 or plan_before.st_size>32768: raise RuntimeError('Bounded event plan required')
 encoded=os.read(plan_fd,32769);plan_after=os.fstat(plan_fd);plan_named=os.lstat(plan_path)
 if len(encoded)!=plan_before.st_size or any(getattr(value,key)!=getattr(plan_before,key) for value in [plan_after,plan_named] for key in ['st_dev','st_ino','st_size','st_mtime_ns','st_ctime_ns']): raise RuntimeError('Stable event plan required')
 plan=json.loads(encoded.decode('utf8'))
finally: os.close(plan_fd)
if set(plan)!={'version','target','sha256','stamp','parents'} or plan['version']!=1 or plan['target']!=target or len(plan['parents'])>64 or len(target)>256 or os.path.realpath(target)!=target: raise RuntimeError('Closed event plan required')
expected_paths=[];parent=os.path.dirname(target)
while True:
 expected_paths.insert(0,parent)
 if os.path.dirname(parent)==parent: break
 parent=os.path.dirname(parent)
if [row['path'] for row in plan['parents']]!=expected_paths or set(plan['stamp'])!={'dev','ino','mode','nlink','size','mtimeNs','ctimeNs','atimeNs'}: raise RuntimeError('Exact event ancestry and timestamps required')
def parents():
 for row in plan['parents']:
  actual=os.lstat(row['path'])
  if not stat.S_ISDIR(actual.st_mode) or stat.S_ISLNK(actual.st_mode) or str(actual.st_dev)!=row['dev'] or str(actual.st_ino)!=row['ino']: raise RuntimeError('Event parent changed')
def stopped(signum,frame): raise RuntimeError('Owned event cancelled')
signal.signal(signal.SIGTERM,stopped)
signal.signal(signal.SIGINT,stopped)
parents()
fd=os.open(target,os.O_RDONLY|getattr(os,'O_NOFOLLOW',0));primary=None;changed=False
expected=plan['stamp'];keys=['dev','ino','mode','nlink','size','mtimeNs','ctimeNs']
def facts(value): return {'dev':str(value.st_dev),'ino':str(value.st_ino),'mode':str(value.st_mode),'nlink':str(value.st_nlink),'size':str(value.st_size),'mtimeNs':str(value.st_mtime_ns),'ctimeNs':str(value.st_ctime_ns),'atimeNs':str(value.st_atime_ns)}
def read():
 os.lseek(fd,0,os.SEEK_SET);chunks=[];total=0
 while True:
  part=os.read(fd,65536)
  if not part: break
  total+=len(part)
  if total>65536: raise RuntimeError('Bounded event file required')
  chunks.append(part)
 return hashlib.sha256(b''.join(chunks)).hexdigest()
try:
 before=os.fstat(fd);named=os.lstat(target)
 if not stat.S_ISREG(before.st_mode) or before.st_nlink!=1 or before.st_size>65536 or any(facts(before)[key]!=expected[key] or facts(named)[key]!=expected[key] for key in keys): raise RuntimeError('Planned event file changed')
 if read()!=plan['sha256']: raise RuntimeError('Planned event bytes changed')
 parents()
 changed=True
 os.utime(fd,ns=(int(expected['atimeNs']),int(expected['mtimeNs'])+1000000000))
 event=os.fstat(fd)
 if event.st_mtime_ns!=int(expected['mtimeNs'])+1000000000: raise RuntimeError('Exact event timestamp unsupported')
 time.sleep(.05)
 if read()!=plan['sha256']: raise RuntimeError('Event content changed')
 parents()
 named=os.lstat(target)
 if named.st_dev!=before.st_dev or named.st_ino!=before.st_ino: raise RuntimeError('Event path replaced')
except BaseException as error: primary=error
finally:
 try:
  if changed:
   os.utime(fd,ns=(int(expected['atimeNs']),int(expected['mtimeNs'])))
   after=os.fstat(fd);named=os.lstat(target)
   if any(str(getattr(value,'st_'+key))!=expected[key] for value in [after,named] for key in ['dev','ino','mode','nlink','size']) or after.st_atime_ns!=int(expected['atimeNs']) or after.st_mtime_ns!=int(expected['mtimeNs']) or named.st_atime_ns!=after.st_atime_ns or named.st_mtime_ns!=after.st_mtime_ns: raise RuntimeError('Exact event restoration failed')
   parents()
 except BaseException as error:
  if primary is None: primary=error
  else: print('[DEBUG] Event restoration secondary: '+str(error),file=sys.stderr)
 try: os.close(fd)
 except BaseException as error:
  if primary is None: primary=error
if primary is not None: raise primary
receipt={'version':1,'target':target,'sha256':plan['sha256'],'restored':True,'atimeNs':expected['atimeNs'],'mtimeNs':expected['mtimeNs'],'ctimePreserved':False,'scope':'content/inode/atime/mtime restored; timestamp update necessarily changes ctime; watcher may coalesce or duplicate events'}
encoded=json.dumps(receipt)+'\n'
if len(encoded.encode())>8192: raise RuntimeError('Bounded event result required')
with open(result_path,'x',encoding='utf8') as result: result.write(encoded)
print('[DEBUG] Owned content-preserving event restored exact nanosecond times')
`;

await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"watch:event-diagnostic",maximumElapsedMilliseconds:60000},{workspaceRoot:root},Date.now()),async original=>{
 const control=routing.createIssuedResourceControlV1(original,"watch:event",0);
 if(mode==="--strict"){await runOwnedCommand(process.execPath,[join(root,"node_modules/typescript/lib/tsc.js"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowJs","--resolveJsonModule","--allowImportingTsExtensions","--module","preserve","--moduleResolution","bundler","--target","esnext","--lib","esnext,dom,dom.asynciterable","--types","bun","--pretty","false",import.meta.path],root,"watch:event-typescript",Math.floor(checkResourceLeaseControl(control)??0),{signal:control.signal,env:routing.scriptInvocationEnvironment(original,process.env),onProgress:()=>routing.advanceScriptInvocation(original,"watch:event-typescript","running")});return;}
 const plan=(target:string):Plan=>{checkResourceLeaseControl(control);if(target.length>256||target!==resolve(target)||realpathSync(target)!==target)throw Error("Canonical bounded event file required");const parents:Plan["parents"][number][]=[],paths:string[]=[];for(let path=dirname(target);;path=dirname(path)){paths.unshift(path);if(path===parse(path).root)break;}if(paths.length>64)throw Error("Bounded event ancestry required");for(const path of paths){const value=lstatSync(path,{bigint:true});if(!value.isDirectory()||value.isSymbolicLink())throw Error("Physical event ancestry required");parents.push({path,dev:String(value.dev),ino:String(value.ino)});}let fd:number|undefined,primary:{error:unknown}|undefined;try{fd=openSync(target,constants.O_RDONLY|(constants.O_NOFOLLOW??0));const before=fstatSync(fd,{bigint:true});if(!before.isFile()||before.nlink!==1n||before.size>65536n)throw Error("Bounded single-owner event file required");const bytes=Buffer.alloc(Number(before.size));let offset=0;while(offset<bytes.length){checkResourceLeaseControl(control);const count=readSync(fd,bytes,offset,bytes.length-offset,offset);checkResourceLeaseControl(control);if(!count)throw Error("Complete event bytes required");offset+=count;}const after=fstatSync(fd,{bigint:true}),named=lstatSync(target,{bigint:true}),a=stamp(before);for(const key of["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"]as const)if(a[key]!==stamp(after)[key]||a[key]!==stamp(named)[key])throw Error("Event changed during plan");for(const row of parents){const now=lstatSync(row.path,{bigint:true});if(!now.isDirectory()||now.isSymbolicLink()||String(now.dev)!==row.dev||String(now.ino)!==row.ino)throw Error("Event parent changed during plan");}return{version:1,target,sha256:createHash("sha256").update(bytes).digest("hex"),stamp:stamp(after),parents};}catch(error){primary={error};throw error;}finally{if(fd!==undefined)try{closeSync(fd);}catch(error){if(!primary)throw error;}}};
 const event=async(target:string,input:string,result:string)=>{checkResourceLeaseControl(control);await runOwnedCommand("python3",["-c",python,input,target,result],root,"watch:controlled-event",Math.floor(checkResourceLeaseControl(control)??0),{signal:control.signal,env:routing.scriptInvocationEnvironment(original,process.env),onProgress:async()=>{checkResourceLeaseControl(control);await control.publish({resource:"watch:event",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();checkResourceLeaseControl(control);}});checkResourceLeaseControl(control);};
 if(mode==="--self-test"){const directory=join(output,"we-"+randomBytes(4).toString("hex"));await mkdir(directory);try{const target=join(directory,"fixture"),input=join(directory,"plan.json"),result=join(directory,"result.json");await writeFile(target,"owned disposable event fixture\n");const observed=plan(target);await writeFile(input,JSON.stringify(observed)+"\n");await event(target,input,result);const restored=lstatSync(target,{bigint:true});assert.equal(String(restored.ino),observed.stamp.ino);assert.equal(String(restored.mtimeNs),observed.stamp.mtimeNs);assert.equal(String(restored.atimeNs),observed.stamp.atimeNs);assert.equal(plan(target).sha256,observed.sha256);console.log("[DEBUG] Disposable fixture event preserves bytes/inode and exact nanosecond atime/mtime; ctime intentionally changes");}finally{await rm(directory,{recursive:true,force:true});}return;}
 if(mode==="--plan"){const project=JSON.parse(await readFile(join(root,"📋️project.json"),"utf8"));if(project.name!=="workspace")throw Error("Declared workspace watch owner required");await writeFile(planPath,JSON.stringify(plan(candidate),null,2)+"\n",{flag:"wx"});console.log("[DEBUG] Read-only Source event plan retained; event execution requires separate Root release");return;}
 await event(candidate,planPath,resultPath);
});
