/** 👶 Identify the occluding shape and pose of a projected stroller passenger. */
import {readFileSync} from 'node:fs'
import {DEFAULT_PRINT_PROFILE,measureSuitability} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,r=a.representations['500']
for(const view of Object.keys(r.geometry)){
 const geometry=Object.fromEntries(Object.entries(r.geometry).map(([k,v])=>[k,k===view?v:[]]))
 for(const p of a.poses){
  const subset={...a,poses:[p]},rep={...r,geometry}
  const res=measureSuitability(subset,rep,350,DEFAULT_PRINT_PROFILE)
  if(!res.suitable){
   const index=geometry[view].findIndex(q=>q.id==='child-head'),clears=[]
   for(const shape of geometry[view].slice(index+1)){
    const test={...rep,geometry:{...geometry,[view]:geometry[view].filter(q=>q.id!==shape.id)}}
    if(measureSuitability(subset,test,350,DEFAULT_PRINT_PROFILE).suitable)clears.push(shape.id)
   }
   console.log(view,p.id,JSON.stringify(res.violations),clears.join(','))
  }
 }
}
