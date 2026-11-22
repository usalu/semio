/** 🛒 Compare the four-scale chassis plan against the actual restored grip, bumper, axle and fork surfaces. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {featureOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts'
import {worldSolids,measureBodyView} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,d=500,rep=a.representations[d],base=rep.geometry.plan,fine=a.representations[50].geometry.plan
const state=()=>evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose},{kind:'scale',scale:{denominator:d}}),mesh=worldSolids(a,state().bones,d)
for(const pen of [.10,.11,.12,.13,.15]){
 const add=fine.filter(s=>/^(?:hand-[lr]|bumper-bar|handle-grip)$/.test(s.id)),ids=new Set(add.map(s=>s.id))
 rep.geometry.plan=base.filter(s=>!ids.has(s.id)).map(s=>/^(?:chassis|front-leg-[lr])$/.test(s.id)?{...s,penMm:pen}:s).concat(add.map(s=>({...s,penMm:.21})))
 const v=measureBodyView(a,state(),mesh,'plan',d,featureOf(d),new Set());console.log(pen,v.iou,v.p95M,v.hausdorffM)
}
