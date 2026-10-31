import fs from "node:fs";
import crypto from "node:crypto";
const rows=await Promise.all(["https://architektur-und-technologie.de/","https://semio.iek.uni-hannover.de/instance"].map(async url=>{const at=new Date().toISOString(),start=performance.now();try{const response=await fetch(url,{signal:AbortSignal.timeout(10000),redirect:"follow"});const body=await response.text();let identity;try{identity=JSON.parse(body).id;}catch{}return{url,at,status:response.status,finalUrl:response.url,bytes:Buffer.byteLength(body),sha256:crypto.createHash("sha256").update(body).digest("hex"),title:body.match(/<title>([^<]*)<\/title>/u)?.[1],identity,milliseconds:performance.now()-start};}catch(error){return{url,at,errorName:error.name,errorMessage:error.message,milliseconds:performance.now()-start};}}));
fs.writeFileSync(new URL("./🗑️generated/public-http-current.json",import.meta.url),JSON.stringify({at:new Date().toISOString(),requests:rows},null,2)+"\n");
console.log(JSON.stringify(rows));
