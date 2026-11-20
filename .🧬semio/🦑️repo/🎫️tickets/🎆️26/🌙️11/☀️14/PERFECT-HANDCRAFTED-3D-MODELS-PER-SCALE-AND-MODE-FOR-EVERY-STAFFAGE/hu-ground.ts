/** 🥾 Checks the evaluated height of declared floor contacts of each named human pose, excluding elevated supports. */
import {catalogEntries,loadAsset} from './volumes-lib.ts'
import {samplePose} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts'
import {rigAt} from './r3-lib.ts';
for(const e of catalogEntries().filter(e=>e.category==='humans')){
 const a=loadAsset(e.path);
 for(const p of a.poses){
  const rig=rigAt(a,100,{kind:'pose',pose:p.id});
  for(const id of p.contacts.filter(k=>/^(heel|ball|contact|wheel-contact|box-base|tip)-?([lr])?$/.test(k))){
   const h=rig.state.anchors[id]?.at?.y;
   if(Number.isFinite(h)&&Math.abs(h)>.008)console.log([e.slug,p.id,id,h.toFixed(4)].join('\t'))
  }
 }
}
