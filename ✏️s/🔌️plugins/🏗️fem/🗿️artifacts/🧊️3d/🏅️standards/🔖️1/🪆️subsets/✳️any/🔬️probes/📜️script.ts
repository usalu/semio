#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.fem.fem3d@1/✳️any`.
//
// Everything here MARSHALS and INVOKES; nothing here computes geometry. `../🚪️io/📤️export/🧵️serializers/
// 🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs` and the sibling `🟪️stl` leaf both bridge every
// `FemSolid` through `crate::fem3d_engine::meshing::build_semio_mesh_snapshot` (triangulate the
// outline+holes footprint, extrude by the solid's OWN `height`, offset by `base_z`, take the boundary
// faces) into the real, already-oracled `s.stdio.semio@v1/✳️mesh` OBJ/STL bridge — so these ARE real
// geometry carriers, not `print_dsl` under a foreign extension. `three` parses them; `manifold-3d`
// measures the solid. Different engine families, so the measurement checks the parse instead of
// confirming it.
//
// ONLY THREE of this subset's twenty-five mutations move a vertex a carrier can see:
// `create-solid`/`replace-solid`/`delete-solid`. Every other kind — nodes, bar/frame elements (no
// cross-section PROFILE, so no honest 3D solid — see the bridge fn's own doc), materials, sections,
// supports, load cases, loads, combinations, analysis settings — is INVISIBLE in OBJ/STL by
// construction: `crate::fem3d_engine::meshing::build_semio_mesh_snapshot` reads only
// `solid.{outline,holes,height,baseZ}` and consults nothing else on the document. There is no
// "unsupported" branch below for that reason — this probe suite is never even asked about them; the
// owning `🔣️oracle.json`'s `mutationManifests` route those 22 kinds to no oracle at all.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts mesh-import  --input <a.obj|a.stl>
//   bun 📜️script.ts measure      --input <a.obj|a.stl>
//   bun 📜️script.ts topology     --input <a.obj|a.stl>
//   bun 📜️script.ts mesh-compare --input <expected.*> --input <actual.*>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🔬️probes/📜️script.ts
//      — the pilot this file trims down to exactly the two carriers FEM 3D actually exports.
// @see ../../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the sibling 2D probe suite
//      this file mirrors byte-for-byte except for this header.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { extname } from "node:path";
import * as THREE from "three";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import { MeshBVH } from "three-mesh-bvh";
import Module from "manifold-3d";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 🔬️ The typed report every probe emits. The orchestrator compares `measurements`; it never computes them. */
type ProbeReport = {
  schema: "semio.repository-test.probe-report/v2";
  probe: string;
  probeVersion: string;
  engine: { family: string; implementation: string; version: string };
  status: "ok" | "failed" | "unsupported";
  durationMs: number;
  measurements: Record<string, unknown>;
  diagnostics?: { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
};

/** ⚙️ Carrier parsing is three's. Measurement is manifold's — a different engine family, so one checks
 *  the other rather than confirming its own reading. */
const PARSE_ENGINE = { family: "threejs", implementation: "three loaders", version: "0.182.0" } as const;
const MEASURE_ENGINE = { family: "manifold", implementation: "manifold-3d wasm", version: "3.5.1" } as const;
const PROBE_VERSION = "three@0.182.0 + manifold-3d@3.5.1 + three-mesh-bvh@0.9.14";

type IndexedMesh = { positions: number[]; triangles: number[] };
//#endregion 🧬️Contract

//#region 📥️Carrier
function geometryOf(object: THREE.Object3D): THREE.BufferGeometry[] {
  const found: THREE.BufferGeometry[] = [];
  object.traverse((child) => {
    const mesh = child as THREE.Mesh;
    if (mesh.isMesh && mesh.geometry) found.push(mesh.geometry as THREE.BufferGeometry);
  });
  return found;
}

/** 📥️ Parse a carrier with THREE'S OWN loader for that format. Only `.obj`/`.stl` are registered here
 *  — the two real carriers this subset exports — an unknown extension is `unsupported` rather than a
 *  guess, because a mis-parsed carrier would silently compare noise. */
function readCarrier(absPath: string): { geometries: THREE.BufferGeometry[] } {
  const extension = extname(absPath).toLowerCase();
  const bytes = readFileSync(absPath);
  const buffer = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;
  if (extension === ".stl") return { geometries: [new STLLoader().parse(buffer)] };
  if (extension === ".obj") return { geometries: geometryOf(new OBJLoader().parse(new TextDecoder().decode(bytes))) };
  throw new Error(`unsupported carrier extension ${extension} — this subset exports only .obj and .stl`);
}

/** 🔺️ Flatten every geometry in the carrier into one indexed triangle set. */
function indexedOf(geometries: readonly THREE.BufferGeometry[]): IndexedMesh {
  const positions: number[] = [];
  const triangles: number[] = [];
  for (const geometry of geometries) {
    const base = positions.length / 3;
    const attribute = geometry.getAttribute("position");
    if (!attribute) continue;
    for (let i = 0; i < attribute.count; i += 1) positions.push(attribute.getX(i), attribute.getY(i), attribute.getZ(i));
    if (geometry.index) for (let i = 0; i < geometry.index.count; i += 1) triangles.push(base + (geometry.index.array[i] as number));
    else for (let i = 0; i < attribute.count; i += 1) triangles.push(base + i);
  }
  return { positions, triangles };
}
//#endregion 📥️Carrier

//#region 🔺️Mesh
type ManifoldModule = Awaited<ReturnType<typeof Module>>;
type ManifoldSolid = ReturnType<ManifoldModule["Manifold"]["cube"]>;
let kernelPromise: Promise<ManifoldModule> | undefined;
async function meshKernel(): Promise<ManifoldModule> {
  kernelPromise ??= (async () => {
    const wasm = await Module();
    wasm.setup();
    return wasm;
  })();
  return kernelPromise;
}

function boundsOf(positions: readonly number[]): { min: [number, number, number]; max: [number, number, number]; diagonal: number } {
  const min: [number, number, number] = [Infinity, Infinity, Infinity];
  const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < positions.length; i += 3)
    for (let axis = 0; axis < 3; axis += 1) {
      const value = positions[i + axis] as number;
      if (value < min[axis]!) min[axis] = value;
      if (value > max[axis]!) max[axis] = value;
    }
  const diagonal = Math.hypot(max[0] - min[0], max[1] - min[1], max[2] - min[2]);
  return { min, max, diagonal: Number.isFinite(diagonal) ? diagonal : 0 };
}

