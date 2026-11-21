import {readFileSync,writeFileSync} from 'node:fs'
import {createHash} from 'node:crypto'
import {join} from 'node:path'
import {catalogEntries} from './volumes-lib.ts'
import {volumeIssues} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volume/🟦️.ts'
import {availabilityRule,bonesRule,featureRule,thermalCoverageRule,structureCoverageRule,slenderRule} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts'
for(const entry of catalogEntries().filter(e=>(e.path.includes('vegetation')||e.path.includes('animals'))&&(!process.argv[2]||process.argv[2]===e.slug))){
 const raw=readFileSync(entry.path,'utf8'),hash=createHash('sha256').update(raw).digest('hex')
 const asset=JSON.parse(raw).asset;const findings=[...volumeIssues(asset),...[availabilityRule,bonesRule,featureRule,thermalCoverageRule,structureCoverageRule,slenderRule].flatMap(rule=>rule(asset).findings)].map(f=>({rule:f.code,path:f.path,message:f.message.en}))
 writeFileSync(join(import.meta.dir,'🗑️generated','living-current',asset.id+'-source.json'),JSON.stringify({id:asset.id,hash,findings}))
 console.log('[DEBUG]',asset.id,findings.length)
 Bun.gc(true)
}
