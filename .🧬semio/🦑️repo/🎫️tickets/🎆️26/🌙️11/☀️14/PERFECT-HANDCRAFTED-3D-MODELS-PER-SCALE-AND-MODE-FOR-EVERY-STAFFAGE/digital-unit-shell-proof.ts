/** 🌳 Measure a single dense botanical view with the exact product masks, pose, resolution and acceptance limits. */
import {createHash} from 'node:crypto'
import {readFileSync,writeFileSync,renameSync} from 'node:fs'
import {join} from 'node:path'
import {catalogEntries} from './volumes-lib.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {collector,featureOf,VOLUME_LIMITS} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts'
import {measureView,worldSolids,VIEWPOINTS} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const [slug,file]=process.argv.slice(2),k=200,e=catalogEntries().find(p=>p.slug===slug)!,raw=JSON.parse(readFileSync(e.path,'utf8')).asset,trial=JSON.parse(readFileSync(file,'utf8')).asset
const base=raw.volumes[k].body, next=trial.volumes[k].body
const added=next.filter(s=>!base.some(b=>b.id===s.id)),removed=base.filter(s=>!next.some(b=>b.id===s.id))
function inspect(label,body){const a={...trial,volumes:{...trial.volumes,[k]:{...trial.volumes[k],body}}};const state=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose},{kind:'scale',scale:{denominator:k}});const m=measureView(a,state,worldSolids(a,state.bones,k).filter(p=>p.texture!==true),'elevation',k,95,.045);console.log('[DEBUG]',label,m.iou.toFixed(3),m.p95M.toFixed(3));Bun.gc(true)}
inspect('baseline',base);inspect('candidate',next);console.log('added',added.map(s=>s.id),'removed',removed.map(s=>s.id));
inspect('candidate with true ring triangles',next.map(s=>s.slender?{...s,slender:false}:s));
for(const s of removed)inspect('restore '+s.id,[...next,s]);
for(const tag of ['guard','fan','motor','side','bell','coil']){
 const parts=base.filter(s=>s.id.includes(tag));inspect('baseline '+tag,[...next.filter(s=>!s.id.includes(tag)),...parts]);
}
