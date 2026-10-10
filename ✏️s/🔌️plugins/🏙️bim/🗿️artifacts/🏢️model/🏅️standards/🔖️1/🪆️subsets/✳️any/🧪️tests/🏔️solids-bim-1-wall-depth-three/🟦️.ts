// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🧗️ Third-party ORACLE (three.js) for the element solids of the wall depth package: the walls whose top or base is attached (their elevation ends in the slope of a roof or follows a sloped slab),
 * the wall sweeps and the fillers of the attic model.
 *
 * The case under `🧫️fixtures/💡️inferences/🧗️wall-depth/🏠️attic` commits the authored `📸️snapshot`, the table the shapely case `../🧗️infer-bim-1-wall-depth/🐍️.py` wrote and the blessed
 * `🧊️meshes` (welded positions and triangle indices) of the solids the subject inferred from the snapshot. This file loads those meshes into `THREE.BufferGeometry` and measures them with
 * three's own `Triangle`, `Vector3` and `Box3`: volume (the signed sum of tetrahedra `a . (b x c) / 6`, three ships no volume function), surface area (`Triangle.getArea`), bounds (`Box3`) and the
 * triangle count. The subject answers the same projection from its live `ModelInference`, so a mesh that is no longer what the inference produces, or a volume that is not the volume of its
 * triangles, shows up as a disagreement. The volume of the trimmed free walls is also compared with the volume the shapely table derives analytically.
 *
 * @see ../🧗️infer-bim-1-wall-depth/🐍️.py
 * @see ../../🔮️oracles/🔣️.json
 */

// #region 🔌️Adapters
import * as THREE from "three";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Cases
const MESHES = "shared://💡️inferences/🧗️wall-depth/🏠️attic/🧊️meshes/🔣️.json";

type Mesh = { positions: number[]; indices: number[] };
type Row = { volume: number; area: number; bounds: { min: number[]; max: number[] }; triangles: number };
// #endregion 🧫️Cases

// #region 📏️Measure
/** 🧮️ Rounds to twelve digits like the committed tables. */
const round = (value: number) => Math.round(value * 1e12) / 1e12;

/** 📏️ Volume, area, bounds and triangle count of one mesh, measured with three.js. */
export function measure(mesh: Mesh): Row {
  const geometry = new THREE.BufferGeometry();
  const position = new THREE.BufferAttribute(new Float64Array(mesh.positions), 3);
  geometry.setAttribute("position", position);
  geometry.setIndex(mesh.indices);
  const index = geometry.getIndex()!;
  const [a, b, c] = [new THREE.Vector3(), new THREE.Vector3(), new THREE.Vector3()];
  const triangle = new THREE.Triangle();
  let volume = 0;
  let area = 0;
  for (let at = 0; at < index.count; at += 3) {
    a.fromBufferAttribute(position, index.getX(at));
    b.fromBufferAttribute(position, index.getX(at + 1));
    c.fromBufferAttribute(position, index.getX(at + 2));
    volume += a.dot(b.clone().cross(c)) / 6;
    area += triangle.set(a, b, c).getArea();
  }
  const box = new THREE.Box3().setFromBufferAttribute(position);
  return { volume: round(volume), area: round(area), bounds: { min: box.min.toArray().map(round), max: box.max.toArray().map(round) }, triangles: index.count / 3 };
}
// #endregion 📏️Measure

// #region 🧭️Adapter
/** 🧊️ Oracle answer: `{element id: {volume, area, bounds, triangles}}` measured on the committed meshes. */
function wallDepthThree(ctx: AdapterContext): AdapterOutcome {
  const document = JSON.parse(new TextDecoder().decode(ctx.inputBytes(MESHES))) as { meshes: Record<string, Mesh> };
  const projection: Record<string, Row> = Object.fromEntries(Object.entries(document.meshes).map(([id, mesh]) => [id, measure(mesh)]));
  return { projection, raw: JSON.stringify(projection) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "wall-depth-three": { oracle: wallDepthThree },
  },
});
// #endregion 🧭️Adapter
