/** 📦 Locate the carton and garment edges visible through each orthographic drawing of the lift. */
import {readFileSync} from 'node:fs'
import {DEFAULT_PRINT_PROFILE,measureSuitability} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts'
const [source,study]=process.argv.slice(2),original=JSON.parse(readFileSync(source,'utf8')).asset,a=JSON.parse(readFileSync(study,'utf8')).asset
const rep=a.representations['500']
for(const view of Object.keys(rep.geometry)){
 const geom=structuredClone(rep.geometry)
 for(const k of Object.keys(geom))if(k!==view)geom[k]=[]
 const box=geom[view].find(q=>q.id==='box'),shape=original.representations['500'].geometry[view].find(q=>q.id==='box')
 for(const pose of a.poses){
  const sub={...a,poses:[pose]}
  for(const [label,points] of [['original',shape.points],['enlarged',box.points]]){
   const draw={...rep,geometry:{...geom,[view]:geom[view].map(q=>q.id==='box'?{...q,points}:q)}}
   const k=label==='original'?1000:500,report=measureSuitability(sub,draw,k,DEFAULT_PRINT_PROFILE)
   if(!report.suitable)console.log(label,view,pose.id,k,JSON.stringify(report.violations))
  }
 }
}
