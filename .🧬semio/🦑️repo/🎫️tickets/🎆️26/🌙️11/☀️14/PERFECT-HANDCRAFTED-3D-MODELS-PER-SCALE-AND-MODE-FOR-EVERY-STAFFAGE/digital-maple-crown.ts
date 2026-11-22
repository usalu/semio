/** 🍁 Sculpt the nine Japanese maple branch pads as overlapping airy terminal shoots, each with palmate foliage. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3,Vector2,ShapeUtils} from 'three'
import {maplePaper} from './digital-maple-paper.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {bodyDimensions,worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'

const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,patterns=JSON.parse(readFileSync(new URL('./digital-maple-sprays.json',import.meta.url),'utf8'))
const vector=p=>new Vector3(...p),snap=p=>p.toArray().map(v=>+v.toFixed(6)),turn=Math.PI*2
const lobes=[[-.34,-.23,.65,.57,-.19],[.42,-.12,.55,.67,.24],[-.08,.43,.71,.55,.06]]
const left=[[-.04,.025],[-.21,.06],[-.29,.1],[-.56,.21],[-.39,.205],[-.29,.19],[-.37,.32],[-.55,.54],[-.35,.45],[-.17,.32],[-.25,.58],[-.32,.83],[-.20,.71],[-.09,.49],[-.055,.72]]
const blade=[...left,[0,1],...left.slice().reverse().map(([x,y])=>[-x,y]),[0,0]]
const profile=[[-.89,.17],[-.67,.66],[-.28,.97],[.05,1],[.49,.68],[.81,.21]]
const mix=(data,p)=>{const f=((p%1)+1)%1*data.length,i=Math.floor(f);return data[i]+(data[(i+1)%data.length]-data[i])*(f-i)}
function mesh(){return {vertices:[],triangles:[]}}
function blob(out,pt,du,dv,rz,n,phase,pad,rotation){
 const start=out.vertices.length
 for(const [level,rad] of profile)for(let k=0;k<n;k++){
  const t=k/n,b=turn*t,c=Math.cos(b),s=Math.sin(b),sw=.21*Math.sin(2*b+1.3*phase),ripple=mix(pad.reach,t+phase),height=mix(pad.lift,t+phase)
  const reach=rad*ripple*(1+.25*Math.cos(5*b+rotation+1.1*level)+.14*Math.sin(3*b-.81*rotation-.85*level))
  out.vertices.push(snap(pt(du+rz[0]*reach*c,dv+rz[1]*reach*s,rz[2]+rz[3]*(level+rad*(.93*height+sw+.28*c-.28*s-.30*Math.max(0,Math.cos(5*b+rotation)))) )))
 }
 for(let j=0;j<profile.length-1;j++)for(let k=0;k<n;k++){const lo=start+j*n+k,ln=start+j*n+(k+1)%n,hi=lo+n,hn=ln+n;out.triangles.push([lo,ln,hn],[lo,hn,hi])}
 const bottom=out.vertices.length;out.vertices.push(snap(pt(du,dv,rz[2]-.94*rz[3])),snap(pt(du,dv,rz[2]+.94*rz[3])))
 for(let k=0;k<n;k++){const last=start+(profile.length-1)*n;out.triangles.push([bottom,start+(k+1)%n,start+k],[bottom+1,last+k,last+(k+1)%n])}
}
function leaf(out,base,x,y,up,L,roll){
 const xy=blade.concat([[0,.35],[0,.67]]),n=blade.length,stride=xy.length,m=left.length
 for(const face of [1,-1])for(const [u,v] of xy){
  const ridge=.09*(1-Math.abs(u))*(1-Math.abs(2*v-1)),rise=ridge-.22*v*v+.028*Math.sin(8*u)*v+face*.003
  out.vertices.push(snap(base.clone().addScaledVector(x,L*u).addScaledVector(y,L*v).addScaledVector(up,L*(rise+roll*u))))
 }
 const first=[n-1,...Array.from({length:m+1},(_,i)=>i),n+1,n]
 const second=[...Array.from({length:n-m},(_,i)=>m+i),n,n+1]
 for(const half of [first,second]){
  const coordinates=half.map(i=>new Vector2(...xy[i]))
  for(const [a,b,c] of ShapeUtils.triangulateShape(coordinates,[])){
   const i=half[a],j=half[b],k=half[c]
   out.triangles.push([k,j,i],[stride+i,stride+j,stride+k])
  }
 }
 for(let i=0;i<n;i++){const j=(i+1)%n;out.triangles.push([j,i,stride+i],[j,stride+i,stride+j])}
}
function twig(mesh,points,radii,up){
 const n=6,base=mesh.vertices.length
 let priorAxis
 for(let i=0;i<points.length;i++){
  const along=points[Math.min(points.length-1,i+1)].clone().sub(points[Math.max(0,i-1)]).normalize()
  const axis=priorAxis??[new Vector3(1,0,0),new Vector3(0,1,0),new Vector3(0,0,1)].sort((a,b)=>Math.abs(a.dot(along))-Math.abs(b.dot(along)))[0]
  const x=axis.clone().addScaledVector(along,-axis.dot(along)).normalize(),y=along.clone().cross(x)
  priorAxis=x
  for(let j=0;j<n;j++)mesh.vertices.push(snap(points[i].clone().addScaledVector(x,Math.cos(j*turn/n)*radii[i]).addScaledVector(y,Math.sin(j*turn/n)*radii[i])))
  if(i)for(let j=0;j<n;j++){const lo=base+(i-1)*n+j,ln=base+(i-1)*n+(j+1)%n;mesh.triangles.push([lo,ln,ln+n],[lo,ln+n,lo+n])}
 }
 for(let j=1;j<n-1;j++){const end=base+(points.length-1)*n;mesh.triangles.push([base,base+j+1,base+j],[end,end+j,end+j+1])}
}
const foliage=b=>b.material?.startsWith('maple-leaf-')&&b.kind==='polyhedron'
const master=structuredClone(a.volumes['50'].body.filter(foliage))
for(const [k,v] of Object.entries(a.volumes)){
 const scale=+k,n=20
 if(scale===500){
  for(const q of a.volumes['200'].body.filter(q=>/^limb-t[123][ew]$/.test(q.id))){
   v.body.push(structuredClone(q));v.thermal=v.thermal.filter(s=>s.id!=='th-'+q.id);v.thermal.push({id:'th-'+q.id,of:q.id,region:'limb-bark'})
   const member=v.structure.find(s=>s.id==='member-'+q.id)
   if(member){const j=v.structure.indexOf(member);v.structure[j]={id:member.id,of:q.id,member:q.bone,part:'member'}}
  }
 }
 const old=new Set(v.body.filter(foliage).map(q=>q.id))
 v.body=v.body.filter(q=>!old.has(q.id));v.thermal=v.thermal.filter(q=>!old.has(q.of))
 for(const [order,source] of master.entries()){
  const b=structuredClone(source)
  v.body.push(b)
  v.thermal.push({id:'th-'+b.id,of:b.id,region:b.material.endsWith('sun')?'leaf-sun':'leaf-shade'})
  const pad=patterns[b.bone.slice(4)],seq=Number(b.id.match(/\d+$/)?.[0]??1)-1,rotation=(pad.phase+seq*47)*Math.PI/180,phase=rotation/turn
  const raw=b.vertices.map(vector),lower=raw.at(-2),upper=raw.at(-1),up=upper.clone().sub(lower).normalize(),mid=lower.clone().lerp(upper,.5)
  const side=raw[0].clone().sub(mid);side.addScaledVector(up,-side.dot(up)).normalize()
  const across=up.clone().cross(side).normalize(),range=axis=>Math.max(...raw.map(p=>Math.abs(p.clone().sub(mid).dot(axis))))
  const rx=range(side),ry=range(across),h=lower.distanceTo(upper)*.5
  const pt=(x,y,z)=>mid.clone().addScaledVector(side,rx*x).addScaledVector(across,ry*y).addScaledVector(up,h*z)
  const out=mesh(),sag=1+.07*Math.sin(phase*9)
  const form=(u,v,w)=>{
   const A=Math.atan2(v,u),reach=Math.hypot(u,v),fork=Math.cos(3*A+rotation),fold=Math.sin(4*A+phase*5)
   return pt(u*(.94+.11*fork),v*(.94+.11*fork),sag*(.90*w+.48*reach*fork-.24*reach*fold+.19*u-.15*v))
  }
  blob(out,form,0,0,[1.01,.98,0,.99],n,phase,pad,rotation)
  Object.assign(b,out)
  const leafRegion=b.material.endsWith('sun')?'leaf-sun':'leaf-shade'
  const showTwig=scale===50||!b.id.includes('-f')||scale===100||order%2===0
  if(showTwig){
   const target=pt(.27*Math.cos(rotation),.27*Math.sin(rotation),-.12),rise=target.length(),origin=new Vector3()
   if(rise>.065){
    const d=target.clone().normalize(),sideBend=up.clone().addScaledVector(d,-up.dot(d))
    const knee=target.clone().multiplyScalar(.54).addScaledVector(sideBend,-Math.min(.085,rise*.105))
    const wood=mesh(),R=Math.min(.014,rise*.014),twigId='twig-fork-'+b.id
    twig(wood,[origin,knee,target],[R,R*.66,R*.25],up)
    const shafts=scale<=100?['heel','tip'].map((name,i)=>{
     const vertices=wood.vertices.slice(i*6,i*6+12),triangles=wood.triangles.filter(t=>t.every(j=>j>=i*6&&j<i*6+12)&&t.some(j=>j<i*6+6)&&t.some(j=>j>=i*6+6)).map(t=>t.map(j=>j-i*6))
     for(let j=1;j<5;j++)triangles.push([0,j+1,j],[6,6+j,7+j])
     return {id:twigId+'-'+name,vertices,triangles}
    }):[{id:twigId,...wood}]
    for(const stock of shafts){
     v.body.push({...stock,kind:'polyhedron',bone:b.bone,material:'maple-bark-young',role:'detail',tone:.38,slender:true})
     v.structure.push({id:'st-'+stock.id,of:stock.id,member:b.bone,part:'member'})
     v.thermal.push({id:'th-'+stock.id,of:stock.id,region:'limb-bark'})
    }
   }
  }
  if(scale===50||order%4!==(scale===100?1:2)&&(!b.id.includes('-i')||order%2===0)){
   const theta=rotation+seq*.41+.2,dx=Math.cos(theta),dy=Math.sin(theta)
   const dir=side.clone().multiplyScalar(dx).addScaledVector(across,dy).addScaledVector(up,-.42).normalize()
   const tangent=up.clone().cross(dir).normalize(),normal=dir.clone().cross(tangent).normalize()
   const origin=form(.91*dx,.91*dy,-.05),leaves=mesh(),length=.101+.014*Math.sin(theta*2+seq)
   leaf(leaves,origin.clone().addScaledVector(dir,.0028),tangent,dir,normal,length,.12*Math.sin(theta))
   const id='maple-spray-'+b.id
   v.body.push({id,kind:'polyhedron',bone:b.bone,material:b.material,role:'texture',tone:Math.min(.79,b.tone+.075),texture:true,...leaves,repeat:{count:2,rotationAxis:snap(normal),rotationRad:Math.PI,origin:snap(origin)}})
   v.thermal.push({id:'th-'+id,of:id,region:leafRegion})
  }
 }
 const markers=new Map(v.structure.filter(q=>q.part==='mass'&&q.bone?.startsWith('fol-')).map(q=>[q.bone,q]))
 v.structure=v.structure.filter(q=>!(q.part==='mass'&&markers.has(q.bone)))
 for(const [bone,marker] of markers){
  const pads=v.body.filter(q=>q.bone===bone&&foliage(q)&&!q.texture)
  const weights=pads.map(q=>{
   let volume=0;const moment=new Vector3()
   for(const [i,j,k] of q.triangles){
    const x=vector(q.vertices[i]),y=vector(q.vertices[j]),z=vector(q.vertices[k]),tet=x.dot(y.clone().cross(z))/6
    volume+=tet;moment.addScaledVector(x.add(y).add(z),tet/4)
   }
   return {pad:q,weight:Math.abs(volume),centroid:moment.multiplyScalar(1/volume)}
  })
  const total=weights.reduce((sum,w)=>sum+w.weight,0),centre=new Vector3()
  for(const {pad,weight,centroid} of weights){
   centre.addScaledVector(centroid,weight/total)
   const radius=.056*Math.cbrt(weight/(total/weights.length))
   v.structure.push({id:'mass-'+pad.id,kind:'ellipsoid',bone,material:pad.material,role:'detail',tone:marker.tone,member:marker.member,part:'mass',radii:[radius,radius*.66,radius*.82],transform:{translation:snap(centroid)}})
  }
  if(scale===50){const b=a.rig.bones.find(q=>q.id===bone);b.mass.comX=+centre.x.toFixed(5);b.mass.comY=+centre.y.toFixed(5)}
 }
 v.decisions.push({en:'The foliage follows nine branch groups. Each foliage pad has three asymmetric, drooping lobes and a distinct lower surface. Seven-lobed maple blades at their edges follow the same group and distinguish the sunlit and shaded surfaces.',de:'Das Laub folgt neun Astgruppen. Jedes Polster besitzt drei asymmetrische, überhängende Ausladungen und eine eigenständig geformte Unterseite. Siebenlappige Ahornblätter folgen am Rand derselben Astgruppe und unterscheiden besonnte und beschattete Flächen.'})
}
maplePaper(a)
const solved=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose}).bones
const [widthM,heightM,depthM]=bodyDimensions(worldSolids(a,solved,50)).map(v=>+v.toFixed(3))
Object.assign(a.dimensions,{widthM,heightM,depthM})
for(const sym of a.symbols)sym.dimensionalNote={en:`The crown outline and the three stems are drawn at their true extent of ${widthM} × ${depthM} × ${heightM} m in width, depth and height.`,de:`Der Kronenumriss und die drei Stämme sind in ihrer tatsächlichen Ausdehnung von ${widthM} × ${depthM} × ${heightM} m in Breite, Tiefe und Höhe gezeichnet.`}
for(const [region,area] of [['stem-bark',3.93538],['limb-bark',8.28644]])a.thermal.regions.find(r=>r.id===region).areaM2=area
writeFileSync(process.argv[3],JSON.stringify(doc))
