import{randomBytes}from"node:crypto";
import{lstat,mkdir,open,readdir,readFile,rename,rm}from"node:fs/promises";
import{dirname,join}from"node:path";
import{P,aborted,fail,sha256}from"../🧱️guards/🟨️.mjs";

/** 🗂️ The filesystem port on real files: a missing file reads undefined, writes are atomic by rename, a content addressed write that finds its file does nothing. */
export function createNodeFilePortV1(){
 return Object.freeze({
  async read(path){try{return await readFile(path);}catch(error){if(error?.code==="ENOENT"||error?.code==="ENOTDIR")return undefined;throw error;}},
  async write(path,bytes,{ifAbsent=false}={}){
   if(ifAbsent){try{await lstat(path);return false;}catch(error){if(error?.code!=="ENOENT")throw error;}}
   await mkdir(dirname(path),{recursive:true});
   const temporary=path+"."+randomBytes(8).toString("hex")+".tmp";
   try{const file=await open(temporary,"wx");try{await file.writeFile(bytes);}finally{await file.close();}await rename(temporary,path);}finally{await rm(temporary,{force:true});}
   return true;
  },
  async names(directory){try{return await readdir(directory);}catch(error){if(error?.code==="ENOENT")return[];throw error;}},
  async list(directory){
   let names;try{names=await readdir(directory);}catch(error){if(error?.code==="ENOENT")return[];throw error;}
   const rows=await Promise.all(names.map(async name=>{try{const status=await lstat(join(directory,name));return status.isFile()?{name,modifiedMs:status.mtimeMs}:undefined;}catch(error){if(error?.code==="ENOENT")return undefined;throw error;}}));
   return rows.filter(Boolean);
  },
  async remove(path){await rm(path,{force:true});},
 });
}

export const defaultSleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/** 🔒️ A reference exclusive lease over lock files for tests and non repository runtimes: acquisition is `open(wx)`, a dead owner's lock is replaced, waiting polls with backoff and honours the abort signal. */
export function createFileLeasePortV1(directory,{staleMilliseconds=P.leaseStaleMilliseconds,now=()=>Date.now(),sleep=defaultSleep,onHold}={}){
 const alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error?.code==="EPERM";}};
 return Object.freeze({
  async exclusive(resource,body,{signal,timeoutMs}={}){
   const path=join(directory,sha256(resource)+".lock"),token=randomBytes(12).toString("hex"),started=now();let delay=2;
   await mkdir(directory,{recursive:true});
   for(;;){
    aborted(signal);
    try{const file=await open(path,"wx");try{await file.writeFile(JSON.stringify({pid:process.pid,token,since:now()}));}finally{await file.close();}break;}
    catch(error){
     if(error?.code!=="EEXIST")throw error;
     try{const held=JSON.parse(await readFile(path,"utf8"));if(!alive(held.pid)||now()-held.since>staleMilliseconds){const again=JSON.parse(await readFile(path,"utf8"));if(again.token===held.token)await rm(path,{force:true});continue;}}catch{}
     if(timeoutMs!==undefined&&now()-started>timeoutMs)throw fail("Resource lease timed out: "+resource,"LEASE_TIMEOUT");
     await sleep(delay,signal);delay=Math.min(delay*2,P.pollMaximumMilliseconds);
    }
   }
   const acquired=now();
   try{return await body();}finally{try{const held=JSON.parse(await readFile(path,"utf8"));if(held.token===token)await rm(path,{force:true});}catch{}onHold?.({resource,heldMs:now()-acquired,waitedMs:acquired-started});}
  },
 });
}
