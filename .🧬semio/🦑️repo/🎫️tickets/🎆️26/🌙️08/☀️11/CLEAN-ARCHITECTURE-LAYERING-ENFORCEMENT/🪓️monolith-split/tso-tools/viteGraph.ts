import { build } from "esbuild";
import { relative, resolve } from "node:path";
const repoRoot="/Users/ueli/Documents/semio";
const entry=resolve(repoRoot,process.argv[2]), target=process.argv[3];
const externalizeDeps={name:"externalize-deps",setup(b:any){b.onResolve({filter:/^[^.#].*/},({path,kind}:any)=>(kind==="entry-point"||path.startsWith("/")?null:{external:true}));}};
const r=await build({entryPoints:[entry],bundle:true,write:false,metafile:true,platform:"node",format:"esm",logLevel:"silent",absWorkingDir:repoRoot,plugins:[externalizeDeps as any]});
const inputs=r.metafile!.inputs; const rel=(p:string)=>p;
console.log("modules",Object.keys(inputs).length);
// BFS chain to target
const prev=new Map<string,string>(); const q=[Object.keys(inputs).find(k=>resolve(repoRoot,k)===entry)!]; const seen=new Set(q);
while(q.length){const cur=q.shift()!; for(const imp of inputs[cur]!.imports){ if(imp.external||!inputs[imp.path]) continue; if(!seen.has(imp.path)){seen.add(imp.path);prev.set(imp.path,cur);q.push(imp.path)} }}
const hit=[...seen].find(p=>p.includes(target));
if(hit){const chain=[hit];let c=hit;while(prev.has(c)){c=prev.get(c)!;chain.push(c)}console.log(chain.reverse().join("\n -> "))} else console.log("target not reachable");
