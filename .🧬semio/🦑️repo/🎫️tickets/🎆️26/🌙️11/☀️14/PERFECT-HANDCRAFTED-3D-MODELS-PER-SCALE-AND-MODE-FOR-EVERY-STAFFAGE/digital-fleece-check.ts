/** 🐑 Measure the fleeced trunk independently using closed edge pairs, signed volume and radial ray intersections. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {DoubleSide,Mesh,MeshBasicMaterial,BufferGeometry,Float32BufferAttribute,Raycaster,Vector3} from 'three'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),f=JSON.parse(readFileSync(new URL('./digital-fleece-case.json',import.meta.url),'utf8'))
const a=doc.asset
for(const [scale,settings] of Object.entries(f.scales)){
 const measurements=[]
 for(const id of f.parts){
  const part=a.volumes[scale].body.find(s=>s.id===id)
  assert.equal(part.kind,'polyhedron',scale+' '+id)
  assert(a.volumes[scale].thermal.some(s=>s.of===id&&s.region===f.surface[id]))
  const v=part.vertices.map(p=>new Vector3(...p)),indices=part.triangles.flat(),edges=new Map()
  if(id==='hindbody'){
   const n=settings.around,tipIndex=(settings.axial+1)*n
   const center=v.slice(0,n).reduce((sum,p)=>sum.add(p),new Vector3()).divideScalar(n)
   const normal=v[0].clone().sub(center).cross(v[1].clone().sub(center)).normalize()
   assert(v[tipIndex].clone().sub(center).dot(normal)>(scale==='500'?.030:.008))
  }
  let volume=0
  for(const [x,y,z] of part.triangles){
   volume+=v[x].dot(v[y].clone().cross(v[z]))/6
   for(const [i,j] of [[x,y],[y,z],[z,x]]){
    const k=[Math.min(i,j),Math.max(i,j)].join('/'),edge=edges.get(k)??[0,0]
    edge[0]++;edge[1]+=i<j?1:-1;edges.set(k,edge)
   }
  }
  assert(volume>(id==='neck'?.003:.020)&&volume<.095);assert([...edges.values()].every(([n,w])=>n===2&&w===0))
  const geo=new BufferGeometry()
  geo.setAttribute('position',new Float32BufferAttribute(part.vertices.flat(),3));geo.setIndex(indices)
  const mesh=new Mesh(geo,new MeshBasicMaterial({side:DoubleSide}))
  const lo=Math.min(...v.map(p=>p.x)),hi=Math.max(...v.map(p=>p.x)),peakCounts=[],prominences=[]
  for(const tx of [.40,.55,.68,.77]){
   const x=lo+(hi-lo)*tx,cuts=[]
   for(let j=0;j<144;j++){
    const angle=j*Math.PI/72,dir=new Vector3(0,Math.sin(angle),Math.cos(angle))
    const ray=new Raycaster(new Vector3(x,id==='neck'?.018:id==='chest'?-.07:-.1,0),dir)
    const hit=ray.intersectObject(mesh)[0];assert(hit);cuts.push(hit.distance)
   }
   const highs=cuts.flatMap((d,j)=>d>cuts[(j+143)%144]&&d>=cuts[(j+1)%144]?[j]:[])
   peakCounts.push(highs.length)
   for(const j of highs){const h=Math.max(2,Math.round(60/settings.locks));prominences.push(cuts[j]-(cuts[(j+h)%144]+cuts[(j+144-h)%144])/2)}
  }
  assert(Math.max(...peakCounts)>=(id==='neck'?settings.locks-3:settings.locks)*.5,scale+' '+id+' peaks '+peakCounts)
  assert(Math.max(...prominences)>(id==='neck'?.001:.005))
  measurements.push({id,litres:+(volume*1000).toFixed(2),peakCounts,peakReliefMm:+(Math.max(...prominences)*1000).toFixed(1)})
 }
 console.log(JSON.stringify({scale,measurements}))
}
