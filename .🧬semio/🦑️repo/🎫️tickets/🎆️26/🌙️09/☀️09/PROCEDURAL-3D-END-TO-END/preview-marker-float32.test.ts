import { expect, test } from "bun:test";
import { Float32BufferAttribute, Vector3 } from "three";

const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/marker-float32.json").json();

for (const row of fixture.cases) {
  test(row.id, () => {
    const vertex = new Float32BufferAttribute(row.value, 3);
    const anchor = new Vector3().fromBufferAttribute(vertex, 0);
    const finite = anchor.toArray().every(Number.isFinite);
    const zero = anchor.equals(new Vector3());
    const marker = !finite ? [0, 0] : row.kind === "plane" ? [10, 9] :
      row.kind === "vector" && !zero ? [6, 5] : [1, 3];
    expect(marker).toEqual([row.positions, row.segments]);
  });
}
