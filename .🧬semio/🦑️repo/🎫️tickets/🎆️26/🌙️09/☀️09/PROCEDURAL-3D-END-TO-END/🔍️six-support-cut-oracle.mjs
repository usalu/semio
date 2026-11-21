import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import loadManifold from "manifold-3d";
import { Matrix4, Triangle, Vector3 } from "three";

const source = resolve(process.argv[2]);
const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
const fixture = JSON.parse(readFileSync(source, "utf8"));

function rotation(shape) {
  return new Matrix4().makeScale(shape.mirror ? -1 : 1, 1, 1)
    .multiply(new Matrix4().makeRotationX(shape.angles[1]))
    .multiply(new Matrix4().makeRotationY(shape.angles[0]));
}

function box(shape) {
  const cube = Manifold.cube(shape.size);
  const placed = cube.transform(rotation(shape).toArray());
  const result = placed.translate(shape.origin);
  placed.delete();
  cube.delete();
  return result;
}

function trimMetrics(stockShape, toolShape) {
  const stock = box(stockShape), tool = box(toolShape), cut = stock.subtract(tool);
  const components = cut.decompose();
  const planes = [];
  const rotationMatrix = rotation(toolShape);
  for (let axis = 0; axis < 3; axis++) {
    for (const side of [0, 1]) {
      const normal = new Vector3().setComponent(axis, side ? 1 : -1).transformDirection(rotationMatrix);
      const point = new Vector3().setComponent(axis, side ? toolShape.size[axis] : 0)
        .applyMatrix4(rotationMatrix).add(new Vector3(...toolShape.origin));
      planes.push([normal, point.dot(normal)]);
    }
  }
  const { triVerts, vertProperties, numProp } = cut.getMesh();
  const walls = new Set();
  const vertices = new Set(triVerts);
  for (let step = 0; step < triVerts.length; step += 3) {
    const points = Array.from(triVerts.slice(step, step + 3), id => new Vector3().fromArray(vertProperties, id * numProp));
    planes.forEach(([normal, offset], index) => {
      if (points.every(point => Math.abs(point.dot(normal) - offset) < 1e-7)) walls.add(index);
    });
  }
  const bounds = cut.boundingBox();
  const center = new Vector3();
  const anchor = new Vector3(...bounds.min);
  const point = id => new Vector3().fromArray(vertProperties, id * numProp).sub(anchor);
  let mass = 0;
  for (let step = 0; step < triVerts.length; step += 3) {
    const [a, b, c] = Array.from(triVerts.slice(step, step + 3), point);
    const signed = a.dot(b.clone().cross(c));
    mass += signed;
    center.addScaledVector(a.add(b).add(c), signed / 4);
  }
  center.divideScalar(mass).add(anchor);
  const result = {
    center: center.toArray(),
    volume: cut.volume(), area: cut.surfaceArea(), bounds: [bounds.min, bounds.max],
    meshVertices: vertices.size, components: components.map(component => ({volume: component.volume(), area: component.surfaceArea(), genus: component.genus()})),
    genus: cut.genus(), walls: [...walls].sort((a, b) => a - b)
  };
  for (const item of [...components, cut, tool, stock]) item.delete();
  return result;
}

function interior(material) {
  const bounds=material.boundingBox();
  const {triVerts,vertProperties,numProp}=material.getMesh();
  const triangles=[];
  for (let offset=0;offset<triVerts.length;offset+=3)
    triangles.push(new Triangle(...Array.from(triVerts.slice(offset,offset+3),id=>new Vector3().fromArray(vertProperties,id*numProp))));
  const nearest=new Vector3();
  const fractions=[0.25,0.375,0.5,0.625,0.75];
  let best, clearance=-1;
  for (const x of fractions) for (const y of fractions) for (const z of fractions) {
    const point=new Vector3(...[x,y,z].map((fraction,axis)=>bounds.min[axis]+fraction*(bounds.max[axis]-bounds.min[axis])));
    if (material.rayCast(point.toArray(),[37,43,51]).length%2!==1) continue;
    const distance=Math.min(...triangles.map(triangle=>triangle.closestPointToPoint(point,nearest).distanceTo(point)));
    if(distance>clearance){clearance=distance;best=point.toArray();}
  }
  if(!best||clearance<=1e-5)throw new Error("No known interior sample");
  return best;
}

const selected = fixture.convexCases[1];
const materialStock=box(selected.stock),materialTool=box(selected.tool);
const material=materialStock.subtract(materialTool),gap=materialStock.intersect(materialTool);
const remnants=material.decompose();
const classify=[...remnants.map(solid=>[interior(solid),0]),[interior(gap),1],[Array.from(material.getMesh().vertProperties.slice(0,3)),2]];
const mesh=material.getMesh();
const nearest=[classify[0][0],classify[3][0]].map(probe=>{
  const query=new Vector3(...probe),triangle=new Triangle(),candidate=new Vector3();
  let best, distance=Infinity;
  for(let i=0;i<mesh.triVerts.length;i+=3){
    for(const [offset,point] of [triangle.a,triangle.b,triangle.c].entries())
      point.fromArray(mesh.vertProperties,mesh.triVerts[i+offset]*mesh.numProp);
    const length=triangle.closestPointToPoint(query,candidate).distanceTo(query);
    if(length<distance){distance=length;best=candidate.toArray();}
  }
  return {probe,point:best,distance};
});
for(const shape of [...remnants,gap,material,materialTool,materialStock])shape.delete();
console.log(JSON.stringify({ name: selected.name, ...trimMetrics(selected.stock, selected.tool), classify, nearest }));
for (const height of [2.85, 2.95, 3.05, 3.15, 3.25]) {
  const stock = { origin: [0, 0, 0], size: [4, 4, 4], angles: [0, 0], mirror: false };
  const tool = { origin: [1.3, height, 1.5], size: [1, 1, 1], angles: [.43, -.36], mirror: false };
  console.log(JSON.stringify({ origin: tool.origin, ...trimMetrics(stock, tool) }));
}
