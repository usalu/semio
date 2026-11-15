LIB="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
RE=LIB+"🌐️remote-execution/"
def edit(path,pairs):
    s=open(path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path.split('/')[-2],s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,'w',encoding='utf8').write(s)
edit(RE+'🚢️status/🟦️.ts',[
('import {resolveRemoteCacheV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";','import {remoteCacheFetchV1,resolveRemoteCacheV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";'),
('''token=resolution?.token??context.environment[policy.environment.workerToken];
 const scheduler''','''workerToken=context.environment[policy.environment.workerToken];
 const scheduler'''),
('''headers:Record<string,string>=token?{authorization:`Bearer ${token}`}:{};
 const response=await fetcher(`${scheduler.replace(/\\/+$/u,"")}/v1/fleet/load`,{signal:signal(),headers});''','''headers:Record<string,string>=workerToken?{authorization:`Bearer ${workerToken}`}:{},schedulerCall=remoteCacheFetchV1(environment,{root:context.root,base:fetcher,audience:"execution",roleOf:()=>"submit"}),cacheCall=remoteCacheFetchV1(environment,{root:context.root,base:fetcher});
 const response=await schedulerCall(`${scheduler.replace(/\\/+$/u,"")}/v1/fleet/load`,{signal:signal(),headers});'''),
('const status=await fetcher(`${resolution.contract.endpoint.replace(/\\/+$/u,"")}/status`,{signal:signal(),headers}),body=','const status=await cacheCall(`${resolution.contract.endpoint.replace(/\\/+$/u,"")}/status`,{signal:signal()}),body='),
])
edit(RE+'⌨️command/🟦️.ts',[
('  const started=performance.now(),client=connectGrpcV1(endpoint);','  const started=performance.now(),client=connectGrpcV1(endpoint,{sign:method=>token?{}:fleetHeadersV1(context.root,"submit","execution","POST",`http://x/${method}`)});'),
])
edit(RE+'🚀️offload/🔌️client/🟦️.ts',[
('export async function executeOffloadV1(input:Readonly<{endpoint:string;token:string|undefined;instance:string;','export async function executeOffloadV1(input:Readonly<{endpoint:string;token:string|undefined;sign?:(method:string)=>Record<string,string>;instance:string;'),
('client=connectGrpcV1(input.endpoint),cancel','client=connectGrpcV1(input.endpoint,input.sign===undefined?{}:{sign:input.sign}),cancel'),
('  const worker=String(result.execution_metadata?.worker??""),cached=response.cached_result===true;','  const worker=/^executed by (.+)$/u.exec(String(response.message??""))?.[1]??"",cached=response.cached_result===true;'),
])
edit(RE+'🚀️offload/🟦️.ts',[
('  const token=ports?.token??cache?.token,cacheEndpoint=','  if(!ports?.content&&cache!==undefined&&cache.tokenFrom==="none")return settle(local("no-credential"));\n  const token=ports?.token??cache?.token,cacheEndpoint='),
('httpRemoteExecutionContentV1({endpoint:cacheEndpoint!,token,timeoutSeconds','httpRemoteExecutionContentV1({endpoint:cacheEndpoint!,token,...(workspace===undefined||cache===undefined?{}:{fetch:remoteCacheFetchV1({...environment,...cache.environment,SEMIO_WORKSPACE_ROOT:workspace},{root:workspace})}),timeoutSeconds'),
('execution=await executeOffloadV1({endpoint,token,instance:instance!,','execution=await executeOffloadV1({endpoint,token,...(workspace===undefined||token!==undefined?{}:{sign:method=>fleetHeadersV1(workspace,"submit","execution","POST",`http://x/${method}`)}),instance:instance!,'),
])
print("exec part 3 ok")
