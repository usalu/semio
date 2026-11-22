/** 🚲 Check the lacing planes, wheel supports, nipple passages and bearing seats against the wheel specification. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Ray,Vector3} from 'three'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-cycle-wheel-case.json',import.meta.url),'utf8'))[a.id]
for(const v of Object.values(a.volumes))for(const end of ['f','r']){
 const b='wheel-'+end,stock=v.body.filter(q=>q.bone===b)
 if(f.rearWindows){
  assert.ok(stock.some(q=>q.id==='tyre-'+end))
  if(end==='r'){assert.equal(stock.find(q=>q.id==='web-r')?.holes.length,f.rearWindows);assert.equal(stock.filter(q=>q.id.startsWith('bearing-r-')).length,2)}
  continue
 }
 const wires=stock.filter(q=>q.id.startsWith('spoke-')),fittings=stock.filter(q=>q.id.startsWith('nipple-'))
 assert.equal(wires.reduce((s,q)=>s+q.repeat.count,0),f.spokesPerWheel);assert.equal(fittings.length,4)
 for(const q of wires){
  assert.equal(q.kind,'polyhedron');assert.ok(v.thermal.some(n=>n.of===q.id));assert.ok(v.structure.some(n=>n.of===q.id))
  const nipple=fittings.find(n=>n.id===q.id.replace('spoke-','nipple-'));assert.ok(nipple)
  const source=nipple.vertices.map(p=>new Vector3(...p)),N=8
  const center=start=>source.slice(start,start+N).reduce((v,p)=>v.add(p),new Vector3()).multiplyScalar(1/N)
  const A=center(0),B=center(source.length-N),dir=B.clone().sub(A).normalize(),ray=new Ray(A.clone().addScaledVector(dir,-.01),dir)
  assert.ok(!nipple.triangles.some(t=>ray.intersectTriangle(...t.map(i=>source[i]),false,new Vector3())),nipple.id)
  assert.ok(Math.abs(B.distanceTo(A)-f.nippleM)<.0001)
 }
}
