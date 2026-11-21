import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import { Matrix4, Triangle, Vector3 } from "three";
import fixture from "./🔣️original-skew-box-intersection.json";
import difference from "./🔣️original-skew-box-difference.json";

const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
type Vector = [number, number, number];
type Matrix = Parameters<ReturnType<typeof Manifold.cube>["transform"]>[0];
type Placement = typeof fixture.cases[number]["stock"];

function placed(stock: Placement) {
  const box = Manifold.cube(stock.size as Vector);
  const tilt = box.rotate([0, stock.angles[0] * 180 / Math.PI, 0]);
  const pitch = tilt.rotate([stock.angles[1] * 180 / Math.PI, 0, 0]);
  const reflect = pitch.transform(new Matrix4().makeScale(stock.mirror ? -1 : 1, 1, 1).toArray() as Matrix);
  const result = reflect.translate(stock.origin as Vector);
  for (const item of [reflect, pitch, tilt, box]) item.delete();
  return result;
}

for (const row of fixture.cases) {
  test("Manifold intersects skew boxes when " + row.name, () => {
    const stock = placed(row.stock);
    const tool = placed(row.tool);
    const result = stock.intersect(tool);
    expect(result.volume()).toBeCloseTo(row.volume, 9);
    expect(result.surfaceArea()).toBeCloseTo(row.area, 9);
    expect(result.genus()).toBe(0);
    const bounds = result.boundingBox();
    for (const [side, actual] of [bounds.min, bounds.max].entries()) {
      for (const [axis, point] of actual.entries()) expect(point).toBeCloseTo(row.bounds[side][axis], 8);
    }
    const mesh = result.getMesh();
    const capOrigin = new Vector3().fromArray(row.capOrigin);
    const capNormal = new Vector3().fromArray(row.capNormal);
    const cap = new Set<number>();
    const vertex = (n: number) => new Vector3().fromArray(mesh.vertProperties, n * mesh.numProp);
    for (let i = 0; i < mesh.triVerts.length; i += 3) {
      const indices = Array.from(mesh.triVerts.slice(i, i + 3));
      if (indices.every(n => Math.abs(vertex(n).sub(capOrigin).dot(capNormal)) < 1e-6)) {
        for (const n of indices) cap.add(n);
      }
    }
    expect(cap.size).toBe(row.capVertices);
    const triangle = new Triangle();
    for (const [coords, expected] of row.classify) {
      const point = new Vector3().fromArray(coords as Vector);
      let boundary = false;
      for (let i = 0; i < mesh.triVerts.length; i += 3) {
        triangle.a.copy(vertex(mesh.triVerts[i]));
        triangle.b.copy(vertex(mesh.triVerts[i + 1]));
        triangle.c.copy(vertex(mesh.triVerts[i + 2]));
        if (triangle.closestPointToPoint(point, new Vector3()).distanceTo(point) < 1e-6) {
          boundary = true;
          break;
        }
      }
      const classification = boundary ? 2 : result.rayCast(point.toArray(), [35, 40, 45]).length % 2 ? 0 : 1;
      expect(classification).toBe(expected as number);
    }
    for (const item of [result, tool, stock]) item.delete();
  });
}

function stockSupports(spec: Placement) {
  const x = new Vector3(1, 0, 0);
  const y = new Vector3(0, 1, 0);
  const origin = new Vector3().fromArray(spec.origin);
  return [0, 1, 2].flatMap(axis => {
    const normal = new Vector3().setComponent(axis, 1)
      .applyAxisAngle(y, spec.angles[0]).applyAxisAngle(x, spec.angles[1]);
    if (spec.mirror) normal.x = -normal.x;
    return [[origin, normal.clone().negate()], [origin.clone().addScaledVector(normal, spec.size[axis]), normal]];
  });
}

