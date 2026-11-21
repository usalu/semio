import { expect, test } from "bun:test";
import { BoxGeometry, ExtrudeGeometry, Line3, Plane, Quaternion, Shape, SphereGeometry, TorusGeometry, Vector2, Vector3 } from "three";
import { agree, casesOf, fixtureDisagreements, loadFixture, measure } from "../../../⏱️phased-job/🧪️tests/🧰️support/🟦️.ts";

const fixture = loadFixture(import.meta.url);

test("three agrees with the fixture on volume, area, bounds and closedness of every pinned feature mesh", () => {
  expect(fixtureDisagreements(fixture)).toEqual([]);
});

test("every successful feature case states a geometric witness", () => {
  for (const fixtureCase of fixture.cases) {
    if (!fixtureCase.outputs) continue;
    const shape = fixtureCase.outputs.shape as { bbox?: number[][]; area?: number; volume?: number };
    expect(shape.bbox?.flat().every(Number.isFinite) || Number.isFinite(shape.area) || Number.isFinite(shape.volume), fixtureCase.name).toBe(true);
  }
});

test("the stated fillet and chamfer numbers follow the closed forms", () => {
  const edge = 2;
  const radius = 0.25;
  const strip = (1 - Math.PI / 4) * radius * radius * edge;
  const fillet = casesOf(fixture, "brep.feature.filletEdges").find(fixtureCase => fixtureCase.name === "fillet one edge")!;
  const shape = fillet.outputs!.shape as { volume: number; area: number };
  expect(agree(shape.volume, 8 - strip, 1e-9)).toBe(true);
  expect(agree(shape.area, 24 - 2 * radius * edge + (Math.PI / 2) * radius * edge - 2 * (1 - Math.PI / 4) * radius * radius, 1e-9)).toBe(true);
  const chamfer = casesOf(fixture, "brep.feature.chamferEdges")[0]!.outputs!.shape as { volume: number; area: number };
  expect(agree(chamfer.volume, 8 - (0.25 * 0.25) / 2 * edge, 1e-9)).toBe(true);
  expect(agree(chamfer.area, 24 - 2 * 0.25 * edge + Math.SQRT2 * 0.25 * edge - 0.25 * 0.25, 1e-9)).toBe(true);
});

test("Three independently measures the stock, cavity and shrunken box volumes", () => {
  const boxVolume = (width: number, depth: number, height: number): number => {
    const geometry = new BoxGeometry(width, depth, height);
    return measure({ deflection: fixture.tolerance, positions: Array.from(geometry.getAttribute("position").array),
      indices: Array.from(geometry.getIndex()!.array) }).volume;
  };
  const shell = casesOf(fixture, "brep.feature.shell").find(row => row.name === "shell with the top face open")!;
  const shrunk = casesOf(fixture, "brep.feature.offsetSolid").find(row => row.name === "offset shrinks the solid")!;
  expect((shell.outputs!.shape as { volume: number }).volume).toBeCloseTo(boxVolume(2, 2, 2) - boxVolume(1.6, 1.6, 1.8), 5);
  expect((shrunk.outputs!.shape as { volume: number }).volume).toBeCloseTo(boxVolume(1.5, 1.5, 1.5), 5);
});

test("every fault case names a code under the geometry prefix", () => {
  for (const fixtureCase of fixture.cases.filter(item => item.fault)) expect(fixtureCase.fault!.code.startsWith("generation3d.geometry."), fixtureCase.name).toBe(true);
});

const extrudedSection = (ring: [number, number][], depth: number): ReturnType<typeof measure> => {
  const shape = new Shape(ring.map(point => new Vector2(...point)));
  const mesh = new ExtrudeGeometry(shape, { depth, bevelEnabled: false, steps: 1 });
  const position = mesh.getAttribute("position");
  return measure({ deflection: fixture.tolerance,positions: Array.from(position.array), indices: Array.from({ length: position.count }, (_, i) => i) });
};

test("Three extrudes every retained chamfer section into a closed bevel prism", () => {
  type Row = { axis: number; size: [number, number, number]; ring: [number, number][]; area: number; volume: number };
  const rows = (fixture as typeof fixture & { retainedBoxEdgeChamfer: Row[] }).retainedBoxEdgeChamfer;
  for (const row of rows) {
    const measures = extrudedSection(row.ring,row.size[row.axis]);
    expect(measures.area).toBeCloseTo(row.area, 5);
    expect(Math.abs(measures.volume)).toBeCloseTo(row.volume, 5);
    expect(measures.boundaryEdges).toBe(0);
    expect(measures.nonManifoldEdges).toBe(0);
  }
});

