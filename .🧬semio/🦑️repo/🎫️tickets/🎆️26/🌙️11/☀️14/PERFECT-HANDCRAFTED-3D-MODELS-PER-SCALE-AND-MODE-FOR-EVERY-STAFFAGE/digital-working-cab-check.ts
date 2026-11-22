/** 🚚 Trace the forward and transverse sightlines of the truck cab and count the excavator's fitted cutting teeth. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3,Ray} from 'three'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-working-cab-cases.json',import.meta.url),'utf8'))[a.id]
for(const v of Object.values(a.volumes)){
 if(a.id==='excavator'){
  for(const id of [...f.cheeks,f.shell,f.cuttingLip])assert.ok(v.structure.some(q=>q.of===id),id)
  assert.equal(v.body.find(q=>q.id==='bucket-tooth')?.repeat?.count,f.teeth)
  continue
 }
 const body=v.body.find(q=>q.id==='cab-shell');assert.equal(body.kind,'polyhedron')
 const rayHits=(q,from,axis)=>{
  const vertices=q.vertices.map(v=>new Vector3(...v)),ray=new Ray(new Vector3(...from),new Vector3(...axis))
  return q.triangles.some(([a,b,c])=>ray.intersectTriangle(vertices[a],vertices[b],vertices[c],false,new Vector3()))
 }
 for(const [pane,from,axis] of [['windscreen',f.frontSightline,[1,0,0]],['door-glass-l',f.sideSightline,[0,0,1]],['door-glass-r',f.sideSightline,[0,0,-1]]]){
  assert.ok(!rayHits(body,from,axis),pane);assert.ok(rayHits(v.body.find(q=>q.id===pane),from,axis),pane)
 }
 assert.ok(rayHits(body,[3.87,1.87,0],[0,0,1]))
 assert.ok(!rayHits(body,[4.2,1.506,0],[1,0,0]))
 assert.ok(v.structure.some(q=>q.of==='cab-shell'));assert.ok(v.body.some(q=>q.id==='driver-seat'))
 assert.equal(v.body.filter(q=>q.id.startsWith('grille-bar-')).length,f.grilleSlats)
}
