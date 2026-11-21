import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Triangle,Vector3} from "three";
import fixture from "../🧫️fixtures/🔃️face-flip.json";

const near=(actual:number[],expected:number[])=>actual.forEach((value,axis)=>expect(value).toBeCloseTo(expected[axis]!,6));

test("Three verifies selected winding and the original shared-edge identity",()=>{
  const directed=fixture.source.faces.flatMap(face=>face.map((a,i)=>[a,face[(i+1)%face.length]!]));
  const twins=directed.flatMap(([a,b],i)=>{
    const twin=directed.findIndex(([x,y])=>x===b&&y===a);
    return twin>=0?[[i,twin]]:[];
  });
  expect(twins).toEqual(fixture.expected.twins);
  const normals=fixture.source.faces.map((face,index)=>{
    const [a,b,c]=(fixture.selection.includes(index)?[...face].reverse():face).map(i=>new Vector3().fromArray(fixture.source.vertices[i]!));
    return new Triangle(a,b,c).getNormal(new Vector3()).toArray();
  });
  normals.forEach((normal,index)=>near(normal,fixture.expected.effectiveNormals[index]!));
  const path=process.env.SEMIO_TEST_FACE_FLIP;
  if(!path)return;
  const out=JSON.parse(readFileSync(path,"utf8"));
  expect(out.flipped).toEqual(fixture.expected.flipped);
  expect(out.twins).toEqual(twins);
  expect(out.uv).toEqual(fixture.source.surface.filter(row=>row.kind==="cornerUv").map(row=>row.uv));
  expect(out.smooth).toEqual([true,false]);
  expect(out.seams).toEqual([1]);
  near(out.normals[0],fixture.expected.normalAtExclusiveVertex);
  near(out.normals[1],fixture.expected.normalAtSharedVertex);
});
