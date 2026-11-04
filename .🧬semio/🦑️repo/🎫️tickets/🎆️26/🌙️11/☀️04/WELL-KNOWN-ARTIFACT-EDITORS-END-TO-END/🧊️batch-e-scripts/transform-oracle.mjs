import * as THREE from "/home/deck/semio/node_modules/three/build/three.module.js";
import { writeFileSync } from "node:fs";
const out = process.argv[2];
const cases = [
  { id: "translate-only", transform: { translate: [1, -2, 3] }, points: [[0, 0, 0], [1, 1, 1]] },
  { id: "scale-about-origin", transform: { scale: [2, 3, 4] }, points: [[1, 1, 1], [-1, 0.5, 2]] },
  { id: "rotate-z-90", transform: { rotate: [0, 0, 90] }, points: [[1, 0, 0], [0, 1, 0], [1, 2, 3]] },
  { id: "rotate-xyz", transform: { rotate: [30, 45, 60] }, points: [[1, 0, 0], [0, 1, 0], [0.5, -2, 7]] },
  { id: "full-with-pivot", transform: { translate: [5, 0, -1], rotate: [10, 20, 30], scale: [2, 1, 0.5], pivot: [1, 1, 1] }, points: [[0, 0, 0], [2, 3, 4], [-1, 5, 0.25]] },
];
const result = cases.map(({ id, transform, points }) => {
  const t = transform;
  const [tx, ty, tz] = t.translate ?? [0, 0, 0];
  const [rx, ry, rz] = (t.rotate ?? [0, 0, 0]).map(THREE.MathUtils.degToRad);
  const [sx, sy, sz] = t.scale ?? [1, 1, 1];
  const [px, py, pz] = t.pivot ?? [0, 0, 0];
  const q = new THREE.Quaternion().setFromEuler(new THREE.Euler(rx, ry, rz, "XYZ"));
  const m = new THREE.Matrix4().makeTranslation(px, py, pz)
    .multiply(new THREE.Matrix4().compose(new THREE.Vector3(tx, ty, tz), q, new THREE.Vector3(sx, sy, sz)))
    .multiply(new THREE.Matrix4().makeTranslation(-px, -py, -pz));
  const normal = new THREE.Matrix3().getNormalMatrix(m);
  return {
    id, transform,
    points: points.map((p) => ({ input: p, expected: new THREE.Vector3(...p).applyMatrix4(m).toArray() })),
    directions: points.map((p) => ({ input: p, expected: new THREE.Vector3(...p).applyMatrix3(normal).normalize().toArray() })),
    quaternion: q.toArray(),
  };
});
writeFileSync(out, JSON.stringify({ oracle: `three@${THREE.REVISION}`, tolerance: 1e-9, cases: result }, null, 2) + "\n");