/** 🔗️ OBJ and STL are triangle SOUPS — every facet carries its own copy of each corner. manifold-3d
 *  refuses that outright, so welding onto a grid keyed to the model's OWN size (never a fixed constant
 *  — see the mesh pilot's measured 6336→1056 weld) is a precondition, not tidying. */
function weld(mesh: IndexedMesh): { vertices: number[]; triangles: number[]; weldedAway: number; degenerate: number } {
  const grid = Math.max(1e-9, boundsOf(mesh.positions).diagonal * 1e-7);
  const index = new Map<string, number>();
  const vertices: number[] = [];
  const remapped: number[] = [];
  for (const corner of mesh.triangles) {
    const x = mesh.positions[corner * 3] as number;
    const y = mesh.positions[corner * 3 + 1] as number;
    const z = mesh.positions[corner * 3 + 2] as number;
    const key = `${Math.round(x / grid)},${Math.round(y / grid)},${Math.round(z / grid)}`;
    let at = index.get(key);
    if (at === undefined) {
      at = vertices.length / 3;
      index.set(key, at);
      vertices.push(x, y, z);
    }
    remapped.push(at);
  }
  let degenerate = 0;
  const triangles: number[] = [];
  for (let t = 0; t < remapped.length; t += 3) {
    const [a, b, c] = [remapped[t] as number, remapped[t + 1] as number, remapped[t + 2] as number];
    if (a === b || b === c || a === c) degenerate += 1;
    else triangles.push(a, b, c);
  }
  return { vertices, triangles, weldedAway: mesh.triangles.length - vertices.length / 3, degenerate };
}

async function asSolid(mesh: IndexedMesh): Promise<{ solid: ManifoldSolid; weldedAway: number; degenerate: number }> {
  const kernel = await meshKernel();
  const welded = weld(mesh);
  const solid = kernel.Manifold.ofMesh(new kernel.Mesh({ numProp: 3, vertProperties: new Float32Array(welded.vertices), triVerts: new Uint32Array(welded.triangles) }));
  return { solid, weldedAway: welded.weldedAway, degenerate: welded.degenerate };
}

/** 📐️ Symmetric Hausdorff via three-mesh-bvh closest-point queries. Exact at vertices and a LOWER
 *  BOUND between them, stated rather than hidden. */
