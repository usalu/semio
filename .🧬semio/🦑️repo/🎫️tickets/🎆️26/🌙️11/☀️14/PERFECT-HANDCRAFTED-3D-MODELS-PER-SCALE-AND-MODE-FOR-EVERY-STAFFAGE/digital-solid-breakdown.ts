import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts';
import {catalogEntries,loadAsset} from './volumes-lib.ts';
import {modelOfAsset} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts';
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts';
for(const id of process.argv.slice(2)){
 const entry=catalogEntries().find(e=>e.slug===id);const a=loadAsset(entry?.path??id),pose=a.presentation.defaultPose;
 const model=await modelOfAsset(a,{at:{kind:'pose',pose},poses:[pose],clips:[]}); const data=model.model??model;
 if(a.id==='electric-kettle') console.log(149,data.solids[149]);
 for(const scene of data.scenes.filter(s=>s.id.includes('body'))){
   const list=data.solids.filter(s=>scene.nodes.includes(s.node));const out=list.map(s=>[s.id,tessellateSolid(s,qualityOf(a,Number(scene.id.match(/^s(\d+)/)[1]))).indices.length/3*(s.repeat?.count??1)]);
   console.log(a.id,scene.id,out.sort((a,b)=>b[1]-a[1]).slice(0,6));
 }
}
