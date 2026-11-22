/** ☝️ Fit the standing adult’s right wrist, thumb and four articulated finger rays across the authored drawing scales. */
import {readFileSync,writeFileSync} from 'node:fs'
import {restFrames} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./digital-hand-case.json',import.meta.url),'utf8'))
const r=n=>+n.toFixed(7),mm=n=>r(n/1000),wrist=a.rig.bones.find(b=>b.id===f.wrist)
const digits=Object.entries(f.digits),rayIds=name=>(name==='thumb'?['metacarpal','proximal','distal']:['proximal','middle','distal']).map(s=>name+'-'+s+'-r')
const members=new Set(digits.flatMap(([name])=>rayIds(name))),de={thumb:'Daumen',index:'Zeigefinger',middle:'Mittelfinger',ring:'Ringfinger',little:'Kleinfinger'}
a.rig.bones=a.rig.bones.filter(b=>!members.has(b.id))
const g=(kg,comX)=>({kg:r(kg),comX:r(comX),comY:0,distribution:'point',source:'authored-estimate'})
wrist.lengthM=.098;wrist.mass=g(f.massKg-digits.reduce((sum,[,d])=>sum+d.massG.reduce((a,b)=>a+b)/1000,0),.053)
for(const [name,ref] of digits){
 const ids=rayIds(name),root=ref.baseMm.map(mm)
 for(const [i,id] of ids.entries()){
  const level=name==='thumb'?['metacarpal','proximal','distal'][i]:['proximal','middle','distal'][i]
  const joint=({metacarpal:'Mittelhandknochen',proximal:'Grundglied',middle:'Mittelglied',distal:'Endglied'})[level]
  const b={id,parent:i?ids[i-1]:f.wrist,title:{en:'Right '+name+' '+level,de:de[name]+' rechts, '+joint},x:i?mm(ref.lengthMm[i-1]):root[0],y:i?0:root[1],z:i?0:root[2],lengthM:mm(ref.lengthMm[i]),angleDeg:i?0:ref.spread[i],joint:i?{type:'revolute',minDeg:-10,maxDeg:i===1?110:85}:{type:'revolute',minDeg:-25,maxDeg:30,twistMinDeg:-90,twistMaxDeg:0},mass:g(ref.massG[i]/1000,ref.lengthMm[i]*.46/1000)}
  a.rig.bones.push(b)
  for(const pose of a.poses){
   const posture=({relaxed:'relaxed',pointing:'pointing','arms-crossed':'crossed','hands-warming':'open'})[pose.id]
   const fan=i?0:name==='thumb'?(posture==='open'?12:posture==='pointing'?-18:0):posture==='open'?({index:7,middle:1,ring:-3,little:-9})[name]:0
   pose.bones[id]=i?{angleDeg:ref[posture][i],...(name==='thumb'?{abductionDeg:-ref.spread[i]}:{})}:{angleDeg:fan,abductionDeg:ref[posture][i],twistDeg:-90}
  }
 }
}
const frames=restFrames(a.rig),fine=a.volumes['50'],palm=structuredClone(fine.body.find(s=>s.id==='hand-r')),thenar=structuredClone(fine.body.find(s=>s.id==='thenar-r'))
const palmBones=fine.structure.filter(s=>s.bone===f.wrist&&['carpus-r','joint-wrist-r','metacarpal-r-2','metacarpal-r-3','metacarpal-r-4','metacarpal-r-5'].includes(s.id)).map(s=>structuredClone(s))
const fresh=(id,bone,material,role,tone)=>({id,bone,material,role,tone})
const turn=[0,-.7071067812,0,.7071067812]
const decision={en:'The right wrist carries a separate palm, five metacarpal rays and fourteen phalanges. The thumb opposes at its basal joint. The index straightens while the other fingers curl in the pointing pose. Both digital skin and bone follow the finger joints.',de:'Die rechte Handwurzel trägt die Handfläche, fünf Mittelhandstrahlen und vierzehn Fingerglieder. Der Daumen opponiert im Sattelgelenk. Beim Zeigen streckt sich der Zeigefinger, während sich die übrigen Finger beugen. Haut und Knochen folgen den Fingergelenken.'}
for(const [d,vol] of Object.entries(a.volumes)){
 const rep=a.representations[d],belongs=s=>s.bone===f.wrist||members.has(s.bone)
 const removed=new Set(vol.body.filter(belongs).map(s=>s.id))
 vol.body=vol.body.filter(s=>!belongs(s));vol.thermal=vol.thermal.filter(s=>!removed.has(s.of)&&!belongs(s));vol.structure=vol.structure.filter(s=>!belongs(s))
 const skin=s=>{vol.body.push(s);vol.thermal.push({id:'th-'+s.id,of:s.id,region:'hand-skin'})}
 skin(structuredClone(palm));skin(structuredClone(thenar));vol.structure.push(...structuredClone(palmBones))
 const drawing=[]
 for(const [name,ref] of digits){
  for(const [i,id] of rayIds(name).entries()){
   const len=mm(ref.lengthMm[i]),wide=mm(ref.radiusMm[i]),tip=mm(ref.radiusMm[i+1]),last=i===2
   const skinId='skin-'+id,st=(x,depth,breadth)=>({at:[r(x),0,0],rx:r(depth),ry:r(breadth),exponent:2.1,rotation:turn})
   const stations=[st(-wide*.60,wide*.48,wide*.53),st(0,wide*.92,wide),st(len*.42,wide*.95,wide*.96),st(len*.82,tip*.95,tip),st(len,tip*.84,tip*.91),st(len+tip*(last?.85:.58),tip*.39,tip*.44)]
   skin({...fresh(skinId,id,'skin','body',palm.tone),kind:'loft',sections:stations,capStart:true,capEnd:true})
   const rad=r(Math.max(.0038,wide*.49)),end=r(Math.max(.00375,tip*.50))
   vol.structure.push({...fresh('shaft-'+id,id,'bone','body',.85),kind:'capsule',p0:[.0015,0,0],p1:[r(len-.0015),0,0],r0:rad,r1:end,member:id,part:'member'})
   const headWidth=r(Math.max(.0039,wide*.61))
   vol.structure.push({...fresh('articulation-'+id,id,'bone','detail',.83),kind:'ellipsoid',radii:[r(Math.max(.0038,headWidth*.78)),headWidth,r(Math.max(.0038,headWidth*.87))],member:id,part:'joint'})
   const z=frames[id].frame.o[2]
   drawing.push({id:skinId,bone:id,role:'body',z:6.24+i*.002,filled:true,tone:.3,region:'hand-skin',kind:'capsule',x0:0,y0:0,x1:len,y1:0,r0:wide,r1:tip,solid:{kind:'extruded',sweep:'w',z0:r(z-wide*.88),z1:r(z+wide*.88)},...(+d>=200?{penMm:.20}:{})})
  }
 }
 const shape={id:'palm-r',bone:f.wrist,kind:'poly',closed:true,filled:true,role:'body',region:'hand-skin',z:6.2,tone:.3,points:[[-.015,-.027],[.044,-.038],[.082,-.039],[.100,-.029],[.104,-.016],[.108,.005],[.102,.03],[.081,.038],[.050,.039],[.022,.033],[-.015,.024]],solid:{kind:'extruded',sweep:'w',z0:r(frames[f.wrist].frame.o[2]-.015),z1:r(frames[f.wrist].frame.o[2]+.018)}}
 for(const [view,shapes] of Object.entries(rep.geometry)){
  const index=shapes.findIndex(belongs),kept=shapes.filter(s=>!belongs(s))
  kept.splice(index===-1?kept.length:index,0,structuredClone(shape),...structuredClone(drawing));rep.geometry[view]=kept
 }
 const words=[
  ['a mitten hand curling towards the thigh with a thumb along it instead of fingers','a grouped left hand and an articulated right hand'],
  ['eine zum Oberschenkel gekrümmte Fäustlingshand mit anliegendem Daumen statt Fingern','eine zusammengefasste linke Hand und eine bewegliche rechte Hand'],
  ['mitten hands with thumb','a grouped left hand with thumb and an articulated right hand'],['Fäustlingshände mit Daumen','eine zusammengefasste linke Hand mit Daumen und eine bewegliche rechte Hand'],
  ['mitten hand with thumb','grouped left fingers and an articulated right hand'],['Fäustlingshand mit Daumen','zusammengefasste linke Finger und eine bewegliche rechte Hand'],
  ['the three finger groups form one mitten','the left finger groups share one outline'],['die drei Fingergruppen bilden einen Fäustling','die linken Fingergruppen teilen sich einen Umriss'],
  ['a mitten hand without a thumb curling towards the thigh','a grouped left hand or the articulated right hand'],['eine zum Oberschenkel gekrümmte Fäustlingshand ohne Daumen','die zusammengefasste linke oder bewegliche rechte Hand'],
  ['mitten hands','grouped left and articulated right hands'],['Fäustlingshände','zusammengefasste linke und bewegliche rechte Hand'],
  ['a hand plate','a left hand plate and individual right phalanges'],['Thumb into the hand','Left thumb into the palm'],['Daumen in die Hand','Linker Daumen in die Hand'],
  ['Fingers, ear','Left fingers, ear'],['Finger, Ohr','Linke Finger, Ohr'],['Each hand is one filled shape','The left hand is one filled shape'],['Jede Hand ist eine gefüllte Form','Die linke Hand ist eine gefüllte Form'],
  ['forearm, mitten hand, thigh','forearm, hand, thigh'],['Unterarm, Fäustlingshand, Oberschenkel','Unterarm, Hand, Oberschenkel'],
  ['the hand is a mass of 72 mm','the left hand is a mass of 72 mm'],['die Hand ist eine Masse von 72 mm','die linke Hand ist eine Masse von 72 mm'],
  ['the hands are 43 mm thick','the left hand is 43 mm thick'],['die Hände sind 43 mm dick','die linke Hand ist 43 mm dick'],['sole line, thumb and fingers','sole line and left digits'],['Sohlenlinie, Daumen und Finger','Sohlenlinie und linke Finger']
 ]
 for(const label of [...vol.decisions,...Object.values(rep.decisions)])for(const language of ['en','de']){
  if(label?.[language])for(const [old,fresh] of words)label[language]=label[language].replace(old,fresh)
 }
 if(!vol.decisions.some(s=>s.en===decision.en))vol.decisions.push(decision)
}
writeFileSync(process.argv[3],JSON.stringify(doc))
