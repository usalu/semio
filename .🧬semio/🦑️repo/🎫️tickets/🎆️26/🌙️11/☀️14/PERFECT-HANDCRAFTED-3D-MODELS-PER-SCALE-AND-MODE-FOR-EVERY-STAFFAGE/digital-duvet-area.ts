/** 🪡 Measure exterior heat-transfer areas and anatomical bounds on the posed quilt. */
import {readFileSync} from 'node:fs'
import {validateAsset} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts'
import {measureVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
if(process.argv[3]==='semantic'){console.log(validateAsset(a).map(i=>i.message.en));process.exit(0)}
for(const scale of (process.argv.slice(3).length?process.argv.slice(3).map(Number):[50])){
 const measures=measureVolumes(a,{scale,rules:['volumes-thermal-coverage','volumes-bounds']})
 console.log(JSON.stringify(measures.slice(0,3)));console.log(JSON.stringify(measures.filter(m=>m.name.includes('outerAreaM2')||m.name.includes('heightM')||m.name.includes('depthM')).map(m=>[m.name,m.value])))
}
