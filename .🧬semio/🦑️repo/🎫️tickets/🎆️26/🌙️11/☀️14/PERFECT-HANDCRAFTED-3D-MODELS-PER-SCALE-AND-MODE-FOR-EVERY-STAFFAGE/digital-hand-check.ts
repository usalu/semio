/** ☝️ Independently measure wrist and digit attachments, ray containment and directional fingertip travel on the standing adult. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Vector3,Mesh,MeshBasicMaterial,BufferGeometry,Float32BufferAttribute,Raycaster,DoubleSide} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts'
import {projectWorld} from './volumes-base.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-hand-case.json',import.meta.url),'utf8'))
const bones=new Map(a.rig.bones.map(b=>[b.id,b])),v=p=>new Vector3(p.x,p.y,p.z)
const ids=name=>(name==='thumb'?['metacarpal','proximal','distal']:['proximal','middle','distal']).map(k=>name+'-'+k+'-r')
function inside(p,points){
 let crossings=0
 for(const [i,a] of points.entries()){
  const b=points[(i+1)%points.length]
  if((a.y>p.y)!==(b.y>p.y)&&p.x<a.x+(p.y-a.y)*(b.x-a.x)/(b.y-a.y))crossings++
 }
 return crossings%2===1
}
let drawings=0
const kg=bones.get(f.wrist).mass.kg+Object.keys(f.digits).flatMap(ids).reduce((s,id)=>s+(bones.get(id)?.mass.kg??0),0)
assert(Math.abs(kg-f.massKg)<1e-7)
for(const pose of a.poses){
 const state=evaluateDrawing(a,{kind:'pose',pose:pose.id}).bones,hand=state[f.wrist],h=hand.frame
 assert(v(h.o).distanceTo(v(state['forearm-r'].tip))<1e-6)
 const reach={}
 for(const [name,ref] of Object.entries(f.digits)){
  const chain=ids(name),origin=v(h.o).addScaledVector(v(h.u),ref.baseMm[0]/1000).addScaledVector(v(h.v),ref.baseMm[1]/1000).addScaledVector(v(h.w),ref.baseMm[2]/1000)
  let prev=origin
  for(const [i,id] of chain.entries()){
   const b=bones.get(id);assert(b,pose.id+' '+id)
   assert.equal(b.parent,i===0?f.wrist:chain[i-1])
   const fr=state[id].frame,end=v(state[id].tip)
   assert(v(fr.o).distanceTo(prev)<1e-6);assert(Math.abs(end.distanceTo(prev)*1000-ref.lengthMm[i])<.001)
   prev=end
  }
  reach[name]=+(prev.sub(origin).dot(v(h.u))*1000).toFixed(1)
 }
 if(pose.id==='pointing'){assert(reach.index>75);for(const k of ['middle','ring','little'])assert(reach[k]<10)}
 if(pose.id==='hands-warming'){for(const k of ['index','middle','ring','little'])assert(reach[k]>.94*f.digits[k].lengthMm.reduce((a,b)=>a+b))}
 for(const scale of [50,100,200,500]){
  const draw=evaluateDrawing(a,{kind:'pose',pose:pose.id},{kind:'scale',scale:{denominator:scale}})
  for(const name of Object.keys(f.digits))for(const id of ids(name)){
   const mid=v(state[id].frame.o).lerp(v(state[id].tip),.5).toArray()
   for(const view of ['elevation','front','plan']){
    const shape=draw.shapes[view].find(s=>s.shape.id==='skin-'+id)
    assert(shape?.rings.some(r=>inside(projectWorld(view,mid),r)),[pose.id,scale,view,id].join('/'));drawings++
   }
  }
 }
 console.log(JSON.stringify({pose:pose.id,axialReachMm:reach}))
}
let cuts=0
for(const [scale,vol] of Object.entries(a.volumes)){
 const q=qualityOf(a,+scale)
 const cache=new Map()
 function surface(s){
  if(!cache.has(s)){
   const t=tessellateSolid({...s,node:s.bone},q),g=new BufferGeometry();g.setAttribute('position',new Float32BufferAttribute(t.positions,3));g.setIndex(Array.from(t.indices));cache.set(s,new Mesh(g,new MeshBasicMaterial({side:DoubleSide})))
  }
  return cache.get(s)
 }
 const section=(s,x)=>new Raycaster(new Vector3(x,0,0),new Vector3(0,1,0)).intersectObject(surface(s))[0]?.distance
 function contains(s,p){
  const hits=new Raycaster(p,new Vector3(.417,.814,.405).normalize()).intersectObject(surface(s))
  return hits.filter((h,i)=>i===0||h.distance-hits[i-1].distance>1e-6).length%2===1
 }
 const hand=vol.body.find(s=>s.id==='hand-r'),fore=vol.body.find(s=>s.id===(scale==='50'?'cuff-rib-r':'forearm-r'))
 assert(contains(hand,new Vector3())&&contains(fore,new Vector3(.247,0,0)),'wrist surfaces are separated')
 for(const [digit,ref] of Object.entries(f.digits))for(const [i,id] of ids(digit).entries()){
  const skin=vol.body.find(s=>s.bone===id),bone=vol.structure.find(s=>s.member===id&&s.part==='member');assert(skin&&bone)
  const at=ref.lengthMm[i]/2000,outer=section(skin,at),inner=section(bone,at)
  assert(outer>inner&&inner>.002);assert(vol.structure.some(s=>s.member===id&&s.part==='joint'))
  assert(contains(skin,new Vector3()),id+' skin omits proximal joint')
  if(i){const prev=vol.body.find(s=>s.bone===ids(digit)[i-1]);assert(contains(prev,new Vector3(ref.lengthMm[i-1]/1000,0,0)),id+' skin omits distal joint')}
  else assert(contains(hand,new Vector3(...ref.baseMm.map(x=>x/1000))),id+' skin misses palm')
  assert(vol.thermal.some(s=>s.of===skin.id&&s.region==='hand-skin'));cuts++
 }
}
console.log(JSON.stringify({sectionTests:cuts,projectedMidpoints:drawings,handMassKg:kg}))
