import {readdirSync,mkdirSync,writeFileSync} from "node:fs";import {join} from "node:path";
const root=process.argv[2]!,per=Number(process.argv[3]);const o=join(root,"v1","o"),l=join(root,"v1","l");let lines=0;
for(const shard of readdirSync(o)){const names=readdirSync(join(o,shard));mkdirSync(join(l,shard),{recursive:true});
 const out:string[]=[];for(const n of names){const d=shard+n;for(let i=0;i<per;i++){out.push(JSON.stringify({d,t:Date.now()-i*1000-Math.floor(Math.random()*1e6)}));lines++;}}
 writeFileSync(join(l,shard,"old-writer-"+shard+".jsonl"),out.join("\n")+"\n");}
console.log("lines",lines);
