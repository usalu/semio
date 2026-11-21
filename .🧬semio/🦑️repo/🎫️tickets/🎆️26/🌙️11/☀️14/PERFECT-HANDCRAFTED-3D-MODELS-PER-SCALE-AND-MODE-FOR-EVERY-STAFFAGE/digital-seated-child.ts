/** 🧒 Articulate the seated toddler at the measured snowsuit seams, skeletal endpoints and atlas. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Euler,Quaternion,Vector3} from 'three'
const d=JSON.parse(readFileSync(process.argv[2],'utf8')),a=d.asset,f=JSON.parse(readFileSync(new URL('./digital-passenger-joints.json',import.meta.url),'utf8'))[a.id]
const r=x=>+x.toFixed(8),V=p=>new Vector3(...p),arr=p=>p.toArray().map(r),rad=Math.PI/180,mid=.1925
const root=a.rig.bones.find(b=>b.id==='child'),originalRoot=structuredClone(root),centre=new Vector3(0,0,mid)
root.z=r((root.z??0)+mid);root.title={en:'Seated toddler trunk',de:'Rumpf des sitzenden Kleinkinds'}
const frames=new Map([['child',{o:centre,q:new Quaternion()}]]),bones=[]
const local=(p,b)=>V(p).sub(frames.get(b).o).applyQuaternion(frames.get(b).q.clone().invert())
function link(id,parent,start,end,kg,en,de,min,max){
 const frame=frames.get(parent),o=V(start),ray=V(end).sub(o),length=ray.length()
 const dir=ray.clone().applyQuaternion(frame.q.clone().invert()),yaw=Math.atan2(dir.y,dir.x),side=Math.atan2(dir.z,Math.hypot(dir.x,dir.y))
 const pos=local(start,parent),q=frame.q.clone().multiply(new Quaternion().setFromEuler(new Euler(0,-side,yaw,'ZYX')))
 frames.set(id,{o,q})
 bones.push({id,parent,title:{en,de},x:r(pos.x),y:r(pos.y),z:r(pos.z),angleDeg:r(yaw/rad),abductionDeg:r(side/rad),lengthM:r(length),
 joint:{type:'revolute',minDeg:min,maxDeg:max,twistMinDeg:-30,twistMaxDeg:30},mass:{kg,comX:r(.46*length),comY:0,distribution:'point',source:'authored-estimate'}})
}
link(f.neck,'child',f.c7,f.atlas,.18,'Toddler neck','Hals des Kleinkinds',-18,20)
link(f.head,f.neck,f.atlas,[-.035,.445,mid],1.58,'Toddler skull','Schädel des Kleinkinds',-23,22)
for(const [suffix,side,en,de] of [['l','left','left','links'],['r','right','right','rechts']]){
 const arm=f[side+'Arm'],leg=f[side+'Leg']
 for(const [i,id,parent,stations,mass,label,german,min,max] of [
  [0,'upper-arm','child',arm,.24,'upper arm','Oberarm',-60,70],[1,'forearm','upper-arm',arm,.16,'forearm','Unterarm',-45,65],[2,'hand','forearm',arm,.07,'hand','Hand',-25,35],
  [0,'thigh','child',leg,.86,'thigh','Oberschenkel',-25,32],[1,'shin','thigh',leg,.38,'lower leg','Unterschenkel',-32,35],[2,'foot','shin',leg,.16,'foot','Fuß',-24,28]]){
  link('child-'+id+'-'+suffix,parent==='child'?parent:'child-'+parent+'-'+suffix,stations[i],stations[i+1],mass,'Toddler '+en+' '+label,'Kleinkind '+german+' '+de,min,max)
 }
}
const restMoment=new Vector3(originalRoot.mass.comX,originalRoot.mass.comY,mid).multiplyScalar(originalRoot.mass.kg)
let remainder=originalRoot.mass.kg
for(const b of bones){
 const fr=frames.get(b.id),massPoint=new Vector3(b.mass.comX,b.mass.comY,0).applyQuaternion(fr.q).add(fr.o)
 restMoment.addScaledVector(massPoint,-b.mass.kg);remainder-=b.mass.kg
}
restMoment.divideScalar(remainder).sub(centre)
Object.assign(root.mass,{kg:r(remainder),comX:r(restMoment.x),comY:r(restMoment.y),source:'authored-estimate'});delete root.mass.reference
a.rig.bones.splice(a.rig.bones.indexOf(root)+1,0,...bones)
for(const [name,outlet] of f.breath){
 const anchor=a.anchors.find(q=>q.id===name),p=local(a.volumes['50'].thermal.find(s=>s.id===outlet).transform.translation,f.head)
 Object.assign(anchor,{bone:f.head,x:r(p.x),y:r(p.y),z:r(p.z)})
}
const fine=structuredClone(a.volumes['50'])
const ends=(q,i,j)=>({...structuredClone(q),sections:q.sections.slice(i,j).map(s=>structuredClone(s))})
function bind(s,id){
 const fr=frames.get(id),old=s.transform??{},rot=new Quaternion(...(old.rotation??[0,0,0,1]))
 s.bone=id;s.transform={...old,translation:arr(local(old.translation??[0,0,0],id)),rotation:fr.q.clone().invert().multiply(rot).normalize().toArray().map(r)}
 if(s.member)s.member=id
 return s
}
function newSolid(id,bone,material,radii,at,mode){
 const solid={id,kind:'ellipsoid',bone,material,role:'body',tone:material==='bone'?.70:.76,radii,transform:{translation:at}}
 return mode?{...solid,member:bone,part:mode}:solid
}
function mapping(id){
 const base=id.replace(/^th-/,'')
 if(/^child-(head|face$|face-|facial|eye|hat|pompom|nose|mouth|skull|mandible)/.test(base))return f.head
 const side=base.slice(-1)
 if(/^child-(boot|foot-bones)-[lr]$/.test(base))return 'child-foot-'+side
 if(/^child-(hand|hand-bones)-[lr]$/.test(base))return 'child-hand-'+side
 if(/^child-femur-[lr]$/.test(base))return 'child-thigh-'+side
 if(/^child-tibia-[lr]$/.test(base))return 'child-shin-'+side
 if(/^child-humerus-[lr]$/.test(base))return 'child-upper-arm-'+side
 if(/^child-forearm-[lr]$/.test(base))return 'child-forearm-'+side
 return 'child'
}
for(const [scale,v] of Object.entries(a.volumes)){
 const coarse=scale==='500',t=v.thermal
 if(coarse){
  const trunk=structuredClone(fine.body.find(s=>s.id==='child-trunk'))
  trunk.id='child-body';trunk.sections=[trunk.sections[0],trunk.sections[Math.floor(trunk.sections.length/2)],trunk.sections.at(-1)]
  v.body[v.body.findIndex(s=>s.id==='child-body')]=trunk
 }
 for(const side of ['l','r']){
  const arm=side==='l'?f.leftArm:f.rightArm,leg=side==='l'?f.leftLeg:f.rightLeg
  for(const [label,upper,lower,joint] of [['arm','upper-arm','forearm',arm[1]],['leg','thigh','shin',leg[1]]]){
   const id='child-'+label+'-'+side,up='child-'+upper+'-'+side,down='child-'+lower+'-'+side
   let original=v.body.find(s=>s.id===id)
   if(!original){original=structuredClone(a.volumes['200'].body.find(s=>s.id===id)??fine.body.find(s=>s.id===id))
    if(original.bone!=='child'||original.transform)original=structuredClone(fine.body.find(s=>s.id===id))
    v.body.push(original);t.push({id:'th-'+id,of:id,region:'child-clothing'})
   }
   const split=original.sections.reduce((best,s,i,all)=>V(s.at).distanceTo(V(joint))<V(all[best].at).distanceTo(V(joint))?i:best,0)
   if(split===0||split===original.sections.length-1)throw Error(id+' has no shared seam')
   const distal=ends(original,split),prox=ends(original,0,split+1)
   if(coarse){prox.sections=[prox.sections[0],prox.sections.at(-1)];distal.sections=[distal.sections[0],distal.sections.at(-1)]}
   if(label==='leg'&&distal.sections.length>2){
    const ankle=leg[2],idx=distal.sections.reduce((best,s,i,all)=>V(s.at).distanceTo(V(ankle))<V(all[best].at).distanceTo(V(ankle))?i:best,0)
    if(idx>0)distal.sections=distal.sections.slice(0,idx+1)
   }
   Object.assign(original,prox);distal.id=id+'-distal'
   bind(original,up);bind(distal,down);v.body.push(distal);t.push({id:'th-'+distal.id,of:distal.id,region:'child-clothing'})
  }
  for(const [part,region] of [['boot','child-clothing'],['hand','child-skin']]){
   const id='child-'+part+'-'+side
   if(!v.body.some(q=>q.id===id)){
    v.body.push(structuredClone(fine.body.find(q=>q.id===id)))
    v.thermal.push({id:'th-'+id,of:id,region})
   }
  }
  for(const id of ['child-foot-bones-'+side,'child-hand-bones-'+side,'child-femur-'+side,'child-tibia-'+side,'child-humerus-'+side,'child-forearm-'+side]){
   if(!v.structure.some(s=>s.id===id))v.structure.push(structuredClone(fine.structure.find(s=>s.id===id)))
  }
 }
 v.structure=v.structure.filter(s=>!/^child-(leg|arm)-bones-[lr]$/.test(s.id))
 const oldSpine=fine.structure.find(s=>s.id==='child-spine')
 const spine=v.structure.find(s=>s.id==='child-spine')
 if(spine?.sections){const positions=spine.sections.filter(s=>s.at[1]<f.c7[1]);spine.sections=[...positions,structuredClone(oldSpine.sections[3])]}
 const cervical=ends(oldSpine,3,5);cervical.id='child-cervical-column';v.structure.push(bind(cervical,f.neck))
 for(const q of [...v.body,...v.structure,...v.thermal].filter(q=>q.bone==='child')){
  if(q.id==='mass-child')q.transform.translation=[r(root.mass.comX),r(root.mass.comY),mid]
  bind(q,mapping(q.id))
 }
 v.structure.push(newSolid('child-atlas',f.head,'bone',[.011,.009,.014],[0,0,0],'joint'))
 const neck={id:'child-exposed-neck',kind:'loft',bone:f.neck,material:'skin',role:'body',tone:.76,capStart:true,capEnd:true,
 sections:[{at:[.002,0,0],rx:.036,ry:.034,rotation:[0,-.70710678,0,.70710678]},{at:[.036,0,0],rx:.032,ry:.032,rotation:[0,-.70710678,0,.70710678]},{at:[.052,0,0],rx:.033,ry:.032,rotation:[0,-.70710678,0,.70710678]}]}
 v.body.push(neck);v.thermal.push({...structuredClone(neck),id:'th-child-neck-band',region:'child-skin',capStart:false,capEnd:false,sections:neck.sections.slice(1)})
 const rep=a.representations[scale]
 if(rep.status==='authored')for(const [view,shapes] of Object.entries(rep.geometry))for(const s of shapes.filter(q=>q.bone==='child')){
  const target=mapping(s.id);s.bone=target
  if(view==='elevation'&&target!=='child')s.points=s.points.map(([x,y])=>arr(local([x,y,mid],target)).slice(0,2))
 }
 if(rep.status==='authored'){
  if(coarse)rep.geometry.front.find(s=>s.id==='child-head').penMm=.20
  const profiles=Object.fromEntries(rep.geometry.elevation.filter(s=>['child-head','child-hat'].includes(s.id)).map(s=>[s.id,s.points]))
  for(const shapes of Object.values(rep.geometry))for(const s of shapes){
   if(!(s.id in profiles))continue
   const hat=s.id==='child-hat',half=hat?.070:.057,tip=hat?.060:.064
   s.points=structuredClone(profiles[s.id]);s.solid={kind:'extruded',sweep:'w',z0:-half,z1:half,zTip:[-tip,tip]}
  }
 }
 v.decisions.push({en:'The seated passenger bends at the neck, hips, knees, elbows and wrists. The snowsuit follows the limb seams. The eyes and breath outlets move with the skull.',de:'Das sitzende Kind bewegt Hals, Hüften, Knie, Ellbogen und Handgelenke. Der Schneeanzug folgt den Gliedmaßen. Augen und Atemöffnungen bewegen sich mit dem Schädel.'})
}
for(const p of a.poses){
 if(p.id==='checking-child')Object.assign(p.bones,{'child-neck':{angleDeg:3},'child-head':{angleDeg:4,twistDeg:11},'child-upper-arm-l':{angleDeg:11},'child-forearm-l':{angleDeg:8},'child-hand-l':{angleDeg:-6}})
 if(p.id==='kerb-tilt')Object.assign(p.bones,{'child-neck':{angleDeg:-3},'child-head':{angleDeg:-2},'child-forearm-r':{angleDeg:7},'child-shin-l':{angleDeg:5},'child-foot-l':{angleDeg:-4}})
 if(p.id==='pausing-with-stroller')Object.assign(p.bones,{'child-head':{twistDeg:-6},'child-hand-r':{angleDeg:5}})
}
a.thermal.regions.find(q=>q.id==='child-skin').areaM2=f.skinAreaM2
a.thermal.assumptions.push({en:'The uncovered face, neck above the collar and both hands of the seated passenger have 0.042 m² of measured exterior skin.',de:'Gesicht, freier Hals über dem Kragen und beide Hände des sitzenden Kindes ergeben zusammen 0,042 m² gemessene Hautfläche.'})
a.structure.assumptions.push({en:'The 10 kg passenger is resolved into trunk, neck, head, paired upper and lower limbs, hands and feet. The neutral sagittal centre of mass is retained and the passenger sits on the seat centreline.',de:'Die Masse des 10 kg schweren Kindes verteilt sich auf Rumpf, Hals, Kopf, Arme, Beine, Hände und Füße. Der sagittale Schwerpunkt in Neutralstellung bleibt erhalten. Das Kind sitzt auf der Sitzmittellinie.'})
writeFileSync(process.argv[3],JSON.stringify(d))
