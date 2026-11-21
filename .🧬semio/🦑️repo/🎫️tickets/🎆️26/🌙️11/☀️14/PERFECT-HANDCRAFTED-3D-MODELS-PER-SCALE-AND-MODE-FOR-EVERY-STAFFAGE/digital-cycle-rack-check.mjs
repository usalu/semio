/** 🚲 Check the open carrier deck, wheel-guard clearance and load path in the human cyclist model. */
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {Line3,Vector3} from 'three';
const {asset}=JSON.parse(readFileSync(process.argv[2])),ref=JSON.parse(readFileSync(new URL('./digital-cycle-rack.json',import.meta.url)));
const eq=(a,b,t=1e-6)=>assert.ok(Math.abs(a-b)<t,`${a}, ${b}`);
for(const [scale,n] of Object.entries(ref.bars)){
 const v=asset.volumes[scale],steel=v.body.filter(q=>q.id.startsWith('carrier-'));
 const p=id=>steel.find(q=>q.id===id),point=(q,i)=>new Vector3(...q.sections.at(i).at);
 assert.equal(steel.filter(q=>q.id.startsWith('carrier-bar-')).length,n);
 for(const suffix of ['l','r']){
  const rail=p('carrier-rail-'+suffix),segment=new Line3(point(rail,0),point(rail,-1));eq(segment.distance(),ref.rackLengthM);eq(segment.start.y,ref.deckHeightM);eq(rail.sections[0].rx*2,ref.railDiameterM);
  const support=p('carrier-leg-'+suffix);eq(point(support,-1).y,ref.dropoutHeightM);
  assert.ok(segment.closestPointToPoint(point(support,0),true,new Vector3()).distanceTo(point(support,0))<.01);
  for(const bar of steel.filter(q=>q.id.startsWith('carrier-bar-'))){
    eq(bar.sections[0].rx*2,ref.crossbarDiameterM);assert.ok(segment.closestPointToPoint(point(bar,suffix==='l'?0:-1),true,new Vector3()).distanceTo(point(bar,suffix==='l'?0:-1))<.005);
  }
 }
 eq(Math.abs(point(p('carrier-rail-l'),0).z-point(p('carrier-rail-r'),0).z),ref.railSpacingM);
 const guard=asset.volumes['50'].body.find(q=>q.id==='rear-fender');eq(ref.deckHeightM-ref.railDiameterM/2-guard.origin[1]-Math.max(...guard.profile.map(q=>q[1])),ref.fenderClearanceM);
 assert.ok(steel.every(s=>v.thermal.some(q=>q.of===s.id&&q.region==='bike-metal')&&v.structure.some(q=>q.of===s.id&&q.member==='frame')));
}
console.log('The load rails, dropout legs, crossbars and wheel-guard clearance passed all four scales.');
