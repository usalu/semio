import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
PATH=LIB+"/🏗️build/🕸️graph/📥️load/🟦️.ts"
def patch(t):
    t=sub1(t,'import {portableInputs,buildGraphMembersV1,','import {readAdmissionMemoV1,writeAdmissionMemoV1} from "./🧠️admission-memo/🟦️.ts";\nimport {portableInputs,buildGraphMembersV1,')
    t=sub1(t,"const inventory=async()=>{let started=performance.now();const captured=selected?await observation.captureObservedSourceInputsV1(",
"const memoPath=scope?join(dirname(path),scope+\".admission.json\"):\"\",zero=\"0\".repeat(64),memoModule={implementation:loadedGraphModuleAuthorityV1.implementation,publication:loadedGraphModuleAuthorityV1.publication};\n  const inventory=async()=>{let started=performance.now();if(selected&&scope&&buildGraphScopeKeyV1(options.selection,readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))))!==scope)return ownerRegistryInvalidationV1;const remembered=selected&&scope?await readAdmissionMemoV1(memoPath,scope,value=>buildGraphProducerAuthorityV1({version:3,module:value.module,membership:zero,graph:zero,observations:value.observations},loadedGraphModuleAuthorityV1),rows=>buildGraphSourceFactsHoldV1(rows,invocation)):undefined;if(selected)timing(\"admission-memo\",started,{hit:remembered!==undefined});started=performance.now();const captured=remembered!==undefined?{value:remembered.admission as unknown,observations:remembered.observations}:selected?await observation.captureObservedSourceInputsV1(")
    t=sub1(t,"candidate=captured?.value;if(isOwnerRegistryInvalidationV1(candidate))return candidate;",
"candidate=captured?.value;if(isOwnerRegistryInvalidationV1(candidate))return candidate;if(remembered===undefined&&selected&&scope&&captured!==undefined){if(invocation)checkScriptInvocation(invocation);try{await writeAdmissionMemoV1(memoPath,{version:1,module:memoModule,scope,admission:candidate as BuildOwnerAdmissionV1,observations:captured.observations},graphSnapshotPolicyV1.maximumSnapshotBytes);}catch(error){if(invocation)checkScriptInvocation(invocation);}timing(\"admission-memo-write\",started);}")
    return t
if __name__=="__main__":
    t=read(PATH);n=patch(t);print(sha(t),"->",sha(n));write(sys.argv[1],n)
