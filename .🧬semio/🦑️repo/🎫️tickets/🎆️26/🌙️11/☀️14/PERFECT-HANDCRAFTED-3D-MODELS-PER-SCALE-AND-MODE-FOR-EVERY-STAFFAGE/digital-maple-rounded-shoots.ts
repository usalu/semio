/** 🍃 Shape the upper and lower surfaces of each maple spray around its own terminal, preserving the palmate leaf attachment. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,vec=p=>new Vector3(...p),snap=p=>p.toArray().map(x=>+x.toFixed(6)),N=20
for(const v of Object.values(a.volumes)){
 for(const pad of v.body.filter(q=>q.kind==='polyhedron'&&q.material?.startsWith('maple-leaf')&&!q.texture)){
  const orig=pad.vertices.map(vec),count=(orig.length-2)/N
  if(!Number.isInteger(count))throw Error(pad.id)
  let points=orig
  for(let pass=0;pass<4;pass++){
   const prev=points
   points=prev.map((x,i)=>{
    if(i>=N*count)return x.clone()
    const j=Math.floor(i/N),k=i%N,here=prev[i]
    const p=prev[j*N+(k+N-1)%N].clone().add(prev[j*N+(k+1)%N]).multiplyScalar(.5)
    return here.clone().lerp(p,.63)
   })
  }
  const offset=points.map((p,i)=>p.clone().sub(orig[i])),near=p=>orig.reduce((best,v,i)=>p.distanceToSquared(v)<p.distanceToSquared(orig[best])?i:best,0)
  const leaves=v.body.find(q=>q.id==='maple-spray-'+pad.id)
  if(leaves){const origin=vec(leaves.repeat.origin),shift=offset[near(origin)];leaves.vertices=leaves.vertices.map(v=>snap(vec(v).add(shift)));leaves.repeat.origin=snap(origin.add(shift))}
  pad.vertices=points.map(snap)
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
