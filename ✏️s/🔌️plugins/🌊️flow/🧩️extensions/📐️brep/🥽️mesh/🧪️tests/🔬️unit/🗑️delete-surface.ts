import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {BufferGeometry,Float32BufferAttribute,ShapeUtils,Triangle,Vector2,Vector3} from "three";
import fixture from "../🧫️fixtures/🗑️delete-surface.json";

test("Three verifies the surviving mesh groups, seams and authored samples",()=>{
  const independent=fixture.source.faces.filter((_,face)=>!fixture.remove.includes(face));
  const geometry=new BufferGeometry();
  geometry.setAttribute("position",new Float32BufferAttribute(fixture.source.vertices.flat(),3));
  const point=(index:number)=>new Vector3().fromBufferAttribute(geometry.getAttribute("position"),index);
  let area=0;
  for(const face of independent) {
    const points=face.map(point);
    const triangles=ShapeUtils.triangulateShape(points.map(p=>new Vector2(p.x,p.y)),[]);
    for(const [a,b,c] of triangles)area+=new Triangle(points[a!]!,points[b!]!,points[c!]!).getArea();
  }
  expect(area).toBe(fixture.expected.area);
  const edges=(faces:number[][])=>faces.flatMap(face=>face.map((a,index)=>{
    const b=face[(index+1)%face.length]!;
    return Math.min(a,b)+","+Math.max(a,b);
  }));
  const originalEdges=edges(fixture.source.faces);
  const authoredSeams=new Set(fixture.source.surface.filter(row=>row.kind==="seamEdge").map(row=>originalEdges[row.index]));
  const seams=edges(independent).flatMap((key,index)=>authoredSeams.has(key)?[index]:[]);
  expect(seams).toEqual(fixture.expected.seams);
  const path=process.env.SEMIO_TEST_DELETE_SURFACE;
  if(!path)return;
  const out=JSON.parse(readFileSync(path,"utf8"));
  expect(out.vertices).toEqual(fixture.expected.vertices);
  expect(out.faces).toEqual(fixture.expected.faces);
  expect(out.uv).toEqual(fixture.expected.uv);
  expect(out.smooth).toEqual(fixture.expected.smooth);
  expect(out.seams).toEqual(seams);
  expect(out.panels).toEqual(fixture.expected.panels);
  for(const [index,normal] of fixture.expected.normals as [number,number[]][]) {
    expect(out.normals[index]).toBeDefined();
    normal.forEach((value,axis)=>expect(out.normals[index][axis]).toBeCloseTo(value,6));
  }
});
