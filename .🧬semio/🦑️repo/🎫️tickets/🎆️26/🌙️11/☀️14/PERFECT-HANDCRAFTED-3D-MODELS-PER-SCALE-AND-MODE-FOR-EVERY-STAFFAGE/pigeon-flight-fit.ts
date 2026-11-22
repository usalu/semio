/** 🕊️ Bind the shoulders and primary fans to the articulated humeri and manus at 1:200. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,v=a.volumes[200],ref=a.volumes[50]
for(const side of ['near','far'])for(const part of ['primaries','shoulder']){
 const id=side+'-'+part,src=ref.body.find(q=>q.id===id),target=v.body.find(q=>q.id===id)
 const stops=part==='primaries'?[0,2,3,5,7]:[0,3,7,10,14]
 Object.assign(target,src,{sections:src.sections.filter((_,i)=>stops.includes(i)||i===src.sections.length-1)})
}
v.decisions[0]={en:'The shoulder feathers follow the humerus, the coverts follow the forearm and the long primaries open from the manus. Five longitudinal stations retain the physical wrist-to-tip taper. The thighs merge into the trunk, the three neck segments form one neck, and the bill and head form one mass. Each foot and shank keeps its separate slender silhouette.',de:'Die Schulterfedern folgen dem Oberarm, die Deckfedern dem Unterarm und die langen Handschwingen öffnen sich mit dem Handflügel. Fünf Längsschnitte erhalten die Verjüngung von der Handwurzel zur Spitze. Die Schenkel gehen im Rumpf auf, die drei Halsabschnitte bilden einen Hals, Kopf und Schnabel einen gemeinsamen Körper. Lauf und Fuß behalten ihre schlanken Konturen.'}
const r=a.representations[200]
r.pens.body=.2
for(const [view,shapes] of Object.entries(r.geometry)){
 const articulated=a.representations[100].geometry[view].filter(q=>/^(far|near)-(primaries|coverts|shoulder)(-|$)/.test(q.id))
 r.geometry[view]=[...shapes.filter(q=>!/^((far|near)-wing|((far|near)-(primaries|coverts|shoulder)))(-|$)/.test(q.id)),...articulated].sort((a,b)=>a.z-b.z)
}
r.decisions.merged={en:'The bill and cere merge into the head, the neck segments into one mass, the thigh into the trunk. Shoulder, coverts and primaries keep separate drawing contours for flight.',de:'Schnabel und Wachshaut gehen im Kopf auf, die Halsabschnitte in einer Masse, die Schenkel im Rumpf. Schulter-, Deck- und Handschwingen behalten für den Flug getrennte Konturen.'}
writeFileSync(process.argv[3],JSON.stringify(doc))
