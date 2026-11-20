/** 🏋 Samples the lift rig from feet to carton in the subject's authored phases. */
import {entryOf} from './volumes-lib.ts';
import {rigAt} from './r3-lib.ts';
const {asset}=entryOf('humans/worker-carrying-box');
for(const p of asset.poses.slice(0,5)){
 const rig=rigAt(asset,100,{kind:'pose',pose:p.id});console.log(p.id,p.bones.pelvis,p.bones.torso);
 for(const id of ['pelvis','thigh-l','shank-l','foot-l','hand-l','box-l','box-r']){const f=rig.frames[id];if(f)console.log(id,f.o.map(n=>+n.toFixed(3)),Math.atan2(f.u[1],f.u[0])*180/Math.PI)}
 for(const id of ['box-base','contact-l','heel-l'])console.log(id,rig.state.anchors[id])
}
