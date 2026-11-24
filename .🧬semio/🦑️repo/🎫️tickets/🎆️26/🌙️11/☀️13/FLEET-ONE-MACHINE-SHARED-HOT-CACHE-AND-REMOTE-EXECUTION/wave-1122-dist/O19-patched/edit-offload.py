import sys
p=sys.argv[1]
s=open(p).read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:70],s.count(old))
    s=s.replace(old,new)
rep('import {readRemoteExecutionPolicyV1,remoteExecutionHostPropertiesV1,type RemoteExecutionPropertyV1}','import {readRemoteExecutionPolicyV1,remoteExecutionHostPropertiesV1,remoteExecutionMatchPropertiesV1,type RemoteExecutionPropertyV1}')
rep('import {readOffloadMemoV1,readUnitCostV1,','import {eligibleFleetWorkersV1,fleetWorkersV1,readOffloadMemoV1,readUnitCostV1,')
rep('uploadMilliseconds?:number}>;','uploadMilliseconds?:number;workers?:Readonly<{own:string;workers:readonly FleetWorkerV1[]}>}>;')
rep('import {releaseImplicitTokenV1}','import type {FleetWorkerV1} from "./🚦️admission/🟦️.ts";\nimport {releaseImplicitTokenV1}')
rep('"no-idle-machine","queue-timeout","infrastructure","incomplete-inputs","worker-policy"','"no-idle-machine","no-compatible-worker","queue-timeout","infrastructure","incomplete-inputs","worker-policy"')
rep('const declined=outcome.reason==="no-idle-machine"||outcome.reason==="queue-timeout"||','const declined=outcome.reason==="no-idle-machine"||outcome.reason==="no-compatible-worker"||outcome.reason==="queue-timeout"||')
rep('outcome.reason==="no-idle-machine"||outcome.reason==="queue-timeout"?{refused:1}','outcome.reason==="no-idle-machine"||outcome.reason==="no-compatible-worker"||outcome.reason==="queue-timeout"?{refused:1}')
anchor='cancelled=input.signal??new AbortController().signal;\n'
i=s.index(anchor,s.index('const learned=readUnitPeakV1'))+len(anchor)
s=s[:i]+'''  const fleet=ports?.workers??(ports?.endpoint!==undefined?undefined:await fleetWorkersV1(environment,input.root));
  if(fleet!==undefined){const fit=eligibleFleetWorkersV1(fleet,remoteExecutionMatchPropertiesV1(platform));if(fit.eligible===0)return settle(local("no-compatible-worker",`none of ${fit.total} workers advertises ${fit.unmet.length>0?fit.unmet.join(", "):"all of it together"}`));}
'''+s[i:]
rep(''' const declined=input.queue===true?undefined:readOffloadMemoV1''',''' const full=readOffloadMemoV1<{atMs:number}>(input.root,"store-full",Date.now(),policy.limits.storeFullMemoMilliseconds);
 if(full)return{decline:"infrastructure",detail:"memo the content store is full (507)"};
 const declined=input.queue===true?undefined:readOffloadMemoV1''')
old='''  if(input.signal?.aborted)return local("cancelled");
  return settle(local("infrastructure",String((error as Error)?.message??error).slice(0,120)));
 }
}

/** 🏷️ The platform properties of an offloaded command'''
new='''  if(input.signal?.aborted)return local("cancelled");
  if((error as {code?:unknown})?.code===8&&/answered 507\\b/u.test(String((error as Error)?.message)))writeOffloadMemoV1(input.root,"store-full",{atMs:Date.now()});
  return settle(local("infrastructure",String((error as Error)?.message??error).slice(0,120)));
 }
}

/** 🏷️ The platform properties of an offloaded command'''
rep(old,new)
open(p,'w').write(s)
