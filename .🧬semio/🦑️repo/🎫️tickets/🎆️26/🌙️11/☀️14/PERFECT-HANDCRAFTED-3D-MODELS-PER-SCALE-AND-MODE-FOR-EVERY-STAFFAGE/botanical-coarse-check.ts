/** 🌿 Measure the internals and scale reductions in the four follow-up plant models. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Vector3,Box3} from 'three'
const [path,subject]=process.argv.slice(2),a=JSON.parse(readFileSync(path,'utf8')).asset,f=JSON.parse(readFileSync(new URL('./botanical-coarse-case.json',import.meta.url),'utf8'))[subject],v=a.volumes[500]
const diam=q=>{const n=q.triangles[0][2],p=q.vertices.map(x=>new Vector3(...x)),w=[]
 for(let j=0;j<p.length-2;j+=n){const r=p.slice(j,j+n);w.push(Math.max(...r.flatMap(a=>r.map(b=>a.distanceTo(b)))))}
 return Math.max(...w)}
const extent=q=>new Box3().setFromPoints(q.vertices.map(p=>new Vector3(...p))).getSize(new Vector3()).length()
if(subject==='agave'){
 const ribs=v.structure.filter(s=>s.id.startsWith('m-leaf-')),w=ribs.map(diam)
 assert.equal(ribs.length,f.members);assert.equal(new Set(ribs.map(q=>q.bone.split('-')[1])).size,f.leaves)
 assert.ok(Math.max(...w)<=f.maximumMemberDiameterM&&Math.max(...w)-Math.min(...w)>f.minimumDiameterVariationM)
 for(const r of ribs)assert.ok(diam(v.body.find(q=>q.id===r.bone))>diam(r))
 console.log(JSON.stringify({subject,members:ribs.length,diameterM:[Math.min(...w),Math.max(...w)]}))
}
if(subject==='birch'){
 const masses=v.structure.filter(s=>s.part==='mass')
 const stations=masses.filter(m=>m.member.startsWith('fol-')&&m.kind==='polyhedron')
 assert.ok(stations.length>=f.distributedStations);assert.ok(masses.length<=f.maximumTotalStations)
 const d=stations.map(extent);assert.ok(Math.max(...d)<=f.maximumStationDiameterM+1e-5)
 console.log(JSON.stringify({subject,stations:stations.length,maximumM:Math.max(...d),members:[...new Set(stations.map(q=>q.member))].length}))
}
if(subject==='miscanthus'){
 const leaves=v.body.filter(q=>q.id.startsWith('blade-')),widths=leaves.map(diam),panicles=v.body.filter(q=>q.id.startsWith('plume-'))
 assert.ok(leaves.length>=f.minimumBlades&&Math.max(...widths)<=f.maximumBladeWidthM&&panicles.length>=f.minimumPanicleFingers)
 assert.ok(!v.body.some(q=>q.id.startsWith('lobe-'))&&!v.structure.some(q=>q.of?.startsWith('lobe-')))
 console.log(JSON.stringify({subject,leaves:leaves.length,panicleFingers:panicles.length,widthM:[Math.min(...widths),Math.max(...widths)]}))
}
if(subject==='apple'){
 const foliage=v.body.filter(q=>q.material?.startsWith('apple-leaf')),joints=new Set(foliage.map(q=>q.bone)),fruit=v.body.filter(q=>q.material==='apple-fruit'),scaffold=v.body.filter(q=>q.material==='apple-bark-young')
 assert.ok(joints.size>=f.minimumCanopyJoints&&fruit.length>=f.minimumApples&&scaffold.length>=f.minimumScaffolds)
 for(const q of [...foliage,...fruit,...scaffold])assert.ok(v.thermal.some(s=>s.of===q.id))
 console.log(JSON.stringify({subject,canopyJoints:joints.size,fruit:fruit.length,scaffolds:scaffold.length,foliageClusters:foliage.length}))
}
