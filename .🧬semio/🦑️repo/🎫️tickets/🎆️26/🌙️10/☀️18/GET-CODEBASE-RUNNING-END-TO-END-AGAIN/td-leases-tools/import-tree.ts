/** 🌳️ [td-leases] prints the static import tree (first discoverer = parent) of one TypeScript entry, with file sizes, to see which edge pulls a heavy module into a light command. usage: bun import-tree.ts <entry> */
import {readFileSync,statSync,existsSync} from "node:fs";
import {dirname,resolve} from "node:path";
const entry=resolve(process.argv[2]!),t=new Bun.Transpiler({loader:"ts"}),seen=new Map<string,string>(),q=[entry];seen.set(entry,"");
while(q.length){const f=q.shift()!;let imports:{path:string;kind:string}[]=[];try{imports=t.scan(readFileSync(f,"utf8")).imports;}catch{}for(const i of imports){if(i.kind!=="import-statement"||!i.path.startsWith("."))continue;const p=resolve(dirname(f),i.path);if(!existsSync(p)||!/\.(ts|mjs)$/.test(p)||seen.has(p))continue;seen.set(p,f);q.push(p);}}
const short=(p:string)=>p.replace(/^.*?🧰️framework\//,"").replace("🛍️products/🦑️repo/🔨️modules/📚️library/","L/").replace("🔨️modules/🏃️process/","P/");
for(const [f,parent] of seen)console.log(statSync(f).size.toString().padStart(7)+" "+short(f)+"  <- "+(parent?short(parent):"-"));
console.log("total",seen.size);
