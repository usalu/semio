import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {ShapeUtils,Triangle,Vector2,Vector3} from "three";
import fixture from "../🧫️fixtures/🎈️expanded-surface.json";

const triangulate=(uv:number[][])=>ShapeUtils.triangulateShape(uv.map(point=>new Vector2().fromArray(point)),[]);
const area=(points:number[][],uv:number[][])=>triangulate(uv).reduce((total,[a,b,c])=>total+new Triangle(
  new Vector3().fromArray(points[a!]!),new Vector3().fromArray(points[b!]!),new Vector3().fromArray(points[c!]!),
).getArea(),0);

test("Three verifies planar cap and positive-area UVs after Extrude and Inset",()=>{
  const path=process.env.SEMIO_TEST_EXPANDED_SURFACE;
  const native=path?JSON.parse(readFileSync(path,"utf8")):undefined;
  for (const operation of fixture.operations) {
    const boundary=fixture.source.vertices;
    const sides=boundary.map((point,index)=>[point,boundary[(index+1)%4]!,operation.cap[(index+1)%4]!,operation.cap[index]!]);
    expect(area(operation.cap,fixture.capUv)+sides.reduce((sum,side)=>sum+area(side,fixture.quadUv),0)).toBe(operation.area);
    expect(area(fixture.quadUv.map(uv=>[...uv,0]),fixture.quadUv)).toBe(1);
    if(!native)continue;
    const output=native[operation.id];
    expect(output.cap).toEqual(operation.cap);
    expect(output.uv[0]).toEqual(fixture.capUv);
    expect(output.uv.slice(1)).toEqual(sides.map(()=>fixture.quadUv));
    expect(output.smooth).toEqual(operation.smooth);
    expect(area(output.cap,output.uv[0])+output.sides.reduce((sum:number,side:number[][],i:number)=>sum+area(side,output.uv[i+1]),0)).toBe(operation.area);
    expect(output.seams).toContain(0);
    expect(output.seams.length).toBeGreaterThan(2);
  }
});
