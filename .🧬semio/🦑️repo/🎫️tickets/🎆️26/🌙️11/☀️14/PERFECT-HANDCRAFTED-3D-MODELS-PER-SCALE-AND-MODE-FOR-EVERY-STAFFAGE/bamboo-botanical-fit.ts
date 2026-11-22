/** 🎋 Build open bamboo sprays around tapered branchlets. Retain the three individual terminal leaves on each culm. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {drawBamboo} from './bamboo-botanical-paper.ts'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,fine=a.volumes[50],ref=a.volumes[100],V=p=>new Vector3(...p),point=v=>v.toArray().map(x=>+x.toFixed(6))
const rings=q=>{const n=q.triangles[0][2],v=q.vertices.slice(0,-2);return Array.from({length:v.length/n},(_,i)=>v.slice(i*n,(i+1)*n).map(V))}
const center=r=>r.reduce((a,b)=>a.add(b),new Vector3()).divideScalar(r.length)
const mix=(r,t)=>{const f=t*(r.length-1),i=Math.min(r.length-2,Math.floor(f));return r[i].clone().lerp(r[i+1],f-i)}
const middle=vs=>vs.reduce((p,q)=>p.add(q),new Vector3()).divideScalar(vs.length)
const turns=[-18,142,24,176,-42,119],lengths=[.101,.131,.119,.092,.112,.086],widths=[.013,.018,.015,.011,.016,.012]
function branch(s,axis,index,k){
 const centers=rings(axis).map(center),length=centers.reduce((a,p,i)=>i?a+p.distanceTo(centers[i-1]):a,0)
 const n=3,joints=[.22,.33,.44,.55,.65,.75,.84,.92].slice(0,(k===200?7:6)+index%2),positions=[0,...joints.flatMap(t=>[t,t+.002/length]),1]
 const vertices=[],triangles=[],crosses=[],radial=rings(s)[2][0].sub(center(rings(s)[2])).normalize()
 let side=radial
 for(const [j,t] of positions.entries()){
  const c=mix(centers,t),u=mix(centers,Math.min(1,t+.001)).sub(mix(centers,Math.max(0,t-.001))).normalize()
  side=side.clone().addScaledVector(u,-u.dot(side)).normalize()
  if(side.lengthSq()<.5)side=new Vector3(0,0,1).cross(u).normalize()
  const out=side.clone().cross(u).normalize(),r=.0037*(1-t)+.0009,roll=(index%6-2)*.103
  const ring=Array.from({length:n},(_,h)=>c.clone().addScaledVector(side,r*Math.cos(h*2*Math.PI/n+roll)).addScaledVector(out,r*Math.sin(h*2*Math.PI/n+roll)))
  vertices.push(...ring);crosses.push({c,u,side:side.clone(),out,ring})
 }
 for(let j=0;j<positions.length-1;j++)for(let h=0;h<n;h++){
  const b=j*n+h,c=j*n+(h+1)%n,d=c+n,e=b+n,leaf=(j-1)/2
  const face=[b,c,d,e],isBase=Number.isInteger(leaf)&&leaf>=0&&leaf<joints.length&&h===(leaf+index)%n
  if(!isBase){triangles.push([b,c,d],[b,d,e]);continue}
  const root=middle(face.map(i=>vertices[i])),fr=crosses[j],az=turns[(leaf+index)%6]*Math.PI/180
  const outward=root.clone().sub(fr.c).addScaledVector(fr.u,-root.clone().sub(fr.c).dot(fr.u)).normalize()
  const direction=outward.clone().multiplyScalar(.84).addScaledVector(fr.u,.52).normalize()
  direction.applyAxisAngle(outward,.16*Math.sin(az))
  const broad=direction.clone().cross(fr.u).normalize().applyAxisAngle(direction,.20*Math.cos(az)),normal=direction.clone().cross(broad).normalize()
  const L=lengths[(index+leaf)%6],W=widths[(index*2+leaf)%6]
  let previous=face
  for(const [t,amp] of [[.37,1]]){
   const tip=root.clone().addScaledVector(direction,t*L).addScaledVector(fr.u,.008*t*t)
   const ring=[tip.clone().addScaledVector(broad,-W*amp/2),tip.clone().addScaledVector(normal,.0017*amp),tip.clone().addScaledVector(broad,W*amp/2),tip.clone().addScaledVector(normal,-.00065*amp)]
   {
    let best=Infinity,order
    for(const rev of [false,true])for(let shift=0;shift<4;shift++){
     const ids=Array.from({length:4},(_,h)=>(shift+(rev?-h:h)+8)%4),score=ids.reduce((v,n,h)=>v+ring[n].distanceToSquared(vertices[face[h]]),0)
     if(score<best){best=score;order=ids}
    }
    crosses[j].order=order
   }
   const ringIds=crosses[j].order.map(i=>{vertices.push(ring[i]);return vertices.length-1})
   for(let z=0;z<4;z++){const w=(z+1)%4;triangles.push([previous[z],previous[w],ringIds[w]],[previous[z],ringIds[w],ringIds[z]])}
   previous=ringIds
  }
  vertices.push(root.clone().addScaledVector(direction,L).addScaledVector(fr.u,.008));const tip=vertices.length-1
  for(let h=0;h<4;h++)triangles.push([previous[h],previous[(h+1)%4],tip])
 }
 for(const [row,reverse] of [[0,true],[positions.length-1,false]]){
  vertices.push(crosses[row].c);const o=vertices.length-1
  for(let h=0;h<n;h++){const b=row*n+h,c=row*n+(h+1)%n;triangles.push(reverse?[o,c,b]:[o,b,c])}
 }
 const ref=vertices[0];let vol=0
 for(const t of triangles){const [a,b,c]=t.map(i=>vertices[i].clone().sub(ref));vol+=a.dot(b.cross(c))}
 if(vol<0)for(const t of triangles)t.reverse()
 return {vertices:vertices.map(point),triangles}
}
for(const k of [100,200,500]){
 const v=a.volumes[k]
 for(let i=1;i<=6;i++){
  const id='apex'+i+(k===100?'-s':'-tuft'),prev=v.body.find(q=>q.id===id)
  if(!prev)continue
  const leaves=fine.body.filter(q=>q.id.startsWith('apex'+i+'-l')).map(q=>structuredClone(q)),region=v.thermal.find(q=>q.of===id).region
  v.body=v.body.filter(q=>q!==prev);v.thermal=v.thermal.filter(q=>q.of!==id)
  v.body.push(...leaves);v.thermal.push(...leaves.map(q=>({id:'th-'+q.id,of:q.id,region})))
 }
 if(k>=200){
  const selected=new Set()
  for(const q of v.body.filter(q=>k===500?q.id.includes('-plume-'):/^b\d+-\d+[ps]-tuft$/.test(q.id))){
   const tip=V(q.vertices.at(-1)),axis=k===200?ref.body.find(r=>r.id===q.id.replace('-tuft','')):ref.body.filter(r=>r.bone===q.bone&&/^b\d+-\d+[ps]$/.test(r.id)&&!selected.has(r.id)).sort((a,b)=>V(a.vertices.at(-1)).distanceToSquared(tip)-V(b.vertices.at(-1)).distanceToSquared(tip))[0]
   if(!axis)throw Error(q.id)
   selected.add(axis.id);const i=Number(axis.id.match(/-(\d+)/)[1]),culm=Number(axis.id[1])
   Object.assign(q,branch(q,axis,i+culm,k))
  }
  v.decisions.unshift({en:'Each sweeping side branch carries distinct narrow folded blades at staggered nodes. The six terminal leaf fans stay open in the wind.',de:'Die schwingenden Seitenzweige tragen schmale, gefaltete Blätter an versetzten Knoten. Die sechs endständigen Blattfächer bleiben im Wind offen.'})
 }
}
const fineBody=new Map(fine.body.map(q=>[q.id,q])),fineHeat=new Map(fine.thermal.filter(t=>t.of).map(t=>[t.of,t]))
const converted=ref.body.filter(q=>/^b\d+-\d+[ps]-s\d+$/.test(q.id)),oldBuds=new Set(converted.map(q=>q.id))
ref.body=ref.body.filter(q=>!oldBuds.has(q.id));ref.thermal=ref.thermal.filter(q=>!oldBuds.has(q.of))
for(const q of converted){
 const [,arm,num]=q.id.match(/^(b\d+-\d+[ps])-s(\d+)$/),node=Number(num),available=[]
 for(let i=0;fineBody.has(arm+'-f'+node+'-l'+i);i++)available.push(arm+'-f'+node+'-l'+i)
 if(!available.length)throw Error(q.id)
 const selected=new Set([available[(node+Number(arm[1]))%available.length],available[(node+Number(arm[1])+Math.ceil(available.length/2))%available.length]])
 if(fineBody.has(arm+'-t'+node))selected.add(arm+'-t'+node)
 for(const id of selected){
  ref.body.push(structuredClone(fineBody.get(id)))
  ref.thermal.push(structuredClone(fineHeat.get(id)))
 }
}
ref.decisions.unshift({en:'Each side branch shows two real blades on opposite sides of its fan, attached to its folded twig. The middle scale retains the original widths of these blades.',de:'Jeder Seitenzweig zeigt zwei wirkliche Blätter an gegenüberliegenden Fächerseiten mit ihrem Zweig. Die mittlere Darstellung erhält die ursprünglichen Blattbreiten.'})
const pose=posedMesh(a,a.presentation.defaultPose),quality=qualityOf(a,50),outer=[null,null,null,null]
for(const q of fine.body.filter(q=>/-f\d+-l\d+$/.test(q.id))){
 const m=pose(q,quality)
 for(let j=0;j<m.world.length;j+=3)for(const [i,value] of [m.world[j],-m.world[j],m.world[j+2],-m.world[j+2]].entries())
  if(!outer[i]||value>outer[i].value)outer[i]={q,value}
}
const boundary=new Map()
for(const {q} of outer){
 const [,arm,leafNode]=q.id.match(/^(b\d+-\d+[ps])-f(\d+)-l\d+$/)
 for(const s of fine.body.filter(s=>s.id===arm||s.id===arm+'-t'+leafNode||s.id.startsWith(arm+'-f'+leafNode+'-l')))boundary.set(s.id,s)
}
for(const k of [200,500]){
 const v=a.volumes[k]
 for(const [id,q] of boundary){
  v.body.push(structuredClone(q));v.thermal=v.thermal.filter(s=>s.id!=='th-'+id)
  v.thermal.push(structuredClone(fine.thermal.find(s=>s.of===id)))
 }
}
for(const q of fine.body.filter(q=>q.bone==='root-plate'))a.volumes[500].body.push(structuredClone(q))
for(const [view,shapes] of Object.entries(a.representations[200].geometry))a.representations[500].geometry[view].push(...structuredClone(shapes.filter(q=>q.bone==='root-plate')))
drawBamboo(a)
writeFileSync(process.argv[3],JSON.stringify(doc))
