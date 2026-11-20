const {createHash} = require("node:crypto");
const http = require("node:http"), net = require("node:net"), fs = require("node:fs"), path = require("node:path");
const {apps,publicApps,root,identity,previous:old,controlPort,generated,readiness,createPreviewLifecycle} = require("./preview-process.cjs");
const previews = createPreviewLifecycle();
fs.mkdirSync(generated, {recursive:true});
const bridge = () => fs.readFileSync(path.join(root,"runner-bridge.js"));
const osModule = "/@fs/" + path.join(old,"round3-os-live.tsx").replaceAll(path.sep,"/").replace(/^\//,"").split("/").map(encodeURIComponent).join("/");
const checkpoints = path.join(generated, "checkpoints");
const attempts = path.join(generated, "attempts");
for (const directory of [checkpoints,attempts]) fs.mkdirSync(directory,{recursive:true});
const progress = {};
const jsonType = "application/json";
const read = req => new Promise((resolve,reject)=>{
  let body = "";
  req.setEncoding("utf8");
  req.on("data",chunk=>{
    body+=chunk;
    if(body.length>2_000_000){reject(new Error("Matrix payload is too large"));req.destroy()}
  });
  req.on("end",()=>{try{resolve(JSON.parse(body))}catch(error){reject(error)}});
  req.on("error",reject);
});
function write(file,value){
  const temporary=file+".tmp";
  fs.writeFileSync(temporary,JSON.stringify(value));
  fs.renameSync(temporary,file);
}
function checkpoint(app, query){
  const locale=query.get("locale"),width=query.get("width"),height=query.get("height");
  if(!["elements","os"].includes(app.id)||!["en","de"].includes(locale)||!["375","768","1440"].includes(width)||height!=="900")return;
  const files=["runner-bridge.js","silhouette-probe.js",app.id==="elements"?"sweep-boot.js":"os-sweep.js",app.id==="elements"?"elements-live.tsx":path.join(old,"round3-os-live.tsx")];
  const hash=createHash("sha256");
  for(const file of files)hash.update(fs.readFileSync(path.isAbsolute(file)?file:path.join(root,file)));
  const version=hash.digest("hex").slice(0,12);
  return path.join(checkpoints,[app.id,locale,width,height,version].join("-")+".json");
}
function cellName({app,locale,width,height}){
  if (!apps.some(item=>item.id===app) || !["en","de"].includes(locale) ||
      ![375,768,1440].includes(width) || height !== 900) return;
  return [app,locale,width,height].join("-");
}
function stored(file){
  return fs.existsSync(file) ? JSON.parse(fs.readFileSync(file,"utf8")) : undefined;
}
async function registerAttempt(req,res){
  const value = await read(req);
  const name = cellName(value);
  if (!name || typeof value.attempt !== "string" || !/^[a-z0-9-]{36}$/.test(value.attempt)) {
    return reply(res,400,"text/plain","Invalid attempt");
  }
  const file = path.join(attempts,name + ".json");
  const previous = stored(file);
  const {attempt,state,message} = value;
  if (state !== "pending" && previous?.attempt !== attempt) {
    return reply(res,409,"text/plain","The attempt was replaced");
  }
  if (!["pending","timeout","cancelled","error"].includes(state)) return reply(res,400,"text/plain","Invalid state");
  if (state !== "pending") {
    const result = stored(path.join(generated,name + ".json"));
    if (result?.attempt === attempt && !result.errors?.length) return reply(res,409,"text/plain","The result is already saved");
  }
  write(file,{attempt,state,message:typeof message==="string"?message:undefined,at:new Date().toISOString()});
  return reply(res,200,jsonType,"{}");
}
async function saveResult(app,req,res){
  const value = await read(req);
  const name = cellName(value);
  if (!name || app.id !== value.app) return reply(res,400,"text/plain","Invalid result cell");
  const latest = stored(path.join(attempts,name + ".json"));
  if (latest && (latest.attempt !== value.attempt || latest.state !== "pending")) {
    return reply(res,409,"text/plain","The attempt has ended or been replaced");
  }
  write(path.join(generated,name + ".json"),value);
  return reply(res,200,jsonType,"{}");
}
async function bridgeRequest(app,req,res){
  const url=new URL(req.url,"http://127.0.0.1");
  if(url.pathname==="/__matrix_result"&&req.method==="POST")return saveResult(app,req,res);
  if(url.pathname==="/__matrix_checkpoint"){
    const file=checkpoint(app,url.searchParams);
    if(!file)return reply(res,400,"text/plain","Invalid checkpoint cell");
    if(req.method==="GET")return reply(res,200,jsonType,fs.existsSync(file)?fs.readFileSync(file):"{}");
    if(req.method==="POST"){
      const {key, story}=await read(req);
      if(typeof key!=="string"||!key||!story||story.name!==key)return reply(res,400,"text/plain","Invalid checkpoint story");
      const data=fs.existsSync(file)?JSON.parse(fs.readFileSync(file,"utf8")):{};
      write(file,{...data,[key]:story});
      return reply(res,200,jsonType,"{}");
    }
    if(req.method==="DELETE"){
      fs.rmSync(file,{force:true});
      return reply(res,200,jsonType,"{}");
    }
  }
  if(url.pathname==="/__matrix_progress"&&req.method==="POST"){
    const record=await read(req);
    progress[app.id]={...record,app:app.id,at:new Date().toISOString()};
    return reply(res,200,jsonType,"{}");
  }
  reply(res,404,"text/plain","Not found");
}
function reply(res,status,type,body){res.writeHead(status,{"content-type":type,"cache-control":"no-store"});res.end(body)}
async function control(req,res){
  const url = new URL(req.url,"http://127.0.0.1");
  const pathname = url.pathname;
  if (pathname === "/attempt" && req.method === "POST") return registerAttempt(req,res);
  if (pathname === "/progress") return reply(res,200,jsonType,JSON.stringify(progress));
  if (pathname === "/apps") return reply(res,200,jsonType,JSON.stringify(publicApps));
  if (pathname === "/previews") return reply(res,200,jsonType,JSON.stringify({identity}));
  const [,family,subject] = pathname.split("/");
  const app = apps.find(item=>item.id===subject);
  if (family === "ready" && app) {
    const status = await readiness(app);
    return reply(res,status.ready?200:503,"text/plain",String(status.status));
  }
  if (family === "start" && app) {
    const result = await previews.startBatch(app);
    return reply(res,200,"text/plain",result);
  }
  if (pathname === "/stop") {
    await previews.stopBatch();
    return reply(res,200,"text/plain","stopped");
  }
  if (req.method === "POST" && family === "previews") {
    const [, , action, id] = pathname.split("/");
    const selected = id === "all" ? apps : apps.filter(item=>item.id===id);
    if (!selected.length) return reply(res,404,"text/plain","Unknown app");
    if (action === "start" || action === "restart") {
      const results = await Promise.all(selected.map(async item=>[item.id,await previews.startManual(item,action === "restart")]));
      return reply(res,200,jsonType,JSON.stringify(Object.fromEntries(results)));
    }
    if (action === "stop") {
      for (const item of selected) await previews.stopManual(item.id);
      return reply(res,200,jsonType,"{}");
    }
  }
  if (pathname === "/") return reply(res,200,"text/html; charset=utf-8",fs.readFileSync(path.join(root,"runner.html")));
  return reply(res,404,"text/plain","Not found");
}
http.createServer((req,res)=>{
  void control(req,res).catch(error=>reply(res,500,"text/plain",String(error)));
}).listen(controlPort,"127.0.0.1");
for(const app of apps){const server=http.createServer((req,res)=>{if(req.url.startsWith("/__matrix_checkpoint?")||["/__matrix_progress","/__matrix_result"].includes(req.url)){void bridgeRequest(app,req,res).catch(error=>reply(res,400,"text/plain",String(error)));return}if(req.url==="/__matrix_elements_sweep.js")return reply(res,200,"text/javascript",fs.readFileSync(path.join(root,"sweep-boot.js")));if(req.url==="/__matrix_os_sweep.js")return reply(res,200,"text/javascript",fs.readFileSync(path.join(root,"os-sweep.js")));if(req.url==="/__matrix_os_live"){res.writeHead(302,{location:osModule});return res.end()}if(req.url==="/__matrix_bridge.js")return reply(res,200,"text/javascript",bridge());if(req.url==="/__matrix_app_sweep.js"){let script=fs.readFileSync(path.join(root,"app-sweep.js"),"utf8");script=script.replace(/  if (!window.__pumped) {[\s\S]*?\n  }/,"");return reply(res,200,"text/javascript",script)}if(req.url==="/__matrix_probe.js")return reply(res,200,"text/javascript",fs.readFileSync(path.join(root,"silhouette-probe.js")));const upstream=http.request({host:"127.0.0.1",port:app.port,path:req.url,method:req.method,headers:{...req.headers,host:"127.0.0.1:"+app.port,"accept-encoding":"identity"}},up=>{let headers={...up.headers};if((headers["content-type"]||"").includes("text/html")){delete headers["content-length"];res.writeHead(up.statusCode,headers);let html="";up.setEncoding("utf8");up.on("data",x=>html+=x);up.on("end",()=>res.end(html.replace(/<head([^>]*)>/i,'<head$1><script src="/__matrix_bridge.js"></script>')));}else{res.writeHead(up.statusCode,headers);up.pipe(res)}});upstream.on("error",e=>reply(res,502,"text/plain",app.id+" unavailable: "+e.message));req.pipe(upstream)});server.on("upgrade",(req,socket,head)=>{const upstream=net.connect(app.port,"127.0.0.1",()=>{upstream.write(req.method+" "+req.url+" HTTP/"+req.httpVersion+"\r\n"+req.rawHeaders.reduce((s,v,i,a)=>i%2?s+a[i-1]+": "+v+"\r\n":s,"")+"\r\n");upstream.write(head);socket.pipe(upstream).pipe(socket)});upstream.on("error",()=>socket.destroy())});server.listen(app.proxy,"127.0.0.1")}
console.log("Matrix runner: http://127.0.0.1:" + controlPort + "/");

let closing = false;
for (const signal of ["SIGINT","SIGTERM"]) process.on(signal,async ()=>{
  if (closing) return;
  closing = true;
  await previews.shutdown();
  process.exit(0);
});
