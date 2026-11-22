/** 🐿️ Sculpt the red squirrel’s flowing brush and caudal skeleton in six articulated groups. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Matrix4,Quaternion,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./digital-squirrel-brush.json',import.meta.url),'utf8'))
const round=x=>+x.toFixed(7),mm=x=>round(.001*x),V=p=>new Vector3(p.x,p.y,p.z),rad=Math.PI/180
a.dimensions.widthM=.169;a.dimensions.heightM=.196
const prior=new Map(a.poses.map(p=>{const fr=evaluateDrawing(a,{kind:'pose',pose:p.id}).bones['tail-1'].frame;return [p.id,{heading:Math.atan2(fr.u.y,fr.u.x)/rad,bend:p.bones['tail-2'].angleDeg}]}))
const rest=a.rig.bones.find(b=>b.id==='tail-1'),index=a.rig.bones.indexOf(rest),old=new Set(['tail-1','tail-2'])
const names=['root','proximal fan','lower brush','upper brush','distal fan','tip'],de=['Wurzel','proximaler Fächer','unterer Busch','oberer Busch','distaler Fächer','Spitze']
const chain=f.lengthsMm.map((length,i)=>({...structuredClone(rest),id:'tail-'+(i+1),parent:i?'tail-'+i:rest.parent,x:i?mm(f.lengthsMm[i-1]):rest.x,y:i?0:rest.y,angleDeg:i?0:rest.angleDeg,lengthM:mm(length),title:{en:'Tail, '+names[i],de:'Schwanz, '+de[i]},joint:{type:'revolute',minDeg:i?-60:-169,maxDeg:i?60:169},mass:{...rest.mass,kg:f.massKg[i],comX:mm(length*.5)}}))
let level=0
for(const [i,b] of chain.entries()){
 level+=f.vertebrae[i]
 const t=(level-1)/22,R=3*(1-t*.77),W=R*(1.21-.2*t),rx=mm(W*.68),ry=mm(R*.68),area=Math.PI*rx*ry
 b.section={material:'bone',areaM2:area,extremeFibreM:ry,secondMomentM4:area*ry*ry/4,title:{en:'Narrowest centrum isthmus',de:'Engster Wirbelkörperquerschnitt'}}
}
a.rig.bones=a.rig.bones.filter(b=>!old.has(b.id));a.rig.bones.splice(index,0,...chain)
for(const p of a.poses){
 const {heading,bend}=prior.get(p.id),sit=p.id==='sitting-eating'||p.id==='nibble-b',climb=p.id.startsWith('climb'),fractions=[0,.09,.26,.53,.79,1.1]
 for(let i=0;i<6;i++){
  const b=chain[i],degree=sit?f.seatedHeadingsDeg[i]+(p.id==='nibble-b'&&i>2?i-2:0):heading+bend*fractions[i]+[0,-2,-1,0,2,3][i]
  const turn=sit?f.seatedLateralDeg[i]:[0,-1.2,-1.8,.5,2,3][i]*(climb?-1:1)
  const d=new Vector3(Math.cos(degree*rad)*Math.cos(turn*rad),Math.sin(degree*rad)*Math.cos(turn*rad),Math.sin(turn*rad))
  const parent=evaluateDrawing(a,{kind:'pose',pose:p.id}).bones[b.parent].frame
  const x=d.dot(V(parent.u)),y=d.dot(V(parent.v)),z=d.dot(V(parent.w))
  p.bones[b.id]={angleDeg:round((Math.atan2(y,x)/rad-b.angleDeg+540)%360-180),abductionDeg:round(Math.atan2(z,Math.hypot(x,y))/rad)}
 }
}
const fur=a.thermal.regions.find(q=>q.id==='tail-fur')
fur.areaM2=f.areaM2
for(const pose of a.poses)pose.thermal.regions[fur.id].exposedFraction=f.exposedFraction[pose.id]
a.thermal.assumptions.push({en:'The fur exchange area follows the outer brush and guard locks. The exposed fraction accounts for overlap between the six brush sections and contact with the trunk in each pose.',de:'Die Austauschfläche folgt der äußeren Schwanzkontur und den Deckhaarsträhnen. Der exponierte Anteil berücksichtigt die Überlappungen der sechs Abschnitte und den Körperkontakt in jeder Haltung.'})
const stations=f.brushStationsMm
function radii(d){
 if(d<=stations[0][0])return stations[0].slice(1)
 const i=stations.findIndex(s=>s[0]>=d)
 if(i<0)return stations.at(-1).slice(1)
 const [x,z,y]=stations[i-1],[x1,z1,y1]=stations[i],p=(d-x)/(x1-x)
 return [z+(z1-z)*p,y+(y1-y)*p]
}
function shell(i,start,length,scale,under){
 const n=scale===50?30:scale===100?24:scale===200?18:14,full=scale<=100?7:5,last=i===5
 const proximal=radii(start)[1],distal=radii(start+length)[1]
 const cap=[[-.998,.064],[-.945,.327],[-.78,.626],[-.52,.854],[-.26,.966]]
 const rows=i?cap.map(([d,r])=>[d*proximal,r]).concat([[0,1]]):[[-5,.11],[-3,.52],[-1,.91],[0,1]]
 for(let s=1;s<full;s++)rows.push([length*s/full,1])
 rows.push([length,1],...(last?[[length+5,1],[length+10,1],[length+14,1]]:cap.slice().reverse().map(([d,r])=>[length-d*distal,r])))
 const vertices=[],triangles=[],loops=[],pattern=f.hairFlow[i]
 for(const [j,[x,taper]] of rows.entries()){
  const [rz,ry]=radii(start+(i&&x<0?0:!last&&x>length?length:x)),ring=vertices.length
  for(let k=0;k<n;k++){
   const edge=Math.max(0,Math.min(1,x/6,(length-x)/6)),angle=2*Math.PI*k/n,phase=angle*3+.024*(start+x),lock=under?0:edge*(.72*Math.sin(phase)+.40*pattern[(k+2*j)%6])*(last&&x>length?.32:1)
   const z=taper*rz*Math.sin(angle)*(1+.075*lock)*(under?.65:1),y=taper*ry*Math.cos(angle)*(1+.10*lock)*(under?.64:1)
   const adv=under||j<2||j>rows.length-3?0:Math.max(0,lock)*Math.min(1.6,.17*rz)
   vertices.push([mm(x+adv),mm(y),mm(z)])
  }
  loops.push(ring)
 }
 const first=vertices.length;vertices.push([mm(rows[0][0]),0,0],[mm(rows.at(-1)[0]),0,0])
 for(let k=0;k<n;k++){
  const next=(k+1)%n;triangles.push([first,next,k],[first+1,loops.at(-1)+k,loops.at(-1)+next])
  for(let j=0;j<loops.length-1;j++){const u=loops[j],v=loops[j+1];triangles.push([u+k,u+next,v+next],[u+k,v+next,v+k])}
 }
 return {kind:'polyhedron',bone:chain[i].id,material:'tail-fur',role:'body',tone:.30,vertices,triangles}
}
const quat=[0,-.7071068,0,.7071068]
function guardLock(spec,index,scale,start){
 const [i,origin,azimuth,length,lift]=spec,angle=azimuth*rad,cos=Math.cos(angle),sin=Math.sin(angle),bone=chain[i].id
 const axes=new Matrix4().makeBasis(new Vector3(0,sin,-cos),new Vector3(0,cos,sin),new Vector3(1,0,0))
 const rotation=new Quaternion().setFromRotationMatrix(axes).toArray().map(round)
 const sections=[[0,-3.5,.4,.4],[.25,-.8,2.9,1.2],[.48,1.4,3.4,1.35],[.74,lift*.72,2.1,.91],[1,lift,.26,.20]].filter((_,j)=>scale<=100||j!==1).map(([t,d,rx,ry])=>{
  const x=origin+t*length,[rz,rv]=radii(start+x),turn=(index%3-1)*t*1.7
  return {at:[mm(x),mm((rv+d)*cos+turn*sin),mm((rz+d)*sin-turn*cos)],rx:mm(rx),ry:mm(ry),exponent:2,rotation}
 })
 return {id:'brush-lock-'+(index+1),kind:'loft',bone,material:'tail-fur',role:'texture',tone:round(.29+.024*(index%4)),texture:true,sections,capStart:true,capEnd:true}
}
function skeletal(i,offset,length,global,localIndex){
 const fraction=(global+localIndex)/22,R=3.0*(1-fraction*.77),W=R*(1.21-.2*fraction),step=length/f.vertebrae[i],centre=step*(localIndex+.5),axis=chain[i].id
 const sections=[[-.46,.9],[-.34,1],[-.07,.68],[.17,.70],[.34,1],[.46,.95]].map(([u,k])=>({at:[mm(centre+step*u),0,0],rx:mm(W*k),ry:mm(R*k),rotation:quat,exponent:2}))
 return {id:'caudal-centrum-'+(global+localIndex+1),kind:'loft',bone:axis,material:'bone',role:'body',tone:.9,sections,capStart:true,capEnd:true,member:axis,part:'member'}
}
for(const [k,v] of Object.entries(a.volumes)){
 const scale=+k,oldBody=v.body.find(q=>q.id==='tail-1'),region='tail-fur',oldSet=new Set(v.body.filter(q=>old.has(q.bone)).map(q=>q.id))
 v.body=v.body.filter(q=>!old.has(q.bone));v.thermal=v.thermal.filter(q=>q.region!==region);v.structure=v.structure.filter(q=>!old.has(q.member))
 let start=0,global=0
 for(const [i,b] of chain.entries()){
  const L=f.lengthsMm[i],coat={...shell(i,start,L,scale,false),id:b.id,material:oldBody.material};v.body.push(coat)
  if(scale===50)v.thermal.push({...shell(i,start,L,scale,true),id:'under-'+b.id,region,layer:'tail-underfur'})
  v.thermal.push({id:'th-'+b.id,of:b.id,region,...(scale===50?{layer:'tail-guard'}:{})})
  f.guardLocks.forEach((spec,j)=>{
   if(spec[0]!==i||scale===100&&j%5===0||scale===200&&j%2===0||scale===500&&j%3!==1)return
   const lock=guardLock(spec,j,scale,start);v.body.push(lock);v.thermal.push({id:'th-'+lock.id,of:lock.id,region,...(scale===50?{layer:'tail-guard'}:{})})
  })
  for(let j=0;j<f.vertebrae[i];j++)v.structure.push(skeletal(i,start,L,global,j))
  start+=L;global+=f.vertebrae[i]
 }
 const rep=a.representations[k]
 if(rep.status==='authored')for(const [view,shapes] of Object.entries(rep.geometry)){
  const style=shapes.find(q=>old.has(q.bone)),before=shapes.indexOf(style),other=shapes.filter(q=>!old.has(q.bone)),batch=[]
  for(const b of chain){
   const surface=v.body.find(q=>q.id===b.id),vs=surface.vertices,n=scale===50?30:24
   const seq=Array.from({length:(vs.length-2)/n},(_,i)=>vs.slice(n*i,n*(i+1)))
   const samples=seq.map(r=>[r.reduce((s,p)=>s+p[0],0)/n,Math.max(...r.map(p=>p[1])),Math.min(...r.map(p=>p[1])),Math.max(...r.map(p=>p[2]))])
   const lateral=round(samples.reduce((v,p)=>v+p[3],0)/samples.reduce((v,p)=>v+(p[1]-p[2])*.5,0))
   const points=samples.map(([x,y,m,z])=>[round(x),round(view==='front'?z/lateral:(y-m)*.5)]).concat([[round(samples.at(-1)[0]),0],[round(samples[0][0]),0]])
   batch.push({...structuredClone(style),id:b.id,bone:b.id,kind:'poly',closed:true,points,solid:{kind:'revolved',axisBone:b.id,axis:'u',axisX:0,axisZ:0,lateralScale:lateral,mirror:true}})
  }
  other.splice(before,0,...batch);rep.geometry[view]=other
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
