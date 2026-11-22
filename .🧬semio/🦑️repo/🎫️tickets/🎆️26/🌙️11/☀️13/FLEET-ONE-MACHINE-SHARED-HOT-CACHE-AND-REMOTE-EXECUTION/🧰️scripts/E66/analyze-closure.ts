import {readFileSync} from "node:fs";
import {unpackBundleV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";
const dir=process.argv[2]!,name=process.argv[3]!;
const load=(side:string)=>unpackBundleV1(JSON.parse(readFileSync(`${dir}/${side}-closure_${name}_br.json`,"utf8")));
const a:any=load("kept"),b:any=load("refused");
const show=(x:any)=>({keys:Object.keys(x),rows:x.rows.length,layers:[...x.layers].length,sidecarKeys:Object.keys(x.sidecar??{}),facts:x.sidecar?.facts?.length,members:x.sidecar?.members?.map((m:any)=>m?m.length:null)});
console.log(JSON.stringify(show(a)).slice(0,600));console.log(JSON.stringify(show(b)).slice(0,600));
const j=(o:any)=>JSON.stringify(o);
console.log("value equal",j(a.value)===j(b.value),"rows equal",j(a.rows)===j(b.rows),"facts equal",j(a.sidecar?.facts)===j(b.sidecar?.facts));
const la=[...a.layers].map(j),lb=[...b.layers].map(j);console.log("layers equal",j(la)===j(lb),la.length,lb.length);
const wa=new Set(a.sidecar.members.flat().filter(Boolean)),wb=new Set(b.sidecar.members.flat().filter(Boolean));
const qa=new Set(a.sidecar.facts.filter(Boolean).map((f:any)=>f.row.path));
console.log("members distinct",wa.size,wb.size,"inFactsA",[...wa].filter(x=>qa.has(x as string)).length,"facts distinct",qa.size);
const ma=a.sidecar.members.map((m:any,i:number)=>m?m.filter((x:string)=>qa.has(x)).sort():null),mb=b.sidecar.members.map((m:any,i:number)=>m?m.filter((x:string)=>qa.has(x)).sort():null);
console.log("members∩facts equal",j(ma)===j(mb));
const A=[...a.layers],B=[...b.layers];
A.forEach((l:any,i:number)=>{const x=l.map(j),y=B[i].map(j),sx=new Set(x),sy=new Set(y);console.log("layer",i,x.length,y.length,"onlyA",x.filter((r:string)=>!sy.has(r)).length,"onlyB",y.filter((r:string)=>!sx.has(r)).length,"sameSetDiffOrder",x.length===y.length&&[...sx].every(r=>sy.has(r))&&j(x)!==j(y));});
const l1=A.findIndex((l:any,i:number)=>j(l)!==j(B[i]));if(l1>=0){const x=A[l1].map(j),y=new Set(B[l1].map(j));console.log("sampleA",x.filter((r:string)=>!y.has(r)).slice(0,3).map((s:string)=>s.slice(0,300)));const sx=new Set(x);console.log("sampleB",B[l1].map(j).filter((r:string)=>!sx.has(r)).slice(0,3).map((s:string)=>s.slice(0,300)));}
