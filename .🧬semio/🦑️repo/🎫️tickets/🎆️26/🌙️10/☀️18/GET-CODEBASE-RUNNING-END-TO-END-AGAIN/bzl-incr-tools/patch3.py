import sys
C=sys.argv[1]
p=C+"/bzl-incr-tools/src/test.ts";s=open(p,encoding="utf8").read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:70])
    s=s.replace(a,b)
rep('expect(()=>incrementalVerdictV1({direct:[],must:["x"],may:[],cutoff:[]} as never,[])).not.toThrow();','expect(()=>incrementalVerdictV1({direct:[],must:["x:y"],may:[],cutoff:[]} as never,[])).not.toThrow();')
rep('expect(()=>incrementalVerdictV1({direct:[],must:["x"],may:[]} as never,[])).toThrow();','expect(()=>incrementalVerdictV1({direct:[],must:["x:y"],may:[]} as never,[])).toThrow();')
rep('const ports=cancelling.ports({execute:async()=>{cancelling.calls.push("execute");if(cancelling.calls.filter(call=>call==="execute").length===3)cancelling.cancel();throw Error("aborted");}});','const ports=cancelling.ports({execute:async()=>{cancelling.calls.push("execute");if(cancelling.calls.filter(call=>call==="execute").length===3){cancelling.cancel();throw Error("aborted");}return {tool:"bazel" as const,executed:["shell:activate"],restored:[],cached:[],unmapped:[]};}});')
rep('const installed=(()=>{try{return createRequire(join(root,"package.json")).resolve("nx/dist/src/tasks-runner/life-cycles/store-run-information-life-cycle");}catch{return undefined;}})();','const installed=(()=>{const path=join(root,"node_modules/nx/dist/src/tasks-runner/life-cycles/store-run-information-life-cycle.js");return existsSync(path)?realpathSync(path):undefined;})();')
rep('import {createRequire} from "node:module";\n','')
open(p,"w",encoding="utf8").write(s)
g=C+"/bzl-incr-tools/gen-scenarios.py";t=open(g,encoding="utf8").read()
t=t.replace('"milliseconds":T("ms","ms")','"milliseconds":T("time (ms)","Zeit (ms)")')
open(g,"w",encoding="utf8").write(t)
