/** 🛞 Inspect the wheel seats, bearing clearances and pan attachments on the actual tessellated heater hardware. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {BufferGeometry,Float32BufferAttribute,Mesh,MeshBasicMaterial,Raycaster,Vector3,DoubleSide} from 'three'
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-heater-wheel-case.json',import.meta.url),'utf8'))
const agrees=(a,b,tolerance=.0005)=>assert.ok(Math.abs(a-b)<tolerance,JSON.stringify({a,b,tolerance}))
for(const [d,v] of Object.entries(a.volumes)){
 const mesh=id=>{
  const s=v.body.find(s=>s.id===id);assert(s,d+' '+id)
  const m=tessellateSolid({...s,node:s.bone},qualityOf(a,+d)),g=new BufferGeometry()
  g.setAttribute('position',new Float32BufferAttribute(m.positions,3));g.setIndex(Array.from(m.indices))
  return new Mesh(g,new MeshBasicMaterial({side:DoubleSide}))
 }
 const hit=(m,p,dir)=>new Raycaster(new Vector3(...p),new Vector3(...dir)).intersectObject(m)[0]?.distance
 const shaft=mesh('axle'),pan=mesh(+d<=100?'dish':'housing')
 const [x,y]=f.axisOriginM
 for(const [i,side] of ['left','right'].entries()){
  const z=f.wheelCenterZM[i],bz=f.bracketCenterZM[i],tyre=mesh('tyre-'+side),hub=mesh('hub-'+side),plate=mesh('bracket-'+side),bush=mesh('bush-'+side)
  const radial=(m,z)=>hit(m,[x,y,z],[1,0,0])
  agrees(radial(hub,z),f.clearanceRadiusM)
  agrees(radial(shaft,z),f.shaftRadiusM)
  agrees(radial(tyre,z),f.tyreSeatM,.0015)
  agrees(hit(tyre,[x+.08,y,z],[-1,0,0]),.08-f.tyreOuterM,.001)
  agrees(radial(plate,bz),f.bushOuterM,.001)
  agrees(radial(bush,bz),f.clearanceRadiusM)
  assert.equal(hit(plate,[x,y,-.3],[0,0,1]),undefined)
  assert.equal(hit(hub,[x,y,-.3],[0,0,1]),undefined)
  assert.equal(hit(bush,[x,y,-.3],[0,0,1]),undefined)
  const mount=[.026,.195,bz]
  const fromPlate=hit(plate,mount,[0,0,1]),toPan=hit(pan,mount,[0,1,0])
  agrees(fromPlate,f.bracketHalfWidthM)
  assert(toPan>.005&&toPan<.05,'bracket root must intersect the base pan')
  agrees(-.05+hit(tyre,[-.05,y,z],[1,0,0]),f.parkedClearanceM,.001)
  agrees(hit(tyre,[x+.034,y,z-.05],[0,0,1]),.05-f.tyreWidthM/2)
  agrees(hit(tyre,[x+.034,y,z+.05],[0,0,-1]),.05-f.tyreWidthM/2)
  for(const part of ['tyre-'+side,'hub-'+side,'bracket-'+side,'bush-'+side,'axle']){
   assert(v.thermal.some(s=>s.of===part&&s.region==='base-housing'),d+' thermal '+part)
   assert(v.structure.some(s=>s.of===part&&s.member==='base'),d+' structure '+part)
  }
 }
 console.log(JSON.stringify({scale:d,gaugeM:f.wheelCenterZM[0]-f.wheelCenterZM[1],shaftDiameterM:2*f.shaftRadiusM,bearingBoreM:2*f.clearanceRadiusM,parkedClearanceM:f.parkedClearanceM}))
}
