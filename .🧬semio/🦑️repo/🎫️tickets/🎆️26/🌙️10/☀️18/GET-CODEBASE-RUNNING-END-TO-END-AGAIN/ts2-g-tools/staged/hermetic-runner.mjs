import{createHash,randomBytes}from"node:crypto";
import{chmodSync,constants,copyFileSync,existsSync,lstatSync,mkdirSync,readFileSync,readdirSync,rmSync,statSync,symlinkSync,writeFileSync}from"node:fs";
import{createRequire}from"node:module";
import{createServer}from"node:net";
import{dirname,isAbsolute,join,parse,relative,resolve,sep}from"node:path";
import{pathToFileURL}from"node:url";
const sha=value=>createHash("sha256").update(value).digest("hex"),slash=value=>value.split(sep).join("/"),fail=message=>{throw new Error("[hermetic] "+message);},order=(left,right)=>left<right?-1:left>right?1:0,clone=(from,to)=>{try{copyFileSync(from,to,constants.COPYFILE_FICLONE);}catch{copyFileSync(from,to);}};
const executable=process.execPath,held={work:undefined,staged:new Map(),rows:[],links:"",server:undefined,token:"",service:undefined,active:undefined,idle:undefined};
const link=(target,path)=>{mkdirSync(dirname(path),{recursive:true});symlinkSync(target,path,"junction");};
const shim=(target,path,platform=process.platform)=>{if(platform!=="win32"){symlinkSync(target,path);return;}const real=resolve(dirname(path),target);if(/[%"\r\n]/.test(real+executable))fail("a tool path with a percent sign, a quote or a line break cannot be shimmed: "+real);if(/\.(exe|com)$/i.test(real)){copyFileSync(real,path+".exe");return;}readFileSync(real);writeFileSync(path+".cmd",'@"'+executable+'" "'+real+'" %*\r\n');};
const signature=info=>info.size+":"+info.mtimeNs+":"+info.ctimeNs+":"+info.ino;
async function serve(manifest,workspace,inside){
 const spec=manifest.service;if(!spec)return undefined;
 const file=inside(spec.module),identity=sha(readFileSync(file));
 if(held.service?.identity!==identity){
  held.service?.instance.release?.();
  const module=await import(pathToFileURL(file).href+"?"+identity.slice(0,16));
  held.service={identity,instance:module.createHermeticServiceV1({resolve:(directory,name)=>createRequire(join(directory,"package.json"))(name)})};
 }
 if(!held.server){
  held.token=randomBytes(16).toString("hex");
  const answer=async(socket,line)=>{
   const control=new AbortController();socket.on("close",()=>control.abort());let reply;
   try{
    const message=JSON.parse(line);if(message.token!==held.token)throw new Error("service token refused");
    const active=held.active;if(!active)throw new Error("no active request");
    const cwd=resolve(message.request.cwd);if(cwd!==active&&!cwd.startsWith(active+sep))throw new Error("service path escapes the execution root");
    reply={ok:true,result:await held.service.instance.handle(message.request,control.signal)};
   }catch(error){reply={ok:false,message:String(error?.message??error)};}
   if(!socket.destroyed)socket.end(JSON.stringify(reply)+"\n");
  };
  held.server=createServer(socket=>{let buffer="";socket.setEncoding("utf8");socket.on("error",()=>{});socket.on("data",chunk=>{buffer+=chunk;const end=buffer.indexOf("\n");if(end>=0){const line=buffer.slice(0,end);buffer="";void answer(socket,line);}});});
  await new Promise((accept,refuse)=>{held.server.once("error",refuse);held.server.listen(0,"127.0.0.1",()=>{held.server.off("error",refuse);accept();});});
  held.server.unref();
 }
 return held.server.address().address+":"+held.server.address().port+":"+held.token;
}
async function execute(argv,session){
 const worker=session!==undefined,flag=argv.find(argument=>argument.startsWith("--modules=")),[manifestPath,outputArgument,...snapshots]=argv.filter(argument=>argument!==flag);
 if(!manifestPath||!outputArgument)fail("manifest and output required");
 const manifest=JSON.parse(readFileSync(manifestPath,"utf8")),modules=flag?resolve(flag.slice(10)):undefined,declared=manifest.inputs??[],staged=manifest.aliases!==undefined,outputPath=resolve(outputArgument);
 if(manifest.version!==1)fail("manifest version");
 if(worker&&(!staged||(manifest.outputs??[]).length))fail("a worker action declares aliased inputs and no outputs");
 const rule=staged?manifest.scratch[{win32:"win32",darwin:"darwin"}[process.platform]??"linux"]:undefined,fresh=()=>randomBytes(rule.workDigits/2).toString("hex"),scratch=staged?resolve(rule.base.replace("{drive}",parse(process.cwd()).root.replace(/[\\/]$/,"")),worker?(held.work??=fresh()):fresh()):resolve(".hermetic",sha(outputArgument).slice(0,16));
 const source=resolve("s"),workspace=staged?join(scratch,rule.workspace):source;
 if(staged&&!worker)rmSync(scratch,{recursive:true,force:true});
 try{if(!staged&&lstatSync(workspace).isSymbolicLink())fail("an isolated execution root is required: declare the inputs");}catch(error){if(error.code==="ENOENT")mkdirSync(workspace,{recursive:true});else throw error;}
 const inside=path=>{const target=resolve(workspace,path),inner=relative(workspace,target);if(inner===".."||inner.startsWith(".."+sep)||isAbsolute(inner))fail("path escapes the execution root: "+path);return target;},
  expand=value=>value.replaceAll("{workspaceRoot}",workspace).replaceAll("{projectRoot}",inside(manifest.root||".")).replaceAll("{projectName}",manifest.project),
  normalized=info=>info.isDirectory()||(info.mode&0o111)?0o755:0o644,
  sorted=directory=>readdirSync(directory).sort(order);
 try{
 if(worker)mkdirSync(workspace,{recursive:true});
 const present=new Set();
 if(staged)for(const path of declared){
  const cut=path.lastIndexOf("/"),alias=manifest.aliases[cut<0?"":path.slice(0,cut)];if(alias===undefined)fail("an input directory without a declared alias: "+path);
  const from=resolve(alias,path.slice(cut+1)),target=inside(path);present.add(path);
  if(worker){const stamp=signature(statSync(from,{bigint:true}));if(held.staged.get(path)===stamp&&existsSync(target))continue;held.staged.set(path,stamp);rmSync(target,{force:true});}
  mkdirSync(dirname(target),{recursive:true});clone(from,target);
 }
 if(worker)for(const path of[...held.staged.keys()])if(path[0]!=="\0"&&!present.has(path)){rmSync(inside(path),{force:true});held.staged.delete(path);}
 for(const marker of manifest.markers??[]){const target=inside(marker);try{lstatSync(target);}catch(error){if(error.code!=="ENOENT")throw error;mkdirSync(dirname(target),{recursive:true});writeFileSync(target,"{}");}}
 const links=JSON.stringify([manifest.nodeModules??[],modules??""]);
 if(!worker||held.links!==links){
  for(const row of held.rows)rmSync(inside(row.path),{recursive:true,force:true});
  for(const row of manifest.nodeModules??[]){
   const directory=inside(row.path);
   for(const[name,path]of Object.entries(row.workspace))link(inside(path),join(directory,name));
   for(const[name,path]of Object.entries(row.tree)){if(!modules)fail("a dependency tree is required for "+row.path);link(join(modules,path),join(directory,name));}
   for(const[name,target]of Object.entries(row.bin)){mkdirSync(join(directory,".bin"),{recursive:true});shim(target,join(directory,".bin",name));}
  }
  if(worker){held.rows=manifest.nodeModules??[];held.links=links;}
 }
 const restored=new Set();
 for(const snapshot of snapshots){
  const content=JSON.parse(readFileSync(join(snapshot,"snapshot.json"),"utf8"));if(content.version!==1||!Array.isArray(content.entries))fail("snapshot rows");
  for(const entry of [...content.entries].sort((left,right)=>left.path.length-right.path.length||order(left.path,right.path))){
   const target=inside(entry.path);if(!/^[a-f0-9]{32}$/.test(entry.file??"0".repeat(32)))fail("snapshot file identity");
   if(entry.kind==="directory"){mkdirSync(target,{recursive:true});continue;}
   if(entry.kind!=="file")fail("snapshot kind refused: "+entry.kind);
   const key="\0"+entry.path,stamp=entry.sha256+":"+entry.mode;restored.add(key);
   if(worker&&held.staged.get(key)===stamp&&existsSync(target))continue;
   if(worker&&held.staged.has(key))rmSync(target,{force:true});
   try{lstatSync(target);fail("a dependency output overlaps a declared input: "+entry.path);}catch(error){if(error.code!=="ENOENT")throw error;}
   mkdirSync(dirname(target),{recursive:true});copyFileSync(join(snapshot,"files",entry.file),target);chmodSync(target,entry.mode&0o755|0o600);
   if(worker)held.staged.set(key,stamp);
  }
 }
 if(worker)for(const key of[...held.staged.keys()])if(key[0]==="\0"&&!restored.has(key)){rmSync(inside(key.slice(1)),{force:true});held.staged.delete(key);}
 const files=()=>{const found=new Map(),visit=directory=>{for(const name of sorted(directory)){const path=join(directory,name),info=lstatSync(path);if(info.isDirectory())visit(path);else found.set(slash(relative(workspace,path)),info.size+":"+info.mtimeMs+":"+info.ino);}};visit(workspace);return found;};
 const before=files();
 if(worker)for(const name of["home","tmp","launcher"])rmSync(join(scratch,name),{recursive:true,force:true});
 mkdirSync(join(scratch,"home"),{recursive:true});mkdirSync(join(scratch,"tmp"),{recursive:true});
 const launcher=join(scratch,"launcher");mkdirSync(launcher,{recursive:true});shim(executable,join(launcher,"repo"));
 const binaries=[join(workspace,"node_modules",".bin"),...(manifest.nodeModules??[]).map(row=>join(inside(row.path),".bin"))].filter((path,index,all)=>all.indexOf(path)===index);
 const environment={PATH:[launcher,dirname(executable),...binaries].join(process.platform==="win32"?";":":"),HOME:join(scratch,"home"),TMPDIR:join(scratch,"tmp"),TEMP:join(scratch,"tmp"),TMP:join(scratch,"tmp"),LC_ALL:"C",SEMIO_LANGUAGE:"en",TZ:"UTC",SOURCE_DATE_EPOCH:"0",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",SEMIO_BUILD_TOOL:"bazel",SEMIO_HERMETIC:"1",SEMIO_WORKSPACE_ROOT:workspace,SEMIO_TASK_PROJECT:manifest.project,SEMIO_TASK_TARGET:manifest.target,...(manifest.configuration?{SEMIO_TASK_CONFIGURATION:manifest.configuration}:{}),...(manifest.inputEnv??{}),...(manifest.env??{})};
 if(manifest.entry){const entry=await import(pathToFileURL(inside(manifest.entry)).href);Object.assign(environment,entry.hermeticEnvironmentV1({workspace,providers:manifest.providers??{},maximumElapsedMilliseconds:manifest.maximumElapsedMilliseconds,buildMode:environment.SEMIO_BUILD_MODE??"prototype"}));}
 if(worker&&manifest.service){clearTimeout(held.idle);held.active=workspace;environment[manifest.service.environment]=await serve(manifest,workspace,inside);}
 const cwd=manifest.cwd?resolve(workspace,expand(manifest.cwd)):workspace;inside(relative(workspace,cwd)||".");mkdirSync(cwd,{recursive:true});
 const quote=value=>"'"+value.replaceAll("'","'\\''")+"'",suffix=manifest.args.length&&manifest.forwardAllArgs!==false?" "+manifest.args.map(quote).join(" "):"";
 const children=new Set(),stop=()=>{for(const child of children)try{if(process.platform==="win32")Bun.spawnSync(["taskkill","/PID",String(child.pid),"/T","/F"]);else process.kill(-child.pid,"SIGKILL");}catch{child.kill("SIGKILL");}};
 if(worker){session.signal.addEventListener("abort",stop,{once:true});if(session.signal.aborted)stop();}
 const run=async command=>{
  const child=Bun.spawn([executable,"exec",expand(command)+suffix],{cwd,env:environment,stdin:"ignore",stdout:worker?"pipe":"inherit",stderr:worker?"pipe":"inherit",detached:worker&&process.platform!=="win32"});children.add(child);
  const[code,out,err]=await Promise.all([child.exited,worker?new Response(child.stdout).text():"",worker?new Response(child.stderr).text():""]);children.delete(child);
  if(worker)session.chunks.push(out,err);
  if(session?.signal.aborted)fail("cancelled");
  if(code!==0)fail("command failed ("+code+"): "+command);
 };
 try{if(manifest.parallel===false)for(const command of manifest.commands)await run(command);else await Promise.all(manifest.commands.map(run));}finally{if(worker)session.signal.removeEventListener("abort",stop);}
 const roots=new Set();
 for(const pattern of manifest.outputs){
  const path=slash(relative(workspace,resolve(workspace,expand(pattern))));inside(path);
  if(/[?*{[]/.test(path)){for(const match of new Bun.Glob(path).scanSync({cwd:workspace,onlyFiles:false,dot:true}))roots.add(slash(match));}
  else try{lstatSync(inside(path));roots.add(path);}catch(error){if(error.code!=="ENOENT")throw error;}
 }
 const ordered=[...roots].sort(order).filter(path=>![...roots].some(parent=>parent!==path&&path.startsWith(parent+"/"))),entries=[];
 if(worker)rmSync(outputPath,{recursive:true,force:true});
 mkdirSync(join(outputPath,"files"),{recursive:true});
 const visit=path=>{
  const target=inside(path),info=lstatSync(target);
  if(info.isSymbolicLink())fail("a hermetic output must not be a symbolic link: "+path);
  if(info.isDirectory()){entries.push({path,kind:"directory",mode:0o755});for(const name of sorted(target))visit(path+"/"+name);return;}
  if(!info.isFile())fail("a hermetic output must be a file or directory: "+path);
  const bytes=readFileSync(target),id=sha(path).slice(0,32);writeFileSync(join(outputPath,"files",id),bytes);entries.push({path,kind:"file",mode:normalized(info),file:id,sha256:sha(bytes)});
 };
 for(const path of ordered)visit(path);
 entries.sort((left,right)=>order(left.path,right.path));
 const claimed=new Set(entries.map(entry=>entry.path)),undeclared=[...files()].filter(([path,identity])=>before.get(path)!==identity&&!claimed.has(path)&&![...roots].some(root=>path.startsWith(root+"/"))).map(([path])=>path).sort(order);
 if(undeclared.length)fail("undeclared writes outside the declared outputs: "+undeclared.slice(0,16).join(", "));
 writeFileSync(join(outputPath,"snapshot.json"),JSON.stringify({version:1,roots:[...roots].sort(order),entries}));
 if(!worker)rmSync(scratch,{recursive:true,force:true});
 }catch(error){
  if(worker){held.active=undefined;held.staged.clear();held.rows=[];held.links="";rmSync(scratch,{recursive:true,force:true});}
  throw error;
 }finally{
  if(worker&&manifest.service){held.active=undefined;const limit=manifest.service.idleMilliseconds;if(limit>0){held.idle=setTimeout(()=>held.service?.instance.release?.(),limit);held.idle.unref();}}
 }
}
if(process.argv.includes("--persistent_worker")){
 const pending=new Map(),write=value=>process.stdout.write(JSON.stringify(value)+"\n");let buffer="",queue=Promise.resolve();
 const job=async request=>{
  const control=new AbortController(),session={signal:control.signal,chunks:[]};pending.set(request.requestId,control);let exitCode=0;
  try{await execute(request.arguments??[],session);}catch(error){exitCode=1;session.chunks.push(String(error?.stack??error)+"\n");}
  pending.delete(request.requestId);write({exitCode,output:session.chunks.join(""),requestId:request.requestId??0,...(control.signal.aborted?{wasCancelled:true}:{})});
 };
 process.stdin.setEncoding("utf8");
 process.stdin.on("data",chunk=>{
  buffer+=chunk;
  for(let end=buffer.indexOf("\n");end>=0;end=buffer.indexOf("\n")){
   const line=buffer.slice(0,end).trim();buffer=buffer.slice(end+1);if(!line)continue;
   const request=JSON.parse(line);if(request.cancel){pending.get(request.requestId)?.abort();continue;}
   queue=queue.then(()=>job(request));
  }
 });
 process.stdin.on("end",()=>process.exit(0));
}else{
 const given=process.argv.slice(2),arguments_=given.length===1&&given[0].startsWith("@")?readFileSync(given[0].slice(1),"utf8").split("\n").filter(Boolean):given;
 await execute(arguments_);
}
