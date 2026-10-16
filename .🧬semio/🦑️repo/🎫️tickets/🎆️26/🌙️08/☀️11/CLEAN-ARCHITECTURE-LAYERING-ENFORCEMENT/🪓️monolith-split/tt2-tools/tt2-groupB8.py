from tt2_edit import *
S=ANY+'🧬️schema/🎬️scene/'
FX='"../../../../../🧪️tests/🧫️fixtures/🟦️.ts"'
edit(S+'🔍️trace/🧪️tests/🔬️unit/🟦️.ts',[
 ('import {ScenePlanCloseJob} from "../../../🧹️retire/🟦️.ts";\n','import {ScenePlanCloseJob} from "../../../🧹️retire/🟦️.ts";\nimport {rasterSceneGroupFixture} from '+FX+';\n'),
 ('for(const cap of ["maxSourceBytes","maxAdmittedPixels"]){','for(const cap of ["maxSourceBytes","maxAdmittedPixels"] as const){'),
 ('n.groups.push(structuredClone(retirementRows.metadata.group))','n.groups.push(rasterSceneGroupFixture(retirementRows.metadata.group))'),
])
edit(S+'🔀️booleans/🧪️tests/🔬️unit/🟦️.ts',[
 ('import {ScenePlanCloseJob} from "../../../🧹️retire/🟦️.ts";\n','import {ScenePlanCloseJob} from "../../../🧹️retire/🟦️.ts";\nimport {rasterSceneAssetFixture,rasterSceneGroupFixture} from '+FX+';\n'),
 ('v.plan.nodes[0]!.id="result";','Object.assign(v.plan.nodes[0]!,{id:"result"});'),
 ('value.plan.nodes.at(-1)!.transform=[1e8,0,0,1e8,0,0];','Object.assign(value.plan.nodes.at(-1)!,{transform:[1e8,0,0,1e8,0,0]});'),
 ('n.groups=[structuredClone(retirementRows.metadata.group)];','n.groups=[rasterSceneGroupFixture(retirementRows.metadata.group)];'),
 ('source.plan.assets=[structuredClone(retirementRows.metadata.asset)];','source.plan.assets=[rasterSceneAssetFixture(retirementRows.metadata.asset)];'),
])