function symmetricHausdorff(expected: IndexedMesh, actual: IndexedMesh): { symmetricHausdorff: number; expectedToActual: number; actualToExpected: number; samples: number } {
  const build = (mesh: IndexedMesh): THREE.BufferGeometry => {
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute("position", new THREE.BufferAttribute(new Float32Array(mesh.positions), 3));
    geometry.setIndex(new THREE.BufferAttribute(new Uint32Array(mesh.triangles), 1));
    return geometry.toNonIndexed();
  };
  const oneSided = (from: IndexedMesh, toward: THREE.BufferGeometry): number => {
    const bvh = new MeshBVH(toward);
    const point = new THREE.Vector3();
    const target = { point: new THREE.Vector3(), distance: 0 };
    let worst = 0;
    for (let i = 0; i < from.positions.length; i += 3) {
      point.set(from.positions[i] as number, from.positions[i + 1] as number, from.positions[i + 2] as number);
      const hit = bvh.closestPointToPoint(point, target as never);
      if (hit && hit.distance > worst) worst = hit.distance;
    }
    return worst;
  };
  const expectedToActual = oneSided(expected, build(actual));
  const actualToExpected = oneSided(actual, build(expected));
  return { symmetricHausdorff: Math.max(expectedToActual, actualToExpected), expectedToActual, actualToExpected, samples: expected.positions.length / 3 + actual.positions.length / 3 };
}

/** 🎯️ The gate. The underlying geometry here is a STRAIGHT-EDGED polygon (outline + holes) extruded
 *  linearly — no curvature anywhere in `build_semio_mesh_snapshot`'s bridge — so unlike a curved BRep
 *  solid, volume and area are triangulation-INVARIANT: any faithful tessellation of the same flat-faced
 *  prism integrates to the same value. That is why this gates NEAR-EXACT (a scale-relative floor, not a
 *  tessellation tolerance) even though the independent `🏭️generator` fixture and the real Rust export
 *  almost certainly triangulate the polygon differently. MEASURED below, both ways, before this comment
 *  was written: see `📓️fem-mesh-oracle-report.md` for the accept/reject numbers this claim rests on. */
async function compareMeshes(expected: IndexedMesh, actual: IndexedMesh): Promise<Record<string, unknown>> {
  const kernel = await meshKernel();
  const expectedSide = await asSolid(expected);
  const actualSide = await asSolid(actual);
  const a = expectedSide.solid;
  const b = actualSide.solid;
  const difference = kernel.Manifold.difference(a, b).add(kernel.Manifold.difference(b, a));
  const symmetricDifferenceVolume = difference.volume();
  const expectedVolume = a.volume();
  const diagonal = boundsOf(expected.positions).diagonal;
  const relative = (x: number, y: number): number => (Math.abs(y) < Number.EPSILON ? Math.abs(x - y) : Math.abs(x - y) / Math.abs(y));
  const hausdorff = symmetricHausdorff(expected, actual);
  return {
    normalizedSymmetricHausdorff: hausdorff.symmetricHausdorff / Math.max(diagonal, Number.EPSILON),
    symmetricHausdorff: hausdorff.symmetricHausdorff,
    hausdorffExpectedToActual: hausdorff.expectedToActual,
    hausdorffActualToExpected: hausdorff.actualToExpected,
    hausdorffSamples: hausdorff.samples,
    normalizedSymmetricDifferenceVolume: symmetricDifferenceVolume / Math.max(expectedVolume, Number.EPSILON),
    symmetricDifferenceVolume,
    relativeVolumeError: relative(b.volume(), expectedVolume),
    relativeAreaError: relative(b.surfaceArea(), a.surfaceArea()),
    connectedComponentsEqual: a.decompose().length === b.decompose().length,
    genusEqual: a.genus() === b.genus(),
    expected: { volume: expectedVolume, area: a.surfaceArea(), genus: a.genus(), triangles: a.numTri(), vertices: a.numVert(), components: a.decompose().length },
    actual: { volume: b.volume(), area: b.surfaceArea(), genus: b.genus(), triangles: b.numTri(), vertices: b.numVert(), components: b.decompose().length },
    boundingBoxDiagonal: diagonal,
    tessellationDiffers: expected.triangles.length !== actual.triangles.length,
    weldedAway: { expected: expectedSide.weldedAway, actual: actualSide.weldedAway },
    degenerate: { expected: expectedSide.degenerate, actual: actualSide.degenerate },
  };
}
//#endregion 🔺️Mesh

