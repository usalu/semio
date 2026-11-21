/** 👁 Measure both eyelid apertures, the sclera inside the lids and the pigment at the two working scales. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3,Ray} from 'three'
const a=JSON.parse(readFileSync(process.argv[2])).asset
const f=JSON.parse(readFileSync(new URL('./digital-commuter-orbits.json',import.meta.url)))
const eq=(x,y,t=1e-6)=>assert.ok(Math.abs(x-y)<t,`${x} vs ${y}`)
function hit(s,x,z){
 const ray=new Ray(new Vector3(x,-1,z),new Vector3(0,1,0)),p=new Vector3(),pts=s.vertices.map(v=>new Vector3(...v))
 let y=Infinity
 for(const [a,b,c] of s.triangles)if(ray.intersectTriangle(pts[a],pts[b],pts[c],false,p))y=Math.min(y,p.y)
 return y
}
for(const [scale,n] of Object.entries(f.scales)){
 const v=a.volumes[scale]
 const nose=v.body.find(s=>s.id==='head-nose'),lip=v.body.find(s=>s.id==='head-lips')
 assert.ok(nose?.kind==='polyhedron'&&lip?.kind==='polyhedron','Nasal fossae and mouth closure need modelled surfaces')
 for(const sign of [-1,1]){
  const floor=hit(nose,-.0035,sign*.0095),wing=hit(nose,-.0035,sign*.014)
  assert.ok(floor>wing+.0025,'Nasal fossa must recess behind the alar rim: '+[floor,wing])
 }
 const seam=hit(lip,-.0324,0)
 assert.ok(seam>hit(lip,-.0348,0)+.0004 && seam>hit(lip,-.0298,0)+.0004,'The lips must meet at a recessed oral seam')
 for(const side of ['l','r']){
  const find=id=>v.body.find(s=>s.id==='head-'+id+'-'+side)
  const lid=find('eyelid'),white=find('eye'),iris=find('iris'),pupil=find('pupil')
  assert.ok(lid&&white&&iris&&pupil,'Missing the physical eyelid aperture or pigmented ocular surface')
  const edges=new Map()
  for(const [a,b,c] of lid.triangles)for(const [x,y] of [[a,b],[b,c],[c,a]])edges.set(x+','+y,1+(edges.get(x+','+y)??0))
  for(const [edge,count] of edges){const [x,y]=edge.split(',');assert.equal(count,1);assert.equal(edges.get(y+','+x),1)}
  const z=(side==='l'?1:-1)*f.eyeSpacingM/2,x=.03658,inner=lid.vertices.slice(n,n*2)
  eq(Math.max(...inner.map(q=>q[2]))-Math.min(...inner.map(q=>q[2])),f.apertureWidthM)
  eq(Math.max(...inner.map(q=>q[0]))-Math.min(...inner.map(q=>q[0])),f.upperApertureM+f.lowerApertureM,.0004)
  eq(Math.max(...iris.vertices.map(q=>q[2]))-Math.min(...iris.vertices.map(q=>q[2])),f.irisDiameterM)
  eq(Math.max(...pupil.vertices.map(q=>q[2]))-Math.min(...pupil.vertices.map(q=>q[2])),f.pupilDiameterM)
  assert.equal(hit(lid,x,z),Infinity)
  assert.ok(hit(pupil,x,z)<hit(iris,x,z)&&hit(iris,x,z)<hit(white,x,z))
  assert.ok(hit(lid,x+.007,z)<Infinity&&hit(lid,x-.006,z)<Infinity)
  assert.ok([lid,white,iris,pupil].every(s=>s.bone==='head'&&v.thermal.some(h=>h.of===s.id&&h.region==='face-skin')))
 }
}
console.log('Paired apertures, surface order, closed lid edges and face-skin references passed at 1:50 and 1:100.')
