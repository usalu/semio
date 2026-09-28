/** 🧭️ A rotation in radians around a nonzero world-space axis. */
export type AxisAngle = { readonly axis: readonly [number, number, number]; readonly angle: number };

/** 📏️ Composes non-collapsing scales independently on each world axis. */
export function composeScale(current: readonly [number, number, number], next: readonly [number, number, number]): [number, number, number] {
  const result = current.map((value, axis) => value * next[axis]) as [number, number, number];
  if (result.some(value => !Number.isFinite(value) || value === 0)) throw new Error("Scale factors must remain finite and nonzero");
  return result;
}

function quaternion(rotation: AxisAngle): [number, number, number, number] {
  if (!Number.isFinite(rotation.angle) || rotation.axis.some(value => !Number.isFinite(value))) throw new Error("Rotation values must be finite");
  const scale = Math.max(...rotation.axis.map(Math.abs));
  if (scale === 0) throw new Error("Rotation axis cannot be zero");
  const axis = rotation.axis.map(value => value / scale);
  const length = Math.hypot(...axis), sine = Math.sin(rotation.angle * 0.5);
  return [axis[0] / length * sine, axis[1] / length * sine, axis[2] / length * sine, Math.cos(rotation.angle * 0.5)];
}

/** 🔄️ Applies the next world-axis rotation after the current rotation. */
export function composeRotation(current: AxisAngle, next: AxisAngle): AxisAngle {
  const [x, y, z, w] = quaternion(current), [a, b, c, d] = quaternion(next);
  let q = [d * x + a * w + b * z - c * y, d * y - a * z + b * w + c * x, d * z + a * y - b * x + c * w, d * w - a * x - b * y - c * z];
  if (q[3] < 0) q = q.map(value => -value);
  const sine = Math.hypot(q[0], q[1], q[2]);
  return sine === 0 ? { axis: [0, 0, 1], angle: 0 } : { axis: [q[0] / sine, q[1] / sine, q[2] / sine], angle: 2 * Math.atan2(sine, q[3]) };

}
