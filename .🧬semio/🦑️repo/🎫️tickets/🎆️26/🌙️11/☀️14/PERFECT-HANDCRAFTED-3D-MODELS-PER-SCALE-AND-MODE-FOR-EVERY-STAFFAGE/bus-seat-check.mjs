import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {ShapeUtils,Vector2} from 'three';
const f=JSON.parse(readFileSync(new URL('./bus-seat-case.json',import.meta.url)));
const catalog='🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/';
const asset=path=>JSON.parse(readFileSync(catalog+path+'/🔣️.json')).asset;
const near=(a,b)=>assert(Math.abs(a-b)<1e-6,`${a} ≠ ${b}`);
const bus=asset('🚜️vehicles/city-bus'),v=bus.volumes['50'];
const outer=v.body.filter(s=>/^seat-[lr]-\d+$/.test(s.id));
function section(s){
  assert.equal(s.kind,'extrude');const loop=s.profile.map(([x,y])=>new Vector2(x,y));
  const triangles=ShapeUtils.triangulateShape(loop,[]);
  near(triangles.reduce((sum,t)=>sum+Math.abs(ShapeUtils.area(t.map(i=>loop[i]))),0),Math.abs(ShapeUtils.area(loop)));
  return loop;
}
let rows=0;
for(const pan of outer){
  const inner=v.body.find(s=>s.id===pan.id+'-aisle'),back=v.body.find(s=>s.id===pan.id.replace('seat-','seat-back-'));
  for(const p of [pan,inner]){
    const x=section(p).map(v=>v.x);near(Math.max(...x)-Math.min(...x),f.depthM);near(p.to-p.from,f.panWidthM);
    near(p.repeat.translation[0],f.rowPitchM);
  }
  const profile=section(back),lumbar=profile.find(p=>p.y===-.16),shoulder=profile.find(p=>p.y===.14);
  assert(lumbar.x-shoulder.x>=f.minimumLumbarRiseM);
  near(Math.max(pan.to,inner.to)-Math.min(pan.from,inner.from),f.pairWidthM);
  near(Math.max(pan.from,inner.from)-Math.min(pan.to,inner.to),f.centreGapM);
  const carrier=v.body.find(s=>s.id===pan.id.replace('seat-','seat-carrier-'));
  const foot=v.body.find(s=>s.id===pan.id.replace('seat-','seat-foot-'));
  assert(carrier.transform.translation[1]+carrier.size[1]/2>pan.transform.translation[1]+Math.min(...pan.profile.map(p=>p[1])));
  assert(foot.transform.translation[1]+foot.size[1]/2>carrier.transform.translation[1]-carrier.size[1]/2);
  rows+=pan.repeat.count;
}
near(rows,f.rows);near(2*rows,f.passengers);
const radiator=asset('🧱️components/panel-radiator'),head=radiator.volumes['500'].body.find(s=>s.id==='valve-head');
const ring=section(head);near(ring.length,f.headFacets);near(head.to-head.from,f.headLengthM);
for(const axis of ['x','y'])near(Math.max(...ring.map(p=>p[axis]))-Math.min(...ring.map(p=>p[axis])),f.headDiameterM);
console.log(JSON.stringify({passengers:2*rows,rows,valveFacets:ring.length}));
