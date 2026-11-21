import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
import {Box3, Vector3} from 'three';
const f=JSON.parse(readFileSync(new URL('./digital-crate-case.json',import.meta.url)));
const {asset}=JSON.parse(readFileSync('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/storage-crate-stack/🔣️.json'));
function match(actual,expected){for(let i=0;i<expected.length;i++)assert(Math.abs(actual[i]-expected[i])<f.toleranceM,`${actual} outside ${expected}`);}
for(const d of f.scales)for(let k=1;k<=f.crates;k++){
  const v=asset.volumes[d], part=name=>v.body.find(s=>s.id===name+'-'+k);
  for(const [id,section] of [['floor',f.floorM],['foot',f.footM],[d<=100?'skirt':'collar',f.rimM]]){
    const s=part(id);assert(s,id+' at '+d);
    const box=new Box3().setFromPoints(s.profile.flatMap(([y,z])=>[s.from,s.to].map(x=>new Vector3(x,y,z))));
    match([box.min.x,box.max.x],section);
  }
  for(const id of ['a','b','c']){
    const ledge=part('ledge-'+id),centre=new Vector3(...ledge.transform.translation),size=new Vector3(...ledge.size);
    const b=new Box3().setFromCenterAndSize(centre,size);match([b.min.x,b.max.x],f.ledgeM);
  }
  assert(v.structure.some(s=>s.of==='foot-'+k));
  assert(k===1||v.structure.filter(s=>s.part==='joint'&&s.member==='crate-'+k).length===3);
}
console.log('Twenty crate floors, feet and rim seats and sixty stacking ledges measured in Three.js.');