for (const row of fixture.convexCases) {
  test("Manifold checks convex skew topology for " + row.name, () => {
    const stock = placed(row.stock);
    const tool = placed(row.tool);
    const result = stock.intersect(tool);
    expect(result.volume()).toBeCloseTo(row.volume, 9);
    expect(result.surfaceArea()).toBeCloseTo(row.area, 9);
    expect(result.genus()).toBe(0);
    const bounds = result.boundingBox();
    for (const [side, actual] of [bounds.min, bounds.max].entries()) {
      for (const [axis, point] of actual.entries()) expect(point).toBeCloseTo(row.bounds[side][axis], 8);
    }
    const mesh = result.getMesh();
    const unique = new Map<string, Vector3>();
    for (const id of mesh.triVerts) {
      const point = new Vector3().fromArray(mesh.vertProperties, id * mesh.numProp);
      unique.set(point.toArray().map(value => value.toFixed(8)).join(","), point);
    }
    expect(unique.size).toBe(row.vertices);
    const points = Array.from(unique.values());
    const supports = [...stockSupports(row.stock), ...stockSupports(row.tool)];
    const active = supports.filter(([origin, normal]) =>
      points.filter(point => Math.abs(point.clone().sub(origin).dot(normal)) < 1e-6).length >= 3);
    expect(active.length).toBe(row.faces);
    expect(row.section.normal).toEqual([0, 0, 1]);
    const section = result.slice(row.section.origin[2]);
    expect(section.toPolygons().length).toBe(1);
    expect(section.area()).toBeCloseTo(row.section.area, 8);
    section.delete();
    expect(unique.size + active.length - 2).toBe(row.edges);
    for (const item of [result, tool, stock]) item.delete();
  });
}

for (const row of fixture.empty) {
  test("Manifold gives empty material for " + row.name, () => {
    const stock = placed(row.stock);
    const tool = placed(row.tool);
    const intersection = stock.intersect(tool);
    expect(Math.abs(intersection.volume())).toBeLessThan(1e-9);
    for (const item of [intersection, tool, stock]) item.delete();
  });
}

for (const row of fixture.unsupported) {
  test("Manifold distinguishes " + row.name + " from separation", () => {
    const stock = placed(row.stock);
    const tool = placed(row.tool);
    const intersection = stock.intersect(tool);
    expect(intersection.volume()).toBeGreaterThan(0);
    expect(intersection.volume()).toBeLessThan(1e-15);
    for (const item of [intersection, tool, stock]) item.delete();
  });
}

function cutOperands(row: { reference?: (string | number)[]; stock?: Placement; tool?: Placement; reverse?: boolean }) {
  const [family, index] = row.reference ?? [];
  const referenced = family === "cases" ? (row.reverse ? difference.cases : fixture.cases)
    : family === "empty" ? fixture.empty : fixture.convexCases;
  const resolved = typeof index === "number" ? referenced[index] : row;
  const operands = [(resolved as typeof row).stock!, (resolved as typeof row).tool!];
  if (row.reverse) operands.reverse();
  return operands.map(placed);
}

function manifoldCenter(result: ReturnType<typeof Manifold.cube>): [number,number,number] {
  const {triVerts,vertProperties,numProp}=result.getMesh();
  const anchor=new Vector3(...result.boundingBox().min);
  const moment=new Vector3();
  const point=(id:number)=>new Vector3().fromArray(vertProperties,id*numProp).sub(anchor);
  let signedVolume=0;
  for (let index=0;index<triVerts.length;index+=3) {
    const [a,b,c]=Array.from(triVerts.slice(index,index+3),point);
    const tetrahedron=a.dot(b.clone().cross(c));
    signedVolume+=tetrahedron;
    moment.addScaledVector(a.add(b).add(c),tetrahedron/4);
  }
  return moment.divideScalar(signedVolume).add(anchor).toArray();
}

