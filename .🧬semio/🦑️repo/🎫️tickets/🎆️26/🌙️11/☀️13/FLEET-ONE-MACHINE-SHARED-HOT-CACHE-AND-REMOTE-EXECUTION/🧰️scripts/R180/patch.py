import sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R180"
def rd(p): return open(p,encoding="utf8").read()
def once(t,old,new):
    assert t.count(old)==1,(t.count(old),old[:80]); return t.replace(old,new)
nx=rd(S+"/base/nx.ts")
nx=once(nx,'import {validateBoundedNxPathsV1} from "../../../📦️artifacts/🔒️bounded/🟦️.ts";','import {validateBoundedNxPathsV1} from "../../../📦️artifacts/🔒️bounded/🟦️.ts";\nimport {admitNestedNxCustodyV1} from "./🤝️nested-custody/🟦️.ts";')
old='const held=cacheScope.getStore();if(held){if(held.workspace!==workspace||held.toolingIdentity!==toolingIdentity||held.environment!==environment||held.graphIdentity!==(environment.SEMIO_BUILD_GRAPH_DIGEST??null)||held.implementationIdentity!==environment.SEMIO_NX_IMPLEMENTATION_IDENTITY||held.mode==="shared"&&mode==="exclusive")throw Error("Original nested cache custody refused");if(!held.live())throw Error("Original Nx cache callback released");checkResourceLeaseControl(held.control);await held.recheck();const value=await withOutputCustodyV1(held,paths,mode,()=>withNxPublicationHeadroomV1(held,paths,mode,body));await held.recheck();return value;}'
new='const open=cacheScope.getStore();if(open){const held=admitNestedNxCustodyV1(open,{workspace,toolingIdentity,environment,mode,paths});if(!held.live())throw Error("Original Nx cache callback released");checkResourceLeaseControl(held.control);await held.recheck();const nested=()=>withOutputCustodyV1(held,paths,mode,()=>withNxPublicationHeadroomV1(held,paths,mode,body));const value=await(held===open?nested():cacheScope.run(held,nested));await held.recheck();return value;}'
nx=once(nx,old,new)
open(S+"/staged-live/nx.ts","w",encoding="utf8").write(nx)
g=rd(S+"/base/graph.ts")
g=once(g,'import {AsyncLocalStorage} from "node:async_hooks";','import {AsyncLocalStorage} from "node:async_hooks";\nimport{nxGraphReleaseFailureV1,type NxGraphReleaseStepFailureV1}from"./🧯️release-failure/🟦️.ts";')
g=once(g,'const failures=[...await untilAbortedV1(readers,entry.control.signal,()=>[entry.control.signal.reason])];','const failures:NxGraphReleaseStepFailureV1[]=[...await untilAbortedV1(readers,entry.control.signal,()=>[entry.control.signal.reason])].map(error=>({step:"readers" as const,error}));')
g=once(g,'try{await child!.close();}catch(error){failures.push(error);}try{lease!.release();}catch(error){failures.push(error);}}if(failures.length>1)throw new AggregateError(failures,"Graph readers and namespace release failed");if(failures.length)throw failures[0];',
 'try{await child!.close();}catch(error){failures.push({step:"child",error});}try{lease!.release();}catch(error){failures.push({step:"lease",error});}}if(failures.length)throw nxGraphReleaseFailureV1("namespace-release",directory,failures);')
g=once(g,'const failures=[...await children.readers.close()];for(const lease of[...children.leases].reverse())try{await lease.release();}catch(error){if(!failures.some(value=>Object.is(value,error)))failures.push(error);}',
 'const failures:NxGraphReleaseStepFailureV1[]=[...await children.readers.close()].map(error=>({step:"readers" as const,error}));for(const lease of[...children.leases].reverse())try{await lease.release();}catch(error){failures.push({step:"lease",error});}')
g=once(g,'if(!primary&&failures.length)throw failures.length===1?failures[0]:new AggregateError(failures,"Graph namespace acquisition and close failed");','if(!primary&&failures.length)throw nxGraphReleaseFailureV1("store-close",base.root,failures);')
g=once(g,'const failures:unknown[]=[];for(const lease of leases)try{await lease.release();}catch(error){failures.push(error);}if(!primary&&failures.length)throw new AggregateError(failures,"Original native graph cleanup failed");',
 'const failures:NxGraphReleaseStepFailureV1[]=[];for(const lease of leases)try{await lease.release();}catch(error){failures.push({step:"lease",error});}if(!primary&&failures.length)throw nxGraphReleaseFailureV1("cleanup",leases.map(lease=>lease.directory).join(", ")||"-",failures);')
open(S+"/staged-live/graph.ts","w",encoding="utf8").write(g)
print("ok")
