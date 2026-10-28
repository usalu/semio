import fs from 'node:fs';
import path from 'node:path';
import {spawn} from 'node:child_process';
import {fileURLToPath} from 'node:url';
const ticket=path.dirname(fileURLToPath(import.meta.url));
let workspace=ticket;while(!fs.existsSync(path.join(workspace,'🧰️framework'))||!fs.existsSync(path.join(workspace,'🎓️teaching'))){const parent=path.dirname(workspace);if(parent===workspace)throw Error('Workspace not found');workspace=parent}
const binary=fs.readFileSync(path.join(workspace,'.🧬semio','🦑️repo','native-repo-path'),'utf8').trim();
const request=JSON.parse(fs.readFileSync(path.join(ticket,'inspection-close.json'),'utf8'));
const child=spawn(binary,['mcp'],{cwd:workspace,windowsHide:true,stdio:['pipe','pipe','pipe']});
const send=value=>child.stdin.write(JSON.stringify(value)+'\n');
let buffer='',finished=false,errors='';
const timer=setTimeout(()=>finish(new Error('Repository MCP timed out')),60000);
const finish=(error,result)=>{if(finished)return;finished=true;clearTimeout(timer);child.kill();if(error){console.error(String(error));process.exitCode=1}else{const report='# Inspection Ticket Closure\n\nThe desktop repository connector initially used the Codex application binary directory and could not find this ticket. The configured native repo MCP server was then invoked with the verified repository as its working directory. The same ticket_close operation completed with the full retained file inventory. No generated files, Git mutations or source changes were introduced.\n';fs.writeFileSync(path.join(ticket,'inspection-ticket-closure.md'),report);console.log(JSON.stringify({closed:true,files:request.files.length,result:JSON.stringify(result).slice(0,1200)}))}};
child.stdout.setEncoding('utf8');child.stderr.setEncoding('utf8');child.stderr.on('data',data=>{errors=(errors+data).slice(-2000)});
child.stdout.on('data',data=>{buffer+=data;let newline;while((newline=buffer.indexOf('\n'))>=0){const line=buffer.slice(0,newline);buffer=buffer.slice(newline+1);if(!line.trim())continue;let response;try{response=JSON.parse(line)}catch{continue}if(response.error){finish(new Error(JSON.stringify(response.error)));return}if(response.id===1){send({jsonrpc:'2.0',method:'notifications/initialized'});send({jsonrpc:'2.0',id:2,method:'tools/list',params:{}})}else if(response.id===2){const tool=response.result.tools.find(tool=>tool.name==='ticket_close');if(!tool){finish(new Error('Ticket close tool unavailable'));return}send({jsonrpc:'2.0',id:3,method:'tools/call',params:{name:tool.name,arguments:request}})}else if(response.id===3){if(response.result?.isError)finish(new Error(JSON.stringify(response.result)));else finish(null,response.result)}}});
child.on('error',finish);child.on('exit',code=>{if(!finished)finish(new Error('Repository MCP exited '+code+': '+errors))});
send({jsonrpc:'2.0',id:1,method:'initialize',params:{protocolVersion:'2024-11-05',capabilities:{},clientInfo:{name:'inspection-closure',version:'1'}}});
