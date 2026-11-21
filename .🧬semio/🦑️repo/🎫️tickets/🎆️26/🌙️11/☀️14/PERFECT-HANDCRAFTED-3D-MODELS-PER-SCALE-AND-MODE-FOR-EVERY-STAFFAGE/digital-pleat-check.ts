import { readFileSync } from 'node:fs';
import { strict as assert } from 'node:assert';
import { Matrix4, Vector3, Box3 } from 'three';
import { evaluate } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import { restFrames } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts';
import { worldFrameOf } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts';
const a=JSON.parse(readFileSync(process.argv[2]??'🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/market-parasol/🔣️.json','utf8')).asset;
const f=JSON.parse(readFileSync(new URL('./digital-pleat-case.json',import.meta.url),'utf8'));
const rest=restFrames(a.rig);
const matrix=frame=>{const {o,u,v,w}=worldFrameOf(frame);return new Matrix4().makeBasis(new Vector3(...u),new Vector3(...v),new Vector3(...w)).setPosition(...o)};
const area=p=>p.slice(2).reduce((s,q,i)=>s+q.clone().sub(p[0]).cross(p[i+1].clone().sub(p[0])).length()*.5,0);
const failures=[];
for(const d of f.scales){
 for(const state of [...f.poses.map(pose=>({kind:'pose',pose})),...Array.from({length:25},(_,i)=>({kind:'transition',from:'open-still',to:'furled',durationS:4,timeS:4*i/24})),...a.clips.flatMap(c=>Array.from({length:17},(_,i)=>({kind:'clip',clip:c.id,t:c.durationS*i/16,timeS:c.durationS*i/16}))) ]){
  const {bones}=evaluate(a,state);
  for(const mode of ['body','thermal']){
   const parts=a.volumes[d][mode].filter(s=>s.id.startsWith(mode==='body'?'panel-':'th-panel-'));
   const profiles=parts.map(s=>{
    const local=s.profile.map(([z,x])=>new Vector3(x,(s.from+s.to)/2,z));
    const weights=s.weights??[{bone:s.bone,weight:1}];
    const points=local.map(v=>weights.reduce((p,w)=>p.addScaledVector(v.clone().applyMatrix4(matrix(rest[s.bone].frame)).applyMatrix4(matrix(rest[w.bone].frame).invert()).applyMatrix4(matrix(bones[w.bone].frame)),w.weight),new Vector3()));
    return {local,points,id:s.id};
   });
   const gap=Math.max(...profiles.map((s,i)=>Math.min(...profiles.filter((_,j)=>j!==i).map(q=>s.points.at(-1).distanceTo(q.points[1])))));
   const midArea=profiles.reduce((v,g)=>v+area(g.points),0),cutArea=profiles.reduce((v,g)=>v+area(g.local),0);
   const w=new Box3().setFromPoints(profiles.flatMap(g=>g.points)).getSize(new Vector3()).x;
   if(gap>f.hemGapM||Math.abs(midArea/cutArea-1)>f.relativeAreaError||(state.pose==='furled'?w>f.furledWidthM:state.kind!=='transition'&&w<f.openWidthM))failures.push({d,...state,mode,gap,midArea,cutArea,w});
  }
 }
}
if(failures.length)console.log(failures.slice(0,4));
assert.equal(failures.length,0,'fabric area, connected hem and width through poses and wind');
console.log('All four scales retain continuous seams and true fabric area through three poses, both authored clips and 25 positions of the folding mechanism.');
