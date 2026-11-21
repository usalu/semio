import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {Vector3, Box3} from 'three';
import {catalogEntries,loadAsset,meshes} from './volumes-lib.ts';
const fixture=JSON.parse(readFileSync(new URL('./digital-shelf-case.json',import.meta.url),'utf8'));
const a=loadAsset(catalogEntries().find(e=>e.slug==='bookshelf')!.path);
const match=(v:number,w:number)=>assert(Math.abs(v-w)<fixture.toleranceM,`measured ${v} m, expected ${w} m`);
for(const scale of fixture.scales){
  const v=a.volumes[scale];
  const selected=v.body.filter(s=>s.id==='back'||s.id==='plinth'||s.id.startsWith('shelf-'));
  const solids=meshes(a,selected,1.8);
  for(const m of solids){
    const bb=new Box3();for(let j=0;j<m.world.length;j+=3)bb.expandByPoint(new Vector3().fromArray(m.world,j));
    const size=bb.getSize(new Vector3());
    if(m.id==='back')match(size.z,fixture.backM);
    else if(m.id==='plinth')match(size.z,fixture.boardsM);
    else {match(size.y,fixture.boardsM);match(size.z,fixture.shelfDepthM);}
  }
  const shelves=v.body.filter(s=>s.id.startsWith('shelf-'));
  assert.equal(shelves.length,10);
  const pins=v.body.filter(s=>s.id.startsWith('pin-'));
  assert.equal(pins.length,10);
  for(const pin of pins){
    assert.equal(pin.repeat.count,2);
    match(pin.repeat.translation[2],fixture.pinPitchM);
    if(pin.kind==='revolve'){
      match(pin.origin[1],-fixture.pinM/2);
      const shank=pin.profile.filter(([x,r])=>x===-fixture.embedM&&r>0);
      match(shank[0][1]*2,fixture.pinM);
      match(Math.max(...pin.profile.map(([x])=>x)),fixture.bearingM);
    }else{
      const ys=pin.profile.map(([y])=>y),zs=pin.profile.map(([,z])=>z);
      match(Math.max(...ys),0); match(Math.min(...ys),-fixture.pinM);
      match(Math.max(...zs)-Math.min(...zs),fixture.pinM);
      match(pin.to-pin.from,fixture.embedM+fixture.bearingM);
    }
  }
  const loaded=a.poses.find(p=>p.id==='loaded');
  for(const s of shelves){
    assert.equal(s.bone,s.id);
    const turn=loaded.bones[s.bone].angleDeg*Math.PI/180;
    const half=a.rig.bones.find(b=>b.id===s.bone).lengthM;
    match(Math.abs(new Vector3(half,0,0).applyAxisAngle(new Vector3(0,0,1),turn).y),fixture.loadedSagM);
  }
}
console.log('Back, kick board, ten shelf halves and loaded midspans agree at all three architectural reductions.');
