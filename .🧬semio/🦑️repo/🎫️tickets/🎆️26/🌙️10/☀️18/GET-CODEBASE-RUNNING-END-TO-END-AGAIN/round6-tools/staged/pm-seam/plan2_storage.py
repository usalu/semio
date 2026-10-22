"""🧱️ Round 2, concern 3: the JavaScript dependency driver reads every manager-specific name from the materialized schema (data) or from the manager that rides on it, never from a literal."""
def declare(edit, paths):
    D = paths.DEPENDENCIES
    S = D + "/💾️storage/🟦️.ts"
    M = "installSchema.$defs.ManagerProtocol"
    edit(S, "driver-imports", "storage2",
         'import {currentHostV1,selectPackageManagerV1} from "../' + "🔌️managers/📚️catalog/🟦️.ts" + '";\nimport {parseJsoncV1} from "../🔌️managers/📖️formats/🟦️.ts";\nconst packageManagerV1=selectPackageManagerV1("javascript"),lockNameV1=packageManagerV1.primaryLockName(),scopeConfigV1=packageManagerV1.configurationNames("scope")[0]!,generationDirectoryV1=packageManagerV1.generationDirectory(),packageManagerRuntimeV1=()=>packageManagerV1.runtimeIdentity(currentHostV1()),readLockDocumentV1=(text:string):any=>parseJsoncV1(text);\n/** 📁️ Names the store directory every generation of the selected JavaScript manager lives in. */\nexport const javascriptDependencyGenerationDirectoryV1=generationDirectoryV1;',
         '/** 📁️ Names the store directory every generation of the selected JavaScript manager lives in. */\nexport const javascriptDependencyGenerationDirectoryV1:string=' + M + '.const.generationDirectory;')
    edit(S, "driver-lock-name", "storage2", "lockNameV1", M + ".const.lockFile", 17)
    edit(S, "driver-scope-configuration", "storage2", "scopeConfigV1", M + ".const.scopeConfig", 3)
    edit(S, "driver-manager", "storage2", "packageManagerV1.", M + ".manager.", 6)
    edit(S, "driver-runtime-key", "storage2", "sourceRoot,packageManagerRuntimeV1(),process.platform,process.arch", "sourceRoot," + M + ".const.runtime,process.platform,process.arch")
    edit(S, "driver-runtime-tier", "storage2", "protocol.ambientFallback,process.versions.bun,process.versions.node", "protocol.ambientFallback," + M + ".const.runtime,process.versions.node")
    edit(S, "driver-lock-project", "storage2",
         'const project=(lock:any,reverse:boolean)=>{const links=reverse?new Map([...mapping].map(([source,target])=>[target,source])):mapping;if(lock.patchedDependencies)lock.patchedDependencies=Object.fromEntries([...patchIdentities].map(([name,row])=>[name,reverse?posix.relative(scope,row.source):row.target]));lock.workspaces=Object.fromEntries(Object.entries(lock.workspaces??{}).filter(([path])=>reverse||path===""||links.has(path)).map(([path,value])=>[links.get(path)??path,value]));for(const[key,row]of Object.entries(lock.packages??{}) as [string,any][]){if(!Array.isArray(row)||typeof row[0]!=="string")continue;const marker=row[0].indexOf("@workspace:");if(marker<0)continue;const path=row[0].slice(marker+11),target=links.get(path);if(!target){if(reverse)throw Error("Locked workspace package lacks original admitted Source");delete lock.packages[key];continue;}row[0]=row[0].slice(0,marker+11)+target;}return JSON.stringify(lock,null,2)+"\\n";};',
         'const project=(lock:string,reverse:boolean)=>' + M + '.manager.projectLock(lock,{reverse,mapping,patches:[...patchIdentities].map(([name,row])=>({name,forward:row.target,reverse:posix.relative(scope,row.source)}))});')
    edit(S, "driver-lock-project-forward", "storage2", "project(readLockDocumentV1(lockBytes.toString()),false)", "project(lockBytes.toString(),false)")
    edit(S, "driver-lock-project-reverse", "storage2", "project(readLockDocumentV1(bytes),true)", "project(bytes,true)")
    edit(S, "driver-lock-document", "storage2", "readLockDocumentV1(projected)", M + ".manager.parseLockDocument(projected)")
    edit(S, "driver-generation-directory", "storage2", '"j"', M + ".const.generationDirectory", 36)
    edit(S, "driver-generation-root", "storage2", 'roots:["j/"+plan.generation.slice(0,20)]', "roots:[" + M + '.const.generationDirectory+"/"+plan.generation.slice(0,20)]')
    edit(S, "driver-profiles", "storage2", 'for(const profile of["bun-import","bun-require"]as const){', "for(const profile of " + M + ".manager.resolutionProfiles()){")
    edit(S, "driver-strict-profile", "storage2", 'profile.startsWith("bun-")', "(strict=>strict!==\"\"&&profile.startsWith(strict))(" + M + ".const.strictProfilePrefix)")
    edit(D + "/🧾️generation/💾️storage/🟦️.ts", "generation-runtime", "generation2", "bun:String(process.versions.bun)", "manager:String(" + M + ".const.runtime)")
    edit(D + "/🧾️generation/🟦️.ts", "generation-runtime-type", "generation2", "readonly bun:string;readonly revision:string", "readonly manager:string;readonly revision:string")
    edit(D + "/🧾️generation/🟦️.ts", "generation-runtime-compare", "generation2", "left.bun===right.bun", "left.manager===right.manager")
    edit(D + "/🧪️tests/🟦️.ts", "driver-bundle-prelude", "storage2",
         '";const packageManagerV1=selectPackageManagerV1(\\"javascript\\");const installSchema="+JSON.stringify(dependencyInstallSchema)+";"+operation.getText(source)',
         '";const installSchema="+JSON.stringify(dependencyInstallSchema)+";Object.defineProperty(installSchema.$defs.ManagerProtocol,\\"manager\\",{value:selectPackageManagerV1(\\"javascript\\")});"+operation.getText(source)')
