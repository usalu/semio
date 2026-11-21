/** 🔥 Compares the fuel, pan and rod temperatures and tank stratification through both climates and the prescribed operating cycle. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {solveThermal} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts'
import {solveStructure} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts'
for(const p of process.argv.slice(2)){
 const {asset}=JSON.parse(readFileSync(p,'utf8')),result={}
 for(const room of [asset.thermal.environment,{...asset.thermal.environment,airTempC:14,skyTempC:14,groundTempC:14,meanRadiantTempC:14,solarWM2:0,windMS:.3}]){
  const cases={}
  for(const pose of asset.poses){
   const state=evaluateDrawing(asset,{kind:'pose',pose:pose.id}),heat=solveThermal(state,room),structure=solveStructure(state)
   const err=Math.max(...Object.values(structure.residual).map(Math.abs));assert(err<.005,asset.id+' '+pose.id+' '+err)
   cases[pose.id]=Object.fromEntries(Object.entries(heat.regions).map(([k,v])=>{assert(Number.isFinite(v.sensibleW)&&Number.isFinite(v.surfaceTempC));return[k,Math.round(v.surfaceTempC*100)/100]}))
  }
  if(asset.id==='charcoal-grill'){const hot=cases['grilling-closed'],cold=cases['cold-closed'];for(const key of ['ash-catcher','charcoal-bed','cooking-grate'])assert(hot[key]>cold[key]+10);assert(hot['charcoal-bed']>hot['cooking-grate']&&hot['cooking-grate']>hot['ash-catcher'])}
  else for(const key of ['zone-1','connections','unions'])assert(cases.charged[key]>cases.cold[key])
  result[room.airTempC]=cases
 }
 let frames=0
 for(const clip of asset.clips)for(let i=0;i<33;i++){
  const state=evaluateDrawing(asset,{kind:'clip',clip:clip.id,timeS:clip.durationS*i/32}),field=solveStructure(state)
  assert(Math.max(...Object.values(field.residual).map(Math.abs))<.005);frames++
 }
 console.log(JSON.stringify({asset:asset.id,frames,result}))
}
