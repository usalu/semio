/** 🦴 Keep the person's anatomically shaped upper arms, forearms, thighs and lower legs in the structural model. */
import {readFileSync,writeFileSync} from 'node:fs'
const f=JSON.parse(readFileSync(new URL('./digital-limb-gauge.json',import.meta.url),'utf8')),doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,master=a.volumes['50'].structure
if(!f.subjects.includes(a.id))throw Error(a.id)
const members=new Set(master.filter(q=>/^(?:sk-)?(femur|tibia|humerus|ulna|radius)-[lr]$/.test(q.id)).map(q=>q.member))
const stock=q=>['member','joint'].includes(q.part)&&q.material==='bone'&&members.has(q.member)
for(const [scale,volume] of Object.entries(a.volumes)){
 if(scale==='50')continue
 const selected=master.filter(stock).map(q=>structuredClone(q))
 volume.structure=volume.structure.filter(q=>!stock(q)).concat(selected)
 const note={en:'The paired radii and ulnae, knee and shoulder ends and individual long-bone shafts keep this figure’s dimensions through each pose. The joints turn in the same skeleton at every display scale.',de:'Speichen und Ellen, Knie- und Schulterenden sowie die Schäfte der langen Knochen behalten die Maße dieser Figur in jeder Haltung. Die Gelenke drehen in allen Darstellungsmaßstäben im selben Skelett.'}
 if(!volume.decisions.some(v=>v.en===note.en))volume.decisions.push(note)
}
writeFileSync(process.argv[3],JSON.stringify(doc))
