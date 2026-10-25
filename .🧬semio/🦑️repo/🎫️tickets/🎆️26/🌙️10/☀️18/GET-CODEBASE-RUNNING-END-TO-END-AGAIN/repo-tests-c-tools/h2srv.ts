import http2 from "node:http2";import {writeFileSync} from "node:fs";
const events=[];
const server=http2.createServer({settings:{initialWindowSize:1048576}});
server.on("stream",(stream,h)=>{const p=String(h[":path"]);stream.on("close",()=>events.push(p+" close "+stream.rstCode));if(p==="/quick"){stream.respond({":status":200});stream.end("ok");return;}stream.respond({":status":200});stream.on("data",()=>{});stream.write("hello");});
server.listen(0,"127.0.0.1",()=>{console.log(server.address().port);});
setInterval(()=>{writeFileSync(process.argv[2],events.join("\n"));},100);
