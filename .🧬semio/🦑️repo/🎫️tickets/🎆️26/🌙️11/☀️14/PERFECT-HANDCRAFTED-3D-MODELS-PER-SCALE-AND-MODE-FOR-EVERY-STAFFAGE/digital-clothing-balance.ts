/** 🧥 Keep each passenger or primary figure’s total clothed body area at the authored outfit value. */
import {readFileSync,writeFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Decimal} from 'decimal.js'
const [file,target]=process.argv.slice(2),document=JSON.parse(readFileSync(file,'utf8')),a=document.asset
const f=JSON.parse(readFileSync(new URL('./digital-clothing-balance.json',import.meta.url),'utf8'))[a.id]
if(target){
 if(a.id==='parent-with-stroller')for(const v of Object.values(a.volumes)){
  for(let i=0;i<v.thermal.length;i++){
   const q=v.thermal[i]
   if(!/^child-(arm|leg)-[lr](-distal)?$/.test(q.of??''))continue
   const distal=q.of.endsWith('-distal')
   v.thermal[i]={...q,sectionFaces:[distal?'-u':'+u']}
  }
 }
 for(const [id,area] of Object.entries(f.garmentAreasM2))a.thermal.regions.find(q=>q.id===id).areaM2=area
 a.thermal.assumptions.push({en:'The exposed skin at the neckline and the outer clothing share the same total clothed body area as the original outfit. The skin and metabolic allocations remain tied to the individual subject.',de:'Die freiliegende Haut am Halsausschnitt und die Oberbekleidung behalten zusammen die ursprüngliche bekleidete Körperfläche. Die physiologischen Haut- und Stoffwechselanteile bleiben der jeweiligen Person zugeordnet.'})
}
const regions=a.thermal.regions.filter(q=>q.model==='body'&&q.areaBasis!=='exchange'&&(q.body.subject??null)===f.subject)
assert.equal(regions.reduce((d,q)=>d.plus(q.areaM2),new Decimal(0)).toString(),String(f.expectedTotalM2))
assert.equal(a.thermal.regions.find(q=>q.id===f.region).areaM2,f.skinM2)
if(target)writeFileSync(target,JSON.stringify(document))
console.log(a.id,'clothed area',f.expectedTotalM2,'m²')
