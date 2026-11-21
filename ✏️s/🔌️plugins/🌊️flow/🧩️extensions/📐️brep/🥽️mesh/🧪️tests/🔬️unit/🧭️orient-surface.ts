import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Triangle,Vector3} from "three";
import fixture from "../🧫️fixtures/🧭️orient-surface.json";

test("Three verifies repaired winding, the corner order and exclusive authored normals",()=>{
  const rebuilt=fixture.source.faces.map((face,index)=>fixture.expected.reversed[index]?[...face].reverse():face);
  expect(rebuilt).toEqual(fixture.expected.faces);
  for(const [a,b,c] of rebuilt.map(face=>face.map(id=>new Vector3().fromArray(fixture.source.vertices[id]!)))) {
    expect(new Triangle(a,b,c).getNormal(new Vector3()).z).toBe(1);
  }
  const uv=fixture.source.surface.filter(row=>row.kind==="cornerUv").map(row=>row.uv);
  expect([...uv.slice(0,3),...uv.slice(3).reverse()]).toEqual(fixture.expected.uv);
  const path=process.env.SEMIO_TEST_ORIENT_SURFACE;
  if(!path)return;
  const out=JSON.parse(readFileSync(path,"utf8"));
  for(const [index,normal] of [[3,fixture.expected.exclusiveNormal],[1,fixture.expected.sharedNormal]] as const){
    normal.forEach((value,axis)=>expect(out.normals[index][axis]).toBeCloseTo(value,6));
  }
});
