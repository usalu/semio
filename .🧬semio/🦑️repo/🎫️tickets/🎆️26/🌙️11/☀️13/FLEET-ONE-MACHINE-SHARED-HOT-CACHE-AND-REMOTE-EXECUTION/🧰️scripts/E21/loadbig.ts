import {readFileSync} from "node:fs";
const url=process.argv[2]!,seconds=Number(process.argv[3]),width=Number(process.argv[4]),hs=JSON.parse(readFileSync(process.argv[5]!,"utf8")) as string[];
const stop=Date.now()+seconds*1000;let bytes=0,n=0;const lat:number[]=[];
await Promise.all(Array.from({length:width},async()=>{while(Date.now()<stop){const t=performance.now();const r=await fetch(`${url}/main/cas/${hs[Math.floor(Math.random()*hs.length)]}`,{headers:{authorization:"Bearer t"}});bytes+=(await r.arrayBuffer()).byteLength;n++;lat.push(performance.now()-t);}}));
lat.sort((a,b)=>a-b);console.log("bigclient",JSON.stringify({n,MBps:bytes/1048576/seconds,p50:lat[Math.floor(lat.length/2)],max:lat.at(-1)}));
