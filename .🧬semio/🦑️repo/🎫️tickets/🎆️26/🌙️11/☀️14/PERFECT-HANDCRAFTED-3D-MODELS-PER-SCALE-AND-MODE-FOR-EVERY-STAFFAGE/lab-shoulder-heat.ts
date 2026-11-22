/** 🌡️ Measure exposed trunk and forelimb fur areas on the standing 1:50 model. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {exposedArea} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
for(const path of process.argv.slice(2)){
 const asset=JSON.parse(readFileSync(path,'utf8')).asset,frames=evaluateDrawing(asset,{kind:'pose',pose:'standing-alert'}).bones
 const meshes=worldSolids(asset,frames,50),heat=asset.volumes['50'].thermal
 const areas={}
 for(const region of ['coat-trunk','coat-forelimbs']){
  const ids=new Set(heat.filter(q=>q.region===region).map(q=>q.of))
  const subset=meshes.filter(m=>ids.has(m.id))
  if(!subset.length)throw Error(region)
  areas[region]=exposedArea(subset,meshes,[],()=>false,.001,0)
 }
 console.log(JSON.stringify(areas))
}
