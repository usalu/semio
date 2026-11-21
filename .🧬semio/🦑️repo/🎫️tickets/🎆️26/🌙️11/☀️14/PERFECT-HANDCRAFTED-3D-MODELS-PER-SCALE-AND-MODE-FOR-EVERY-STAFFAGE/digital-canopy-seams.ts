import {readFileSync} from 'node:fs';
import {Vector3,Matrix4} from 'three';
import {evaluate} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {restFrames} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts';
import {worldFrameOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts';
const {asset:a}=JSON.parse(readFileSync('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/market-parasol/🔣️.json','utf8'));
const mat=f=>{const {u,v,w,o}=worldFrameOf(f);return new Matrix4().makeBasis(new Vector3(...u),new Vector3(...v),new Vector3(...w)).setPosition(...o);};
for(const pose of ['open-still','tilted-to-sun','furled']){
 const rest=restFrames(a.rig),{bones}=evaluate(a,{kind:'pose',pose});
 const panels=a.volumes[50].body.filter(s=>s.id.startsWith('panel-'));
 const vertices=panels.map(s=>[s.profile[0],s.profile[1],s.profile.at(-1)].map(([z,x])=>{
   const pt=new Vector3(x,.00825,z).applyMatrix4(mat(rest[s.bone].frame));
   const here=new Vector3();for(const w of s.weights)here.addScaledVector(pt.clone().applyMatrix4(mat(rest[w.bone].frame).invert()).applyMatrix4(mat(bones[w.bone].frame)),w.weight);return here;
 }));
 const gaps=vertices.map((g,j)=>Math.min(...vertices.filter((_,k)=>k!==j).flatMap(h=>[g[2].distanceTo(h[1]),g[2].distanceTo(h[2])])));
 console.log(pose,'largest hem junction m',Math.max(...gaps),'crown diameter m',Math.max(...vertices.flatMap(v=>vertices.map(h=>v[0].distanceTo(h[0])))));
}
