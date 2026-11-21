import { expect, test } from "bun:test";
import { Box3, Float32BufferAttribute, Vector3 } from "three";

const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/painted-bounds.json").json();
for (const row of fixture.cases) {
  test(row.id, () => {
    const box = new Box3();
    for (const mesh of row.meshes) {
      for (const lane of [mesh.positions, mesh.edgePositions]) {
        if (!lane) continue;
        const buffer = new Float32BufferAttribute(lane.map((value: number | null) => value ?? NaN), 3);
        for (let index = 0; index < Math.floor(buffer.count); index++) {
          const point = new Vector3().fromBufferAttribute(buffer, index);
          if (point.toArray().every(Number.isFinite)) box.expandByPoint(point);
        }
      }
    }
    expect(box.isEmpty() ? null : [box.min.toArray(), box.max.toArray()]).toEqual(row.bounds);
  });
}
