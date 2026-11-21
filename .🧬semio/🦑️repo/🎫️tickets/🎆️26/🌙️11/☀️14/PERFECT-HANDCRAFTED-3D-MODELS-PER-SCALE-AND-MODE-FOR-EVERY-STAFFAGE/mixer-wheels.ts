/** 🛞 Measure the steering and drive treads, wheel-nut flats and mounted ten-stud circles. */
import {strict as assert} from "node:assert";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {Box3,Triangle,Vector3} from "three";
import {entryOf} from "./volumes-lib.ts";
const dims=JSON.parse(readFileSync(join(import.meta.dir,"mixer-wheels.json"),"utf8"));
const {asset}=entryOf("machinery/concrete-mixer-truck");
const {body,thermal}=asset.volumes["50"];
const near=(a:number,b:number,e=1e-6)=>assert.ok(Math.abs(a-b)<e,`expected ${b}, measured ${a}`);
const tyres=body.filter((s:any)=>s.id.startsWith("tyre-"));
const nuts=body.filter((s:any)=>s.id.startsWith("wheel-nuts-"));
assert.equal(tyres.length,dims.tyres);
assert.equal(nuts.length,dims.lugGroups);
assert.ok(!body.some((s:any)=>s.id.startsWith("tread-groove-")));
let rubberArea=0,seatArea=0;
for(const tyre of tyres){
  const front=/^tyre-[12]-/.test(tyre.id);
  const expectedDepth=front?dims.frontTreadDepthM:dims.rearTreadDepthM;
  const width=front?dims.frontGrooveMouthM:dims.rearGrooveMouthM;
  const points=tyre.profile.map(([x,r]:number[])=>new Vector3(x,r,0));
  for(let i=1;i<points.length;i++){
    const a=points[i-1],b=points[i];
    const area=Math.PI*(a.y+b.y)*a.distanceTo(b);
    rubberArea+=area;
    if(Math.abs(a.y-.288)<1e-6&&Math.abs(b.y-.288)<1e-6)seatArea+=area;
  }
  for(const center of dims.grooveCentresM){
    const cut=points.filter((p:Vector3)=>Math.abs(p.x-center)<width/2+.000001);
    assert.equal(cut.length,4);
    near(dims.crownRadiusM-Math.min(...cut.map((p:Vector3)=>p.y)),expectedDepth);
    near(cut.at(-1).distanceTo(cut[0]),width);
    assert.ok(cut[0].x<cut[1].x&&cut[1].x<cut[2].x&&cut[2].x<cut[3].x);
  }
  assert.ok(thermal.some((s:any)=>s.of===tyre.id&&s.region==="tyres"));
}
near(rubberArea,dims.rubberAreaM2,.005);
near(rubberArea-seatArea,dims.unseatedAreaM2,.005);
const region=asset.thermal.regions.find((r:any)=>r.id==="tyres");
near(region.areaM2,dims.rubberAreaM2);
for(const pose of asset.poses){
  const tyre=pose.thermal.regions.tyres;
  const verticalLoad=pose.loads.filter((load:any)=>load.kind==="carried").reduce((sum:number,load:any)=>sum-load.fyN,asset.dimensions.massKg*9.80665);
  near(tyre.exposedFraction*region.areaM2,dims.unseatedAreaM2,.00005);
  near(tyre.contact.fraction*tyre.exposedFraction*region.areaM2*dims.meanRoadContactPa,verticalLoad,15);
}
for(const nut of nuts){
  const front=/^wheel-nuts-[12]-/.test(nut.id),hand=nut.id.includes("-l")?1:-1;
  const z=hand*(front?1.138:.934);
  assert.equal(nut.material,"galvanised-steel");
  const vertex=nut.vertices.map((p:number[])=>new Vector3(...p));
  const box=new Box3().setFromPoints(vertex);
  near(box.max.z-box.min.z,dims.nutHeightM);
  near(box.max.y-box.min.y,dims.flangeDiameterM);
  near(box.min.x+box.max.x,dims.boltCircleDiameterM);
  const flats=vertex.filter((p:Vector3)=>Math.abs((p.z-z)*hand-.025)<1e-6);
  const normal=new Vector3(Math.cos(Math.PI/6),Math.sin(Math.PI/6),0);
  const distances=flats.map((p:Vector3)=>p.dot(normal));
  near(Math.max(...distances)-Math.min(...distances),dims.spannerAcrossFlatsM);
  near(nut.repeat.count,dims.nutsPerWheel);
  near(nut.repeat.rotationRad*dims.nutsPerWheel,2*Math.PI);
  let signed=0;
  for(const [i,j,k] of nut.triangles){
    const a=vertex[i],b=vertex[j],c=vertex[k];
    assert.ok(new Triangle(a,b,c).getArea()>1e-12);
    signed+=a.dot(new Vector3().crossVectors(b,c))/6;
  }
  assert.ok(signed>0);
  assert.ok(thermal.some((s:any)=>s.of===nut.id&&s.region==="brakes"));
}
for(let axle=1;axle<=4;axle++){
  const shape=asset.representations["50"].geometry.elevation.find((s:any)=>s.id===`wheel-fasteners-${axle}`);
  const paths=shape?.d.match(/M\s+[^Z]+Z/g)??[];
  assert.equal(paths.length,dims.nutsPerWheel);
  for(const path of paths){
    const n=path.match(/-?\d+(?:\.\d+)?(?:e[+-]?\d+)?/g).map(Number);
    const corners=Array.from({length:6},(_,i)=>new Vector3(n[2*i],n[2*i+1],0));
    const center=corners.reduce((sum,p)=>sum.add(p),new Vector3()).multiplyScalar(1/6);
    near(2*center.length(),dims.boltCircleDiameterM,2e-6);
    near(corners[0]!.clone().lerp(corners[1]!,.5).distanceTo(center)*2,dims.spannerAcrossFlatsM,2e-6);
  }
}
console.log(JSON.stringify({tyres:tyres.length,mouldedGrooves:tyres.length*3,nuts:nuts.length*dims.nutsPerWheel,...dims}));
