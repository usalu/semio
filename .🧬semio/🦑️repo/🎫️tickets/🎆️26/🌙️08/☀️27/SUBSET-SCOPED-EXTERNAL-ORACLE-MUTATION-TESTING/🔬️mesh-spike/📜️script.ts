/** 🕸️ Spike: does the mesh toolchain close the loop? manifold-3d builds, three exports,
 *  three re-reads, manifold measures, three-mesh-bvh compares. Two engine families. */
import Module from "manifold-3d";
import * as THREE from "three";
import { STLExporter } from "three/examples/jsm/exporters/STLExporter.js";
import { OBJExporter } from "three/examples/jsm/exporters/OBJExporter.js";
import { PLYExporter } from "three/examples/jsm/exporters/PLYExporter.js";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import { PLYLoader } from "three/examples/jsm/loaders/PLYLoader.js";
import { MeshBVH } from "three-mesh-bvh";

// 🩹️ three's PLYExporter calls requestAnimationFrame to deliver its result; supply it off-browser.
(globalThis as { requestAnimationFrame?: (cb: () => void) => number }).requestAnimationFrame ??= (cb) => { cb(); return 0; };

const wasm = await Module();
wasm.setup();
const { Manifold } = wasm;

/** 🧊️ A non-trivial solid: a cube with a cylindrical bore and a sphere fused on. */
const solid = Manifold.cube([20, 20, 20], true)
  .subtract(Manifold.cylinder(30, 5, 5, 64, true))
  .add(Manifold.sphere(8, 64).translate([10, 10, 10]));
const mm = solid.getMesh();
console.log(`[manifold] verts=${mm.numVert} tris=${mm.numTri} volume=${solid.volume().toFixed(4)} area=${solid.surfaceArea().toFixed(4)}`);

function toThree(m: { numProp: number; vertProperties: Float32Array; triVerts: Uint32Array }): THREE.BufferGeometry {
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

const geom = toThree(mm);
const mesh = new THREE.Mesh(geom, new THREE.MeshStandardMaterial());

// 📤️ Export through three's own exporters — a third-party writer, not ours.
const stl = new STLExporter().parse(mesh, { binary: false }) as unknown as string;
const obj = new OBJExporter().parse(mesh);
const ply = new PLYExporter().parse(mesh, () => {}, { binary: false }) as unknown as string;
console.log(`[export] stl=${stl.length}B obj=${obj.length}B ply=${String(ply).length}B`);
console.log(`[export] stl starts "${stl.slice(0, 12)}" | obj first "${obj.split("\n").find((l) => l.startsWith("v "))}"`);

// 📥️ Re-read with the matching third-party loaders — an independent parse of the bytes.
const reStl = new STLLoader().parse(new TextEncoder().encode(stl).buffer as ArrayBuffer);
const reObj = new OBJLoader().parse(obj);
const rePly = new PLYLoader().parse(new TextEncoder().encode(String(ply)).buffer as ArrayBuffer);
const objGeom = (reObj.children[0] as THREE.Mesh).geometry as THREE.BufferGeometry;
const triOf = (g: THREE.BufferGeometry) => (g.index ? g.index.count : g.getAttribute("position").count) / 3;
console.log(`[reimport] stl tris=${triOf(reStl)} obj tris=${triOf(objGeom)} ply tris=${triOf(rePly)} (source ${mm.numTri})`);

// 📏️ Measure the re-imported STL back in manifold — the OTHER engine family.
/** 🔗️ STL is a triangle SOUP — every facet carries its own copy of each corner, so the mesh has no
 *  shared topology and manifold rejects it. Welding on a grid keyed to the model's own size (never a
 *  fixed constant, which merges detail on small models and nothing on large ones) rebuilds it. */
function weldToManifold(g: THREE.BufferGeometry) {
  const p = g.getAttribute("position");
  g.computeBoundingBox();
  const diagonal = g.boundingBox!.min.distanceTo(g.boundingBox!.max);
  const grid = Math.max(1e-9, diagonal * 1e-7);
  const index = new Map<string, number>();
  const verts: number[] = [];
  const tris: number[] = [];
  for (let i = 0; i < p.count; i += 1) {
    const x = p.getX(i), y = p.getY(i), z = p.getZ(i);
    const key = `${Math.round(x / grid)},${Math.round(y / grid)},${Math.round(z / grid)}`;
    let at = index.get(key);
    if (at === undefined) { at = verts.length / 3; index.set(key, at); verts.push(x, y, z); }
    tris.push(at);
  }
  const degenerate = tris.length / 3 - Array.from({ length: tris.length / 3 }, (_, t) => t).filter((t) => tris[t * 3] !== tris[t * 3 + 1] && tris[t * 3 + 1] !== tris[t * 3 + 2] && tris[t * 3] !== tris[t * 3 + 2]).length;
  console.log(`[weld] grid=${grid.toExponential(2)} ${p.count} soup verts -> ${verts.length / 3} shared | degenerate tris=${degenerate}`);
  return new wasm.Mesh({ numProp: 3, vertProperties: new Float32Array(verts), triVerts: new Uint32Array(tris) });
}
const back = Manifold.ofMesh(weldToManifold(reStl));
console.log(`[manifold re-measure] volume=${back.volume().toFixed(4)} area=${back.surfaceArea().toFixed(4)} genus=${back.genus()}`);
console.log(`[delta] dVolume=${Math.abs(back.volume() - solid.volume()).toExponential(3)} dArea=${Math.abs(back.surfaceArea() - solid.surfaceArea()).toExponential(3)}`);

// 📐️ Hausdorff between source and re-imported, via three-mesh-bvh.
const bvh = new MeshBVH(geom.clone().toNonIndexed());
const target = new THREE.Vector3();
let maxD = 0;
const rp = reStl.getAttribute("position");
for (let i = 0; i < rp.count; i += 1) {
  const hit = bvh.closestPointToPoint(new THREE.Vector3(rp.getX(i), rp.getY(i), rp.getZ(i)), { point: target, distance: 0 } as never);
  if (hit && hit.distance > maxD) maxD = hit.distance;
}
console.log(`[hausdorff] one-sided max=${maxD.toExponential(3)}`);
