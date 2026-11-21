/** 🐈 Inspect individual sitting-tail and paw landmarks in the carry frame before and after the feline study. */
import {readFileSync} from 'node:fs'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const asset=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,pose=asset.poses.find(p=>p.id==='sitting-tail-wrapped')
if(process.argv[3])for(const [id,angles] of Object.entries(JSON.parse(process.argv[3])))pose.bones[id]={angleDeg:angles[0],abductionDeg:angles[1],twistDeg:angles[2]??0}
const state=evaluateDrawing(asset,{kind:'pose',pose:pose.id})
for(const q of asset.rig.bones.filter(q=>q.id.startsWith('tail')||q.id.startsWith('paw'))){
 const b=state.bones[q.id].frame
 const mm=x=>Math.round(x*1000)
 console.log(q.id,...['x','y','z'].map(k=>mm(b.o[k])),...['x','y','z'].map(k=>mm(b.o[k]+b.u[k]*q.lengthM)))
}
