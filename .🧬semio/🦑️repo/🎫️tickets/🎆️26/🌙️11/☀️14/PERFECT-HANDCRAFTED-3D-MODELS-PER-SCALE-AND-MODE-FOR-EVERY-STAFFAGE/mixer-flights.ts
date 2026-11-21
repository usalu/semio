/** 🌀 Verify twin-flight pitch, shell attachment, spout clearance and shaft kinematics at every authored scale. */
import {strict as assert} from "node:assert";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {Quaternion,Vector3} from "three";
import {Canvas} from "./volumes-raster.ts";
import {drawAxon,fit} from "./volumes-render.ts";
import {cameraAt,meshesIn,rigAt,savePng} from "./r3-lib.ts";
import {entryOf,GENERATED} from "./volumes-lib.ts";
const f=JSON.parse(readFileSync(join(import.meta.dir,"mixer-flights.json"),"utf8"));
const {asset}=entryOf("machinery/concrete-mixer-truck");
const near=(a:number,b:number,epsilon=1e-6)=>assert.ok(Math.abs(a-b)<epsilon,`${a} vs ${b}`);
const rotor="drum-spiral";
const owner=asset.rig.bones.find((b:any)=>b.id===rotor);
assert.equal(owner.mass?.kg,f.barrelMassKg);
assert.equal(asset.rig.bones.find((b:any)=>b.id==="drum").mass,undefined);
near(asset.rig.bones.reduce((sum:number,b:any)=>sum+(b.mass?.kg??0),0),asset.dimensions.massKg);
const transit=asset.poses.find((p:any)=>p.id==="transit-agitating");
near(asset.dimensions.massKg-transit.loads[0].fyN/asset.structure.gravityMS2,f.fullTruckMassKg,.5);
for(const [den,v] of Object.entries(asset.volumes) as [string,any][]){
  const fixedDrive=v.structure.find((s:any)=>s.id==="member-drive-bearing-housing");
  assert.equal(fixedDrive.member,"drum");assert.equal(fixedDrive.part,"member");
  assert.equal(v.body.find((s:any)=>s.id===fixedDrive.of).bone,"drum");
  const mass=v.structure.find((s:any)=>s.id==="mass-drum");
  assert.equal(mass.member,rotor);assert.equal(mass.bone,rotor);
  near(new Vector3(...mass.transform.translation).distanceTo(new Vector3(...f.barrelMassCentreM)),0);
  const shell=den==="200"?"drum-tube":den==="500"?"member-drum-body":"member-drum-shell";
  assert.equal(v.structure.find((s:any)=>s.id===shell).member,rotor);
}
const meridian=(x:number)=>{
  const i=f.exteriorMeridian.findIndex((p:number[])=>p[0]>=x);
  const [a,ra]=f.exteriorMeridian[i-1], [b,rb]=f.exteriorMeridian[i];
  return ra+(rb-ra)*(x-a)/(b-a);
};
for(const [den,want] of Object.entries(f.scaleStations)){
  const volume=asset.volumes[den];
  const blades=volume.body.filter((s:any)=>s.id.startsWith("drum-blade-")).map((s:any)=>{
    assert.equal(s.kind,"polyhedron");
    const count=den==="50"?6:4;
    const sections=[];
    for(let i=0;i<s.vertices.length;i+=count){
      const ring=s.vertices.slice(i,i+count).map((p:number[])=>new Vector3(...p));
      const xs=ring.map((p:Vector3)=>p.x),rs=ring.map((p:Vector3)=>Math.hypot(p.y,p.z));
      const inner=Math.min(...rs),outer=Math.max(...rs),mid=(outer+inner)/2;
      const theta=Math.atan2(ring[0].z,ring[0].y);
      sections.push({at:[(Math.max(...xs)+Math.min(...xs))/2,mid*Math.cos(theta),mid*Math.sin(theta)],ry:(outer-inner)/2,rx:(Math.max(...xs)-Math.min(...xs))/2,rotation:[Math.sin(theta/2),0,0,Math.cos(theta/2)]});
    }
    return {...s,sections};
  });
  assert.equal(blades.length,2);
  assert.ok(blades.every((s:any)=>s.bone===rotor&&s.sections.length===want&&volume.structure.some((r:any)=>r.of===s.id)));
  let travel=0,last:number|undefined,minSpoutGap=Infinity,attachment=Infinity;
  let previousLip:Vector3|undefined, previousRoot:Vector3|undefined;
  for(let i=0;i<blades[0].sections.length;i++){
    const a=blades[0].sections[i],b=blades[1].sections[i];
    near(a.at[0],b.at[0]);near(a.at[1],-b.at[1]);near(a.at[2],-b.at[2]);
    const center=new Vector3(...a.at);
    const radial=new Vector3(0,1,0).applyQuaternion(new Quaternion(...a.rotation));
    const tip=center.clone().addScaledVector(radial,a.ry);
    const edge=center.clone().addScaledVector(radial,-a.ry);
    const x=center.x,contact=Math.hypot(tip.y,tip.z),lip=Math.hypot(edge.y,edge.z);
    assert.ok(contact<meridian(x)+1e-6);
    attachment=Math.min(attachment,contact-meridian(x)+f.skinM[den]);
    if(previousLip)for(let sample=0;sample<=25;sample++){
      const p=previousLip.clone().lerp(edge,sample/25);
      if(p.x+a.rx>=f.spoutStartM)minSpoutGap=Math.min(minSpoutGap,Math.hypot(p.y,p.z)-f.spoutMaxRadiusM-f.spoutOffsetM);
    }
    if(previousRoot)for(let sample=0;sample<=25;sample++){
      const p=previousRoot.clone().lerp(tip,sample/25);
      attachment=Math.min(attachment,Math.hypot(p.y,p.z)-meridian(p.x)+f.skinM[den]);
    }
    previousLip=edge;previousRoot=tip;
    const theta=Math.atan2(center.z,center.y);
    if(last!==undefined)travel+=Math.atan2(Math.sin(theta-last),Math.cos(theta-last));
    last=theta;
  }
  assert.ok(attachment>0);
  assert.ok(minSpoutGap>0);
  near(-travel/(2*Math.PI),f.turns);
  near(blades[0].sections.at(-1).at[0]-blades[0].sections[0].at[0],f.spanM);
  console.log(JSON.stringify({den,stations:want,spanM:f.spanM,turns:-travel/(2*Math.PI),shellPenetrationM:attachment,spoutClearanceM:minSpoutGap}));
}
const rest=rigAt(asset,50,{kind:"pose",pose:"parked-off"}).frames;
for(const clip of f.clips){
  for(const [t,degrees] of clip.states){
    const rig=rigAt(asset,50,{kind:"clip",clip:clip.id,timeS:t});
    const current=rig.frames[rotor],parent=rig.frames.drum;
    assert.deepEqual(parent,rest.drum);
    near(rig.state.bones[rotor].offset.twistDeg,degrees,.001);
    const axis=new Vector3(...parent.u);
    const v=new Vector3(...rest[rotor].v).applyAxisAngle(axis,-degrees*Math.PI/180);
    near(new Vector3(...current.v).distanceTo(v),0);
    near(new Vector3(...current.o).distanceTo(new Vector3(...rest[rotor].o)),0);
  }
}
const volume=asset.volumes["50"];
const parts=volume.body.filter((s:any)=>/^(drum-shell|drum-blade-[12]|hopper-spout)$/.test(s.id));
const meshes=meshesIn(asset,rest,parts,50);
const crop=new Canvas(760,320,[247,247,244]);
const cam=cameraAt(151,11);
const ax=fit(meshes,cam,760,320,.04);
const items=meshes.map((mesh,i)=>({mesh,color:(parts[i].id==="drum-shell"?[192,203,213]:parts[i].id.endsWith("1")?[226,166,71]:parts[i].id.endsWith("2")?[88,144,159]:[115,124,132]) as [number,number,number]}));
drawAxon(crop,ax,items.filter((_,i)=>parts[i].id!=="drum-shell"));
drawAxon(crop,ax,items.filter((_,i)=>parts[i].id==="drum-shell"),{cut:true,clearDepth:false});
console.log(savePng(join(GENERATED,"equipment-check","mixer-flights.png"),crop));
