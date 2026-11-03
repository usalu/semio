import sys
F=sys.argv[1]
L=open(F).read().split("\n")
def idx(prefix,start=0):
    for i in range(start,len(L)):
        if L[i].startswith(prefix): return i
    raise Exception(prefix)
W="../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/"
i20=idx("import {UnitRetirement");i21=idx("import {RetainedWorkBuffer");i23=idx("import type { Paint, DocumentSceneContent")
L[i20]='import type {WorkRetirementProgress} from "@semio-tech/framework/2d/retire";'
L[i21]='import {RetainedWorkRetirement,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkDemand,type RetainedWorkTurn} from "'+W+'🟦️.ts";\nimport {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "'+W+'📈️producer/🟦️.ts";\nimport {ScenePlanOwner,type ScenePlanRead,type ScenePlanNode} from "../🗺️plan-model/🎟️owned/🟦️.ts";'
L[i23]='import type { Paint, DocumentSceneContent, DocumentSceneNode, Matrix } from "../🗺️plan-model/🟦️.ts";'
a=idx("/** 🎬️ Resolves authored algorithms")
c=idx("async function completeSceneProducer")
VEC=open(sys.argv[2]).read()
L[a:c]=[VEC]
open(F,"w").write("\n".join(L))
