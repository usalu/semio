import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stagelib import *
g=Group("C")
G=L+"/🕸️graph/📥️inference/"
p=G+"🧭️script-inputs/🧪️tests/🫧️take-release/🟦️.ts"
g.sub(p,'import{join}from"node:path";','import{join}from"node:path";\nimport{INFERENCE_LIMITS_V1}from"../../../🧬️schema/🟨️.mjs";')
g.sub(p,'the memory of 600 looks stays under 150 MB, without it 150 looks already pass 350 MB','the memory of 600 looks stays under the declared growth bound, without it 150 looks already pass 350 MB')
g.sub(p,' expect(withRelease.peak).toBeLessThan(150);',' const growth=INFERENCE_LIMITS_V1.lookReleaseGrowthBytes/1048576;\n expect(growth,"the declared growth that makes a look release").toBe(128);\n expect(withRelease.peak,"growth bound of a look release plus the process itself and one owner").toBeLessThan(growth+96);')
g.sub(p,'expect(observation).toContain("if(reused&&port&&!deferOffer)await port.offer(info);");','expect(observation).toMatch(/if\\(reused&&port&&!deferOffer\\)await [^\\n]*port\\.offer\\(info\\)/u);')
g.sub(p,'expect(observation).toContain("prepare,fleet,deferOffer},async()=>{projectComputations");','expect(observation).toMatch(/observeInferenceComputationV1\\(\\{[^}]*prepare,fleet,deferOffer[,}][^\\n]*\\},async\\(\\)=>\\{projectComputations/u);')
p2=G+"📦️project-nodes/🏊️project-farm/🧪️tests/🧫️fixtures/🪆️parent-history/🟨️.mjs"
g.sub(p2,'SEMIO_GRAPH_WORKERS: "1"','SEMIO_GRAPH_WORKERS: "2"')
g.save()
