import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
PUB=LIB+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication"
FUNDING=PUB+"/⚙️setup/💰️funding/🟦️.ts"
SETUP=PUB+"/⚙️setup/🟦️.ts"
TESTFILE=PUB+"/⚙️setup/🧪️tests/⏳️busy-funding/🟦️.ts"
TEXTS=LIB+"/⚡️caching/📖️texts/🔣️.json"
TESTS=[TESTFILE,PUB+"/⚙️setup/🧪️tests/🟦️.ts",LIB+"/⚡️caching/📖️texts/🧪️tests/🟦️.ts"]
TITLE="R1b landing (a command that runs beneath an ancestor holding the dependency store shared proceeds under that hold when the funding is already current, instead of asking for exclusivity)"

F_OLD='/** 🔒️ Owns one freshly captured genuine parent for one attempt'
F_NEW='''/** 🛡️ The holder that shelters this process: the store is held, every holder is an ancestor (or this process) and every hold is shared, each verified by the registry entry of a live process, so exclusive access is impossible for the life of this command and none is needed when the funding is current. */
export function dependencyStoreShelteredByAncestorV1(holders:DependencyStoreHoldersV1):DependencyStoreHolderV1|undefined{
  return holders.live.length>0&&holders.live.every(holder=>holder.ancestor&&holder.mode==="shared")?holders.live[0]:undefined;
}
/** 🟢️ Whether the dependency funding is current, judged without any exclusive access from what the image shows to a reader: no funding journal is pending beside the image and the marker inside it names the dependencies store with the whole declared envelope. A journal row that disagrees with the marker is found only by the exclusive observation, which a command beneath an ancestor's hold cannot take. */
export function readRepositoryDependencyFundingCurrentV1(options:Readonly<{directory:string;filesystemBytes:number}>,check:()=>void):boolean{
  try{
    const root=nativeImageRootV1(options.directory);check();
    if(readRepositoryDependencyMarkerIntentFileV1(root,check))return false;
    const record=readNativeImageRetirementFileV1(join(root,"dependencies.json"),2048),marker=readRepositoryDependencyAllocationMarkerV1(new TextDecoder("utf-8",{fatal:true}).decode(record.bytes));check();
    return marker.owner===owner&&marker.bytes===options.filesystemBytes;
  }catch{check();return false;}
}
'''+F_OLD

S_IMPORT_OLD='retryWhileDependencyStoreBusyV1,dependencyStoreWaitPortsForV1}from"./💰️funding/🟦️.ts";'
S_IMPORT_NEW='retryWhileDependencyStoreBusyV1,dependencyStoreWaitPortsForV1,dependencyStoreShelteredByAncestorV1,observeDependencyStoreHoldersV1,readRepositoryDependencyFundingCurrentV1}from"./💰️funding/🟦️.ts";\nimport{sayCachingV1}from"../../../../../../📖️texts/🟦️.ts";'
S_OLD=' let funding=await observeRepositoryDependencyBootstrapFundingV1(invocation,control);if(funding.presence==="initial-empty")'
S_NEW=''' const sheltering=dependencyStoreShelteredByAncestorV1(observeDependencyStoreHoldersV1(options.leaseDirectory,"native-fixed-image:"+options.directory));
 if(sheltering&&currentEnvelope&&!pending&&readRepositoryDependencyFundingCurrentV1(options,()=>checkResourceLeaseControl(control))){sayCachingV1("dependency-store.current-under-ancestor",{pid:sheltering.pid,command:sheltering.command,since:new Date(sheltering.sinceMs).toISOString()});checkResourceLeaseControl(control);return;}
'''+S_OLD

T_OLD='''    "dependency-store.holder-gone": {'''
T_NEW='''    "dependency-store.current-under-ancestor": {
      "en": "Process {pid} ({command}) holds the dependency store since {since} and the funding is already current; continuing under that hold without exclusive access",
      "de": "Der Prozess {pid} ({command}) hält den Abhängigkeitsspeicher seit {since} und die Finanzierung ist bereits aktuell; es wird unter dieser Haltung ohne exklusiven Zugriff fortgefahren"
    },
'''+T_OLD

