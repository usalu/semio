/** 🌀 Measures the moving and fixed components in the exported model coordinate system across every operating clip. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {insideAny} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const {asset}=JSON.parse(readFileSync(process.argv[2],'utf8')),pump=asset.id==='circulation-pump'
const off=evaluateDrawing(asset,{kind:'pose',pose:'off'}),ref=off.bones.rotor.frame
const coords=(v)=>[v.x,v.y,v.z]
const dot=(a,b)=>a.reduce((s,x,i)=>s+x*b[i],0)
const local=(p,f)=>['u','v','w'].map(k=>dot(p.map((x,i)=>x-coords(f.o)[i]),coords(f[k])))
const world=(p,f)=>coords(f.o).map((x,i)=>x+p.reduce((s,q,j)=>s+q*coords(f[['u','v','w'][j]])[i],0))
function points(s){const a=[];for(let i=0;i<s.positions.length;i+=3)a.push(Array.from(s.positions.slice(i,i+3)));return a}
const mm=x=>Math.round(1e6*x)/1000
for(const d of [50,100,200,500]){
 const base=worldSolids(asset,off.bones,d),fixed=base.filter(q=>q.bone!=='rotor'),by=Object.fromEntries(base.map(q=>[q.id,q]))
 const bl=base.filter(q=>pump?q.id==='impeller-eye':q.id.startsWith('blade'))
 const blade=bl.flatMap(q=>points(q)).map(p=>local(p,ref))
 const maxR=Math.max(...blade.map(p=>Math.hypot(p[0],p[1]))),lo=Math.min(...blade.map(p=>p[2])),hi=Math.max(...blade.map(p=>p[2]))
 const witness=blade.reduce((best,p)=>Math.hypot(...p.slice(0,2))>Math.hypot(...best.slice(0,2))?p:best,blade[0])
 const findings=[],r={asset:asset.id,d,tipRadiusMm:mm(maxR),axialMm:[mm(lo),mm(hi)]}
 if(!pump){
  if(by.core){
   const core=asset.volumes[d].body.find(q=>q.id==='core'),frame=off.bones[core.bone].frame;assert.equal(core.holes.length,37)
   let samples=0
   for(const hole of core.holes){
    const centre=hole.reduce((sum,p)=>[sum[0]+p[0]/hole.length,sum[1]+p[1]/hole.length],[0,0])
    for(let j=1;j<=11;j++){const p=world([...centre,core.from+(core.to-core.from)*j/12],frame);assert(!insideAny(base,p));samples++}
   }
   r.coreThroughSamples=samples
   const rev=t=>evaluateDrawing(asset,{kind:'clip',clip:'reverse-spin',timeS:t})
   assert(rev(2).bones.rotor.offset.angleDeg>rev(1).bones.rotor.offset.angleDeg)
   assert(rev(6).bones.rotor.offset.angleDeg<rev(5).bones.rotor.offset.angleDeg)
   assert.equal(rev(2).thermal.emitters['exhaust-out'],1);assert.equal(rev(6).thermal.emitters['supply-in'],1)
  }
  const support=points(by.strut).map(p=>local(p,ref)),near=Math.min(...support.map(p=>p[2]))
  r.strutGapMm=mm(near-hi)
  const ring=points(by.ring).map(p=>local(p,ref))
  const inner=Math.min(...ring.map(p=>Math.hypot(p[0],p[1])))
  const angles=[...new Set(ring.filter(p=>Math.abs(Math.hypot(p[0],p[1])-inner)<1e-5).map(p=>Math.round(1e6*Math.atan2(p[1],p[0]))/1e6))].sort((a,b)=>a-b)
  const step=Math.max(...angles.map((x,i)=>(i+1===angles.length?angles[0]+2*Math.PI:angles[i+1])-x))
  r.conservativeShroudGapMm=mm(inner*Math.cos(step/2)-maxR)
  if(r.strutGapMm<=0||r.conservativeShroudGapMm<=0)findings.push('impeller clearance')
 }else{
  const house=off.bones.housing.frame
  const probes=[]
  for(const side of [-1,1])for(let i=38;i<=104;i++)for(const dy of [-.009,0,.009])probes.push(world([.104+side*i/1000,dy,.035],house))
  for(let z=.035;z<.061;z+=.001)for(const x of [-.006,.006])probes.push(world([x,0,z],ref))
  for(let i=0;i<=38;i++){probes.push(world([-i/1000,0,.052-.017*(i/38)**2],ref));if(i>=28)probes.push(world([i/1000,0,.027+.008*((i-28)/10)**2],ref))}
  const land=world([.020,0,.0355],ref);assert(insideAny([by['suction-divider']],land));assert(!insideAny([by['suction-divider']],world([0,.005,.0355],ref)))
  r.waterSamples=probes.length
  const hits=probes.filter(p=>insideAny(base,p));if(hits.length)findings.push(['blocked water',hits.length,base.filter(s=>insideAny([s],hits[0])).map(s=>s.id),local(hits[0],house)])
  const start=Math.min(...points(by['neck-r']).map(p=>local(p,ref)[0]));r.pipeGapMm=mm(start-maxR);if(r.pipeGapMm<=0)findings.push('pipe interference')
 }
 const clipEvidence={}
 for(const clip of asset.clips){
  const times=[...new Set([...clip.keys.map(k=>k.t),...Array.from({length:43},(_,i)=>clip.durationS*(i+.23)/43)])].sort((a,b)=>a-b)
  let maximum=0
  for(const timeS of times){
   const state=evaluateDrawing(asset,{kind:'clip',clip:clip.id,timeS}),p=world(witness,state.bones.rotor.frame)
   for(const s of fixed)assert.deepEqual(state.bones[s.bone].frame,off.bones[s.bone].frame)
   maximum=Math.max(maximum,Math.hypot(...p.map((x,i)=>x-world(witness,ref)[i])))
  }
  if(maximum<.01)findings.push(['stationary clip',clip.id])
  const timeS=clip.durationS*.231
  const pose=evaluateDrawing(asset,{kind:'clip',clip:clip.id,timeS}),mesh=worldSolids(asset,pose.bones,d)
  for(const s of mesh){
   for(const index of [0,Math.floor(s.positions.length/6)*3,s.positions.length-3]){
    const p=Array.from(by[s.id].positions.slice(index,index+3)),rest=off.bones[s.bone].frame,posed=pose.bones[s.bone].frame
    const exp=world(local(p,rest),posed);assert(Math.hypot(...exp.map((x,i)=>x-s.positions[index+i]))<1e-8)
   }
  }
  clipEvidence[clip.id]=[times.length,mm(maximum)]
 }
 console.log(JSON.stringify({...r,clipEvidence,findings}));if(findings.length)process.exitCode=1
}