function checkClassification(result: ReturnType<typeof Manifold.cube>, probes: (number | number[])[][]) {
  const { triVerts, vertProperties, numProp } = result.getMesh();
  const triangle = new Triangle();
  const vertex = (id: number, target: Vector3) => target.fromArray(vertProperties, id * numProp);
  const closest = new Vector3();
  for (const [coordinates, classification] of probes) {
    const point = new Vector3().fromArray(coordinates as Vector);
    let distance = Number.POSITIVE_INFINITY;
    for (let index = 0; index < triVerts.length; index += 3) {
      vertex(triVerts[index], triangle.a);
      vertex(triVerts[index + 1], triangle.b);
      vertex(triVerts[index + 2], triangle.c);
      distance = Math.min(distance, triangle.closestPointToPoint(point, closest).distanceTo(point));
    }
    const actual = distance <= 1e-6 ? 2 : result.rayCast(point.toArray(), [37, 43, 51]).length % 2 === 1 ? 0 : 1;
    expect(actual).toBe(classification as number);
  }
}

for (const row of difference.cases) {
  test("Manifold subtracts skew boxes for " + row.name, () => {
    const [stock, tool] = cutOperands(row);
    const result = stock.subtract(tool);
    expect(result.volume()).toBeCloseTo(row.volume, 9);
    expect(result.surfaceArea()).toBeCloseTo(row.area, 9);
    expect(2 - 2 * result.genus()).toBe(row.euler);
    if (row.center) manifoldCenter(result).forEach((coordinate,axis)=>expect(coordinate).toBeCloseTo(row.center![axis],7));
    const bounds = result.boundingBox();
    for (const [side, values] of [bounds.min, bounds.max].entries()) {
      for (const [axis, value] of values.entries()) expect(value).toBeCloseTo(row.bounds[side][axis], 8);
    }
    if (row.classify) checkClassification(result,row.classify);
    if (row.kind === "cavity") {
      const fact = row.section!;
      const above = result.slice(fact.origin[2]);
      const through = result.slice(row.voidSection!.origin[2]);
      expect(above.toPolygons()).toHaveLength(1);
      expect(above.area()).toBeCloseTo(fact.area, 8);
      const rings = through.toPolygons();
      expect(rings).toHaveLength(2);
      expect(through.area()).toBeCloseTo(row.voidSection!.area, 8);
      const areas = rings.map(ring=>ring.reduce((sum,[x,y],i)=>{
        const [u,v]=ring[(i+1)%ring.length];
        return sum+x*v-y*u;
      },0)/2).sort((a,b)=>b-a);
      areas.forEach((area,i)=>expect(area).toBeCloseTo(row.voidSection!.ringAreas[i],8));
      through.delete();
      above.delete();
    }
    if (row.connectedVoidSection) {
      const fact=row.connectedVoidSection;
      const axis=fact.normal.indexOf(1);
      const turn:Vector=axis===0?[0,-90,0]:axis===1?[90,0,0]:[0,0,0];
      const oriented=result.rotate(turn);
      const crossSection=oriented.slice(fact.origin[axis]);
      const rings=crossSection.toPolygons();
      expect(rings).toHaveLength(2);
      expect(crossSection.area()).toBeCloseTo(fact.area,8);
      const areas=rings.map(ring=>ring.reduce((sum,[x,y],i)=>{
        const [u,v]=ring[(i+1)%ring.length];
        return sum+x*v-y*u;
      },0)/2).sort((a,b)=>b-a);
      for (const [i,area] of areas.entries()) expect(area).toBeCloseTo(fact.ringAreas[i],8);
      crossSection.delete();
      oriented.delete();
    }
    if (row.kind === "recess") {
      const planes = stockSupports(row.tool!);
      const { triVerts, vertProperties, numProp } = result.getMesh();
      const walls = new Set<number>();
      const point = (id: number) => new Vector3().fromArray(vertProperties, id * numProp);
      for (let index = 0; index < triVerts.length; index += 3) {
        const [a, b, c] = Array.from(triVerts.slice(index, index + 3), point);
        const normal = b.clone().sub(a).cross(c.clone().sub(a)).normalize();
        planes.forEach(([origin, outside], plane) => {
          if ([a, b, c].every(vertex => Math.abs(vertex.clone().sub(origin).dot(outside)) < 1e-6)) {
            expect(normal.dot(outside)).toBeCloseTo(-1, 8);
            walls.add(plane);
          }
        });
      }
      expect([...walls].sort((a,b)=>a-b)).toEqual(row.cutterFaces??[0,2]);
      if (row.coincidentFloor) {
        const fact = row.coincidentFloor;
        const origin = new Vector3(...fact.origin as Vector);
        const normal = new Vector3(...fact.normal as Vector);
        const x = new Vector3(1, 0, 0);
        const y = normal.clone().cross(x);
        const oriented = result.transform(new Matrix4().makeBasis(x, y, normal).invert().toArray() as Matrix);
        const height = origin.dot(normal);
        const below = oriented.slice(height - 1e-5);
        const above = oriented.slice(height + 1e-5);
        let floorArea = 0;
        for (let index = 0; index < triVerts.length; index += 3) {
          const [a, b, c] = Array.from(triVerts.slice(index, index + 3), point);
          if ([a, b, c].every(vertex => Math.abs(vertex.clone().sub(origin).dot(normal)) < 1e-8)) {
            floorArea += new Triangle(a, b, c).getArea();
          }
        }
        expect(above.toPolygons()).toHaveLength(2);
        expect(above.area()).toBeCloseTo(fact.aboveArea, 8);
        expect(floorArea).toBeCloseTo(fact.floorArea, 6);
        expect(above.area() + floorArea).toBeCloseTo(fact.area, 6);
        expect(below.toPolygons()).toHaveLength(1);
        expect(below.area()).toBeCloseTo(fact.area, 8);
        for (const vertex of fact.vertices) {
          const world = new Vector3(...vertex as Vector);
          expect(below.toPolygons()[0].some(([u, v]) => Math.hypot(u - world.dot(x), v - world.dot(y)) < 1e-8)).toBe(true);
        }
        for (const item of [below, above, oriented]) item.delete();
      }
      for (const fact of [row.vertexSection,row.edgeSection]) {
        if (!fact) continue;
        const z=new Vector3(...fact.normal as Vector).normalize();
        const x=new Vector3(0,0,1).cross(z).normalize();
        const y=z.clone().cross(x);
        const local=result.transform(new Matrix4().makeBasis(x,y,z).invert().toArray() as Matrix);
        const section=local.slice(new Vector3(...fact.origin as Vector).dot(z));
        const polygons=section.toPolygons();
        expect(polygons).toHaveLength(1);
        expect(section.area()).toBeCloseTo(fact.area,8);
        expect(polygons[0].some(([x,y])=>Math.hypot(x,y)<1e-7)).toBe(true);
        section.delete();
        local.delete();
      }
      if (row.section) {
        const fact = row.section;
        expect(fact.normal).toEqual([0, 0, 1]);
        const section = result.slice(fact.origin[2]);
        const polygons = section.toPolygons();
        expect(polygons).toHaveLength(1);
        expect(section.area()).toBeCloseTo(fact.area, 8);
        const [ring] = polygons;
        expect(ring.some(([x,y], index) => {
          const [u,v] = ring[(index + 1) % ring.length];
          const [p,q] = ring[(index + 2) % ring.length];
          return (u - x)*(q - v) - (v - y)*(p - u) < -1e-6;
        })).toBe(true);
        section.delete();
      }
    }
    if (row.vertices !== undefined) {
      const { triVerts, vertProperties, numProp } = result.getMesh();
      const distinct = new Set(Array.from(triVerts, id =>
        new Vector3().fromArray(vertProperties, id * numProp).toArray().map(value => value.toFixed(8)).join(",")));
      expect(distinct.size).toBe(row.vertices);
    }
    for (const item of [result, tool, stock]) item.delete();
  });
}

