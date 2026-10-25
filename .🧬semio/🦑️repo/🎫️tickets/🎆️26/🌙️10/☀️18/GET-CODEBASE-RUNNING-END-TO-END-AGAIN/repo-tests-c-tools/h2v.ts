import http2 from "node:http2";
const log=(...a:unknown[])=>console.log("[h2]",...a);
async function round(label:string,warm:boolean){
  const server=http2.createServer({settings:{initialWindowSize:1048576}});const sessions=new Set<http2.ServerHttp2Session>();server.on("session",s=>{sessions.add(s);s.once("close",()=>sessions.delete(s));});
  const events:string[]=[];
  server.on("stream",(stream,headers)=>{
    const path=String(headers[":path"]);
    stream.on("close",()=>events.push(path+" close rst="+stream.rstCode));
    stream.on("aborted",()=>events.push(path+" aborted"));
    if(path==="/quick"){stream.respond({":status":200});stream.end("ok");return;}
    stream.respond({":status":200},{waitForTrailers:true});stream.on("data",()=>{});stream.write("hello");
  });
  await new Promise<void>(r=>server.listen(0,"127.0.0.1",r));
  const port=(server.address() as any).port;
  const session=http2.connect("http://127.0.0.1:"+port);await new Promise<void>(r=>session.once("remoteSettings",()=>r()));
  if(warm){
    for(let i=0;i<5;i++){
      await new Promise<void>(r=>{const q=session.request({":method":"POST",":path":"/quick"});q.on("data",()=>{});q.on("end",r);q.end();});
    }
  }
  const req=session.request({":method":"POST",":path":"/long"});req.end("x");
  req.on("error",()=>{});
  await new Promise<void>(r=>{req.on("data",()=>r());});
  req.close(http2.constants.NGHTTP2_CANCEL);
  await new Promise(r=>setTimeout(r,500));
  log(label,"server saw:",events.filter(e=>e.startsWith("/long")).join(" | ")||"NOTHING");
  session.close();for(const s of sessions)s.destroy();await new Promise<void>(r=>server.close(()=>r()));
}
await round("cold",false);
await round("warm",true);
await round("cold-again",false);
await round("warm-again",true);
