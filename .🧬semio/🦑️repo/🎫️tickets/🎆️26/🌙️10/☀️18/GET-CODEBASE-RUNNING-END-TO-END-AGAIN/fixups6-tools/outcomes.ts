import {readFileSync} from "node:fs";
const log=readFileSync(process.argv[2]!,"utf8");
const map=JSON.parse(readFileSync(import.meta.dir+"/mapping.json","utf8")) as {tests:{title:string;group:string}[]};
const mine=new Map(map.tests.map(t=>[t.title,t.group]));
const res=new Map<string,string>();
for(const m of log.matchAll(/^\((pass|fail|skip|todo)\) (.*?)(?: \[[\d.]+m?s\])?$/gmu)){
  let name=m[2]!;
  res.set(name,m[1]!);
}
const out:Record<string,string>={};
let mineCount=0;const bad:string[]=[];
for(const [n,r] of res){
  const base=n.replace(/ \[[\d.]+m?s\]$/,"");
  out[base]=r;
}
const summary={pass:0,fail:0,skip:0,todo:0};
for(const r of Object.values(out))(summary as any)[r]++;
const mineRes:Record<string,string>={};
for(const [t] of mine){const k=Object.keys(out).filter(n=>n===t||n.startsWith(t.slice(0,40)));mineRes[t]=k.map(x=>out[x]).join(",")||"missing";}
if(process.argv[3]==="json"){console.log(JSON.stringify({all:out,mine:mineRes},null,1));}
else{
  console.log("log totals",JSON.stringify(summary));
  const c:Record<string,number>={};for(const v of Object.values(mineRes))c[v]=(c[v]??0)+1;
  console.log("mine",JSON.stringify(c));
  for(const [t,v] of Object.entries(mineRes))if(v!=="pass")console.log(v.padEnd(8),mine.get(t),t.slice(0,110));
}