for (const row of difference.compounds) {
  test("Manifold returns all remaining components for " + row.name, () => {
    const [stock, tool] = cutOperands(row);
    const result = stock.subtract(tool);
    const components = result.decompose().sort((a, b) => a.volume() - b.volume());
    expect(result.volume()).toBeCloseTo(row.volume, 9);
    expect(result.surfaceArea()).toBeCloseTo(row.area, 9);
    expect(components).toHaveLength(row.components.length);
    const bounds = result.boundingBox();
    for (const [side, axes] of [bounds.min, bounds.max].entries()) {
      axes.forEach((coordinate, axis) => expect(coordinate).toBeCloseTo(row.bounds[side][axis], 9));
    }
    manifoldCenter(result).forEach((coordinate,axis)=>expect(coordinate).toBeCloseTo(row.center[axis],7));
    checkClassification(result,row.classify);
    const {triVerts,vertProperties,numProp}=result.getMesh();
    const triangle=new Triangle(), point=new Vector3();
    for (const probe of row.nearest) {
      const query=new Vector3(...probe.probe as Vector);
      let distance=Infinity, closest: number[] | undefined;
      for (let index=0;index<triVerts.length;index+=3) {
        for (const [offset,vertex] of [triangle.a,triangle.b,triangle.c].entries())
          vertex.fromArray(vertProperties,triVerts[index+offset]*numProp);
        const length=triangle.closestPointToPoint(query,point).distanceTo(query);
        if(length<distance){distance=length;closest=point.toArray();}
      }
      expect(distance).toBeCloseTo(probe.distance,7);
      closest!.forEach((coordinate,axis)=>expect(coordinate).toBeCloseTo(probe.point[axis],7));
    }
    for (const [index, component] of components.entries()) {
      const expected = row.components[index];
      expect(component.volume()).toBeCloseTo(expected.volume, 9);
      expect(component.surfaceArea()).toBeCloseTo(expected.area, 9);
      expect(2 - 2 * component.genus()).toBe(expected.euler);
      if (expected.euler === 0) {
        const section = component.slice(row.handledSection.origin[2]);
        expect(section.toPolygons()).toHaveLength(1);
        expect(section.area()).toBeCloseTo(row.handledSection.area, 8);
        section.delete();
      }
    }
    for (const item of [...components, result, tool, stock]) item.delete();
  });
}

