from pathlib import Path
p=Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts")
s=p.read_text()
anchor="/** 🏗️ Runs all installers, export fixups and durable ready phases before its first authenticated Source transition. */"
helper='''/** 🪢️ Uses the completed input receiver for every read-only funding check after mutation ownership is consumed. */
function createRepositoryDependencyProducerArtifactPhaseV1(token:Awaited<ReturnType<typeof dependencyMaintenanceNative.captureNativePlatformProtectedArtifactMaintenanceV1>>,control:ResourceLeaseControl,checkpoint:()=>void){
 let receipt:Awaited<ReturnType<typeof dependencyMaintenanceNative.stageNativePlatformProtectedArtifactInputGroupV1>>|undefined;
 return Object.freeze({accept(value:Awaited<ReturnType<typeof dependencyMaintenanceNative.stageNativePlatformProtectedArtifactInputGroupV1>>){checkpoint();if(receipt!==undefined)throw Error("One actual completed producer input receipt required");receipt=value;},recheck(phase:ResourceLeaseControl){checkpoint();return receipt===undefined?dependencyMaintenanceNative.readNativePlatformProtectedArtifactMaintenanceV1(token,phase):dependencyMaintenanceNative.readNativePlatformProtectedArtifactInputGroupV1(receipt,phase);},measure(){checkpoint();return receipt===undefined?dependencyMaintenanceNative.measureNativePlatformProtectedArtifactFundingV1(token,control):dependencyMaintenanceNative.measureNativePlatformProtectedArtifactInputGroupFundingV1(receipt,control);}});
}

'''
assert s.count(anchor)==1
s=s.replace(anchor,helper+anchor)
start=s.index("async function runRepositoryDependencyProducerCompletionV1(")
end=s.index("\n}\n",start)+3
part=s[start:end]
old="token:Awaited<ReturnType<typeof dependencyMaintenanceNative.captureNativePlatformProtectedArtifactMaintenanceV1>>,root:string"
assert old in part
part=part.replace(old,"token:Awaited<ReturnType<typeof dependencyMaintenanceNative.captureNativePlatformProtectedArtifactMaintenanceV1>>,artifactPhase:ReturnType<typeof createRepositoryDependencyProducerArtifactPhaseV1>,root:string",1)
old="stageNativePlatformProtectedArtifactInputGroupV1(token,control,materialization.group);if(resume)"
assert old in part
part=part.replace(old,"stageNativePlatformProtectedArtifactInputGroupV1(token,control,materialization.group);artifactPhase.accept(receipt);if(resume)",1)
old="await dependencyMaintenanceNative.measureNativePlatformProtectedArtifactFundingV1(token,control)"
assert part.count(old)==4,part.count(old)
part=part.replace(old,"await artifactPhase.measure()")
s=s[:start]+part+s[end:]
start=s.index("async function completeRepositoryDependenciesV1(")
end=s.index("\n}\n",start)+3
part=s[start:end]
old='if(gate)requireSelectedToolingLeaseCustodyV1(gate,leaseDirectory,resource,actual);},read=<T>'
new='if(gate)requireSelectedToolingLeaseCustodyV1(gate,leaseDirectory,resource,actual);},artifactPhase=createRepositoryDependencyProducerArtifactPhaseV1(token,actual,checkpoint),read=<T>'
assert old in part
part=part.replace(old,new,1)
old="await dependencyMaintenanceNative.readNativePlatformProtectedArtifactMaintenanceV1(token,phase)"
assert part.count(old)==1
part=part.replace(old,"await artifactPhase.recheck(phase)",1)
old="await dependencyMaintenanceNative.measureNativePlatformProtectedArtifactFundingV1(token,actual)"
assert part.count(old)==1
part=part.replace(old,"await artifactPhase.measure()",1)
old="runRepositoryDependencyProducerCompletionV1(workspace,invocation,actual,parent,token,root"
assert part.count(old)==1
part=part.replace(old,"runRepositoryDependencyProducerCompletionV1(workspace,invocation,actual,parent,token,artifactPhase,root",1)
s=s[:start]+part+s[end:]
p.write_text(s)
