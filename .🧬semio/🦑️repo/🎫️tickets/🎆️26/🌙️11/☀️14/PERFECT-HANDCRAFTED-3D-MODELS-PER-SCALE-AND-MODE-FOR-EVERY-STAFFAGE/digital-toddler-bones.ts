/** 👶 Shape the carried child's femora, humeri, paired forearms and shanks around the authored joint centres. */
import {readFileSync,writeFileSync} from 'node:fs'
const [src,out]=process.argv.slice(2),doc=JSON.parse(readFileSync(src,'utf8')),a=doc.asset,fine=a.volumes['50'],turn=[0,-.7071067812,0,.7071067812]
const profiles={
 femur:[[2,0,0,9,10],[14,-1,7,10,10],[33,-1,8,8.2,8.6],[85,2,4,7.5,8],[145,0,1,8,8.6],[169,-2,0,12,10],[174,-2,0,10,9]],
 tibia:[[4,4,-4,11,9.5],[21,4,-4,9,8],[85,2,-4,6.6,7.3],[145,2,-4,6.8,7.3],[165,2,-4,8,7.8]],
 humerus:[[3,0,0,10,9],[16,1,2,8,7.5],[64,2,12,6.8,6.2],[110,0,22,6.5,6.8],[129,0,26,9.8,8]],
 radius:[[5,8,0,4.4,4.4],[24,8,-6,4.1,4],[59,7,-16,4,4.1],[90,6,-25,4.5,4.8],[103,6,-29,5.8,5]],
 ulna:[[0,-7,0,5.7,5],[18,-7,-5,4.8,4.4],[59,-7,-16,4.2,4.1],[91,-6,-25,4,3.9],[103,-6,-29,4.3,4]],
}
const seg={femur:'thigh',tibia:'shank',humerus:'arm',radius:'forearm',ulna:'forearm'}
const own=s=>s.id.includes('toddler')&&s.part==='member',adult=s=>s.part==='member'&&/^(thigh|shank|upper-arm|forearm|foot|toe)-[lr]$/.test(s.member)
const axial=s=>s.part==='member'&&!s.id.includes('toddler')&&['torso','pelvis','hand-l','hand-r'].includes(s.member)
const child=fine.structure.filter(own).filter(s=>!/^toddler-(femur|tibia|humerus|radius-ulna|radius|ulna)-[lr]$/.test(s.id)).map(s=>structuredClone(s))
for(const side of ['l','r'])for(const [part,rows] of Object.entries(profiles)){
 const sign=side==='l'?1:-1,bone='toddler-'+seg[part]+'-'+side
 child.push({id:'toddler-'+part+'-'+side,bone,kind:'loft',material:'bone',role:'body',tone:.62,member:bone,part:'member',capStart:true,capEnd:true,sections:rows.map(([x,y,z,w,d])=>({at:[x/1000,y/1000,sign*z/1000],rx:w/1000,ry:d/1000,rotation:turn,exponent:2.15}))})
}
const adultBones=fine.structure.filter(adult).map(s=>structuredClone(s)),markers=fine.structure.filter(s=>['joint','mass'].includes(s.part))
function scaleBone(s,d){
 const out=structuredClone(s),v=out.sections
 if(v?.length>3&&+d>=200){const step=+d===200?2:3;out.sections=v.filter((p,i)=>i===0||i===v.length-1||i===Math.floor(v.length/2)||i%step===0)}
 return out
}
const decision={en:'Adult and child bones retain their own calibre. The paired radius and ulna and the tibia and fibula stay separate through the carry. At smaller drawing scales the section stations and number of rib levels decrease while the material dimensions stay physical.',de:'Die Knochen von Elternteil und Kind behalten ihr eigenes Kaliber. Speiche und Elle sowie Schien- und Wadenbein bleiben beim Tragen getrennt. In kleineren Darstellungsmaßstäben werden Querschnittsstationen und Rippenebenen reduziert, die Materialmaße bleiben erhalten.'}
for(const [d,vol] of Object.entries(a.volumes)){
 const before=vol.structure
 const ribSet={'50':[1,2,3,4,5,6,7,8,9,10,11,12],'100':[1,3,5,7,9,11],'200':[2,7,10],'500':[2,7]}[d]
 const c=child.filter(s=>{const m=/^toddler-costal-(\d+)-/.exec(s.id);return !m||ribSet.includes(+m[1])}).map(s=>scaleBone(s,d))
 const adults=adultBones.map(s=>scaleBone(s,d))
 if(+d>50){
  for(const p of fine.structure.filter(axial)){
   const rib=/(?:costal|rib)-(\d+)-/.exec(p.id);if(rib&&!ribSet.includes(+rib[1]))continue
   if(/^bones-[lr]-/.test(p.id)&&!p.id.endsWith('-metacarpal')&&!/^bones-[lr]-(finger-\d|thumb)$/.test(p.id))continue
   adults.push(scaleBone(p,d))
  }
 }
 const references=new Map(markers.map(s=>[s.id,s]))
 vol.structure=before.filter(s=>!own(s)&&!adult(s)&&!(+d>50&&axial(s))).map(s=>references.has(s.id)?structuredClone(references.get(s.id)):s)
 vol.structure.push(...c,...adults)
 if(!vol.decisions.some(q=>q.en===decision.en))vol.decisions.push(decision)
}
writeFileSync(out,JSON.stringify(doc))
