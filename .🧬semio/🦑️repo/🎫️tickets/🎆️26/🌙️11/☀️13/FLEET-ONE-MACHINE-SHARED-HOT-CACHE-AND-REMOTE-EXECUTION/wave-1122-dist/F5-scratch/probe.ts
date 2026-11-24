import {readFileSync,statSync} from "node:fs";
import {remoteCacheFetchV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import {httpRemoteExecutionContentV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🗄️content/🟦️.ts";
const root=process.cwd(),hs=readFileSync(root+"/.cache/scratch-f5-hashes.txt","utf8").trim().split("\n");
const c=httpRemoteExecutionContentV1({endpoint:"http://192.168.178.70:6094",fetch:remoteCacheFetchV1(process.env,{root}),timeoutSeconds:30,retries:0});
const size=(h:string)=>statSync(process.env.HOME+"/.cache/semio/cas/v1/o/"+h.slice(0,2)+"/"+h.slice(2)).size;
const get=async(h:string)=>{const t=performance.now();try{const r=await c.read("",{hash:h,size:size(h)});return [performance.now()-t,r?.length??-1];}catch(e){return [performance.now()-t,String(e).slice(0,100)];}};
const seq=[];for(const h of hs.slice(0,10))seq.push(await get(h));console.log("seq",JSON.stringify(seq.map(x=>[Math.round(x[0] as number),x[1]])));
const t=performance.now();const par=await Promise.all(hs.slice(0,40).map(get));console.log("par32 total",Math.round(performance.now()-t),"max",Math.round(Math.max(...par.map(x=>x[0] as number))),JSON.stringify(par.slice(0,4)));
