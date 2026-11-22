/** 🧒 Preserve each child's measured shafts, paired forearm and lower-leg bones, and joint sizes at smaller drawing scales. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./digital-schoolchild-case.json',import.meta.url),'utf8'))
if(!f.subjects[a.id])throw Error('No measured reference for '+a.id)
const limb=/^(arm|forearm|thigh|shank)-[lr]$/,own=s=>limb.test(s.member)&&['joint','member'].includes(s.part),fine=a.volumes[50].structure.filter(own)
const note={en:'Each leg retains the femoral shaft and knee, and the tibia and fibula stay separate. The upper arm, radius, ulna and joint shapes follow this child’s own limb lengths and centres. The smaller scales reduce the surface tessellation while keeping the measured shaft and joint contours.',de:'Jedes Bein behält den Femurschaft und das Knie; Schien- und Wadenbein bleiben getrennt. Oberarm, Speiche, Elle und Gelenkformen folgen den Gliedmaßenlängen und Gelenkzentren dieses Kindes. Die kleineren Maßstäbe verringern die Oberflächenunterteilung und bewahren die vermessenen Schaft- und Gelenkkonturen.'}
for(const [scale,vol] of Object.entries(a.volumes)){
 if(+scale>50){
  const start=vol.structure.findIndex(own),rest=vol.structure.filter(s=>!own(s)),replacement=fine.map(s=>structuredClone(s))
  rest.splice(Math.max(0,start),0,...replacement);vol.structure=rest
 }
 if(!vol.decisions.some(s=>s.en===note.en))vol.decisions.push(note)
}
writeFileSync(process.argv[3],JSON.stringify(doc))
