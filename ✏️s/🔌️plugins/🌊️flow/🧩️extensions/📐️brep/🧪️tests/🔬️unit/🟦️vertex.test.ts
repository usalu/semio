import { expect, test } from "bun:test";
import { BufferGeometry, Float32BufferAttribute, Vector3 } from "three";
import fixture from "./🔣️original-vertex.json";

for (const row of fixture.cases) {
  test(`standalone vertex geometry at ${row.coordinates.join(", ")}`, () => {
    const point = new Vector3().fromArray(row.coordinates);
    const shape = new BufferGeometry();
    shape.setAttribute("position", new Float32BufferAttribute(point.toArray(), 3));
    shape.computeBoundingBox();
    expect(Array.from(shape.getAttribute("position").array)).toEqual(row.preview.positions);
    expect(Array.from({ length: shape.getAttribute("position").count }, (_, index) => index)).toEqual(row.preview.vertexIds);
    expect(Array.from(shape.getIndex()?.array ?? [])).toEqual(row.preview.indices);
    expect(shape.boundingBox?.min.toArray()).toEqual(row.coordinates);
    expect(shape.boundingBox?.max.toArray()).toEqual(row.coordinates);
    shape.dispose();
  });
}
