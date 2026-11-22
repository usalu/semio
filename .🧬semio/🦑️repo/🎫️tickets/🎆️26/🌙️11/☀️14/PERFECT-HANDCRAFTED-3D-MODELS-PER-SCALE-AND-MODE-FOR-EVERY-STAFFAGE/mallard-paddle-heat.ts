/** 🌊 Measure the outer skin area on both articulated paddles in the reference stance. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {exposedArea} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,v=a.volumes[50],m=worldSolids(a,evaluateDrawing(a,{kind:'pose',pose:'standing'}).bones,50)
const ids=new Set(v.thermal.filter(s=>s.region==='web-feet').map(s=>s.of))
console.log(exposedArea(m.filter(s=>ids.has(s.id)),m,[],()=>false,.00029,0))
