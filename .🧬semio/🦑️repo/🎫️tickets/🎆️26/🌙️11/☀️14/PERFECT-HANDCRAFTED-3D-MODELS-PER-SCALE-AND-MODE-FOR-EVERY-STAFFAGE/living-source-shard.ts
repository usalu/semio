/** 🌱 Measure a scale with its original fine stock available for dimensional comparisons. */
import {createHash} from 'node:crypto'
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {catalogEntries} from './volumes-lib.ts'
import {paperInput} from './living-paper-input.ts'
import {volumeIssues} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volume/🟦️.ts'
import {availabilityRule,bonesRule,featureRule,thermalCoverageRule,structureCoverageRule,slenderRule} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts'
const [name,scale]=process.argv.slice(2);const denom=Number(scale) as 50|100|200|500
const entry=catalogEntries().find(e=>e.slug===name)!
let raw=readFileSync(entry.path,'utf8')
const hash=createHash('sha256').update(raw).digest('hex')
function after(s:string,start:number){
 let depth=0,quote=false
 for(let i=start;i<s.length;i++){const ch=s[i]
  if(quote){if(ch==='\\')i++;else if(ch==='"')quote=false;continue}
  if(ch==='"')quote=true
  else if(ch==='['||ch==='{')depth++
  else if(ch===']'||ch==='}'){if(!--depth)return i+1}
 }
 throw Error('Unclosed JSON value.')
}
const asset=paperInput(raw)
const start=raw.indexOf('{',raw.indexOf('"volumes":'))
for(const key of new Set(['50',scale])){
 if(!asset.volumes[key])continue
 const wanted=raw.indexOf('{',raw.indexOf('"'+key+'"',start))
 asset.volumes[key]=JSON.parse(raw.slice(wanted,after(raw,wanted)))
}
raw='';Bun.gc(true)
const findings=[...volumeIssues(asset).filter(f=>f.path.startsWith('volumes.'+scale+'.')),...[availabilityRule,bonesRule,featureRule,thermalCoverageRule,structureCoverageRule,slenderRule].flatMap(rule=>rule(asset,[denom]).findings)].map(f=>({rule:f.code,path:f.path,message:f.message.en}))
writeFileSync(join(import.meta.dir,'🗑️generated/living-current',name+'-source-'+scale+'.json'),JSON.stringify({id:name,hash,denom,findings}))
console.log('[DEBUG]',name,scale,findings.length)

