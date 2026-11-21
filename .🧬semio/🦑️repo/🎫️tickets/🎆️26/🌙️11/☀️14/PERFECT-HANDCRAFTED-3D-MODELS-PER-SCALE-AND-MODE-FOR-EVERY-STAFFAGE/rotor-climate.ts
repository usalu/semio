/** 🌡️ Solves each appliance at rest and under both operating stages, including the alternating HRV core boundary. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {solveThermal} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts'
for(const path of process.argv.slice(2)){
 const {asset}=JSON.parse(readFileSync(path,'utf8')),rows=[]
 for(const room of [asset.thermal.environment,{...asset.thermal.environment,airTempC:14,groundTempC:14,meanRadiantTempC:14,skyTempC:14,solarWM2:0,windMS:.3}]){
  const results={}
  for(const pose of asset.poses){
   const state=evaluateDrawing(asset,{kind:'pose',pose:pose.id})
   const field=solveThermal(state,room)
   const values=Object.fromEntries(Object.entries(field.regions).map(([k,q])=>{for(const n of [q.surfaceTempC,q.steadyTempC,q.sensibleW,q.netRadiativeW])assert(Number.isFinite(n));return [k,Math.round(q.surfaceTempC*1000)/1000]}))
   results[pose.id]=values
  }
  if(asset.id==='decentralised-hrv-fan')assert(results.extract.core>results.supply.core)
  else {const id=asset.id==='duct-fan'?'hub':'motor';assert(results.high[id]>results.low[id]&&results.low[id]>results.off[id])}
  rows.push(results)
 }
 console.log(JSON.stringify({asset:asset.id,rows}))
}
