/** 🌡️ Solves the human heat field in every named pose at its reference climate and at a common 14 °C shaded condition. */
import { createHash } from 'node:crypto'
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {catalogEntries,GENERATED} from './volumes-lib.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {solveThermal} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts'
const rows=[]
for(const e of catalogEntries().filter(e=>e.category==='humans')){
 const raw=readFileSync(e.path),a=JSON.parse(raw.toString()).asset,problems=[]
 const cold={...a.thermal.environment,airTempC:14,meanRadiantTempC:14,skyTempC:14,groundTempC:14,solarWM2:0,windMS:.3}
 const reference={}
 let worstBalanceW=0,subjectChecks=0
 for(const pose of a.poses){
  const state=evaluateDrawing(a,{kind:'pose',pose:pose.id})
  for(const env of [a.thermal.environment,cold]){
   const field=solveThermal(state,env)
   if(!field.body||!Number.isFinite(field.body.coreTempC))problems.push(pose.id+' missing body solution')
   for(const passenger of a.thermal.passengers??[])if(!Number.isFinite(field.passengers[passenger.id]?.coreTempC))problems.push(pose.id+' missing passenger '+passenger.id)
   for(const [subject,body] of [[null,field.body],...Object.entries(field.passengers)]){
    const regions=a.thermal.regions.filter(r=>r.body&&(r.body.subject??null)===subject)
    const surfaceLoss=regions.reduce((sum,r)=>sum+field.regions[r.id].conductedWM2*r.areaM2+field.regions[r.id].latentW,0)
    const work=subject===null?(pose.thermal.externalWorkW??0):0
    const balance=body.metabolicW-work-body.respiratorySensibleW-body.respiratoryLatentW-surfaceLoss-body.storageW
    worstBalanceW=Math.max(worstBalanceW,Math.abs(balance));subjectChecks++
    if(!Number.isFinite(balance)||Math.abs(balance)>.001)problems.push(pose.id+' '+(subject??'primary')+' energy balance '+balance+' W')
   }
   for(const r of a.thermal.regions){
    const v=field.regions[r.id]
    if(!v||!Number.isFinite(v.surfaceTempC)||!Number.isFinite(v.sensibleW)||!Number.isFinite(v.latentW))problems.push(pose.id+' '+r.id+' missing heat field')
    if(pose.id===a.presentation.defaultPose&&env===cold&&['headband','hair','spectacle-frame','badge','face-skin','ankle-socks','leg-skin','suspension-liner','socket','jeans','coat-torso','shoes','shoe','toddler-suit','toddler-face','boots'].includes(r.id))reference[r.id]=Number(v.surfaceTempC.toFixed(2))
   }
  }
 }
 rows.push({slug:e.slug,hash:createHash('sha256').update(raw).digest('hex'),poses:a.poses.length,problems,reference,subjectChecks,worstBalanceW})
 console.log(e.slug,a.poses.length,problems.length)
 Bun.gc(true)
}
writeFileSync(join(GENERATED,'hu-review','heat-field.json'),JSON.stringify(rows))
