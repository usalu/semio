import {createHash,randomBytes} from "node:crypto";
const url=process.argv[2]!,seconds=Number(process.argv[3]),width=Number(process.argv[4]??32);
const auth={authorization:"Bearer t"};const lat:number[]=[];let errors=0;const stop=Date.now()+seconds*1000;
const hex=()=>randomBytes(32).toString("hex");
async function worker(){
 while(Date.now()<stop){
  const t=performance.now();
  try{
   const r=Math.random();let res:Response;
   if(r<0.4){res=await fetch(`${url}/main/ac/${hex()}`,{headers:auth});}
   else if(r<0.6){const b=randomBytes(2000+Math.floor(Math.random()*200000)),h=createHash("sha256").update(b).digest("hex");res=await fetch(`${url}/main/cas/${h}`,{method:"PUT",headers:auth,body:b});}
   else if(r<0.8){const k=hex();res=await fetch(`${url}/main/ac/${k}`,{method:"PUT",headers:{...auth,"x-semio-slot":hex()},body:randomBytes(300)});}
   else {res=await fetch(`${url}/healthz`);}
   await res.arrayBuffer();
  }catch{errors++;}
  lat.push(performance.now()-t);
 }
}
await Promise.all(Array.from({length:width},worker));
lat.sort((a,b)=>a-b);console.log("client",JSON.stringify({n:lat.length,errors,p50:lat[Math.floor(lat.length/2)],p99:lat[Math.floor(lat.length*0.99)],max:lat[lat.length-1]}));
