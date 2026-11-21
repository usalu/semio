import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Mesh,Triangle,Vector3} from "three";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import fixture from "../🧫️fixtures/🧲️weld-surface.json";

const point=(p:number[])=>new Vector3(p[0],p[1],p[2]);
test("Three verifies distinct corner UV and normal samples at welded positions",async()=>{
  const source=fixture.source.faces.map(face=>new Triangle(...face.map(id=>point(fixture.source.vertices[id]!)) as [Vector3,Vector3,Vector3]));
  expect(source.reduce((sum,triangle)=>sum+triangle.getArea(),0)).toBe(fixture.expected.area);
  expect(new Set(fixture.source.vertices.map(position=>position.join(","))).size).toBe(fixture.expected.vertices);
  const path=process.env.SEMIO_TEST_WELDED_GLB;
  if(!path)return;
  const bytes=readFileSync(path);
  const buffer=bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer;
  const scene=(await new GLTFLoader().parseAsync(buffer,"")).scene;
  let surfaces=0;
  const vertices=new Set<string>();
  scene.traverse(node=>{
    if(!(node instanceof Mesh))return;
    const geometry=node.geometry;
    const position=geometry.getAttribute("position");
    const uv=geometry.getAttribute("uv");
    const normal=geometry.getAttribute("normal");
    const indices=geometry.index!;
    for(let offset=0;offset<indices.count;offset+=3) {
      const ids=[0,1,2].map(corner=>indices.getX(offset+corner));
      const points=ids.map(id=>new Vector3().fromBufferAttribute(position,id));
      const side=Number(points.some(vertex=>vertex.z>0));
      const ring=fixture.source.faces[side]!;
      for(const [corner,vertex] of points.entries()) {
        vertices.add(vertex.toArray().join(","));
        const local=ring.findIndex(original=>point(fixture.source.vertices[original]!).equals(vertex));
        const expectedUv=fixture.expected.cornerUvs[side*3+local]!;
        expect([uv.getX(ids[corner]!),uv.getY(ids[corner]!)]).toEqual(expectedUv);
        const actual=new Vector3().fromBufferAttribute(normal,ids[corner]!).toArray();
        expect(actual).toEqual(fixture.expected.faceNormals[side]!);
      }
      surfaces++;
    }
  });
  expect(surfaces).toBe(fixture.expected.faces);
  expect(vertices.size).toBe(fixture.expected.vertices);
});
