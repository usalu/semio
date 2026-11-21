/** 🪟 Checks mesh occupancy at the ash passages, lid perforations, and low-fire shutter positions. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {insideAny} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const {asset}=JSON.parse(readFileSync(process.argv[2],'utf8')),point=(x,y,z)=>[x,y,z]
for(const scale of [50,100,200,500]){
 const throat=asset.volumes[scale].body.find(q=>q.id==='vent-dial')
 const upper=throat.holes.map(h=>h.reduce(([x,y],p)=>[x-p[0]/h.length,y+p[1]/h.length],[0,0]))
 const lower=Array.from({length:3},(_,i)=>[.04*Math.cos(i*2*Math.PI/3),.04*Math.sin(i*2*Math.PI/3)])
 for(const [pose,open]of [['grilling-closed',true],['embers-vents-low',false]]){
  const state=evaluateDrawing(asset,{kind:'pose',pose}),solids=worldSolids(asset,state.bones,scale)
  const bowl=solids.filter(q=>['bowl-shell','damper-plate'].includes(q.id)),lid=solids.filter(q=>['lid-shell','vent-dial','vent-skirt'].includes(q.id))
  let rays=0
  for(const [holes,sol,min,max]of [[lower,bowl,.465,.51],[upper,lid,.970,1.005]]){
   for(const [x,z]of holes)for(const dx of [-.002,0,.002]){
    let hits=0
    for(let y=min;y<=max;y+=.00035)if(insideAny(sol,point(x+dx,y,z)))hits++
    assert(open?hits===0:hits>0,JSON.stringify({scale,pose,x,z,hits}));rays++
   }
  }
  console.log(JSON.stringify({scale,pose,rays}))
 }
}
