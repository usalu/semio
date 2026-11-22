/** 🎸 Locate each wrist, palm and fingertip in the local frame of the guitar. */
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,v=p=>new Vector3(p.x,p.y,p.z),out=p=>p.toArray().map(x=>Math.round(x*1000)),q=qualityOf(a,50)
for(const pose of a.poses){
 const b=evaluateDrawing(a,{kind:'pose',pose:pose.id}).bones,g=b.guitar.frame
 const local=p=>{const delta=p.clone().sub(v(g.o));return new Vector3(delta.dot(v(g.u)),delta.dot(v(g.v)),delta.dot(v(g.w)))}
 const m=posedMesh(a,pose.id)
 const at=id=>out(local(v(b[id].frame.o))),tip=id=>out(local(v(b[id].tip)))
 const digits=[];for(const id of ['palm-l','thumb-l','finger-1-l','finger-2-l','finger-3-l','finger-4-l','index-distal-l']){
  const s=a.volumes[50].body.find(q=>q.id===id)||a.volumes[50].body.find(q=>q.bone===id);if(!s)continue
  const points=m(s,q).world;const extent=[[],[],[]]
  for(let i=0;i<points.length;i+=3){const p=out(local(new Vector3().fromArray(points,i)));p.forEach((x,k)=>extent[k].push(x))}
  digits.push([id,extent.map(arr=>[Math.min(...arr),Math.max(...arr)])])
 }
 console.log(pose.id,JSON.stringify({shoulder:at('upper-arm-l'),elbow:at('forearm-l'),wrist:at('hand-l'),tip:tip('hand-l'),pick:at('hand-r'),digits}))
}
