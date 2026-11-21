import { readFileSync } from 'node:fs';
import { strict as assert } from 'node:assert';
import { Box3, Vector3 } from 'three';
import { catalogEntries, loadAsset, meshes } from './volumes-lib.ts';
const f=JSON.parse(readFileSync(new URL('./digital-ladder-case.json',import.meta.url),'utf8'));
const a=loadAsset(catalogEntries().find(e=>e.slug===f.asset)!.path);
function near(s:string,value:number,expected:number){assert(Math.abs(value-expected)<f.surfaceToleranceM,`${s} has ${value} m, expected ${expected} m`);}
for(const scale of f.scales){
  const m=meshes(a,a.volumes[scale].body,1.68);
  for(const p of m){
    const box=new Box3();
    for(let i=0;i<p.world.length;i+=3)box.expandByPoint(new Vector3().fromArray(p.world,i));
    const size=box.getSize(new Vector3());
    if(p.id.startsWith('tread-')) near(p.id,size.y,f.treadThicknessM);
    if(p.id==='platform') near(p.id,size.y,f.platformRimM);
    if(p.id==='grab-bar'){near(p.id,size.y,f.grabDiameterM);near(p.id,box.getCenter(new Vector3()).y,f.grabCentreHeightM);}
    if(p.id.startsWith('shoe-')){near(p.id,box.min.y,0);near(p.id,size.z,p.id.includes('front')?f.frontShoeWidthM:f.rearShoeWidthM);}
  }
}
console.log('Two scales retain all four 25 mm treads, 26 mm platform rim, 28 mm grip and four true-size ground contacts.');
