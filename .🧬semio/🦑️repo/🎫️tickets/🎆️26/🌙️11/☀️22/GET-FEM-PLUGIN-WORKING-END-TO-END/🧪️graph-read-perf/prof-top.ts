import {readFileSync} from "node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8"));
const self=new Map<number,number>();
const dt=p.timeDeltas as number[],samples=p.samples as number[];
for(let i=0;i<samples.length;i++)self.set(samples[i],(self.get(samples[i])??0)+(dt[i]??0));
const rows=new Map<string,number>();
for(const n of p.nodes){const cf=n.callFrame,k=cf.functionName+" "+cf.url.split(/[\\/]/).slice(-2).join("/")+":"+cf.lineNumber;rows.set(k,(rows.get(k)??0)+(self.get(n.id)??0));}
const total=[...rows.values()].reduce((a,b)=>a+b,0);
console.log("total ms",Math.round(total/1000));
for(const [k,v] of [...rows].sort((a,b)=>b[1]-a[1]).slice(0,Number(process.argv[3]??15)))console.log(String(Math.round(v/1000)).padStart(8),"ms",k.slice(0,160));
