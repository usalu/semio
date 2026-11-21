/** 🌿 Verify alternating hornbeam blades, moving leaves and cross-row wind. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,plant=JSON.parse(readFileSync(new URL('./digital-hornbeam-flush.json',import.meta.url),'utf8')).plants
for(const [scale,volume] of Object.entries(a.volumes)){
 const sprays=volume.body.filter(s=>s.id.startsWith('leaf-spray-'))
 assert.ok(sprays.length>=6,scale+' hand-shaped shoots')
 for(let i=0;i<plant.length;i++){
  const ids=sprays.filter(q=>q.bone==='block-'+(i+1))
  assert.ok(ids.length>=1)
  for(const q of ids)assert.ok(volume.thermal.some(s=>s.of===q.id&&s.region==='leaf-face'))
 }
 if(scale==='50')assert.ok(sprays.reduce((n,q)=>n+q.vertices.length,0)>15000)
}
const neutral=evaluateDrawing(a,{kind:'pose',pose:'calm'}).bones,wind=evaluateDrawing(a,{kind:'pose',pose:'windy'}).bones
for(let i=1;i<=6;i++){
 const end=states=>{const b=states['block-'+i],p=b.frame;return new Vector3(p.u.x,p.u.y,p.u.z).multiplyScalar(1.45).add(new Vector3(p.o.x,p.o.y,p.o.z))}
 const delta=end(wind).sub(end(neutral));assert.ok(Math.abs(delta.z)>.035&&Math.abs(delta.z)>Math.abs(delta.x))
}
