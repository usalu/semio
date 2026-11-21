/** 🏃 Capture the actual failing jogger print contours for the independent polygon-union regression. */
import {readFileSync,writeFileSync} from 'node:fs';
import {DEFAULT_PRINT_PROFILE,measureSuitability} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts';
const {asset}=JSON.parse(readFileSync(process.argv[2],'utf8'));
for(const [d,rep] of Object.entries(asset.representations)){
 if(rep.status!=='authored')continue;
 for(const k of [Number(d),rep.validRange.minDenominator,rep.validRange.maxDenominator]){
  try{const r=measureSuitability(asset,rep,k,DEFAULT_PRINT_PROFILE);console.log(d,k,r.suitable);}
  catch(e){if(!Array.isArray(e.input))throw e;writeFileSync(process.argv[3],JSON.stringify({scale:d,denominator:k,rings:e.input}));console.log(d,k,e.message);process.exit(1);}
 }
}
