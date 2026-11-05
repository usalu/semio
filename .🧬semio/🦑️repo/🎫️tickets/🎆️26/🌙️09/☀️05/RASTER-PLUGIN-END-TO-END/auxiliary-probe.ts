import {startToolV1,runToolSyncV1} from "@semio-tech/process/tools";

for(const tool of ["node","bun"]){
  const query=runToolSyncV1(tool,["-e","console.log('raster-runtime-probe')"],{root:process.cwd(),cwd:process.cwd(),trusted:true,encoding:"utf8",timeout:5000});
  console.log(JSON.stringify({tool,mode:"synchronous-query",status:query.status,signal:query.signal,stdout:query.stdout,stderr:query.stderr,error:String(query.error)}));
for(const mode of ["pipe","overlapped"] as const)for(const descriptor of [0,3]){
  const child=startToolV1(tool,["-e",`const fs=require('node:fs');console.log(fs.readFileSync(${descriptor},'utf8'));`],{root:process.cwd(),cwd:process.cwd(),trusted:true,stdio:descriptor===3?["ignore","pipe","pipe",mode]:[mode,"pipe","pipe"],timeout:5000});
  let stdout="",stderr="",failure:unknown=null;
  child.stdout.on("data",bytes=>{stdout=(stdout+String(bytes)).slice(-1024);});
  child.stderr.on("data",bytes=>{stderr=(stderr+String(bytes)).slice(-1024);});
  const pipe=child.stdio[descriptor] as import("node:stream").Writable;
  pipe.on("error",error=>{failure={code:(error as NodeJS.ErrnoException).code,message:error.message};});
  const write=new Promise<void>(accept=>pipe.end("raster-auxiliary-probe","utf8",error=>{if(error)failure={code:(error as NodeJS.ErrnoException).code,message:error.message};accept();}));
  const terminal=await new Promise(resolve=>{child.once("error",error=>resolve({error:String(error)}));child.once("close",(code,signal)=>resolve({code,signal}));});
  await write;console.log(JSON.stringify({tool,mode,descriptor,executable:child.spawnfile,terminal,failure,stdout,stderr}));
}
}
