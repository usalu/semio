/** 🔩 Check the scissor's platform perimeter and the motor scooter's double saddle using independently intersected bore rays. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {ShapeUtils,Vector2} from 'three'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,c=JSON.parse(readFileSync(new URL('./digital-vehicle-joint-cases.json',import.meta.url),'utf8'))[a.id]
for(const v of Object.values(a.volumes)){
 if(a.id==='scissor-lift'){
  for(const id of [...c.requiredTopRails,...c.requiredPosts])assert.ok(v.structure.some(s=>s.of===id),id)
 }else{
  const halves=v.body.filter(q=>q.id.startsWith('bar-saddle-'));assert.equal(halves.length,2)
  for(const q of halves){
   assert.ok(Math.abs(Math.abs(q.to-q.from)-c.clampWidthM)<1e-6)
   assert.ok(Math.abs(ShapeUtils.area(q.profile.map(p=>new Vector2(...p))))>.0004)
   assert.ok(q.profile.every(([x,y])=>Math.hypot(x,y)>=c.saddleBoreM/2-.000002))
   assert.ok(v.structure.some(s=>s.of===q.id));assert.ok(v.thermal.some(s=>s.of===q.id))
  }
  assert.equal(v.body.filter(q=>q.id.startsWith('saddle-pinch-')).length,c.pinchBolts)
 }
}
