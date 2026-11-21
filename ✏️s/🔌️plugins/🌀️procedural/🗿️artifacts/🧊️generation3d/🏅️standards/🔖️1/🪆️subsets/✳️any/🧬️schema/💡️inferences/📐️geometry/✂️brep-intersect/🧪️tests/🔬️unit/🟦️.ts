import { expect, test } from "bun:test";
import { Box3, CylinderGeometry, EllipseCurve, Line3, Plane, Ray, ShapeUtils, Sphere, Vector2, Vector3 } from "three";
import { agree, casesOf, fixtureDisagreements, loadFixture, type BrepCase, type ShapeExpectation } from "../../../⏱️phased-job/🧪️tests/🧰️support/🟦️.ts";

const fixture = loadFixture(import.meta.url);

type Recipe = { recipe: string; start?: number[]; end?: number[]; origin?: number[]; normal?: number[] };
const vector = (value: number[]): Vector3 => new Vector3(...(value as [number, number, number]));
const lineOf = (recipe: Recipe): Line3 => new Line3(vector(recipe.start!), vector(recipe.end!));

test("three agrees with the fixture on volume, area, bounds and closedness of every pinned section face, split half and intersection wire", () => {
  expect(fixtureDisagreements(fixture)).toEqual([]);
});

test("the two halves of every split add up to the box and sit on the stated sides of the plane", () => {
  for (const fixtureCase of casesOf(fixture, "brep.intersect.split").filter(item => item.outputs)) {
    const plane = fixtureCase.inputs.plane as { origin: number[]; normal: number[] };
    const reference = new Plane().setFromNormalAndCoplanarPoint(vector(plane.normal).normalize(), vector(plane.origin));
    const stock = fixtureCase.inputs.solid as { size: number[] };
    let totalVolume = 0;
    for (const [port, sign] of [["positive", 1], ["negative", -1]] as const) {
      const side = fixtureCase.outputs![port] as Required<Pick<ShapeExpectation, "bbox" | "volume">>;
      const bounds = new Box3(vector(side.bbox[0]), vector(side.bbox[1]));
      const size = bounds.getSize(new Vector3());
      expect(agree(size.x * size.y * size.z, side.volume, fixture.tolerance), fixtureCase.name).toBe(true);
      totalVolume += side.volume;
      const [min, max] = [bounds.min, bounds.max];
      for (const x of [min.x, max.x]) for (const y of [min.y, max.y]) for (const z of [min.z, max.z]) {
        expect(sign * reference.distanceToPoint(new Vector3(x, y, z)), fixtureCase.name).toBeGreaterThanOrEqual(-fixture.tolerance);
      }
    }
    expect(agree(totalVolume, stock.size.reduce((a, b) => a * b, 1), fixture.tolerance), fixtureCase.name).toBe(true);
  }
});

test("the stated crossing of two lines and of a line with a plane is where three computes it", () => {
  const crossing = casesOf(fixture, "brep.intersect.curveCurve").find(item => item.name === "two crossing lines meet once")!;
  const [a, b] = [lineOf(crossing.inputs.a as Recipe), lineOf(crossing.inputs.b as Recipe)];
  const mid = a.closestPointToPoint(b.at(0.5, new Vector3()), false, new Vector3());
  expect(mid.distanceTo(new Vector3(...((crossing.outputs!.points as number[][])[0] as [number, number, number])))).toBeLessThan(1e-9);
  const piercing = casesOf(fixture, "brep.intersect.curveSurface").find(item => item.name === "a line pierces a plane")!;
  const plane = piercing.inputs.surface as Recipe;
  const hit = new Plane().setFromNormalAndCoplanarPoint(vector(plane.normal!).normalize(), vector(plane.origin!)).intersectLine(lineOf(piercing.inputs.curve as Recipe), new Vector3());
  expect(hit?.distanceTo(vector((piercing.outputs!.points as number[][])[0]!))).toBeLessThan(1e-9);
});

