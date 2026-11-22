/** 🌱 Verify timber sizes, all face contacts, clear cells and structural and thermal bindings. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Box3,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./ivy-botanical-case.json',import.meta.url),'utf8')),epsilon=1e-7
const close=(l,r)=>assert.ok(Math.abs(l-r)<epsilon),box=(s,i=0)=>new Box3().setFromCenterAndSize(new Vector3(...s.transform.translation).addScaledVector(new Vector3(...(s.repeat?.translation??[0,0,0])),i),new Vector3(...s.size))
for(const k of f.scales){
 const v=a.volumes[k],groups=[]
 for(const [name,rule,dimension] of [['lath-v',f.vertical,1],['lath-h',f.horizontal,0]]){
  const q=v.body.find(s=>s.id===name);assert.equal(q?.bone,'panel');assert.equal(q.material,'timber-c24');assert.equal(q.repeat.count,rule.count)
  close(q.size[dimension],rule.lengthM);close(q.size[1-dimension],f.widthM);close(q.size[2],f.depthM);close(q.repeat.translation[1-dimension],rule.pitchM)
  assert.ok(v.structure.some(s=>s.of===q.id&&s.member==='panel'));assert.ok(v.thermal.some(s=>s.of===q.id&&s.region==='trellis-timber'))
  groups.push(Array.from({length:rule.count},(_,i)=>box(q,i)))
 }
 const [vertical,horizontal]=groups
 for(const l of vertical)for(const h of horizontal){close(l.max.z,h.min.z);assert.ok(l.intersectsBox(h))}
 for(const l of vertical)for(const id of ['rail-top','rail-bottom'])assert.ok(l.intersectsBox(box(v.body.find(s=>s.id===id))))
 let openings=0
 for(let i=1;i<vertical.length;i++)for(let j=1;j<horizontal.length;j++){
  close(vertical[i].min.x-vertical[i-1].max.x,.18);close(horizontal[j].min.y-horizontal[j-1].max.y,.16875);openings++
 }
 assert.ok(openings>=f.minOpenings)
 for(const pose of f.poses){const state=evaluateDrawing(a,{kind:'pose',pose});assert.ok(state.bones.panel.frame)}
 console.log(JSON.stringify({scale:k,contacts:vertical.length*horizontal.length,interiorOpenings:openings,clearWidthM:.18,clearHeightM:.16875}))
}
