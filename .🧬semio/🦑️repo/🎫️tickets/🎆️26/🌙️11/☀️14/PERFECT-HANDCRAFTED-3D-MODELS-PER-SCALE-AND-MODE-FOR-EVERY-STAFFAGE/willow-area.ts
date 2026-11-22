/** 🌳 Measure exposed woody surfaces separately from transpiring foliage in the reference stance. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {exposedArea} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,k=Number(process.argv[3]),v=a.volumes[k],meshes=worldSolids(a,evaluateDrawing(a,{kind:'pose',pose:'calm'}).bones,k)
for(const id of ['trunk-bark','limb-bark']){
 const ids=new Set(v.thermal.filter(q=>q.region===id).map(q=>q.of)),area=exposedArea(meshes.filter(q=>ids.has(q.id)),meshes)
 console.log(JSON.stringify({k,id,areaM2:area}))
}
