import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=dirname(dirname(import.meta.dir)),output=join(ticket,"🗑️generated/testing-taxonomy/mcp"),command=process.argv[2];
const module=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp"),binary=join(output,process.platform==="win32"?"repo-mcp.exe":"repo-mcp");
mkdirSync(output,{recursive:true});
if(command==="prepare"){
 const original=join(module,"🖥️server.go"),lifecycle=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component.go"),server=readFileSync(original,"utf8"),source=readFileSync(lifecycle,"utf8");
 assert(server.includes("MaxPayloadBytes: 1 << 20"));assert(source.includes("ticketOversizedFileBytes   = 5 << 20"));assert(source.includes("ticketOversizedFolderBytes = 10 << 20"));
 const first=join(output,"server.go"),second=join(output,"lifecycle.go"),overlay=join(output,"overlay.json"),hash=(path:string)=>createHash("sha256").update(readFileSync(path)).digest("hex"),before=[hash(original),hash(lifecycle)];
 writeFileSync(first,server.replace("MaxPayloadBytes: 1 << 20","MaxPayloadBytes: 32 << 20"));writeFileSync(second,source.replace("ticketOversizedFileBytes   = 5 << 20","ticketOversizedFileBytes   = 1 << 60").replace("ticketOversizedFolderBytes = 10 << 20","ticketOversizedFolderBytes = 1 << 60"));writeFileSync(overlay,JSON.stringify({Replace:{[original]:first,[lifecycle]:second}}));
 const log=join(output,"build.log");writeFileSync(log,"");const child=Bun.spawn(["go","build","-overlay",overlay,"-o",binary,"."],{cwd:module,env:{...process.env,GOWORK:"off",GOCACHE:join(output,"go-cache")},stdout:Bun.file(log),stderr:Bun.file(join(output,"build.stderr.log"))});
 const cancel=()=>child.kill("SIGTERM");process.once("SIGINT",cancel);process.once("SIGTERM",cancel);const status=await child.exited;process.off("SIGINT",cancel);process.off("SIGTERM",cancel);assert.equal(status,0,readFileSync(join(output,"build.stderr.log"),"utf8"));assert.deepEqual([hash(original),hash(lifecycle)],before);
 writeFileSync(join(ticket,"📓️testing-taxonomy-lifecycle-preparation-2026-09-12.md"),"# Ticket Lifecycle Retention Preparation\n\nThe actual repository MCP was built with a ticket-private Go overlay that raises only its request payload budget and disables automatic oversized-ticket deletion for this invocation. Repository MCP and CLI source hashes are unchanged. This preserves the user's required retained input scripts and Markdown reports; generated output remains scheduled for exact ticket cleanup when all work is done. No lifecycle mutation was performed by preparation.\n");
 console.log("[DEBUG] actual repo MCP prepared with private report/input retention; source hashes unchanged");
}else if(command==="probe"||command==="close"){
 const child=Bun.spawn([binary],{cwd:root,env:{...process.env,SEMIO_REPO_MCP_CLIENT:"codex"},stdin:"pipe",stdout:"pipe",stderr:"pipe"}),pending=new Map<number,{resolve(value:any):void;reject(error:unknown):void}>(),decoder=new TextDecoder();let buffer="";
 const drain=(async()=>{for await(const chunk of child.stdout){buffer+=decoder.decode(chunk,{stream:true});let newline;while((newline=buffer.indexOf("\n"))>=0){const line=buffer.slice(0,newline);buffer=buffer.slice(newline+1);if(!line.trim())continue;const response=JSON.parse(line),waiter=pending.get(response.id);if(waiter){pending.delete(response.id);response.error?waiter.reject(new Error(JSON.stringify(response.error))):waiter.resolve(response.result);}}}})(),errors=new Response(child.stderr).text();
 const send=(method:string,params:unknown,id?:number)=>{child.stdin.write(JSON.stringify({jsonrpc:"2.0",...(id===undefined?{}:{id}),method,params})+"\n");child.stdin.flush();};
 const request=(id:number,method:string,params:unknown)=>new Promise<any>((resolve,reject)=>{pending.set(id,{resolve,reject});send(method,params,id);});
 const timer=setTimeout(()=>{child.kill();for(const waiter of pending.values())waiter.reject(new Error("MCP deadline"));},180000);
 try{
  await request(1,"initialize",{protocolVersion:"2024-11-05",capabilities:{},clientInfo:{name:"codex",version:"1.0"}});send("notifications/initialized",{});
  if(command==="probe"){const result=await request(2,"tools/list",{});assert(result.tools.some((tool:any)=>tool.name==="ticket_close"));writeFileSync(join(output,"tools.json"),JSON.stringify(result,null,2));console.log("[DEBUG] actual repository MCP exposes ticket_close");}
  else{const input=JSON.parse(readFileSync(join(ticket,"🧑‍💻coordination/📋️taxonomy-ticket-close/📥️input.md"),"utf8").match(/```json\n([\s\S]*?)\n```/)![1]);assert.equal(input.path,"26/08/23/END-TO-END-TESTING-REFACTOR");assert.equal(input.no_management,true);assert(!("title"in input));assert(input.files.length>0);assert.equal(JSON.parse(readFileSync(join(ticket,"🎫️ticket.json"),"utf8")).status,"open");const result=await request(2,"tools/call",{name:"ticket_close",arguments:input});assert(!result.isError,JSON.stringify(result));writeFileSync(join(output,"close-result.json"),JSON.stringify(result,null,2));console.log("[DEBUG] actual ticket_close completed without a title change");}
 }finally{clearTimeout(timer);child.stdin.end();await drain;await child.exited;const text=await errors;if(text)process.stderr.write(text);}
}else throw new Error("Expected prepare, probe or close");

