import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stagelib import *
g=Group("I")
RS=L+"/🏡️resident/🧪️tests/"
# containment
p=L+"/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧹️containment/🧪️tests/🟦️.ts"
g.sub(p,' expect(source.match(/resource:"native-artifact-input-(?:stage|verification|read):"\\+nativeArtifactInputStepV1\\(\\)/gu)?.length).toBe(5);',' const sites=source.match(/resource:"native-artifact-input-(?:stage|verification|read)[:"]/gu)??[],stepped=source.match(/resource:"native-artifact-input-(?:stage|verification|read):"\\+nativeArtifactInputStepV1\\(\\)/gu)??[];\n expect(stepped.length,"every staging, verification and read site of an input carries the counter").toBe(sites.length);expect(stepped.length).toBeGreaterThanOrEqual(5);')
# plan refusal
p=RS+"🪜️plan-refusal/🟦️.ts"
g.sub(p,'expect(worker).toContain("onPlanStageV1(event=>parentPort!.postMessage(event))");','expect(worker).toMatch(/onPlanStageV1\\(event=>\\{[^\\n]*parentPort!\\.postMessage\\(event\\)/u);')
# short bases
for name,count in (("🆔️host-identity",4),("🧭️start-identity",2)):
    p=RS+name+"/🟦️.ts"
    t=g.text(p)
    g.sub(p,'import {tmpdir} from "node:os";','import {tmpdir} from "node:os";\nconst shortBase=process.platform==="win32"?tmpdir():"/tmp";')
    g.sub(p,'join(tmpdir(),"','join(shortBase,"',count=count)
# farm waits below one worker
p=L+"/🌡️host-capacity/🧪️tests/👥️agents/🟦️.ts"
g.sub(p,'sample:async()=>({host:host(3000),swappedOutMegabytes:0})});\n await sleep(80);\n const [waiter]','sample:async()=>({host:host(300),swappedOutMegabytes:0})});\n await sleep(80);\n const [waiter]')
p=L+"/🌡️host-capacity/🧪️tests/🧾️label/🟦️.ts"
g.sub(p,'host(3000)','host(300)')
# first child
p=L+"/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🥇️first-child/🟦️.ts"
g.sub(p,'covers one worker beside a parent of one worker\'s size,','covers one worker beside a lean parent of half a worker\'s size,')
g.sub(p,'.toBeGreaterThanOrEqual(2*GRAPH_FARM_POLICY_V1.workerMegabytes);','.toBeGreaterThanOrEqual(Math.floor(1.5*GRAPH_FARM_POLICY_V1.workerMegabytes));')
g.save()