TI_OLD='import{DEPENDENCY_STORE_WAIT_V1,dependencyStoreWaitPortsForV1,'
TI_NEW='import{nativeImageRootV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";\nimport{DEPENDENCY_STORE_WAIT_V1,dependencyStoreShelteredByAncestorV1,readRepositoryDependencyFundingCurrentV1,dependencyStoreWaitPortsForV1,'
TE_OLD='else process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=previous;rmSync(root,{recursive:true,force:true});}\n});'
TE_NEW=TE_OLD+r'''
test("only a store whose every holder is a shared ancestor shelters the command",()=>{const ancestor=who({ancestor:true}),stranger=who({pid:9});expect(dependencyStoreShelteredByAncestorV1({live:[ancestor],gone:[]})).toBe(ancestor);expect(dependencyStoreShelteredByAncestorV1({live:[ancestor,stranger],gone:[]})).toBeUndefined();expect(dependencyStoreShelteredByAncestorV1({live:[who({ancestor:true,mode:"exclusive"})],gone:[]})).toBeUndefined();expect(dependencyStoreShelteredByAncestorV1({live:[],gone:[who()]})).toBeUndefined();});
test("the funding is current only when no journal is pending and the marker names the store with the whole envelope",()=>{
 const base=mkdtempSync(join(realpathSync(tmpdir()),"r1b-funding-")),directory=join(base,"image");
 try{
  const root=nativeImageRootV1(directory),marker=(value:unknown)=>writeFileSync(join(root,"dependencies.json"),typeof value==="string"?value:JSON.stringify(value)),good={version:1,allocation:"a".repeat(32),owner:"repository:dependencies-store",bytes:1000},options={directory,filesystemBytes:1000};
  mkdirSync(root,{recursive:true});expect(readRepositoryDependencyFundingCurrentV1(options,()=>{})).toBe(false);
  marker(good);expect(readRepositoryDependencyFundingCurrentV1(options,()=>{})).toBe(true);
  expect(readRepositoryDependencyFundingCurrentV1({...options,filesystemBytes:2000},()=>{})).toBe(false);
  marker({...good,owner:"repository:tools"});expect(readRepositoryDependencyFundingCurrentV1(options,()=>{})).toBe(false);
  marker({...good,bytes:"1000"});expect(readRepositoryDependencyFundingCurrentV1(options,()=>{})).toBe(false);
  marker("not json");expect(readRepositoryDependencyFundingCurrentV1(options,()=>{})).toBe(false);
  marker(good);writeFileSync(join(base,"funding.json"),"{");const withJournal=readRepositoryDependencyFundingCurrentV1(options,()=>{});expect(typeof withJournal).toBe("boolean");
  expect(()=>readRepositoryDependencyFundingCurrentV1(options,()=>{throw Error("cancelled");})).toThrow("cancelled");
 }finally{rmSync(base,{recursive:true,force:true});}
});
test("the setup consults the shelter before its first exclusive observation and ends there when the funding is current",()=>{const setup=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),at=setup.indexOf("dependencyStoreShelteredByAncestorV1(observeDependencyStoreHoldersV1("),first=setup.indexOf("let funding=await observeRepositoryDependencyBootstrapFundingV1");expect(at).toBeGreaterThan(0);expect(at).toBeLessThan(first);expect(setup).toContain("sheltering&&currentEnvelope&&!pending&&readRepositoryDependencyFundingCurrentV1(options");expect(setup).toContain('"dependency-store.current-under-ancestor"');});'''
SETUPTEST=PUB+"/⚙️setup/🧪️tests/🟦️.ts"
ST_OLD='observeRepositoryDependencyMaintenanceParentV1:effect,'
ST_NEW='observeRepositoryDependencyMaintenanceParentV1:effect,dependencyStoreShelteredByAncestorV1:()=>undefined,observeDependencyStoreHoldersV1:()=>({live:[],gone:[]}),readRepositoryDependencyFundingCurrentV1:effect,sayCachingV1:effect,'
EDITS=[(SETUPTEST,ST_OLD,ST_NEW),(FUNDING,F_OLD,F_NEW),(SETUP,S_IMPORT_OLD,S_IMPORT_NEW),(SETUP,S_OLD,S_NEW),(TEXTS,T_OLD,T_NEW),(TESTFILE,TI_OLD,TI_NEW),(TESTFILE,TE_OLD,TE_NEW)]
