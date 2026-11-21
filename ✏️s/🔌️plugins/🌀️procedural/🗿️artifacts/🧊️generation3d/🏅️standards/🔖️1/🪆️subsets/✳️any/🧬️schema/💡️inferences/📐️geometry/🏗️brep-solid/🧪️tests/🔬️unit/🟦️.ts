import { expect, test } from "bun:test";
import { CircleGeometry, CylinderGeometry, ExtrudeGeometry, LineCurve3, Matrix4, Path, Quaternion, Shape, SphereGeometry, TorusGeometry, Triangle, Vector3 } from "three";
import { readFileSync } from "node:fs";
import { disagreements, type FixtureCase } from "../../../🧪️tests/🧰️oracle-support/🟦️.ts";
import { oracleCase } from "../../../🧪️tests/🧰️construction-support/🟦️.ts";

type XYZ = [number, number, number];
type Fixture = {
  tolerance: number;
  cases: FixtureCase[];
  retainedSphericalShell: { center:XYZ; radius:number; inner:number; thickness:number; volume:number; area:number; refusedThicknesses:number[] }[];
  retainedMeridionalRevolve: { origin:XYZ; axis:XYZ; radial:XYZ; angle:number; inner:number; outer:number; height:number; area:number; volume:number; centroid:XYZ }[];
  retainedPartialRevolve: { origin:XYZ; center:XYZ; axis:XYZ; end:XYZ; radius:number; major:number; angle:number; area:number; volume:number }[];
  retainedConicalLoft: { start:XYZ; end:XYZ; radius0:number; radius1:number; height:number; area:number; volume:number }[];
  retainedClosedHelicalSweep: {
    profile: { radius: number };
    axisOrigin: XYZ;
    axisDirection: XYZ;
    radius: number;
    pitch: number;
    turns: number[];
    area: number;
    volume: number;
    refused: { pitch: number; turns: number; radius: number }[];
  };
};
const fixture = JSON.parse(readFileSync(new URL("../../../../../../🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json", import.meta.url), "utf8")) as Fixture;
const catalogue = JSON.parse(readFileSync(new URL("../../../../../🗂️catalogue/🔣️brep-solid.json", import.meta.url), "utf8")) as { kinds: { id: string }[] };

test("the fixture states every brep.solid kind of the catalogue and no other", () => {
  expect([...new Set(fixture.cases.map(({ kind }) => kind))].sort()).toEqual(catalogue.kinds.map(({ id }) => id).sort());
});

test("every fixture case states a fault or at least one measure", () => {
  for (const { name, outputs, fault } of fixture.cases) expect(fault !== undefined || Object.keys((outputs?.shape ?? {}) as object).length > 1, name).toBe(true);
});

test("three recomputes the volume, area, face count and bounding box of every solid case (signed tetrahedra on ruled solids, LatheGeometry, Pappus integrals)", () => {
  expect(disagreements(fixture.cases, oracleCase, fixture.tolerance)).toEqual([]);
}, { timeout: 120000 });


test("Three.js detects the open line and in-plane circular extrusion fixtures", () => {
  type XYZ = [number, number, number];
  const open = fixture.cases.find(({ name }) => name === "refuse an open line as an extrusion profile")!;
  const { start, end } = (open.inputs as { wire: { inputs: { start: XYZ; end: XYZ } } }).wire.inputs;
  const line = new LineCurve3(new Vector3(...start), new Vector3(...end));
  expect(line.getPoint(0).distanceTo(line.getPoint(1))).toBeGreaterThan(0);
  expect(open.fault?.code).toBe("generation3d.geometry.wire-open");

  const flat = fixture.cases.find(({ name }) => name === "refuse a circular extrusion in the profile plane")!;
  const { wire, vector } = flat.inputs as { wire: { inputs: { normal: XYZ } }; vector: XYZ };
  expect(new Vector3(...wire.inputs.normal).dot(new Vector3(...vector))).toBe(0);
  expect(flat.fault?.code).toBe("generation3d.geometry.extrusion-in-plane");
});

