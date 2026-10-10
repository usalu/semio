// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🧊️ Third-party ORACLE (three.js `GLTFLoader`) for the binary glTF export of the BIM house.
 *
 * three.js 0.182.0 has never seen this repository's writer. It parses the committed file `🧫️fixtures/🧊️gltf/🏠️house/🏠️house.glb` (container, JSON chunk, accessors,
 * node transforms, materials) into a scene graph and measures it: the glTF node, mesh, primitive and material counts, the triangles read from the index buffers, the
 * element nodes per kind and per storey (from the node `extras`, which three exposes as `userData`), and the world bounds of every vertex through the whole chain of site,
 * building and storey transforms (`Box3.setFromObject(scene, true)`). The subject reports the same table from its typed document.
 *
 * Standalone use (no test host needed):
 *
 *     bun 🟦️.ts check <path to 🧫️fixtures/🧊️gltf>     # exit 1 on any disagreement
 *     bun 🟦️.ts write <path to 🧫️fixtures/🧊️gltf>     # rewrite the measured table from the committed file
 *
 * @see ../../🔮️oracles/🔣️.json
 * @see ../../🚪️io/📤️export/🧊️gltf/🦀️.rs
 */

// #region 🔌️Adapters
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as THREE from "three";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Cases
const GLB = "shared://🧊️gltf/🏠️house/🏠️house.glb";
const COMPONENTS_GLB = "shared://🧊️gltf/🪑️components/🪑️components.glb";
const SPATIAL = new Set(["site", "building", "storey"]);

type Counts = { elements: number; triangles: number };
type Table = {
  nodes: number;
  meshes: number;
  primitives: number;
  triangles: number;
  materials: number;
  kinds: Record<string, number>;
  storeys: Record<string, Counts>;
  volumes: Record<string, number>;
  bounds: { min: number[]; max: number[] };
};
// #endregion 🧫️Cases

// #region 📏️Measure
/** 📥️ Hands the bytes to `GLTFLoader.parse`, which reads the container by its `glTF` magic and validates the chunk framing and the JSON. */
function load(bytes: Uint8Array): Promise<{ scene: THREE.Group; parser: { json: { nodes: unknown[]; meshes: unknown[]; materials: unknown[] } } }> {
  const buffer = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;
  return new Promise((resolve, reject) => new GLTFLoader().parse(buffer, "", (result) => resolve(result as never), reject));
}

/** 🔺️ The triangles of every mesh at or below an object (a node with several primitives is a group of meshes). */
function trianglesOf(object: THREE.Object3D): number {
  let count = 0;
  object.traverse((child) => {
    const mesh = child as THREE.Mesh;
    if (mesh.isMesh) count += mesh.geometry.getIndex()!.count / 3;
  });
  return count;
}

/** 🧊️ The volume enclosed by every mesh at or below an object: the sum of the signed tetrahedra over the origin of each triangle, from the 32-bit vertices as stored (node transforms are rigid, so the local volume is the world volume). */
function volumeOf(object: THREE.Object3D): number {
  let volume = 0;
  const a = new THREE.Vector3();
  const b = new THREE.Vector3();
  const c = new THREE.Vector3();
  object.traverse((child) => {
    const mesh = child as THREE.Mesh;
    if (!mesh.isMesh) return;
    const position = mesh.geometry.getAttribute("position");
    const index = mesh.geometry.getIndex()!;
    for (let corner = 0; corner < index.count; corner += 3) {
      a.fromBufferAttribute(position, index.getX(corner));
      b.fromBufferAttribute(position, index.getX(corner + 1));
      c.fromBufferAttribute(position, index.getX(corner + 2));
      volume += a.dot(b.cross(c)) / 6;
    }
  });
  return volume;
}

/** 📏️ The table of a GLB, measured with three.js. */
export async function measure(bytes: Uint8Array): Promise<Table> {
  const { scene, parser } = await load(bytes);
  scene.updateMatrixWorld(true);
  const table: Table = { nodes: parser.json.nodes.length, meshes: parser.json.meshes.length, primitives: 0, triangles: 0, materials: parser.json.materials.length, kinds: {}, storeys: {}, volumes: {}, bounds: { min: [], max: [] } };
  const used = new Set<THREE.Material>();
  scene.traverse((object) => {
    const mesh = object as THREE.Mesh;
    if (mesh.isMesh) {
      table.primitives += 1;
      table.triangles += mesh.geometry.getIndex()!.count / 3;
      used.add(mesh.material as THREE.Material);
    }
  });
  let described = 0;
  scene.traverse((object) => {
    const extras = object.userData as { kind?: string; storey?: string };
    if (extras.kind === undefined) return;
    described += 1;
    if (SPATIAL.has(extras.kind)) return;
    table.kinds[extras.kind] = (table.kinds[extras.kind] ?? 0) + 1;
    const row = (table.storeys[extras.storey!] ??= { elements: 0, triangles: 0 });
    row.elements += 1;
    row.triangles += trianglesOf(object);
    table.volumes[(object.userData as { id: string }).id] = volumeOf(object);
  });
  if (described !== table.nodes) throw new Error(`the scene graph holds ${described} described nodes, the document ${table.nodes}`);
  if (used.size !== table.materials) throw new Error(`${used.size} materials are used, the document defines ${table.materials}`);
  const box = new THREE.Box3().setFromObject(scene, true);
  table.bounds = { min: box.min.toArray(), max: box.max.toArray() };
  return table;
}
// #endregion 📏️Measure

// #region 🧭️Adapter
/** 🧊️ Oracle answer: the table of the committed file. */
async function exportGltfHouse(ctx: AdapterContext): Promise<AdapterOutcome> {
  const table = await measure(ctx.inputBytes(GLB));
  return { projection: table, raw: JSON.stringify(table) };
}

/** 🪑️ Oracle answer for the components model: the same table, with the volume of every component and MEP element from the stored vertices. */
async function exportGltfComponents(ctx: AdapterContext): Promise<AdapterOutcome> {
  const table = await measure(ctx.inputBytes(COMPONENTS_GLB));
  return { projection: table, raw: JSON.stringify(table) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "export-gltf-house": { oracle: exportGltfHouse },
    "export-gltf-components": { oracle: exportGltfComponents },
  },
});
// #endregion 🧭️Adapter

// #region 🏃️Standalone
/** 🏃️ `check` compares the committed table with the measurement of the committed file; `write` rewrites the table from the file. */
async function main(command: string, root: string): Promise<number> {
  let failed = 0;
  for (const [name, file] of [["🏠️house", "🏠️house.glb"], ["🪑️components", "🪑️components.glb"]]) {
    const folder = join(root, name);
    const table = await measure(new Uint8Array(readFileSync(join(folder, file))));
    const path = join(folder, "🔬️measure", "🔣️.json");
    if (command === "write") {
      writeFileSync(path, `${JSON.stringify(table, null, 2)}
`, "utf8");
      console.log(`wrote the table of ${name}: ${table.nodes} nodes, ${table.triangles} triangles, ${Object.keys(table.volumes).length} volumes`);
      continue;
    }
    const same = JSON.stringify(JSON.parse(readFileSync(path, "utf8"))) === JSON.stringify(table);
    console.log(same ? `check: oracle agrees on ${name} (three 0.182.0)` : `[FAIL] the committed table of ${name} differs from the measurement of the committed file`);
    failed += same ? 0 : 1;
  }
  return failed === 0 ? 0 : 1;
}

if (import.meta.main) process.exit(await main(process.argv[2]!, process.argv[3]!));
// #endregion 🏃️Standalone
