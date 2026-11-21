/** 🪵 Measure the exterior plywood including the bench seat and its closed storage bay. */
import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
import polygon from 'polygon-clipping';
import {ShapeUtils,Vector2} from 'three';
const {asset}=JSON.parse(readFileSync(process.argv[2])),v=asset.volumes[50];
const ids=new Set(v.thermal.filter(q=>q.region==='box-wood').map(q=>q.of));
const wood=v.body.filter(q=>ids.has(q.id));assert.equal(wood.length,11);
assert.ok(wood.every(q=>q.bone==='box'&&q.plane==='xy'&&q.kind==='extrude'&&!q.transform));
const seat=wood.find(q=>q.id==='bench-seat'),front=wood.find(q=>q.id==='bench-panel'),rear=wood.find(q=>q.id==='box-rear'),floor=wood.find(q=>q.id==='box-floor');
const lo=Math.max(...rear.profile.map(p=>p[0])),hi=Math.min(...front.profile.map(p=>p[0]));
const low=Math.max(...floor.profile.map(p=>p[1])),high=Math.min(...seat.profile.map(p=>p[1]));
function area(set){return set.reduce((sum,poly)=>sum+poly.reduce((s,r,i)=>s+(i?-1:1)*Math.abs(ShapeUtils.area(r.map(([x,y])=>new Vector2(x,y)))),0),0);}
function perimeter(set){return set.flat().reduce((s,r)=>s+r.slice(1).reduce((v,p,i)=>v+Math.hypot(p[0]-r[i][0],p[1]-r[i][1]),0),0);}
function enclosed(parts){
 const cuts=[...new Set(parts.flatMap(p=>[p.from,p.to]))].sort((a,b)=>a-b);let prev=[],total=0;
 for(let i=1;i<cuts.length;i++){
  const mid=(cuts[i]+cuts[i-1])/2;const on=parts.filter(q=>q.from<mid&&q.to>mid).map(q=>[q.profile,...q.holes??[]]);
  const section=on.length?polygon.union(...on):[];
  total+=area(polygon.xor(prev,section))+perimeter(section)*(cuts[i]-cuts[i-1]);prev=section;
 }
 return total+area(prev);
}
const face=enclosed(wood),external=enclosed([...wood,{from:seat.from,to:seat.to,profile:[[lo,low],[hi,low],[hi,high],[lo,high]]}]);
assert.ok(external<face);console.log(JSON.stringify({panels:wood.length,allAccessibleAndSealedM2:face,externalM2:external,nominalAreaM2:+external.toFixed(2)}));
assert.equal(asset.thermal.regions.find(q=>q.id==='box-wood').areaM2,+external.toFixed(2));
