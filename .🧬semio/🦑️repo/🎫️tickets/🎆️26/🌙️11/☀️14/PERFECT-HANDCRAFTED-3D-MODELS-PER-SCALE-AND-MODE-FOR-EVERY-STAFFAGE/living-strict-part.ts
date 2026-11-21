/** 🎋 Bound a dense botanical measurement by one requested scale and a disjoint subset of the product rules. */
import {createHash} from 'node:crypto'
import {readFileSync,renameSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {VOLUME_RULES,checkVolumes,type VolumeRule} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
import {catalogEntries} from './volumes-lib.ts'
const names={
 drawing:['volumes-silhouette','volumes-bounds'],
 heat:['volumes-thermal-coverage'],
 skeleton:['volumes-structure-coverage'],
 pair:['volumes-cross-scale'],
 source:VOLUME_RULES.filter(k=>!['volumes-silhouette','volumes-bounds','volumes-thermal-coverage','volumes-structure-coverage','volumes-cross-scale'].includes(k))
} as const
const [id,scaleText,group]=process.argv.slice(2)
if(!id||!group||(!(group in names)&&group!=='bounds')||!scaleText||!['50','100','200','500'].includes(scaleText))throw Error('Supply asset, scale, and strict rule group')
const all=Object.values(names).flat()
if(new Set(all).size!==VOLUME_RULES.length||all.length!==VOLUME_RULES.length)throw Error('Each product rule must be measured once')
const path=catalogEntries().find(item=>item.slug===id)!.path
const raw=readFileSync(path,'utf8'),hash=createHash('sha256').update(raw).digest('hex')
const scale=Number(scaleText) as 50|100|200|500
const rules=group==='bounds'?['volumes-bounds'] as const:names[group as keyof typeof names] as readonly VolumeRule[]
const findings=checkVolumes(JSON.parse(raw).asset,{scale,rules})
const target=join(import.meta.dir,'🗑️generated','living-current',[id,'strict',scale,group].join('-')+'.json')
const partial=target+'.'+process.pid+'.tmp'
writeFileSync(partial,JSON.stringify({id,hash,code:process.env.LIVING_AUDIT_CODE,scale,rules,findings}))
renameSync(partial,target)
console.log('[DEBUG]',id,scale,group,findings.length)
