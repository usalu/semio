// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🧊️ Third-party ORACLE (three.js) for the shared World3d scene of the BIM surfaces.
 *
 * `../../🧫️fixtures/🔣️.json` is the language-agnostic vector written by the Rust test `bless_the_world_fixture`: for each case the meshes (f32 positions and triangle
 * indices) and the instance transforms (position and `xyzw` quaternion about +Z) the scene builder published, plus the world-space bounds the Rust side computed from
 * the f64 solid bounds and the building placement. This file rebuilds every instance as a `THREE.Mesh` with that transform and lets three.js's own scene graph
 * (`Matrix4` composition, `Box3.setFromObject`) produce the world-space box of each element: the box of the published f32 mesh under the published transform must be
 * the box the Rust side computed independently, and every index must address a real vertex of a whole number of triangles.
 *
 * Run: `bun test <this file>` from the repository root (three is a root dev dependency).
 */

// #region 🔌️Imports
import { describe, expect, test } from "bun:test";
import * as THREE from "three";
import fixture from "../../🧫️fixtures/🔣️.json";
// #endregion 🔌️Imports

// #region 🧫️Types
type Mesh = { id: string; positions: number[]; indices: number[] };
type Instance = { id: string; position: number[]; rotation: number[] };
type Box = { min: number[]; max: number[] };
type Case = { name: string; meshes: Mesh[]; instances: Instance[]; bounds: Record<string, Box> };
// #endregion 🧫️Types

// #region 📏️Measure
const TOLERANCE = 1e-4;

/** 🧊️ The world-space box of one published element, composed by three.js from the mesh and its instance transform. */
export function worldBox(mesh: Mesh, instance: Instance): THREE.Box3 {
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.BufferAttribute(new Float32Array(mesh.positions), 3));
  geometry.setIndex(mesh.indices);
  const object = new THREE.Mesh(geometry);
  object.position.fromArray(instance.position);
  object.quaternion.fromArray(instance.rotation);
  object.updateMatrixWorld(true);
  return new THREE.Box3().setFromObject(object);
}
// #endregion 📏️Measure

// #region 🧪️Cases
describe("BIM shared world scene", () => {
  for (const scene of (fixture as { cases: Case[] }).cases) {
    describe(scene.name, () => {
      test("every mesh has a whole number of triangles addressing real vertices", () => {
        for (const mesh of scene.meshes) {
          const vertices = mesh.positions.length / 3;
          expect(Number.isInteger(vertices)).toBe(true);
          expect(mesh.indices.length % 3).toBe(0);
          expect(Math.max(...mesh.indices)).toBeLessThan(vertices);
        }
      });

      test("each instance is keyed like its mesh and its quaternion is a rotation about +Z", () => {
        expect(scene.instances.map((instance) => instance.id)).toEqual(scene.meshes.map((mesh) => mesh.id));
        for (const instance of scene.instances) {
          const [x, y, z, w] = instance.rotation;
          expect(x).toBe(0);
          expect(y).toBe(0);
          expect(Math.hypot(z, w)).toBeCloseTo(1, 12);
        }
      });

      test("three.js world boxes equal the bounds the subject computed from its solids", () => {
        expect(Object.keys(scene.bounds).sort()).toEqual(scene.meshes.map((mesh) => mesh.id).sort());
        for (const mesh of scene.meshes) {
          const instance = scene.instances.find((row) => row.id === mesh.id)!;
          const box = worldBox(mesh, instance);
          const expected = scene.bounds[mesh.id]!;
          box.min.toArray().forEach((value, axis) => expect(Math.abs(value - expected.min[axis]!)).toBeLessThan(TOLERANCE));
          box.max.toArray().forEach((value, axis) => expect(Math.abs(value - expected.max[axis]!)).toBeLessThan(TOLERANCE));
        }
      });
    });
  }
});
// #endregion 🧪️Cases
