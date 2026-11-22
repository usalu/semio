/** 🧵 Trace sewn crown and lower lining corners through the authored turn-over animation. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,vol=a.volumes['50'],panels=vol.body.filter(s=>s.id.startsWith('baffle-'))
const clip=a.clips.find(c=>c.id==='turn-over'),times=Array.from({length:97},(_,i)=>clip.durationS*i/96)
const v=p=>new Vector3(p.x,p.y,p.z),cases=[...a.poses.map(p=>({kind:'pose',pose:p.id})),...times.map(timeS=>({kind:'clip',clip:clip.id,timeS}))]
const fixture=JSON.parse(readFileSync(new URL('./digital-sleeping-duvet-case.json',import.meta.url),'utf8'))
let worst=0;const intervals=new Set()
for(const state of cases){
 const b=evaluateDrawing(a,state).bones,pts=new Map();let error=0
 for(const s of panels){
  const [,i0,j0,b0]=s.id.split('-'),i=+i0,j=+j0,h=+b0
  const corners=h===1?[[i,j],[i+1,j],[i+1,j+1]]:[[i,j],[i+1,j+1],[i,j+1]]
  const selected=s.kind==='extrude'?s.profile.slice().reverse().map(([z,x])=>[x,0,z]):s.vertices.slice(0,3)
  for(const [n,point] of selected.entries()){
   const p=new Vector3()
   for(const w of s.weights){const f=b[w.bone].frame;p.addScaledVector(v(f.o).addScaledVector(v(f.u),point[0]).addScaledVector(v(f.v),point[1]).addScaledVector(v(f.w),point[2]),w.weight)}
   const key=corners[n].join('-');if(pts.has(key))error=Math.max(error,p.distanceTo(pts.get(key)));else pts.set(key,p)
  }
 }
 if(state.pose)console.log(state.pose,error*1000);if(error>.002&&!state.pose)intervals.add(Math.floor(state.timeS*4)/4);worst=Math.max(worst,error)
}
console.log('Maximum corner gap through 97 frames in millimetres',worst*1000,[...intervals]);assert(worst<fixture.seamLimitM)
