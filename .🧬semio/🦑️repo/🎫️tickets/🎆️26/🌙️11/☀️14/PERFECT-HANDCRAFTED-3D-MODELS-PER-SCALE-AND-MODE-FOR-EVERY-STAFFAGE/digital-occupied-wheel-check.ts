/** 🛞 Section the posed wheel meshes with circular cylinders and count the open-web intervals independently. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3,Triangle} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-occupied-wheel-case.json',import.meta.url),'utf8')),profile=f.subjects[a.id],TAU=2*Math.PI,v=p=>new Vector3(p.x,p.y,p.z)
function cuts(p,q,r){
 const dx=q.x-p.x,dy=q.y-p.y,A=dx*dx+dy*dy,B=2*(p.x*dx+p.y*dy),C=p.x*p.x+p.y*p.y-r*r,delta=B*B-4*A*C
 if(A<1e-18||delta<0)return []
 return [(-B+Math.sqrt(delta))/(2*A),(-B-Math.sqrt(delta))/(2*A)].filter(t=>t>=0&&t<=1).map(t=>Math.atan2(p.y+t*dy,p.x+t*dx))
}
function intervals(meshes,r){
 const N=1800,held=new Uint8Array(N)
 for(const {pts,idx} of meshes)for(let i=0;i<idx.length;i+=3){
  const p=pts[idx[i]],q=pts[idx[i+1]],s=pts[idx[i+2]],hit=[...cuts(p,q,r),...cuts(q,s,r),...cuts(s,p,r)]
  if(hit.length<2)continue
  const base=hit[0],rel=hit.map(x=>Math.atan2(Math.sin(x-base),Math.cos(x-base))),lo=(base+Math.min(...rel))/TAU,hi=(base+Math.max(...rel))/TAU
  for(let k=Math.ceil(lo*N-.5);k<=Math.floor(hi*N+.5);k++)held[(k+2*N)%N]=1
 }
 const groups=held.reduce((n,x,i)=>n+(x&&!held[(i+N-1)%N]?1:0),0),occlusion=held.reduce((n,v)=>n+v,0)/N
 return {groups,occlusion}
}
const radius=p=>Math.hypot(p.x,p.y)
function distance(pieces,points){
 let nearest=Infinity
 for(const item of pieces)for(let i=0;i<item.idx.length;i+=3){
  const t=new Triangle(item.pts[item.idx[i]],item.pts[item.idx[i+1]],item.pts[item.idx[i+2]])
  for(const p of points)nearest=Math.min(nearest,t.closestPointToPoint(p,new Vector3()).distanceTo(p))
 }
 return nearest
}
const issues=[],facts=[]
for(const pose of [a.presentation.defaultPose,profile.otherPose]){
 const b=evaluateDrawing(a,{kind:'pose',pose}).bones,m=posedMesh(a,pose)
 for(const d of f.scales){const vset=a.volumes[d],q=qualityOf(a,d)
  for(const [bone,tag,depth,R,count,width] of profile.wheels){
   const h=b[bone].frame,local=raw=>{const p=raw.clone().sub(v(h.o));return new Vector3(p.dot(v(h.u)),p.dot(v(h.v)),p.dot(v(h.w)))}
   const stock=vset.body.filter(s=>s.bone===bone).map(s=>{
    const g=m(s,q),pts=Array.from({length:g.world.length/3},(_,i)=>local(new Vector3().fromArray(g.world,i*3)));return {s,pts,idx:g.indices}
   }).filter(({pts})=>pts.some(p=>Math.abs(p.z-depth)<.052))
   const wire=s=>s.id.includes('spoke')||s.id.startsWith('web-'),wires=stock.filter(({s})=>wire(s)),hub=stock.filter(({s})=>s.id.includes('hub')),rim=stock.filter(({s})=>s.id.includes('rim')&&!s.id.startsWith('hand-'))
   const row=[.38,.46,.55,.62].map(t=>intervals(stock,R*t));const seen=Math.max(...row.map(r=>r.groups)),air=Math.min(...row.map(r=>r.occlusion))
   if(seen!==count||air>.65||air<.009||!hub.length||!rim.length)issues.push([pose,d,tag,seen,+air.toFixed(3),hub.length,rim.length])
   const tyre=stock.find(({s})=>s.id.includes('tyre')),gauges={cyclist:.040,'wheelchair-user':.0336,'person-with-rollator':.036}
   if(tyre){
    const zz=tyre.pts.map(p=>p.z),rr=Math.max(...tyre.pts.map(radius))
    if(Math.abs(Math.max(...zz)-Math.min(...zz)-(width??gauges[a.id]))>.002||Math.abs(rr-R)>.0025)issues.push([d,tag,'tyre section'])
   }else issues.push([d,tag,'tyre absent'])
   const directions=new Map()
   for(const item of wires){
    const vertices=item.pts.slice(0,item.pts.length/(item.s.repeat?.count??1)),rr=vertices.map(radius),lo=Math.min(...rr),hi=Math.max(...rr)
    const roots=vertices.filter(q=>radius(q)<lo+.0026),ends=vertices.filter(q=>radius(q)>hi-.0026)
    const mean=points=>points.reduce((sum,p)=>sum.add(p),new Vector3()).multiplyScalar(1/points.length),A=mean(roots),B=mean(ends)
    const flange=Math.sign(A.z-depth),sg=Math.sign(A.x*B.y-A.y*B.x);if(!directions.has(flange))directions.set(flange,new Set());directions.get(flange).add(sg)
    if(distance(hub,roots)>.006||distance(rim,ends)>.008)issues.push([d,item.s.id,'hub or rim seat'])
   }
   if(['cyclist','wheelchair-user'].includes(a.id)&&(!directions.get(1)?.has(1)||!directions.get(1)?.has(-1)||!directions.get(-1)?.has(1)||!directions.get(-1)?.has(-1)))issues.push([d,tag,'direction pair on each flange'])
   if(a.id==='wheelchair-user'&&!stock.some(({s})=>s.id==='hand-rim-tabs-'+tag))issues.push([d,tag,'pushrim tabs'])
   for(const {s} of [...wires,...hub,...rim])if(!vset.thermal.some(p=>p.of===s.id)||!vset.structure.some(p=>p.of===s.id))issues.push([d,s.id,'physical mode'])
   facts.push([pose,d,tag,seen,+air.toFixed(3)])
  }
 }
}
console.log(JSON.stringify({asset:a.id,sections:facts.length,sample:facts.slice(-profile.wheels.length),issues:issues.slice(0,6),total:issues.length}))
assert.equal(issues.length,0)
