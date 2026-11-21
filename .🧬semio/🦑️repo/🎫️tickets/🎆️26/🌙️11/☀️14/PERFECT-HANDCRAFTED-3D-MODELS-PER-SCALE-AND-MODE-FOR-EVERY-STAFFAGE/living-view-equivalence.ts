/** 🌳 Compare each independently checkpointed viewpoint against the complete canonical measurements. */
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
import {catalogEntries} from './volumes-lib.ts'
import {measureVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
const [id,k]=process.argv.slice(2),entry=catalogEntries().find(e=>e.slug===id)!,asset=JSON.parse(readFileSync(entry.path,'utf8')).asset
const full=measureVolumes(asset,{scale:Number(k),rules:['volumes-silhouette']}).filter(m=>m.rule==='volumes-silhouette')
const separate=['elevation','front','plan'].flatMap(view=>JSON.parse(readFileSync(join(import.meta.dir,'🗑️generated/living-current',[id,'strict',k,view].join('-')+'.json'),'utf8')).measurements)
if(full.length!==separate.length)throw Error('Measurement count differs')
for(const m of full){const v=separate.find(n=>n.name===m.name);if(JSON.stringify(v)!==JSON.stringify(m))throw Error(JSON.stringify({v,m}))}
console.log('[DEBUG]',id,k,full.length,'identical canonical measurements')
