/** 🖨️ Checks each integer paper denominator in each authored band of the supplied asset. */
import {readFileSync} from 'node:fs'
import {DEFAULT_PRINT_PROFILE,measureSuitability} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts'
const {asset}=JSON.parse(readFileSync(process.argv[2],'utf8'))
for(const d of [50,100,200,500].filter(d=>asset.representations[d].status==="authored"&&(!process.argv[3]||d===Number(process.argv[3])))){
 const rep=asset.representations[d],a=rep.validRange.minDenominator,b=rep.validRange.maxDenominator
 for(let k=a;k<=b;k++){const r=measureSuitability(asset,rep,k,DEFAULT_PRINT_PROFILE);console.log(JSON.stringify({d,k,suitable:r.suitable,issues:r.violations}));if(!r.suitable)process.exitCode=1}
}
