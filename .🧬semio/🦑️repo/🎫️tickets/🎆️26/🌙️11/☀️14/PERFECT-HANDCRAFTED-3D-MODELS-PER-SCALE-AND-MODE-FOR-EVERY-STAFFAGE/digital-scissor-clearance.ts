/** 🔩 Independently probe all three bearing holes in each physical scissor arm. */
import {readFileSync} from 'node:fs'
import {Ray,Vector3} from 'three'
import assert from 'node:assert/strict'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
for(const v of Object.values(a.volumes))for(const b of a.rig.bones.filter(b=>/^[ab][1-4]$/.test(b.id))){
 const q=v.body.find(s=>s.id==='arm-'+b.id);assert.equal(q.kind,'polyhedron')
 const points=q.vertices.map(p=>new Vector3(...p)),at=(x,y)=>{const ray=new Ray(new Vector3(x,y,-1),new Vector3(0,0,1));return q.triangles.some(t=>ray.intersectTriangle(...t.map(i=>points[i]),false,new Vector3()))}
 for(const x of [0,.73,1.46]){assert.ok(!at(x,.02025));assert.ok(at(x,.026))}
}
