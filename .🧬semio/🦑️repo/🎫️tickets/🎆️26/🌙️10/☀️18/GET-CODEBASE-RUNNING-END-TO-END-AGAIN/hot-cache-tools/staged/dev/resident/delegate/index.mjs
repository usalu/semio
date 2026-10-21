/** 🏊 Warm worker pool: idle child processes that already loaded the CLI's module closure and wait for exactly one command each. */
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,renameSync,writeFileSync,existsSync} from "node:fs";
import {join} from "node:path";

/** 📜 Source of the preload that warms the module cache, announces readiness on fd 3, waits for one request line, and then lets the main script run as if it had been started with that command. */
export const WARM_PRELOAD_SOURCE=`import {readSync,writeSync} from "node:fs";
const modules=JSON.parse(process.env.SEMIO_WARM_MODULES??"[]");
for(const module of modules){try{await import(module);}catch{}}
delete process.env.SEMIO_WARM_MODULES;
writeSync(3,"ready\\n");
const buffer=Buffer.alloc(65536);let text="";
for(;;){const count=readSync(3,buffer,0,buffer.length,null);if(count===0)process.exit(0);text+=buffer.subarray(0,count).toString("utf8");if(text.includes("\\n"))break;}
const request=JSON.parse(text.slice(0,text.indexOf("\\n")));
process.argv=[process.argv[0],process.argv[1],...request.argv];
if(request.cwd)process.chdir(request.cwd);
for(const[key,value]of Object.entries(request.env??{})){if(value===null)delete process.env[key];else process.env[key]=value;}
writeSync(3,"started\\n");
`;

/** 📝 Writes the preload under an ASCII path (bun cannot preload a path with emoji) and returns it; content-addressed so concurrent services agree. */
export function writeWarmPreloadV1(directory){
 const name="warm-"+createHash("sha256").update(WARM_PRELOAD_SOURCE).digest("hex").slice(0,12)+".mjs",path=join(directory,name);
 if(!existsSync(path)){mkdirSync(directory,{recursive:true});const temporary=path+"."+process.pid+".tmp";writeFileSync(temporary,WARM_PRELOAD_SOURCE);renameSync(temporary,path);}
 return path;
}

function lines(stream,onLine){
 let text="";
 stream.on("data",chunk=>{text+=chunk.toString("utf8");let at;while((at=text.indexOf("\n"))>=0){onLine(text.slice(0,at));text=text.slice(at+1);}});
}

