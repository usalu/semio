import {readFileSync} from "node:fs";
import {connect} from "node:net";
const [id,pid]=process.argv.slice(2);if(!id?.startsWith("dashboard-real-")||!pid)throw Error("Owned audit session and PID required");
const socket=connect(readFileSync(".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/daemon.pipe.name","utf8").trim());let buffer=Buffer.alloc(0),stopped=false;
function send(value:unknown):void {const body=Buffer.from(JSON.stringify(value)),prefix=Buffer.alloc(5);prefix.writeUInt32LE(body.length+1);prefix[4]=1;socket.write(Buffer.concat([prefix,body]));}
socket.on("connect",()=>send({type:"list"}));socket.on("error",error=>{throw error;});
socket.on("data",chunk=>{buffer=Buffer.concat([buffer,chunk]);while(buffer.length>=4&&buffer.length>=buffer.readUInt32LE(0)+4){const size=buffer.readUInt32LE(0),kind=buffer[4],body=buffer.subarray(5,size+4);buffer=buffer.subarray(size+4);if(kind!==1)continue;const message=JSON.parse(body.toString());
  if(message.type==="sessions") {const session=message.sessions.find((value:any)=>value.session_id===id);if(!session||session.pid!==Number(pid))throw Error("Audit session identity changed");if(["exited","failed"].includes(session.status)){socket.end();continue;}stopped=true;send({type:"stop",session_id:id});}
  if(message.type==="session_changed"&&message.session.session_id===id&&["exited","failed"].includes(message.session.status)){console.log(`[DEBUG] owned audit cancelled id=${id} pid=${pid} code=${message.session.code}`);send({type:"detach"});socket.end();}
}});
setTimeout(()=>{if(!stopped)throw Error("Owned audit cancellation was not acknowledged");},10000).unref();
