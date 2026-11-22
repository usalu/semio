/** 🌧️ Measure the wet fabric in each native pose and each intermediate closing frame. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Vector3} from 'three'
import {projectWorld} from './volumes-base.ts'
import {evaluate,evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
const f=JSON.parse(readFileSync(new URL('./digital-umbrella-gore-case.json',import.meta.url),'utf8'))
const v=(p)=>new Vector3(...(Array.isArray(p)?p:[p.x,p.y,p.z]))
const local=(frame,x,y,z)=>v(frame.o).addScaledVector(v(frame.u),x).addScaledVector(v(frame.v),y).addScaledVector(v(frame.w),z)
function inside(point,ring){
 let odd=false
 for(const [i,q] of ring.entries()){const r=ring[(i+1)%ring.length];if((q.y>point.y)!==(r.y>point.y)&&point.x<(r.x-q.x)*(point.y-q.y)/(r.y-q.y)+q.x)odd=!odd}
 return odd
}
const errors=[],check=(yes,why)=>{if(!yes)errors.push(why)}
for(const scale of f.scales){
 const vol=a.volumes[scale],panels=vol.body.filter(p=>p.material==='umbrella-fabric')
 const results={}
 for(const pose of ['umbrella-up-still','umbrella-furled']){
  const bones=evaluate(a,{kind:'pose',pose}).bones,shaft=bones.umbrella.frame,apex=v(bones['rib-front'].frame.o),axis=v(shaft.u)
  const mesh=posedMesh(a,pose)
  let radius=0
  for(const panel of panels){const {world}=mesh(panel,qualityOf(a,scale));for(let i=0;i<world.length;i+=3){const d=new Vector3(world[i],world[i+1],world[i+2]).sub(apex);radius=Math.max(radius,d.addScaledVector(axis,-d.dot(axis)).length())}}
  results[pose]=+(radius*2).toFixed(5)
 }
 console.log(scale,results)
 check(results['umbrella-furled']<=f.closedEnvelopeMaxM,scale+' folded width')
 check(results['umbrella-up-still']>=f.openDiameterM[0]&&results['umbrella-up-still']<=f.openDiameterM[1],scale+' open diameter')
 const folds=a.rig.radialFolds??[];check(folds.length===f.sectors,'eight complete gores');if(!folds.length)continue
 const clip=a.clips.find(c=>c.id==='close-umbrella')
 const states=[...a.poses.map(p=>({kind:'pose',pose:p.id})),...Array.from({length:f.closingSamples},(_,i)=>({kind:'clip',clip:clip.id,timeS:i*clip.durationS/(f.closingSamples-1)}))]
 for(const state of states){
  const b=evaluate(a,state).bones
  const drawing=state.kind==='pose'?evaluateDrawing(a,state,{kind:'scale',scale:{denominator:scale}}):null
  let area=0,maxGap=0,maxTipGap=0
  const ordered=folds.flatMap(g=>g.facets.map(id=>panels.find(p=>p.bone===id)))
  for(const [i,p] of ordered.entries()){
   const next=ordered[(i+1)%ordered.length]
   const face=(q)=>q.profile.map(([z,x])=>local(b[q.bone].frame,x,(q.from+q.to)/2,z))
   const points=face(p),tip=points.at(-1),start=points[1],origin=points[0]
   maxGap=Math.max(maxGap,tip.distanceTo(face(next)[1]),origin.distanceTo(face(next)[0]))
   if(drawing){
    const mid=origin.clone().add(start).add(tip).multiplyScalar(1/3)
    for(const view of ['elevation','front','plan']){
     const shape=drawing.shapes[view].find(s=>s.shape.id===p.id)
     if(!shape?.rings.some(r=>inside(projectWorld(view,mid.toArray()),r)))check(false,scale+' '+state.pose+' '+view+' '+p.id+' centroid outside drawing')
    }
   }
   const binding=vol.structure.find(s=>s.member===p.bone&&s.part==='member')
   check(binding&&local(b[p.bone].frame,...binding.p0).distanceTo(start)<1e-6&&local(b[p.bone].frame,...binding.p1).distanceTo(tip)<1e-6,'sewn hem attachment')
   const crease=folds.find(g=>g.facets[0]===p.bone)
   if(crease){const rib=vol.structure.find(q=>q.member===crease.left&&q.part==='member');check(!!rib,'missing load member '+crease.left)
    maxTipGap=Math.max(maxTipGap,start.distanceTo(local(b[crease.left].frame,.56,0,0)))}
   for(let k=2;k<points.length;k++)area+=points[k-1].clone().sub(origin).cross(points[k].clone().sub(origin)).length()/2
   check(Math.abs(p.to-p.from-f.fabricGaugeM)<1e-8&&vol.thermal.some(q=>q.of===p.id&&q.region==='canopy'&&q.layer==='pongee'),'cloth gauge or thermal surface')
  }
  check(Math.abs(2*area-a.thermal.regions.find(r=>r.id==='canopy').areaM2)<1e-7&&area>=f.cutAreaM2[0]&&area<=f.cutAreaM2[1]&&maxGap<=f.seamGapMaxM&&maxTipGap<=f.ribHemGapMaxM,JSON.stringify({scale,state,area,maxGap,maxTipGap}))
 }
}
assert.deepEqual([...new Set(errors)].slice(0,8),[])
