import sys
C=sys.argv[1]
p=C+"/bzl-incr-tools/src/test.ts"
s=open(p,encoding="utf8").read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
rep('const hosted=(files:Record<string,string>)=>{','''const scenario=set.scenarios.find(row=>row.id==="fw-ts-comment")!;
const scenarioSlice=slice({...fixture.evaluation[0]!.slice,tasks:{...fixture.evaluation[0]!.slice.tasks,"lib:generate":{...fixture.evaluation[0]!.slice.tasks["lib:generate"]!,inputs:[scenario.mutation.file]}}});
const edited=incrementalExpectedV1(scenarioSlice,"bazel",{kind:scenario.kind,files:[scenario.mutation.file]},set.sensitivity);
const hosted=(files:Record<string,string>)=>{''')
rep('sliceValue:IncrementalSliceV1=slice(fixture.evaluation[0]!.slice)','sliceValue:IncrementalSliceV1=scenarioSlice')
rep('''const scenario=set.scenarios.find(row=>row.id==="fw-ts-comment")!;
const sliceCase=fixture.evaluation[0]!;
const exact=(tool:IncrementalToolV1):IncrementalObservationV1=>({tool,executed:[...sliceCase.expected.must],restored:[],cached:[],unmapped:[]});
''','''const exact=(tool:IncrementalToolV1):IncrementalObservationV1=>({tool,executed:[...edited.must],restored:[],cached:[],unmapped:[]});
''')
rep('call===3?{tool:"bazel",executed:["app:build"],restored:[],cached:[],unmapped:[]}:exact("bazel")','call===3?{tool:"bazel",executed:["app:build","zzz:extra"],restored:[],cached:[],unmapped:[]}:exact("bazel")')
a=s.index("counted=processes")
b=s.index(";\n",a)
s=s[:a]+r'''counted=processes?processes[2]!.replace(/\.$/u,"").split(/,\s*/u).map(part=>/^(\d+) (.+)$/u.exec(part.trim())!).filter(m=>!["internal","action cache hit","disk cache hit","remote cache hit"].includes(m[2]!)).reduce((sum,m)=>sum+Number(m[1]),0):0'''+s[b:]
rep('await runIncrementalCommandV1(live,["incremental","plan","--scenario=lib-none","--root="+copy],ports()).catch(error=>expect(String(error)).toContain("Unknown scenario"));','await expect(runIncrementalCommandV1(live,["incremental","plan","--scenario=lib-none","--root="+copy],ports())).rejects.toThrow("Unknown scenario");')
rep('expect(rows[1]!.verdict.stale).toEqual(["lib:generate","shell:activate"]);','expect(rows[1]!.verdict).toMatchObject({stale:["lib:generate","shell:activate"],excess:["zzz:extra"]});')
open(p,"w",encoding="utf8").write(s)
n=C+"/bzl-incr-tools/names.py"
t=open(n,encoding="utf8").read()
t=t.replace(' "FW":FRAMEWORK,',' "BUILDDIR":B,"PYDIR":n(0x1F40D,"oracle"),"FW":FRAMEWORK,',1)
open(n,"w",encoding="utf8").write(t)
print("patched")
