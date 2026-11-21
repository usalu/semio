import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {ShapeUtils,Vector2} from 'three';
import {evaluate} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {solveStructure} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts';
const {asset}=JSON.parse(readFileSync('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/tied-canopy/🔣️.json','utf8'));
const f=JSON.parse(readFileSync(join(import.meta.dir,'canopy-glazing.json'),'utf8'));
const near=(a:number,b:number,eps=1e-5)=>assert.ok(Math.abs(a-b)<eps,`${a} vs ${b}`);
const panes=asset.volumes['50'].body.filter((s:any)=>s.material==='glass'),g=asset.structure.gravityMS2;
for(const level of Object.values(asset.volumes) as any[]){
 const member=level.structure.find((s:any)=>s.id==='m-strut');assert.deepEqual(member.profile[0],member.profile.at(-1));
 const r=member.profile.map((p:number[])=>p[1]);near(Math.max(...r)-Math.min(...r),.0032);near(Math.max(...r)*2,.0603);
}
const fine=asset.volumes['50'],tube=fine.body.find((s:any)=>s.id==='strut-tube'),skin=fine.thermal.find((s:any)=>s.id==='th-strut-tube');
assert.deepEqual(tube.profile,fine.structure.find((s:any)=>s.id==='m-strut').profile);
assert(skin.profile.some((p:number[])=>p[1]===0));
let area=0,glassMoment=0;
for(const pane of panes){
 const pts=pane.profile.map(([x,y]:number[])=>new Vector2(x,y));
 for(const face of ShapeUtils.triangulateShape(pts,[])){
  const corners=face.map(i=>pts[i]);const a=Math.abs(ShapeUtils.area(corners));
  area+=a;glassMoment+=a*(corners[0].y+corners[1].y+corners[2].y)/3*f.glassThicknessM*f.glassDensityKgM3*g;
 }
}
near(area,f.glassAreaM2);near(area*f.glassThicknessM*f.glassDensityKgM3,f.glassKg);
const moment=glassMoment+f.beamKg*g*f.beamM/2;
const reaction=(q:any)=>solveStructure(evaluate(asset,q)).reactions.find(s=>s.anchor==='tie-rod')!.fyN;
for(const [pose,w] of [['self-weight',0],['snow-load',-f.snowNPerM],['wind-uplift',f.windNPerM]] as const){
 const force=reaction({kind:'pose',pose});
 near(force,(moment-w*f.beamM**2/2)/f.supportM,.04);
}
const gust=(timeS:number)=>reaction({kind:'clip',clip:'wind-gust',timeS});
near(gust(3),(moment-f.gustPeakNPerM*f.beamM**2/2)/f.supportM,.04);
const events=asset.clips.find((c:any)=>c.id==='wind-gust').events;
assert.equal(events.length,2);
for(const e of events)assert(Math.abs(gust(e.t))<1);
console.log(JSON.stringify({area,glassKg:f.glassKg,tensionToCompressionNPerM:moment/(f.beamM**2/2),events:events.map((e:any)=>[e.t,gust(e.t)])}));
