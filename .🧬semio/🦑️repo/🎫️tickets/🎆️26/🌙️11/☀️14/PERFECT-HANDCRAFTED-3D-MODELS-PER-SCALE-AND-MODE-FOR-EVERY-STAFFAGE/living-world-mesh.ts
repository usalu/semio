/** 🌿 Evaluate one neutral mesh in the same native pose, attachment and skin frames as the export. */
import {Matrix4} from 'three'
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {restFrames} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts'
export function posedMesh(asset,pose){
 const now=evaluateDrawing(asset,{kind:'pose',pose}).bones,rest=restFrames(asset.rig)
 const asMatrix=f=>{
  const xyz=p=>Array.isArray(p)?p:[p.x,p.y,p.z],u=xyz(f.u),v=xyz(f.v),w=xyz(f.w),o=xyz(f.o)
  return new Matrix4().set(u[0],v[0],w[0],o[0],u[1],v[1],w[1],o[1],u[2],v[2],w[2],o[2],0,0,0,1)
 }
 const restM=Object.fromEntries(Object.entries(rest).map(([k,r])=>[k,asMatrix(r.frame)]))
 const nowM=Object.fromEntries(Object.entries(now).map(([k,r])=>[k,asMatrix(r.frame)]))
 return (s,q)=>{
  let m=nowM[s.bone].elements
  if(s.weights?.length){m=Array(16).fill(0);for(const w of s.weights){
    const joint=nowM[w.bone].clone().multiply(restM[w.bone].clone().invert()).multiply(restM[s.bone])
    for(let i=0;i<16;i++)m[i]+=joint.elements[i]*w.weight
  }}
  const g=tessellateSolid({...s,node:s.bone},q),v=g.positions,world=new Float64Array(v.length)
  for(let i=0;i<v.length;i+=3)for(let j=0;j<3;j++)world[i+j]=m[j]*v[i]+m[4+j]*v[i+1]+m[8+j]*v[i+2]+m[12+j]
  return {id:s.id,role:s.role,world,indices:g.indices}
 }
}
