import { expect, test } from "bun:test";
import { BoxGeometry, ConeGeometry, CylinderGeometry, EllipseCurve, Line3, Matrix4, Plane, SphereGeometry, TorusGeometry, Triangle, Vector3 } from "three";
import fixture from "./🔣️original-analytic-projection.json";

const vector = (axes: number[]) => new Vector3(axes[0], axes[1], axes[2]);

test("Three.js agrees with the original line parameter and projected point", () => {
  const row = fixture.line;
  const line = new Line3(vector(row.start), vector(row.end));
  const target = vector(row.target);
  expect(line.closestPointToPointParameter(target, false)).toBe(row.parameter);
  const projected = line.closestPointToPoint(target, false, new Vector3());
  expect(projected.toArray()).toEqual(row.point);
  expect(projected.distanceTo(target)).toBeCloseTo(row.distance, 12);
});

test("Three.js ellipse samples bracket the nearest parametric point", () => {
  const row = fixture.ellipse;
  const curve = new EllipseCurve(0,0,row.radii[0],row.radii[1],0,2*Math.PI);
  const target = vector(row.target);
  let best = { distance: Infinity, parameter: 0 };
  curve.getPoints(8192).forEach((point,index) => {
    const distance = new Vector3(-point.y,point.x,0).distanceTo(target);
    if (distance < best.distance) best = { distance, parameter: index * 2*Math.PI / 8192 };
  });
  expect(Math.abs(best.parameter-row.parameter)).toBeLessThan(0.001);
  expect(best.distance).toBeCloseTo(row.distance, 6);
  const projected = curve.getPoint(row.parameter/(2*Math.PI));
  expect(new Vector3(-projected.y,projected.x,0).distanceTo(vector(row.point))).toBeLessThan(1e-12);
});

test("Three.js agrees on plane projection and distance", () => {
  const row = fixture.plane;
  const plane = new Plane().setFromNormalAndCoplanarPoint(vector(row.normal),vector(row.origin));
  const target = vector(row.target);
  expect(plane.projectPoint(target,new Vector3()).toArray()).toEqual(row.point);
  expect(plane.distanceToPoint(target)).toBe(row.distance);
});

test("Three.js projects nonorthogonal plane coordinates through the true dual basis",()=>{
  const row=fixture.skewPlane;
  const plane=new Plane().setFromNormalAndCoplanarPoint(vector(row.normal),vector(row.origin));
  const target=vector(row.target);
  const projected=plane.projectPoint(target,new Vector3());
  const inverse=new Matrix4().makeBasis(vector(row.xAxis),vector(row.yAxis),vector(row.normal)).setPosition(vector(row.origin)).invert();
  expect(projected.toArray()).toEqual(row.point);
  expect(projected.applyMatrix4(inverse).toArray().slice(0,2)).toEqual(row.uv);
  expect(plane.distanceToPoint(target)).toBe(row.distance);
});

test("Three.js agrees with original circular trim nearest points and filled disk projections",()=>{
  for(const row of fixture.circularEdge){
    const curve=new EllipseCurve(0,0,row.radius,row.radius,row.range[0],row.range[1]);
    const target=vector(row.target);
    const closest=curve.getPoints(8192).map(p=>new Vector3(p.x,p.y,0)).reduce((a,b)=>a.distanceToSquared(target)<=b.distanceToSquared(target)?a:b);
    expect(closest.distanceTo(vector(row.point))).toBeLessThan(0.0004);
  }
  const plane=new Plane(new Vector3(0,0,1));
  for(const row of fixture.disk.points){
    const projected=plane.projectPoint(vector(row.target),new Vector3()).clampLength(0,fixture.disk.radius);
    expect(projected.distanceTo(vector(row.point))).toBeLessThan(1e-12);
  }
});

test("certified primitive nearest points agree with Three's triangulated surfaces",()=>{
  const axial=(geometry:CylinderGeometry|ConeGeometry,height:number)=>geometry.rotateX(Math.PI/2).translate(0,0,height/2);
  for(const solid of fixture.solids){
    const p:Record<string,number|undefined>=solid.parameters;
    const mesh=solid.kind==="box"?new BoxGeometry(p.width!,p.depth!,p.height!).translate(p.width!/2,p.depth!/2,p.height!/2)
       :solid.kind==="sphere"?new SphereGeometry(p.radius!,128,96)
       :solid.kind==="cylinder"?axial(new CylinderGeometry(p.radius!,p.radius!,p.height!,128),p.height!)
       :solid.kind==="cone"?axial(new ConeGeometry(p.radius!,p.height!,128),p.height!)
       :new TorusGeometry(p.major!,p.minor!,96,128);
    const positions=mesh.getAttribute("position"),indices=mesh.getIndex()!;
    const triangle=new Triangle();
    const nearest=new Vector3();
    for(const row of solid.points){
      const target=vector(row.target),expected=vector(row.point);
      let minimum=Infinity;
      for(let i=0;i<indices.count;i+=3){
        triangle.a.fromBufferAttribute(positions,indices.getX(i));
        triangle.b.fromBufferAttribute(positions,indices.getX(i+1));
        triangle.c.fromBufferAttribute(positions,indices.getX(i+2));
        minimum=Math.min(minimum,triangle.closestPointToPoint(target,nearest).distanceTo(target));
      }
      expect(Math.abs(minimum-expected.distanceTo(target))).toBeLessThan(0.003);
    }
    mesh.dispose();
  }
});
