import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Mesh,ShapeUtils,Triangle,Vector2,Vector3} from "three";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import cases from "../🧫️fixtures/🎨️subdivide-surface.json";
import raw from "../🧫️fixtures/🎨️triangulate-surface.json";

const vec=(p:number[])=>new Vector3(p[0],p[1],p[2]);
const near=(actual:Vector3,expected:number[])=>expected.forEach((value,axis)=>expect(actual.getComponent(axis)).toBeCloseTo(value,6));
async function draw(path:string) {
  const bytes=readFileSync(path);
  const buffer=bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer;
  const scene=(await new GLTFLoader().parseAsync(buffer,"")).scene;
  const result:{position:Vector3;normal:Vector3;uv:Vector3}[][]=[];
  scene.traverse(node=>{
    if (!(node instanceof Mesh)) return;
    const g=node.geometry;
    const indices=g.index!;
    for (let offset=0;offset<indices.count;offset+=3) {
      result.push([0,1,2].map(delta=>{
        const id=indices.getX(offset+delta);
        const uv=g.getAttribute("uv");
        return {position:new Vector3().fromBufferAttribute(g.getAttribute("position"),id),
          normal:new Vector3().fromBufferAttribute(g.getAttribute("normal"),id),uv:new Vector3(uv.getX(id),uv.getY(id),0)};
      }));
    }
  });
  return result;
}

test("Three verifies subdivided convex center data and a source-face UV seam",async ()=>{
  const [a,b,c,d]=raw.source.vertices.map(vec);
  const center=[a!,b!,c!,d!].reduce((sum,point)=>sum.add(point),new Vector3()).divideScalar(4);
  expect(center.toArray()).toEqual(cases.convex.center);
  const target=process.env.SEMIO_TEST_SUBDIVIDED_CONVEX;
  if (!target) return;
  const triangles=await draw(target);
  expect(triangles).toHaveLength(cases.convex.faceCount);
  let area=0;
  for (const rows of triangles) {
    const [x,y,z]=rows;
    area+=new Triangle(x!.position,y!.position,z!.position).getArea();
    const side=rows.some(row=>row.position.z>0);
    const ring=raw.source.faces[side?1:0]!;
    for (const row of rows) {
      if (row.position.equals(center)) {
        near(row.normal,cases.convex.centerNormal);
        near(row.uv,cases.convex.centerUv);
      }else {
        const vertex=raw.source.vertices.findIndex(point=>vec(point).equals(row.position));
        near(row.normal,side?raw.expected.sideFlatNormal:raw.expected.vertexNormals[vertex]!);
        near(row.uv,raw.expected.cornerUvs[(side?4:0)+ring.indexOf(vertex)]!);
      }
    }
  }
  expect(area).toBeCloseTo(cases.convex.area,6);
});

test("Three verifies concave subdivision and interpolated authored normals and UVs",async ()=>{
  const positions=cases.concave.source.vertices.map(vec);
  const planar=positions.map(point=>new Vector2(point.x,point.y));
  const area=ShapeUtils.triangulateShape(planar,[]).reduce((sum,[a,b,c])=>sum+new Triangle(positions[a!]!,positions[b!]!,positions[c!]!).getArea(),0);
  expect(area).toBeCloseTo(cases.concave.area,6);
  const target=process.env.SEMIO_TEST_SUBDIVIDED_CONCAVE;
  if (!target) return;
  const triangles=await draw(target);
  expect(triangles).toHaveLength(cases.concave.faceCount);
  let drawnArea=0;
  let generated=0;
  for (const rows of triangles) {
    const [a,b,c]=rows;
    drawnArea+=new Triangle(a!.position,b!.position,c!.position).getArea();
    for (const row of rows) {
      const {x,y}=row.position;
      const original=positions.some(point=>point.equals(row.position));
      const normal=new Vector3(x/8,y/8,1);
      if(!original){normal.normalize();generated++;}
      near(row.normal,normal.toArray());
      near(row.uv,[x/4,y/4]);
    }
  }
  expect(generated).toBeGreaterThan(0);
  expect(drawnArea).toBeCloseTo(area,6);
});
