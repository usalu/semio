/** 🪶 Locate the separation between the carpal feathers on the folded and flared wings. */
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,vec=p=>new Vector3(p.x,p.y,p.z)
for(const p of a.poses){
 const b=evaluateDrawing(a,{kind:'pose',pose:p.id}).bones
 for(const side of ['n','f']){
  const world=(id,t)=>{const s=b[id+'-'+side],l=a.rig.bones.find(q=>q.id===id+'-'+side).lengthM;return vec(s.frame.o).addScaledVector(vec(s.frame.u),l*t)}
  const d=(i,j,t)=>Math.round(world(i,t).distanceTo(world(j,t))*1000)
  console.log(p.id,side,'S1-P1',...[0,.2,.4,.6,.8,1].map(t=>d('s1','p1',t)),'S2-S1',d('s2','s1',.5),'P1-P2',d('p1','p2',.5),'angle',Math.round(180/Math.PI*vec(b['s1-'+side].frame.u).angleTo(vec(b['p1-'+side].frame.u))))
 }
}
