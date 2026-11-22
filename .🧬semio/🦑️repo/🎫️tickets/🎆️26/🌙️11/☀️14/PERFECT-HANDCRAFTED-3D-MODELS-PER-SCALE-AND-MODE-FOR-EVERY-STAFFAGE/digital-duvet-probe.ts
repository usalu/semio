/** 🛌 Inspect mattress coordinates and the covered tissue surface of the sleeper in each resting pose. */
import {readFileSync} from 'node:fs'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
for(const pose of a.poses){
 const b=evaluateDrawing(a,{kind:'pose',pose:pose.id}).bones,xyz=p=>[p.x,p.y,p.z].map(v=>+v.toFixed(3));console.log(pose.id,Object.fromEntries(['bed-surface','duvet','torso','pelvis','thigh-r','shank-r','foot-r','hand-r','head'].map(k=>[k,[xyz(b[k].frame.o),xyz(b[k].tip)]])))
 const m=posedMesh(a,pose.id),q=qualityOf(a,50);let pts=[]
 for(const s of a.volumes[50].body.filter(p=>['torso','pelvis','thigh-l','thigh-r','shank-l','shank-r'].includes(p.bone)&&p.material!=='duvet-cover')){
  const {world}=m(s,q);for(let i=0;i<world.length;i+=3)pts.push([...world.slice(i,i+3)])
 }
 for(let x=.6;x<1.87;x+=.15){let row=[+x.toFixed(2)];for(let z=-.45;z<.46;z+=.09){const p=pts.filter(p=>Math.abs(p[0]-x)<.075&&Math.abs(p[2]-z)<.045);row.push(+Math.max(0,...p.map(c=>c[1]-.46)).toFixed(2))}console.log(row.join(' '))}
}
