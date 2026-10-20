import {pathToFileURL} from "node:url";

/** ⚖️ One fairness participant: a separate OS process that exercises the lease implementation under its own runtime. */
type Config={implementation:string;role:"reader"|"writer"|"waiter"|"holder"|"queued";api:"queued"|"raw";mode:"shared"|"exclusive";directory:string;resource:string;owner:string;holdMilliseconds:number;untilMilliseconds:number;offsetMilliseconds:number;timeoutMilliseconds?:number};
const config:Config=JSON.parse(process.argv[2]!);
const api=await import(pathToFileURL(config.implementation).href);
const sleep=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
const say=(value:Record<string,unknown>)=>console.log(JSON.stringify({role:config.role,runtime:process.versions.bun?"bun":"node",...value}));
const controller=new AbortController();
process.once("SIGTERM",()=>controller.abort());
const request=(mode:"shared"|"exclusive",timeoutMs?:number)=>config.api==="queued"
  ?api.acquireQueuedResourceLease({directory:config.directory,resource:config.resource,mode,owner:config.owner,signal:controller.signal,sharedCohort:mode==="shared",...(timeoutMs===undefined?{}:{timeoutMs}),onWait:()=>{}})
  :api.acquireResourceLease({directory:config.directory,resource:config.resource,mode,signal:controller.signal,...(timeoutMs===undefined?{}:{timeoutMs}),onWait:()=>{}});
const cpu=()=>{const usage=process.cpuUsage();return(usage.user+usage.system)/1000;};
await sleep(config.offsetMilliseconds);
try{
  if(config.role==="reader"){
    const end=Date.now()+config.untilMilliseconds;let acquisitions=0,worst=0;
    while(Date.now()<end){const started=Date.now(),lease=await request("shared");worst=Math.max(worst,Date.now()-started);acquisitions++;await sleep(config.holdMilliseconds);lease.release();}
    say({done:true,acquisitions,worstWaitMs:worst});
  }else if(config.role==="writer"){
    const started=Date.now();
    try{const lease=await request("exclusive",config.timeoutMilliseconds);say({acquiredAfterMs:Date.now()-started});lease.release();}
    catch(error){say({error:String(error).slice(0,120),afterMs:Date.now()-started});}
  }else if(config.role==="waiter"){
    const started=Date.now(),before=cpu();
    try{const lease=await request("shared",config.timeoutMilliseconds);lease.release();say({outcome:"acquired",cpuMs:cpu()-before,wallMs:Date.now()-started});}
    catch(error){say({outcome:"timed-out",cpuMs:cpu()-before,wallMs:Date.now()-started,error:String(error).slice(0,80)});}
  }else if(config.role==="holder"){
    const lease=await request(config.mode);say({held:true});await sleep(config.holdMilliseconds);lease.release();say({released:true});
  }else{
    const started=Date.now(),lease=await request("exclusive");say({acquiredAfterMs:Date.now()-started});await sleep(config.holdMilliseconds);lease.release();say({released:true});
  }
}catch(error){say({fault:String(error).slice(0,200)});}
process.exit(0);
