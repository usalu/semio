import { expect, test } from "bun:test";
import { BoxGeometry, BufferGeometry, ConeGeometry, CylinderGeometry, Matrix4, SphereGeometry, TorusGeometry, Triangle, Vector3 } from "three";
import { ConvexGeometry } from "three/addons/geometries/ConvexGeometry.js";
import fixture from "./🔣️original-solids.json";

function solidIntegrals(shape: BufferGeometry): {volume:number;center:Vector3} {
  const positions = shape.getAttribute("position");
  const index = shape.getIndex();
  const source = (offset: number) => index?.getX(offset) ?? offset;
  const a = new Vector3();
  const b = new Vector3();
  const c = new Vector3();
  let volume = 0;
  const weightedCenter = new Vector3();
  for (let offset = 0; offset < (index?.count ?? positions.count); offset += 3) {
    a.fromBufferAttribute(positions, source(offset));
    b.fromBufferAttribute(positions, source(offset + 1));
    c.fromBufferAttribute(positions, source(offset + 2));
    const signed = a.dot(new Vector3().crossVectors(b,c)) / 6;
    weightedCenter.addScaledVector(a.add(b).add(c),signed/4);
    volume += signed;
  }
  shape.dispose();
  return {volume:Math.abs(volume),center:weightedCenter.divideScalar(volume)};
}

function surfaceArea(shape: BufferGeometry): number {
  const points=shape.getAttribute("position");
  const index=shape.getIndex();
  const tri=new Triangle();
  let area=0;
  for(let position=0;position<(index?.count ?? points.count);position+=3){
    tri.a.fromBufferAttribute(points,index?.getX(position) ?? position);
    tri.b.fromBufferAttribute(points,index?.getX(position+1) ?? position+1);
    tri.c.fromBufferAttribute(points,index?.getX(position+2) ?? position+2);
    area+=tri.getArea();
  }
  return area;
}

test("Three.js matches the analytic solid fixture areas and volumes", () => {
  const [width, depth, height] = fixture.dimensions;
  expect(solidIntegrals(new BoxGeometry(width, depth, height)).volume).toBeCloseTo(fixture.volume, 12);

  for (const sample of fixture.curved) {
    const values: Record<string, number> = sample.values;
    const segments = 384;
    const shapes: Record<string, () => BufferGeometry> = {
      "brep.prim3d.sphere": () => new SphereGeometry(values.radius, segments, segments / 2),
      "brep.prim3d.cylinder": () => new CylinderGeometry(values.radius, values.radius, values.height, segments),
      "brep.prim3d.cone": () => new ConeGeometry(values.radius, values.height, segments),
      "brep.prim3d.torus": () => new TorusGeometry(values.major, values.minor, segments / 2, segments),
    };
    const independentMesh = shapes[sample.operator]();
    expect(Math.abs(surfaceArea(independentMesh) / sample.area - 1)).toBeLessThan(0.0005);
    const measure = solidIntegrals(independentMesh);
    expect(Math.abs(measure.volume / sample.volume - 1)).toBeLessThan(0.0005);
    const center = measure.center;
    if (sample.operator === "brep.prim3d.cylinder" || sample.operator === "brep.prim3d.cone") center.set(center.x,-center.z,center.y+values.height/2);
    expect(center.distanceTo(new Vector3(...sample.center))).toBeLessThan(0.0005);
  }
});


test("retained cylinder rigid transforms preserve the geometric center", () => {
  const row=fixture.cylinderAffine;
  const cylinder = new CylinderGeometry(2,2,4,128);
  cylinder.rotateX(Math.PI/2).translate(...[...row.offset.slice(0,2),row.offset[2]+2] as [number,number,number]);
  cylinder.applyMatrix4(new Matrix4().makeRotationAxis(new Vector3(...row.axis),row.angle));
  cylinder.computeBoundingBox();
  const actual=cylinder.boundingBox!.getCenter(new Vector3());
  for (const [i,n] of actual.toArray().entries()) expect(n).toBeCloseTo(row.center[i],12);
  cylinder.dispose();
});

test("Three.js matches the translated planar-solid volume", () => {
  const points = fixture.planarVolume.points.map(([x, y, z]) => new Vector3(x, y, z));
  const shape = new ConvexGeometry(points);
  expect(surfaceArea(shape)).toBeCloseTo(fixture.planarVolume.area,12);
  expect(solidIntegrals(shape).volume).toBeCloseTo(fixture.planarVolume.volume, 12);
});
