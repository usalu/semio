/** 🧊 Measure the mixer cab's steel, laminated glazing and sheltered controls in the same pose. */
import {readFileSync} from 'node:fs'
import {measureVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
const [file,pose]=process.argv.slice(2),a=JSON.parse(readFileSync(file,'utf8')).asset,ids=['cab','cab-glazing','cab-trim']
for(const q of a.thermal.regions.filter(q=>ids.includes(q.id)))q.areaBasis='total'
const measures=measureVolumes(a,{scale:50,pose,rules:['volumes-thermal-coverage']})
const fields=Object.fromEntries(ids.map(id=>[id,Object.fromEntries(['outerAreaM2','exposedAreaM2','totalAreaM2'].map(k=>[k,measures.find(m=>m.name==='info:'+k+':'+id)?.value]))]))
console.log(JSON.stringify({pose,fields}))
