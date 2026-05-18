// #region 📐CoordsPlane
import { Matrix4, Quaternion, Vector3 } from "three";

/** @emoji 🧭 Tuple position in three.js world space (Y-up, RH). */
export type SceneVec3 = readonly [number, number, number];

/** @emoji 🧭 Tuple quaternion in three.js world space (x, y, z, w). */
export type SceneQuat = readonly [number, number, number, number];

/** @emoji 🧭 Maps authoring RH basis (origin + xAxis + yAxis) into three.js Y-up RH position + quaternion. */
export function planeBasisToThreeJs(plane: {
	readonly origin: { x: number; y: number; z: number };
	readonly xAxis: { x: number; y: number; z: number };
	readonly yAxis: { x: number; y: number; z: number };
}): { origin: SceneVec3; orientation: SceneQuat } {
	const authoringToThree = (p: { x: number; y: number; z: number }): SceneVec3 => [p.x, p.z, -p.y];
	const x = new Vector3(...authoringToThree(plane.xAxis)).normalize();
	const y = new Vector3(...authoringToThree(plane.yAxis)).normalize();
	const z = new Vector3().crossVectors(x, y).normalize();
	const o = authoringToThree(plane.origin);
	const q = new Quaternion().setFromRotationMatrix(new Matrix4().makeBasis(x, y, z));
	return { origin: o, orientation: [q.x, q.y, q.z, q.w] };
}
// #endregion 📐CoordsPlane
