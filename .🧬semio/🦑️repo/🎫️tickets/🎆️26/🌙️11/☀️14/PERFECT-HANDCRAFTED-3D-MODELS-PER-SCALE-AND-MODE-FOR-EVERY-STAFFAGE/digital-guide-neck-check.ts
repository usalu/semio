/** 🦴 Exercise the guide’s independent cervical and skull pivots and the preserved combined centre of mass. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-guide-neck.json',import.meta.url),'utf8'))
const neck=a.rig.bones.find(b=>b.id==='neck'),head=a.rig.bones.find(b=>b.id==='head')
assert.equal(neck?.parent,'torso');assert.equal(head?.parent,'neck')
const vec=q=>new Vector3(q.x??0,q.y??0,q.z??0)
const at=(b,q)=>vec(b.frame.o).addScaledVector(vec(b.frame.u),q[0]).addScaledVector(vec(b.frame.v),q[1]).addScaledVector(vec(b.frame.w),q[2])
const near=(d,t=1e-6)=>assert.ok(d<t,String(d))
near(vec(head).distanceTo(new Vector3(...f.atlas)))
near(Math.abs(head.mass.kg+neck.mass.kg-f.headNeckMassKg))
const com=new Vector3(head.x+head.mass.comX,head.y+head.mass.comY,0).multiplyScalar(head.mass.kg).addScaledVector(new Vector3(neck.mass.comX,neck.mass.comY,0),neck.mass.kg).divideScalar(f.headNeckMassKg)
near(com.distanceTo(new Vector3(...f.combinedComM)))
for(const p of a.poses){
 const state=evaluateDrawing(a,{kind:'pose',pose:p.id})
 near(at(state.bones.neck,f.atlas).distanceTo(vec(state.bones.head.frame.o)))
 near(Math.abs(state.bones.neck.offset.angleDeg-f.lowerPitchDeg[p.id]))
}
assert.equal(a.poses.length,f.poseCount)
const hold=a.poses.find(p=>p.id==='holding-flag')
const probe={...hold,id:'isolated-skull-nod',bones:{...hold.bones,head:{angleDeg:15}}}
const d={...a,poses:[...a.poses,probe]},before=evaluateDrawing(d,{kind:'pose',pose:hold.id}),after=evaluateDrawing(d,{kind:'pose',pose:probe.id})
near(vec(before.bones.neck.tip).distanceTo(vec(after.bones.neck.tip)))
near(vec(before.bones.head.frame.o).distanceTo(vec(after.bones.head.frame.o)))
assert.ok(vec(before.bones.head.tip).distanceTo(vec(after.bones.head.tip))>.03)
for(const v of Object.values(a.volumes)){
 assert.equal(v.body.find(b=>b.id==='neck').bone,'neck');assert.equal(v.thermal.find(b=>b.id==='th-neck').bone,'neck')
 assert.equal(v.structure.find(b=>b.id==='cervical-spine').member,'neck')
 assert.ok(['neck','head'].every(k=>v.structure.some(s=>s.member===k&&s.part==='joint')))
}
console.log('Seven guide poses, four skeletons, mass, atlas attachment and an isolated skull nod passed.')
