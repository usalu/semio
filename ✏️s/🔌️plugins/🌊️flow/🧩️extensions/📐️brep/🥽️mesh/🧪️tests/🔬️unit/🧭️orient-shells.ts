import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import loadManifold from "manifold-3d";
import {Triangle, Vector3} from "three";
import fixture from "../🧫️fixtures/🧭️orient-shells.json";

type Mesh = {vertices:number[][];faces:number[][];uv?:number[][];normals?:number[][]};
test("Manifold and Three verify each disconnected shell faces outward", async () => {
  const wasm=await loadManifold();
  wasm.setup();
  const nativePath=process.env.SEMIO_TEST_ORIENT_OUTPUT;
  const native=nativePath ? JSON.parse(readFileSync(nativePath,"utf8")) as Mesh : undefined;
  for (const mesh of [{vertices:fixture.closed.vertices,faces:fixture.outward.faces}, ...(native?[native]:[])]) {
    for (let shell=0;shell<2;shell++) {
      const offset=shell*4;
      const vertices=mesh.vertices.slice(offset,offset+4).map(point=>new Vector3(point[0],point[1],point[2]));
      const center=vertices.reduce((sum,point)=>sum.add(point),new Vector3()).divideScalar(4);
      const faces=mesh.faces.slice(offset,offset+4).map(face=>face.map(id=>id-offset));
      for (const face of faces) {
        const triangle=new Triangle(vertices[face[0]!],vertices[face[1]!],vertices[face[2]!]);
        const away=triangle.getMidpoint(new Vector3()).sub(center);
        expect(triangle.getNormal(new Vector3()).dot(away)).toBeGreaterThan(0);
      }
      const local=vertices.flatMap(point=>point.clone().sub(center).toArray());
      const solid=new wasm.Manifold(new wasm.Mesh({numProp:3,vertProperties:new Float32Array(local),triVerts:new Uint32Array(faces.flat())}));
      expect(solid.volume()).toBeCloseTo(fixture.outward.signedVolumes[shell]!,7);
      solid.delete();
    }
  }
  if (native) {
    const authored=fixture.closed.attributes;
    for (const [face,ring] of native.faces.entries()) {
      const source=fixture.closed.faces[face]!;
      for (const [corner,vertex] of ring.entries()) {
        const index=face*3+corner;
        expect(native.uv?.[index]).toEqual(authored.paint.values[face*3+source.indexOf(vertex)]);
        expect(native.normals?.[index]).toEqual(authored.submittedNormal.values[face]);
      }
    }
  }
});
