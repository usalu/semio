/** 🦴 Fit the authored cervical segments, poses, collars and occipital joints for nine human figures. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Euler,Quaternion} from 'three'
import {restFrames} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset
const f=JSON.parse(readFileSync(new URL('./digital-cervical-stations.json',import.meta.url),'utf8'))[a.id]
const head=a.rig.bones.find(b=>b.id==='head'),old=structuredClone(head),c=f.atlas,r=n=>+n.toFixed(7)
const oldFrame=restFrames(a.rig).head.frame
const neckMass=f.neckKg,headMass=r(old.mass.kg-neckMass),mass=(kg,x,y)=>({kg,comX:r(x),comY:r(y),distribution:'point',source:'authored-estimate'})
const neck={id:'neck',parent:old.parent,title:{en:'Cervical column',de:'Halswirbelsäule'},x:old.x,y:old.y,angleDeg:old.angleDeg,lengthM:c[0],mass:mass(neckMass,...f.neckCom),joint:{type:'revolute',minDeg:-40,maxDeg:35,twistMinDeg:-40,twistMaxDeg:40}}
for(const child of a.rig.bones.filter(b=>b.parent==='head')){child.x=r(child.x-c[0]);child.y=r(child.y-c[1])}
Object.assign(head,{parent:'neck',x:c[0],y:c[1],lengthM:r(old.lengthM-c[0]),angleDeg:0,title:{en:'Head',de:'Kopf'},joint:{type:'revolute',minDeg:-30,maxDeg:35,twistMinDeg:-58,twistMaxDeg:58},
 mass:mass(headMass,(old.mass.comX*old.mass.kg-neckMass*f.neckCom[0])/headMass-c[0],(old.mass.comY*old.mass.kg-neckMass*f.neckCom[1])/headMass-c[1])})
a.rig.bones.splice(a.rig.bones.indexOf(head),0,neck)
const rad=Math.PI/180
function orientation(pitch,side,turn){return new Quaternion().setFromEuler(new Euler(-turn*rad,-side*rad,pitch*rad,'ZYX'))}
const offset=(pitch,side,turn)=>({angleDeg:r(pitch),...(Math.abs(side)>1e-7?{abductionDeg:r(side)}:{}),...(Math.abs(turn)>1e-7?{twistDeg:r(turn)}:{})})
for(const p of a.poses){
 const o=p.bones.head??{},[pitch,side,turn]=f.poses[p.id]
 const full=orientation(old.angleDeg+(o.angleDeg??0),o.abductionDeg??0,o.twistDeg??0)
 const top=new Euler().setFromQuaternion(orientation(old.angleDeg+pitch,side,turn).invert().multiply(full),'ZYX')
 p.bones.neck=offset(pitch,side,turn)
 p.bones.head=offset(top.z/rad,-top.y/rad,-top.x/rad)
}
for(const e of a.anchors.filter(b=>b.bone==='head')){e.x=r(e.x-c[0]);e.y=r(e.y-c[1])}
const bindLower=id=>['neck','th-neck','scarf','shirt-collar','th-scarf',f.spine].includes(id)
const base=v=>v.structure.find(s=>s.id===f.spine)
const fine=a.volumes['50'],skin=structuredClone(fine.body.find(q=>q.id==='neck')),spine=structuredClone(base(fine))
const baseJoint=structuredClone(fine.structure.find(s=>s.member==='head'&&s.part==='joint'&&s.id!=='joint-atlas'))
function shortForm(s,bone){
 const out={...structuredClone(s),bone}
 if(out.sections)out.sections=out.sections.filter((q,i,all)=>i===0||i===Math.floor(all.length/2)||i===all.length-1)
 return out
}
function neckSurface(region){
 const [lo,hi]=f.exposedNeckBandM
 const stations=skin.sections
 function at(x){
  const j=stations.findIndex(s=>s.at[0]>=x)
  if(j<=0)throw Error(a.id+' invalid cervical skin station '+x)
  const start=stations[j-1],end=stations[j],t=(x-start.at[0])/(end.at[0]-start.at[0])
  const blend=(l,h)=>r(l+t*(h-l))
  const rotation=new Quaternion(...start.rotation).slerp(new Quaternion(...end.rotation),t).normalize().toArray().map(r)
  return {at:[x,...[1,2].map(i=>blend(start.at[i],end.at[i]))],rx:blend(start.rx,end.rx),ry:blend(start.ry,end.ry),exponent:blend(start.exponent??2,end.exponent??2),rotation}
 }
 return {...structuredClone(skin),id:'th-cervical-skin',bone:'neck',region,sections:[at(lo),...stations.filter(s=>s.at[0]>lo&&s.at[0]<hi),at(hi)],capStart:false,capEnd:false}
}
for(const [scale,volume] of Object.entries(a.volumes)){
 if(!volume.body.some(s=>s.id==='neck'))volume.body.push(shortForm(skin,'neck'))
 if(!base(volume))volume.structure.push({...shortForm(spine,'neck'),member:'neck'})
 const joints=volume.structure.filter(s=>s.member==='head'&&s.part==='joint')
 for(const j of joints){
  if(j.id==='joint-atlas')continue
  j.bone='neck';j.member='neck'
 }
 if(!joints.some(s=>s.bone==='neck'))volume.structure.push({...structuredClone(baseJoint),bone:'neck',member:'neck'})
 for(const s of [...volume.body,...volume.thermal,...volume.structure]){
  if(s.bone!=='head')continue
  if(bindLower(s.id)){s.bone='neck';if(s.member)s.member='neck';continue}
  s.transform??={};s.transform.translation=(s.transform.translation??[0,0,0]).map((n,i)=>r(n-c[i]))
  if(s.member==='head'&&s.part==='mass')s.transform.translation=[head.mass.comX,head.mass.comY,0]
 }
 for(const side of [-1,1])volume.structure.push({id:'occipital-condyle-'+(side<0?'r':'l'),kind:'ellipsoid',bone:'head',material:'bone',role:'body',tone:.54,radii:[.011,.009,.0075],transform:{translation:[0,0,side*f.condyleSpacingM/2]},member:'head',part:'joint'})
 if(!volume.thermal.some(s=>s.of==='neck'||s.id==='th-neck')){
  const surface=volume.thermal.find(s=>s.of==='head'||['th-face','th-head'].includes(s.id))
  if(surface)volume.thermal.push(neckSurface(surface.region))
 }
 const rep=a.representations[scale]
 if(rep.status==='authored')for(const [view,shapes] of Object.entries(rep.geometry))for(const s of shapes){
  if(a.id==='worker-carrying-box'&&scale==='500'&&s.id==='box'&&view==='plan')s.points=s.points.map(([x,y])=>[r(x+Math.sign(x)*.009),r(y+Math.sign(y+.085)*.009)])
  if(s.bone!=='head')continue
  if(bindLower(s.id)){
   s.bone='neck'
   const e=s.solid
   if(e?.zTip)e.zTip=e.zTip.map((tip,i)=>r((i?e.z1:e.z0)+(tip-(i?e.z1:e.z0))*neck.lengthM/old.lengthM))
   if(e?.axisBone==='head')e.axisBone='neck'
   continue
  }
  if(!s.solid&&view!=='elevation')continue
  if(s.kind!=='poly')throw Error(a.id+' '+scale+' '+s.id+' '+s.kind)
  const e=s.solid
  const dims=e?.sweep==='v'?[0,2,1]:e?.sweep==='u'?[1,2,0]:[0,1,2]
  s.points=s.points.map(q=>[r(q[0]-c[dims[0]]),r(q[1]-c[dims[1]])])
  if(e?.kind==='revolved'){
   if(e.axisBone!=='head'||e.axis!=='u')throw Error(s.id+' revolution axis')
   const u=oldFrame.u,v=oldFrame.v
   const move=u.map((x,i)=>x*c[0]+v[i]*c[1])
   e.axisX=r(e.axisX-move[0]+move[1]*u[0]/u[1]);e.axisZ=r((e.axisZ??0)-move[2]+move[1]*u[2]/u[1])
  }else if(e){
   const shift=c[dims[2]],oldSpan=[e.z0,e.z1]
   e.z0=r(oldSpan[0]-shift);e.z1=r(oldSpan[1]-shift)
   if(e.zTip){
    e.z0=r(e.z0+(e.zTip[0]-oldSpan[0])*c[0]/old.lengthM)
    e.z1=r(e.z1+(e.zTip[1]-oldSpan[1])*c[0]/old.lengthM)
    e.zTip=e.zTip.map(q=>r(q-shift))
   }
  }
 }
 volume.decisions.push({en:'This scale retains separate lower cervical and occipital articulation. The skull carries the eyes and respiratory outlets as the neck flexes.',de:'Dieser Maßstab erhält getrennte Bewegungen der unteren Halswirbelsäule und des Hinterhaupts. Beim Beugen des Halses führt der Schädel die Augen und Atemöffnungen mit.'})
}
if(f.faceAndNeckAreaM2){
 const region=a.thermal.regions.find(q=>q.id===fine.thermal.find(s=>s.id==='th-cervical-skin').region)
 region.areaM2=f.faceAndNeckAreaM2
 region.title={en:'Face and exposed neck skin',de:'Gesicht und freiliegende Halshaut'}
 a.thermal.assumptions.push({en:'The face and exposed neck area is measured on the fine exterior mesh. The cervical skin ends at the collar and jaw, with both cut planes open.',de:'Die Fläche von Gesicht und freiem Hals ist am feinen Außenmodell gemessen. Die Halshaut endet offen an Kragen und Kiefer.'})
}
a.structure.assumptions.push({en:`The authored mass split assigns ${neckMass} kg to the neck and ${headMass} kg to the head, preserving their combined neutral centre of mass.`,de:`Die geschätzte Masse verteilt sich mit ${String(neckMass).replace('.',',')} kg auf den Hals und ${String(headMass).replace('.',',')} kg auf den Kopf. Der gemeinsame Schwerpunkt in Neutralstellung bleibt erhalten.`})
writeFileSync(process.argv[3],JSON.stringify(doc))
