import {readFileSync,writeFileSync} from 'node:fs'
import {createHash} from 'node:crypto'
import {join} from 'node:path'
import {copiedRule} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts'
import {catalogEntries} from './volumes-lib.ts'
const name=process.argv[2]!,entry=catalogEntries().find(e=>e.slug===name)!
let raw=readFileSync(entry.path,'utf8')
const hash=createHash('sha256').update(raw).digest('hex'),asset=JSON.parse(raw).asset
raw='';Bun.gc(true)
const r=copiedRule(asset)
writeFileSync(join(import.meta.dir,'🗑️generated/living-current',name+'-copied.json'),JSON.stringify({id:name,hash,findings:r.findings,comparisons:r.measurements.length}))
console.log('[DEBUG]',name,r.findings.length)

