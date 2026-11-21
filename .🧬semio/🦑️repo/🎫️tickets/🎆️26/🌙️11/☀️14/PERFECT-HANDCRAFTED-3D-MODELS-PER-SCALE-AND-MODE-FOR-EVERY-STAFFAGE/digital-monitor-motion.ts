import {readFileSync} from 'node:fs';
import {strict as assert} from 'node:assert';
import {Vector3, Matrix4} from 'three';
import {evaluate} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {worldFrameOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts';
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset;
const matrix=frame=>{const {u,v,w,o}=worldFrameOf(frame);return new Matrix4().makeBasis(new Vector3(...u),new Vector3(...v),new Vector3(...w)).setPosition(...o)};
let min=Infinity,max=-Infinity;
for(let i=0;i<=96;i++){
 const {bones}=evaluate(a,{kind:'clip',clip:'adjust-screen-tilt',timeS:i/24});
 const top=matrix(bones.top.frame),panel=matrix(bones['screen-tilt'].frame);
 const centre=new Vector3(.6,.305,.177).applyMatrix4(top);
 for(const x of [-.042,.042]){
   const pin=new Vector3(.6+x,.305,.177).applyMatrix4(top),eye=new Vector3(x,0,0).applyMatrix4(panel);
   assert(pin.distanceTo(eye)<1e-8);
 }
 const moved=new Vector3(0,.16,-.0617).applyMatrix4(panel);
 assert(Math.abs(centre.distanceTo(moved)-Math.hypot(.16,.0617))<1e-8);
 const upper=new Vector3(0,1,0).transformDirection(panel).transformDirection(top.clone().invert());
 const deg=Math.atan2(-upper.z,upper.y)*180/Math.PI;min=Math.min(min,deg);max=Math.max(max,deg);
}
assert(Math.abs(max-20)<.05&&Math.abs(min+5)<.05,`${min}..${max}`);console.log('The axle and both hinge eyes coincide for 97 frames from -5 to 20 degrees.');
