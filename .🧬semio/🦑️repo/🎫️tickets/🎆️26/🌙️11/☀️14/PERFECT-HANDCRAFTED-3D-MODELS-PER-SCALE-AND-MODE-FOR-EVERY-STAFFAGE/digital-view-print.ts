import {readFileSync} from 'node:fs';
import {measureSuitability,DEFAULT_PRINT_PROFILE} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts';
for(const name of ['elevation','front','plan']){
 const {asset}=JSON.parse(readFileSync(process.argv[2],'utf8'));
 for(const input of process.argv.slice(3)){
  const [d,k]=input.split('/').map(Number);
  const rep=asset.representations[d];
  const isolated={...rep,geometry:Object.fromEntries(Object.entries(rep.geometry).map(([v,shapes])=>[v,v===name?shapes:[]]))};
  const result=measureSuitability(asset,isolated,k,DEFAULT_PRINT_PROFILE);
  if(!result.suitable)console.log(name,input,result.violations.map(v=>v.shapes.join('+')).join(','));
 }
}
