/** 🔺️ generation3d rotate-transforms/🔺️diff — mirror of `AxisAngle::then`: the operator's BASE rotation followed by the
 * payload rotation, as a unit axis and an angle in radians. */
import type { RotateTransforms } from "../🦠️mutation/🟦️.ts";

type AxisAngle = { axis: [number, number, number]; angle: number };

const quaternion = ({ axis, angle }: AxisAngle): [number, number, number, number] => {
  const scale = Math.max(...axis.map(Math.abs));
  if (scale === 0) throw new RangeError("Rotation axis cannot be zero");
  const unit = axis.map((value) => value / scale);
  const length = Math.hypot(unit[0], unit[1], unit[2]);
  const s = Math.sin(angle * 0.5);
  return [(unit[0] / length) * s, (unit[1] / length) * s, (unit[2] / length) * s, Math.cos(angle * 0.5)];
};

export function diff(payload: RotateTransforms, base: AxisAngle = { axis: [0, 0, 1], angle: 0 }): AxisAngle {
  const [x, y, z, w] = quaternion(base);
  const [a, b, c, d] = quaternion({ axis: [payload.ax, payload.ay, payload.az], angle: payload.angle });
  let q = [d * x + a * w + b * z - c * y, d * y - a * z + b * w + c * x, d * z + a * y - b * x + c * w, d * w - a * x - b * y - c * z];
  if (q[3] < 0) q = q.map((value) => -value);
  const sine = Math.hypot(q[0], q[1], q[2]);
  if (sine === 0) return { axis: [0, 0, 1], angle: 0 };
  return { axis: [q[0] / sine, q[1] / sine, q[2] / sine], angle: 2 * Math.atan2(sine, q[3]) };
}
