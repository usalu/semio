/** 📦 Measures the lift worker's ground contacts and the attitude of both boots throughout the named poses. */
import {entryOf} from './volumes-lib.ts';
import {rigAt} from './r3-lib.ts';
const {asset}=entryOf('humans/worker-carrying-box');
for(const p of asset.poses){
 const state=rigAt(asset,50,{kind:'pose',pose:p.id}).state;
 console.log(p.id,Object.entries(state.anchors).filter(([k])=>['heel-l','contact-l','heel-r','contact-r','box-ground'].includes(k)).map(([k,v])=>k+' '+v.at.x.toFixed(3)+' '+v.at.y.toFixed(3)).join(' '))
}
