/** 🪽 Section the posed, tessellated vanes through the carpal seam and measure the shortest edge gap. */
import {readFileSync} from 'node:fs'
import {Line3,Plane,Ray,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset, f=JSON.parse(readFileSync(new URL('./gull-touch-case.json',import.meta.url),'utf8')),V=p=>new Vector3(p.x,p.y,p.z),bad=[]
const cut=(mesh,plane)=>{
 const lines=[],vertices=[];for(let i=0;i<mesh.positions.length;i+=3)vertices.push(new Vector3().fromArray(mesh.positions,i))
 for(let i=0;i<mesh.indices.length;i+=3){
  const points=[],v=mesh.indices.slice(i,i+3)
  for(let j=0;j<3;j++){const hit=plane.intersectLine(new Line3(vertices[v[j]],vertices[v[(j+1)%3]]),new Vector3());if(hit)points.push(hit)}
  if(points.length===2)lines.push(points)
 }
 return lines
}
function distance(l,r){
 let min=Infinity
 for(const [a,b] of l)for(const [c,d] of r){const delta=b.clone().sub(a);if(delta.lengthSq()<1e-14)continue
  const ray=new Ray(a,delta.clone().normalize()),p=new Vector3(),q=new Vector3()
  const d2=ray.distanceSqToSegment(c,d,p,q)
  min=Math.min(min,p.distanceToSquared(a)<=delta.lengthSq()+1e-14?Math.sqrt(Math.max(0,d2)):new Line3(c,d).closestPointToPoint(b,true,q).distanceTo(b))
 }
 return min
}
for(const k of f.scales)for(const pose of f.poses){
 const bones=evaluateDrawing(a,{kind:'pose',pose}).bones,vol=a.volumes[k],ids=new Set(['near-primary-1','far-primary-1','near-secondary-1','far-secondary-1'])
 const subset={...a,volumes:{...a.volumes,[k]:{...vol,body:vol.body.filter(s=>ids.has(s.id))}}}
 const world=new Map(worldSolids(subset,bones,k).map(s=>[s.id,s])),values=[]
 for(const [side,label] of [['n','near'],['f','far']]){
  const near=bones['s1-'+side].frame,far=bones['p1-'+side].frame,normal=V(near.u).add(V(far.u)).normalize()
  const root=V(near.o).lerp(V(far.o),.5),ms=['secondary','primary'].map(s=>world.get(label+'-'+s+'-1'))
  if(ms.some(m=>!vol.thermal.some(s=>s.of===m.id)))bad.push('missing thermal')
  for(const t of f.stations){const plane=new Plane().setFromNormalAndCoplanarPoint(normal,root.clone().addScaledVector(normal,.204*t))
   const gap=distance(cut(ms[0],plane),cut(ms[1],plane));values.push(+(gap*1000).toFixed(2))
   if(gap>f.maximumSeparationM)bad.push([k,pose,side,t,+gap.toFixed(4)])
  }
 }
 console.log(k,pose,values)
}
if(bad.length){console.error(JSON.stringify(bad));process.exitCode=1}
