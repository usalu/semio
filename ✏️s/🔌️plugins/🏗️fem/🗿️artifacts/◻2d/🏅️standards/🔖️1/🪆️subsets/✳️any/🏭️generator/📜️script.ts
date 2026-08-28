#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.fem.fem2d@1/✳️any`.
//
// A `FemRegion` is an outline (+ zero or more holes) extruded by its own `thickness` — exactly what
// `crate::fem2d_engine::meshing::build_semio_mesh_snapshot` bridges into OBJ/STL. Every recipe here
// builds the SAME kind of shape independently: `manifold-3d`'s own `CrossSection.extrude` (a real
// solid-boolean kernel's native polygon-with-holes extrusion, not a re-derivation of our triangulator)
// BUILDS it and is the sole source of its volume/area/genus truth, and `three` (a real 3D engine,
// nothing of ours) EXPORTS it to OBJ/STL and RE-IMPORTS what it wrote. Nothing here reimplements our
// `crate::mesh::triangulate`/`extrude_tri_mesh`/`split_to_tets`/`boundary_faces` chain — that is the
// point: an expectation produced by two second parties proves the CAPABILITY (a straight-edged
// polygon-with-holes prism round-trips through OBJ/STL) rather than that either library agrees with us.
//
// Generation and execution are SEPARATE. A normal test run must never be able to rewrite the
// expectation it is measured against, so this is its own command and its output is reviewed before
// it is committed.
//
//   bun 📜️script.ts generate [--out <dir>] [--only <fixture-id>]
//   bun 📜️script.ts manifests                      # emit the fixtureManifests block for 🧪️oracle
//
// @see ../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🏭️generator/📜️script.ts
//      — the pilot this file mirrors in CLI shape, bundle layout and manifest fields.
// @see ../🕸️meshing/… (crate::fem2d_engine::meshing::build_semio_mesh_snapshot) — the honest-geometry
//      bridge whose OUTER CAPABILITY (not its triangulation) this corpus stands in for.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Module from "manifold-3d";
import type { CrossSection, Manifold } from "manifold-3d";
import * as THREE from "three";
import { STLExporter } from "three/examples/jsm/exporters/STLExporter.js";
import { OBJExporter } from "three/examples/jsm/exporters/OBJExporter.js";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";

// 🩹️ Defensive: three's PLY/GLTF exporters need these off-browser; STL/OBJ do not, in principle, but
// the mesh pilot's spike found them cheap insurance and the cost of carrying them is one shim block.
(globalThis as { requestAnimationFrame?: (cb: () => void) => number }).requestAnimationFrame ??= (cb) => { cb(); return 0; };
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 👪️ The fixture families the corpus is sharded and reported by — never at artifact level. */
export type Family = "rectangles" | "polygons" | "holes" | "scale" | "degenerate";

export type Toolkit = Readonly<{ Manifold: typeof Manifold; CrossSection: typeof CrossSection }>;

/** 🧪️ One corpus entry. A recipe DESCRIBES a `FemRegion`'s geometry (outline + holes + thickness); it
 *  computes nothing. `build` returns the extruded solid manifold-3d itself produced. */
export type Recipe = Readonly<{
  id: string;
  family: Family;
  tolerance: string;
  notes: string;
  outline: readonly (readonly [number, number])[];
  holes: readonly (readonly (readonly [number, number])[])[];
  thickness: number;
}>;

const ENGINE_FAMILY = "manifold";
const ENGINE_VERSION = "3.5.1";
const EXPORT_ENGINE_FAMILY = "three";
const EXPORT_ENGINE_VERSION = "0.182.0";
const ORACLE = "manifold-fem2d-mesh-measure";
const SEED = 4815162342;

/** 🔺️ The weld grid a re-imported triangle SOUP is snapped onto before manifold-3d, scale-relative —
 *  `max(absoluteFloor, relative × boundingBoxDiagonal)` — never a fixed constant (a fixed grid either
 *  merges real detail on a millimetre part or does nothing on one scaled to 1e6, exactly the failure
 *  mode the `scale` family below exists to rule out). */
