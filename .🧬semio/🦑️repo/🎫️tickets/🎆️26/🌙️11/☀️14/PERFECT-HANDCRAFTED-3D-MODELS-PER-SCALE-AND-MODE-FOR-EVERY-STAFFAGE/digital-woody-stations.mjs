import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
import {Vector2,Vector3,Box2} from 'three';
const cases=JSON.parse(readFileSync(new URL('./digital-woody-stations.json',import.meta.url)));
const near=(actual,expected,tol=1e-6)=>assert.ok(Math.abs(actual-expected)<tol,`${actual} versus ${expected}`);
for(const path of process.argv.slice(2)){
 const {asset}=JSON.parse(readFileSync(path)),c=cases[asset.id];
 if(asset.id==='weeping-willow'){
  for(const [scale,v] of Object.entries(asset.volumes))for(const [key,length] of Object.entries(c.lengthsM)){
   const p=v.body.find(q=>q.id==='switch-'+key),rings=p.sections;
   near(rings[0].rx,c.basalRadiiM[key]);near(rings.at(-1).rx,c.basalRadiiM[key]*c.tipFraction);
   assert.ok(rings.slice(1).every((s,i)=>s.rx<rings[i].rx&&s.at[0]>rings[i].at[0]));
   const points=rings.map(s=>new Vector3(...s.at)),axis=points.at(-1).clone().sub(points[0]);
   near(axis.x,length);assert.ok(points.slice(1).reduce((sum,q,i)=>sum+q.distanceTo(points[i]),0)>axis.length());
   assert.ok(v.thermal.some(s=>s.of===p.id&&s.region==='limb-bark'));assert.ok(v.structure.some(s=>s.of===p.id&&s.member===p.bone));
  }
 }else if(asset.id==='scots-pine'){
  const v=asset.volumes[c.scale],leader=v.body.find(q=>q.id===c.leader);assert.equal(leader.bone,c.bone);
  assert.ok(v.thermal.some(q=>q.of===c.leader&&q.region===c.surface));assert.ok(v.structure.some(q=>q.of===c.leader&&q.member===c.bone));
 }else{
  const blades=asset.volumes[c.scale].body.filter(q=>q.material==='palm-frond');assert.equal(blades.length,c.blades);
  for(const view of Object.values(asset.representations[c.scale].geometry))for(const blade of blades){
   const panels=view.filter(q=>q.id.startsWith(blade.id+'-')&&q.bone===blade.bone);assert.ok(panels.length>0);
   for(const p of panels){
    assert.equal(p.solid.sweep,'v');const bounds=new Box2().setFromPoints(p.points.map(pt=>new Vector2(...pt)));assert.ok(bounds.getSize(new Vector2()).y>c.polygonWidthMinM);
   }
  }
 }
 console.log(asset.id,'anatomical dimensions and bindings passed');
}
