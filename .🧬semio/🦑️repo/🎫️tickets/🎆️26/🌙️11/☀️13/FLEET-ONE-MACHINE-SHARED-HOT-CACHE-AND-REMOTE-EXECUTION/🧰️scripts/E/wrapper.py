import glob,sys
U=glob.glob("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache")[0]
path=sys.argv[1] if len(sys.argv)>1 else U+"/📜️script.ts"
s=open(path).read()
def once(a,b):
    global s
    assert s.count(a)==1,(a[:70],s.count(a))
    s=s.replace(a,b)
once('import { isMainModuleV1 }','import {OFFLOAD_POLICY_V1,offloadModeV1} from "../../../🏗️build/🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts";\nimport type {OffloadAdmissionPortsV1} from "../../../🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts";\nimport { isMainModuleV1 }')
once('export async function wrapRustcV1(argv:readonly string[],environment:Environment,cwd:string):Promise<number>{','export async function wrapRustcV1(argv:readonly string[],environment:Environment,cwd:string,ports:Readonly<{admission?:OffloadAdmissionPortsV1}>={}):Promise<number>{')
once(' try{if(await serve()){event("hit"',''' const offloadUnitV1=async():Promise<(Outcome&{origin:string})|undefined>=>{
  const mode=offloadModeV1(environment);
  if(mode==="never"||!unit.source.endsWith(".rs")||(unit.crateTypes.length?unit.crateTypes:["bin"]).some(type=>!OFFLOAD_POLICY_V1.crateTypes.includes(type)))return undefined;
  const controller=new AbortController();let received:number|undefined;
  const handlers=(["SIGINT","SIGTERM"] as const).map(name=>{const handler=()=>{received=name==="SIGINT"?130:143;controller.abort();};process.once(name,handler);return[name,handler] as const;});
  try{
   const offload=await import("../../../🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts");
   if(mode==="auto"&&offload.slotQueueMillisecondsV1(settings.root,settings.slots)<OFFLOAD_POLICY_V1.limits.queueThresholdMilliseconds&&(await offload.sharedUnitAdmissionV1(settings.root,ports.admission)).verdict!=="wait")return undefined;
   const answer=await offload.offloadRustcUnitV1({args,environment,cwd,root:settings.root,lane:settings.lane,crate,key,compiler:settings.inner,digest:path=>digestFileV1(path,settings.stamps),signal:controller.signal});
   if(received!==undefined)return{status:received,stdout:"",stderr:"",origin:""};
   return answer.kind==="done"?{status:answer.status,stdout:answer.stdout,stderr:answer.stderr,origin:answer.origin}:undefined;
  }catch{return received===undefined?undefined:{status:received,stdout:"",stderr:"",origin:""};}
  finally{for(const [name,handler] of handlers)process.off(name,handler);}
 };
 try{if(await serve()){event("hit"''')
once('const compiled=Date.now(),outcome=await compile(true);','const compiled=Date.now(),remote=await offloadUnitV1(),outcome=remote??await compile(true);\n  if(remote){process.stdout.write(remote.stdout);process.stderr.write(remote.stderr);}')
once('event("miss",key,Date.now()-compiled,waited,unit.incremental===undefined?parts:parts+" incremental:lane");','event("miss",key,Date.now()-compiled,waited,(unit.incremental===undefined?parts:parts+" incremental:lane")+(remote?" offloaded:"+remote.origin:""));')
once('const provenance=facts?await recallProvenanceV1(environment,facts):undefined;','const provenance=facts&&!remote?await recallProvenanceV1(environment,facts):undefined;')
open(path,"w").write(s)
print("ok")
