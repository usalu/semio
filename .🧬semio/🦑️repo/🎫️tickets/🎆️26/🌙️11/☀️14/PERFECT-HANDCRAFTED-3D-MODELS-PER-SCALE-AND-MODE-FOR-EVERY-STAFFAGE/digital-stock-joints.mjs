import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {Box2,Box3,Ray,Vector2,Vector3} from 'three';
const cases=JSON.parse(readFileSync(new URL('./digital-stock-joints.json',import.meta.url)));
const near=(a,b)=>assert(Math.abs(a-b)<1e-7,`${a} / ${b}`);
for(const file of process.argv.slice(2)){
 const {asset}=JSON.parse(readFileSync(file)),expected=cases[asset.id];
 for(const [scale,vol] of Object.entries(asset.volumes)){
  const body=new Map(vol.body.map(s=>[s.id,s]));
  if(asset.id==='folding-deck-chair'){
   const arm=body.get('strut-arm'),bar=body.get('strut-bar');
   const box=new Box2().setFromPoints(arm.profile.map(p=>new Vector2(...p)));
   near(arm.to-arm.from,expected.propThicknessM);near(box.getSize(new Vector2()).y,expected.propWidthM);
   near(new Vector3(...arm.repeat.translation).length(),expected.bearingSpacingM);
   near(bar.profile.at(-1)[0]-bar.profile[0][0],expected.dowelLengthM);
   near(2*bar.profile[0][1],expected.dowelEndDiameterM);near(2*Math.max(...bar.profile.map(p=>p[1])),expected.dowelShaftDiameterM);
   for(const id of ['strut-bar','strut-arm','rear-rail','foot-rail']){
    assert(vol.structure.some(p=>p.of===id));assert(vol.thermal.some(p=>p.of===id));
   }
  }else if(asset.id==='external-venetian-blind'){
   const panes=['pane-in','pane-out'].map(id=>{
    const pane=body.get(id),size=new Vector3(...pane.size);
    near(size.x,expected.paneM);near(size.y,expected.sightHeightM);near(size.z,expected.sightWidthM);
    return new Box3().setFromCenterAndSize(new Vector3(...pane.transform.translation),size);
   });
   const ray=new Ray(new Vector3(-1,1.625,0),new Vector3(1,0,0));
   const faces=panes.map(p=>ray.intersectBox(p,new Vector3()));assert(faces.every(Boolean));
   near(faces[1].x-faces[0].x-expected.paneM,expected.sealedGapM);
   const spacer=body.get('edge-spacer'),rim=new Box2().setFromPoints(spacer.profile.map(p=>new Vector2(...p)));
   const hole=new Box2().setFromPoints(spacer.holes[0].map(p=>new Vector2(...p)));
   near(hole.min.x-rim.min.x,expected.spacerFaceM);near(rim.max.y-hole.max.y,expected.spacerFaceM);
   near(spacer.holes[0][0][0],hole.max.x);near(hole.min.y+expected.bendRadiusM,spacer.holes[0][0][1]);
   for(let i=0;i<expected.slatCount;i++){
    const id='slat-'+i,slat=body.get(id);assert.equal(slat.bone,id);
    near(slat.to-slat.from,expected.slatSpanM);assert.ok(vol.thermal.some(q=>q.of===id));
   }
   const glazing=asset.thermal.regions.find(r=>r.id==='glazing-behind');
   assert.deepEqual(glazing.layers.map(r=>r.id),['pane-in','cavity','pane-out']);
  }else{
   const knob=body.get('knob'),shaft=vol.structure.find(p=>p.id==='st-knob-stem');
   const box=new Box2().setFromPoints(knob.profile.map(p=>new Vector2(...p)));
   near(box.getSize(new Vector2()).x,expected.knobDepthM);near(2*box.max.y,expected.knobDiameterM);near(2*shaft.r0,expected.spindleDiameterM);
   const axis=new Vector3(...knob.axis),spindle=new Vector3(...shaft.p1).sub(new Vector3(...shaft.p0)).normalize();near(axis.dot(spindle),1);
   for(const id of ['knob','knob-grip','knob-boss'])assert(vol.thermal.some(p=>p.of===id));
  }
  console.log(asset.id,scale,'working dimensions passed');
 }
}
