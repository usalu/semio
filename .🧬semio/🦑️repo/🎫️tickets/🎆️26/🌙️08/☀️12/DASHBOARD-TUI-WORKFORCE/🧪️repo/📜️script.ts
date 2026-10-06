import assert from "node:assert/strict";
import {spawn} from "node:child_process";
import {appendFileSync,readFileSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
const ticket=resolve(import.meta.dir,".."),generated=join(ticket,"🗑️generated"),mode=process.argv[2]??"schema",pending=new Map<number,{resolve:(value:any)=>void;reject:(error:Error)=>void}>();
const child=spawn(process.execPath,["./📜️script.ts","dev","mcp","stdio","codex"],{cwd:process.cwd(),stdio:["pipe","pipe","pipe"],windowsHide:true});
let buffer="",sequence=0;
const exited=new Promise<number|null>(accept=>child.once("exit",accept));
child.stderr.on("data",chunk=>appendFileSync(join(generated,`repo-${mode}-stderr.log`),chunk));
child.stdout.on("data",chunk=>{
 appendFileSync(join(generated,`repo-${mode}-stdout.log`),chunk);buffer+=chunk.toString();
 for(;;){const index=buffer.indexOf("\n");if(index<0)break;const line=buffer.slice(0,index);buffer=buffer.slice(index+1);let message;try{message=JSON.parse(line);}catch{continue;}const request=pending.get(message.id);if(!request)continue;pending.delete(message.id);if(message.error)request.reject(Error(JSON.stringify(message.error)));else request.resolve(message.result);}
});
function send(method:string,params:unknown,id?:number):void{child.stdin.write(JSON.stringify({jsonrpc:"2.0",method,params,...(id===undefined?{}:{id})})+"\n");}
async function request(method:string,params:unknown):Promise<any>{const id=++sequence,timer=setTimeout(()=>pending.get(id)?.reject(Error(`MCP ${method} deadline`)),300000);try{return await new Promise((resolve,reject)=>{pending.set(id,{resolve,reject});send(method,params,id);});}finally{clearTimeout(timer);pending.delete(id);}}
try{
 await request("initialize",{protocolVersion:"2024-11-05",capabilities:{},clientInfo:{name:"codex-ticket-verification",version:"1.0.0"}});send("notifications/initialized",{});
 if(mode==="reopen"){const goals=await request("resources/read",{uri:"repo://goals"});writeFileSync(join(generated,"startup-goals.json"),JSON.stringify(goals));const listing=await request("tools/list",{});const tool=listing.tools.find((value:any)=>value.name==="ticket_reopen");assert.ok(tool);console.log(JSON.stringify(tool.inputSchema));const result=await request("tools/call",{name:"ticket_reopen",arguments:{path:"26/08/12/DASHBOARD-TUI-WORKFORCE",client:"codex",goal:"🎯runningframework🎯runningproducts🎯runningrepo",prompt:"Make the dashboard a fast clean native Rust binary with opinionated defaults, persistent optional customization, simple startup and complete developer controls.",no_management:true}});assert.ok(!result.isError,JSON.stringify(result));console.log("[DEBUG] ticket_reopen "+JSON.stringify(result).slice(0,900));}
 else if(mode==="schema"){const result=await request("tools/list",{});const tool=result.tools.find((value:any)=>value.name==="ticket_close");assert.ok(tool);console.log(JSON.stringify(tool.inputSchema));}
 else if(mode==="close"){const args=JSON.parse(readFileSync(join(import.meta.dir,"🔣️close.json"),"utf8")),result=await request("tools/call",{name:"ticket_close",arguments:args});assert.ok(!result.isError,JSON.stringify(result));console.log("[DEBUG] repo MCP ticket_close "+JSON.stringify(result));}
 else throw Error(`Unknown mode ${mode}`);
}finally{
 child.stdin.end();const timer=setTimeout(()=>child.kill(),5000);await exited;clearTimeout(timer);
}
