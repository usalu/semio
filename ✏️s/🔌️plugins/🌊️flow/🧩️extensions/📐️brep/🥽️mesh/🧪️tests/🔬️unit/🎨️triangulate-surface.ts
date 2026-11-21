import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Mesh, Triangle, Vector3} from "three";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import fixture from "../🧫️fixtures/🎨️triangulate-surface.json";

const point = (value:number[]) => new Vector3(value[0],value[1],value[2]);
test("Three triangulates the source and reads retained raw surface channels from native GLB",async () => {
  const [quad,side]=fixture.source.faces;
  const triangle=(a:number,b:number,c:number)=>new Triangle(point(fixture.source.vertices[a]!),point(fixture.source.vertices[b]!),point(fixture.source.vertices[c]!));
  const expectedTriangles=[triangle(quad![0]!,quad![1]!,quad![2]!),triangle(quad![0]!,quad![2]!,quad![3]!),triangle(side![0]!,side![1]!,side![2]!)];
  expect(expectedTriangles.reduce((area,face)=>area+face.getArea(),0)).toBe(fixture.expected.area);
  const sideNormal=expectedTriangles[2]!.getNormal(new Vector3()).toArray().map(value=>value===0?0:value);
  expect(sideNormal).toEqual(fixture.expected.sideFlatNormal);
  const file=process.env.SEMIO_TEST_TRIANGULATED_GLB;
  if (!file) return;
  const bytes=readFileSync(file);
  const buffer=bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer;
  const asset=await new GLTFLoader().parseAsync(buffer,"");
  const parts:Mesh[]=[];
  asset.scene.traverse(object=>{if(object instanceof Mesh)parts.push(object);});
  let count=0;
  let area=0;
  for (const {geometry} of parts) {
    const vertices=geometry.getAttribute("position");
    const normals=geometry.getAttribute("normal");
    const uvs=geometry.getAttribute("uv");
    const indices=geometry.index!;
    for(let corner=0;corner<indices.count;corner+=3) {
      const ids=[0,1,2].map(delta=>indices.getX(corner+delta));
      const actual=ids.map(id=>new Vector3().fromBufferAttribute(vertices,id));
      const fromSide=actual.some(vertex=>vertex.z>0);
      const face=fixture.source.faces[fromSide?1:0]!;
      area+=new Triangle(actual[0],actual[1],actual[2]).getArea();
      count++;
      for(const [local,vertex] of actual.entries()) {
        const original=fixture.source.vertices.findIndex(source=>point(source).equals(vertex));
        const originalCorner=(fromSide?4:0)+face.indexOf(original);
        const uv=fixture.expected.cornerUvs[originalCorner]!;
        const normal=fromSide?sideNormal:fixture.expected.vertexNormals[original]!;
        const index=ids[local]!;
        expect(uvs.getX(index)).toBeCloseTo(uv[0]!,6);
        expect(uvs.getY(index)).toBeCloseTo(uv[1]!,6);
        expect(normals.getX(index)).toBeCloseTo(normal[0]!,6);
        expect(normals.getY(index)).toBeCloseTo(normal[1]!,6);
        expect(normals.getZ(index)).toBeCloseTo(normal[2]!,6);
      }
    }
  }
  expect(count).toBe(fixture.expected.triangleCount);
  expect(area).toBe(fixture.expected.area);
});