//#region 🔬️Probes
type Probe = (inputs: string[]) => Promise<Pick<ProbeReport, "status" | "measurements"> & { diagnostics?: ProbeReport["diagnostics"]; engine?: ProbeReport["engine"] }>;

function requireInputs(inputs: readonly string[], n: number, probe: string): void {
  if (inputs.length < n) throw new Error(`${probe} needs ${n} --input path(s), got ${inputs.length}`);
}

const PROBES: Record<string, Probe> = {
  "mesh-import": async (inputs) => {
    requireInputs(inputs, 1, "mesh-import");
    const mesh = indexedOf(readCarrier(inputs[0]!).geometries);
    const bounds = boundsOf(mesh.positions);
    return { status: "ok", engine: PARSE_ENGINE, measurements: { parsed: true, vertices: mesh.positions.length / 3, triangles: mesh.triangles.length / 3, boundingBoxMin: bounds.min, boundingBoxMax: bounds.max, boundingBoxDiagonal: bounds.diagonal, sha256: createHash("sha256").update(readFileSync(inputs[0]!)).digest("hex") } };
  },
  measure: async (inputs) => {
    requireInputs(inputs, 1, "measure");
    const mesh = indexedOf(readCarrier(inputs[0]!).geometries);
    const side = await asSolid(mesh);
    const bounds = boundsOf(mesh.positions);
    return { status: "ok", measurements: { volume: side.solid.volume(), surfaceArea: side.solid.surfaceArea(), boundingBoxMin: bounds.min, boundingBoxMax: bounds.max, boundingBoxDiagonal: bounds.diagonal, weldedAway: side.weldedAway, degenerateTriangles: side.degenerate } };
  },
  topology: async (inputs) => {
    requireInputs(inputs, 1, "topology");
    const side = await asSolid(indexedOf(readCarrier(inputs[0]!).geometries));
    return { status: "ok", measurements: { genus: side.solid.genus(), connectedComponents: side.solid.decompose().length, triangles: side.solid.numTri(), vertices: side.solid.numVert() } };
  },
  "mesh-compare": async (inputs) => {
    requireInputs(inputs, 2, "mesh-compare");
    const expected = indexedOf(readCarrier(inputs[0]!).geometries);
    const actual = indexedOf(readCarrier(inputs[1]!).geometries);
    return { status: "ok", measurements: await compareMeshes(expected, actual) };
  },
};
//#endregion 🔬️Probes

//#region 🚀️Entry
function parseArgv(argv: readonly string[]): { probe: string; inputs: string[] } {
  const [probe = "", ...rest] = argv;
  const inputs: string[] = [];
  for (let i = 0; i < rest.length; i += 1) if (rest[i] === "--input") inputs.push(rest[i + 1] ?? "");
  return { probe, inputs };
}

async function main(argv: readonly string[]): Promise<number> {
  const { probe, inputs } = parseArgv(argv);
  const started = Date.now();
  const emit = (report: ProbeReport): number => {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    return report.status === "failed" ? 1 : 0;
  };
  // ⏱️A probe that hangs reports nothing, which is strictly worse than one that fails. Bound every run.
  const budgetMs = Number(process.env.SEMIO_PROBE_TIMEOUT_MS ?? 120_000);
  const watchdog = new Promise<never>((_, reject) => setTimeout(() => reject(new Error(`probe exceeded ${budgetMs} ms`)), budgetMs).unref?.());
  const run = PROBES[probe];
  if (!run) return emit({ schema: "semio.repository-test.probe-report/v2", probe: probe || "(none)", probeVersion: PROBE_VERSION, engine: MEASURE_ENGINE, status: "failed", durationMs: 0, measurements: {}, diagnostics: [{ severity: "error", message: `unknown probe ${probe}`, detail: `known: ${Object.keys(PROBES).join(", ")}` }] });
  try {
    const result = await Promise.race([run(inputs), watchdog]);
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: result.engine ?? MEASURE_ENGINE, status: result.status, durationMs: Date.now() - started, measurements: result.measurements, ...(result.diagnostics ? { diagnostics: result.diagnostics } : {}) });
  } catch (error) {
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: MEASURE_ENGINE, status: "failed", durationMs: Date.now() - started, measurements: {}, diagnostics: [{ severity: "error", message: String((error as Error).message ?? error) }] });
  }
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
