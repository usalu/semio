/** 🦮 Measure scapular and cervical contour stations in metres along the standing dog's back. */
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,V=q=>new Vector3(q.x,q.y,q.z),r=x=>+x.toFixed(3)
const fr=evaluateDrawing(a,{kind:'pose',pose:'standing-alert'}).bones
for(const q of a.volumes['50'].body.filter(s=>['chest','hindbody','croup','neck','shoulder-n','upper-arm-n','forearm-n'].includes(s.id))){
 const b=fr[q.bone].frame,local=s=>V(b.o).addScaledVector(V(b.u),s[0]).addScaledVector(V(b.v),s[1]).addScaledVector(V(b.w),s[2]).toArray().map(r)
 console.log(q.id,q.bone,JSON.stringify(q.sections.map(s=>[...local(s.at),s.rx,s.ry])))
}
