import { expect, test } from "bun:test";
import { BoxGeometry, Triangle, Vector3 } from "three";
import fixture from "./🔣️original-compound.json";

test("Three.js agrees on each planar compound member and the grouped mesh", () => {
  let area = 0, volume = 0, triangles = 0;
  for (const [x,y,z] of fixture.offsets) {
    const [width,depth,height] = fixture.dimensions;
    const body = new BoxGeometry(width,depth,height);
    body.translate(x+width/2,y+depth/2,z+height/2);
    const points = body.getAttribute("position");
    const index = body.getIndex()!;
    const t = new Triangle();
    for (let i = 0; i < index.count; i+=3) {
      t.a.fromBufferAttribute(points,index.getX(i));
      t.b.fromBufferAttribute(points,index.getX(i+1));
      t.c.fromBufferAttribute(points,index.getX(i+2));
      area += t.getArea();
      volume += t.a.dot(new Vector3().crossVectors(t.b,t.c))/6;
      triangles++;
    }
    body.dispose();
  }
  expect(area).toBeCloseTo(fixture.area, 12);
  expect(volume).toBeCloseTo(fixture.volume, 12);
  expect(triangles).toBe(fixture.triangles);
});
