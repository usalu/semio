/** 🔑 Session primitives of the resident service, owned by the authority core: macaroon style capability tokens, the worktree identity and endpoints, and the private key file. The resident service and the service invocation share this one implementation. */
import {createHash,createHmac,randomBytes,timingSafeEqual} from "node:crypto";
import {appendFileSync,existsSync,lstatSync,readFileSync,realpathSync,statSync} from "node:fs";
import {homedir,tmpdir,userInfo} from "node:os";
import {join} from "node:path";

const b64=bytes=>Buffer.from(bytes).toString("base64url");
const unb64=text=>Buffer.from(text,"base64url");
const hmac=(key,data)=>createHmac("sha256",key).update(data).digest();

/** 📐 Canonical JSON: sorted keys, no whitespace, the byte string every implementation must MAC. */
export function canonicalJsonV1(value){
 if(value===null||typeof value!=="object")return JSON.stringify(value);
 if(Array.isArray(value))return"["+value.map(canonicalJsonV1).join(",")+"]";
 return"{"+Object.keys(value).sort().map(key=>JSON.stringify(key)+":"+canonicalJsonV1(value[key])).join(",")+"}";
}

/** 🗝 A fresh 32 byte root key. */
export function createRootKeyV1(){return randomBytes(32);}

/** 🧮 The digest by which an invocation names the key it trusts. */
export function keyDigestV1(key){return createHash("sha256").update(key).digest("hex");}

/** 🏷 Mints a token for a caveat list. */
export function mintCapabilityV1(key,caveats,id=randomBytes(8).toString("hex")){
 let mac=hmac(key,id);
 for(const caveat of caveats)mac=hmac(mac,canonicalJsonV1(caveat));
 return"v1."+b64(Buffer.from(JSON.stringify({id,caveats})))+"."+b64(mac);
}

function split(token){
 const parts=typeof token==="string"?token.split("."):[];
 if(parts.length!==3||parts[0]!=="v1")return undefined;
 try{const payload=JSON.parse(unb64(parts[1]).toString("utf8")),mac=unb64(parts[2]);if(typeof payload?.id!=="string"||!Array.isArray(payload.caveats)||mac.length!==32)return undefined;return{payload,mac};}catch{return undefined;}
}

/** ✂ Narrows a token with one more caveat; needs no key. */
export function attenuateCapabilityV1(token,caveat){
 const parts=split(token);if(!parts)throw new Error("Not a capability token");
 const mac=hmac(parts.mac,canonicalJsonV1(caveat)),caveats=[...parts.payload.caveats,caveat];
 return"v1."+b64(Buffer.from(JSON.stringify({id:parts.payload.id,caveats})))+"."+b64(mac);
}

/** ✅ Verifies the MAC chain; returns the caveats of a genuine token or undefined. */
export function verifyCapabilityV1(key,token){
 const parts=split(token);if(!parts)return undefined;
 let mac=hmac(key,parts.payload.id);
 for(const caveat of parts.payload.caveats)mac=hmac(mac,canonicalJsonV1(caveat));
 return mac.length===parts.mac.length&&timingSafeEqual(mac,parts.mac)?parts.payload.caveats:undefined;
}

/** 🪪 The identifier of a token (the session it names), or undefined for anything that is not a token. */
export function capabilityIdV1(token){return split(token)?.payload.id;}

/** 🧭 Evaluates every caveat against one request; returns the first refusal reason or undefined when all hold. */
export function authorizeCapabilityV1(caveats,request){
 for(const caveat of caveats){
  if(caveat.k==="scope"){if(!caveat.v.includes(request.commandClass))return{code:"scope",class:request.commandClass};}
  else if(caveat.k==="notAfter"){if(request.now>caveat.v)return{code:"capability",reason:"expired"};}
  else if(caveat.k==="cwd"){if(!(request.cwd===caveat.v||request.cwd.startsWith(caveat.v.endsWith("/")?caveat.v:caveat.v+"/")))return{code:"scope",class:request.commandClass,reason:"cwd"};}
  else if(caveat.k==="projects"){if(request.projects.length>0&&!request.projects.every(project=>caveat.v.some(prefix=>prefix.endsWith("*")?project.startsWith(prefix.slice(0,-1)):project===prefix||project.startsWith(prefix+":"))))return{code:"scope",class:request.commandClass,reason:"projects"};}
  else if(caveat.k==="worktree"){if(request.worktree!==caveat.v)return{code:"scope",class:request.commandClass,reason:"worktree"};}
  else if(caveat.k==="agent"){if(request.agent!==caveat.v)return{code:"scope",class:request.commandClass,reason:"agent"};}
  else return{code:"capability",reason:"unknown caveat"};
 }
 return undefined;
}

/** 🏠 The realpath of a worktree root: the launcher and the service must hash the same spelling. */
export function canonicalRootV1(root){try{return realpathSync(root);}catch{return root;}}