for (const row of difference.refused) {
  test("Manifold distinguishes the rejected Cut case " + row.name, () => {
    const [stock, tool] = cutOperands(row);
    const result = stock.subtract(tool);
    if (row.kind === "empty") expect(result.volume()).toBeCloseTo(0, 10);
    else {
      expect(result.volume()).toBeGreaterThan(0);
      expect(result.volume()).toBeLessThan(stock.volume());
    }
    for (const item of [result, tool, stock]) item.delete();
  });
}

test("Manifold keeps the exact independent volumes and areas in separated skew Union", () => {
  const row = fixture.empty[fixture.separatedUnion.reference];
  const stock = placed(row.stock);
  const tool = placed(row.tool);
  const result = Manifold.union([stock, tool]);
  const members = result.decompose().sort((a, b) => b.volume() - a.volume());
  expect(members.length).toBe(fixture.separatedUnion.components);
  members.forEach((member, index) => {
    expect(member.volume()).toBeCloseTo(fixture.separatedUnion.volumes[index], 9);
    expect(member.surfaceArea()).toBeCloseTo(fixture.separatedUnion.areas[index], 9);
    member.delete();
  });
  for (const item of [result,tool,stock]) item.delete();
});

for (const row of fixture.overlappingUnion) {
  test("Manifold sews "+row.name+" in both orders", () => {
    const stock = placed(row.stock);
    const tool = placed(row.tool);
    try {
      for (const [first,second] of [[stock,tool],[tool,stock]]) {
        const result = first.add(second);
        const shells = result.decompose();
        try {
          expect(shells).toHaveLength(1);
          expect(result.genus()).toBe(0);
          expect(result.volume()).toBeCloseTo(row.volume, 9);
          expect(result.surfaceArea()).toBeCloseTo(row.area, 9);
          const bounds = result.boundingBox();
          for (const [side, values] of [bounds.min,bounds.max].entries()) {
            for (const [axis,value] of values.entries()) expect(value).toBeCloseTo(row.bounds[side][axis], 8);
          }
        } finally {
          for (const shape of [...shells,result]) shape.delete();
        }
      }
    } finally {
      tool.delete();
      stock.delete();
    }
  });
}
