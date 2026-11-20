/** 🚶 Adjusts the opposing thigh swings to keep one full wheel revolution in a planted walk after positioning the heel centres. */
import { writeFileSync } from 'node:fs'
import {join} from 'node:path'
import {entryOf,GENERATED} from './volumes-lib.ts'
import {sampleClip} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts'
const {asset}=entryOf('humans/person-with-rollator')
function swing(amplitude:number){
 const next=structuredClone(asset),clip=next.clips.find(c=>c.id==='rollator-walk');
 for(const key of clip.keys){
  for(const [side,sign] of [['l',1],['r',-1]] as const){
   const delta=amplitude*sign*Math.cos(2*Math.PI*key.t/clip.durationS);
   for(const [segment,gain] of [['thigh',1],['shank',-1]] as const){
    const part=key.bones[segment+'-'+side];
    part.angleDeg=Number((part.angleDeg+gain*delta).toFixed(5));
   }
  }
 }
 const travel=sampleClip(next,clip,clip.durationS-1e-9).root.x-sampleClip(next,clip,0).root.x;
 return {travel,clip,err:travel-clip.advanceM}
}
let angle=0
for(let i=0;i<5;i++){
 const a=swing(angle);
 if(Math.abs(a.err)<1e-6)break
 angle-=a.err*.02/(swing(angle+.02).err-a.err)
}
const s=swing(angle);
if(!Number.isFinite(angle)||Math.abs(angle)>.6||Math.abs(s.err)>1e-6)throw Error('Unexpected gait correction '+angle+' '+s.err);
console.log('Swing correction',angle,'degrees; travel',s.travel,'m');
writeFileSync(join(GENERATED,'hu-review','stride.json'),JSON.stringify(s.clip.keys))
