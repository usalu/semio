/** 🍃 Identify whether a maple projection mismatch comes from its limb stock, leaf profile, or printed line widths. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids,drawnFor} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {silhouetteAgreement} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
for(const view of ['elevation','front']){
 const pair=[500,200].map(d=>{const state=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose},{kind:'scale',scale:{denominator:d}});return {w:worldSolids(a,state.bones,d).filter(q=>!q.texture),ink:drawnFor(a,state,view,d)}})
 for(const [name,filter] of [['pads',q=>q.bone?.startsWith('fol-')&&!q.id.startsWith('twig-fork-')],['without-forks',q=>!q.id.startsWith('twig-fork-')],['all',q=>true]]){
  const p=pair.map(z=>z.w.filter(filter))
  for(const ink of [true,false]){
   const result=silhouetteAgreement(p[0],p[1],view,.075,.075,{limitM:.15,...(ink?{drawnFirst:pair[0].ink,drawnSecond:pair[1].ink}:{})})
   console.log(JSON.stringify([view,name,ink,result.rawIou,result.p95M]));Bun.gc(true)
  }
 }
}