test("Three rotates the selected support around the neutral line before extruding the exact draft", () => {
  type XYZ = [number, number, number];
  type Row = { axis: number; origin: XYZ; size: XYZ; normal: XYZ; pullDirection: XYZ; neutralPoint: XYZ; angle: number; ring: [number,number][]; area: number; volume: number };
  const rows = (fixture as typeof fixture & { retainedBoxFaceDraft: Row[] }).retainedBoxFaceDraft;
  for (const row of rows) {
    const normal = new Vector3(...row.normal);
    const pull = new Vector3(...row.pullDirection).normalize();
    const axis = normal.clone().cross(pull).normalize();
    const side = row.normal.findIndex(n => n !== 0);
    const pullAxis = row.pullDirection.findIndex(n => n !== 0);
    const lo = row.origin, hi = row.origin.map((a,i) => a+row.size[i]) as XYZ;
    const u = (row.axis+1)%3, v = (row.axis+2)%3;
    const wall = normal.getComponent(side)>0 ? hi[side] : lo[side];
    const hinge = new Vector3(...lo).setComponent(side,wall).setComponent(pullAxis,row.neutralPoint[pullAxis]);
    const drafted = new Plane().setFromNormalAndCoplanarPoint(normal.clone().applyAxisAngle(axis,row.angle),hinge);
    const ring = [[lo[u],lo[v]],[hi[u],lo[v]],[hi[u],hi[v]],[lo[u],hi[v]]].map(([a,b],i) => {
      let p = new Vector3(...lo).setComponent(u,a).setComponent(v,b);
      if (p.getComponent(side)===wall) {
        const probe = new Line3(p.clone().addScaledVector(normal,-100),p.clone().addScaledVector(normal,100));
        p = drafted.intersectLine(probe,new Vector3())!;
      }
      expect(p.getComponent(u)).toBeCloseTo(row.ring[i][0],10);
      expect(p.getComponent(v)).toBeCloseTo(row.ring[i][1],10);
      return [p.getComponent(u),p.getComponent(v)] as [number,number];
    });
    const result=extrudedSection(ring,row.size[row.axis]);
    expect(result.area).toBeCloseTo(row.area,5);
    expect(Math.abs(result.volume)).toBeCloseTo(row.volume,5);
    expect(result.boundaryEdges).toBe(0);
  }
});

test("Three verifies signed smooth offsets of displaced spheres and tilted ring tori", () => {
  type XYZ = [number,number,number];
  type Row = { kind:"sphere"|"torus"; origin:XYZ; axis:XYZ; angle:number; major:number; minor:number;
    cases:{ distance:number; radius:number; area:number; volume:number }[]; refused:number[] };
  const rows=(fixture as typeof fixture & { retainedSmoothSolidOffset:Row[] }).retainedSmoothSolidOffset;
  for (const row of rows) {
    const center=new Vector3(...row.origin);
    const rotation=new Quaternion().setFromAxisAngle(new Vector3(...row.axis).normalize(),row.angle);
    const axis=new Vector3(0,0,1).applyQuaternion(rotation);
    for (const item of row.cases) {
      const radius=row.minor+item.distance;
      const geometry=row.kind==="sphere" ? new SphereGeometry(radius,192,96) : new TorusGeometry(row.major,radius,96,192);
      geometry.applyQuaternion(rotation);
      geometry.translate(...row.origin);
      const positions=geometry.getAttribute("position");
      for (let i=0;i<positions.count;i+=193) {
        const local=new Vector3().fromBufferAttribute(positions,i).sub(center);
        const axial=local.dot(axis);
        const actual=row.kind==="sphere" ? local.length() : Math.hypot(axial,local.addScaledVector(axis,-axial).length()-row.major);
        expect(actual).toBeCloseTo(item.radius,5);
      }
      const result=measure({deflection:fixture.tolerance,positions:Array.from(positions.array),indices:Array.from(geometry.getIndex()!.array)});
      expect(Math.abs(result.area-item.area)).toBeLessThan(0.08);
      expect(Math.abs(Math.abs(result.volume)-item.volume)).toBeLessThan(0.07);
    }
    for (const distance of row.refused) {
      const radius=row.minor+distance;
      expect(radius<=0||(row.kind==="torus"&&radius>=row.major)).toBe(true);
    }
  }
});


test("Three extrudes the exact quarter-circle fillet profiles on all principal axes", () => {
  type XYZ=[number,number,number];
  type Edge=[XYZ,XYZ];
  type Row={ origin:XYZ; size:XYZ; axis:number; radius:number; area:number; volume:number };
  const {retainedBoxEdgeFillet,retainedMultiBoxFillet}=fixture as typeof fixture & {
    retainedBoxEdgeFillet:(Row & {edge:Edge})[];
    retainedMultiBoxFillet:(Row & {edges:Edge[]})[];
  };
  const rows=[...retainedBoxEdgeFillet.map(row=>({...row,edges:[row.edge]})),...retainedMultiBoxFillet];
  for (const row of rows) {
    const [u,v]=[(row.axis+1)%3,(row.axis+2)%3];
    const [x0,y0]=[row.origin[u],row.origin[v]];
    const [x1,y1]=[x0+row.size[u],y0+row.size[v]];
    const rect=[new Vector2(x0,y0),new Vector2(x1,y0),new Vector2(x1,y1),new Vector2(x0,y1)];
    const picked=new Set(row.edges.map(([point])=>rect.findIndex(p=>p.x===point[u]&&p.y===point[v])));
    const shape=new Shape();
    for(const [corner,point] of rect.entries()){
      const side=(delta:number)=>rect[(corner+delta)%4]!.clone().sub(point).normalize().multiplyScalar(row.radius);
      const left=picked.has(corner)?point.clone().add(side(3)):point;
      if(corner===0)shape.moveTo(left.x,left.y);
      else shape.lineTo(left.x,left.y);
      if(picked.has(corner)){
        const center=left.clone().add(side(1));
        const start=left.clone().sub(center).angle();
        shape.absarc(center.x,center.y,row.radius,start,start+Math.PI/2,false);
      }
    }
    shape.closePath();
    const geometry=new ExtrudeGeometry(shape,{depth:row.size[row.axis],bevelEnabled:false,curveSegments:96});
    const positions=geometry.getAttribute("position");
    const result=measure({deflection:fixture.tolerance,positions:Array.from(positions.array),indices:Array.from({length:positions.count},(_,i)=>i)});
    expect(Math.abs(Math.abs(result.volume)-row.volume)).toBeLessThan(0.005);
    expect(Math.abs(result.area-row.area)).toBeLessThan(0.005);
    expect(result.boundaryEdges).toBe(0);
  }
});
