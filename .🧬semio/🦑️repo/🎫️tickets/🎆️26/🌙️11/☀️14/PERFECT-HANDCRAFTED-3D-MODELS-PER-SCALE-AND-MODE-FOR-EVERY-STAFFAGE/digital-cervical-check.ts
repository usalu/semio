/** 🦴 Compare skull orientation and cervical support in every authored pose with an independent coordinate transform. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const [p,q]=process.argv.slice(2)
const base=JSON.parse(readFileSync(p,'utf8')).asset,a=JSON.parse(readFileSync(q,'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-cervical-stations.json',import.meta.url),'utf8'))[a.id]
const bone=(x,k)=>x.rig.bones.find(b=>b.id===k),neck=bone(a,'neck'),head=bone(a,'head'),old=bone(base,'head')
assert.equal(neck?.parent,'torso');assert.equal(head.parent,'neck')
const v=p=>new Vector3(p.x,p.y,p.z??0),near=(n,t=1e-6)=>assert.ok(n<t,String(n))
near(v(head).distanceTo(new Vector3(...f.atlas)))
near(Math.abs(head.mass.kg+neck.mass.kg-old.mass.kg))
const centroid=v(head).add(new Vector3(head.mass.comX,head.mass.comY,0)).multiplyScalar(head.mass.kg)
centroid.addScaledVector(new Vector3(neck.mass.comX,neck.mass.comY,0),neck.mass.kg).divideScalar(old.mass.kg)
near(centroid.distanceTo(new Vector3(old.mass.comX,old.mass.comY,0)))
for(const pose of a.poses){
 const query={kind:'pose',pose:pose.id},before=evaluateDrawing(base,query),after=evaluateDrawing(a,query),c=after.bones.neck.frame
 const pivot=v(c.o).addScaledVector(v(c.u),head.x).addScaledVector(v(c.v),head.y)
 near(pivot.distanceTo(v(after.bones.head.frame.o)))
 for(const axis of ['u','v','w'])near(v(before.bones.head.frame[axis]).distanceTo(v(after.bones.head.frame[axis])),2e-6)
 for(const [k,i] of [['angleDeg',0],['abductionDeg',1],['twistDeg',2]])near(Math.abs(after.bones.neck.offset[k]-f.poses[pose.id][i]))
}
if(f.faceAndNeckAreaM2){
 const surface=a.volumes['50'].thermal.find(s=>s.id==='th-cervical-skin')
 assert.equal(a.thermal.regions.find(q=>q.id===surface.region).areaM2,f.faceAndNeckAreaM2)
}
for(const volume of Object.values(a.volumes)){
 const exposed=volume.thermal.find(s=>s.id==='th-cervical-skin')
 if(exposed){assert.equal(exposed.bone,'neck');assert.equal(exposed.capStart,false);assert.equal(exposed.capEnd,false);assert.deepEqual([exposed.sections[0].at[0],exposed.sections.at(-1).at[0]],f.exposedNeckBandM)}
 assert.ok(volume.body.some(s=>s.bone==='neck'&&s.id==='neck'))
 assert.ok(volume.structure.some(s=>s.member==='neck'&&s.part==='member')&&volume.structure.some(s=>s.member==='head'&&s.part==='joint'))
}
console.log(a.id,a.poses.length,'poses retained head orientation, mass and atlanto-occipital attachment.')
