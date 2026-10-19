import {readFileSync,existsSync,writeFileSync} from "node:fs";
import {resolve,dirname,relative} from "node:path";
const root=resolve(process.argv[2]!),entry=resolve(process.argv[3]!),out=process.argv[4]!;
const pattern=/((?:import|export)\b[^;'"]*?from\s*|import\s*|import\(\s*|require\(\s*)(["'])([^"']+)\2/gm;
const seen=new Set<string>(),queue=[entry],bare=new Set<string>();
const cands=(p:string)=>[p,p+".ts",p+".tsx",p+"/index.ts",p.replace(/\.js$/,".ts")];
while(queue.length){const f=queue.pop()!;if(seen.has(f))continue;seen.add(f);let t:string;try{t=readFileSync(f,"utf8");}catch{continue;}
 for(const m of t.matchAll(pattern)){const s=m[3]!;if(!s.startsWith(".")){if(s.startsWith("@semio-tech/"))bare.add(relative(root,f)+" -> "+s);continue;}
  const n=resolve(dirname(f),s.split("?")[0]!);const hit=cands(n).find(c=>/\.(ts|tsx|mts|js|mjs|cjs|json)$/.test(c)&&existsSync(c));if(hit&&!/\.json$/.test(hit))queue.push(hit);}}
writeFileSync(out,[...seen].map(f=>relative(root,f)).sort().join("\n")+"\n");
console.log("files",seen.size,"bare",bare.size,[...bare].slice(0,10));
