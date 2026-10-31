const {run}=require("./native-current-validation.cjs"),fs=require("fs"),path=require("path");
(async()=>{
 const messages=[{jsonrpc:"2.0",id:1,method:"initialize",params:{protocolVersion:"2025-06-18",capabilities:{},clientInfo:{name:"linux-readiness",version:"1"}}},{jsonrpc:"2.0",method:"notifications/initialized"},{jsonrpc:"2.0",id:2,method:"tools/list",params:{}},{jsonrpc:"2.0",id:3,method:"resources/list",params:{}}];
 const result=await run("native-decoder-final-mcp",["--repo=/workspaces/semio","mcp","codex"],messages.map(x=>JSON.stringify(x)).join("\n")+"\n");
 const replies=result.stdout.split(/\r?\n/u).filter(Boolean).map(JSON.parse),ids=replies.map(x=>x.id).sort(),initial=replies.find(x=>x.id===1),tools=replies.find(x=>x.id===2),resources=replies.find(x=>x.id===3);
 const valid=result.status===0&&result.signal===null&&!result.forced&&JSON.stringify(ids)===JSON.stringify([1,2,3])&&replies.every(x=>!x.error)&&initial.result.protocolVersion==="2025-06-18"&&Array.isArray(tools.result.tools)&&Array.isArray(resources.result.resources);
 const proof={at:new Date().toISOString(),valid,ids,tools:tools?.result?.tools?.length,resources:resources?.result?.resources?.length,status:result.status,signal:result.signal,forced:result.forced,elapsedMilliseconds:result.elapsedMilliseconds};
 fs.writeFileSync(path.join(__dirname,"🗑️generated/native-decoder-final-mcp-proof.json"),JSON.stringify(proof,null,2)+"\n");
 console.log("[DEBUG] native-final-mcp-proof "+JSON.stringify(proof));if(!valid)throw Error("Actual installed MCP protocol proof required");
})().catch(error=>{console.error("[DEBUG] "+error.stack);process.exitCode=1});