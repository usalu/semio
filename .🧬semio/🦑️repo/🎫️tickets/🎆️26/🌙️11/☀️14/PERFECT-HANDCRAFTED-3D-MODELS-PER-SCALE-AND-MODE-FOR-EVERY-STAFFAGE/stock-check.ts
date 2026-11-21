/** 📐 Runs each product volume rule group on a single authored scale without retaining other scales’ meshes. */
import {readFileSync} from 'node:fs'
import {checkVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
const {asset}=JSON.parse(readFileSync(process.argv[2],'utf8')),scale=Number(process.argv[3]),rules=process.argv.slice(4).map(rule=>'volumes-'+rule)
console.log(JSON.stringify(checkVolumes(asset,{scale,...(rules.length?{rules}:{})}).map(q=>[q.rule,q.message?.en??q.message])))
