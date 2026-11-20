/** 📊 Runs measured volume rules on each document from the live human catalog, preserving per-asset findings. */
import { entryOf, catalogEntries } from './volumes-lib.ts';
import { checkVolumes } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts';
for(const e of catalogEntries().filter(e=>e.category==='humans')){
 const {asset}=entryOf('humans/'+e.slug);
 const all=checkVolumes(asset);
 console.log(JSON.stringify({asset:e.slug,counts:Object.fromEntries(Object.entries(asset.volumes).map(([k,v])=>[k,[v.body.length,v.thermal.length,v.structure.length]])),issues:all.map(i=>({rule:i.rule,scale:i.scale,msg:i.message.en}))}));
 if(global.gc)global.gc();
}