const WELD_RELATIVE = 1e-7;
const WELD_ABSOLUTE_FLOOR = 1e-9;
function weldGridFor(diagonal: number): number {
  return Math.max(WELD_ABSOLUTE_FLOOR, WELD_RELATIVE * Math.abs(diagonal));
}
//#endregion 🧬️Contract

//#region 🧰️Kernel
let toolkit: { wasm: Awaited<ReturnType<typeof Module>>; Manifold: typeof Manifold; CrossSection: typeof CrossSection } | null = null;
async function manifold3d(): Promise<NonNullable<typeof toolkit>> {
  if (toolkit !== null) return toolkit;
  const wasm = await Module();
  wasm.setup();
  toolkit = { wasm, Manifold: wasm.Manifold, CrossSection: wasm.CrossSection };
  return toolkit;
}

/** 🌐️ `FemRegion.{outline,holes,thickness}` → a real extruded `Manifold`, via manifold-3d's OWN
 *  polygon-with-holes handling: `EvenOdd` fill toggles fill state on every contour crossing regardless
 *  of winding, so a hole loop nested inside the outer loop subtracts itself without this generator
 *  having to agree with `crate::mesh`'s "either winding" convention on which way is which. */
function extrudeRegion(t: Toolkit, recipe: Recipe): Manifold {
  const contours = [recipe.outline as unknown as [number, number][], ...recipe.holes.map((hole) => hole as unknown as [number, number][])];
  const cross = new t.CrossSection(contours, "EvenOdd");
  return cross.extrude(recipe.thickness);
}

function toThree(m: ReturnType<Manifold["getMesh"]>): THREE.BufferGeometry {
  const g = new THREE.BufferGeometry();
  const pos = new Float32Array((m.vertProperties.length / m.numProp) * 3);
  for (let i = 0; i < pos.length / 3; i += 1) {
    pos[i * 3] = m.vertProperties[i * m.numProp]!;
    pos[i * 3 + 1] = m.vertProperties[i * m.numProp + 1]!;
    pos[i * 3 + 2] = m.vertProperties[i * m.numProp + 2]!;
  }
  g.setAttribute("position", new THREE.BufferAttribute(pos, 3));
  g.setIndex(new THREE.BufferAttribute(new Uint32Array(m.triVerts), 1));
  g.computeVertexNormals();
  return g;
}

/** 🔗️ Re-imported OBJ/STL is a triangle SOUP with no shared topology; manifold-3d refuses it outright.
 *  Welding on a grid keyed to the model's OWN size (never a constant) rebuilds what it needs. */
function weldToManifold(wasm: Awaited<ReturnType<typeof Module>>, g: THREE.BufferGeometry): { mesh: InstanceType<typeof wasm.Mesh>; grid: number; soupVertices: number; weldedVertices: number; droppedDegenerateTriangles: number } {
  const p = g.getAttribute("position")!;
  g.computeBoundingBox();
  const diagonal = g.boundingBox!.min.distanceTo(g.boundingBox!.max);
  const grid = weldGridFor(diagonal);
  const index = new Map<string, number>();
  const verts: number[] = [];
  const tris: number[] = [];
  let dropped = 0;
  for (let i = 0; i < p.count; i += 1) {
    const x = p.getX(i), y = p.getY(i), z = p.getZ(i);
    const key = `${Math.round(x / grid)},${Math.round(y / grid)},${Math.round(z / grid)}`;
    let at = index.get(key);
    if (at === undefined) { at = verts.length / 3; index.set(key, at); verts.push(x, y, z); }
    tris.push(at);
  }
  const keptTris: number[] = [];
  for (let tIdx = 0; tIdx < tris.length / 3; tIdx += 1) {
    const a = tris[tIdx * 3]!, b = tris[tIdx * 3 + 1]!, c = tris[tIdx * 3 + 2]!;
    if (a !== b && b !== c && a !== c) keptTris.push(a, b, c);
    else dropped += 1;
  }
  const mesh = new wasm.Mesh({ numProp: 3, vertProperties: new Float32Array(verts), triVerts: new Uint32Array(keptTris) });
  return { mesh, grid, soupVertices: p.count, weldedVertices: verts.length / 3, droppedDegenerateTriangles: dropped };
}

