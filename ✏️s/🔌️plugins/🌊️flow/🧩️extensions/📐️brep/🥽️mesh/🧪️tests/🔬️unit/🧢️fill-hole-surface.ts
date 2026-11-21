import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Triangle,Vector3} from "three";
import fixture from "../🧫️fixtures/🧢️fill-hole-surface.json";

test("Three verifies the cap winding, volume and normalized chart",()=>{
  const points=fixture.source.vertices.map(point=>new Vector3().fromArray(point));
  const [a,b,c]=fixture.expected.cap.map(i=>points[i]!);
  const cap=new Triangle(a,b,c);
  expect(cap.getNormal(new Vector3()).z).toBe(-1);
  expect(cap.getArea()).toBe(fixture.expected.capArea);
  const volume=[...fixture.source.faces,fixture.expected.cap].reduce((sum,[a,b,c])=>sum+
    points[a!]!.dot(points[b!]!.clone().cross(points[c!]!))/6,0);
  expect(volume).toBeCloseTo(fixture.expected.volume,8);
  const uv=fixture.expected.cap.map(i=>[points[i]!.y/2,points[i]!.x/2]);
  expect(uv).toEqual(fixture.expected.capUv);
  const [u,v,w]=uv.map(([a,b])=>new Vector3(a,b,0));
  expect(new Triangle(u,v,w).getArea()).toBe(fixture.expected.chartArea);
  const path=process.env.SEMIO_TEST_CAP_UV;
  if(path)expect(JSON.parse(readFileSync(path,"utf8"))).toEqual(uv);
});
