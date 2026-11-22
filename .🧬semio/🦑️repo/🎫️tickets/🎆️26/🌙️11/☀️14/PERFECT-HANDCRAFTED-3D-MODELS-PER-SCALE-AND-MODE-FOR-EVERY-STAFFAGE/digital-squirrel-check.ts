/** 🐿️ Independently check the caudal chain, its seated sweep, regional fur, and count of individual caudals. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const f=JSON.parse(readFileSync(new URL('./digital-squirrel-brush.json',import.meta.url),'utf8')),a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
const bones=a.rig.bones.filter(b=>/^tail-[1-6]$/.test(b.id));assert.equal(bones.length,6)
assert.ok(Math.abs(bones.reduce((s,b)=>s+b.lengthM,0)-.175)<1e-8);assert.ok(Math.abs(bones.reduce((s,b)=>s+b.mass.kg,0)-.04)<1e-8)
const V=q=>new Vector3(q.x,q.y,q.z)
for(const p of a.poses){
 const fr=evaluateDrawing(a,{kind:'pose',pose:p.id}).bones
 for(let i=0;i<6;i++){
  const b=bones[i],{u,o}=fr[b.id].frame
  if(i<5){const next=bones[i+1];assert.equal(next.parent,b.id);assert.ok(V(o).addScaledVector(V(u),b.lengthM).distanceTo(V(fr[next.id].frame.o))<.00001)}
  if(p.id==='sitting-eating')assert.ok(Math.abs(Math.atan2(u.y,u.x)*180/Math.PI-f.seatedHeadingsDeg[i])<.02)
 }
}
for(const v of Object.values(a.volumes)){
 assert.equal(v.structure.filter(q=>q.id.startsWith('caudal-centrum-')).length,23)
 for(const b of bones){assert.ok(v.body.some(q=>q.id===b.id&&q.bone===b.id));assert.ok(v.thermal.some(q=>q.of===b.id&&q.region==='tail-fur'))}
}
console.log(JSON.stringify({centra:23,bones:6,lengthM:.175,massKg:.04,poses:a.poses.length}))