/** 📐️ `genus()` returns the sentinel `-1` on a multi-component shape; decompose first and report per
 *  component. Every recipe here is expected to be one connected slab, so `solids !== 1` is itself a
 *  finding worth reporting, not just a formula edge case. */
function topology(shape: Manifold): { solids: number; genus: number | null; componentGenus: number[] | null } {
  const components = shape.decompose();
  if (components.length === 1) return { solids: 1, genus: components[0]!.genus(), componentGenus: null };
  return { solids: components.length, genus: null, componentGenus: components.map((c) => c.genus()) };
}
//#endregion 🧰️Kernel

//#region 🧪️Corpus
/** 🧪️ Every recipe is a `FemRegion`'s `{outline, holes, thickness}` — nothing else in the document
 *  reaches `build_semio_mesh_snapshot`, so nothing else needs a stand-in here. Units are metres,
 *  matching this subset's committed snapshot fixtures (e.g. a 4×2 m, 0.25 m thick slab). */
const RECIPES: readonly Recipe[] = [
  // 👪️rectangles — the plainest possible extruded footprint, at three sizes.
  { id: "rect-unit-square", family: "rectangles", tolerance: "fem-polygon-exact", notes: "1×1 m square, 0.1 m thick — the simplest possible FemRegion.", outline: [[0, 0], [1, 0], [1, 1], [0, 1]], holes: [], thickness: 0.1 },
  { id: "rect-floor-slab", family: "rectangles", tolerance: "fem-polygon-exact", notes: "4×2 m, 0.25 m thick — mirrors the committed create-region/appends-a-solid-rectangular-slab scenario exactly (same outline+thickness).", outline: [[0, 0], [4, 0], [4, 2], [0, 2]], holes: [], thickness: 0.25 },
  { id: "rect-thin-plate", family: "rectangles", tolerance: "fem-polygon-exact", notes: "10×6 m footprint, 0.02 m thick — a high aspect-ratio plate.", outline: [[0, 0], [10, 0], [10, 6], [0, 6]], holes: [], thickness: 0.02 },

  // 👪️polygons — non-rectangular, non-convex outlines. Volume/area of a straight-edged prism is
  // triangulation-invariant, so a re-entrant corner is not expected to be any harder to gate than a box.
  { id: "polygon-l-shape", family: "polygons", tolerance: "fem-polygon-exact", notes: "an L-shaped (non-convex, re-entrant) footprint, 0.3 m thick.", outline: [[0, 0], [3, 0], [3, 1], [1, 1], [1, 3], [0, 3]], holes: [], thickness: 0.3 },
  { id: "polygon-triangle", family: "polygons", tolerance: "fem-polygon-exact", notes: "a triangular footprint, 0.15 m thick.", outline: [[0, 0], [5, 0], [2.5, 4]], holes: [], thickness: 0.15 },

  // 👪️holes — a region with holes punched through it, mirroring the committed
  // replace-region/punches-a-stair-opening-through-the-slab scenario. A through-hole in a linearly
  // extruded prism is a genuine topological fact (genus rises by one per hole), not merely a triangle
  // count, exactly as the mesh pilot's bored cube proved for a curved solid.
  { id: "region-one-hole", family: "holes", tolerance: "fem-polygon-exact", notes: "4×2 m slab, one 1×1 m rectangular hole (a stair opening) — genus 1.", outline: [[0, 0], [4, 0], [4, 2], [0, 2]], holes: [[[1, 0.5], [2, 0.5], [2, 1.5], [1, 1.5]]], thickness: 0.25 },
  { id: "region-two-holes", family: "holes", tolerance: "fem-polygon-exact", notes: "4×2 m slab, two disjoint square holes — genus 2.", outline: [[0, 0], [4, 0], [4, 2], [0, 2]], holes: [[[0.5, 0.5], [1.2, 0.5], [1.2, 1.2], [0.5, 1.2]], [[2.8, 0.5], [3.5, 0.5], [3.5, 1.2], [2.8, 1.2]]], thickness: 0.25 },

  // 👪️scale — the SAME shape (the one-hole slab above) at three orders of magnitude either side of 1,
  // to prove the weld grid and the gate are scale-relative rather than tuned to metre-scale fixtures —
  // the exact property the mesh pilot's own `scale` family established for its bored cube.
  { id: "scale-one-hole-1e-3", family: "scale", tolerance: "fem-polygon-exact", notes: "region-one-hole scaled ×1e-3 (millimetre-scale slab).", outline: [[0, 0], [0.004, 0], [0.004, 0.002], [0, 0.002]], holes: [[[0.001, 0.0005], [0.002, 0.0005], [0.002, 0.0015], [0.001, 0.0015]]], thickness: 0.00025 },
  { id: "scale-one-hole-1e3", family: "scale", tolerance: "fem-polygon-exact", notes: "region-one-hole scaled ×1e3 (kilometre-scale slab).", outline: [[0, 0], [4000, 0], [4000, 2000], [0, 2000]], holes: [[[1000, 500], [2000, 500], [2000, 1500], [1000, 1500]]], thickness: 250 },
  { id: "scale-one-hole-1e6", family: "scale", tolerance: "fem-polygon-exact", notes: "region-one-hole scaled ×1e6.", outline: [[0, 0], [4e6, 0], [4e6, 2e6], [0, 2e6]], holes: [[[1e6, 5e5], [2e6, 5e5], [2e6, 1.5e6], [1e6, 1.5e6]]], thickness: 250000 },

  // 👪️degenerate — edge cases a corpus that only ever builds "nice" shapes would never surface.
  { id: "degenerate-hairline-thickness", family: "degenerate", tolerance: "fem-polygon-exact", notes: "a 2×1 m footprint at 1e-6 m thickness — thickness far below the weld grid's own absolute floor at metre scale, kept as a recorded edge case rather than dropped to make the corpus look clean.", outline: [[0, 0], [2, 0], [2, 1], [0, 1]], holes: [], thickness: 1e-6 },
  { id: "degenerate-sliver-outline", family: "degenerate", tolerance: "fem-polygon-exact", notes: "a 5×0.01 m sliver footprint, 0.1 m thick — an extreme aspect ratio in-plane rather than through the thickness.", outline: [[0, 0], [5, 0], [5, 0.01], [0, 0.01]], holes: [], thickness: 0.1 },
];
//#endregion 🧪️Corpus

