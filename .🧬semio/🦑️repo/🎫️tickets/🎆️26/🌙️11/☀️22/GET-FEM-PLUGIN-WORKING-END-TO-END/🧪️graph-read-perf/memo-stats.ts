import {readFileSync,lstatSync} from "node:fs";
const path=process.argv[2];
let t=performance.now();
const text=readFileSync(path,"utf8");
console.log("read ms",Math.round(performance.now()-t),"bytes",text.length);
t=performance.now();
const memo=JSON.parse(text);
console.log("parse ms",Math.round(performance.now()-t),"observations",memo.observations.length,"owners",memo.owners.length);
const kinds=new Map<string,number>();
for(const row of memo.observations)kinds.set(row.kind,(kinds.get(row.kind)??0)+1);
console.log([...kinds].sort((a,b)=>b[1]-a[1]));
const per=memo.owners.map((o:any)=>o.rows.length).sort((a:number,b:number)=>b-a);
console.log("owner row counts top",per.slice(0,10),"sum",per.reduce((a:number,b:number)=>a+b,0));
const sample=memo.observations.filter((_:any,i:number)=>i%20000===0).map((r:any)=>JSON.stringify(r).slice(0,200));
console.log(sample);
const paths=memo.observations.filter((r:any)=>r.kind==="file").slice(0,20000);
t=performance.now();let n=0;for(const r of paths){try{lstatSync(r.path);n++;}catch{}}
console.log("20000 lstatSync ms",Math.round(performance.now()-t),n);