/** 🏊 Creates the pool. `command` is the argv that starts the main script with the preload already included. */
export function createWarmPoolV1({command,environment,cwd,size,idleMilliseconds,readyTimeoutMilliseconds=60000,spawnProcess=spawn,log=()=>{},now=()=>Date.now()}){
 const idle=[];let starting=0,closed=false,spawned=0,served=0,coldServed=0;
 const create=()=>{
  starting++;spawned++;
  const child=spawnProcess(command[0],command.slice(1),{cwd,env:environment,stdio:["ignore","pipe","pipe","pipe"]});
  const worker={child,state:"starting",bornAt:now(),readyAt:0};
  const control=child.stdio[3];
  worker.ready=new Promise(resolve=>{
   const timer=setTimeout(()=>{if(worker.state==="starting"){child.kill("SIGKILL");resolve(false);}},readyTimeoutMilliseconds);timer.unref?.();
   lines(control,line=>{
    if(line==="ready"&&worker.state==="starting"){worker.state="idle";worker.readyAt=now();starting--;clearTimeout(timer);idle.push(worker);worker.idleTimer=setTimeout(()=>retire(worker),idleMilliseconds);worker.idleTimer.unref?.();resolve(true);}
    else if(line==="started"&&worker.onStarted)worker.onStarted();
   });
   child.once("exit",()=>{if(worker.state==="starting"){starting--;clearTimeout(timer);resolve(false);}const at=idle.indexOf(worker);if(at>=0)idle.splice(at,1);worker.state="dead";});
   child.once("error",error=>{log("warm-spawn-failed",String(error));});
  });
  child.stderr.on("data",()=>{});child.stdout.on("data",()=>{});
  return worker;
 };
 const retire=worker=>{const at=idle.indexOf(worker);if(at>=0)idle.splice(at,1);worker.state="retired";worker.child.kill("SIGTERM");};
 const top=()=>{if(closed||size<=0)return;while(idle.length+starting<size)create();};
 return{
  top,
  /** 🤝 Hands out one warm worker, or undefined when none is ready (the caller then starts a cold process). */
  take(){
   const worker=idle.shift();
   if(!worker){coldServed++;top();return undefined;}
   clearTimeout(worker.idleTimer);worker.state="busy";served++;
   queueMicrotask(top);
   return worker;
  },
  /** ⚙ Runs one command in a taken worker: sends the request, streams output, resolves with the exit status. */
  run(worker,request,{onOutput,signal}){
   return new Promise((resolve,reject)=>{
    const child=worker.child;let started=false;
    const kill=()=>{child.kill("SIGINT");const hard=setTimeout(()=>child.kill("SIGKILL"),5000);hard.unref?.();};
    if(signal?.aborted){kill();}else signal?.addEventListener("abort",kill,{once:true});
    child.stdout.removeAllListeners("data");child.stderr.removeAllListeners("data");
    child.stdout.on("data",chunk=>{void onOutput("stdout",chunk.toString("utf8"));});
    child.stderr.on("data",chunk=>{void onOutput("stderr",chunk.toString("utf8"));});
    worker.onStarted=()=>{started=true;};
    child.once("exit",(code,sig)=>{signal?.removeEventListener("abort",kill);resolve({exit:code??(sig?130:1),signal:sig,started});});
    worker.child.stdio[3].write(JSON.stringify(request)+"\n");
   });
  },
  stats:()=>({idle:idle.length,starting,spawned,served,coldServed}),
  idleCount:()=>idle.length,
  async close(){closed=true;for(const worker of[...idle])retire(worker);},
 };
}

/** ▶ Execution port that runs a task as the real command line: in a resident executor when one is ready and the repository allows it, otherwise in a warm one-shot worker, otherwise in a cold process. */
export function createDelegatedExecutionPortV1({pool,executors,executorArgumentsFor=id=>["run",id],coldCommand,argvFor,environment,cwd,timing=()=>{}}){
 let sequence=0;
 return{
  async run({id,env,client,signal,onSpawn,onOutput}){
   if(executors){
    const executor=executors.take();
    if(executor){
     let wrote=false;
     const result=await executors.run(executor,{id:++sequence,argv:executorArgumentsFor(id),cwd,env:{...(env??{})},client},{onOutput:(stream,text)=>{wrote=true;return onOutput(stream,text);},signal});
     if(result.refusal==="service"){timing("refused");return{exit:result.exit,refusal:{code:result.code,fields:result.fields}};}
     if(!(result.refusal==="authority"||result.lost&&!wrote)){onSpawn?.();timing("executor");return{exit:result.exit};}
    }
   }
   const request={argv:argvFor(id),cwd,env:{...(env??{})}};
   const worker=pool.take();
   if(worker){onSpawn?.();timing("warm");return pool.run(worker,request,{onOutput,signal});}
   timing("cold");
   return new Promise((resolve,reject)=>{
    const child=spawn(coldCommand[0],[...coldCommand.slice(1),...request.argv],{cwd,env:{...environment,...request.env},stdio:["ignore","pipe","pipe"]});
    onSpawn?.();
    const kill=()=>child.kill("SIGINT");signal?.addEventListener("abort",kill,{once:true});
    child.stdout.on("data",chunk=>{void onOutput("stdout",chunk.toString("utf8"));});child.stderr.on("data",chunk=>{void onOutput("stderr",chunk.toString("utf8"));});
    child.once("error",reject);child.once("exit",(code,sig)=>{signal?.removeEventListener("abort",kill);resolve({exit:code??1,signal:sig});});
   });
  },
 };
}
