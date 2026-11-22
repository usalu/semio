/** 📐 Bind the hedge drawings to the sections of the same living plant when the crown sways across the row. */
import {evaluateDrawing,flattenGeometry} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const r=x=>+x.toFixed(7)
export function drawSway(a){
 const bones=evaluateDrawing(a,{kind:'pose',pose:'calm'}).bones
 for(const [scale,rep] of Object.entries(a.representations)){
  if(rep.status!=='authored')continue
  const vol=a.volumes[scale],memo=new Map()
  for(const [view,shapes] of Object.entries(rep.geometry))for(const [i,shape] of shapes.entries()){
   if(!/^(stem|block)-[1-6]$/.test(shape.bone)||shape.kind==='member'||shape.solid)continue
   const axis=view==='elevation'?2:view==='plan'?0:1,frame=bones[shape.bone].frame
   const body=vol.body.find(s=>s.id===shape.id)??vol.thermal.find(s=>s.id==='th-'+shape.id&&!s.of)??vol.body.find(s=>s.bone===shape.bone&&s.id===shape.bone)??vol.body.find(s=>s.bone===shape.bone)
   if(!body)throw Error(shape.id)
   if(!memo.has(body)){
    const vertices=tessellateSolid({...body,node:body.bone},qualityOf(a,Number(scale))).positions
    const low=[Infinity,Infinity,Infinity],high=[-Infinity,-Infinity,-Infinity]
    for(let j=0;j<vertices.length;j++){const n=j%3;low[n]=Math.min(low[n],vertices[j]);high[n]=Math.max(high[n],vertices[j])}
    memo.set(body,[low,high])
   }
   const [lo,hi]=memo.get(body),detail=['texture','detail'].includes(shape.role)
   const near=hi[axis],solid={kind:'extruded',sweep:['u','v','w'][axis],z0:r(detail?near-.0015:lo[axis]),z1:r(near)}
   if(view==='elevation'){shape.solid=solid;continue}
   const flat=flattenGeometry(shape)
   const local=p=>view==='front'?[r(p.y-frame.o.y),r(p.x-frame.o.z)]:[r(frame.o.x-p.x),r(p.y-frame.o.z)]
   if(flat.rings.length!==1)throw Error('Expected single projected outline '+shape.id)
   const {points,d,closed,kind,cx,cy,rx,ry,x,y,width,height,...style}=shape
   shapes[i]={...style,kind:'poly',closed:flat.closed,points:flat.rings[0].map(local),solid}
  }
 }
}
