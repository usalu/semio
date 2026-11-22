/** 🐿️ Measure physical fine-scale exchange areas of the digits and open facial surfaces. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {exposedArea} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,v=a.volumes[50],m=worldSolids(a,evaluateDrawing(a,{kind:'pose',pose:'sitting-eating'}).bones,50).filter(q=>!q.bone.startsWith('tail'))
for(const r of ['nose','eyes','hands-skin','feet-skin']){
 const ids=new Set(v.thermal.filter(x=>x.of&&x.region===r).map(s=>s.of))
 console.log(r,exposedArea(m.filter(s=>ids.has(s.id)),m,[],()=>false,.00015,0))
}
