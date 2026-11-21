/** 🖨️ Replay the public office-paper endpoints and nominal drawings of one live asset. */
import {readFileSync} from 'node:fs';
import {validateRanges} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts';
import {DEFAULT_PRINT_PROFILE,measureSuitability} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts';
const {asset}=JSON.parse(readFileSync(process.argv[2]!,'utf8'));
const issues=validateRanges(asset,DEFAULT_PRINT_PROFILE).map(i=>[i.code,i.path,i.message.en]);
for(const [d,rep] of Object.entries(asset.representations) as [string,any][]){
 if(rep.status!=='authored'||!rep.geometry)continue;
 const measured=measureSuitability(asset,rep,Number(d),DEFAULT_PRINT_PROFILE);
 if(!measured.suitable)issues.push(['nominal-print',d,JSON.stringify(measured.violations)]);
}
console.log(JSON.stringify({asset:asset.id,issues}));
