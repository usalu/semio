/** 🦴 Cut the exported child and adult long-bone surfaces at their physical midshaft and measure each transverse section. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3,Box3} from 'three'
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-toddler-bone-case.json',import.meta.url),'utf8'))
function slice(s,q){
 if(!s)return null
 const mesh=tessellateSolid({...s,node:s.bone},q),xyz=i=>new Vector3(...mesh.positions.slice(i*3,i*3+3)),box=new Box3()
 for(let i=0;i<mesh.positions.length/3;i++)box.expandByPoint(xyz(i))
 const mid=(box.min.x+box.max.x)/2,section=new Box3()
 for(let i=0;i<mesh.indices.length;i+=3){const t=mesh.indices.slice(i,i+3)
  for(let j=0;j<3;j++){const a=xyz(t[j]),b=xyz(t[(j+1)%3]);if((a.x<=mid)!==(b.x<=mid))section.expandByPoint(a.clone().lerp(b,(mid-a.x)/(b.x-a.x)))}
 }
 const size=section.getSize(new Vector3()).multiplyScalar(1000)
 return {mm:Math.min(size.y,size.z),center:section.getCenter(new Vector3()),length:box.max.x-box.min.x}
}
const errors=[]
for(const d of f.scales){
 const v=a.volumes[d].structure,q=qualityOf(a,d),metrics={}
 for(const side of ['l','r']){
  const adult=slice(v.find(s=>s.id==='femur-'+side),q)
  if(adult.mm<f.adultFemurMm[0]||adult.mm>f.adultFemurMm[1])errors.push(d+' adult femur '+adult.mm)
  let ulna,radius
  for(const [k,[lo,hi]] of Object.entries(f.shaftWidthMm)){
   const s=v.find(s=>s.id==='toddler-'+k+'-'+side),m=slice(s,q);metrics[k]=m?+m.mm.toFixed(2):null
   if(!m||m.mm<lo||m.mm>hi)errors.push(d+' '+k+' '+m?.mm)
   if(k==='femur'&&m&&m.mm/adult.mm>f.maxChildAdultFemurRatio)errors.push('femur ratio')
   if(k==='ulna')ulna=m;if(k==='radius')radius=m
  }
  if(!ulna||!radius||ulna.center.distanceTo(radius.center)*1000<f.minRadiusUlnaCentreGapMm)errors.push(d+' forearm space')
 }
 console.log(d,metrics)
}
const vi=p=>new Vector3(p.x,p.y,p.z)
for(const pose of a.poses){
 const b=evaluateDrawing(a,{kind:'pose',pose:pose.id}).bones
 for(const d of f.scales)for(const side of ['l','r']){
  const solid=k=>a.volumes[d].structure.find(s=>s.id==='toddler-'+k+'-'+side)
  function joint(s,tip){
   const frame=b[s.bone].frame,p=s.sections?.at(tip?-1:0)?.at??(tip?s.p1:s.p0)
   return vi(frame.o).addScaledVector(vi(frame.u),p[0]).addScaledVector(vi(frame.v),p[1]).addScaledVector(vi(frame.w),p[2])
  }
  for(const [proximal,distal] of [['femur','tibia'],['humerus','ulna'],['humerus','radius']]){
   const m=solid(proximal),n=solid(distal)
   if(!m||!n||joint(m,true).distanceTo(joint(n,false))>.036)errors.push([pose.id,d,proximal,distal])
  }
 }
}
assert.deepEqual([...new Set(errors)],[])