/** 🔢 Twelve hex digits identifying a worktree root. */
export function worktreeIdV1(root){return createHash("sha256").update("semio-repo-service-v1\0"+root).digest("hex").slice(0,12);}

/** 📍 Runtime directory, unix socket or pipe name, key file and descriptor file of one worktree; paths stay under 100 bytes so they fit sockaddr_un. */
export function endpointsV1(root,environment=process.env,platform=process.platform,user=undefined){
 const id=worktreeIdV1(root),owner=user??(()=>{try{return String(userInfo().uid);}catch{return"0";}})();
 if(platform==="win32"){
  const base=join(environment.LOCALAPPDATA||tmpdir(),"semio","repo",id);
  return{id,directory:base,endpoint:"\\\\.\\pipe\\semio-repo-"+owner+"-"+id,key:join(base,"key"),descriptor:join(base,"service.json"),state:join(base,"state.json"),lock:join(base,"service.lock"),audit:join(base,"audit.jsonl"),revoked:join(base,"revoked.jsonl")};
 }
 let directory=join(environment.XDG_RUNTIME_DIR||tmpdir(),"semio-repo-"+owner,id);
 if(Buffer.byteLength(join(directory,"s"))>96)directory=join("/tmp","semio-repo-"+owner,id);
 return{id,directory,endpoint:join(directory,"s"),key:join(directory,"key"),descriptor:join(directory,"service.json"),state:join(directory,"state.json"),lock:join(directory,"service.lock"),audit:join(directory,"audit.jsonl"),revoked:join(directory,"revoked.jsonl")};
}

/** 📌 The private state directory of one worktree and user that no per-process variable moves: below `XDG_RUNTIME_DIR` when that names one, else below the user cache directory of the account database (never `TMPDIR`, which a nested build action sets to its own scratch), so that every process of the worktree reaches the same change journal. */
export function stateDirectoryV1(root,environment=process.env,platform=process.platform,user=undefined,home=undefined){
 const id=worktreeIdV1(root),owner=user??(()=>{try{return String(userInfo().uid);}catch{return"0";}})(),account=home??(()=>{try{return userInfo().homedir;}catch{return homedir();}})();
 if(platform==="win32")return join(environment.LOCALAPPDATA||join(account,"AppData","Local"),"semio","repo",id);
 if(environment.XDG_RUNTIME_DIR)return join(environment.XDG_RUNTIME_DIR,"semio-repo-"+owner,id);
 return join(account,platform==="darwin"?"Library/Caches":".cache","semio","repo",id);
}

/** 🔐 Reads the service key: a regular file owned by the caller that no other user can read, inside a private directory owned by the caller. */
export function readServiceKeyV1(keyPath,platform=process.platform){
 if(platform!=="win32"){
  const file=lstatSync(keyPath),directory=lstatSync(keyPath.slice(0,Math.max(keyPath.lastIndexOf("/"),1)));
  if(!file.isFile()||file.uid!==process.getuid()||(file.mode&0o077)!==0||!directory.isDirectory()||directory.uid!==process.getuid()||(directory.mode&0o077)!==0)throw new Error("The service key must be a private file of the current user in a private directory");
 }
 const key=readFileSync(keyPath);
 if(key.length!==32)throw new Error("The service key must be 32 bytes");
 return key;
}

/** 🚫 Append-only registry of revoked sessions and agents shared by the service and every executor through one private file; a stat per check keeps it cheap and a restart keeps it. */
export function createRevocationRegistryV1(path,now=()=>Date.now()){
 let sessions=new Set(),agents=new Set(),signature="";
 const reload=()=>{
  if(!existsSync(path)){sessions=new Set();agents=new Set();signature="";return;}
  const status=statSync(path),next=status.size+":"+status.mtimeMs;
  if(next===signature)return;
  const nextSessions=new Set(),nextAgents=new Set();
  for(const line of readFileSync(path,"utf8").split("\n")){if(!line)continue;try{const row=JSON.parse(line);if(typeof row.session==="string")nextSessions.add(row.session);if(typeof row.agent==="string")nextAgents.add(row.agent);}catch{}}
  sessions=nextSessions;agents=nextAgents;signature=next;
 };
 const append=row=>{appendFileSync(path,JSON.stringify({...row,at:now()})+"\n",{mode:0o600});signature="";reload();};
 return{
  has:session=>{reload();return sessions.has(session);},
  hasAgent:agent=>{reload();return agents.has(agent);},
  revoke:session=>append({session}),
  revokeAgent:agent=>append({agent}),
  list:()=>{reload();return{sessions:[...sessions],agents:[...agents]};},
 };
}

/** 📒 Appends one audit row as a JSON line to the private audit file. */
export function appendAuditV1(path,row){appendFileSync(path,JSON.stringify(row)+"\n",{mode:0o600});}
