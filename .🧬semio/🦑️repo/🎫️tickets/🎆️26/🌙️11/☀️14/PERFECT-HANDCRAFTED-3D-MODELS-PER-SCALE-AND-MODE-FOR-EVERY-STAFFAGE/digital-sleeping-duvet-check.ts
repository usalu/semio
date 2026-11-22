/** 🪡 Measure coverage and clearances against the full sleeping body using the independent Three.js triangle raycaster. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3,Raycaster,Mesh,MeshBasicMaterial,BufferGeometry,Float32BufferAttribute,DoubleSide} from 'three'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-sleeping-duvet-case.json',import.meta.url),'utf8'))
const ray=new Raycaster(),down=new Vector3(0,-1,0),mat=new MeshBasicMaterial({side:DoubleSide}),errors=[]
function geometry(m){const g=new BufferGeometry();g.setAttribute('position',new Float32BufferAttribute(m.world,3));g.setIndex(Array.from(m.indices));g.computeBoundingBox();g.computeBoundingSphere();return new Mesh(g,mat)}
function height(xs,x,z){ray.set(new Vector3(x,2,z),down);const hits=ray.intersectObjects(xs,false);return hits.length?hits[0].point.y:-1}
for(const pose of f.poses){
 const region=a.poses.find(q=>q.id===pose).thermal.regions['quilt-foot']
 assert(region&&Math.abs(region.contact.tempC-(pose==='curled'?24:29))<.01)
 const mesh=posedMesh(a,pose),q=qualityOf(a,50),anatomy=a.volumes[50].body.filter(s=>s.material!=='duvet-cover'&&f.bodyBones.includes(s.bone)).map(s=>geometry(mesh(s,q)))
 for(const scale of f.scales){const vol=a.volumes[scale],cover=vol.body.filter(s=>s.material==='duvet-cover'&&s.id!=='pillow').map(s=>geometry(mesh(s,qualityOf(a,scale))))
  for(const anchor of a.anchors.filter(q=>q.id.startsWith('quilt-bearing-'))){
   assert(a.structure.supports.includes(anchor.id))
   const seat=vol.structure.find(q=>q.anchor===anchor.id)
   assert(seat&&Math.abs(seat.transform.translation[1]-seat.size[1]/2)<1e-6)
  }
  assert.equal(a.anchors.filter(q=>q.id.startsWith('quilt-bearing-')).length,4)
  for(const panel of vol.body.filter(s=>s.id.startsWith('baffle-'))){
   const thermal=vol.thermal.find(s=>s.of===panel.id)
   assert(thermal&&['quilt-chest','bottom','duvet-edge','quilt-foot'].includes(thermal.region))
  }
  let onBody=0,sheltered=0,holes=0,empty=0,contact=0;const heights=[]
  for(let x=.622;x<=1.81;x+=.031)for(let z=-.47;z<=.47;z+=.031){
   const tissue=height(anatomy,x,z),quilt=height(cover,x,z)
   if(quilt<.42)holes++
   if(tissue>.475){onBody++;if(quilt>=tissue-.004)sheltered++;if(quilt>=tissue-.004&&quilt<=tissue+f.contactRiseM[1])contact++}
   if(pose==='curled'&&x>1.68&&Math.abs(z)<.24){empty++;heights.push(quilt)}
  }
  const coverage=sheltered/onBody,close=contact/onBody,emptyFoot=heights.length?Math.max(...heights)-f.bedHeightM:0
  console.log(JSON.stringify({pose,scale,coverage:+coverage.toFixed(4),close:+close.toFixed(4),holes,emptyFoot:+emptyFoot.toFixed(4)}))
  if(coverage<f.minCoveredFraction||close<.77||holes||emptyFoot>.092)errors.push([pose,scale])
  for(const x of cover)x.geometry.dispose()
 }
 for(const x of anatomy)x.geometry.dispose()
}
for(const t of f.turnOverSeconds){
 const scale=500,model=posedMesh(a,{kind:'clip',clip:'turn-over',timeS:t})
 const tissue=a.volumes[50].body.filter(s=>s.material!=='duvet-cover'&&f.bodyBones.includes(s.bone)).map(s=>geometry(model(s,qualityOf(a,50))))
 const quilt=a.volumes[scale].body.filter(s=>s.material==='duvet-cover'&&s.id!=='pillow').map(s=>geometry(model(s,qualityOf(a,scale))))
 let on=0,covered=0,holes=0
 for(let x=.622;x<=1.81;x+=.039)for(let z=-.47;z<=.47;z+=.039){
  const body=height(tissue,x,z),top=height(quilt,x,z)
  if(top<.42)holes++
  if(body>.475){on++;if(top>=body-.004)covered++}
 }
 const fraction=covered/on
 console.log(JSON.stringify({timeS:t,scale,coverage:fraction,holes}))
 if(fraction<f.minCoveredFraction||holes)errors.push(['turn-over',t,fraction,holes])
 for(const part of [...tissue,...quilt])part.geometry.dispose()
}
assert.deepEqual(errors,[])
