/** 🛢️ Measure the physical polished rod, its paper projection, and the paired bridle paying off the horsehead. */
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {Vector3} from 'three';
import {evaluate} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
const {asset:a}=JSON.parse(readFileSync('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/oil-pumpjack/🔣️.json','utf8')),f=JSON.parse(readFileSync(join(import.meta.dir,'pumpjack-rod.json'),'utf8'));
const near=(x:number,y:number,eps=1e-5)=>assert(Math.abs(x-y)<eps,`${x} vs ${y}`);
const p=(v:any)=>new Vector3(v.x,v.y,v.z??0);
const finish=a.materials.find((m:any)=>m.id==='chrome');assert(finish);near(finish.emissivity,f.rodEmissivity);near(finish.solarAbsorptivity,f.rodSolarAbsorptivity);
const cap=Math.PI*(f.rodDiameterM/2)**2,surface=Math.PI*f.rodDiameterM*f.rodM+cap;
near(a.thermal.regions.find((r:any)=>r.id==='polished-rod').areaM2,surface);
const length=f.pivotHeightM-f.restCarrierM+f.faceRadiusM*(180-f.topOfFaceDeg)*Math.PI/180;
for(const [k,v] of Object.entries(a.volumes) as [string,any][]){
 const rod=v.body.find((s:any)=>s.id==='polished-rod');assert.equal(rod.bone,'rod');assert.equal(rod.material,'chrome');near(rod.profile.at(-1)[0]-rod.profile[0][0],f.rodM);
 assert.equal(v.structure.find((s:any)=>s.id==='member-polished-rod').member,'rod');
 for(const shapes of Object.values(a.representations[k].geometry) as any[]){
  const s=shapes.find((s:any)=>s.id==='polished-rod');assert.equal(s.kind,'member');assert.equal(s.from.bone,'base');assert.equal(s.to.bone,'rod');
  near(s.from.y,f.glandM);near(s.to.x,f.topAboveCarrierM);
 }
}
let frames=0,maxError=0;const extremes:number[]=[];
for(const q of [...a.poses.map((v:any)=>({kind:'pose',pose:v.id})),...a.clips.flatMap((c:any)=>Array.from({length:31},(_,i)=>({kind:'clip',clip:c.id,timeS:i*c.durationS/30}))) ]){
 const state=evaluate(a,q);const carrier=p(state.bones.rod!.frame.o);
 const top=carrier.y+f.topAboveCarrierM,bottom=top-f.rodM;assert(bottom<=f.glandM&&bottom>=-f.belowGradeM);extremes.push(top);
 const exposure=state.thermal.regions['polished-rod']!.exposedFraction;
 near(exposure,(Math.PI*f.rodDiameterM*(top-f.glandM)+cap)/surface,.001);
 const beam=state.bones.beam!.frame.u,theta=Math.atan2(beam.y,beam.x),ang=f.topOfFaceDeg*Math.PI/180+theta;
 const target=new Vector3(f.faceRadiusM*Math.cos(ang),f.pivotHeightM+f.faceRadiusM*Math.sin(ang),0);
 const last=state.bones['bridle-'+f.links]!,err=p(last.tip).distanceTo(target);maxError=Math.max(err,maxError);assert(err<f.endpointToleranceM,JSON.stringify({q,err}));
 let L=0;
 for(let i=1;i<=f.links;i++){
  const b=state.bones['bridle-'+i]!,start=p(b.frame.o),end=p(b.tip);L+=start.distanceTo(end);
  if(i===1)near(start.distanceTo(carrier),0);
  if(i>1)near(start.distanceTo(p(state.bones['bridle-'+(i-1)]!.tip)),0);
 }
 near(L,length);frames++;
}
console.log(JSON.stringify({frames,rodTravelM:Math.max(...extremes)-Math.min(...extremes),wireM:length,worstUpperAttachmentM:maxError}));
