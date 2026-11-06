import {test,expect} from "bun:test";
import {createServer,connect} from "node:net";
import {once} from "node:events";
test("loopback echo and close",async()=>{console.error("[DEBUG] net probe entered");const server=createServer(socket=>socket.pipe(socket));await new Promise<void>(resolve=>server.listen(0,"127.0.0.1",resolve));try{const port=(server.address() as {port:number}).port,socket=connect(port,"127.0.0.1");await once(socket,"connect");socket.write(Buffer.from([1,2,3]));expect([...(await once(socket,"data") as [Buffer])[0]]).toEqual([1,2,3]);socket.destroy()}finally{await new Promise<void>(resolve=>server.close(()=>resolve()))}console.error("[DEBUG] net probe closed")});
