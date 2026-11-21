/** 🕳️ Cast independent inspection rays through each manufactured bore and through its surrounding material. */
import assert from 'node:assert/strict';
import {readFileSync,readdirSync} from 'node:fs';
import {join} from 'node:path';
import {Vector3,Ray} from 'three';
import {tessellateRevolve} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🌀️revolved/🟦️.ts';
const root='🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog';
const cases=JSON.parse(readFileSync(join(import.meta.dir,'equipment-bore-case.json'),'utf8'));
let count=0;
for(const [slug,scales] of Object.entries(cases)){
 const category=readdirSync(root).find(c=>c.endsWith(slug==='glass-balustrade'?'components':'vehicles'))!;
 const {asset:a}=JSON.parse(readFileSync(join(root,category,slug,'🔣️.json'),'utf8'));
 for(const [scale,parts] of Object.entries(scales as object))for(const [prefix,[number,radius]] of Object.entries(parts as Record<string,[number,number]>)){
  const level=a.volumes[scale];
  const solids=(prefix.startsWith('th-')?level.thermal:level.body).filter((s:any)=>s.id===prefix||prefix.endsWith('-')&&s.id.startsWith(prefix));
  assert.equal(solids.length,number,prefix);
  for(const solid of solids){
   const outer=Math.max(...solid.profile.map((v:number[])=>v[1]));
   assert.equal(Math.min(...solid.profile.map((v:number[])=>v[1])),radius);
   const mesh=tessellateRevolve(solid,{chordM:.0005,minSegments:24,maxSegments:80});
   const axis=new Vector3(...solid.axis).normalize(),origin=new Vector3(...solid.origin);
   const off=new Vector3(0,1,0);if(Math.abs(off.dot(axis))>.9)off.set(1,0,0);off.cross(axis).normalize();
   const start=origin.clone().addScaledVector(axis,Math.min(...solid.profile.map((v:number[])=>v[0]))-1);
   for(const [r,shouldHit] of [[radius*.5,false],[(radius+outer)*.5,true]] as const){
    const ray=new Ray(start.clone().addScaledVector(off,r),axis);
    let hit=false;
    for(let j=0;j<mesh.indices.length&&!hit;j+=3){
     const p=mesh.indices.slice(j,j+3).map(i=>new Vector3().fromArray(mesh.positions,i*3));
     hit=ray.intersectTriangle(p[0]!,p[1]!,p[2]!,false,new Vector3())!==null;
    }
    assert.equal(hit,shouldHit,`${slug} ${solid.id} r=${r}`);
   }
   count++;
  }
 }
}
console.log(JSON.stringify({closedBores:count,rayQueries:2*count}));
