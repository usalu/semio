#!/usr/bin/env python3
"""q4: replica: a volume without symbolic links keeps the link entries (X16). usage: q4-links.py <module-root>"""
import sys
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:80],s.count(old))
    return s.replace(old,new)
p=root+"/🔁️replica/🟦️.ts"
s=open(p,encoding="utf8").read()
s=sub(s,'modeless?:boolean;lock?:boolean;','modeless?:boolean;symlinks?:boolean;lock?:boolean;')
s=sub(s,'persistMilliseconds:number;modeless:boolean;now:()=>number','persistMilliseconds:number;modeless:boolean;symlinks:boolean|undefined;now:()=>number')
s=sub(s,'modeless:options.modeless??process.platform==="win32",now:','modeless:options.modeless??process.platform==="win32",symlinks:options.symlinks,now:')
s=sub(s,'const token=():string=>randomBytes(8).toString("hex");','''const token=():string=>randomBytes(8).toString("hex");
const capable=new WeakMap<object,boolean>();

/** 🔗️ Tells whether the volume that holds `directory` can hold symbolic links (a Windows account without the privilege or the developer mode cannot): one link is created, read back and removed. */
async function linksWork(directory:string):Promise<boolean>{
 const name=join(directory,`probe-${token()}`);
 try{await mkdir(directory,{recursive:true});await symlink("probe-target",name);return await readlink(name)==="probe-target";}
 catch{return false;}
 finally{await rm(name,{force:true}).catch(()=>{});}
}
const linkless=(replica:ReplicaV1,id:number):boolean=>capable.get(replica)===false&&replica.files.operationAt(id)==="link";''')
s=sub(s,'  restoreHistory(replica);\n  const hot=local.memo.get("hot");','  restoreHistory(replica);\n  capable.set(replica,settings.symlinks??await linksWork(paths.tmp));\n  const hot=local.memo.get("hot");')
s=sub(s,'''  if(indexRealmV1(path)||queue.hasId(id)||manifest.entryAt(id).operation==="delete"||!sharedV1(path,false,replica.ignore))continue;
  try{await lstat(absolute(settings.worktree,path));}''','''  if(indexRealmV1(path)||queue.hasId(id)||manifest.entryAt(id).operation==="delete"||!sharedV1(path,false,replica.ignore)||linkless(replica,id))continue;
  try{await lstat(absolute(settings.worktree,path));}''')
s=sub(s,'''for(const id of replica.files.ids())if(id<walk.universe&&!walk.seen.has(id)&&replica.files.operationAt(id)!=="delete"&&!replica.queue.hasId(id)&&!indexRealmV1(replica.book.name(id)))candidates.push''','''for(const id of replica.files.ids())if(id<walk.universe&&!walk.seen.has(id)&&replica.files.operationAt(id)!=="delete"&&!replica.queue.hasId(id)&&!indexRealmV1(replica.book.name(id))&&!linkless(replica,id))candidates.push''')
s=sub(s,'''return state!==undefined&&state.operation!=="delete"&&!replica.queue.has(path);});''','''return state!==undefined&&state.operation!=="delete"&&!replica.queue.has(path)&&!(state.operation==="link"&&capable.get(replica)===false);});''')
s=sub(s,'''catch(error){return gone(error)&&id!==undefined&&files.live(id)&&files.operationAt(id)!=="delete"&&!queue.hasId(id);}''','''catch(error){return gone(error)&&id!==undefined&&files.live(id)&&files.operationAt(id)!=="delete"&&!queue.hasId(id)&&!linkless(replica,id);}''')
s=sub(s,''' if(entry.operation==="link"){
  try{
   if(lose)await keepLost(replica,path,target,here!,entry.hlc);''',''' if(entry.operation==="link"&&capable.get(replica)===false&&(here===undefined||here.isSymbolicLink())){
  noteOnce(replica,`skipped:${path}`,{event:"skipped",path,reason:"symlink-unsupported"});
  setFile(replica,path,{...plain(entry),mtimeMs:0});
  return "applied";
 }
 if(entry.operation==="link"){
  try{
   if(lose)await keepLost(replica,path,target,here!,entry.hlc);''')
s=sub(s,'''   if(code(error)==="EPERM"||code(error)==="EACCES"||code(error)==="ENOSYS"||code(error)==="UNKNOWN"){noteOnce(replica,`skipped:${path}`,{event:"skipped",path,reason:"symlink-unsupported"});return "dropped";}''','''   if(code(error)==="EPERM"||code(error)==="EACCES"||code(error)==="ENOSYS"||code(error)==="UNKNOWN"){capable.set(replica,false);noteOnce(replica,`skipped:${path}`,{event:"skipped",path,reason:"symlink-unsupported"});setFile(replica,path,{...plain(entry),mtimeMs:0});return "applied";}''')
s=sub(s,''' const found=new ReplicaFoundV1(book),seen=new Bits(),drifted=new Bits(),drift:Drift[]=[];
 const {refused}=await scanSharedV1(''',''' const found=new ReplicaFoundV1(book),seen=new Bits(),drifted=new Bits(),drift:Drift[]=[],links=settings.symlinks??await linksWork(paths.tmp);
 const {refused}=await scanSharedV1(''')
s=sub(s,'''  if(files.live(id)&&files.operationAt(id)==="delete"&&files.sameHlcAs(manifest,id))continue;
  drift.push({path,kind:"missing-local"''','''  if(files.live(id)&&files.operationAt(id)==="delete"&&files.sameHlcAs(manifest,id))continue;
  if(operation==="link"&&!links)continue;
  drift.push({path,kind:"missing-local"''')
s=sub(s,'''  if(seen.has(id)||drifted.has(id)||queue.hasId(id)||files.operationAt(id)==="delete"||indexRealmV1(book.name(id)))continue;''','''  if(seen.has(id)||drifted.has(id)||queue.hasId(id)||files.operationAt(id)==="delete"||indexRealmV1(book.name(id))||(files.operationAt(id)==="link"&&!links))continue;''')
open(p,"w",encoding="utf8").write(s)
p=root+"/🔁️replica/🧪️tests/🟦️.ts"
s=open(p,encoding="utf8").read()
s=sub(s,"import {tendBlobsV1,checkpointReplicaV1,","import {repairV1,tendBlobsV1,checkpointReplicaV1,")
open(p,"w",encoding="utf8").write(s)
print("ok")
