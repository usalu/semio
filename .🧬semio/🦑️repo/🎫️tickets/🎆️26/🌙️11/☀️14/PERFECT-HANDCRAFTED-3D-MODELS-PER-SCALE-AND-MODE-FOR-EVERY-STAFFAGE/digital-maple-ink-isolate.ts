/** 🖊️ Distinguish missing woody strokes from the outline of the foliage on the same paper grid. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids,measureBodyView} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
for(const [d,view] of [[50,'elevation'],[500,'front'],[100,'elevation']]){
 const state=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose},{kind:'scale',scale:{denominator:d}}),world=worldSolids(a,state.bones,d),soft=new Set(a.volumes[d].thermal.filter(q=>q.region.startsWith('leaf-')&&q.of).map(q=>q.of))
 for(const [label,filter] of [['none',q=>!q.id.startsWith('twig-fork-')],['twigs',q=>true]]){
  const m=measureBodyView(a,state,world.filter(filter),view,d,.00015*d,soft)
  console.log(JSON.stringify([d,view,label,m.iou,m.p95M,m.p95SoftM]));Bun.gc(true)
 }
}
