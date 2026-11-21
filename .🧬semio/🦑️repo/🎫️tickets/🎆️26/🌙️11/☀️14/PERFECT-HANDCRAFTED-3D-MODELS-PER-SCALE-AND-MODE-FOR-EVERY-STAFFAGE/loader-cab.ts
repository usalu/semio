/** 🚜 Verify the wiper contact and door opening on the published loader and render its cab from the front and door side. */
import {strict as assert} from "node:assert";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {Box3, Vector2, Vector3} from "three";
import {entryOf,GENERATED} from "./volumes-lib.ts";
import {cameraAt,listsIn,rigAt,savePng} from "./r3-lib.ts";
import {Canvas} from "./volumes-raster.ts";
import {drawAxon,fit} from "./volumes-render.ts";
const expected=JSON.parse(readFileSync(join(import.meta.dir,"loader-cab.json"),"utf8"));
const {asset}=entryOf("machinery/"+expected.asset);
const near=(a:number,b:number)=>assert.ok(Math.abs(a-b)<1e-6,`${a} vs ${b}`);
const crop=new Canvas(780,395,[248,248,245]);
for(const [scale,expectedCount] of Object.entries(expected.scales)){
  const volume=asset.volumes[scale];
  const parts=volume.body.filter((s:any)=>s.id.startsWith("cab-wiper-")||s.id==="cab-door-gasket");
  assert.equal(parts.length,expectedCount);
  if(!parts.length)continue;
  for(const part of parts)assert.ok(volume.thermal.some((s:any)=>s.of===part.id&&s.region==="cab"));
  const gasket=parts.find((s:any)=>s.id==="cab-door-gasket");
  const blade=parts.find((s:any)=>s.id==="cab-wiper-blade");
  const arm=parts.find((s:any)=>s.id==="cab-wiper-arm");
  const screen=volume.body.find((s:any)=>s.id==="cab-windscreen");
  const side=volume.body.find((s:any)=>s.id==="cab-glass-side-r");
  const sideBounds=new Box3().setFromCenterAndSize(new Vector3(...side.transform.translation),new Vector3(...side.size));
  const aperture=gasket.holes[0].map(([x,y]:number[])=>new Vector3(x,y,gasket.from));
  const clear=new Box3().setFromPoints(aperture).getSize(new Vector3());
  near(clear.x,expected.clearOpeningM[0]);near(clear.y,expected.clearOpeningM[1]);
  near(gasket.holes[0][6][1]-gasket.holes[0][1][1],clear.y);
  near(gasket.holes[0][0][0]-gasket.profile[0][0],expected.doorSealWidthM);
  for(const [x,y] of gasket.profile)assert.ok(sideBounds.containsPoint(new Vector3(x,y,sideBounds.min.z)));
  assert.ok(gasket.from<sideBounds.min.z&&gasket.to>sideBounds.min.z);
  const points=blade.profile.map(([y,z]:number[])=>new Vector2(y,z));
  const span=Math.max(...points.flatMap((p:Vector2)=>points.map((q:Vector2)=>q.distanceTo(p))));
  assert.ok(span>=expected.bladeSpanM[0]&&span<=expected.bladeSpanM[1]);
  near(Math.max(...screen.profile.map(([x]:number[])=>x))-blade.from,expected.bladeContactDepthM);
  near(2*arm.r0,expected.armDiameterM);
  const midpoint=new Vector2(arm.p1[1],arm.p1[2]);
  const axis=points[2].clone().sub(points[0]);
  const along=midpoint.clone().sub(points[0]).dot(axis)/axis.lengthSq();
  assert.ok(along>0.4&&along<0.6&&points[0].clone().addScaledVector(axis,along).distanceTo(midpoint)<0.01);
  if(scale==="50")near(2*Math.max(...parts.find((s:any)=>s.id==="cab-wiper-spindle").profile.map((p:number[])=>p[1])),expected.spindleDiameterM);
  if(scale==="50"){
    const cab=structuredClone(asset);
    cab.volumes[scale]={...volume,body:volume.body.filter((s:any)=>/^(cab-|door-handle|mirror-|work-lamp)/.test(s.id)&&s.id!=="cab-riser"),thermal:[],structure:[]};
    const rig=rigAt(cab,50,{kind:"pose",pose:asset.presentation.defaultPose});
    const meshes=listsIn(cab,50,rig.frames);
    assert.deepEqual(meshes.errors,[]);
    const items=meshes.body.map((mesh,i)=>{
      const part=meshes.solids.body[i];
      const color:[number,number,number]=part.material==="glass"?[155,201,217]:part.id.startsWith("cab-wiper-")||part.id==="cab-door-gasket"?[44,48,50]:[210,164,72];
      return {mesh,color};
    });
    for(const [i,turn] of [83,22].entries()){
      const view=new Canvas(390,370,[248,248,245]);
      drawAxon(view,fit(meshes.body,cameraAt(turn,5),390,370,.04),items);
      crop.blit(view,i*390,22);
      crop.text(i*390+12,7,i===0?"WINDSCREEN":"DRIVER DOOR",[40,40,40],1);
    }
  }
  console.log(JSON.stringify({scale,parts:parts.length,clearOpeningM:[clear.x,clear.y],bladeSpanM:span}));
}
near(asset.thermal.regions.find((r:any)=>r.id==="cab").areaM2,expected.paintedCabAreaM2);
console.log(savePng(join(GENERATED,"equipment-check","loader-cab.png"),crop));
