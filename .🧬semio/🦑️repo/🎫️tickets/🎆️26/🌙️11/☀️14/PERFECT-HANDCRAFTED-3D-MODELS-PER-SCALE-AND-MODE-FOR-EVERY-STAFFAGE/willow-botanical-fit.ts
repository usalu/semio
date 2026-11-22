/** 🌿 Shape the upper willow boughs, loose pendant shoots and alternate narrow blades in their wind frames. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {frames} from './volumes-base.ts'
import {drawWillow} from './willow-botanical-paper.ts'
import {smallestDimension} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset
const round=x=>+x.toFixed(6),V=p=>new Vector3(...p),point=v=>v.toArray().map(round)
const restFrames=frames(a)
const fine=new Map(a.volumes[50].body.map(q=>[q.id,structuredClone(q)])),leafRegion='leaf-curtain'
const design=[
 [.99,.71,.55, .16,.04],[.74,.98,.63,-.26,.12],[.61,.89,.97,.12,-.18],[.92,.52,.77,-.15,-.05],
 [.56,.84,.93,.24,.16],[.91,.67,.98,-.19,.13],[.81,.97,.52,-.09,-.14],[.67,.54,.96,.17,-.12],
 [.98,.68,.83,-.17,.05],[.57,.93,.66,.19,.11],[.75,.98,.59,-.21,-.13],[.97,.71,.86,.15,.10]]
const empty=()=>({vertices:[],triangles:[]})
function surface(target,rows){
 const {vertices,triangles}=target,start=vertices.length,n=rows[0].length,h=rows.length,first=triangles.length
 vertices.push(...rows.flat().map(point))
 for(let j=0;j+1<h;j++)for(let i=0;i<n;i++){const l=start+j*n+i,b=start+j*n+(i+1)%n;triangles.push([l,b,l+n],[b,b+n,l+n])}
 for(const [row,reverse] of [[0,true],[h-1,false]]){
  const o=vertices.length;vertices.push(point(rows[row].reduce((p,q)=>p.add(q),new Vector3()).divideScalar(n)))
  for(let i=0;i<n;i++){const l=start+row*n+i,b=start+row*n+(i+1)%n;triangles.push(reverse?[o,b,l]:[o,l,b])}
 }
 let vol=0;const ref=V(vertices[start])
 for(let j=first;j<triangles.length;j++){const [l,b,c]=triangles[j].map(i=>V(vertices[i]).sub(ref));vol+=l.dot(b.cross(c))}
 if(vol<0)for(let j=first;j<triangles.length;j++)triangles[j].reverse()
}
function stations(src){
 const n=src.triangles[0][2],v=src.vertices.map(V),h=(v.length-2)/n
 if(!Number.isInteger(h)||n<4)throw Error(src.id)
 return t=>{
  const at=Math.min(h-1,Math.max(0,t)*(h-1)),j=Math.min(h-2,Math.floor(at)),r=Array.from({length:n},(_,i)=>v[j*n+i].clone().lerp(v[(j+1)*n+i],at-j))
  const c=r.reduce((p,q)=>p.add(q),new Vector3()).divideScalar(n),b=r[0].clone().sub(c),d=r[Math.round(n/4)].clone().sub(c)
  d.addScaledVector(b,-d.dot(b)/b.lengthSq())
  return {c,b,d}
 }
}
function blade(mesh,origin,tangent,side,dir,length,width){
 const along=tangent.clone().multiplyScalar(.54).addScaledVector(side,dir*.84).normalize(),across=tangent.clone().cross(side).normalize(),normal=along.clone().cross(across).normalize()
 const at=(t,x,bend)=>origin.clone().addScaledVector(along,t*length).addScaledVector(tangent,.009*t*t).addScaledVector(across,x*width/2).addScaledVector(normal,bend)
 const ring=[at(0,0,0),at(.32,1,0),at(.67,.58,0),at(1,0,0),at(.67,-.58,0),at(.32,-1,0)]
 const top=at(.37,0,.0018),base=at(.36,0,-.00055)
 mesh.vertices=[...ring,top,base].map(point)
 for(let j=0;j<6;j++){const i=(j+1)%6;mesh.triangles.push([j,i,6],[i,j,7])}
 let volume=0;for(const t of mesh.triangles){const [a,b,c]=t.map(i=>V(mesh.vertices[i]).sub(origin));volume+=a.dot(b.cross(c))}
 if(volume<0)for(const t of mesh.triangles)t.reverse()
}
for(const [k,v] of Object.entries(a.volumes)){
 const scale=+k,count={50:6,100:6,200:6,500:6}[scale],facets=scale<=100?4:3,rows={50:4,100:3,200:3,500:3}[scale]
 const extra=[],heat=[]
 for(const sprig of v.body){
  if(!/^(tress-|inner-|fringe-)/.test(sprig.id))continue
  const arch=sprig.id.endsWith('-arch'),m=sprig.id.match(/(?:[abc]|-)(\d+)/),number=Number(m[1]),lane=Number(sprig.id.match(/spray-(\d)/)?.[1]??2)-1
  const family=sprig.id.startsWith('inner')?4:sprig.id.startsWith('fringe')?3:'abc'.indexOf(sprig.id[6])
  const [a0,a1,a2,lean,drift]=design[(number+family*3-1)%design.length],length=arch?1:[a0,a1,a2][lane],old=stations(fine.get(sprig.id)),key=number*2+family*5+lane
  const frame=t=>{const f=old(t*length);f.c.addScaledVector(f.b,lean*Math.sin(Math.PI*t*1.47+lane*.33)).addScaledVector(f.d,drift*Math.sin(2*Math.PI*t+.5*family));f.c.z*=1.025
   if(arch){const fr=restFrames[sprig.bone],rise=.17*Math.sin(Math.PI*Math.min(1,t/.52));f.c.addScaledVector(new Vector3(fr.u[1],fr.v[1],fr.w[1]),rise)}
   return f}
  const loop=(f,x,y,wx,wy,n)=>Array.from({length:n},(_,j)=>f.c.clone().addScaledVector(f.b,x+wx*Math.cos(2*Math.PI*j/n)).addScaledVector(f.d,y+wy*Math.sin(2*Math.PI*j/n)))
  const masses=empty(),wood=empty(),massRows=[]
  const N=arch?count-1:count
  for(let j=0;j<N;j++){
   const turn=(j+key)%2?1:-1,span=(arch?.92:.88)+.045*Math.sin(key*1.27+j*2.18),lo=j/N,hi=(j+span)/N
   const envelope=Array.from({length:rows},(_,i)=>{
    const z=i/(rows-1),t=lo+(hi-lo)*z,f=frame(t),shape=Math.max(.016,Math.sin(Math.PI*Math.pow(z,.59))**.71)
    const width=(arch?.98:.81)*(1-.23*t)*(.69+.23*Math.sin(key*.54+j*2.36))*(.80+.20*Math.cos(2*Math.PI*z))
    return loop(f,turn*width*shape*.51,Math.sin(key+j)*shape*.27,width*shape,(arch?.83:.55)*shape*(1-.25*t),facets)
   })
   massRows.push(...envelope)
  }
  surface(masses,massRows)
  const principal=arch||family>=3||lane===(number+family)%3
  const woodRows=Array.from({length:count+1},(_,i)=>{
   const t=i/count,f=frame(t),radius=(arch?.065:.013)*(1-.84*t)
   return loop(f,0,0,radius/f.b.length(),radius/f.d.length(),4)
  });surface(wood,woodRows)
  const twigId='twig-'+sprig.id
  if(principal)extra.push({id:twigId,kind:'polyhedron',bone:sprig.bone,role:'body',material:'willow-bark',tone:.42,...wood})
  if(principal)heat.push({id:'th-'+twigId,of:twigId,region:'limb-bark'})
  if(arch?scale<=100||number%2===0:family<3&&lane===(number+family)%3&&(scale<=100||family===2||number%2===0)){
   v.structure.push({id:'member-'+twigId,of:twigId,member:sprig.bone,part:'member'})
  }
  if(!arch&&principal&&(family===2||number%2===0)){
   const n={50:6,100:5,200:4,500:3}[scale]
   for(let j=0;j<n;j++){
    const t=.996-(n-1-j)*.0105,f=frame(t),axis=frame(Math.min(1,t+.001)).c.sub(frame(t-.001).c).normalize(),side=f.b.clone().normalize()
    side.addScaledVector(axis,-side.dot(axis)).normalize();side.applyAxisAngle(axis,Math.sin(key+j*.63)*.61)
    const sign=(key+j)%2?1:-1
    const bladeMesh=empty(),id='blade-'+sprig.id+'-'+(j+1)
    const h=t*count,i=Math.min(count-1,Math.floor(h)),mean=row=>row.reduce((s,p)=>s.add(p),new Vector3()).divideScalar(row.length)
    const petiole=mean(woodRows[i]).lerp(mean(woodRows[i+1]),h-i).addScaledVector(side,sign*.0010)
    blade(bladeMesh,petiole,axis,side,sign,.095+.044*(.5+.5*Math.cos(key*.81+j*1.7)),.008+.006*(.5+.5*Math.sin(key*.62+j*1.16)))
    extra.push({id,kind:'polyhedron',bone:sprig.bone,role:'detail',texture:true,material:'willow-leaf',tone:.62,...bladeMesh});heat.push({id:'th-'+id,of:id,region:leafRegion})
   }
  }
  Object.assign(sprig,masses)
 }
 for(const side of ['w','e']){
  const member=v.structure.find(q=>q.id==='member-cap-'+side)
  if(member?.sections){
   const wood=structuredClone(member);delete wood.member;delete wood.part;wood.id='wood-cap-'+side;wood.tone=.34
   wood.sections=wood.sections.map((s,i,ss)=>({...s,rx:.055-.025*i/(ss.length-1),ry:.053-.024*i/(ss.length-1)}))
   extra.push(wood);heat.push({id:'th-'+wood.id,of:wood.id,region:'limb-bark'})
  }
 }
 v.body.push(...extra);v.thermal.push(...heat)
 v.decisions.unshift({en:'Upper boughs divide into open, unevenly spaced foliage clusters. Tapering switches carry long-pointed alternate leaves and swing on their own curtain joints. The printed groups preserve visible openings and the structural view follows the principal woody axes.',de:'Die oberen Äste verzweigen sich in lockere, ungleichmäßig verteilte Blattgruppen. Spitz auslaufende Hängeruten tragen schmale, wechselständige Blätter und schwingen an den zugehörigen Gelenken. Die Zeichnung erhält die Durchblicke. Die Struktur folgt den tragenden Holzachsen.'})
 for(const e of v.thermal.filter(q=>q.tags?.includes('emitter'))){
  const origin=e.transform?V(e.transform.translation):e.vertices.reduce((p,v)=>p.add(V(v)),new Vector3()).divideScalar(e.vertices.length)
  let best=Infinity,bestRows
  for(const q of v.body.filter(q=>q.material==='willow-leaf'&&q.bone===e.bone&&!q.transform&&q.id.startsWith('tress-'))){
   for(let j=0;j+rows*facets<=q.vertices.length-2;j+=rows*facets){
    const piece=Array.from({length:rows},(_,i)=>q.vertices.slice(j+i*facets,j+(i+1)*facets).map(V)),ring=piece[Math.floor(rows/2)]
    const center=ring.reduce((p,q)=>p.add(q),new Vector3()).divideScalar(facets),dist=center.distanceToSquared(origin)
    if(dist<best){const preview=empty();surface(preview,piece);if(smallestDimension({kind:'polyhedron',...preview})>=.00015*scale){best=dist;bestRows=piece}}
   }
  }
  const patch=empty();if(bestRows)surface(patch,bestRows)
  if(!bestRows)throw Error(e.id);delete e.transform;Object.assign(e,patch)
 }
}
a.thermal.regions.find(r=>r.id==='limb-bark').areaM2=52.25
a.thermal.regions.find(r=>r.id==='trunk-bark').areaM2=15.66
drawWillow(a)
writeFileSync(process.argv[3],JSON.stringify(doc))
