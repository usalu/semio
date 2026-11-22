/** 🚜 Check the complete rolling and thermal load path through all forklift, mixer and scooter wheels. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
for(const v of Object.values(a.volumes)){
 const tyres=v.body.filter(q=>/^tyre-/.test(q.id)&&q.bone.startsWith('wheel-'))
 assert.equal(tyres.length,a.id==='concrete-mixer-truck'?12:a.id==='forklift'?4:2)
 for(const q of tyres){
  assert.ok(q.kind==='revolve');assert.ok(Math.min(...q.profile.map(x=>x[1]))>0,q.id)
  assert.ok(v.structure.some(s=>s.member===q.bone&&s.of===q.id))
  assert.ok(v.thermal.some(s=>s.region==='tyres'&&s.of===q.id))
  const rim=v.body.find(s=>s.id===q.id.replace('tyre-','rim-'))
  assert.ok(rim);assert.ok(new Vector3(...rim.origin).distanceTo(new Vector3(...q.origin))<.2)
  assert.ok(v.structure.some(s=>s.of===rim.id))
 }
}
