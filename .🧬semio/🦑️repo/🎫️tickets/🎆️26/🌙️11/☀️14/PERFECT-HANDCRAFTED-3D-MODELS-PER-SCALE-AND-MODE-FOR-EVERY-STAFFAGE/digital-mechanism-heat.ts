/** 🌡️ Measure contact and accessible heat exchange on the moving links in each authored pose. */
import {readFileSync} from 'node:fs'
import {measureVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
const [file,region,pose]=process.argv.slice(2),a=JSON.parse(readFileSync(file,'utf8')).asset
a.thermal.regions.find(r=>r.id===region).areaBasis='total'
for(const p of a.poses.filter(p=>!pose||p.id===pose)){
 const items=measureVolumes(a,{scale:50,pose:p.id,rules:['volumes-thermal-coverage']})
 const read=s=>items.find(q=>q.name==='info:'+s+':'+region).value
 console.log(JSON.stringify({pose:p.id,total:read('totalAreaM2'),outer:read('outerAreaM2'),air:read('exposedAreaM2')}));Bun.gc(true)
}
