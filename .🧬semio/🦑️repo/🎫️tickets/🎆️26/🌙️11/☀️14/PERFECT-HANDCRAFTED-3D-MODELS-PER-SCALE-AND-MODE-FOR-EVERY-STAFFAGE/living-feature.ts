import {featureRule} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts'
import {catalogEntries} from './volumes-lib.ts'
import {readFileSync} from 'node:fs'
const names=process.argv.slice(2)
for(const x of catalogEntries().filter(e=>names.includes(e.slug))){
 const a=JSON.parse(readFileSync(x.path,'utf8')).asset;const result=featureRule(a)
 console.log(JSON.stringify({asset:a.id,findings:result.findings.map(f=>({list:f.list,scale:f.scale,source:f.path,issue:f.message.en}))}))
 Bun.gc(true)
}
