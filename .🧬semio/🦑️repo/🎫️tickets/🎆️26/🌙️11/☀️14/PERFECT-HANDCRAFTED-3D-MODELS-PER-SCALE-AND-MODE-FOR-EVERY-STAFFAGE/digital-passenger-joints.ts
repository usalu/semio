/** 🧒 Measure each passenger’s attached respiratory outlets and articulated head against the authored seat or hip. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-passenger-joints.json',import.meta.url),'utf8'))[a.id]
const bone=id=>a.rig.bones.find(q=>q.id===id),xyz=q=>new Vector3(q.x,q.y,q.z??0)
assert.equal(bone(f.neck)?.parent,f.root);assert.equal(bone(f.head)?.parent,f.neck)
if(f.neckMassKg)assert.equal(bone(f.neck).mass.kg,f.neckMassKg)
for(const p of a.poses){
 const state=evaluateDrawing(a,{kind:'pose',pose:p.id}),head=state.bones[f.head].frame
 for(const [name,id] of f.breath){
  const anchor=a.anchors.find(q=>q.id===name);assert.equal(anchor.bone,f.head)
  const position=q=>xyz(head.o).addScaledVector(xyz(head.u),q[0]).addScaledVector(xyz(head.v),q[1]).addScaledVector(xyz(head.w),q[2])
  for(const v of Object.values(a.volumes)){
   const outlet=v.thermal.find(q=>q.id===id)
   assert.equal(outlet.bone,f.head);assert.ok(position(outlet.transform.translation).distanceTo(position([anchor.x,anchor.y,anchor.z??0]))<.001)
   assert.ok(v.structure.some(s=>s.member===f.neck));assert.ok(v.structure.some(s=>s.member===f.head))
  }
 }
}
if(f.leftArm){
 const root=bone('child'),mass=a.rig.bones.filter(b=>b.id==='child'||b.id.startsWith('child-')).reduce((sum,b)=>sum+b.mass.kg,0)
 assert.ok(Math.abs(mass-10)<1e-6)
 assert.equal(a.thermal.regions.find(q=>q.id==='child-skin').areaM2,f.skinAreaM2)
 const state=evaluateDrawing(a,{kind:'pose',pose:'pushing-stroller'}),frame=state.bones.child.frame
 function world(q){return xyz(frame.o).addScaledVector(xyz(frame.u),q[0]).addScaledVector(xyz(frame.v),q[1]).addScaledVector(xyz(frame.w),q[2]-.1925)}
 for(const [side,suffix] of [['left','l'],['right','r']])for(const [family,segments] of [['Arm',['upper-arm','forearm','hand']],['Leg',['thigh','shin','foot']]]){
  const path=f[side+family]
  for(const [i,segment] of segments.entries()){
   const id='child-'+segment+'-'+suffix,b=state.bones[id].frame,tip=xyz(b.o).addScaledVector(xyz(b.u),bone(id).lengthM)
   assert.ok(xyz(b.o).distanceTo(world(path[i]))<.00003,id+' origin')
   assert.ok(tip.distanceTo(world(path[i+1]))<.00003,id+' tip')
   for(const v of Object.values(a.volumes)){assert.ok(v.body.some(q=>q.bone===id),id+' body');assert.ok(v.structure.some(q=>q.member===id),id+' structure')}
  }
 }
}
console.log(a.id,a.poses.length,'passenger poses with independent skull and attached nose and mouth outlets checked.')
