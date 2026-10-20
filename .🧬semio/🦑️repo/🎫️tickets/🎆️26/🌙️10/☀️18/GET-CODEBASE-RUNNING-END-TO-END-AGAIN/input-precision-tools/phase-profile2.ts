import {programInputsV1} from "/private/tmp/ip/run/p/🟦️.ts";
import {realHost} from "./real-host.ts";
const {host}=await realHost("");
let listMs=0,existsMs=0,calls=0;
const wrapped={...host,listDirectory:(p:string)=>{const t=performance.now();const r=host.listDirectory(p);listMs+=performance.now()-t;return r;},exists:(p:string)=>{const t=performance.now();const r=host.exists(p);existsMs+=performance.now()-t;calls++;return r;},
 program:{compiler:"x",imports:async()=>({resolved:[],facts:[]}),fact:()=>"d"}} as any;
const t0=performance.now(),r=await programInputsV1("✏️s/tsconfig.json",wrapped);
console.log(JSON.stringify({total:Math.round(performance.now()-t0),listMs:Math.round(listMs),existsMs:Math.round(existsMs),existsCalls:calls,roots:r.roots.length,files:r.files.length}));
