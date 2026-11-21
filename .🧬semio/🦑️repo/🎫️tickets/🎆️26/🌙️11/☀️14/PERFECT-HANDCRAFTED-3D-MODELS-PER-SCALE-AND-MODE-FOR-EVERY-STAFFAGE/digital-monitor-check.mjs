import {readFileSync} from 'node:fs';
import {strict as assert} from 'node:assert';
import {Box3,Vector3} from 'three';
const p=process.argv[2]??'🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/height-adjustable-desk/🔣️.json';
const a=JSON.parse(readFileSync(p,'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-monitor-case.json',import.meta.url),'utf8'));
const near=(a,b)=>assert(Math.abs(a-b)<1e-7,`${a} != ${b}`);
for(const d of f.scales){
 const v=a.volumes[d],by=id=>v.body.find(s=>s.id===id),fr=by('monitor-panel'),glass=by('monitor-screen');
 assert.equal(fr.kind,'extrude','open bezel');
 const ext=new Box3().setFromPoints(fr.profile.flatMap(([x,y])=>[fr.from,fr.to].map(z=>new Vector3(x,y,z)))).getSize(new Vector3());
 [ext.x,ext.y,ext.z].forEach((n,i)=>near(n,[...f.outsideM,f.housingDepthM][i]));
 glass.size.forEach((n,i)=>near(n,f.displayM[i]));
 near(by('monitor-base').size[0],f.standFootprintM[0]);near(by('monitor-base').size[2],f.standFootprintM[1]);
 const bolts=v.body.filter(s=>s.id.startsWith('vesa-screw-'));assert.equal(bolts.length,4);
 const spread=new Box3().setFromPoints(bolts.map(b=>new Vector3(...b.origin))).getSize(new Vector3());near(spread.x,f.rearMountSpacingM);near(spread.y,f.rearMountSpacingM);
 assert.equal(v.thermal.find(x=>x.of==='monitor-screen').region,'display-front');
}
const fields=a.thermal.regions.filter(r=>r.operating?.key==='monitor');near(fields.reduce((s,r)=>s+r.operating.heatW,0),f.electricalW);
console.log('Bezel, display glazing, four VESA fixings, footprint and thermal power verified across four scales.');
