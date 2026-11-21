import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Mesh,Vector3} from "three";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import fixture from "../🧫️fixtures/🪶️decimate-surface.json";

function vector(point:number[]) {return new Vector3(point[0],point[1],point[2]);}

test("Three verifies the decimated polygon and its three independent midpoint UVs",async()=>{
  let sixVolume=0;
  for (const face of fixture.expected.result.faces) {
    const [a,b,c]=face.map(index=>vector(fixture.expected.result.vertices[index]!));
    sixVolume+=a!.dot(b!.cross(c!));
  }
  expect(sixVolume/6).toBeCloseTo(fixture.expected.volume,7);
  const submitted=fixture.source.surface.filter(row=>row.kind==="vertexNormal").map(row=>vector(row.normal!));
  const expectedNormal=submitted[0]!.add(submitted[4]!).normalize();
  expect(expectedNormal.y).toBeCloseTo(fixture.expected.midpointNormal[1]!,7);
  expect(expectedNormal.z).toBeCloseTo(fixture.expected.midpointNormal[2]!,7);
  const path=process.env.SEMIO_TEST_DECIMATED_SURFACE_GLB;
  if (!path) return;
  const bytes=readFileSync(path);
  const body=bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer;
  const scene=(await new GLTFLoader().parseAsync(body,"")).scene;
  const midpoint=vector(fixture.expected.midpoint);
  const found=new Set<number>();
  scene.traverse(node=>{
    if (!(node instanceof Mesh)) return;
    const geometry=node.geometry;
    const indices=geometry.index!;
    const pos=geometry.getAttribute("position");
    const uv=geometry.getAttribute("uv");
    const normal=geometry.getAttribute("normal");
    for (let row=0;row<indices.count;row+=3) {
      const ids=[0,1,2].map(corner=>indices.getX(row+corner));
      const points=ids.map(id=>new Vector3().fromBufferAttribute(pos,id));
      const moved=points.findIndex(point=>point.equals(midpoint));
      if (moved<0) continue;
      const face=points.every(point=>point.z===0)?0:points.every(point=>point.y===0)?1:2;
      const id=ids[moved]!;
      expect([uv.getX(id),uv.getY(id)]).toEqual(fixture.expected.midpointUv[face]!);
      expect(normal.getX(id)).toBe(0);
      expect(normal.getY(id)).toBeCloseTo(expectedNormal.y,6);
      expect(normal.getZ(id)).toBeCloseTo(expectedNormal.z,6);
      found.add(face);
    }
  });
  expect(found.size).toBe(3);
});
