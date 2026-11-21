import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {ShapeUtils,Triangle,Vector2,Vector3} from "three";
import fixture from "../🧫️fixtures/🧷️coplanar-surface.json";

const area=(points:number[][],faces:number[][])=>faces.reduce((total,face)=>{
  const position=face.map(index=>new Vector3().fromArray(points[index]!));
  const triangles=ShapeUtils.triangulateShape(position.map(p=>new Vector2(p.x,p.y)),[]);
  return total+triangles.reduce((sum,[a,b,c])=>sum+new Triangle(position[a!]!,position[b!]!,position[c!]!).getArea(),0);
},0);

test("Three verifies coplanar boundaries and authored UV compatibility",()=>{
  expect(area(fixture.source.vertices,fixture.source.faces)).toBe(fixture.expected.area);
  const uv=(index:number)=>fixture.source.surface.find(row=>row.kind==="cornerUv"&&row.index===index)!.uv!;
  expect(uv(1)).toEqual(uv(4));
  expect(uv(2)).toEqual(uv(7));
  expect(uv(5)).not.toEqual(uv(8));
  expect(uv(6)).not.toEqual(uv(11));
  const path=process.env.SEMIO_TEST_COPLANAR_SURFACE;
  if(!path)return;
  const output=JSON.parse(readFileSync(path,"utf8"));
  expect(output.faces.length).toBe(fixture.expected.faceCount);
  expect(output.faces.map((face:number[])=>face.length)).toEqual(fixture.expected.cornerCounts);
  expect(area(output.positions,output.faces)).toBe(fixture.expected.area);
  expect(output.smooth).toEqual([true,true]);
  expect(output.seams).toEqual([fixture.expected.markedEdge]);
  expect(output.faces.map((face:number[],i:number)=>output.uv[i][face.indexOf(fixture.expected.uvVertex)])).toEqual(fixture.expected.uvSamples);
  for(const {index,normal} of fixture.source.surface.filter(row=>row.kind==="vertexNormal")) {
    normal!.forEach((value,axis)=>expect(output.normals[index][axis]).toBeCloseTo(value,6));
  }
});
