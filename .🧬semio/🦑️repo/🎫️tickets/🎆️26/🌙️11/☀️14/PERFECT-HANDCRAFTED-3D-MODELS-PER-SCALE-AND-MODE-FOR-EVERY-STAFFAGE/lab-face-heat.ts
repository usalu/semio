/** 🌡️ Measure uncovered fur, nasal skin and wet eye geometry for the facial exchange fields. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {exposedArea} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,body=worldSolids(a,evaluateDrawing(a,{kind:'pose',pose:process.argv[3]||'standing-alert'}).bones,50)
for(const region of ['eyes','nose-skin','coat-head','tongue']){
 const ids=new Set(a.volumes['50'].thermal.filter(q=>q.region===region&&q.of).map(s=>s.of)),ss=body.filter(s=>ids.has(s.id))
 console.log(region,exposedArea(ss,body,[],()=>false,.00015,0))
}
