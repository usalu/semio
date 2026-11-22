/** 🕊️ Compare the posed primary tips with the finest physical wings throughout the flight and walk sequence. */
import {readFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./pigeon-flight-case.json',import.meta.url),'utf8'))
const failures=new Set(),v=p=>new Vector3(p.x,p.y,p.z),point=(fr,p)=>v(fr.o).addScaledVector(v(fr.u),p[0]).addScaledVector(v(fr.v),p[1]).addScaledVector(v(fr.w),p[2])
for(const k of f.scales){
 const vol=a.volumes[k],rep=a.representations[k];let worst=0
 for(const side of f.sides){
  const suffix=side==='near'?'n':'f'
  for(const [field,bone] of Object.entries(f.wingBones)){
   const id=side+'-'+field,s=vol.body.find(s=>s.id===id),carrier=bone+'-'+suffix
   if(s?.bone!==carrier||!vol.thermal.some(t=>t.of===id&&t.region===f.featherRegion))failures.add(k+' '+id+' attachment')
   if(rep.status==='authored')for(const [view,shapes] of Object.entries(rep.geometry))if(!shapes.some(q=>q.bone===carrier))failures.add(k+' '+view+' '+carrier)
  }
  const solid=vol.body.find(s=>s.id===side+'-primaries'),ref=a.volumes[50].body.find(s=>s.id===solid.id)
  for(const pose of a.poses){
   const bones=evaluateDrawing(a,{kind:'pose',pose:pose.id}).bones
   const tip=s=>point(bones[s.bone].frame,s.sections.at(-1).at)
   const dist=tip(solid).distanceTo(tip(ref));worst=Math.max(worst,dist)
   if(dist>f.tipToleranceM)failures.add(k+' '+pose.id+' '+side+' tip '+dist.toFixed(3))
  }
 }
 console.log({scale:k,poses:a.poses.length,worstTipDifferenceM:worst})
}
if(failures.size){console.error([...failures]);process.exitCode=1}
