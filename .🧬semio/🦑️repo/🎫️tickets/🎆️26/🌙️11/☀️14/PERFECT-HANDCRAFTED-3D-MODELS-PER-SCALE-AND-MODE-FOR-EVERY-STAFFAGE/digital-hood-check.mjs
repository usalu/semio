import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {Box3, Vector3} from 'three';
const f=JSON.parse(readFileSync(new URL('./digital-hood-case.json',import.meta.url)));
const {asset}=JSON.parse(readFileSync(process.argv[2]??'🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/kitchen-extractor-hood/🔣️.json'));
const near=(v,w)=>assert(Math.abs(v-w)<f.toleranceM,`${v} m, expected ${w} m`);
for(const d of f.scales){
 const v=asset.volumes[d],body=new Map(v.body.map(s=>[s.id,s]));
 for(const id of ['top-plate','back-plate','slope-left','slope-right','slope-front','base-panel']){
   const s=body.get(id);assert(s,id);near(s.to-s.from,f.sheetM);
 }
 const top=body.get('top-plate');
 const hole=new Box3().setFromPoints(top.holes[0].map(([z,x])=>new Vector3(x,0,z)));
 near(hole.getSize(new Vector3()).x,f.ductDiameterM);near(hole.getSize(new Vector3()).z,f.ductDiameterM);
 for(const part of ['blower-scroll','blower-back','blower-inlet','blower-motor','wheel-backplate','wheel-inlet-ring'])assert(body.has(part),part);
 const plate=body.get('wheel-backplate'),ring=body.get('wheel-inlet-ring');
 assert.equal(plate.bone,'blower-wheel');assert.equal(ring.bone,'blower-wheel');
 near(Math.max(...ring.profile.map(p=>p[1])),f.wheelRadiusM);
 const outer=ring.profile.concat(plate.profile).filter(p=>p[1]>=f.wheelRadiusM-f.toleranceM).map(p=>p[0]);
 near(Math.max(...outer)-Math.min(...outer),f.wheelWidthM);
 near(Math.max(...plate.profile.map(p=>p[0]))-Math.max(...outer),.01);
 {const blade=body.get('wheel-vane');assert.equal(blade.repeat.count,f.bladeCount);near(blade.repeat.rotationRad*f.bladeCount,2*Math.PI);}
}
assert(asset.rig.bones.find(s=>s.id==='blower-wheel').joint.continuousTwist);
for(const [id,sign] of [['damper-left',-1],['damper-right',1]]){
 const shut=asset.poses.find(p=>p.id==='off').bones[id].twistDeg;
 const open=asset.poses.find(p=>p.id==='extracting').bones[id].twistDeg;
 near(shut,0);near(open,80*sign);
 const tip=new Vector3(0,0,-.072*sign).applyAxisAngle(new Vector3(1,0,0),open*Math.PI/180);
 assert(tip.y>.070);
}
const out=asset.thermal.emitters.find(e=>e.id==='extract');
const air=out.volumeFlowM3S*1.204*1006*out.temperatureRiseK;
const room=asset.thermal.regions.reduce((n,r)=>n+(r.operating?.heatW??0),0);
near(room+air,220);near(out.volumeFlowM3S*3600,626);
assert.deepEqual(asset.thermal.operating.find(s=>s.id==='fan').levels.map(l=>l.v),[0,.4,.7,1]);
console.log('Six physical sheets, 150 mm open throat, shaft, motor, casing and 134 × 68 mm wheel checked at all scales.');
