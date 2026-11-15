LIB="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(path,pairs):
    s=open(path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path.split('/')[-2],s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,'w',encoding='utf8').write(s)
edit(LIB+'🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts',[
('const comparable=','''/** 🌐️ The CIDR network of the fleet this machine belongs to: `SEMIO_FLEET_NETWORK`, else the worker network of the topology row of its group. */
export async function fleetNetworkV1(root:string,environment:Readonly<Record<string,string|undefined>>):Promise<string|undefined>{
 const named=environment.SEMIO_FLEET_NETWORK;
 if(named!==undefined&&named!=="")return named;
 try{
  const group=(JSON.parse(await readFile(join(root,".cache","f","machine.json"),"utf8")) as {groupId?:string}).groupId;
  return((JSON.parse(await readFile(join(root,TOPOLOGY),"utf8")) as {groups?:{id?:string;server?:{network?:string}}[]}).groups??[]).find(row=>row.id===group)?.server?.network;
 }catch{return undefined;}
}

const comparable='''),
])
edit(LIB+'🌐️remote-execution/🛰️sidecar/🟦️.ts',[
('import {setFleetExecutionV1} from "../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts";\nimport {readFleetExecutionV1,refreshFleetCacheFactsV1,resolveRemoteCacheV1,fleetStoreRootV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";','import {fleetNetworkV1,setFleetExecutionV1} from "../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts";\nimport {readFleetExecutionV1,refreshFleetCacheFactsV1,remoteCacheFetchV1,resolveRemoteCacheV1,fleetStoreRootV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";\nimport {createFleetRegistryV1,createFleetVerifierV1,fleetIdentityDirectoryV1,fleetInterfaceAddressV1,readFleetMachineV1,syncFleetIdentityV1} from "../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";\nimport {createFleetGateV1} from "../../🌐️remote-cache/🛰️fleet/🪪️identity/🛂️gate/🟦️.ts";'),
(' const policy=readRemoteExecutionPolicyV1(),environment={...context.environment,SEMIO_WORKSPACE_ROOT:context.root},say=context.write;\n while(!context.signal.aborted){',' const policy=readRemoteExecutionPolicyV1(),environment={...context.environment,SEMIO_WORKSPACE_ROOT:context.root},say=context.write;\n await syncFleetIdentityV1({root:context.root}).then(result=>{if(result.error!==undefined)say(`fleet identity: ${result.error}`);},()=>undefined);\n while(!context.signal.aborted){'),
('  const resolution=resolveRemoteCacheV1(environment,{root:context.root}),token=resolution?.token??context.environment[policy.environment.workerToken];','  const token=context.environment[policy.environment.workerToken],schedulerCall=remoteCacheFetchV1(environment,{root:context.root,audience:"execution",roleOf:()=>"worker"}),cacheCall=remoteCacheFetchV1(environment,{root:context.root});'),
('''    if(next!==undefined&&next!==endpoint){say("the scheduler moved, joining the new one");controller.abort();}
    else if(resolveRemoteCacheV1(environment,{root:context.root})?.token!==token&&token!==undefined){say("the fleet credential rotated, joining again with the new one");controller.abort();}''','''    if(next!==undefined&&next!==endpoint){say("the scheduler moved, joining the new one");controller.abort();}'''),
('root:context.root,environment,scheduler,token,cacheToken:token,concurrency:','root:context.root,environment,scheduler,token,cacheToken:token,fetch:schedulerCall,cacheFetch:cacheCall,actions:policy.worker.actions as "rustc"|"any"|"none",concurrency:'),
('with the fleet credential as bearer token, publishes its endpoint','with signature-checked requests of the group\'s origins (no shared secret), bound to the fleet interface only, publishes its endpoint'),
(''' const resolution=resolveRemoteCacheV1(environment,{root:context.root});
 if(resolution?.token===undefined)throw Error("no fleet credential for the scheduler yet");
 const token=resolution.token,cacheUrl=resolution.contract.endpoint,storeRoot=context.environment.SEMIO_FLEET_STORE_ROOT??fleetStoreRootV1(context.root);''',''' const resolution=resolveRemoteCacheV1(environment,{root:context.root}),machine=readFleetMachineV1(context.root);
 if(resolution===undefined||machine===undefined)throw Error("no fleet facts for the scheduler yet");
 await syncFleetIdentityV1({root:context.root});
 const cacheUrl=resolution.contract.endpoint,storeRoot=context.environment.SEMIO_FLEET_STORE_ROOT??fleetStoreRootV1(context.root),network=await fleetNetworkV1(context.root,context.environment),address=network===undefined?undefined:fleetInterfaceAddressV1(network);
 if(storeRoot===undefined||network===undefined||address===undefined)throw Error("the scheduler needs the group store and an address of this machine inside the fleet network (SEMIO_FLEET_NETWORK)");
 const registry=createFleetRegistryV1({storeRoot,group:machine.group,pinsPath:join(fleetIdentityDirectoryV1(context.root),"pins-execution.json")}),verifier=createFleetVerifierV1({registry,audience:"execution",adminOrigin:`${machine.group}-server`}),workerToken=context.environment[policy.environment.workerToken];
 await registry.refresh(true);
 const gate=createFleetGateV1({verifier,networks:[network],tokens:{worker:workerToken}});'''),
('  host:"0.0.0.0",grpcPort:ports.grpc,workerPort:ports.worker,content:httpRemoteExecutionContentV1({endpoint:cacheUrl,token}),cacheUrl,tokens:{read:token,write:token,worker:token},advertise:shown,','  host:address,grpcPort:ports.grpc,workerPort:ports.worker,content:httpRemoteExecutionContentV1({endpoint:cacheUrl,fetch:remoteCacheFetchV1(environment,{root:context.root})}),cacheUrl,tokens:{},gate,maximumWorkers:server.maximumWorkers,maximumRequestBytes:server.maximumRequestBytes,advertise:shown,'),
('defaultActionSeconds:server.defaultActionSeconds,maximumQueued:server.maximumQueued,','defaultActionSeconds:server.defaultActionSeconds,maximumActionSeconds:server.maximumActionSeconds,maximumQueued:server.maximumQueued,'),
('''   const again=resolveRemoteCacheV1(environment,{root:context.root});
   if(again?.token!==undefined&&again.token!==token){say("the fleet credential rotated, restarting with the new one");break;}
   await publish();''','''   await publish();'''),
])
print("sidecar ok")
