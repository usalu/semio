/** 🍃 Paint successive stations of each cupped hosta leaf at their actual camera depths. */
import {readFileSync,writeFileSync} from 'node:fs';
import {poseState,toWorld} from './volumes-lib.ts';
import {CAMERAS} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🔭️views/🟦️.ts';
const original=JSON.parse(readFileSync(process.argv[2],'utf8')).asset;
const doc=JSON.parse(readFileSync(process.argv[3],'utf8')),a=doc.asset;
for(const d of [50,200]){
 const frames=poseState(a,d).frames;
 for(const view of d===50?['front']:['front','elevation']){
  const toward=CAMERAS[view].looking,groups=new Map();
  for(const s of a.representations[d].geometry[view]){
   if(!s.id.includes('-rib-'))continue;
   const x=s.points.reduce((v,p)=>v+p[0],0)/s.points.length,z=s.points.reduce((v,p)=>v+p[1],0)/s.points.length;
   const bone=a.rig.bones.find(b=>b.id===s.bone),u=Math.max(0,Math.min(1,x/bone.lengthM)),v=s.solid;
   const y=((v.z0+v.z1)*(1-u)+(v.zTip[0]+v.zTip[1])*u)/2;
   const wp=toWorld(frames[s.bone],[x,y,z]),depth=-(wp[0]*toward.x+wp[1]*toward.y+wp[2]*toward.z);
   if(!groups.has(s.bone))groups.set(s.bone,[]);
   groups.get(s.bone).push([s,depth]);
  }
  for(const [bone,seg] of groups){
   const olds=original.representations[d].geometry[view].filter(q=>q.bone===bone&&q.filled).map(s=>s.z);
   const avg=olds.reduce((a,b)=>a+b)/olds.length,mean=seg.reduce((a,[,b])=>a+b,0)/seg.length;
   for(const [s,depth] of seg)s.z=+(avg+depth-mean).toFixed(7);
  }
  a.representations[d].geometry[view].sort((a,b)=>a.z-b.z);
 }
}
writeFileSync(process.argv[4],JSON.stringify(doc));
