import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Matrix3,Matrix4,Triangle,Vector3} from "three";
import fixture from "../🧫️fixtures/🪞️mirror-surface.json";

test("Three reflects a mesh half and resolves shared normal and corner bindings",()=>{
  const reflection=new Matrix4().makeScale(-1,1,1);
  const inverseTranspose=new Matrix3().getNormalMatrix(reflection);
  const right=fixture.source.vertices.map(point=>new Vector3().fromArray(point).applyMatrix4(reflection));
  expect(right[1]!.toArray()).toEqual(fixture.expected.vertices[3]!);
  const normal=new Vector3().fromArray(fixture.source.surface[1]!.normal!);
  const offPlane=normal.clone().applyNormalMatrix(inverseTranspose);
  const seam=normal.clone().add(offPlane).normalize();
  for(let axis=0;axis<3;axis++){
    expect(offPlane.getComponent(axis)).toBeCloseTo(fixture.expected.normals[3]![axis]!);
    expect(seam.getComponent(axis)).toBeCloseTo(fixture.expected.normals[2]![axis]!);
  }
  const [a,b,c]=fixture.source.faces[0]!.map(index=>right[index]!);
  expect(new Triangle(c,b,a).getNormal(new Vector3()).toArray()).toEqual([0,0,1]);
  expect(fixture.expected.uv).toEqual([...fixture.expected.uv.slice(0,3),...fixture.expected.uv.slice(0,3).reverse()]);
  const native=process.env.SEMIO_TEST_MIRROR_SURFACE;
  if(!native)return;
  const output=JSON.parse(readFileSync(native,"utf8"));
  const samples=(kind:string)=>output.surface.filter((row:{kind:string})=>row.kind===kind).sort((a:{index:number},b:{index:number})=>a.index-b.index);
  expect(output.vertices).toEqual(fixture.expected.vertices);
  expect(output.faces).toEqual(fixture.expected.faces);
  expect(samples("cornerUv").map((row:{uv:number[]})=>row.uv)).toEqual(fixture.expected.uv);
  expect(samples("smoothFace").map((row:{index:number})=>row.index)).toEqual(fixture.expected.smooth);
  expect(samples("seamEdge").map((row:{index:number})=>row.index)).toEqual(fixture.expected.seams);
  for(const row of samples("vertexNormal"))row.normal.forEach((value:number,axis:number)=>expect(value).toBeCloseTo(fixture.expected.normals[row.index]![axis]!));
});
