/** 🌡️ Measure the external and closed surfaces of the two gland halves and teats in their standing position. */
import {readFileSync} from 'node:fs'
import {Vector3,Triangle} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {exposedArea} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,fr=evaluateDrawing(a,{kind:'pose',pose:'standing-alert'}).bones
for(const pose of a.poses){
const fr=evaluateDrawing(a,{kind:'pose',pose:pose.id}).bones
const bodies=worldSolids(a,fr,50),picked=bodies.filter(b=>b.bone==='mammary-suspension')
let total=0
for(const b of picked){for(let i=0;i<b.indices.length;i+=3){const xyz=j=>new Vector3().fromArray(b.positions,3*b.indices[i+j]);total+=new Triangle(xyz(0),xyz(1),xyz(2)).getArea()}}
console.log(JSON.stringify({pose:pose.id,closedM2:total,freeM2:exposedArea(picked,bodies,[],()=>false,.001,0),isolatedM2:exposedArea(picked,picked,[],()=>false,.001,0)}))
}
