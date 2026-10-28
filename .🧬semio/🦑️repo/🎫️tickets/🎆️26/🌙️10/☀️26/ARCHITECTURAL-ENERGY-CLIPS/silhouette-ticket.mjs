import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const root=process.cwd(), ticket=resolve(import.meta.dirname);
const binary=readFileSync(resolve(root,".🧬semio/🦑️repo/native-repo-path"),"utf8").trim();
const child=spawn(binary,["mcp"],{cwd:root,stdio:["pipe","pipe","ignore"],windowsHide:true});
let buffer="", next=1;const pending=new Map();
child.stdout.on("data",chunk=>{buffer+=chunk;let end;while((end=buffer.indexOf("\n"))>=0){const line=buffer.slice(0,end);buffer=buffer.slice(end+1);try{const response=JSON.parse(line);if(pending.has(response.id)){pending.get(response.id)(response);pending.delete(response.id);}}catch{}}});
const call=(method,params)=>new Promise(resolve=>{const id=next++;pending.set(id,resolve);child.stdin.write(JSON.stringify({jsonrpc:"2.0",id,method,params})+"\n");});
try {
await call("initialize",{protocolVersion:"2024-11-05",capabilities:{},clientInfo:{name:"codex",version:"1"}});
child.stdin.write(JSON.stringify({jsonrpc:"2.0",method:"notifications/initialized"})+"\n");
if(process.argv[2]==="reopen"){
for(const uri of ["repo://goals"]){const r=await call("resources/read",{uri});console.log(JSON.stringify({uri,bytes:JSON.stringify(r).length}));}
const r=await call("tools/call",{name:"ticket_reopen",arguments:{path:"26/10/26/ARCHITECTURAL-ENERGY-CLIPS",prompt:"Replace all pictograms in energy clips with original architectural or abstract ink-brush silhouettes informed by Modulor proportions; audit shared and chapter-local art, test and visually verify all changes.",client:"codex",no_management:true}});
console.log(JSON.stringify(r));if(r.error||r.result?.isError)process.exitCode=1;
}else{
const summary=readFileSync(resolve(ticket,"silhouettes.md"),"utf8");
const files=readFileSync(resolve(ticket,"silhouette-files.md"),"utf8").split(String.fromCharCode(10)).filter(line=>line.startsWith("- ")).map(line=>line.slice(2));
if (!files.length) throw new Error("Changed file inventory is empty");
const r=await call("tools/call",{name:"ticket_close",arguments:{path:"26/10/26/ARCHITECTURAL-ENERGY-CLIPS",summary,files,no_management:true}});
console.log(JSON.stringify(r));if(r.error||r.result?.isError)process.exitCode=1;
}
} finally {child.kill();}
