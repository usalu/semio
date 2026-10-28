# [asm-g] wires the owner by owner test discovery into the not-retained branch of discoverBuildGraphPartV1 (after "cutoff-g: retention landed"). usage: python3 wire-owners.py <graph 🟦️.ts>
import sys
path=sys.argv[1];s=open(path,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,("anchors",s.count(old),old[:90])
    s=s.replace(old,new)
rep('import{advanceFrontierV1}from"./🕰️generations/🌿️frontier-walk/🟨️.mjs";','import{advanceFrontierV1}from"./🕰️generations/🌿️frontier-walk/🟨️.mjs";\nimport{discoverTestOwnersV1,selectTestFeaturesV1}from"./🎯️test-owners/🟨️.mjs";')
rep('''    const tagRows:BuildGraphRowV1[]=[],discoveredForTags=async<T>(run:()=>Promise<T>|T):Promise<T>=>{const taken=await observationV1.captureObservedSourceInputsV1(async()=>run(),undefined,false);tagRows.push(...(taken.observations as BuildGraphRowV1[]));return taken.value;};
    const testContributions=retained?[]:await discoveredForTags(()=>tests.discoverTestBuildProjectsV1(files.filter(file=>file.endsWith(".feature")),{},context));''','''    const incremental=!retained&&!part.cold&&part.changed!==undefined&&tagged.some(([,stub])=>stub.tag===TEST_TAG_V1)&&tagged.every(([,stub])=>stub.tag===TEST_TAG_V1||stub.tag===POLICY_TAG_V1)&&env.digest===part.previousEnv;
    const tagRows:BuildGraphRowV1[]=[],testRows:BuildGraphRowV1[]=[],factsInto=(sink:BuildGraphRowV1[])=>async<T>(run:()=>Promise<T>|T):Promise<T>=>{const taken=await observationV1.captureObservedSourceInputsV1(async()=>run(),undefined,false);sink.push(...(taken.observations as BuildGraphRowV1[]));return taken.value;},discoveredForTags=factsInto(tagRows),discoveredForTests=factsInto(testRows);
    const known=new Set(tagged.filter(([,stub])=>stub.tag===TEST_TAG_V1).map(([,stub])=>stub.root)),groups=retained?new Map<string,string[]>():selectTestFeaturesV1(files.filter(file=>file.endsWith(".feature")),{all:!incremental,roots:new Set([...part.roots,...(part.changed??[]).filter(path=>path.endsWith(".feature")).map(path=>dirname(path))]),known});
    const discovery=await discoverTestOwnersV1({tests,capture:run=>observationV1.captureObservedSourceInputsV1(async()=>run(),undefined,false) as never,context,groups}),testContributions=discovery.contributions as [string,any][];''')
rep('''if(!retained)await discoveredForTags(()=>withholdDanglingTestProjectsV1(''','''if(!retained)await discoveredForTests(()=>withholdDanglingTestProjectsV1(''')
rep('''const testEdges:unknown=retained?[]:await discoveredForTags(()=>tests.discoverTestBuildDependenciesV1({},graphContext));''','''const testEdges:unknown=retained?[]:await discoveredForTests(()=>tests.discoverTestBuildDependenciesV1({},graphContext));''')
rep('''...(retained?tagged.filter(([,stub])=>stub.kind==="project"):[])].map(''','''...keptTagged.filter(([,stub])=>stub.kind==="project")].map(''')
rep('''    const edgeNames=!part.cold&&env.digest!==part.previousEnv?kept.map(([name])=>name):[]''','''    const keptTagged=retained?tagged:incremental?tagged.filter(([,stub])=>stub.tag===TEST_TAG_V1&&!groups.has(stub.root)&&!part.roots.has(stub.root)):[],keptNames=new Set(keptTagged.map(([name])=>name));
    const edgeNames=!part.cold&&env.digest!==part.previousEnv?kept.map(([name])=>name):[]''')
rep('''stubs.filter(([,stub])=>(!stub.tag||retained)&&stub.kind==="unavailable"&&!part.roots.has(stub.root))''','''stubs.filter(([name,stub])=>(!stub.tag||keptNames.has(name))&&stub.kind==="unavailable"&&!part.roots.has(stub.root))''')
rep('''nodeRows:tags[name]?tagRows:nodeRows[name]})),...rows.filter''','''nodeRows:tags[name]===TEST_TAG_V1?[...(discovery.rows.get(name)??[]),...testRows]:tags[name]?tagRows:nodeRows[name]})),...rows.filter''')
rep('''unavailable:row,nodeRows:tags[row.project!]?tagRows:nodeRows[row.project!]}))],''','''unavailable:row,nodeRows:tags[row.project!]===TEST_TAG_V1?[...(discovery.rows.get(row.project!)??[]),...testRows]:tags[row.project!]?tagRows:nodeRows[row.project!]}))],''')
rep('''replaceTags:retained?[]:["tests","policy",TEST_TAG_V1,POLICY_TAG_V1],''','''replaceTags:retained?[]:incremental?["policy",POLICY_TAG_V1]:["tests","policy",TEST_TAG_V1,POLICY_TAG_V1],''')
open(path,"w",encoding="utf8").write(s)
print("wired")
