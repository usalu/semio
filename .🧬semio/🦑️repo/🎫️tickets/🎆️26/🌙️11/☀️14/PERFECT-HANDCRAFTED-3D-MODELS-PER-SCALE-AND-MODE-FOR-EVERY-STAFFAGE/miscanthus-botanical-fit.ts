/** 🌾 Keep separated, folded Miscanthus blades and open panicle fingers on the original wind joints. */
import {readFileSync,writeFileSync} from 'node:fs'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,v=a.volumes[500],medium=a.volumes[200]
const keep=new Set<string>(),tillers=new Map<string,any[]>()
for(const q of medium.body){
 if(!q.id.startsWith('blade-')){keep.add(q.id);continue}
 const tuft=q.id.slice(0,q.id.lastIndexOf('-')),group=tillers.get(tuft)??[];group.push(q);tillers.set(tuft,group)
}
for(const group of tillers.values()){
 group.sort((l,r)=>Number(l.id.split('-').at(-1))-Number(r.id.split('-').at(-1)))
 keep.add(group[0].id);keep.add(group.at(-1).id)
 if(group.length>=5)keep.add(group[Math.floor(group.length/2)].id)
}
for(const q of v.body)if(q.id.startsWith('blade-'))keep.add(q.id)
const pose=posedMesh(a,a.presentation.defaultPose),quality=qualityOf(a,200),outer=[null,null,null,null]
for(const q of medium.body.filter(s=>s.id.startsWith('blade-'))){
 const mesh=pose(q,quality)
 for(let j=0;j<mesh.world.length;j+=3)for(const [i,value] of [mesh.world[j],-mesh.world[j],mesh.world[j+2],-mesh.world[j+2]].entries())
  if(!outer[i]||value>outer[i].value)outer[i]={id:q.id,value}
}
for(const tip of outer)keep.add(tip.id)
for(const s of medium.structure)if(s.of)keep.add(s.of)
v.body=medium.body.filter(s=>keep.has(s.id)).map(s=>structuredClone(s))
v.thermal=medium.thermal.filter(s=>s.of&&keep.has(s.of)).map(s=>structuredClone(s))
for(const e of medium.thermal.filter(s=>s.tags?.includes('emitter'))){
 const support=v.body.find(q=>q.bone===e.bone&&q.id.startsWith('blade-'))
 if(!support)throw Error(e.id)
 v.thermal.push({id:e.id,region:e.region,tags:e.tags,of:support.id})
}
const reference=new Map(medium.structure.map(q=>[q.id,q]))
v.structure=v.structure.map(q=>q.of?.startsWith('lobe-')?structuredClone(reference.get(q.id)):q)
for(const [view,shapes] of Object.entries(a.representations[200].geometry))a.representations[500].geometry[view]=structuredClone(shapes.filter(s=>keep.has(s.id)))
v.decisions.unshift({en:'The outside and inside blades of each tiller remain separate, including a middle blade on larger fans. Four fingers retain the open panicle on each flowering culm. Structural members follow the culms and the individual terminal blades.',de:'Außen- und Innenblatt jedes Halms bleiben getrennt, an größeren Fächern zusätzlich das Mittelblatt. Vier Finger gliedern die Rispe jedes Blütenhalms. Die Struktur folgt den Halmen und einzelnen endständigen Blättern.'})
writeFileSync(process.argv[3],JSON.stringify(doc))
