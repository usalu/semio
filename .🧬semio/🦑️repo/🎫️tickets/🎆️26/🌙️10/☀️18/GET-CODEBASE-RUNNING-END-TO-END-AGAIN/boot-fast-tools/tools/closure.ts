import fs from "node:fs";import path from "node:path";import {scanImportsV1} from "./scan-imports.ts";
export function closure(entry:string,opts:{dynamic?:boolean}={}):{files:Map<string,number>;external:Set<string>;json:Set<string>}{
 const files=new Map<string,number>(),external=new Set<string>(),json=new Set<string>(),todo=[entry];
 while(todo.length){const f=todo.pop()!;if(files.has(f))continue;let text:string;try{text=fs.readFileSync(f,"utf8");}catch{continue;}files.set(f,Buffer.byteLength(text));
  const sc=scanImportsV1(text);
  for(const spec of [...sc.static,...(opts.dynamic?sc.dynamic:[])]){const i={path:spec};
   if(!i.path.startsWith(".")){external.add(i.path);continue;}
   const p=path.normalize(path.join(path.dirname(f),i.path));if(p.endsWith(".json")){json.add(p);continue;}todo.push(p);}}
 return {files,external,json};
}
if(import.meta.main){const [entry,...rest]=process.argv.slice(2);const c=closure(path.resolve(entry),{dynamic:rest.includes("--dynamic")});
 console.log(`${path.basename(entry)}: modules=${c.files.size} bytes=${[...c.files.values()].reduce((a,b)=>a+b,0)} json=${c.json.size} external=${[...c.external].filter(e=>!e.startsWith("node:")).join(",")||"-"} builtins=${[...c.external].filter(e=>e.startsWith("node:")||e.startsWith("bun:")).length}`);
 if(rest.includes("--list"))for(const f of c.files.keys())console.log("  "+f.split("/").slice(-4).join("/"));}
