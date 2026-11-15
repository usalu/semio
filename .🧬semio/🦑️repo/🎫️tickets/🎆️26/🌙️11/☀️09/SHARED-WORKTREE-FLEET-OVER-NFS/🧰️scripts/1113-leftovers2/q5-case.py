#!/usr/bin/env python3
"""q5: replica: repair sees a spelling that differs only by case on a volume that ignores case, apply respells it (X17). usage: q5-case.py <module-root>"""
import sys
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:80],s.count(old))
    return s.replace(old,new)
p=root+"/🔁️replica/🟦️.ts"
s=open(p,encoding="utf8").read()
s=sub(s,'modeless?:boolean;symlinks?:boolean;lock?:boolean;','modeless?:boolean;symlinks?:boolean;caseInsensitive?:boolean;lock?:boolean;')
s=sub(s,'modeless:boolean;symlinks:boolean|undefined;now:','modeless:boolean;symlinks:boolean|undefined;caseInsensitive:boolean|undefined;now:')
s=sub(s,'symlinks:options.symlinks,now:','symlinks:options.symlinks,caseInsensitive:options.caseInsensitive,now:')
s=sub(s,'const capable=new WeakMap<object,boolean>();','''const capable=new WeakMap<object,boolean>(),caseless=new WeakMap<object,boolean>();

/** 🔡️ Tells whether the volume that holds `directory` ignores case: a probe file is looked up under another spelling. */
async function caseIgnored(directory:string):Promise<boolean>{
 const stem=token(),name=join(directory,`Case-${stem}`);
 try{await mkdir(directory,{recursive:true});await writeFile(name,"");return await lstat(join(directory,`case-${stem}`)).then(()=>true,()=>false);}
 catch{return false;}
 finally{await rm(name,{force:true}).catch(()=>{});}
}

/** 🔤️ Tells whether the exact spelling of an absolute path is what its directory lists; `listings` remembers one listing per directory for the caller. */
async function spelledIn(listings:Map<string,Set<string>>,target:string):Promise<boolean>{
 const directory=dirname(target);
 let names=listings.get(directory);
 if(names===undefined){names=new Set((await readdir(directory).catch(()=>[] as string[])).map(name=>name.normalize("NFC")));listings.set(directory,names);}
 return names.has(basename(target));
}

/** 🔠️ On a volume that ignores case, gives the file at `path` the spelling of the manifest by renaming the entry that the directory lists under another case (a file written over an existing one keeps the old spelling); nothing happens when the spelling is right, when another live path of this machine owns the other spelling or when the rename fails. */
async function respell(replica:ReplicaV1,path:string):Promise<void>{
 if(caseless.get(replica)!==true)return;
 const owner=replica.folds.find(path);
 if(owner!==undefined&&owner!==path)return;
 const target=absolute(replica.settings.worktree,path),directory=dirname(target),base=basename(path);
 const names=await readdir(directory).catch(()=>[] as string[]);
 if(names.some(name=>name.normalize("NFC")===base))return;
 const actual=names.find(name=>foldNameV1(name.normalize("NFC"))===foldNameV1(base));
 if(actual===undefined)return;
 try{await rename(join(directory,actual),target);}catch{return;}
 replica.say({event:"repaired",path,reason:"respelled",count:1});
}''')
s=sub(s,'  capable.set(replica,settings.symlinks??await linksWork(paths.tmp));\n','  capable.set(replica,settings.symlinks??await linksWork(paths.tmp));\n  caseless.set(replica,settings.caseInsensitive??await caseIgnored(paths.tmp));\n')
s=sub(s,''' let count=await takeOverRegistersV1(replica,signal);
 for(const id of manifest.ids()){''',''' let count=await takeOverRegistersV1(replica,signal);
 const listings=new Map<string,Set<string>>(),ignoresCase=caseless.get(replica)===true;
 for(const id of manifest.ids()){''')
s=sub(s,'''  try{await lstat(absolute(settings.worktree,path));}
  catch(error){if(!gone(error))throw error;forget(replica,path);enqueueId(replica,id);count++;}''','''  try{
   await lstat(absolute(settings.worktree,path));
   if(ignoresCase&&!await spelledIn(listings,absolute(settings.worktree,path))){enqueueId(replica,id);count++;}
  }
  catch(error){if(!gone(error))throw error;forget(replica,path);enqueueId(replica,id);count++;}''')
s=sub(s,'''  if(entry.operation==="put"&&!replica.settings.modeless&&((here!.mode&0o111)!==0)!==entry.executable)await chmod(target,entry.executable?0o755:0o644);
  const info=await lstat(target);''','''  if(entry.operation==="put"&&!replica.settings.modeless&&((here!.mode&0o111)!==0)!==entry.executable)await chmod(target,entry.executable?0o755:0o644);
  await respell(replica,path);
  const info=await lstat(target);''')
open(p,"w",encoding="utf8").write(s)
print("ok")
