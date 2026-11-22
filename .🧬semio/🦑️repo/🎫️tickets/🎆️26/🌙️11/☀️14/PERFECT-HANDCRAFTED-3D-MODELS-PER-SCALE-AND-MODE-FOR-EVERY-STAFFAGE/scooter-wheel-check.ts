/** 🛴 Check wheel dimensions, tyre running space and frame-bearing contacts against every posed road wheel. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Ray,Vector3} from 'three'
import {rigAt,meshesIn} from './r3-lib.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
const f=JSON.parse(readFileSync(new URL('./scooter-wheel-case.json',import.meta.url),'utf8')),fails=new Set<string>()
const check=(bool,msg)=>{if(!bool)fails.add(msg)}
assert.equal(a.poses.length,f.poseCount)
const v=p=>new Vector3(...p),up=new Vector3(.234,.613,.917).normalize(),hit=new Vector3()
const inside=(p,tris)=>{
 const ray=new Ray(p,up),cross=new Set()
 for(const t of tris)if(ray.intersectTriangle(...t,false,hit))cross.add(Math.round(hit.distanceTo(p)*1e8))
 return cross.size%2===1
}
let maxRoad=0,roadContacts=0
for(const k of f.scales){
 const vol=a.volumes[k],parts=vol.body.filter(s=>s.bone==='frame'&&/^(deck|strut-[lr]|fork(-[lr]|-crown)?|neck-arm|axle-(rear|front))$/.test(s.id))
 const neutral={o:[0,0,0],u:[1,0,0],v:[0,1,0],w:[0,0,1]}
 const mounts=meshesIn(a,{frame:neutral},parts,k).map((m,i)=>({id:parts[i].id,tris:Array.from({length:m.indices.length/3},(_,j)=>[0,1,2].map(n=>new Vector3().fromArray(m.world,m.indices[3*j+n]*3)))}))
 for(const end of f.wheelPositions){
  const x=end==='front'?f.axleSpacingM:0,id='tyre-'+end
  const tyre=vol.body.find(q=>q.id===id)
  const anchor=a.anchors.find(q=>q.id==='wheel-'+end+'-contact'),patch=vol.structure.find(q=>q.anchor===anchor.id)
  const spans=patch.profile.map(p=>p[0]),radius=Math.max(...patch.profile.map(p=>p[1]))
  check(Math.abs(Math.max(...spans)-Math.min(...spans)-anchor.footprintWidthM)<1e-7&&Math.abs(2*radius-anchor.footprintLengthM)<1e-7&&Math.abs(patch.origin[1]-radius)<1e-7,k+' support patch '+end)
  check(tyre.bone==='wheel-'+end&&tyre.axis.join(',')==='0,0,1',k+' wheel spin '+end)
  const spoke=vol.body.find(s=>s.id==='spoke-'+end),rim=vol.body.find(s=>s.id==='rim-'+end),hub=vol.body.find(s=>s.id==='hub-'+end)
  check(spoke?.repeat?.count===3&&Math.abs(spoke.repeat.rotationRad-Math.PI/3)<1e-6&&rim?.bone===tyre.bone&&hub?.bone===tyre.bone,k+' six-spoke wheel '+end)
  for(const hardware of [spoke,rim,hub].filter(Boolean)){
   check(vol.thermal.some(t=>t.of===hardware.id&&t.region==='wheels')&&vol.structure.some(t=>t.of===hardware.id&&t.member===tyre.bone),k+' wheel fields '+hardware.id)
  }
  const tread=tyre.profile.filter(p=>p[1]>=.6*f.radiusM)
  const width=Math.max(...tread.map(p=>p[0]))-Math.min(...tread.map(p=>p[0]))
  check(Math.abs(width-f.treadWidthM)<1e-6,k+' tread '+end+' '+width)
  check(Math.abs(Math.max(...tread.map(p=>p[1]))-f.radiusM)<1e-6,k+' radius '+end)
  for(const z of [-.013,0,.013])for(let i=0;i<32;i++){
   const angle=(i+.23)*Math.PI/16
   const p=new Vector3(x+.064*Math.cos(angle),.0725+.064*Math.sin(angle),z)
   for(const mount of mounts)if(inside(p,mount.tris))fails.add(k+' tread intrusion '+end+' '+mount.id)
  }
  for(const z of [-.022,.022]){
   const bore=new Vector3(x,.0725,z),pins=mounts.filter(q=>q.id==='axle-'+end),cheeks=mounts.filter(q=>q.id!=='axle-'+end)
   check(pins.some(q=>inside(bore,q.tris))&&cheeks.some(q=>inside(bore,q.tris)),k+' axle to dropout '+end+' '+z)
  }
  for(const part of [tyre,...parts]){
   check(vol.thermal.some(q=>q.of===part.id)&&vol.structure.some(q=>q.of===part.id),k+' mode '+part.id)
  }
  for(const pose of a.poses){
   const frames=rigAt(a,k,{kind:'pose',pose:pose.id}).frames,frame=frames.frame
   const ground=v(frame.o).addScaledVector(v(frame.u),x),axis=v(frames[tyre.bone].o)
   check(Math.abs(ground.y)<.001,k+' road plane '+pose.id)
   check(Math.abs(axis.distanceTo(ground)-f.radiusM)<1e-7,k+' pivot '+pose.id)
   const mesh=meshesIn(a,frames,[tyre],k)[0]
   let low=Infinity
   for(let i=1;i<mesh.world.length;i+=3)low=Math.min(low,mesh.world[i])
   maxRoad=Math.max(maxRoad,Math.abs(low-ground.y))
   check(Math.abs(low-ground.y)<f.permittedRoadErrorM,k+' tyre road '+pose.id)
   roadContacts++
  }
 }
}
console.log(JSON.stringify({maxRoadErrorM:maxRoad,roadContacts,issues:[...fails]}))
if(fails.size)process.exitCode=1
