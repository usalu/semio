import {readFileSync} from "node:fs";
import {createHash,randomBytes} from "node:crypto";
import {fleetSignedFetchV1,fleetContentHashV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {encodeProtobufV1,REAPI_V1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
const url=process.argv[2]!,seconds=Number(process.argv[3]),width=Number(process.argv[4]),roots=JSON.parse(readFileSync(process.argv[5]!,"utf8")) as string[];
const signers=roots.map(root=>fleetSignedFetchV1({root,audience:"cache"}));
const sha=(b:Uint8Array|string)=>createHash("sha256").update(b).digest("hex");
const stop=Date.now()+seconds*1000,lat:number[]=[];let errors=0,codes:Record<string,number>={};
const known:string[]=[];
async function worker(i:number){const g=signers[i%signers.length]!,f=((u:any,o:any={})=>g(u,{...o,keepalive:false,headers:{...(o.headers??{}),connection:"close"}})) as typeof fetch;
 while(Date.now()<stop){const t=performance.now();
  try{const r=Math.random();let res:Response;
   if(r<0.5&&known.length>0){res=await f(`${url}/main/cas/${known[Math.floor(Math.random()*known.length)]}`);}
   else if(r<0.62){const b=randomBytes(300+Math.floor(Math.random()*3000)),h=sha(b);res=await f(`${url}/main/cas/${h}`,{method:"PUT",headers:{"x-semio-content":"-"},body:b});if(res.status===200)known.push(h);if(known.length>5000)known.shift();}
   else if(r<0.72&&known.length>1){const out=[known[0]!,known[known.length-1]!],body=encodeProtobufV1(REAPI_V1.actionResult,{output_files:out.map((hash,index)=>({path:"o"+index,digest:{hash,size_bytes:3}})),exit_code:0}),k=sha(randomBytes(8));res=await f(`${url}/main/ac/${k}`,{method:"PUT",headers:{"x-semio-slot":sha(k)},body});}
   else if(r<0.9){const k=sha(randomBytes(8));res=await f(`${url}/main/claim/ac/${k}`,{method:"POST",body:JSON.stringify({owner:"w"+i,ttlMilliseconds:30000})});}
   else res=await f(`${url}/main/ac/${sha(randomBytes(8))}`);
   await res.arrayBuffer();codes[res.status]=(codes[res.status]??0)+1;
  }catch{errors++;}
  lat.push(performance.now()-t);}}
await Promise.all(Array.from({length:width},(_,i)=>worker(i)));
lat.sort((a,b)=>a-b);console.log("client",JSON.stringify({n:lat.length,rps:Math.round(lat.length/seconds),errors,codes,p50:+lat[Math.floor(lat.length/2)]!.toFixed(1),p99:+lat[Math.floor(lat.length*0.99)]!.toFixed(1),max:+lat.at(-1)!.toFixed(0)}));
