/** 🌱 Compare each paper twig to the center of its projected stock and find any displaced stations. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids,drawnFor,strokesOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,d=50,s=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose},{kind:'scale',scale:{denominator:d}}),world=worldSolids(a,s.bones,d).filter(q=>q.id.startsWith('twig-fork-'))
for(const view of ['elevation','front','plan']){
 const drawn=drawnFor(a,s,view,d),rows=[]
 for(const w of world){
  const q=s.shapes[view].find(q=>q.shape.id==='ink-'+w.id);if(!q)continue
  const source=q.rings[0],rings=strokesOf(view,w,drawn),distance=(p,a,b)=>{
   const dx=b.x-a.x,dy=b.y-a.y,f=Math.max(0,Math.min(1,((p.x-a.x)*dx+(p.y-a.y)*dy)/(dx*dx+dy*dy||1)))
   return Math.hypot(p.x-a.x-f*dx,p.y-a.y-f*dy)
  }
  if(w.id==="twig-fork-t1-o2")console.log(view,JSON.stringify({paper:source,stroke:rings[0],pos:Array.from(w.positions).filter((n,i)=>i<3||i>=18&&i<21||i>=36&&i<39)}));const biggest=Math.max(...rings.flat().map(p=>Math.min(...source.slice(1).map((b,i)=>distance(p,source[i],b)))))
  rows.push([w.id,+biggest.toFixed(4),rings.length,source.length])
 }
 console.log(view,JSON.stringify(rows.sort((a,b)=>b[1]-a[1]).slice(0,10)))
}
