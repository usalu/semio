/** 🔬️ Summarizes one persisted project inference record: header bytes, observation kinds, top paths by row count and size. */
import {readFileSync} from "node:fs";
const path=process.argv[2]!,text=readFileSync(path,"utf8"),split=text.indexOf("\n"),header=JSON.parse(text.slice(0,split));
const rows=header.observations as {kind:string;path:string;fact:any}[],authority=header.authorityObservations as unknown[];
const kinds=new Map<string,{count:number;bytes:number}>();for(const row of rows){const k=kinds.get(row.kind)??{count:0,bytes:0};k.count++;k.bytes+=JSON.stringify(row).length;kinds.set(row.kind,k);}
const nested=rows.filter(row=>row.kind==="analysis").map(row=>({path:row.path,nested:row.fact.nestedObservations.length,bytes:JSON.stringify(row).length})).sort((a,b)=>b.bytes-a.bytes).slice(0,5);
const files=rows.filter(row=>row.kind==="file").map(row=>row.path);
const buckets=new Map<string,number>();for(const file of files){const key=file.split("/").slice(5,12).join("/");buckets.set(key,(buckets.get(key)??0)+1);}
console.log(JSON.stringify({bytes:text.length,headerBytes:split,resultBytes:text.length-split,rows:rows.length,authority:authority.length,kinds:Object.fromEntries(kinds),largestAnalysis:nested,fileBuckets:[...buckets].sort((a,b)=>b[1]-a[1]).slice(0,15)},null,1));
