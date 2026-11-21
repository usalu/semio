import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {ShapeUtils,Triangle,Vector2,Vector3} from "three";
import fixture from "../🧫️fixtures/🪡️cut-surface.json";

test("Three checks UV interpolation across each side of an authored cut seam",()=>{
  const authored=(index:number)=>fixture.source.surface.find(row=>row.kind==="cornerUv"&&row.index===index)!.uv!;
  const uv=(i:number,j:number)=>new Vector2().fromArray(authored(i)).lerp(new Vector2().fromArray(authored(j)),0.5);
  expect(uv(1,2).toArray()).toEqual(fixture.expected.leftUv);
  expect(uv(4,7).toArray()).toEqual(fixture.expected.rightUv);
  expect(uv(0,1).lerp(uv(2,3),0.5).toArray()).toEqual(fixture.expected.centerUv);
  const path=process.env.SEMIO_TEST_CUT_SURFACE;
  if(!path)return;
  const native=JSON.parse(readFileSync(path,"utf8"));
  const normalize=(normal:number[])=>new Vector3().fromArray(normal).normalize();
  const midpointNormal=normalize([0.3,0,0.9]);
  for(const {id} of fixture.operations) {
    const mesh=native[id];
    const vertex=mesh.positions.findIndex((point:number[])=>point.every((value,i)=>value===fixture.expected.edgePoint[i]));
    const outputNormal=mesh.normals[vertex];
    for(let axis=0;axis<3;axis++)expect(outputNormal[axis]).toBeCloseTo(midpointNormal.getComponent(axis));
    const uvSamples:number[][]=[];
    let totalArea=0;
    for(const [index,face] of mesh.faces.entries()) {
      const points=face.map((vertex:number)=>mesh.positions[vertex]);
      const outline=points.map((point:number[])=>new Vector2(point[0],point[1]));
      totalArea+=ShapeUtils.triangulateShape(outline,[]).reduce((sum,[a,b,c])=>sum+new Triangle(
        new Vector3().fromArray(points[a!]),new Vector3().fromArray(points[b!]),new Vector3().fromArray(points[c!]),
      ).getArea(),0);
      const left=points.some((point:number[])=>point[0]!<2);
      expect(mesh.smooth[index]).toBe(left);
      for(const [corner,point] of face.entries())if(point===vertex)uvSamples.push(mesh.uv[index][corner]);
    }
    expect(totalArea).toBe(fixture.expected.area);
    expect(uvSamples).toContainEqual(fixture.expected.leftUv);
    expect(uvSamples).toContainEqual(fixture.expected.rightUv);
    expect(mesh.seams.length).toBe(fixture.expected.seamSegments);
    if(id==="loopCut") {
      const center=mesh.positions.findIndex((point:number[])=>point.every((value,i)=>value===fixture.expected.center[i]));
      for(const [i,face] of mesh.faces.entries())if(face.includes(center))expect(mesh.uv[i][face.indexOf(center)]).toEqual(fixture.expected.centerUv);
    }
  }
});
