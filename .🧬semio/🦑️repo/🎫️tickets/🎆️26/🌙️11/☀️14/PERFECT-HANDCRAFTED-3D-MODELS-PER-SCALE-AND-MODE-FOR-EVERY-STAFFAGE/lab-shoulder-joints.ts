/** 🦴 Confirm the cervical, glenoid and costal intersections on the posed meshes, including both forelegs. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {Vector3,Ray} from 'three'
import {rigAt,meshesIn} from './r3-lib.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./lab-shoulder-case.json',import.meta.url),'utf8'))
const direction=new Vector3(.317,.453,.833).normalize(),v=arr=>new Vector3(...arr)
const inside=(p,triangles)=>{
 const ray=new Ray(p,direction),hits=new Set()
 for(const t of triangles){const hit=ray.intersectTriangle(...t,false,new Vector3());if(hit)hits.add(Math.round(hit.distanceTo(p)*1e8))}
 return hits.size%2===1
}
assert.equal(a.poses.length,f.poseCount)
let pairs=0
for(const k of f.scaleDenominators){
 const vol=a.volumes[k],parts=vol.body.filter(s=>/^(chest|neck|shoulder-[nf]|upper-arm-[nf]|fore-upper-[nf])$/.test(s.id))
 for(const s of parts){const key=s.id.replace(/-[nf]$/,'');assert.deepEqual(vol.thermal.filter(t=>t.of===s.id).map(t=>[t.region,t.layer]),[[f.fields[key],'fur']])}
 for(const pose of a.poses){
  const fr=rigAt(a,k,{kind:'pose',pose:pose.id}).frames,meshes=meshesIn(a,fr,parts,k)
  const shapes=new Map(parts.map((s,i)=>{
   const m=meshes[i],frame=fr[s.bone]
   const samples=s.sections.map(t=>v(frame.o).addScaledVector(v(frame.u),t.at[0]).addScaledVector(v(frame.v),t.at[1]).addScaledVector(v(frame.w),t.at[2]))
   const tris=Array.from({length:m.indices.length/3},(_,j)=>[0,1,2].map(l=>new Vector3().fromArray(m.world,3*m.indices[j*3+l])))
   return [s.id,{samples,tris}]
  }))
  const check=(l,r)=>{
   const L=shapes.get(l),R=shapes.get(r)
   const hit=L.samples.some(p=>inside(p,R.tris))||R.samples.some(p=>inside(p,L.tris))
   assert.ok(hit,[k,pose.id,l,r].join(' '));pairs++
  }
  check('neck','chest')
  for(const side of ['n','f']){
   if(shapes.has('shoulder-'+side)){check('shoulder-'+side,'chest');check('shoulder-'+side,'upper-arm-'+side)}
   else check('fore-upper-'+side,'chest')
  }
 }
}
console.log(JSON.stringify({poses:a.poses.length,scales:f.scaleDenominators,intersectingPairs:pairs}))
