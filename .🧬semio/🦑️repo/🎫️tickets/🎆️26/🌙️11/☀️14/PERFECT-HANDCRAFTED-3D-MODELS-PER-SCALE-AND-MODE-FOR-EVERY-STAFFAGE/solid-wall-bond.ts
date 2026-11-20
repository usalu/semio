import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {strict as assert} from "node:assert";
import {Box3,Vector3} from "three";
import expected from "./solid-wall-bond.json";

const path=resolve(import.meta.dir,"../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/solid-brick-wall-1960/🔣️.json");
const asset=JSON.parse(readFileSync(path,"utf8")).asset;
const volume=asset.volumes[expected.scale];
const body=volume.body.flatMap((s:any)=>Array.from({length:s.repeat?.count??1},(_,i)=>({...s,transform:{translation:s.transform.translation.map((n:number,j:number)=>n+i*(s.repeat?.translation[j]??0))}})));
const bricks=body.filter((s:any)=>s.material==="solid-brick");
const mortar=body.filter((s:any)=>s.material==="masonry-mortar");
assert.equal(new Set(bricks.map((s:any)=>s.transform.translation[1])).size,expected.courseCount);
assert.ok(bricks.every((s:any)=>Math.abs(s.size[1]-expected.brickHeightM)<.00002));
assert.ok(bricks.some((s:any)=>Math.abs(s.size[2]-expected.headerWidthM)<.00002));
assert.ok(bricks.some((s:any)=>Math.abs(s.size[2]-expected.stretcherLengthM)<.00002));
assert.ok(mortar.some((s:any)=>Math.abs(s.size[2]-expected.headJointM)<.00002));
const boxes=body.map((s:any)=>new Box3().setFromCenterAndSize(new Vector3(...s.transform.translation),new Vector3(...s.size)));
const outer=new Box3();
let sum=0;
for(const [i,box] of boxes.entries()) {
  const size=box.getSize(new Vector3());
  sum+=size.x*size.y*size.z;
  outer.union(box);
  for(const other of boxes.slice(i+1)) {
    const common=box.clone().intersect(other).getSize(new Vector3());
    assert.ok(Math.min(common.x,common.y,common.z)<.00002);
  }
}
assert.ok(outer.getSize(new Vector3()).distanceTo(new Vector3(...expected.sampleM))<.00002);
assert.ok(Math.abs(sum-.4)<.00002);
console.log(bricks.length,"bricks,",mortar.length,"mortar solids,",sum,"m³");
