import fs from "node:fs";
import {scanImportsV1} from "/private/tmp/boot-fast/tools/scan-imports.ts";
import {statSync} from "node:fs";
const out=process.env.BOOTFAST_OUT??"/private/tmp/boot-fast/trace.json";
type Call={api:string;path:string;t:number;ms:number;by:string};type Ev={path:string;bytes:number;readMs:number;endMs:number|null;static:string[];dynamic:string[];order:number};
const events:Ev[]=[],byPath=new Map<string,Ev>(),calls:Call[]=[],marks:{name:string;t:number}[]=[];
const now=()=>Bun.nanoseconds()/1e6;
(globalThis as any).__bfEnd=(path:string)=>{const e=byPath.get(path);if(e)e.endMs=now();};
(globalThis as any).__bfMark=(name:string)=>marks.push({name,t:now()});
(globalThis as any).__bfCalls=calls;
Bun.plugin({name:"bf-trace",setup(build){
 build.onLoad({filter:/\.(ts|mjs)$/},args=>{
  if(args.path.includes("/node_modules/")||args.path.includes("/boot-fast/tools/"))return {contents:fs.readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};
  const s=now(),text=fs.readFileSync(args.path,"utf8"),readMs=now()-s,sc=scanImportsV1(text);
  const ev:Ev={path:args.path,bytes:Buffer.byteLength(text),readMs,endMs:null,static:sc.static,dynamic:sc.dynamic,order:events.length};
  events.push(ev);byPath.set(args.path,ev);
  const shimmed=text.replace(/from\s*(["'])(node:)?fs\1/g,'from "/private/tmp/boot-fast/tools/fs-shim.ts"');
  return {contents:shimmed+`\n;globalThis.__bfEnd(${JSON.stringify(args.path)});\n`,loader:args.path.endsWith(".ts")?"ts":"js"};
 });
}});
{const cl=console.log.bind(console),ow=process.stdout.write.bind(process.stdout);let first=true;process.stdout.write=((...a:any[])=>{if(first){first=false;marks.push({name:"first-stdout",t:now()});}return (ow as any)(...a);}) as any;}
process.on("exit",()=>{fs.writeFileSync(out,JSON.stringify({exitMs:now(),events,calls,marks}));});
