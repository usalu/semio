/** 🪽 Shape the overlapping ulnar vanes from their individual insertions and pitch them with the wing. */
import {readFileSync} from 'node:fs'
import {Quaternion,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const f=JSON.parse(readFileSync(new URL('./digital-gull-wing.json',import.meta.url),'utf8'))
const r=x=>+x.toFixed(8),rad=Math.PI/180,V=p=>new Vector3(p.x,p.y,p.z)
const sectionTurn=new Quaternion().setFromAxisAngle(new Vector3(0,1,0),Math.PI/2).toArray().map(r)
export function cubitalFan(a,fractions){
 for(const [side,label,sign] of [['n','near',-1],['f','far',1]]){
  const ulna='forearm-'+side,arm=a.rig.bones.find(b=>b.id===ulna),neutral=evaluateDrawing(a,{kind:'pose',pose:'standing'}).bones[ulna].frame
  const rows=f.ulnaStationsMm.map((x,i)=>{
   const rot=f.foldDropDeg[i]*rad,world=new Vector3(-Math.cos(rot),-Math.sin(rot),0)
   const angle=Math.atan2(world.dot(V(neutral.v)),world.dot(V(neutral.u)))/rad
   const bone={id:'s'+(12-i)+'-'+side,parent:ulna,title:{en:'Secondary S'+(12-i)+' ('+label+' wing)',de:'Armschwinge S'+(12-i)+' ('+(side==='n'?'naher':'ferner')+' Flügel)'},x:x/1000,y:r(-.0015+.003*i/11),z:r(sign*(.006+.002*i/11)),angleDeg:r(angle),lengthM:f.lengthMm[i]/1000,joint:{type:'revolute',minDeg:-180,maxDeg:180,twistMinDeg:-100,twistMaxDeg:100},mass:{kg:f.massG[i]/1000,comX:r(.41*f.lengthMm[i]/1000),comY:0,distribution:'uniform',source:'authored-estimate'}}
   a.rig.bones.push(bone);return bone
  })
  const before=arm.mass.kg,after=r(before-rows.reduce((sum,b)=>sum+b.mass.kg,0))
  for(const [key,coord,fn] of [['comX','x',Math.cos],['comY','y',Math.sin]])arm.mass[key]=r((before*arm.mass[key]-rows.reduce((sum,b)=>sum+b.mass.kg*(b[coord]+b.mass.comX*fn(b.angleDeg*rad)),0))/after)
  arm.mass.kg=after
  for(const p of a.poses){
   const t=Math.min(1,fractions[p.id]);if(!t)continue
   const fr=evaluateDrawing(a,{kind:'pose',pose:p.id}).bones[ulna].frame
   for(const [i,b] of rows.entries()){
    const sag=f.glideSweepDeg[i]*rad,u=V(fr.u),v=V(fr.v),w=V(fr.w)
    const desired=new Vector3(-Math.cos(sag),-.06+.035*i/11,sign*Math.sin(sag)).normalize()
    const angle=Math.atan2(desired.dot(v),desired.dot(u))/rad
    const delta=((angle-b.angleDeg+540)%360)-180
    p.bones[b.id]={angleDeg:r(t*delta),abductionDeg:r(t*Math.asin(desired.dot(w))/rad)}
   }
   const frames=evaluateDrawing(a,{kind:'pose',pose:p.id}).bones
   for(const b of [...rows,...a.rig.bones.filter(b=>b.parent==='hand-'+side&&/^p\d/.test(b.id))]){
    const frame=frames[b.id].frame
    const ref=a.volumes['50'].body.find(s=>s.bone===b.id)
    const width=ref?new Vector3(0,1,0).applyQuaternion(new Quaternion(...ref.sections[Math.floor(ref.sections.length/2)].rotation)):new Vector3(0,1,0)
    const u=V(frame.u),v=V(frame.v),w=V(frame.w),aim=new Vector3(0,1,0).cross(u).normalize()
    let twist=(Math.atan2(width.z,width.y)-Math.atan2(aim.dot(w),aim.dot(v)))/rad
    twist=((twist+270)%180+180)%180-90
    b.joint.twistMinDeg=-100;b.joint.twistMaxDeg=100
    p.bones[b.id].twistDeg=r(t*twist)
   }
  }
  for(const [scale,vol] of Object.entries(a.volumes)){
   if(scale==='500'){
    const discarded=[label+'-leg',...(side==='n'?['neck-1']:[])]
    vol.body=vol.body.filter(s=>!discarded.includes(s.id));vol.thermal=vol.thermal.filter(s=>!discarded.includes(s.of))
    for(const id of [label+'-shank',label+'-tarsus',...(side==='n'?['neck-1','neck-2']:[])]){
     const source=structuredClone(a.volumes['200'].body.find(s=>s.id===id))
     source.sections=source.sections.filter((s,i,ss)=>i===0||i===Math.floor(ss.length/2)||i===ss.length-1)
     vol.body.push(source);vol.thermal.push(structuredClone(a.volumes['200'].thermal.find(q=>q.of===id)))
    }
   }
   for(const part of ['propatagium','arm-web','hand-web']){
    const id=label+'-'+part
    if(!vol.body.some(s=>s.id===id)){
     const source=structuredClone(a.volumes['50'].body.find(s=>s.id===id))
     source.sections=source.sections.filter((s,i,ss)=>i===0||i===Math.floor(ss.length/2)||i===ss.length-1)
     vol.body.push(source);vol.thermal.push({id:'th-'+id,of:id,region:'wing-plumage'})
    }
   }
   for(const [i,b] of rows.entries()){
    const id=label+'-secondary-'+(12-i),ss=scale==='50'?[0,.08,.25,.48,.67,.83,.94,.98,1]:scale==='100'?[0,.08,.25,.48,.83,.94,1]:scale==='200'?[0,.25,.48,.83,1]:[0,.25,.67,1]
    const sections=ss.map(t=>{
     const chord=f.widthMm[i]/1000*(.05*(1-t)+Math.pow(Math.max(0,Math.sin(Math.PI*t)),.55)*(.78+.28*t)+.05*t)
     return {at:[r(t*b.lengthM),r(-.08*chord),r(sign*.0025*Math.sin(Math.PI*t))],rx:r(.0004+.0008*(1-t)),ry:r(chord/2),exponent:2,rotation:sectionTurn}
    })
    const body={id,bone:b.id,kind:'loft',role:'body',material:'wing-feathers',tone:r(.58+.045*(i%3)),capStart:true,capEnd:true,sections}
    vol.body.push(body);vol.thermal.push({id:'th-'+id,of:id,region:'wing-plumage'})
    vol.structure.push({...body,id:id+'-rachis',slender:true,part:'member',member:b.id,sections:sections.map((s,j)=>({...s,at:[s.at[0],0,s.at[2]],rx:r(.0002+.0007*(1-ss[j])),ry:r(.0002+.0008*(1-ss[j]))}))})
    if(scale==='50'){
     const shape={id:id+'-exposed-rachis',bone:b.id,role:'detail',z:4.61,filled:true,tone:.52,region:'wing-plumage',kind:'poly',closed:true,penMm:.20,points:[[r(.67*b.lengthM),.0007],[r(.83*b.lengthM),.00055],[r(.83*b.lengthM),-.00055],[r(.67*b.lengthM),-.0007]],solid:{kind:'extruded',sweep:'w',z0:r(sign*.0018-.0004),z1:r(sign*.0018+.0004)}}
     a.representations[scale].geometry.elevation.push(shape)
    }
   }
  }
 }
}
