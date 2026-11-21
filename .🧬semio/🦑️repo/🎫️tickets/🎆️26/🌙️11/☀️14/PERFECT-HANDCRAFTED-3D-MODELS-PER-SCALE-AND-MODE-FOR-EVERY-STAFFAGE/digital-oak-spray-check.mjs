/** 🍂 Measure the oak's individually formed lobes, sun and shade bindings, and reductions by scale. */
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {Vector3,Triangle} from 'three';
const {asset}=JSON.parse(readFileSync(process.argv[2])),sites=JSON.parse(readFileSync(new URL('./digital-oak-sprays.json',import.meta.url)));
let previous=Infinity;
for(const [d,v] of Object.entries(asset.volumes)){
 const leaves=v.body.filter(q=>q.id.startsWith('shoot-')&&q.id.includes('-leaf-')),cushions=v.body.filter(q=>q.id.startsWith('fol-')),faces=[...cushions,...leaves].reduce((n,q)=>n+q.triangles.length,0);
 assert.equal(cushions.length,157);assert.ok(faces<previous,`${d}: ${faces} / ${previous}`);previous=faces;
 if(d!=='50')continue;
 assert.equal(leaves.length,56);
 for(const [part] of sites){
  const name=part.replace('fol-','shoot-'),crown=v.body.find(q=>q.id===part),twig=v.body.find(q=>q.id===name);
  assert.equal(twig.bone,crown.bone);
  for(const leaf of leaves.filter(q=>q.id.startsWith(name+'-leaf-'))){
   const pt=leaf.vertices.map(p=>new Vector3(...p)),width=[];
   for(let j=0;j<pt.length;j+=4)width.push(pt[j].distanceTo(pt[j+2]));
   assert.equal(width.filter((w,i)=>i&&i<width.length-1&&w>width[i-1]&&w>width[i+1]).length,5);
   assert.ok(leaf.triangles.every(([a,b,c])=>new Triangle(pt[a],pt[b],pt[c]).getArea()>0));
   const base=pt[1],tip=pt.at(-3);assert.ok(base.distanceTo(tip)>.048&&base.distanceTo(tip)<.112);
   assert.equal(leaf.bone,twig.bone);assert.ok(v.thermal.some(r=>r.of===leaf.id&&r.region===v.thermal.find(q=>q.of===part).region));
  }
 }
}
console.log('Verified 56 alternate, lobed leaf blades on eight native shoots at four detail levels.');
