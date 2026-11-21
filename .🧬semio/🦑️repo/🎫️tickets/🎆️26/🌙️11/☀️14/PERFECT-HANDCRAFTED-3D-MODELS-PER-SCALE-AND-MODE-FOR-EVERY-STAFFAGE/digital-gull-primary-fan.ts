/** 🪶 Fit ten asymmetric primary remiges to each hand and articulate their spread in flight. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Quaternion,Vector3} from 'three'
import {cubitalFan} from './digital-gull-wing.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const d=JSON.parse(readFileSync(process.argv[2],'utf8')),a=d.asset,f=JSON.parse(readFileSync(new URL('./digital-gull-fan.json',import.meta.url),'utf8'))
const fine=structuredClone(a.volumes['50']),r=x=>+x.toFixed(8),rad=Math.PI/180,V=p=>new Vector3(p.x,p.y,p.z),sides=[['n','near'],['f','far']]
const poseFrames=new Map(a.poses.map(p=>[p.id,evaluateDrawing(a,{kind:'pose',pose:p.id}).bones]))
for(const [side,label] of sides){
 const hand=a.rig.bones.find(b=>b.id==='hand-'+side),stock=fine.body.find(q=>q.id===label+'-primaries').sections,mirror=side==='n'?1:-1
 const centre=x=>{
  let j=stock.findIndex(q=>q.at[0]>=x);if(j<0)j=stock.length-1
  const lo=stock[Math.max(0,j-1)],hi=stock[j],t=j?Math.max(0,Math.min(1,(x-lo.at[0])/(hi.at[0]-lo.at[0]))):0
  return {at:[x,...[1,2].map(k=>r(lo.at[k]+(hi.at[k]-lo.at[k])*t))],q:new Quaternion(...lo.rotation).slerp(new Quaternion(...hi.rotation),t).normalize()}
 }
 const roots=f.rootXM.map((x,i)=>{const v=centre(x).at;return [x,r(v[1]+.0012*(i/9-.5)),r(v[2]+mirror*.004*(i/9-.5))]})
 const areaMass=f.weightG.reduce((sum,n)=>sum+n/1000,0),before=hand.mass.kg,newMass=r(before-areaMass)
 const offsets=f.rootXM.map((x,i)=>r(f.tipXM[i]-x))
 const centres=roots.map((p,i)=>[p[0]+.43*offsets[i],p[1]])
 hand.mass.comX=r((hand.mass.comX*before-centres.reduce((s,c,i)=>s+c[0]*f.weightG[i]/1000,0))/newMass)
 hand.mass.comY=r((hand.mass.comY*before-centres.reduce((s,c,i)=>s+c[1]*f.weightG[i]/1000,0))/newMass)
 hand.mass.kg=newMass;hand.lengthM=.135
 hand.title={en:(side==='n'?'Near':'Far')+' carpometacarpus and digits',de:(side==='n'?'Nahe':'Ferne')+' Mittelhand und Finger'}
 const feather=i=>'p'+(i+1)+'-'+side
 for(const [i,o] of roots.entries()){
  a.rig.bones.push({id:feather(i),parent:hand.id,title:{en:'Primary feather P'+(i+1)+' ('+label+' wing)',de:'Handschwinge P'+(i+1)+' ('+(side==='n'?'naher':'ferner')+' Flügel)'},x:o[0],y:o[1],z:o[2],angleDeg:0,lengthM:offsets[i],joint:{type:'revolute',minDeg:-69,maxDeg:69,twistMinDeg:-15,twistMaxDeg:15},mass:{kg:f.weightG[i]/1000,comX:r(.43*offsets[i]),comY:0,distribution:'uniform',source:'authored-estimate'}})
 }
 for(const p of a.poses){
  const fraction=f.poseFraction[p.id],fr=poseFrames.get(p.id)[hand.id].frame,u=V(fr.u),v=V(fr.v),w=V(fr.w)
  let trailing=new Vector3(0,1,0).cross(u).normalize();if(trailing.x>0)trailing.negate()
  for(let i=0;i<10;i++){
   const spread=f.glideFanDeg[i]*fraction*rad,dir=u.clone().multiplyScalar(Math.cos(spread)).addScaledVector(trailing,Math.sin(spread))
   p.bones[feather(i)]={angleDeg:r(Math.atan2(dir.dot(v),dir.dot(u))/rad),abductionDeg:r(Math.asin(Math.min(1,Math.max(-1,dir.dot(w))))/rad)}
  }
 }
 for(const [scale,volume] of Object.entries(a.volumes)){
  const primary=label+'-primaries',mark=label+'-mirror',old=volume.thermal.find(q=>q.of===primary),ref=structuredClone(old??{region:'wing-plumage'})
  volume.body=volume.body.filter(s=>s.id!==primary&&s.id!==mark)
  volume.thermal=volume.thermal.filter(s=>s.of!==primary&&s.of!==mark&&s.id!=='th-'+primary+'-down')
  volume.structure=volume.structure.filter(s=>!s.id.startsWith(label+'-primary-shaft-'))
  const manus=volume.structure.find(q=>q.id===label+'-hand-bones')
  if(manus){
   const orig=structuredClone(fine.structure.find(q=>q.id===label+'-manus'))
   if(orig.sections)orig.sections=orig.sections.filter((q,i,s)=>i===0||i===Math.floor(s.length/2)||i===s.length-1)
   volume.structure[volume.structure.indexOf(manus)]={...orig,id:manus.id}
  }
  const stops=scale==='50'?[0,.08,.18,.34,.52,.69,.82,.92,.97,1]:scale==='100'?[0,.08,.34,.52,.82,.97,1]:scale==='200'?[0,.18,.52,.82,1]:[0,.34,.69,1]
  for(let i=0;i<10;i++){
   const o=roots[i],id=feather(i),length=offsets[i],blade=label+'-primary-'+(i+1)
   const station=t=>{
    const x=o[0]+length*t,c=centre(x),span=f.wideVaneM[i]*(.042*(1-t)+Math.pow(Math.max(0,Math.sin(Math.PI*t)),.62)*(1.2-.42*t)+.012*t)
    const thin=(.00045+.00055*(1-t))*(1-.87*t*t)
    const bias=(i/9-.5)*.012*Math.sin(Math.PI*t)+.005*(i/9-.5)*t
    const rachis=[r(x-o[0]),r((c.at[1]-o[1])*t+.0018*(i/9-.5)*Math.sin(Math.PI*t)),r((c.at[2]-o[2])*t+mirror*bias)]
    const rotated=new Vector3(0,1,0).applyQuaternion(c.q),mid=mirror*.11*span
    return {at:rachis.map((v,k)=>r(v+rotated.getComponent(k)*mid)),rx:r(thin),ry:r(span/2),exponent:2,rotation:c.q.toArray().map(r),rachis}
   }
   const ss=stops.map(station),sections=ss.map(({rachis,...s})=>s)
   const vane={id:blade,kind:'loft',bone:id,material:'wing-feathers',role:'body',tone:r(.13+.022*(9-i)),sections,capStart:true,capEnd:true}
   volume.body.push(vane);volume.thermal.push({...ref,id:'th-'+blade,of:blade})
   const shafts=ss.map((s,j)=>({...s,at:s.rachis,rx:r(.00022+.0009*(1-stops[j])**1.3),ry:r(.00030+.0011*(1-stops[j])**1.5)})).map(({rachis,...s})=>s)
   volume.structure.push({...vane,id:blade+'-rachis',tone:.69,sections:shafts,slender:true,member:id,part:'member'})
   if(scale==='50'&&i>=8){
    const gallery=[i===8?.79:.72,i===8?.815:.75,i===8?.85:.80,i===8?.87:.83].map(station)
    volume.body.push({...vane,id:blade+'-mirror',role:'detail',texture:true,tone:.95,sections:gallery.map(({rachis,...s})=>({...s,rx:r(s.rx+.00017),ry:r(s.ry*.985)}))})
   }
  }
  const rep=a.representations[scale]
  if(rep.status==='authored'){
   const base=rep.geometry.elevation.find(s=>s.id===primary)
   for(const [view,shapes] of Object.entries(rep.geometry)){
    const index=shapes.findIndex(q=>q.id===primary)
    if(index<0)continue
    const template=shapes[index];shapes.splice(index,1)
    for(let i=0;i<10;i++){
     const blade=volume.body.find(q=>q.id===label+'-primary-'+(i+1)),along=view==='elevation'?1:2,cross=3-along
     const slices=blade.sections.map(s=>{
      const q=new Quaternion(...s.rotation),ex=new Vector3(s.rx,0,0).applyQuaternion(q),ey=new Vector3(0,s.ry,0).applyQuaternion(q)
      return {at:s.at,h:[0,1,2].map(j=>Math.hypot(ex.getComponent(j),ey.getComponent(j)))}
     })
     const edge=sign=>slices.map(s=>[s.at[0],r(s.at[along]+sign*s.h[along])])
     const start=slices[0],end=slices.at(-1)
     const shape={...structuredClone(template),id:blade.id,bone:feather(i),points:[...edge(1),...edge(-1).reverse()],solid:{kind:'extruded',sweep:along===1?'w':'v',z0:r(start.at[cross]-start.h[cross]),z1:r(start.at[cross]+start.h[cross]),zTip:[r(end.at[cross]-end.h[cross]),r(end.at[cross]+end.h[cross])]}}
     delete shape.followTip;shapes.splice(index,0,shape)
     if(scale==='50'&&view==='elevation'){
      const axis=volume.structure.find(q=>q.member===feather(i)&&q.id.endsWith('-rachis')).sections
      {
       const ends=axis.slice(5,7).map(s=>{
        const q=new Quaternion(...s.rotation),u=new Vector3(s.rx,0,0).applyQuaternion(q),v=new Vector3(0,s.ry,0).applyQuaternion(q)
        return {at:s.at,h:[0,1,2].map(k=>Math.hypot(u.getComponent(k),v.getComponent(k)))}
       })
       const [a,b]=ends,corner=(s,sign)=>[s.at[0],r(s.at[1]+sign*s.h[1])]
       shapes.splice(index+1,0,{...shape,id:blade.id+'-exposed-rachis',role:'detail',penMm:.20,tone:.46,points:[corner(a,1),corner(b,1),corner(b,-1),corner(a,-1)],solid:{...shape.solid,z0:r(a.at[2]-a.h[2]),z1:r(a.at[2]+a.h[2]),zTip:[r(b.at[2]-b.h[2]),r(b.at[2]+b.h[2])]}})
      }
     }
    }
    for(const q of shapes.filter(s=>s.id===mark||s.id==='primary-lines'&&s.bone===hand.id)){
     q.bone=feather(8)
     if(view==='elevation'||q.solid)q.points=q.points.map(([x,y])=>[r(x-roots[8][0]),r(y-roots[8][1])])
     if(q.solid?.kind==='extruded'){q.solid.z0=r(q.solid.z0-roots[8][2]);q.solid.z1=r(q.solid.z1-roots[8][2]);if(q.solid.zTip)q.solid.zTip=q.solid.zTip.map(z=>r(z-roots[8][2]))}
    }
   }
  }
 }
}
cubitalFan(a,f.poseFraction)
a.structure.assumptions.push({en:'Ten primary feather shafts on each hand spread separately in flight. Their 5.61 g total per wing is taken from the measured hand-and-primary assembly, retaining its folded centre of mass in the sagittal plane.',de:'Zehn Handschwingen je Flügel spreizen sich im Flug einzeln. Ihre Masse ist aus der Hand-Schwingen-Gruppe ausgegliedert. Deren sagittaler Schwerpunkt bleibt in Faltstellung erhalten.'})
writeFileSync(process.argv[3],JSON.stringify(d))
