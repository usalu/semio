import { test, expect } from "bun:test";
import { BufferGeometry, Float32BufferAttribute, Vector3 } from "three";

const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/spatial-preview-variables.json").json();
const native = await Bun.file(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/spatial-preview-variables-native.json").json();
test("spatial variable channels retain their authored marker positions", () => {
  expect(native.map(row => row.id)).toEqual(fixture.expected);
  for (const row of native) {
    const source = fixture.widgets.find(widget => row.id === `${widget.id}@${widget.channel}#0`);
    const location = new Vector3().fromArray(source.coords);
    expect(row.role).toBe(source.role);
    const edges = new BufferGeometry();
    try {
      edges.setAttribute("position", new Float32BufferAttribute(row.edges, 3));
      edges.computeBoundingBox();
      const bounds = edges.boundingBox!;
      if (source.role === "point") {
        expect(bounds.getCenter(new Vector3()).distanceTo(location)).toBeLessThan(1e-6);
        const size = bounds.getSize(new Vector3());
        for (const axis of size.toArray()) expect(axis).toBeCloseTo(0.1);
        expect(new Vector3().fromArray(row.positions).distanceTo(location)).toBeLessThan(1e-6);
      } else {
        expect(new Vector3().fromArray(row.positions).length()).toBe(0);
        expect(new Vector3().fromArray(row.positions, 3).distanceTo(location)).toBeLessThan(1e-6);
        expect(row.edges.length).toBe(30);
        const axis = location.clone().normalize();
        for (let offset = 6; offset < row.edges.length; offset += 6) {
          const tip = new Vector3().fromArray(row.edges, offset);
          const fin = new Vector3().fromArray(row.edges, offset + 3);
          expect(tip.distanceTo(location)).toBeLessThan(1e-6);
          expect(fin.dot(axis)).toBeCloseTo(0.8 * location.length());
          expect(fin.addScaledVector(axis, -fin.dot(axis)).length()).toBeCloseTo(0.07 * location.length());
        }
      }
    } finally {
      edges.dispose();
    }
  }
});
