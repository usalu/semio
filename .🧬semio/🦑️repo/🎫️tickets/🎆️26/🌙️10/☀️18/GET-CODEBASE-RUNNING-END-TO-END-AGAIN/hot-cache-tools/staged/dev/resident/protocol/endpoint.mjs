/** 📍 Where a worktree's resident service lives: one deterministic endpoint and runtime directory per worktree and user. */
import {createHash} from "node:crypto";
import {tmpdir,userInfo} from "node:os";
import {join} from "node:path";

/** 🔢 Twelve hex digits identifying a worktree root. */
export function worktreeIdV1(root){return createHash("sha256").update("semio-repo-service-v1\0"+root).digest("hex").slice(0,12);}

/** 📍 Runtime directory, unix socket or pipe name, key file and descriptor file of one worktree; paths stay under 100 bytes so they fit sockaddr_un. */
export function endpointsV1(root,environment=process.env,platform=process.platform,user=undefined){
 const id=worktreeIdV1(root),owner=user??(()=>{try{return String(userInfo().uid);}catch{return"0";}})();
 if(platform==="win32"){
  const base=join(environment.LOCALAPPDATA??tmpdir(),"semio","repo",id);
  return{id,directory:base,endpoint:"\\\\.\\pipe\\semio-repo-"+owner+"-"+id,key:join(base,"key"),descriptor:join(base,"service.json"),state:join(base,"state.json"),lock:join(base,"service.lock")};
 }
 let directory=join(environment.XDG_RUNTIME_DIR??tmpdir(),"semio-repo-"+owner,id);
 if(Buffer.byteLength(join(directory,"s"))>96)directory=join("/tmp","semio-repo-"+owner,id);
 return{id,directory,endpoint:join(directory,"s"),key:join(directory,"key"),descriptor:join(directory,"service.json"),state:join(directory,"state.json"),lock:join(directory,"service.lock")};
}
