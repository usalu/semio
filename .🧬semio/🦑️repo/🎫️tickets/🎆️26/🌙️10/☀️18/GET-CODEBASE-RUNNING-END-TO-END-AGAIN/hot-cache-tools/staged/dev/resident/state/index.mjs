/** 🗃 Event-sourced workspace state: journal of file events, owner digests, task keys, memo and the membership frontier, all as projections of watcher batches. */
import {createHash} from "node:crypto";
import {lstat,readFile,readdir} from "node:fs/promises";

const sha=(...parts)=>{const hash=createHash("sha256");for(const part of parts)hash.update(part);return hash.digest("hex");};
const byCodePoint=(a,b)=>a<b?-1:a>b?1:0;
const missing=error=>["ENOENT","ENOTDIR","ELOOP"].includes(error?.code);

/** 🚫 The exclusion rule shared with the native watcher: rooted prefixes and component names. */
export function createFilterV1({rooted=[],names=[]}={}){
 const prefixes=rooted.map(item=>item.replace(/^\/+|\/+$/gu,"")),set=new Set(names);
 return Object.freeze({
  excluded(path){
   if(!path)return false;
   for(const prefix of prefixes)if(path===prefix||(path.length>prefix.length&&path.startsWith(prefix)&&path.charCodeAt(prefix.length)===47))return true;
   if(set.size)for(const part of path.split("/"))if(set.has(part))return true;
   return false;
  },
 });
}

/** 🌳 Longest-prefix owner lookup over owner roots. */
export function createOwnerIndexV1(roots){
 const top={children:new Map(),owner:undefined};
 for(const[root,owner]of roots){
  let node=top;
  if(root!=="")for(const part of root.split("/")){let next=node.children.get(part);if(!next){next={children:new Map(),owner:undefined};node.children.set(part,next);}node=next;}
  node.owner=owner;
 }
 const collect=(node,out)=>{if(node.owner!==undefined)out.push(node.owner);for(const child of node.children.values())collect(child,out);return out;};
 return Object.freeze({
  ownerOf(path){
   let node=top,found=top.owner;
   if(path==="")return found;
   for(const part of path.split("/")){node=node.children.get(part);if(!node)break;if(node.owner!==undefined)found=node.owner;}
   return found;
  },
  ownersUnder(prefix){
   let node=top;
   if(prefix!=="")for(const part of prefix.split("/")){node=node.children.get(part);if(!node)return[];}
   return collect(node,[]);
  },
 });
}

