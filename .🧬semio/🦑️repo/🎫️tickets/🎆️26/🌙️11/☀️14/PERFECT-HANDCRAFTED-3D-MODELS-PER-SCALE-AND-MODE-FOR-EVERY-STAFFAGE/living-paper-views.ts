/** 🌿 Compare axis extents of the three independently drawn elevations, fronts and plans, without meshing. */
import {createHash} from 'node:crypto'
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {checkVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
import {catalogEntries} from './volumes-lib.ts'
import {paperInput} from './living-paper-input.ts'
const e=catalogEntries().find(e=>e.slug===process.argv[2])!
const raw=readFileSync(e.path,'utf8'),hash=createHash('sha256').update(raw).digest('hex')
const paperOnly=process.argv.includes('--paper-only')
const findings=checkVolumes(paperOnly?paperInput(raw):JSON.parse(raw).asset,{rules:['volumes-views-disagree']})
writeFileSync(join(import.meta.dir,'🗑️generated','living-current',e.slug+'-paper-extents.json'),JSON.stringify({hash,paperOnly,findings}))
console.log('[DEBUG]',e.slug,findings.length)
