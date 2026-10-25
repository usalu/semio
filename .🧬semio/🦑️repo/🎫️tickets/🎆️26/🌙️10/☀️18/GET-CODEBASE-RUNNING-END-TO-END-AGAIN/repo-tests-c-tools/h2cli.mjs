import http2 from "node:http2";
const port=Number(process.argv[2]);
for(let round=0;round<3;round++){
 const s=http2.connect("http://127.0.0.1:"+port);
 for(let i=0;i<3;i++)await new Promise<void>(r=>{const q=s.request({":method":"POST",":path":"/quick"});q.on("data",()=>{});q.on("end",r);q.end();});
 const req=s.request({":method":"POST",":path":"/long"+round});req.on("error",()=>{});req.end("x");
 await new Promise<void>(r=>{req.on("data",()=>r());});
 req.close(http2.constants.NGHTTP2_CANCEL);await new Promise(r=>setTimeout(r,300));s.close();
}
process.exit(0);
