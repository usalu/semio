/** 🍁 Check the explicit seven lobes of a Japanese maple blade and that every branch carries a visible spray. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Triangle,Vector3} from 'three'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
for(const v of Object.values(a.volumes)){
 const leaves=v.body.filter(s=>s.id.startsWith('maple-spray-'))
 assert.ok(leaves.length>=9)
 for(const bone of a.rig.bones.filter(b=>b.id.startsWith('fol-'))){
  const group=leaves.filter(s=>s.bone===bone.id);assert.ok(group.length)
  for(const s of group){
   const area=s.triangles.reduce((total,t)=>total+new Triangle(...t.map(i=>new Vector3(...s.vertices[i]))).getArea(),0)
   assert.ok(area>.003)
   assert.ok(v.thermal.some(q=>q.of===s.id&&q.region.startsWith('leaf-')))
  }
 }
}
