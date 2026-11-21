/** 🍁 Check the exact drawings and the named poses at both ends and the preset of each supported print band. */
import {readFileSync,writeFileSync} from 'node:fs'
import {createHash} from 'node:crypto'
import {join} from 'node:path'
import {catalogEntries} from './volumes-lib.ts'
import {paperInput} from './living-paper-input.ts'
import {DEFAULT_PRINT_PROFILE,measureSuitability} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts'
const [id,band,onePose,oneScale]=process.argv.slice(2),raw=readFileSync(catalogEntries().find(x=>x.slug===id)!.path,'utf8')
const a=paperInput(raw),rep=a.representations[band!]
if(onePose){
 if(!a.poses.some(p=>!p.key&&p.id===onePose))throw Error('Unknown named pose')
 a.poses=a.poses.map(p=>p.id===onePose?p:{...p,key:true})
}
const checks=[]
for(const d of oneScale?[Number(oneScale)]:new Set([Number(band),...Object.values(rep.validRange)])){
 const result=measureSuitability(a,rep,d as number,DEFAULT_PRINT_PROFILE)
 checks.push({denominator:d,findings:result.violations})
 console.log('[DEBUG]',id,band,d,result.violations.length)
}
writeFileSync(join(import.meta.dir,'🗑️generated/living-current',[id,'paper',band,onePose,oneScale].filter(Boolean).join('-')+'.json'),JSON.stringify({hash:createHash('sha256').update(raw).digest('hex'),onePose,checks}))
