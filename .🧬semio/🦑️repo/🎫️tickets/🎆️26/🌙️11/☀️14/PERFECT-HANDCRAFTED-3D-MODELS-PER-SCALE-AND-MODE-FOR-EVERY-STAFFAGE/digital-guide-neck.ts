/** 🦴 Articulate the tour guide at the lower cervical column and occipital condyles, in all representations. */
import {readFileSync,writeFileSync} from 'node:fs'
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const d=JSON.parse(readFileSync(process.argv[2],'utf8')),a=d.asset,f=JSON.parse(readFileSync(new URL('./digital-guide-neck.json',import.meta.url),'utf8'))
const r=x=>+x.toFixed(7),atlas=f.atlas,head=a.rig.bones.find(b=>b.id==='head'),length=head.lengthM
const nkg=f.neckMassKg,hkg=r(head.mass.kg-nkg)
const mass=(kg,x,y)=>({kg,comX:r(x),comY:r(y),distribution:'point',source:'authored-estimate'})
const neck={id:'neck',parent:head.parent,title:{en:'Cervical column',de:'Halswirbelsäule'},x:head.x,y:head.y,angleDeg:head.angleDeg,lengthM:atlas[0],joint:{type:'revolute',minDeg:-35,maxDeg:30,twistMinDeg:-28,twistMaxDeg:28},mass:mass(nkg,...f.neckComM)}
Object.assign(head,{parent:'neck',title:{en:'Head',de:'Kopf'},x:atlas[0],y:atlas[1],angleDeg:0,lengthM:r(length-atlas[0]),joint:{type:'revolute',minDeg:-20,maxDeg:25,twistMinDeg:-40,twistMaxDeg:40},mass:mass(hkg,(f.headNeckMassKg*f.combinedComM[0]-nkg*f.neckComM[0])/hkg-atlas[0],-nkg*f.neckComM[1]/hkg-atlas[1])})
a.rig.bones.splice(a.rig.bones.indexOf(head),0,neck)
for(const p of a.poses){
 const old=p.bones.head??{},angle=f.lowerPitchDeg[p.id],turn=p.id==='turning-to-group'?f.turnLowerTwistDeg:0
 p.bones.neck={angleDeg:angle,...(turn?{twistDeg:turn}:{})}
 p.bones.head={...old,angleDeg:r((old.angleDeg??0)-angle),...(turn?{twistDeg:r(old.twistDeg-turn)}:{})}
}
for(const p of a.anchors.filter(b=>b.bone==='head')){p.x=r(p.x-atlas[0]);p.y=r(p.y-atlas[1])}
const lower=id=>['neck','th-neck','jacket-collar','jacket-zip-collar','jacket-zip-puller','cervical-spine'].includes(id)
function hull(points){
 const sorted=[...new Map(points.map(q=>[q.join(','),q])).values()].sort((p,q)=>p[0]-q[0]||p[1]-q[1])
 const cross=(a,b,c)=>(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
 const half=p=>{const rows=[];for(const q of p){while(rows.length>=2&&cross(rows.at(-2),rows.at(-1),q)<=0)rows.pop();rows.push(q)}rows.pop();return rows}
 return [...half(sorted),...half([...sorted].reverse())]
}
for(const [k,v] of Object.entries(a.volumes)){
 for(const s of [...v.body,...v.thermal,...v.structure]){
  if(s.bone!=='head')continue
  if(s.id==='joint-head'){s.id='joint-neck';s.bone=s.member='neck';continue}
  if(lower(s.id)){s.bone='neck';if(s.member)s.member='neck';continue}
  s.transform??={}
  const t=s.transform.translation??[0,0,0]
  s.transform.translation=t.map((n,i)=>r(n-atlas[i]))
  if(s.id==='mass-head')s.transform.translation=[head.mass.comX,head.mass.comY,0]
 }
 for(const sign of [-1,1]){
  v.structure.push({id:'atlas-condyle-'+(sign<0?'r':'l'),kind:'ellipsoid',bone:'head',material:'bone',role:'body',tone:.56,radii:[.012,.009,.0075],transform:{translation:[0,0,sign*.0145]},member:'head',part:'joint'})
 }
 if(k!=='500')v.structure.push({id:'mass-neck',kind:'ellipsoid',bone:'neck',material:'bone',role:'detail',tone:.2,radii:[.012,.012,.012],transform:{translation:f.neckComM},member:'neck',part:'mass'})
 for(const [view,shapes] of Object.entries(a.representations[k].geometry))for(const s of shapes){
  if(s.bone!=='head')continue
  const e=s.solid
  if(lower(s.id)){
   s.bone='neck'
   if(e?.zTip)e.zTip=e.zTip.map((tip,i)=>r((i?e.z1:e.z0)+(tip-(i?e.z1:e.z0))*neck.lengthM/length))
   continue
  }
  if(s.kind!=='poly'||e.kind!=='extruded')throw Error(s.id)
  if(view==='plan'&&e.zTip){
   const parts=v.body.filter(q=>q.id===s.id||q.bone==='head'&&q.id.startsWith(s.id+'-'))
   const points=[]
   let min=Infinity,max=-Infinity
   for(const part of parts){
    const mesh=tessellateSolid({...part,node:'head'},qualityOf(a,Number(k))),p=mesh.positions
    for(let i=0;i<p.length;i+=3){points.push([r(p[i+1]),r(p[i+2])]);min=Math.min(min,p[i]);max=Math.max(max,p[i])}
   }
   s.points=hull(points);s.solid={kind:'extruded',sweep:'u',z0:r(min),z1:r(max)}
  }else{
   const dims=e.sweep==='v'?[0,2,1]:e.sweep==='u'?[1,2,0]:[0,1,2]
   s.points=s.points.map(q=>[r(q[0]-atlas[dims[0]]),r(q[1]-atlas[dims[1]])])
   e.z0=r(e.z0-atlas[dims[2]]);e.z1=r(e.z1-atlas[dims[2]])
   if(e.zTip)throw Error('Unaccounted projected taper in '+view+' '+s.id)
  }
 }
 v.decisions.push({en:'The cervical member supports the skull at paired occipital condyles. The neck carries the collar while the cap, jaw and breath outlets follow the skull.',de:'Die Halswirbelsäule trägt den Schädel an zwei Hinterhauptkondylen. Der Kragen folgt dem Hals, Kappe, Kiefer und Atemöffnungen folgen dem Schädel.'})
}
a.structure.assumptions.push({en:'The 5.413 kg head and neck segment is distributed across a 0.88 kg cervical segment and a 4.533 kg head. The estimated split retains the original combined centre of mass in the neutral pose.',de:'Die 5,413 kg für Kopf und Hals verteilen sich auf 0,88 kg für den Hals und 4,533 kg für den Kopf. Die geschätzte Aufteilung erhält den gemeinsamen Schwerpunkt in Neutralstellung.'})
writeFileSync(process.argv[3],JSON.stringify(d))
