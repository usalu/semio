/** 🛏️ Compare each bed-cover panel's deformed outer vertices with the native orthographic panel. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {projectWorld} from './volumes-base.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,v=p=>new Vector3(p.x,p.y,p.z)
function ds(p,q,k){const dx=k.x-q.x,dy=k.y-q.y,d=dx*dx+dy*dy,t=Math.max(0,Math.min(1,((p.x-q.x)*dx+(p.y-q.y)*dy)/(d||1)));return Math.hypot(q.x+t*dx-p.x,q.y+t*dy-p.y)}
function enclosed(p,r){
 let inside=false
 for(let i=0,j=r.length-1;i<r.length;j=i++){
  const a=r[i],b=r[j]
  if((a.y>p.y)!==(b.y>p.y)&&p.x<(b.x-a.x)*(p.y-a.y)/(b.y-a.y)+a.x)inside=!inside
 }
 return inside
}
for(const pose of a.poses)for(const d of [50,100,200,500]){
 const state=evaluateDrawing(a,{kind:'pose',pose:pose.id},{kind:'scale',scale:{denominator:d}}),b=state.bones
 const maximum={}
 for(const view of ['elevation','front','plan']){
  const discrepancies=[]
  for(const panel of a.volumes[d].body.filter(q=>q.id.startsWith('baffle-'))){
   const shp=state.shapes[view].find(q=>q.shape.id===panel.id),rings=shp?.rings??[]
   for(const [x,y,z] of panel.vertices.slice(0,3)){
    const p=new Vector3()
    for(const w of panel.weights){const f=b[w.bone].frame;p.addScaledVector(v(f.o).addScaledVector(v(f.u),x).addScaledVector(v(f.v),y).addScaledVector(v(f.w),z),w.weight)}
    const target=projectWorld(view,p.toArray());discrepancies.push(rings.some(r=>enclosed(target,r))?0:Math.min(...rings.flatMap(r=>r.map((q,i)=>ds(target,q,r[(i+1)%r.length])))))
   }
  }
  discrepancies.sort((a,b)=>a-b);maximum[view]=discrepancies[Math.floor(.95*discrepancies.length)]
 }
 assert(Object.values(maximum).every(x=>x<.001),'Projected quilt seams in '+pose.id+' '+d)
}
