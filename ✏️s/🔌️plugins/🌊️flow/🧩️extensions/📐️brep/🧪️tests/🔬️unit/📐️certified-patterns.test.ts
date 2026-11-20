import { expect, test } from "bun:test";
import { Box3, BoxGeometry, Matrix4, Quaternion, SphereGeometry, Vector3 } from "three";
import fixture from "./🔣️certified-patterns.json";

type Parameters = Record<string, number | number[]>;
const vector = (xyz: number[]) => new Vector3(xyz[0], xyz[1], xyz[2]);

test("Three.js transforms agree with certified output order and separation", () => {
  const origin = new Vector3();
  const unit = new Vector3(1, 1, 1);
  for (const row of fixture.cases) {
    const source = row.source;
    const bounds = source.kind === "box"
      ? new Box3(origin.clone(), vector(source.size!))
      : new Box3().setFromBufferAttribute(new SphereGeometry(source.radius!).getAttribute("position"));
    bounds.translate(vector(source.origin));
    const p: Parameters = row.parameters as Parameters;
    const offset = (key: string, amount: number) => vector(p[key] as number[]).multiplyScalar(amount);
    const boxes: Box3[] = [];
    for (const [index, expected] of row.centers.entries()) {
      const transform = new Matrix4();
      if (row.kind === "circular") {
        const rotation = new Quaternion().setFromAxisAngle(vector(p.axis as number[]).normalize(), 2 * Math.PI * index / Number(p.count));
        transform.compose(origin, rotation, unit);
      } else {
        const shift = row.kind === "linear" ? offset("direction", Number(p.spacing) * index)
          : offset("dirX", Number(p.spacingX) * Math.floor(index / Number(p.countY)))
            .add(offset("dirY", Number(p.spacingY) * (index % Number(p.countY))));
        transform.setPosition(shift);
      }
      const moved = bounds.clone().applyMatrix4(transform);
      expect(moved.getCenter(new Vector3()).distanceTo(vector(expected))).toBeLessThan(1e-10);
      expect(boxes.some(box => box.intersectsBox(moved))).toBe(false);
      boxes.push(moved);
    }
  }
  const source = new BoxGeometry(1, 2, 1.5);
  const original = new Box3().setFromBufferAttribute(source.getAttribute("position"));
  const overlap = fixture.overlap;
  expect(original.intersectsBox(original.clone().translate(vector(overlap.direction).multiplyScalar(overlap.spacing)))).toBe(true);
});
