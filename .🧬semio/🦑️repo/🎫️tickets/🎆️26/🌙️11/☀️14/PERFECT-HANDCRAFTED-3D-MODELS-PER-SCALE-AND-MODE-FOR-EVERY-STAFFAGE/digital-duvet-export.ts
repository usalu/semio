/** 🧳 Inspect the geometry audit and static binary package separately for the fitted quilt. */
import {readFileSync} from 'node:fs'
import {auditGeometry,checkGlb,expectedScenes,BUDGET} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts'
import {writeGlb} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
if(process.argv[3]==='audit'){
 const check=await auditGeometry(a)
 console.log(JSON.stringify({findings:check.findings,scenes:check.facts.scenes}))
}else{
 const bytes=await writeGlb(a,{pose:a.presentation.defaultPose})
 console.log(JSON.stringify({bytes:bytes.length,budget:BUDGET.glbBytes}))
 const check=await checkGlb(bytes,a.id+'.glb',a.id,expectedScenes(a))
 console.log(JSON.stringify(check.findings))
}
