# [asm-g] compile-atomic patch of discoverBuildGraphPartV1: per-owner incremental test discovery. usage: python3 patch_part.py <graph 🟦️.ts>
import sys
path=sys.argv[1];s=open(path,encoding="utf8").read()
def once(old,new,what):
    global s
    assert s.count(old)==1,what+" anchors="+str(s.count(old))
    s=s.replace(old,new)
# 1. block between the primary inference and the edge names
start=s.index('    const testContributions=await tests.discoverTestBuildProjectsV1(')
end=s.index('    const edgeNames=!part.cold&&env.digest!==part.previousEnv')
old_block=s[start:end]
new_block='''    const projects:Record<string,any>={},externalNodes:Record<string,unknown>={},unavailable=new Map<string,BuildUnavailableProjectV1>(),tags:Record<string,string>={},configs:Record<string,string>={},primaryNames=new Set<string>(),testNames=new Set<string>();
    const merge=(contributions:readonly [string,any][],accept:(name:string)=>void)=>{for(const[file,contribution]of contributions){
      Object.assign(externalNodes,contribution.externalNodes);
      for(const row of (contribution.unavailable??[]) as BuildUnavailableProjectV1[])unavailable.set(row.root,row);
      for(const[name,project]of Object.entries(contribution.projects??{}) as [string,any][]){
        if(projects[name]&&projects[name].root!==project.root)throw Error(`Duplicate build project ${name}`);
        projects[name]={...projects[name],...project,name,targets:{...projects[name]?.targets,...project.targets}};
        accept(name);configs[name]??=file;
      }
    }};
    merge(primary,name=>{primaryNames.add(name);});
    const kept=stubs.filter(([name,stub])=>!stub.tag&&stub.kind==="project"&&!projects[name]&&!part.roots.has(stub.root));
    const listed=[...kept.map(([name,stub])=>({name,root:stub.root,nativeRoot:stub.nativeRoot,config:stub.config})),...Object.entries(projects).map(([name,project])=>({name,root:project.root,nativeRoot:project.metadata?.nativeRoot as string|undefined,config:configs[name]}))];
    const env=await deriveEnvironmentV1({projects:listed,workspaceManifests:files.filter(file=>file==="Cargo.toml"||file.endsWith("/Cargo.toml")),lock:javascriptManagerV1.primaryLockName(),taxonomy:"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",read,stat});
    const allTests=part.cold||part.envStale||env.digest!==part.previousEnv,known=new Set(stubs.filter(([,stub])=>stub.tag==="tests").map(([,stub])=>stub.root));
    const discovery=await discoverTestOwnersV1(tests,context,selectTestFeaturesV1(files.filter(file=>file.endsWith(".feature")),{all:allTests,roots:part.roots,known}));
    const testContributions=discovery.contributions,policyOwners=Object.assign({},...[...primary,...testContributions].map(([,contribution])=>contribution.projects??{}));
    const policyContributions=await policies.default.createNodesV2[1](files.filter(file=>file.endsWith("📜️script.ts")&&(part.cold||part.roots.has(dirname(file))||!realRoots.has(dirname(file)))),{}, {workspaceRoot:root,projects:policyOwners});
    merge(testContributions,name=>{testNames.add(name);});
    merge(policyContributions,()=>undefined);
    for(const name of Object.keys(projects))if(!primaryNames.has(name))tags[name]=testNames.has(name)?"tests":"policy";
    for(const[name,rows]of discovery.rows)nodeRows[name]=rows;
    withholdDanglingTestProjectsV1({projects,configs,unavailable,dangling:tests.danglingTestDependenciesV1({...Object.fromEntries(kept.map(([name,stub])=>[name,{root:stub.root}])),...projects}),digest:file=>observedFileDigestV1(root,file)});
    const keptTests=allTests?[]:stubs.filter(([name,stub])=>stub.tag==="tests"&&!part.roots.has(stub.root)&&!projects[name]&&!unavailable.has(stub.root));
'''
s=s[:start]+new_block+s[end:]
# 2. context projects and withheld roots include the kept test owners
once('''const contextProjects:Record<string,any>={...Object.fromEntries(kept.map(([name,stub])=>[name,{name,root:stub.root,targets:Object.fromEntries(stub.targets.map(target=>[target,{}])),...(stub.nativeRoot?{metadata:{nativeRoot:stub.nativeRoot}}:{})}])),...projects};''',
'''const contextProjects:Record<string,any>={...Object.fromEntries([...kept,...keptTests.filter(([,stub])=>stub.kind==="project")].map(([name,stub])=>[name,{name,root:stub.root,targets:Object.fromEntries(stub.targets.map(target=>[target,{}])),...(stub.nativeRoot?{metadata:{nativeRoot:stub.nativeRoot}}:{})}])),...projects};''',"context")
once('''...stubs.filter(([,stub])=>!stub.tag&&stub.kind==="unavailable"&&!part.roots.has(stub.root)).map(([,stub])=>stub.root)],projectFileMap''','''...stubs.filter(([,stub])=>!stub.tag&&stub.kind==="unavailable"&&!part.roots.has(stub.root)).map(([,stub])=>stub.root),...keptTests.filter(([,stub])=>stub.kind==="unavailable").map(([,stub])=>stub.root)],projectFileMap''',"withheld")
once('replaceTags:["tests","policy"],','replaceTags:allTests?["tests","policy"]:["policy"],',"replaceTags")
# 3. signature
once('part:{cold:boolean;roots:ReadonlySet<string>;stubs:Record<string,BuildGraphPartStubV1>;previousEnv:string|null}):Promise<BuildGraphPartV1> {','part:{cold:boolean;roots:ReadonlySet<string>;stubs:Record<string,BuildGraphPartStubV1>;previousEnv:string|null;envStale:boolean}):Promise<BuildGraphPartV1> {',"signature")
once('discoverPart:(spec:{cold:boolean;owners:string[];discover:string[];removed:string[];previousEnv:string|null;stubs:Record<string,BuildGraphPartStubV1>}','discoverPart:(spec:{cold:boolean;owners:string[];discover:string[];removed:string[];previousEnv:string|null;envStale:boolean;stubs:Record<string,BuildGraphPartStubV1>}',"hostspec")
once('previousEnv:spec.previousEnv});},','previousEnv:spec.previousEnv,envStale:spec.envStale});},',"hostcall")
# 4. helpers before the Part function
marker='/** 🧩️ Infers only the projects whose roots are in `part.roots`'
helpers='''/** 🧪️ Selects the feature files whose test owners one partial inference discovers again, grouped by the `🧪️tests` directory that owns them (`features/<case>/<feature>` below it): all of them when `all` (cold build, moved environment), otherwise an owner already published (`known` holds the roots of the test owners of the generation) when it is stale itself (its root is a stale root) or a stale root lies below it (a case directory that gained a feature), and an owner not yet published when a stale root is, lies below or contains its directory (a first case below a project that changed). */
export function selectTestFeaturesV1(features:readonly string[],scope:{readonly all:boolean;readonly roots:ReadonlySet<string>;readonly known:ReadonlySet<string>}):Map<string,string[]> {
  const parent=(path:string)=>path.slice(0,Math.max(0,path.lastIndexOf("/"))),roots=[...scope.roots],groups=new Map<string,string[]>(),decided=new Map<string,boolean>();
  for(const feature of features){
    const owner=parent(parent(feature));let chosen=decided.get(owner);
    if(chosen===undefined){chosen=scope.all||(scope.known.has(owner)?scope.roots.has(owner)||roots.some(stale=>stale.startsWith(owner+"/")):roots.some(stale=>stale==="."||owner===stale||owner.startsWith(stale+"/")||stale.startsWith(owner+"/")));decided.set(owner,chosen);}
    if(!chosen)continue;
    const list=groups.get(owner);if(list)list.push(feature);else groups.set(owner,[feature]);
  }
  return groups;
}

/** 🧪️ Discovers the test owners of `groups` one by one, each inside its own observation capture, and answers the complete facts of every project: what all owners share (the command closure of the testing domain, captured once) and what its own discovery read, so a later edit finds exactly the owners that consumed it. */
async function discoverTestOwnersV1(tests:any,context:object,groups:ReadonlyMap<string,readonly string[]>):Promise<{contributions:[string,any][];rows:Map<string,BuildGraphRowV1[]>}> {
  const contributions:[string,any][]=[],rows=new Map<string,BuildGraphRowV1[]>();
  if(!groups.size)return{contributions,rows};
  const shared=await observationV1.captureObservedSourceInputsV1(()=>tests.discoverTestSharedV1(context),undefined,false);
  for(const[,features]of [...groups].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b)))){
    const one=await observationV1.captureObservedSourceInputsV1(()=>tests.discoverTestBuildProjectsV1(features,{},{...context,testShared:shared.value}),undefined,false);
    for(const entry of one.value as [string,any][]){contributions.push(entry);const facts=[...shared.observations,...one.observations] as BuildGraphRowV1[];for(const name of Object.keys(entry[1].projects??{}))rows.set(name,facts);}
  }
  return{contributions,rows};
}

'''
once(marker,helpers+marker,"helpers")
open(path,"w",encoding="utf8").write(s)
print("patched")
