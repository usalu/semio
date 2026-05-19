// #region 🧾FixtureAuthoringToThree
import { Matrix4, Quaternion, Vector3 } from "three";
import type { SemioAuthoringPlane } from "./semioDesignPlane.ts";

/** @emoji 🧾 Tuple position in three.js fixture space (Y-up, RH). */
export type FixtureThreeVec3 = readonly [number, number, number];

/** @emoji 🧾 Tuple quaternion in three.js fixture space (x, y, z, w). */
export type FixtureThreeQuat = readonly [number, number, number, number];

/** @emoji 🧾 Bake-only: external Z-up authoring basis → three.js Y-up fixture origin + quaternion (never imported by the canvas runtime). */
export function authoringPlaneToThreeFixture(plane: SemioAuthoringPlane): { origin: FixtureThreeVec3; orientation: FixtureThreeQuat } {
	const sourceToThree = (p: { x: number; y: number; z: number }): FixtureThreeVec3 => [p.x, p.z, -p.y];
	const x = new Vector3(...sourceToThree(plane.xAxis)).normalize();
	const y = new Vector3(...sourceToThree(plane.yAxis)).normalize();
	const z = new Vector3().crossVectors(x, y).normalize();
	const o = sourceToThree(plane.origin);
	const q = new Quaternion().setFromRotationMatrix(new Matrix4().makeBasis(x, y, z));
	return { origin: o, orientation: [q.x, q.y, q.z, q.w] };
}
// #endregion 🧾FixtureAuthoringToThree

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("authoringPlaneToThreeFixture", () => {
		it("maps source xyz to three (x,z,-y)", () => {
			const { origin, orientation } = authoringPlaneToThreeFixture({
				origin: { x: 1, y: 2, z: 3 },
				xAxis: { x: 1, y: 0, z: 0 },
				yAxis: { x: 0, y: 1, z: 0 },
			});
			expect(origin[0]).toBe(1);
			expect(origin[1]).toBe(3);
			expect(origin[2]).toBe(-2);
			expect(orientation.length).toBe(4);
		});
	});
}
