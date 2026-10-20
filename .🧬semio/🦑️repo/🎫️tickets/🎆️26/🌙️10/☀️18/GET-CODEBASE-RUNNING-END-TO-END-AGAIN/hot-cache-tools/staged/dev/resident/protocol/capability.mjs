/** 🔑 Macaroon-style capability tokens: HMAC chain over caveats, attenuable by any holder, forgeable by none. */
import {createHmac,randomBytes,timingSafeEqual} from "node:crypto";

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

/** 🧭 Evaluates every caveat against one request; returns the first refusal reason or undefined when all hold. */
export function authorizeCapabilityV1(caveats,request){
 for(const caveat of caveats){
  if(caveat.k==="scope"){if(!caveat.v.includes(request.commandClass))return{code:"scope",class:request.commandClass};}
  else if(caveat.k==="notAfter"){if(request.now>caveat.v)return{code:"capability",reason:"expired"};}
  else if(caveat.k==="cwd"){if(!(request.cwd===caveat.v||request.cwd.startsWith(caveat.v.endsWith("/")?caveat.v:caveat.v+"/")))return{code:"scope",class:request.commandClass,reason:"cwd"};}
  else if(caveat.k==="projects"){if(request.projects.length>0&&!request.projects.every(project=>caveat.v.some(prefix=>prefix.endsWith("*")?project.startsWith(prefix.slice(0,-1)):project===prefix||project.startsWith(prefix+":"))))return{code:"scope",class:request.commandClass,reason:"projects"};}
  else return{code:"capability",reason:"unknown caveat"};
 }
 return undefined;
}
