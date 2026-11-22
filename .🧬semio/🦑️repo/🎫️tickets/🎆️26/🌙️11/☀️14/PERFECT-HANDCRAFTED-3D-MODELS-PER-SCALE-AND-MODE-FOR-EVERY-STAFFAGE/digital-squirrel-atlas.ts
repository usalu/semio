/** 🐿️ Locate the caudal attachment, both original tail joints and the skull in each activity. */
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,V=q=>new Vector3(q.x,q.y,q.z),mm=v=>v.toArray().map(n=>Math.round(1000*n))
for(const p of a.poses.slice(0,6)){
 const f=evaluateDrawing(a,{kind:'pose',pose:p.id}).bones
 console.log(p.id,['tail-1','tail-2','head'].map(id=>{const b=a.rig.bones.find(q=>q.id===id),{o,u}=f[id].frame;return [id,mm(V(o)),mm(V(o).addScaledVector(V(u),b.lengthM))]}))
}
