/** 🌡️ Measures thermal coverage using the product's area rule on the catalog document. */
import { entryOf } from './volumes-lib.ts';
import { measureVolumes } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts';
const {asset}=entryOf('humans/'+process.argv[2]);
for(const scale of [50,100,200,500]){
  const m=measureVolumes(asset,{scale,rules:['volumes-thermal-coverage']});
  for(const v of m.filter(x=>x.name.startsWith('areaDeviation:')&&(x.value>x.limit||x.name.endsWith(':face')))) console.log(scale,v.name,v.value.toFixed(3),v.limit);
}
