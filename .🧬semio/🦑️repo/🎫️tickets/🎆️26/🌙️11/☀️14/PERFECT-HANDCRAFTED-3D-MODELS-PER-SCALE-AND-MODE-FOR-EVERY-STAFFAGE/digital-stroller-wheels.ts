/** 👶 Fit the stroller wheels, caster forks, axle, parent grip and child's bumper at the four drawing scales. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,fine=structuredClone(a.volumes[50]),drawing=structuredClone(a.representations[50].geometry)
const hand=s=>/^hand-[lr]$/.test(s.bone),wheels=s=>['wheel-rear','wheel-front'].includes(s.bone),struts=s=>s.bone==='stroller'&&/^(?:push-(?:bar|frame)|handle-grip|bumper-bar|rear-(?:leg-[lr]|axle)|front-leg-[lr]|caster-[lr]|swivel-[lr]|fork-(?:crown-[lr]|prong-[ab]-[lr]))$/.test(s.id)
const notes={en:'Six open webs in each rear wheel and five in each caster connect the hub and rim. The rear axle, front fork, push grip and passenger bumper retain their physical positions.',de:'Sechs offene Stege je Hinterrad und fünf je Schwenkrad verbinden Nabe und Felge. Hinterachse, Vordergabel, Schiebegriff und Kinderbügel behalten ihre räumliche Lage.'}
for(const [key,v] of Object.entries(a.volumes)){
 const d=+key;if(d===50)continue
 const ids=new Set(v.body.filter(s=>wheels(s)||struts(s)||hand(s)).map(s=>s.id)),parts=fine.body.filter(s=>wheels(s)||struts(s)||hand(s)),fresh=new Set(parts.map(s=>s.id))
 v.body=v.body.filter(s=>!ids.has(s.id)).concat(structuredClone(parts))
 v.thermal=v.thermal.filter(s=>!ids.has(s.of)&&!wheels(s)&&!hand(s)).concat(structuredClone(fine.thermal.filter(s=>fresh.has(s.of))))
 v.structure=v.structure.filter(s=>!ids.has(s.of)&&!(wheels(s)&&s.part==='member')&&!/^hand-[lr]$/.test(s.member)).concat(structuredClone(fine.structure.filter(s=>fresh.has(s.of)||/^hand-[lr]$/.test(s.member))))
 if(!v.decisions.some(s=>s.en===notes.en))v.decisions.push(notes)
 const rep=a.representations[d]
 for(const [view,items] of Object.entries(rep.geometry)){
  const projected=drawing[view].filter(wheels).map(s=>{
   const old=items.find(b=>b.id===s.id.replace(/^wheel-/,'tyre-'))
   return {...structuredClone(s),z:(old?.z??s.z)+(s.id.startsWith('wheel-')?.02:0),penMm:d===500?.23:.18}
  })
  const plan=d===500&&view==='plan',contacts=plan?drawing.plan.filter(s=>/^(?:hand-[lr]|bumper-bar|handle-grip)$/.test(s.id)).map(s=>({...structuredClone(s),penMm:.21})):[]
  const named=new Set(contacts.map(s=>s.id))
  const retained=items.filter(s=>!wheels(s)&&!named.has(s.id)).map(s=>plan&&/^(?:chassis|front-leg-[lr])$/.test(s.id)?{...s,penMm:.11}:s)
  rep.geometry[view]=retained.concat(projected,contacts)
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
