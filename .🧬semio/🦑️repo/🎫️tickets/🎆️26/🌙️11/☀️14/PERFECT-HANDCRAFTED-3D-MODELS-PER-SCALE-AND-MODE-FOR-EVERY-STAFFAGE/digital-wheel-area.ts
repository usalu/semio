/** 🌡️ Measure air-accessible surface in the parked pose on the actual tyre, castings and brakes. */
import {readFileSync} from 'node:fs'
import {measureVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
const rows=measureVolumes(a,{scale:+(process.argv[3]??'50'),rules:['volumes-thermal-coverage']})
console.log(JSON.stringify({id:a.id,areas:Object.fromEntries(rows.filter(x=>/^info:.*(tyres|brakes|wheel-metal)/.test(x.name)).map(x=>[x.name,x.value])),issues:rows.filter(x=>!x.ok).map(x=>[x.name,x.value,x.limit])}))
