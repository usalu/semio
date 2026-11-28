#!/usr/bin/env python3
# anchored patch of the repo library: suite remote port, stretch floor in the checkout test request, callers.
# usage: apply-product.py <library-root>
import sys,os,shutil
R=sys.argv[1]; HERE=os.path.dirname(os.path.abspath(__file__))
MEM={}; NEW={}
def rep(path,old,new,count=1):
    p=os.path.join(R,path)
    if p not in MEM: MEM[p]=open(p,encoding="utf-8").read()
    assert MEM[p].count(old)==count,(path,old[:60],MEM[p].count(old))
    MEM[p]=MEM[p].replace(old,new)
def put(path,source):
    NEW[os.path.join(R,path)]=open(os.path.join(HERE,"staged",source),encoding="utf-8").read()
def flush():
    if os.environ.get("DRY")=="1": print("dry run: all anchors found"); return
    for p,v in list(MEM.items())+list(NEW.items()):
        os.makedirs(os.path.dirname(p),exist_ok=True); open(p+".tmp","w",encoding="utf-8").write(v); os.replace(p+".tmp",p)
K="🏗️build/🌐️remote-execution/🚀️offload/🔬️checkout-test/"
rep(K+"🧬️schema/🔣️.json",'''        "budgetSeconds": {"type": "integer", "minimum": 1}
      }
    },
    "Result": {''','''        "budgetSeconds": {"type": "integer", "minimum": 1},
        "stretchBasisPoints": {"type": "integer", "minimum": 10000, "maximum": 640000}
      }
    },
    "Result": {''')
rep(K+"📐️policy/🟦️.ts","language:\"en\"|\"de\";budgetSeconds:number}>;","language:\"en\"|\"de\";budgetSeconds:number;stretchBasisPoints?:number}>;")
rep(K+"🔌️client/🟦️.ts","file:string;budgetSeconds:number;language:","file:string;budgetSeconds:number;stretchBasisPoints?:number;language:")
rep(K+"🔌️client/🟦️.ts","language:input.language,budgetSeconds:input.budgetSeconds},text=","language:input.language,budgetSeconds:input.budgetSeconds,...input.stretchBasisPoints===undefined?{}:{stretchBasisPoints:input.stretchBasisPoints}},text=")
rep(K+"👷️worker/🟦️.ts",'import {homedir} from "node:os";','import {homedir} from "node:os";\nimport {stretchEnvironmentV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";')
rep(K+"👷️worker/🟦️.ts","[policy.environment.mode]:policy.environment.workerMode,SEMIO_LANGUAGE:request.language},","[policy.environment.mode]:policy.environment.workerMode,SEMIO_LANGUAGE:request.language,...request.stretchBasisPoints===undefined?{}:stretchEnvironmentV1(request.stretchBasisPoints)},")
rep("🏗️build/🔬️test-file/🌐️remote/🟦️.ts","language:input.language,signal:controller.signal,claim,","language:input.language,stretchBasisPoints:(await import(\"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts\")).launchStretchBasisPointsV1(),signal:controller.signal,claim,")
put("🏗️build/🔬️test-file/🌐️remote/🗂️suite/🟦️.ts","suite-remote.ts")
rep("🏗️build/🗂️suites/📜️script.ts",'output,environment,launchers,workers:{learning:testWorkerLearningV1(environment)}}),suite.scratch);','output,environment,launchers,workers:{learning:testWorkerLearningV1(environment)},...(remote===undefined?{}:{remote})}),suite.scratch);')
rep("🏗️build/🗂️suites/📜️script.ts","  await withProcessTestOutputV1(checkout,\"build-contract\"","  const{createSuiteRemoteV1}=await import(\"../🔬️test-file/🌐️remote/🗂️suite/🟦️.ts\"),remote=await createSuiteRemoteV1(checkout,suite.id,process.env);\n  await withProcessTestOutputV1(checkout,\"build-contract\"")
rep("⚡️caching/📜️script.ts","output, environment, launchers, workers: { learning: testWorkerLearningV1(process.env) } }), table.suites","output, environment, launchers, workers: { learning: testWorkerLearningV1(process.env) }, ...(remote === undefined ? {} : { remote }) }), table.suites")
rep("⚡️caching/📜️script.ts","      await withProcessTestOutputV1(checkout, \"cache-contract\"","      const { createSuiteRemoteV1 } = await import(\"../🏗️build/🔬️test-file/🌐️remote/🗂️suite/🟦️.ts\"), remote = await createSuiteRemoteV1(checkout, args[0]!, process.env);\n      await withProcessTestOutputV1(checkout, \"cache-contract\"")
# thresholds derived from paired local/remote durations (see result-H50.md): below 0.5 per core the file stays; from there it leaves unless its measured remote duration is not shorter than 0.8 of the local one
rep(K+"🧬️schema/🔣️.json",'"required": ["perCoreAlways", "perCoreMeasured", "remoteWorthRatio",','"required": ["perCoreMeasured", "remoteWorthRatio",')
rep(K+"🧬️schema/🔣️.json",'            "perCoreAlways": {"type": "number", "minimum": 0},\n','')
rep(K+"🧬️schema/🔣️.json",'"load": {"perCoreAlways": 1.5, "perCoreMeasured": 0.75,','"load": {"perCoreMeasured": 0.5,')
rep(K+"🚦️gate/🟦️.ts","at or above `perCoreAlways` the file always leaves; below `perCoreMeasured` it stays; in between it leaves unless both a local and a remote duration are known","below `perCoreMeasured` the file stays; from there on, however high the load, it leaves unless both a local and a remote duration are known")
rep(K+"🚦️gate/🟦️.ts",' if(input.loadPerCore>=rule.perCoreAlways)return"offload";\n','')
rep(K+"🧪️tests/🔣️.json",'"policy": {"perCoreAlways": 1.5, "perCoreMeasured": 0.75,','"policy": {"perCoreMeasured": 0.5,')
rep(K+"🧪️tests/🔣️.json",'"name": "at the always threshold the file leaves without any measure", "loadPerCore": 1.5,','"name": "at a high load a file without any measure leaves to learn its remote duration", "loadPerCore": 1.5,')
rep(K+"🧪️tests/🔣️.json",'{"name": "far above the always threshold the file leaves even when the remote run is measured as worthless", "loadPerCore": 4.2, "local": {"seconds": 10, "ageSeconds": 10}, "remote": {"seconds": 30, "ageSeconds": 10}, "verdict": "offload"}','{"name": "far above the measured threshold a file measured as worthless remotely stays", "loadPerCore": 4.2, "local": {"seconds": 10, "ageSeconds": 10}, "remote": {"seconds": 30, "ageSeconds": 10}, "verdict": "keep"}')
rep(K+"🧪️tests/🔣️.json",'"name": "below the measured threshold the file stays", "loadPerCore": 0.5,','"name": "below the measured threshold the file stays", "loadPerCore": 0.4,')
rep(K+"🧪️tests/🐍️.py",'    if load >= policy["perCoreAlways"]:\n        return "offload"\n','')

put("🏗️build/🔬️test-file/🧪️tests/🗂️suite-remote/🟦️.ts","suite-remote-test.ts")
flush()