test("Three torus reproduces the zero-pitch circular sweep and signed full turns", () => {
  const row=fixture.retainedClosedHelicalSweep;
  const center=new Vector3(...row.axisOrigin);
  const axis=new Vector3(...row.axisDirection).normalize();
  for (const turns of row.turns) {
    const orientedAxis=axis.clone().multiplyScalar(turns);
    const geometry=new TorusGeometry(row.radius,row.profile.radius,72,144);
    geometry.applyQuaternion(new Quaternion().setFromUnitVectors(new Vector3(0,0,1),orientedAxis));
    geometry.translate(...row.axisOrigin);
    const positions=geometry.getAttribute("position");
    const indices=geometry.getIndex()!;
    let area=0, volume=0;
    const point=(i: number) => new Vector3().fromBufferAttribute(positions,indices.getX(i));
    for (let i=0;i<indices.count;i+=3) {
      const [a,b,c]=[point(i),point(i+1),point(i+2)];
      area+=new Triangle(a,b,c).getArea();
      volume+=a.clone().sub(center).dot(b.clone().sub(center).cross(c.clone().sub(center)))/6;
      if (i%576===0) {
        const local=a.sub(center), height=local.dot(axis);
        const minor=Math.hypot(height,local.addScaledVector(axis,-height).length()-row.radius);
        expect(minor).toBeCloseTo(row.profile.radius,5);
      }
    }
    expect(Math.abs(area-row.area)).toBeLessThan(0.06);
    expect(Math.abs(volume-row.volume)).toBeLessThan(0.025);
  }
  for (const { pitch,turns,radius } of row.refused) {
    expect(pitch!==0||Math.abs(turns)!==1||radius<=row.profile.radius).toBe(true);
  }
});

test("Three verifies the exact two-circle conical loft volume and cylindrical-axis placement", () => {
  for (const row of fixture.retainedConicalLoft) {
    const start=new Vector3(...row.start), end=new Vector3(...row.end);
    const axis=end.clone().sub(start).normalize();
    const geometry=new CylinderGeometry(row.radius1,row.radius0,start.distanceTo(end),192,1,false);
    geometry.applyQuaternion(new Quaternion().setFromUnitVectors(new Vector3(0,1,0),axis));
    geometry.translate(...start.clone().lerp(end,0.5).toArray());
    const positions=geometry.getAttribute("position"), indices=geometry.getIndex()!;
    const point=(i:number)=>new Vector3().fromBufferAttribute(positions,indices.getX(i));
    let area=0,volume=0;
    for(let i=0;i<indices.count;i+=3){
      const [a,b,c]=[point(i),point(i+1),point(i+2)];
      area+=new Triangle(a,b,c).getArea();
      volume+=a.sub(start).dot(b.sub(start).cross(c.sub(start)))/6;
    }
    expect(Math.abs(area-row.area)).toBeLessThan(0.02);
    expect(Math.abs(volume-row.volume)).toBeLessThan(0.02);
    const span=Array.from({length:positions.count},(_,i)=>new Vector3().fromBufferAttribute(positions,i).sub(start).dot(axis));
    expect(Math.min(...span)).toBeCloseTo(0,5);
    expect(Math.max(...span)).toBeCloseTo(row.height,5);
  }
});

test("Three torus and cap charts independently measure partial circular Revolve", () => {
  for (const row of fixture.retainedPartialRevolve) {
    const origin=new Vector3(...row.origin);
    const start=new Vector3(...row.center);
    const axis=new Vector3(...row.axis).normalize().multiplyScalar(Math.sign(row.angle));
    const relative=start.clone().sub(origin);
    const center=origin.clone().addScaledVector(axis,relative.dot(axis));
    const x=start.clone().sub(center).normalize(), y=axis.clone().cross(x);
    const placed=new TorusGeometry(row.major,row.radius,128,160,Math.abs(row.angle));
    placed.applyMatrix4(new Matrix4().makeBasis(x,y,axis));placed.translate(...center.toArray());
    const radial=x.clone().applyAxisAngle(axis,Math.abs(row.angle));
    const end=center.clone().addScaledVector(radial,row.major);
    expect(end.distanceTo(new Vector3(...row.end))).toBeLessThan(1e-12);
    const startCap=new CircleGeometry(row.radius,128), endCap=new CircleGeometry(row.radius,128);
    startCap.applyMatrix4(new Matrix4().makeBasis(x,axis,x.clone().cross(axis)));startCap.translate(...start.toArray());
    const down=axis.clone().negate();
    endCap.applyMatrix4(new Matrix4().makeBasis(radial,down,radial.clone().cross(down)));endCap.translate(...end.toArray());
    let area=0,volume=0;
    for (const geometry of [placed,startCap,endCap]) {
      const positions=geometry.getAttribute("position"), indices=geometry.getIndex()!;
      const point=(i:number)=>new Vector3().fromBufferAttribute(positions,indices.getX(i));
      for(let i=0;i<indices.count;i+=3){
        const [a,b,c]=[point(i),point(i+1),point(i+2)];
        area+=new Triangle(a,b,c).getArea();
        volume+=a.sub(center).dot(b.sub(center).cross(c.sub(center)))/6;
      }
    }
    expect(Math.abs(area-row.area)).toBeLessThan(0.03);
    expect(Math.abs(volume-row.volume)).toBeLessThan(0.025);
  }
});

