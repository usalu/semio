/** 🐐 Verify paired mammary anatomy, gravity response, mass transfer and thermal skin allocation. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-doe-suspension.json',import.meta.url),'utf8')),id='mammary-suspension'
const bone=a.rig.bones.find(b=>b.id===id);assert.equal(bone?.parent,'pelvis');assert.equal(bone.mass.kg,f.massKg)
const pelvis=a.rig.bones.find(b=>b.id==='pelvis'),sum=pelvis.mass.kg+bone.mass.kg
assert.ok(Math.abs(sum-4.387)<1e-8)
const V=p=>new Vector3(p.x,p.y,p.z),world=(fr,p)=>V(fr.o).addScaledVector(V(fr.u),p[0]).addScaledVector(V(fr.v),p[1]).addScaledVector(V(fr.w),p[2])
for(const [pose,want] of [['standing-alert',0],['climbing-ledge',8.8],['browsing-upright',24.8]]){
 const s=evaluateDrawing(a,{kind:'pose',pose}).bones,fr=s[id].frame
 assert.ok(Math.abs(V(fr.u).angleTo(new Vector3(0,-1,0))*180/Math.PI-want)<.01)
 if(pose==='standing-alert'){
  assert.ok(V(fr.o).distanceTo(new Vector3(...f.attachmentM))<1e-6)
  const old=world(s.pelvis.frame,[.0631,.0049,0]);const current=world(s.pelvis.frame,[pelvis.mass.comX,pelvis.mass.comY,0]).multiplyScalar(pelvis.mass.kg).addScaledVector(world(fr,[...f.comM,0]),bone.mass.kg).divideScalar(sum)
  assert.ok(old.distanceTo(current)<1e-6)
 }
}
for(const [scale,vol] of Object.entries(a.volumes)){
 for(const part of ['udder-near','udder-far','teat-near','teat-far']){
  const body=vol.body.find(q=>q.id===part);assert.equal(body?.bone,id);assert.equal(body.material,'bare-skin');assert.ok(vol.thermal.some(q=>q.of===part&&q.region==='mammary-skin'))
 }
 assert.ok(vol.structure.some(q=>q.member===id&&q.id==='median-suspensory-lamina'),scale)
}
const reg=a.thermal.regions.find(q=>q.id==='mammary-skin');assert.equal(reg.areaM2,f.freshSkinAreaM2);assert.equal(reg.layers.length,0)
console.log(JSON.stringify({poses:a.poses.length,tiltDeg:[0,8.8,24.8],kg:sum,areaM2:reg.areaM2}))