test("the stated circle and line crossings lie on both curves", () => {
  const circleLine = casesOf(fixture, "brep.intersect.curveCurve").find(item => item.name.startsWith("a circle and a line"))! as BrepCase;
  const line = lineOf(circleLine.inputs.b as Recipe);
  for (const point of circleLine.outputs!.points as number[][]) {
    const found = vector(point);
    expect(Math.abs(found.length() - 1)).toBeLessThan(1e-9);
    expect(line.closestPointToPoint(found, false, new Vector3()).distanceTo(found)).toBeLessThan(1e-9);
  }
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

test("Three reconstructs the retained plane intersection line and classifies parallel planes", () => {
  type PlaneInput = { origin: number[]; normal: number[] };
  type Case = { a: PlaneInput; b: PlaneInput; fault?: string; samples?: [number,number[]][] };
  const rows = (fixture as typeof fixture & { retainedPlaneIntersection: Case[] }).retainedPlaneIntersection;
  const plane = ({ origin, normal }: PlaneInput): Plane => new Plane().setFromNormalAndCoplanarPoint(vector(normal).normalize(), vector(origin));
  for (const row of rows) {
    const a = plane(row.a), b = plane(row.b);
    const direction = a.normal.clone().cross(b.normal);
    if (row.fault) {
      expect(direction.lengthSq()).toBe(0);
      const separation = Math.abs(a.distanceToPoint(b.coplanarPoint(new Vector3())));
      expect(separation <= fixture.tolerance).toBe(row.fault.includes("infinitely many"));
      continue;
    }
    const origin = a.projectPoint(new Vector3(), new Vector3());
    const search = b.normal.clone().addScaledVector(a.normal, -b.normal.dot(a.normal)).normalize();
    const start = origin.clone().addScaledVector(search, -100);
    const end = origin.clone().addScaledVector(search, 100);
    const hit = b.intersectLine(new Line3(start, end), new Vector3())!;
    for (const [t, expected] of row.samples!) {
      const observed = hit.clone().addScaledVector(direction.clone().normalize(), t);
      expect(observed.distanceTo(vector(expected))).toBeLessThan(fixture.tolerance);
      expect(Math.abs(a.distanceToPoint(observed))).toBeLessThan(fixture.tolerance);
      expect(Math.abs(b.distanceToPoint(observed))).toBeLessThan(fixture.tolerance);
    }
  }
});

test("Three verifies continuous sphere and perpendicular cylinder section circles", () => {
  type Row = { plane: { origin: number[]; normal: number[] }; axial: { kind: string; origin: number[]; axis: number[]; radius: number }; center?: number[]; radius?: number; fault?: string };
  for (const row of (fixture as typeof fixture & { retainedAxialIntersection: Row[] }).retainedAxialIntersection) {
    const normal = vector(row.plane.normal).normalize();
    const plane = new Plane().setFromNormalAndCoplanarPoint(normal, vector(row.plane.origin));
    const origin = vector(row.axial.origin);
    if (row.axial.kind === "sphere") {
      const sphere = new Sphere(origin, row.axial.radius);
      const distance = Math.abs(plane.distanceToPoint(origin));
      expect(sphere.intersectsPlane(plane)).toBe(row.fault !== "no intersection wire");
      if (row.fault === "tangent configuration") expect(distance).toBe(sphere.radius);
      if (!row.fault) {
        expect(plane.projectPoint(origin, new Vector3()).distanceTo(vector(row.center!))).toBeLessThan(fixture.tolerance);
        expect(Math.hypot(row.radius!, distance)).toBeCloseTo(sphere.radius);
      }
    } else {
      const axis = vector(row.axial.axis).normalize();
      if (row.fault) {
        expect(axis.cross(normal).length()).toBeGreaterThan(fixture.tolerance);
      } else {
        const center = plane.intersectLine(new Line3(origin.clone().addScaledVector(axis, -100), origin.clone().addScaledVector(axis, 100)), new Vector3())!;
        expect(center.distanceTo(vector(row.center!))).toBeLessThan(fixture.tolerance);
        expect(row.radius).toBe(row.axial.radius);
      }
    }
  }
});

test("Three independently intersects sampled circle and ellipse segments with each plane", () => {
  type Row={curve:{center:number[];axisU:number[];axisV:number[];radii:[number,number]};
    plane:{origin:number[];normal:number[]};points?:number[][];fault?:string};
  const rows=(fixture as typeof fixture & {retainedConicPlane:Row[]}).retainedConicPlane;
  for(const row of rows){
    const plane=new Plane().setFromNormalAndCoplanarPoint(vector(row.plane.normal).normalize(),vector(row.plane.origin));
    const ellipse=new EllipseCurve(0,0,...row.curve.radii,0,2*Math.PI,false,0);
    const center=vector(row.curve.center), u=vector(row.curve.axisU), v=vector(row.curve.axisV);
    const samples=ellipse.getPoints(2048).map(p=>center.clone().addScaledVector(u,p.x).addScaledVector(v,p.y));
    if(row.fault==="infinitely many intersections"){
      expect(samples.every(p=>Math.abs(plane.distanceToPoint(p))<1e-12)).toBe(true);
      continue;
    }
    const intersections:Vector3[]=[];
    for(let i=0;i+1<samples.length;i++){
      const a=samples[i]!,b=samples[i+1]!;
      const hit=plane.intersectLine(new Line3(a,b),new Vector3());
      if(hit&&intersections.every(p=>p.distanceTo(hit)>1e-6))intersections.push(hit);
    }
    const expected=row.points??[];
    expect(intersections.length).toBe(expected.length);
    for(const point of expected)expect(intersections.some(hit=>hit.distanceTo(vector(point))<1e-5)).toBe(true);
  }
});

test("Three verifies the exact sphere section and oblique cylinder rim", () => {
  type Row = { kind:"sphere"|"cylinder"; origin:number[]; radius:number; height?:number; planeOrigin:number[];
    normal:number[]; center:number[]; radii:[number,number]; area:number };
  const rows=(fixture as typeof fixture & {retainedConicSection:Row[]}).retainedConicSection;
  for(const row of rows){
    const solidOrigin=vector(row.origin), center=vector(row.center);
    const plane=new Plane().setFromNormalAndCoplanarPoint(vector(row.normal).normalize(),vector(row.planeOrigin));
    if(row.kind==="sphere"){
      const sphere=new Sphere(solidOrigin,row.radius);
      expect(sphere.intersectsPlane(plane)).toBe(true);
      expect(plane.projectPoint(solidOrigin,new Vector3()).distanceTo(center)).toBeLessThan(1e-12);
      const rim=new EllipseCurve(0,0,...row.radii,0,2*Math.PI,false,0).getPoints(256);
      for(const point of rim)expect(sphere.distanceToPoint(center.clone().add(new Vector3(point.x,point.y,0)))).toBeCloseTo(0,10);
      expect(Math.abs(ShapeUtils.area(rim)-row.area)).toBeLessThan(0.008);
      continue;
    }
    const cylinder=new CylinderGeometry(row.radius,row.radius,row.height!,256,1,true);
    cylinder.rotateX(Math.PI/2);
    cylinder.translate(solidOrigin.x,solidOrigin.y,solidOrigin.z+row.height!/2);
    const points=cylinder.getAttribute("position");
    const rim=[];
    for(let i=0;i<257;i++){
      const a=new Vector3().fromBufferAttribute(points,i), b=new Vector3().fromBufferAttribute(points,i+257);
      const hit=plane.intersectLine(new Line3(a,b),new Vector3())!;
      expect(hit).toBeDefined();
      rim.push(hit.sub(center));
    }
    const x=new Vector3(0,0,1).projectOnPlane(plane.normal).normalize(),y=plane.normal.clone().cross(x);
    expect(Math.max(...rim.map(p=>p.dot(x)))).toBeCloseTo(row.radii[0],5);
    expect(Math.max(...rim.map(p=>p.dot(y)))).toBeCloseTo(row.radii[1],5);
    const area=ShapeUtils.area(rim.map(p=>new Vector2(p.dot(x),p.dot(y))));
    expect(Math.abs(Math.abs(area)-row.area)).toBeLessThan(0.008);
  }
});

test("Three verifies the cap chords, signed winding and area of an axial cylinder section", () => {
  type Row = { origin:number[]; radius:number; height:number; planeOrigin:number[];
    normals:number[][]; corners:number[][]; area:number; refusedPlaneOrigins:number[][] };
  for (const row of (fixture as typeof fixture & { retainedAxialCylinderSection:Row[] }).retainedAxialCylinderSection) {
    const bottom=vector(row.origin), axis=new Vector3(0,0,1), top=bottom.clone().addScaledVector(axis,row.height);
    for (const supplied of row.normals) {
      const plane=new Plane().setFromNormalAndCoplanarPoint(vector(supplied).normalize(),vector(row.planeOrigin));
      const direction=axis.clone().cross(plane.normal).normalize();
      const chord=(center:Vector3,from:number) => {
        const start=plane.projectPoint(center,new Vector3()).addScaledVector(direction,from*100);
        return new Ray(start,direction.clone().multiplyScalar(-from)).intersectSphere(new Sphere(center,row.radius),new Vector3())!;
      };
      const corners=[chord(bottom,-1),chord(bottom,1),chord(top,1),chord(top,-1)];
      for(const expected of row.corners)expect(corners.some(p=>p.distanceTo(vector(expected))<1e-10)).toBe(true);
      const side=corners[1]!.clone().sub(corners[0]!),vertical=corners[3]!.clone().sub(corners[0]!);
      const normal=side.cross(vertical);
      expect(normal.dot(plane.normal)).toBeCloseTo(row.area,10);
      for(const refused of row.refusedPlaneOrigins){
        const miss=new Plane().setFromNormalAndCoplanarPoint(plane.normal,vector(refused));
        expect(Math.abs(miss.distanceToPoint(bottom))).toBeGreaterThanOrEqual(row.radius);
      }
    }
  }
});

test("Three reconstructs cap-clipped elliptic cylinder sections and their signed contours", () => {
  type Row={origin:number[];radius:number;height:number;planeOrigin:number[];normal:number[];capCorners:number[][];area:number};
  for(const row of (fixture as typeof fixture & {retainedClippedCylinderSection:Row[]}).retainedClippedCylinderSection){
    const base=vector(row.origin), axis=new Vector3(0,0,1), normal=vector(row.normal).normalize();
    const plane=new Plane().setFromNormalAndCoplanarPoint(normal,vector(row.planeOrigin));
    const clipped:Vector3[]=[];
    const cylinder=new CylinderGeometry(row.radius,row.radius,row.height,4096,1,true);
    cylinder.rotateX(Math.PI/2);
    cylinder.translate(base.x,base.y,base.z+row.height/2);
    const positions=cylinder.getAttribute("position");
    for(let i=0;i<=4096;i++){
      const a=new Vector3().fromBufferAttribute(positions,i), b=new Vector3().fromBufferAttribute(positions,i+4097);
      const intersection=plane.intersectLine(new Line3(a,b),new Vector3());
      if(intersection)clipped.push(intersection);
    }
    const lateral=axis.clone().cross(normal).normalize();
    for(const z of [base.z,base.z+row.height]){
      const center=base.clone().setZ(z);
      const radial=normal.clone().projectOnPlane(axis);
      const mid=center.clone().addScaledVector(radial,-plane.distanceToPoint(center)/radial.lengthSq());
      for(const sign of [-1,1]){
        const point=new Ray(mid.clone().addScaledVector(lateral,sign*50),lateral.clone().multiplyScalar(-sign)).intersectSphere(new Sphere(center,row.radius),new Vector3());
        if(point)clipped.push(point);
      }
    }
    expect(clipped.length).toBeGreaterThan(100);
    for(const corner of row.capCorners)expect(clipped.some(point=>point.distanceTo(vector(corner))<1e-10)).toBe(true);
    const h=normal.clone().cross(lateral);
    const points=clipped.map(p=>new Vector2(p.dot(lateral),p.dot(h)));
    const center=points.reduce((a,p)=>a.add(p),new Vector2()).multiplyScalar(1/points.length);
    points.sort((a,b)=>Math.atan2(a.y-center.y,a.x-center.x)-Math.atan2(b.y-center.y,b.x-center.x));
    expect(Math.abs(ShapeUtils.area(points)-row.area)).toBeLessThan(0.0001);
  }
});

test("Three recovers bounded conical section contours from exact generator/cap contacts", () => {
  type Row={origin:number[];radii:[number,number];height:number;normal:number[];planeOrigin:number[];
    area:number;capCorners:number[][]};
  const corpus=fixture as typeof fixture & {retainedConicalSections:Row[];retainedParabolicSections:Row[];retainedHyperbolicSections:Row[]};
  for (const row of [...corpus.retainedConicalSections,...corpus.retainedParabolicSections,...corpus.retainedHyperbolicSections]) {
    const base=vector(row.origin), axis=new Vector3(0,0,1), normal=vector(row.normal).normalize();
    const plane=new Plane().setFromNormalAndCoplanarPoint(normal,vector(row.planeOrigin));
    const points:Vector3[]=[];
    const resolution=4096;
    const solid=new CylinderGeometry(row.radii[1],row.radii[0],row.height,resolution,1,true);
    solid.rotateX(Math.PI/2);
    solid.translate(base.x,base.y,base.z+row.height/2);
    const positions=solid.getAttribute("position");
    for(let i=0;i<resolution;i++) {
      const lower=new Vector3().fromBufferAttribute(positions,i+resolution+1);
      const upper=new Vector3().fromBufferAttribute(positions,i);
      const hit=plane.intersectLine(new Line3(lower,upper),new Vector3());
      if(hit)points.push(hit);
    }
    const radial=normal.clone().projectOnPlane(axis);
    if(radial.lengthSq()>0){
      const tangent=axis.clone().cross(normal).normalize();
      for(const i of [0,1]){
        const center=base.clone().addScaledVector(axis,i*row.height);
        const mid=center.clone().addScaledVector(radial,-plane.distanceToPoint(center)/radial.lengthSq());
        for(const sign of [-1,1]){
          const hit=new Ray(mid.clone().addScaledVector(tangent,sign*200),tangent.clone().multiplyScalar(-sign))
            .intersectSphere(new Sphere(center,row.radii[i]!),new Vector3());
          if(hit)points.push(hit);
        }
      }
    }
    expect(points.length).toBeGreaterThan(100);
    for(const corner of row.capCorners)expect(points.some(p=>p.distanceTo(vector(corner))<1e-9)).toBe(true);
    const u=new Vector3(0,1,0),v=normal.clone().cross(u);
    const hull=points.map(p=>new Vector2(p.dot(u),p.dot(v)));
    const center=hull.reduce((a,p)=>a.add(p),new Vector2()).multiplyScalar(1/hull.length);
    hull.sort((a,b)=>Math.atan2(a.y-center.y,a.x-center.x)-Math.atan2(b.y-center.y,b.x-center.x));
    expect(Math.abs(ShapeUtils.area(hull)-row.area)).toBeLessThan(0.001);
  }
});

test("Three verifies signed generator-and-chord sections through the cone apex", () => {
  type Row={kind:string;origin:number[];radii:[number,number];height:number;
    planeOrigin:number[];normal:number[];area:number;points:number[][]};
  for (const row of (fixture as typeof fixture & {retainedApexSections:Row[]}).retainedApexSections) {
    const base=vector(row.origin), axis=new Vector3(0,0,1), requested=vector(row.normal).normalize();
    const plane=new Plane().setFromNormalAndCoplanarPoint(requested,vector(row.planeOrigin));
    const radial=requested.clone().projectOnPlane(axis), lateral=axis.clone().cross(requested).normalize();
    const cap=(index:number)=>{
      const point=base.clone().addScaledVector(axis,row.height*index);
      const radius=row.radii[index]!;
      if(radius===0){
        expect(Math.abs(plane.distanceToPoint(point))).toBeLessThan(1e-12);
        return [point];
      }
      const projection=point.clone().addScaledVector(radial,-plane.distanceToPoint(point)/radial.lengthSq());
      return [-1,1].map(sign=>new Ray(projection.clone().addScaledVector(lateral,sign*100),lateral.clone().multiplyScalar(-sign))
        .intersectSphere(new Sphere(point,radius),new Vector3())!);
    };
    const observed=[...cap(0),...cap(1).reverse()];
    expect(observed.length).toBe(row.points.length);
    observed.forEach((p,i)=>expect(p.distanceTo(vector(row.points[i]!))).toBeLessThan(1e-9));
    const tangent=plane.normal.clone().cross(lateral);
    const area=ShapeUtils.area(observed.map(p=>new Vector2(p.dot(lateral),p.dot(tangent))));
    expect(area).toBeCloseTo(row.area,9);
    const apex=vector(row.planeOrigin);
    for(const pair of [0,observed.length-1]){
      const edge=new Line3(observed[pair]!,apex);
      expect(Math.abs(plane.distanceToPoint(edge.at(0.5,new Vector3())))).toBeLessThan(1e-12);
    }
  }
});
