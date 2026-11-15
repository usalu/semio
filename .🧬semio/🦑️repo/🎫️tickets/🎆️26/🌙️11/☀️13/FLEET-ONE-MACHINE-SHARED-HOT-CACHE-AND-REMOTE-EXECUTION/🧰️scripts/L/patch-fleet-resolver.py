B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/"
t=open(B+'🟦️.ts',encoding='utf8').read()
def rept(old,new):
    global t
    assert t.count(old)==1,old[:70]
    t=t.replace(old,new)
rept('import {readRemoteCachePolicyV1,readRemoteCacheTokenV1,resolveRemoteCacheContractV1,type RemoteCacheContractV1,type RemoteCachePolicyV1} from "../🟦️.ts";','import {readRemoteCachePolicyV1,readRemoteCacheTokenV1,resolveRemoteCacheContractV1,type RemoteCacheContractV1,type RemoteCachePolicyV1} from "../🟦️.ts";\nimport {fleetSignedFetchV1,loadFleetSignerV1,syncFleetIdentityV1} from "./🪪️identity/🟦️.ts";')
rept('tokenFrom:"environment"|"fleet-facts"|"none";generation:number|null}>;','tokenFrom:"environment"|"fleet-identity"|"none";generation:number|null}>;')
rept('export type RemoteCacheResolutionOptionsV1=Readonly<{root?:string;copy?:FleetCacheLocalCopyV1;policy?:RemoteCachePolicyV1;preferFleetToken?:boolean}>;','export type RemoteCacheResolutionOptionsV1=Readonly<{root?:string;copy?:FleetCacheLocalCopyV1;policy?:RemoteCachePolicyV1}>;')
rept('''/** 🧭️ THE resolver of every client of the remote cache: the environment first, field by field, then the fleet facts of the local copy. The fleet credential is only ever paired with an endpoint the facts name, so an explicitly foreign endpoint never receives it.''','''/** 🧭️ THE resolver of every client of the remote cache: the environment first, field by field, then the fleet facts of the local copy. The credential of a joined machine is its own signing identity (`tokenFrom: fleet-identity`, no secret in any variable or file that others read) and is only ever used for an endpoint the facts name, so an explicitly foreign endpoint never sees even a signature; an operator token from the environment (`tokenFrom: environment`) is sent as before.''')
rept('''  const named=facts.cache.endpoints.some(endpoint=>origin(endpoint)===origin(overlay[names.endpoint]??""));
  if((own===undefined||options.preferFleetToken===true)&&named)overlay[tokenName]=facts.credential.token;
 }''','''  named=facts.cache.endpoints.some(endpoint=>origin(endpoint)===origin(overlay[names.endpoint]??""));
 }''')