test("Three sweeps the annular section with exact signed world placement", () => {
  for(const row of fixture.retainedMeridionalRevolve){
    const sweep=Math.min(Math.abs(row.angle),2*Math.PI);
    const profile=new Shape();
    profile.absarc(0,0,row.outer,0,sweep,false);
    if(sweep===2*Math.PI){
      const hole=new Path();
      hole.absarc(0,0,row.inner,0,2*Math.PI,true);
      profile.holes.push(hole);
    }else{
      profile.lineTo(row.inner*Math.cos(sweep),row.inner*Math.sin(sweep));
      profile.absarc(0,0,row.inner,sweep,0,true);
      profile.closePath();
    }
    const geometry=new ExtrudeGeometry(profile,{depth:row.height,curveSegments:128,bevelEnabled:false});
    const x=new Vector3(...row.radial), z=new Vector3(...row.axis).multiplyScalar(Math.sign(row.angle));
    geometry.applyMatrix4(new Matrix4().makeBasis(x,z.clone().cross(x),z));
    const base=new Vector3(...row.origin).addScaledVector(new Vector3(...row.axis),row.angle<0?row.height:0);
    geometry.translate(...base.toArray());
    const points=geometry.getAttribute("position");
    let area=0,volume=0;
    const moment=new Vector3();
    for(let i=0;i<points.count;i+=3){
      const [a,b,c]=[0,1,2].map(j=>new Vector3().fromBufferAttribute(points,i+j));
      area+=new Triangle(a,b,c).getArea();
      a.sub(base);
      b.sub(base);
      c.sub(base);
      const weight=a.dot(b.clone().cross(c))/6;
      volume+=weight;
      moment.addScaledVector(a.add(b).add(c),weight);
    }
    expect(Math.abs(area-row.area)).toBeLessThan(0.025);
    expect(Math.abs(volume-row.volume)).toBeLessThan(0.025);
    expect(moment.divideScalar(4*volume).add(base).distanceTo(new Vector3(...row.centroid))).toBeLessThan(0.003);
  }
});

test("Three verifies the inward offset and signed volume of a closed spherical shell", () => {
  for(const row of fixture.retainedSphericalShell){
    const center=new Vector3(...row.center);
    const measures=(radius:number)=>{
      const geometry=new SphereGeometry(radius,192,96).translate(...row.center);
      const positions=geometry.getAttribute("position"),indices=geometry.index!;
      let area=0,volume=0;
      for(let i=0;i<indices.count;i+=3){
        const a=new Vector3().fromBufferAttribute(positions,indices.getX(i)).sub(center);
        const b=new Vector3().fromBufferAttribute(positions,indices.getX(i+1)).sub(center);
        const c=new Vector3().fromBufferAttribute(positions,indices.getX(i+2)).sub(center);
        area+=b.clone().sub(a).cross(c.clone().sub(a)).length()/2;
        volume+=a.dot(b.cross(c))/6;
      }
      return {area,volume};
    };
    const outer=measures(row.radius),inner=measures(row.inner);
    expect(row.radius-row.thickness).toBe(row.inner);
    expect(Math.abs(outer.volume-inner.volume-row.volume)).toBeLessThan(0.08);
    expect(Math.abs(outer.area+inner.area-row.area)).toBeLessThan(0.08);
    for(const refused of row.refusedThicknesses) expect(refused<=0||row.radius-refused<=0).toBe(true);
  }
});
