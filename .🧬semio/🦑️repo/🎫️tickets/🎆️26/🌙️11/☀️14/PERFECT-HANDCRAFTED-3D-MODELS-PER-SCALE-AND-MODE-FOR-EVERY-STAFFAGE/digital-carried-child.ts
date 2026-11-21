/** 🧒 Fit the two-year-old’s lower cervical column, atlas and paired occipital joints in the carrying parent’s rig. */
import {readFileSync,writeFileSync} from 'node:fs'
const d=JSON.parse(readFileSync(process.argv[2],'utf8')),a=d.asset,f=JSON.parse(readFileSync(new URL('./digital-passenger-joints.json',import.meta.url),'utf8'))[a.id]
const r=x=>+x.toFixed(7),head=a.rig.bones.find(s=>s.id===f.head),c=f.atlas,m=head.mass.kg,h=m-f.neckMassKg
const joint={type:'revolute',minDeg:-20,maxDeg:24,twistMinDeg:-25,twistMaxDeg:25}
const mass=(kg,x,y)=>({kg:r(kg),comX:r(x),comY:r(y),distribution:'point',source:'authored-estimate'})
const neck={id:f.neck,parent:f.root,title:{en:'Toddler cervical column',de:'Halswirbelsäule des Kleinkinds'},x:head.x,y:head.y,angleDeg:head.angleDeg,lengthM:c[0],joint,mass:mass(f.neckMassKg,...f.neckCentreM)}
Object.assign(head,{parent:f.neck,x:c[0],y:c[1],angleDeg:0,lengthM:r(head.lengthM-c[0]),title:{en:'Toddler skull',de:'Schädel des Kleinkinds'},mass:mass(h,(m*head.mass.comX-f.neckMassKg*f.neckCentreM[0])/h-c[0],(m*head.mass.comY-f.neckMassKg*f.neckCentreM[1])/h-c[1])})
a.rig.bones.splice(a.rig.bones.indexOf(head),0,neck)
for(const p of a.poses){
 const lower=f.lowerPitch[p.id];p.bones[f.neck]={angleDeg:lower}
 p.bones[f.head].angleDeg-=lower
}
for(const q of a.anchors.filter(q=>q.bone===f.head)){q.x=r(q.x-c[0]);q.y=r(q.y-c[1])}
const fine=a.volumes['50'],skin=structuredClone(fine.body.find(q=>q.id===f.neck)),column=structuredClone(fine.structure.find(q=>q.id==='toddler-cervical-spine')),rootJoint=structuredClone(fine.structure.find(q=>q.id==='joint-toddler-head'))
const band=x=>{const [s,e]=skin.sections,t=(x-s.at[0])/(e.at[0]-s.at[0]),blend=(a,b)=>r(a+t*(b-a));return {...structuredClone(s),at:[x,blend(s.at[1],e.at[1]),0],rx:blend(s.rx,e.rx),ry:blend(s.ry,e.ry)}}
for(const [k,v] of Object.entries(a.volumes)){
 if(!v.body.some(q=>q.id===f.neck))v.body.push(structuredClone(skin))
 if(!v.structure.some(q=>q.id===column.id))v.structure.push(structuredClone(column))
 if(!v.structure.some(q=>q.id===rootJoint.id))v.structure.push(structuredClone(rootJoint))
 for(const q of [...v.body,...v.thermal,...v.structure].filter(q=>q.bone===f.head)){
  if([f.neck,column.id,rootJoint.id].includes(q.id)){q.bone=f.neck;if(q.member)q.member=f.neck;continue}
  q.transform??={};q.transform.translation=(q.transform.translation??[0,0,0]).map((x,i)=>r(x-c[i]))
  if(q.id==='mass-toddler-head')q.transform.translation=[head.mass.comX,head.mass.comY,0]
 }
 for(const side of [-1,1])v.structure.push({id:'toddler-occipital-'+(side>0?'l':'r'),kind:'ellipsoid',bone:f.head,material:'bone',role:'body',tone:.62,radii:[.007,.006,.005],transform:{translation:[0,0,side*.0085]},member:f.head,part:'joint'})
 v.thermal.push({...structuredClone(skin),id:'th-toddler-cervical-skin',bone:f.neck,region:'toddler-face',sections:[band(.012),band(.026)],capStart:false,capEnd:false})
 const rep=a.representations[k]
 if(rep.status==='authored')for(const [view,shapes] of Object.entries(rep.geometry))for(const q of shapes.filter(q=>q.bone===f.head)){
  if(q.id===f.neck){q.bone=f.neck;continue}
  if(q.solid)throw Error('Unexpected solid at '+q.id)
  if(view==='elevation')q.points=q.points.map(([x,y])=>[r(x-c[0]),r(y-c[1])])
 }
 v.decisions.push({en:'The carried toddler has a separate 61 mm cervical segment. The warm exposed neck follows the collar, while the face and hat follow the skull.',de:'Das getragene Kleinkind besitzt ein eigenes 61 mm langes Halssegment. Der warme freie Hals folgt dem Kragen, Gesicht und Mütze folgen dem Schädel.'})
}
a.structure.assumptions.push({en:'The carried child’s estimated 1.83 kg head and neck mass includes 0.21 kg of cervical tissue. The neutral combined centre of mass is preserved.',de:'Die geschätzte Kopf-Hals-Masse des getragenen Kindes von 1,83 kg enthält 0,21 kg Halsgewebe. Der gemeinsame Schwerpunkt in Neutralstellung bleibt erhalten.'})
writeFileSync(process.argv[3],JSON.stringify(d))