/** 🧮 Creates the state. `options.root` is the canonical worktree root. */
export function createWorkspaceStateV1(options){
 const{root,instance,policy,filter,fence=undefined,now=()=>Date.now()}=options;
 const owners=new Map(),tasks=new Map(),members=new Map(),directories=new Map(),memo=new Map();
 const stats={events:0,batches:0,statCalls:0,hashCalls:0,hashedBytes:0,ownerRefreshes:0,envHashes:0,losses:0,reconciles:0,ignored:0,unowned:0};
 let seq=0,lostSeq=0,envSeq=0,envDigest="",envDigestSeq=-1,envSalt="",envRefreshing,ownerIndex=createOwnerIndexV1([]),outputIndex=createOwnerIndexV1([]),invalidOutputs=new Set(),running=new Set(),envPaths=new Set(),envPrefixes=[],journal=[],journalRows=0,journalFloor=0,generation=null,closureCache=new Map(),trust="exact";
 const abs=path=>path===""?root:root+"/"+path;
 const isIgnored=path=>path===""||(fence!==undefined&&(path===fence||path.startsWith(fence+"/")))||filter.excluded(path);
 const safeStat=async path=>{stats.statCalls++;try{return await lstat(abs(path));}catch(error){if(missing(error))return undefined;throw error;}};
 const newOwner=item=>({name:item.name,root:item.root,deps:item.deps??[],tasks:new Map(),files:new Map(),dirtyFiles:new Set(),seq:0,digest:undefined,digestSeq:-1,refreshing:undefined,statCheck:false});

 const configure=({owners:list,env})=>{
  owners.clear();tasks.clear();closureCache=new Map();
  for(const item of list){
   const owner=newOwner(item);
   for(const task of item.tasks??[]){const id=item.name+":"+task.name;const entry={id,owner:item.name,name:task.name,command:task.command,dependsOn:task.dependsOn??[],outputs:task.outputs??[],cacheable:task.cacheable!==false,key:undefined,keySeq:-1};owner.tasks.set(task.name,entry);tasks.set(id,entry);}
   owners.set(item.name,owner);
  }
  ownerIndex=createOwnerIndexV1(list.map(item=>[item.root,item.name]));
  outputIndex=createOwnerIndexV1([...tasks.values()].flatMap(task=>task.outputs.map(output=>[output.replace(/\/+$/u,""),task.id])));
  invalidOutputs=new Set();running=new Set();
  envPaths=new Set(env?.paths??[]);envPrefixes=(env?.prefixes??[]).map(item=>item.replace(/\/+$/u,""));
 };

 const classify=path=>{
  if(envPaths.has(path)||envPrefixes.some(prefix=>path===prefix||path.startsWith(prefix+"/")))return{env:true};
  const owner=ownerIndex.ownerOf(path);
  return owner===undefined?{}:{owner};
 };

 const track=(path,mode)=>{members.set(path,mode);const owner=ownerIndex.ownerOf(path);if(owner!==undefined&&!envPaths.has(path)){const files=owners.get(owner).files;if(!files.has(path))files.set(path,{digest:undefined,stamp:undefined});}};
 const untrack=path=>{members.delete(path);const owner=ownerIndex.ownerOf(path);if(owner!==undefined)owners.get(owner)?.files.delete(path);};

 /** 🔁 Replaces the owner and task tables while the file membership stays: files are repartitioned, digests and keys are recomputed on demand, remembered results survive only if their keys still match and their outputs re-verify. */
 const reconfigure=config=>{
  configure(config);
  for(const[path,mode]of members)track(path,mode);
  seq++;for(const owner of owners.values())owner.seq=seq;
  envSeq=seq;envDigestSeq=-1;
  for(const id of memo.keys())invalidOutputs.add(id);
 };

 /** 📥 Loads the initial membership from a frontier-shaped listing (native scan or walk). */
 const load=listing=>{
  members.clear();directories.clear();
  for(const owner of owners.values()){owner.files=new Map();owner.dirtyFiles.clear();owner.digest=undefined;owner.digestSeq=-1;owner.statCheck=false;}
  for(const[path,mode]of listing.members)if(!isIgnored(path)&&outputIndex.ownerOf(path)===undefined)track(path,mode);
  for(const[path,mtime,ino]of listing.directories)if(path===""||!isIgnored(path))directories.set(path,[mtime,ino]);
  seq++;for(const owner of owners.values())owner.seq=seq;
  envSeq=seq;closureCache=new Map();
  for(const task of tasks.values()){task.key=undefined;task.keySeq=-1;}
 };

 const pushJournal=entry=>{
  journal.push(entry);journalRows+=entry.changes?.length??1;
  while(journalRows>policy.journalMaximumEvents&&journal.length>1){const old=journal.shift();journalRows-=old.changes?.length??1;journalFloor=old.seq;}
 };

 const walkTree=async rel=>{
  const files=[],dirs=[];
  const recurse=async dir=>{
   let entries;
   try{entries=await readdir(abs(dir),{withFileTypes:true});}catch(error){if(missing(error))return;throw error;}
   const subdirs=[];
   await Promise.all(entries.map(async entry=>{
    const child=dir===""?entry.name:dir+"/"+entry.name;
    if(isIgnored(child))return;
    const status=await safeStat(child);if(!status)return;
    if(status.isDirectory()){dirs.push([child,status.mtimeMs,status.ino]);subdirs.push(child);}
    else if(status.isFile())files.push([child,status.mode]);
   }));
   await Promise.all(subdirs.map(recurse));
  };
  await recurse(rel);
  return{files,dirs};
 };

 /** 📥 Applies one watcher batch: stat each distinct path once, update members and owners, append to the journal. */
 const apply=async(rows,at=now())=>{
  stats.batches++;stats.events+=rows.length;
  const distinct=new Map();
  for(const[kind,dir,path]of rows){if(isIgnored(path)){stats.ignored++;continue;}const old=distinct.get(path);distinct.set(path,{kind,dir:Boolean(dir)||old?.dir===true});}
  const changes=[];
  const touch=(path,structural)=>{
   const target=classify(path);
   if(target.env){envSeq=++seq;}
   else if(target.owner!==undefined){const owner=owners.get(target.owner);owner.seq=++seq;owner.dirtyFiles.add(path);}
   else stats.unowned++;
   void structural;
  };
  const adopt=async(path,status)=>{
   directories.set(path,[status.mtimeMs,status.ino]);changes.push({path,kind:"added",directory:true});
   const tree=await walkTree(path);
   for(const[dir,mtime,ino]of tree.dirs)if(!directories.has(dir)){directories.set(dir,[mtime,ino]);changes.push({path:dir,kind:"added",directory:true});}
   for(const[file,mode]of tree.files){changes.push({path:file,kind:members.has(file)?"changed":"added",directory:false});track(file,mode);touch(file,true);}
   touch(path,true);
  };
  for(const[path,info]of distinct){
   const producer=outputIndex.ownerOf(path);
   if(producer!==undefined){if(!running.has(producer))invalidOutputs.add(producer);continue;}
   const segments=path.split("/");
   for(let depth=1;depth<segments.length;depth++){
    const ancestor=segments.slice(0,depth).join("/");
    if(directories.has(ancestor)||isIgnored(ancestor))continue;
    const known=await safeStat(ancestor);
    if(known?.isDirectory())await adopt(ancestor,known);
    break;
   }
   const status=await safeStat(path);
   if(!status){
    const prefix=path+"/";
    if(directories.has(path)){
     for(const member of[...members.keys()])if(member.startsWith(prefix)){untrack(member);changes.push({path:member,kind:"removed",directory:false});touch(member,true);}
     for(const directory of[...directories.keys()])if(directory===path||directory.startsWith(prefix)){directories.delete(directory);changes.push({path:directory,kind:"removed",directory:true});}
    }else if(members.has(path)){untrack(path);changes.push({path,kind:"removed",directory:false});}
    touch(path,true);
    continue;
   }
   if(status.isDirectory()){
    const was=directories.get(path);
    if(!was)await adopt(path,status);
    else if(was[0]!==status.mtimeMs||was[1]!==status.ino){directories.set(path,[status.mtimeMs,status.ino]);changes.push({path,kind:"changed",directory:true});}
    touch(path,true);
    continue;
   }
   if(status.isFile()){
    const existed=members.has(path);track(path,status.mode);changes.push({path,kind:existed?"changed":"added",directory:false});
   }
   touch(path,!members.has(path));
  }
  const parents=new Set();
  for(const change of changes)if(change.kind!=="changed"||!change.directory){const cut=change.path.lastIndexOf("/");parents.add(cut<0?"":change.path.slice(0,cut));}
  for(const parent of parents){
   if(!directories.has(parent)||distinct.has(parent))continue;
   const status=await safeStat(parent),was=directories.get(parent);
   if(status?.isDirectory()&&(was[0]!==status.mtimeMs||was[1]!==status.ino)){directories.set(parent,[status.mtimeMs,status.ino]);changes.push({path:parent,kind:"changed",directory:true});}
  }
  const published=++seq;
  pushJournal({seq:published,at,changes});
  return published;
 };

 /** 🕳 A gap in the events: owners under the scope are re-proven by stat before their digests are trusted. */
 const lost=(scope,reason,at=now())=>{
  const batch=++seq;lostSeq=batch;stats.losses++;
  const names=scope===""?[...owners.keys()]:[...new Set([...ownerIndex.ownersUnder(scope),...(ownerIndex.ownerOf(scope)===undefined?[]:[ownerIndex.ownerOf(scope)])])];
  for(const name of names){const owner=owners.get(name);if(owner){owner.seq=batch;owner.statCheck=true;}}
  if(scope===""||classify(scope).env)envSeq=batch;
  for(const id of tasks.keys())if(memo.has(id)&&(scope===""||tasks.get(id).outputs.some(output=>output===scope||output.startsWith(scope+"/")||scope.startsWith(output+"/"))))invalidOutputs.add(id);
  pushJournal({seq:batch,at,lost:true,scope,reason});
  return batch;
 };

 /** 🧹 Re-proves membership under a scope against the disk and feeds the differences through the normal path. */
 const reconcile=async(scope="")=>{
  stats.reconciles++;
  const tree=await walkTree(scope),prefix=scope===""?"":scope+"/",rows=[];
  const actual=new Map(tree.files);
  for(const[path,mode]of actual){const was=members.get(path);if(was===undefined)rows.push(["a",0,path]);else if(was!==mode)rows.push(["m",0,path]);}
  for(const path of members.keys())if((prefix===""||path.startsWith(prefix))&&!actual.has(path))rows.push(["r",0,path]);
  for(const[dir,mtime,ino]of tree.dirs){const was=directories.get(dir);if(!was)rows.push(["a",1,dir]);else if(was[0]!==mtime||was[1]!==ino)rows.push(["m",1,dir]);}
  for(const dir of directories.keys())if(dir!==scope&&(prefix===""||dir.startsWith(prefix))&&!tree.dirs.some(item=>item[0]===dir))rows.push(["r",1,dir]);
  if(rows.length)await apply(rows);
  return rows.length;
 };

 const dirtyOwnersSince=from=>{const out=new Set();for(const owner of owners.values())if(owner.seq>from)out.add(owner.name);return out;};
 const cursor=()=>({instance,seq});

 /** 📖 Net membership changes after a cursor, folded in order; incomplete when the cursor is another instance, predates the journal, or a loss happened since. */
 const since=from=>{
  if(from.instance!==instance||from.seq<journalFloor)return{complete:false,reason:"cursor"};
  const net=new Map();
  for(const entry of journal){
   if(entry.seq<=from.seq)continue;
   if(entry.lost)return{complete:false,reason:"lost",scope:entry.scope};
   for(const change of entry.changes){
    const was=net.get(change.path);
    if(!was){net.set(change.path,{...change});continue;}
    if(was.kind==="added"&&change.kind==="removed")net.delete(change.path);
    else if(was.kind==="removed"&&change.kind==="added")net.set(change.path,{path:change.path,kind:"changed",directory:change.directory});
    else if(was.kind==="added")continue;
    else net.set(change.path,{...change,kind:change.kind==="removed"?"removed":"changed"});
   }
  }
  const rows=[...net.values()].sort((a,b)=>byCodePoint(a.path,b.path)).map(change=>{
   if(change.kind==="removed")return change;
   return change.directory?{...change,stamp:directories.get(change.path)}:{...change,mode:members.get(change.path)};
  });
  return{complete:true,seq,changes:rows};
 };

 const limit=async(items,width,body)=>{
  let next=0;
  await Promise.all(Array.from({length:Math.min(width,items.length)},async()=>{while(next<items.length){const at=next++;await body(items[at]);}}));
 };

 /** 🔏 Brings the digest of an owner current: rehashes only dirty or unknown files and composes the owner digest. */
 const refreshOwner=name=>{
  const owner=owners.get(name);
  if(!owner)throw new Error("Unknown owner "+name);
  if(owner.digest!==undefined&&owner.digestSeq>=owner.seq&&!owner.statCheck&&trust==="exact")return Promise.resolve(owner.digest);
  if(owner.refreshing)return owner.refreshing;
  const startSeq=seq;
  owner.refreshing=(async()=>{
   stats.ownerRefreshes++;
   const pending=[...owner.dirtyFiles];owner.dirtyFiles.clear();
   const proving=owner.statCheck||trust!=="exact";owner.statCheck=false;
   try{
    const files=owner.files,todo=new Set(pending.filter(path=>files.has(path)));
    for(const[path,rec]of files)if(rec.digest===undefined)todo.add(path);
    if(proving)await limit([...files.keys()],32,async path=>{
     const rec=files.get(path);
     if(!rec||rec.stamp===undefined){todo.add(path);return;}
     const status=await safeStat(path);
     if(!status||!status.isFile()){untrack(path);return;}
     if(status.mtimeMs!==rec.stamp.mtimeMs||status.size!==rec.stamp.size||status.ino!==rec.stamp.ino||rec.stamp.hashedAt-status.mtimeMs<policy.racyWindowMilliseconds)todo.add(path);
    });
    await limit([...todo],32,async path=>{
     const rec=files.get(path);if(!rec)return;
     const status=await safeStat(path);
     if(!status||!status.isFile()){untrack(path);return;}
     const hashedAt=now();stats.hashCalls++;const bytes=await readFile(abs(path));stats.hashedBytes+=bytes.length;
     rec.digest=sha(bytes);rec.stamp={mtimeMs:status.mtimeMs,size:status.size,ino:status.ino,hashedAt};
    });
    const lines=[];for(const path of[...files.keys()].sort(byCodePoint))lines.push(path+"\0"+files.get(path).digest+"\n");
    owner.digest=sha(lines.join(""));owner.digestSeq=startSeq;
    return owner.digest;
   }catch(error){
    for(const path of pending)owner.dirtyFiles.add(path);owner.statCheck=owner.statCheck||proving;throw error;
   }
  })().finally(()=>{owner.refreshing=undefined;});
  return owner.refreshing;
 };

 const closureOf=name=>{
  let cached=closureCache.get(name);
  if(cached)return cached;
  const seen=new Set([name]),order=[name];
  for(let at=0;at<order.length;at++){const owner=owners.get(order[at]);for(const dep of owner?.deps??[])if(!seen.has(dep)){seen.add(dep);order.push(dep);}}
  cached=order.sort(byCodePoint);closureCache.set(name,cached);return cached;
 };

 const closureSeq=name=>{let max=envSeq;for(const owner of closureOf(name)){const s=owners.get(owner)?.seq??0;if(s>max)max=s;}return max;};

 /** 🌍 Digest of the environment inputs (named files and files under named prefixes) plus the salt the graph port supplies (tool versions, policy). */
 const refreshEnvironment=()=>{
  if(envDigestSeq>=envSeq&&trust==="exact")return Promise.resolve(envDigest);
  if(envRefreshing)return envRefreshing;
  const at=seq;
  envRefreshing=(async()=>{
   const paths=new Set(envPaths);
   for(const member of members.keys())if(envPrefixes.some(prefix=>member.startsWith(prefix+"/")))paths.add(member);
   const lines=[];
   for(const path of[...paths].sort(byCodePoint)){stats.envHashes++;try{lines.push(path+"\0"+sha(await readFile(abs(path)))+"\n");}catch(error){if(missing(error))lines.push(path+"\0missing\n");else throw error;}}
   envDigest=sha(envSalt+"\0"+lines.join(""));envDigestSeq=at;return envDigest;
  })().finally(()=>{envRefreshing=undefined;});
  return envRefreshing;
 };

 /** 🔑 The input key of a task: command, environment digest, owner digest and the keys of dependency tasks. */
 const taskKey=(id,visiting=new Map())=>{
  const task=tasks.get(id);if(!task)throw new Error("Unknown task "+id);
  const hit=visiting.get(id);if(hit)return hit;
  const atSeq=closureSeq(task.owner);
  if(task.key!==undefined&&task.keySeq>=atSeq&&trust==="exact"){const ready=Promise.resolve(task.key);visiting.set(id,ready);return ready;}
  const promise=(async()=>{
   let environment=visiting.get("\0env");if(!environment){environment=refreshEnvironment();visiting.set("\0env",environment);}
   const[env,digest]=await Promise.all([environment,refreshOwner(task.owner)]);
   const deps=await Promise.all(task.dependsOn.map(dep=>taskKey(dep,visiting)));
   const key=sha(JSON.stringify([task.command,env,digest,deps]));
   task.key=key;task.keySeq=atSeq;return key;
  })();
  visiting.set(id,promise);return promise;
 };

 const memoCurrent=async id=>{const entry=memo.get(id);return entry!==undefined&&tasks.get(id)?.cacheable!==false&&entry.key===await taskKey(id);};
 const outputsInvalid=id=>invalidOutputs.has(id);
 const clearOutputsInvalid=id=>{invalidOutputs.delete(id);};
 const beginTask=id=>{running.add(id);};
 const endTask=id=>{running.delete(id);invalidOutputs.delete(id);};

 const topological=ids=>{
  const order=[],seen=new Set();
  const visit=id=>{if(seen.has(id))return;seen.add(id);for(const dep of tasks.get(id).dependsOn)visit(dep);order.push(id);};
  for(const id of ids)visit(id);
  return order;
 };

 /** 🧱 Tasks of the closure of `id`, dependencies first, whose memo is missing or stale: the rebuild plan. */
 const stalePlan=async id=>{
  const order=topological([id]),current=await Promise.all(order.map(memoCurrent));
  return order.filter((_,at)=>!current[at]);
 };

 const dropCaches=()=>{
  memo.clear();
  for(const owner of owners.values()){owner.digest=undefined;owner.digestSeq=-1;owner.dirtyFiles.clear();for(const rec of owner.files.values()){rec.digest=undefined;rec.stamp=undefined;}}
  for(const task of tasks.values()){task.key=undefined;task.keySeq=-1;}
 };

 return{
  configure,reconfigure,load,apply,lost,reconcile,cursor,since,dirtyOwnersSince,refreshOwner,closureOf,taskKey,memoCurrent,stalePlan,dropCaches,outputsInvalid,clearOutputsInvalid,beginTask,endTask,
  setEnvironmentSalt:value=>{envSalt=value;envDigestSeq=-1;},
  memo:{get:id=>memo.get(id),set:(id,entry)=>{memo.set(id,entry);while(memo.size>policy.memoMaximumEntries)memo.delete(memo.keys().next().value);},delete:id=>memo.delete(id),size:()=>memo.size},
  owners,tasks,members,directories,stats,topological,
  get seq(){return seq;},get lostSeq(){return lostSeq;},get envSeq(){return envSeq;},get journalFloor(){return journalFloor;},
  setTrust:value=>{trust=value;},get trust(){return trust;},
  ownerOf:path=>ownerIndex.ownerOf(path),classifyEnvironment:path=>classify(path).env===true,
  setGeneration:value=>{generation=value;},get generation(){return generation;},
 };
}

/** 🖊 Signature of a task's outputs by bounded stat; undefined when the output tree exceeds the bound. */
export async function outputSignatureV1(root,outputs,maximumEntries){
 const rows=[];let budget=maximumEntries;
 const walk=async rel=>{
  let status;try{status=await lstat(root+"/"+rel);}catch(error){if(missing(error)){rows.push(rel+"\0missing");return true;}throw error;}
  rows.push(rel+"\0"+status.mtimeMs+"\0"+status.size+"\0"+status.ino);
  if(--budget<0)return false;
  if(status.isDirectory())for(const name of(await readdir(root+"/"+rel)).sort())if(!(await walk(rel+"/"+name)))return false;
  return true;
 };
 for(const output of outputs)if(!(await walk(output)))return undefined;
 return sha(rows.join("\n"));
}
