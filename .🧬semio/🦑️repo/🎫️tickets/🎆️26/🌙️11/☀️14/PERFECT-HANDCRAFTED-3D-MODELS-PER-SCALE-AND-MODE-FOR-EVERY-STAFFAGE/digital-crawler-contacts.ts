/** 🛤️ Verify the paired roller stations, rail window and yoke pin centre in the real wheel plane. */
import assert from 'node:assert/strict'
import {Vector2} from 'three'
import {readFileSync} from 'node:fs'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
for(const v of Object.values(a.volumes)){
 const shoes=v.body.find(s=>s.id==='track-shoes'),pin=v.body.find(s=>s.id==='idler-cross-pin')
 assert.equal(shoes?.holes.length,1);assert.equal(shoes.repeat.count,2);assert.ok(pin)
 for(const q of v.body.filter(s=>s.id.startsWith('idler-yoke-'))){
  const centre=q.holes[0].reduce((sum,p)=>sum.add(new Vector2(...p)),new Vector2()).multiplyScalar(1/q.holes[0].length)
  assert.ok(centre.distanceTo(new Vector2(pin.origin[0],pin.origin[1]))<.00001)
  assert.ok(Math.abs(new Vector2(...q.holes[0][0]).distanceTo(centre)-pin.profile[0][1]-.002)<.0001)
 }
 for(const side of ['l','r']){const id='roller-'+side,q=v.body.find(s=>s.id===id);assert.equal(q.repeat.count,8);assert.ok(v.structure.some(s=>s.of===id));assert.ok(v.thermal.some(s=>s.of===id))}
}
