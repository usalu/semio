import {createServer,connect} from "node:net";
import {once} from "node:events";
import assert from "node:assert/strict";
const signal=AbortSignal.timeout(15000),server=createServer(),live=new Set();let calls=0;
await new Promise((resolve,reject)=>{server.once("error",reject);server.listen(0,"127.0.0.1",resolve)});
try{
 for(let i=0;i<128;i++){
  signal.throwIfAborted();const accepted=once(server,"connection",{signal}),client=connect(server.address().port,"127.0.0.1");live.add(client);client.on("error",()=>{});
  const [,row]=await Promise.all([once(client,"connect",{signal}),accepted]),peer=row[0];live.add(peer);peer.on("error",()=>{});
  client.once("close",()=>{assert(peer.destroyed);peer.end();calls++});peer.once("close",()=>{assert(client.destroyed);client.end();calls++});
  const closed=Promise.all([once(client,"close",{signal}),once(peer,"close",{signal})]);client.destroy();peer.destroy();await closed;live.delete(client);live.delete(peer);
 }
 assert.equal(calls,256);console.log(JSON.stringify({cycles:128,endOnDestroyedCalls:calls}));
}finally{for(const socket of live)socket.destroy();await new Promise(resolve=>server.close(resolve))}
