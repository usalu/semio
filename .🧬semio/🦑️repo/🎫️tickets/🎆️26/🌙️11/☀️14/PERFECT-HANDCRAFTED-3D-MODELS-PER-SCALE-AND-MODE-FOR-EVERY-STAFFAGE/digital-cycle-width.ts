/** 🚲 Measure which small-scale wheel and carrier signs need an independent printable stroke. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids,measureBodyView} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,rep=a.representations['500'],shapes=rep.geometry.elevation
for(const names of [['wheel-f'],['wheel-r'],['frame'],['bar'],['fork'],['frame','bar','fork'],['box'],['all']]){
 const keep=q=>names[0]==='all'||names.includes(q.bone);rep.geometry.elevation=shapes.filter(keep)
 const s=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose},{kind:'scale',scale:{denominator:500}}),world=worldSolids(a,s.bones,500).filter(keep),m=measureBodyView(a,s,world,'elevation',500,.075,new Set())
 console.log(names.join(','),m.iou,m.p95M);Bun.gc(true)
}
