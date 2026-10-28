/** 🕰️ [td-leases] the static import closure (module count, bytes, list) of one entry at a git revision, read with `git show` (no checkout touched). usage: bun import-tree-rev.ts <repo-root> <rev> <repo-relative entry> */
import {spawnSync} from "node:child_process";
import {dirname,join,normalize} from "node:path";
const [root,rev,entry]=process.argv.slice(2) as [string,string,string],t=new Bun.Transpiler({loader:"ts"}),read=(p:string)=>{const r=spawnSync("git",["show",rev+":"+p],{cwd:root,encoding:"buffer",maxBuffer:1<<26});return r.status===0?r.stdout:undefined;};
const seen=new Map<string,number>(),q=[normalize(entry)];
while(q.length){const f=q.shift()!;if(seen.has(f))continue;const data=read(f);if(!data)continue;seen.set(f,data.length);let imports:{path:string;kind:string}[]=[];try{imports=t.scan(data.toString("utf8")).imports;}catch{}for(const i of imports){if(i.kind!=="import-statement"||!i.path.startsWith("."))continue;const p=normalize(join(dirname(f),i.path));if(/\.(ts|mjs)$/.test(p)&&!seen.has(p))q.push(p);}}
let bytes=0;for(const [f,n] of seen){bytes+=n;console.log(String(n).padStart(7)+" "+f.replace(/^.*?🧰️framework\//,""));}
console.log(`total ${seen.size} modules ${bytes} bytes at ${rev}`);
