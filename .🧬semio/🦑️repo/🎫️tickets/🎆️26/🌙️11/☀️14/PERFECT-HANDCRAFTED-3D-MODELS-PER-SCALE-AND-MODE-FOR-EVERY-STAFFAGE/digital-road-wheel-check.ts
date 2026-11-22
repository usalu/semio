/** 🛞 Check the separate tyre bead, wheel centre, brake, structural load path and posed lateral track. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
const v1929=a.id==='vintage-car-1929',van=a.id==='delivery-van'
for(const [scale,v] of Object.entries(a.volumes)){
 const location=(part,c)=>v1929?part+'-'+c:c+'-'+part
 for(const c of ['fl','fr','rl','rr']){
  const tire=v.body.find(q=>q.id===location('tyre',c)),rim=v.body.find(q=>q.id===location('rim',c))
  assert.ok(tire&&rim,a.id+' '+scale+' '+c)
  assert.ok(Math.min(...tire.profile.map(p=>p[1]))>.16)
  assert.ok(Math.max(...rim.profile.map(p=>p[1]))>=Math.min(...tire.profile.map(p=>p[1])))
  assert.ok(new Vector3(...rim.origin).distanceTo(new Vector3(...tire.origin))<1e-6)
  for(const part of [location(v1929?'spokes':van?'disc-wheel':'spoke',c),location('hub',c)]){
   assert.ok(v.body.some(q=>q.id===part),'missing wheel centre '+part)
   assert.ok(v.structure.some(q=>q.of===part),'missing wheel load path '+part)
   assert.ok(v.thermal.some(q=>q.of===part&&q.region==='wheel-metal'),'missing wheel heat surface '+part)
  }
  assert.ok(v.thermal.some(q=>q.of===location('tyre',c)&&q.region==='tyres'))
 }
}
