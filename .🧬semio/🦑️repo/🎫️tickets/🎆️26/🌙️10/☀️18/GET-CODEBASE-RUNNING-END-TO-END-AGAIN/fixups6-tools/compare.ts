import {readFileSync} from "node:fs";
const parse=(p:string)=>{
  const x=readFileSync(p,"utf8");const out=new Map<string,string>();
  for(const m of x.matchAll(/<testcase name="([^"]*)"[^>]*?(?:\/>|>([\s\S]*?)<\/testcase>)/gu)){
    const body=m[2]??"";const st=/<failure|<error/u.test(body)?"fail":/<skipped/u.test(body)?"skip":"pass";
    const key=m[1]!;out.set(key,(out.get(key)?out.get(key)+"|":"")+st);
  }
  return out;
};
const a=parse(process.argv[2]!),b=parse(process.argv[3]!);
let diff=0;
for(const k of new Set([...a.keys(),...b.keys()])){
  if(a.get(k)!==b.get(k)){diff++;console.log("DIFF",k.slice(0,120),"| orig:",a.get(k),"| split:",b.get(k));}
}
const count=(m:Map<string,string>)=>{const c:Record<string,number>={};for(const v of m.values())for(const s of v.split("|"))c[s]=(c[s]??0)+1;return c;};
console.log("orig",a.size,JSON.stringify(count(a)),"split",b.size,JSON.stringify(count(b)),"diffs",diff);