rept('''  let endpointFrom:RemoteCacheResolutionV1["endpointFrom"]="environment";''','''  let endpointFrom:RemoteCacheResolutionV1["endpointFrom"]="environment",named=false;''') if False else None
rept(' let endpointFrom:RemoteCacheResolutionV1["endpointFrom"]="environment";',' let endpointFrom:RemoteCacheResolutionV1["endpointFrom"]="environment",named=false;')
rept('''const token=readRemoteCacheTokenV1(contract,overlay),tokenFrom:RemoteCacheResolutionV1["tokenFrom"]=token===undefined?"none":facts!==undefined&&token===facts.credential.token?"fleet-facts":"environment";''','''const token=readRemoteCacheTokenV1(contract,overlay),tokenFrom:RemoteCacheResolutionV1["tokenFrom"]=token!==undefined?"environment":named&&root!==undefined&&loadFleetSignerV1(root,"cache-read")!==undefined?"fleet-identity":"none";''')
rept('''/** 🔭️ The execution scheduler of the group from the local copy''','''const copies=new Map<string,{at:number;copy:FleetCacheLocalCopyV1|undefined}>();
const facted=(root:string):FleetCacheLocalCopyV1|undefined=>{const known=copies.get(root),now=Date.now();if(known!==undefined&&now-known.at<5000)return known.copy;const copy=readLocalFleetCacheFactsV1(root);copies.set(root,{at:now,copy});return copy;};

/** 🪝️ The `fetch` every client of the remote cache uses: it adds what the resolved credential needs. An operator token of the environment becomes a bearer header. Otherwise, for an origin that the fleet facts of this machine name (the cache endpoints and the scheduler), every request is signed with the machine's role key (`cache-read` for reads, `cache-write` for writes, `submit` or `worker` as `roleOf` says), so no secret is ever sent; any other URL is requested as it is. */
export function remoteCacheFetchV1(environment:Readonly<Record<string,string|undefined>>,options:Readonly<{root?:string;base?:typeof fetch;audience?:"cache"|"execution";roleOf?:(method:string,pathname:string)=>import("./🪪️identity/🟦️.ts").FleetRoleV1}>={}):typeof fetch{
 const policy=readRemoteCachePolicyV1(),root=options.root??fleetCacheRootV1(environment),base=options.base??fetch,tokenName=filled(environment,policy.environment.tokenEnvironment)??policy.defaults.tokenEnvironment,token=filled(environment,tokenName),audience=options.audience??"cache";
 const signed=root===undefined?undefined:fleetSignedFetchV1({root,audience,base:undefined as never,...(options.roleOf===undefined?{}:{roleOf:options.roleOf}),fetch:base});
 const named=(url:string):boolean=>{if(root===undefined)return false;const facts=facted(root)?.facts;if(facts===undefined)return false;const where=origin(url);return facts.cache.endpoints.some(endpoint=>origin(endpoint)===where)||(facts.execution!==undefined&&origin(facts.execution.endpoint)===where)||(facts.execution!==undefined&&sameHost(facts.execution.endpoint,url));};
 return ((input:RequestInfo|URL,init?:RequestInit)=>{
  const url=typeof input==="string"||input instanceof URL?String(input):input.url;
  if(token!==undefined&&!(named(url)&&signed!==undefined&&loadFleetSignerV1(root!,"cache-read")!==undefined)){const headers=new Headers(init?.headers);headers.set("authorization",`Bearer ${token}`);return base(input,{...init,headers});}
  return signed!==undefined&&named(url)?signed(input,init):base(input,init);
 }) as typeof fetch;
}
const sameHost=(left:string,right:string):boolean=>{try{return new URL(left).hostname===new URL(right).hostname;}catch{return false;}};

/** 🔭️ The execution scheduler of the group from the local copy''')
rept('''  const storeRoot=fleetStoreRootV1(options.root);
  if(storeRoot===undefined)throw Error("no store root in .cache/f/machine.json");''','''  const storeRoot=fleetStoreRootV1(options.root);
  if(storeRoot===undefined)throw Error("no store root in .cache/f/machine.json");
  await syncFleetIdentityV1({root:options.root,...(options.timeoutMilliseconds===undefined?{}:{timeoutMilliseconds:options.timeoutMilliseconds})});''')
rept('''   const response=await ask(`${resolution.contract.endpoint}/${resolution.contract.instance}/v1/cache/semio-probe`,{signal:AbortSignal.timeout(options.probeMilliseconds??1500),headers:resolution.token===undefined?{}:{authorization:`Bearer ${resolution.token}`}});''','''   const response=await remoteCacheFetchV1({...environment,...resolution.environment},{...(root===undefined?{}:{root}),base:ask})(`${resolution.contract.endpoint}/${resolution.contract.instance}/v1/cache/semio-probe`,{signal:AbortSignal.timeout(options.probeMilliseconds??1500)});''')
rept('''  const again=resolveRemoteCacheV1(environment,{root,preferFleetToken:true});
  if(again!==undefined&&(again.contract.endpoint!==resolution.contract.endpoint||again.token!==resolution.token)){resolution=again;status=await probe(again);}''','''  const again=resolveRemoteCacheV1(environment,{root});
  if(again!==undefined&&(again.contract.endpoint!==resolution.contract.endpoint||again.token!==resolution.token||again.tokenFrom!==resolution.tokenFrom)){resolution=again;status=await probe(again);}
  else if(status===401&&again?.tokenFrom==="fleet-identity"){await new Promise(done=>setTimeout(done,Math.min(6000,options.probeMilliseconds??1500)));status=await probe(again);}''')
rept('/** 🔐️ Resolves the remote cache the way a build needs it: refresh the facts (TTL, timeout), resolve, probe the Nx archive route once. A 401 or an unreachable endpoint with fleet facts at hand forces a refresh (the credential may have rotated, another endpoint may answer) and tries once more, with the fleet credential in place of an explicit token that the server rejected.','/** 🔐️ Resolves the remote cache the way a build needs it: refresh the facts (TTL, timeout; this also registers the identity of the machine in the group store), resolve, probe the Nx archive route once. A 401 or an unreachable endpoint with fleet facts at hand forces a refresh (a new identity may not be known to the server yet, another endpoint may answer) and tries once more.')
open(B+'🟦️.ts','w',encoding='utf8').write(t)
print("resolver ok")