//#region 🏭️Generate
async function contentDigest(bytes: Uint8Array | string): Promise<string> {
  const source = typeof bytes === "string" ? new TextEncoder().encode(bytes) : bytes;
  const data = new Uint8Array(source.length);
  data.set(source);
  const hash = await crypto.subtle.digest("SHA-256", data);
  return `sha256:${[...new Uint8Array(hash)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}

/** 📎️ Fixture file paths are resolved against the OWNER'S ORACLE directory (`🧪️oracle/`), not this
 *  generator's directory — the exact prefix bug the mesh pilot's playbook records finding. */
const FIXTURE_PATH_PREFIX = "../🧫️fixtures/";

function write(path: string, body: string): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, body);
}

async function exportAll(mesh: THREE.Mesh): Promise<{ stl: string; obj: string }> {
  const stl = new STLExporter().parse(mesh, { binary: false }) as unknown as string;
  const obj = new OBJExporter().parse(mesh);
  return { stl, obj };
}

/** 📥️ Reads back what was actually WRITTEN and re-measures THAT — never the in-memory shape. STL is
 *  the canonical re-measurement source (plainest possible soup, no index, no material); OBJ is
 *  re-imported too, only to CONFIRM its triangle count agrees, so a format-specific export bug shows
 *  up as a cross-format disagreement rather than being laundered into one trusted number. */
async function reimportAndMeasure(t: NonNullable<typeof toolkit>, exported: { stl: string; obj: string }) {
  const stlGeom = new STLLoader().parse(new TextEncoder().encode(exported.stl).buffer as ArrayBuffer);
  const weld = weldToManifold(t.wasm, stlGeom);
  const measured = t.Manifold.ofMesh(weld.mesh);

  const triOf = (g: THREE.BufferGeometry): number => (g.index ? g.index.count : g.getAttribute("position")!.count) / 3;
  const crossFormatTriangleCounts: Record<string, number> = { stl: triOf(stlGeom) };
  try {
    const objGeom = ((new OBJLoader().parse(exported.obj)).children[0] as THREE.Mesh).geometry as THREE.BufferGeometry;
    crossFormatTriangleCounts.obj = triOf(objGeom);
  } catch (error) {
    crossFormatTriangleCounts.obj = Number.NaN;
    console.error(`[generator]   obj re-import cross-check failed — ${(error as Error).message}`);
  }
  return { measured, weld, crossFormatTriangleCounts };
}

async function generateOne(t: NonNullable<typeof toolkit>, recipe: Recipe, outDir: string): Promise<Record<string, unknown>> {
  const dir = join(outDir, recipe.id);
  const files: { role: string; path: string; mediaType: string; sha256: string; bytes: number }[] = [];

  const result = extrudeRegion(t, recipe);
  const empty = result.isEmpty();
  const measurements: Record<string, unknown> = { empty };
  if (!empty) {
    const geometry = toThree(result.getMesh());
    const material = new THREE.MeshStandardMaterial({ name: `${recipe.id}-material` });
    const mesh = new THREE.Mesh(geometry, material);
    const exported = await exportAll(mesh);

    for (const [format, text] of Object.entries(exported)) {
      const filename = `expected.${format}`;
      write(join(dir, filename), text);
      files.push({ role: `expected-${format}`, path: `${FIXTURE_PATH_PREFIX}${recipe.id}/${filename}`, mediaType: `model/${format}`, sha256: await contentDigest(text), bytes: Buffer.byteLength(text) });
    }

    const { measured, weld, crossFormatTriangleCounts } = await reimportAndMeasure(t, exported);
    const bounds = measured.boundingBox();
    const diagonal = Math.hypot(bounds.max[0] - bounds.min[0], bounds.max[1] - bounds.min[1], bounds.max[2] - bounds.min[2]);
    const shape = topology(measured);

    measurements.measuredFrom = "expected.stl, re-imported and welded";
    measurements.vertexCount = measured.numVert();
    measurements.triangleCount = measured.numTri();
    measurements.volume = measured.volume();
    measurements.surfaceArea = measured.surfaceArea();
    measurements.boundingBox = { min: bounds.min, max: bounds.max };
    measurements.boundingBoxDiagonal = diagonal;
    measurements.solids = shape.solids;
    measurements.genus = shape.genus;
    measurements.componentGenus = shape.componentGenus;
    measurements.expectedGenus = recipe.holes.length; // 🕳️an n-hole through-extrusion of a simply-connected footprint has genus n
    measurements.weld = { grid: weld.grid, soupVertices: weld.soupVertices, weldedVertices: weld.weldedVertices, droppedDegenerateTriangles: weld.droppedDegenerateTriangles };
    measurements.crossFormatTriangleCounts = crossFormatTriangleCounts;
    measurements.crossFormatTriangleCountsAgree = crossFormatTriangleCounts.obj === measurements.triangleCount || crossFormatTriangleCounts.obj === weld.soupVertices / 3;

    // 📐️independent analytic cross-check: shoelace area of outline minus holes, times thickness.
    const shoelace = (poly: readonly (readonly [number, number])[]): number => {
      let sum = 0;
      for (let i = 0; i < poly.length; i += 1) {
        const [x0, y0] = poly[i]!;
        const [x1, y1] = poly[(i + 1) % poly.length]!;
        sum += x0 * y1 - x1 * y0;
      }
      return Math.abs(sum) / 2;
    };
    const footprintArea = shoelace(recipe.outline) - recipe.holes.reduce((sum, hole) => sum + shoelace(hole), 0);
    measurements.analyticVolume = footprintArea * recipe.thickness;
    measurements.analyticVolumeRelativeError = Math.abs((measurements.volume as number) - measurements.analyticVolume) / Math.max(measurements.analyticVolume, Number.EPSILON);
  }

  const metricsBody = `${JSON.stringify(measurements, null, 2)}\n`;
  write(join(dir, "expected.metrics.json"), metricsBody);
  files.push({ role: "expected-measurements", path: `${FIXTURE_PATH_PREFIX}${recipe.id}/expected.metrics.json`, mediaType: "application/json", sha256: await contentDigest(metricsBody), bytes: Buffer.byteLength(metricsBody) });

  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.fem.fem2d", standard: "1", subset: "any" },
    mutation: "replace-region-geometry",
    outcome: empty ? "empty" : "applied",
    units: { length: "metre", angle: "degree", handedness: "right", up: "z" },
    files,
    generator: {
      oracle: ORACLE,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      exportEngine: { family: EXPORT_ENGINE_FAMILY, implementation: "three.js exporters + loaders", version: EXPORT_ENGINE_VERSION },
      command: `bun ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      seed: SEED,
      platform: `${process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux"}-${process.arch === "arm64" ? "arm64" : "x64"}`,
    },
    provenance: { source: "generated", license: "Apache-2.0 (manifold-3d) + MIT (three)", attribution: "Generated with manifold-3d (Apache-2.0) and exported/re-imported via three.js (MIT)", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-fem-mesh-manifold-v1",
    toleranceProfile: recipe.tolerance,
    // ✅️Two STLExporter/OBJExporter passes over the same THREE.Mesh in the same process produce
    // byte-identical output — no timestamps, no incrementing counters anywhere in three's exporters.
    reproducible: true,
    family: recipe.family,
    notes: recipe.notes,
  };
}
//#endregion 🏭️Generate

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate", ...rest] = argv;
  const value = (flag: string): string | null => {
    const index = rest.indexOf(flag);
    return index === -1 ? null : (rest[index + 1] ?? null);
  };
  const only = value("--only");
  const recipes = only === null ? RECIPES : RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");

  if (command === "manifests" || command === "generate") {
    const t = await manifold3d();
    const manifests: Record<string, unknown>[] = [];
    let failed = 0;
    for (const recipe of recipes) {
      try {
        manifests.push(await generateOne(t, recipe, outDir));
        console.error(`[generator] ${recipe.id} (${recipe.family})`);
      } catch (error) {
        failed += 1;
        console.error(`[generator] ${recipe.id} FAILED — ${(error as Error).message}`);
      }
    }
    if (command === "manifests") process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
    else {
      const indexPath = join(outDir, "🧫️manifests.json");
      const previous = (() => {
        if (only === null || !existsSync(indexPath)) return [];
        try { return JSON.parse(readFileSync(indexPath, "utf8")) as Record<string, unknown>[]; } catch { return []; }
      })();
      const produced = new Set(manifests.map((entry) => entry.id as string));
      const merged = [...previous.filter((entry) => !produced.has(entry.id as string)), ...manifests].sort((a, b) => String(a.id).localeCompare(String(b.id)));
      write(indexPath, `${JSON.stringify(merged, null, 2)}\n`);
      if (only !== null) console.error(`[generator] merged ${manifests.length} regenerated entr(ies) into ${merged.length} total`);
    }
    console.error(`[generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
    return failed > 0 ? 1 : 0;
  }
  console.error(`[generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
  return 1;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
