/** 🧒 Measure bilateral bone cross sections and aligned joint centres against each subject's own fine posed anatomy. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3,Box3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,cases=['digital-schoolchild-case.json','digital-human-shaft-case.json'].map(name=>JSON.parse(readFileSync(new URL('./'+name,import.meta.url),'utf8'))),f=cases.find(c=>c.subjects[a.id]),pat=f.subjects[a.id],v=p=>new Vector3(p.x,p.y,p.z)
function section(mesh,frame,station){
 const box=new Box3(),p=mesh.world,idx=mesh.indices,points=Array.from({length:p.length/3},(_,i)=>{const d=new Vector3().fromArray(p,i*3).sub(v(frame.o));return new Vector3(d.dot(v(frame.u)),d.dot(v(frame.v)),d.dot(v(frame.w)))})
 for(let k=0;k<idx.length;k+=3)for(let j=0;j<3;j++){
  const p=points[idx[k+j]],q=points[idx[k+(j+1)%3]]
  if((p.x<=station)!==(q.x<=station))box.expandByPoint(p.clone().lerp(q,(station-p.x)/(q.x-p.x)))
 }
 if(box.isEmpty())return null
 return {dia:box.getSize(new Vector3()),centre:box.getCenter(new Vector3())}
}
let worstD=0,worstC=0;const failures=[]
for(const pose of [a.presentation.defaultPose,pat.pose]){
 const bones=evaluateDrawing(a,{kind:'pose',pose}).bones,m=posedMesh(a,pose)
 for(const side of f.sides){
  for(const [part,limb] of Object.entries(f.limbs)){
   const bone=(pat.members?.[limb]??limb)+'-'+side,id=pat.prefix+part+'-'+side,frame=bones[bone].frame,l=pat.sideLengths?.[side]?.[limb]??pat.lengths[limb]
   assert(Math.abs(v(bones[bone].tip).distanceTo(v(frame.o))-l)<.0005)
   for(const t of pat.stationsBySide?.[side]?.[limb]??f.stations){
    const fine=a.volumes[50].structure.find(s=>s.id===id),ref=section(m(fine,qualityOf(a,50)),frame,l*t);assert(ref,'fine section '+pose+' '+id+' '+t)
    for(const d of f.scales){
     const s=a.volumes[d].structure.find(s=>s.id===id);if(!s){failures.push([d,id,'absent']);continue}
     const actual=section(m(s,qualityOf(a,d)),frame,l*t);if(!actual){failures.push([d,id,'empty section',t]);continue}
     const diameter=Math.max(Math.abs(ref.dia.y-actual.dia.y),Math.abs(ref.dia.z-actual.dia.z)),centre=ref.centre.distanceTo(actual.centre)
     worstD=Math.max(worstD,diameter);worstC=Math.max(worstC,centre)
     if(diameter>f.maxDiameterDeviationM||centre>f.maxCentreDeviationM)failures.push([d,id,diameter,centre])
     if(part==='femur'&&t===.52&&Math.min(actual.dia.y,actual.dia.z)<pat.femurDiameterM[0])failures.push([d,id,'narrow'])
    }
   }
  }
  for(const slot of ['arm','forearm','thigh','shank']){
   const id=(pat.jointNames?.[slot]??'joint-'+(pat.joints?.[slot]??slot))+'-'+side,ref=a.volumes[50].structure.find(s=>s.id===id)
   for(const d of f.scales){const s=a.volumes[d].structure.find(s=>s.id===id)
    if(!s){failures.push([d,id,'missing joint']);continue}
    const extent=(q,d)=>{const mesh=m(q,qualityOf(a,d)),box=new Box3();for(let i=0;i<mesh.world.length;i+=3)box.expandByPoint(new Vector3().fromArray(mesh.world,i));return box.getSize(new Vector3())}
    if(extent(s,d).sub(extent(ref,50)).length()>.006)failures.push([d,id,'joint width'])
   }
  }
 }
}
console.log(a.id,JSON.stringify({maximumDiameterChangeMm:worstD*1000,maximumCentreOffsetMm:worstC*1000,failed:failures.length,examples:failures.slice(0,6)}))
assert.equal(failures.length,0)
