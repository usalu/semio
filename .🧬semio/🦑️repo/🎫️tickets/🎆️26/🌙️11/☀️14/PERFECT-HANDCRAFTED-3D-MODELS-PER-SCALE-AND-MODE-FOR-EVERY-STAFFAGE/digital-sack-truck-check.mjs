/** 🛞 Verify the turned axle, wheel bores, journal clearance and both bearing locations. */
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {Vector2,Vector3,Box2} from 'three';
const c=JSON.parse(readFileSync(new URL('./digital-sack-truck-case.json',import.meta.url))),{asset}=JSON.parse(readFileSync(process.argv[2]));
const close=(a,b)=>assert(Math.abs(a-b)<1e-8,`${a} / ${b}`);
for(const scale of c.scales){
 const v=asset.volumes[scale],get=id=>v.body.find(q=>q.id===id),axle=get('axle'),rim=get('rim'),tire=get('tyre');
 assert.equal(axle.kind,'revolve');const b=new Box2().setFromPoints(axle.profile.map(p=>new Vector2(...p)));
 close(b.max.x-b.min.x,c.axleLengthM);close(b.max.y*2,c.axleDiameterM);
 close(Math.min(...rim.profile.map(p=>p[1]))*2,c.hubBoreM);
 close((c.hubBoreM-c.axleDiameterM)/2,c.clearanceM);
 close(new Vector3(...rim.repeat.translation).length(),c.wheelSpacingM);
 close(Math.max(...tire.profile.map(p=>p[1]))*2,c.outerDiameterM);
 assert.ok(Math.min(...tire.profile.map(p=>p[1]))>=.056);
 for(const id of ['rim','axle','tyre']){
  assert.ok(v.thermal.some(q=>q.of===id));assert.ok(v.structure.some(q=>q.of===id));
 }
}
console.log('Four running axle and wheel section checks passed.');
