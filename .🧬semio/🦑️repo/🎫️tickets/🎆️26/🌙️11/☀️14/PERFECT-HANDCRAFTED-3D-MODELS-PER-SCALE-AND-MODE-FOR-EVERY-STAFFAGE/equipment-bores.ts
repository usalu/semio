/** 🕳️ Find positive-radius meridians whose implicit axis closure fills a plausible hollow section. */
import {readFileSync,readdirSync} from 'node:fs';
import {join} from 'node:path';
import {revolveClosure} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🌀️revolved/🟦️.ts';
const catalog='🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog';
for(const c of readdirSync(catalog).filter(n=>/(machinery|vehicles|components)$/.test(n))){
 for(const n of readdirSync(join(catalog,c))){
  const {asset:a}=JSON.parse(readFileSync(join(catalog,c,n,'🔣️.json'),'utf8'));
  for(const [scale,v] of Object.entries(a.volumes) as [string,any][]){
   for(const mode of ['body','thermal','structure']){
    for(const s of v[mode]){
     if(s.kind!=='revolve')continue;
     const closing=revolveClosure(s.profile);
     if(closing.kind==='shell')console.log(JSON.stringify({asset:n,scale,mode,id:s.id,fillRatio:closing.loopAreaM2/closing.filledAreaM2,profile:s.profile}));
    }
   }
  }
 }
}
