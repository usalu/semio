/** 📏 Exercises the bilingual body-area finding directly against the same sample as the regression suite. */
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
import {STAFFAGE} from './volumes-lib.ts'
import {checkVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
const {asset}=JSON.parse(readFileSync(join(STAFFAGE,'🧫️fixtures/👤️sample-figure/🔣️.json'),'utf8'))
asset.thermal.regions.find(q=>q.id==='feet').areaM2=2
const f=checkVolumes(asset,{scale:50,rules:['volumes-thermal-coverage']}).find(q=>q.measurements.some(m=>m.name==='areaDeviation:feet'))
if(f?.measurements[0].limit!==.3||!f.message.en.includes('limit 30 %')||!f.message.de.includes('Grenze 30 %'))throw Error(JSON.stringify(f))
console.log(f.message.en,f.message.de)
