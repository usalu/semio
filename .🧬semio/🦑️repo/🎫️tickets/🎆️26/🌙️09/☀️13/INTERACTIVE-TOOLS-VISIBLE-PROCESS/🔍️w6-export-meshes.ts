/** 🩺️ [DEBUG] temp: export catalog GLB meshes in the CAD Z-up frame for a native fill reproduction. */
import { readFileSync, writeFileSync } from "node:fs";
const repo = `${import.meta.dir}/../../../../../../..`;
type V3 = [number, number, number];
type Quat = [number, number, number, number];
interface Mesh { tris: Float64Array; count: number; min: V3; max: V3; grid: Map<string, number[]>; cell: number; samples: V3[] }

function rotate(q: Quat, v: V3): V3 {
  const [x, y, z, w] = q;
  const tx = 2 * (y * v[2] - z * v[1]), ty = 2 * (z * v[0] - x * v[2]), tz = 2 * (x * v[1] - y * v[0]);
  return [v[0] + w * tx + (y * tz - z * ty), v[1] + w * ty + (z * tx - x * tz), v[2] + w * tz + (x * ty - y * tx)];
}
const conj = (q: Quat): Quat => [-q[0], -q[1], -q[2], q[3]];

/** Loads a GLB's triangles into the CAD object-local Z-up frame (+90° about X: (x, y, z) → (x, −z, y)). */
function loadGlb(path: string): Mesh {
  const bytes = readFileSync(path);
  const jsonLength = bytes.readUInt32LE(12);
  const gltf = JSON.parse(bytes.subarray(20, 20 + jsonLength).toString("utf8"));
  const binStart = 20 + jsonLength + 8;
  const view = (accessorIndex: number) => {
    const accessor = gltf.accessors[accessorIndex];
    const bufferView = gltf.bufferViews[accessor.bufferView];
    const offset = binStart + (bufferView.byteOffset ?? 0) + (accessor.byteOffset ?? 0);
    return { accessor, offset, stride: bufferView.byteStride };
  };
  const triangles: number[] = [];
  const nodeMatrixFree = (gltf.nodes ?? []).every((node: { matrix?: unknown; translation?: unknown; rotation?: unknown; scale?: unknown }) => !node.matrix && !node.translation && !node.rotation && !node.scale);
  if (!nodeMatrixFree) throw new Error(`${path}: node transforms are not supported by this probe`);
  for (const mesh of gltf.meshes) {
    for (const primitive of mesh.primitives) {
      if ((primitive.mode ?? 4) !== 4) continue;
      const positions = view(primitive.attributes.POSITION);
      const read = (index: number): V3 => {
        const at = positions.offset + index * (positions.stride ?? 12);
        const x = bytes.readFloatLE(at), y = bytes.readFloatLE(at + 4), z = bytes.readFloatLE(at + 8);
        return [x, -z, y];
      };
      const indexCount = primitive.indices === undefined ? positions.accessor.count : gltf.accessors[primitive.indices].count;
      const indices = primitive.indices === undefined ? null : view(primitive.indices);
      const indexAt = (index: number) => {
        if (!indices) return index;
        const type = indices.accessor.componentType;
        return type === 5125 ? bytes.readUInt32LE(indices.offset + 4 * index) : type === 5123 ? bytes.readUInt16LE(indices.offset + 2 * index) : bytes.readUInt8(indices.offset + index);
      };
      for (let index = 0; index + 2 < indexCount; index += 3) triangles.push(...read(indexAt(index)), ...read(indexAt(index + 1)), ...read(indexAt(index + 2)));
    }
  }
  const tris = Float64Array.from(triangles);
  const count = tris.length / 9;
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < tris.length; i += 3) for (let axis = 0; axis < 3; axis += 1) { min[axis] = Math.min(min[axis], tris[i + axis]); max[axis] = Math.max(max[axis], tris[i + axis]); }
  const cell = Math.max(max[0] - min[0], max[1] - min[1]) / 48;
  const grid = new Map<string, number[]>();
  for (let t = 0; t < count; t += 1) {
    const o = 9 * t;
    const x0 = Math.min(tris[o], tris[o + 3], tris[o + 6]), x1 = Math.max(tris[o], tris[o + 3], tris[o + 6]);
    const y0 = Math.min(tris[o + 1], tris[o + 4], tris[o + 7]), y1 = Math.max(tris[o + 1], tris[o + 4], tris[o + 7]);
    for (let gx = Math.floor((x0 - min[0]) / cell); gx <= Math.floor((x1 - min[0]) / cell); gx += 1) {
      for (let gy = Math.floor((y0 - min[1]) / cell); gy <= Math.floor((y1 - min[1]) / cell); gy += 1) {
        const key = `${gx},${gy}`;
        (grid.get(key) ?? grid.set(key, []).get(key)!).push(t);
      }
    }
  }
  // Surface samples: every vertex, every centroid and every edge midpoint (deduplicated).
  const seen = new Set<string>();
  const samples: V3[] = [];
  const add = (p: V3) => { const key = p.map((value) => value.toFixed(5)).join(","); if (!seen.has(key)) { seen.add(key); samples.push(p); } };
  for (let t = 0; t < count; t += 1) {
    const o = 9 * t;
    const a: V3 = [tris[o], tris[o + 1], tris[o + 2]], b: V3 = [tris[o + 3], tris[o + 4], tris[o + 5]], c: V3 = [tris[o + 6], tris[o + 7], tris[o + 8]];
    add(a); add(b); add(c);
    add([(a[0] + b[0] + c[0]) / 3, (a[1] + b[1] + c[1]) / 3, (a[2] + b[2] + c[2]) / 3]);
    add([(a[0] + b[0]) / 2, (a[1] + b[1]) / 2, (a[2] + b[2]) / 2]); add([(b[0] + c[0]) / 2, (b[1] + c[1]) / 2, (b[2] + c[2]) / 2]); add([(a[0] + c[0]) / 2, (a[1] + c[1]) / 2, (a[2] + c[2]) / 2]);
  }
  return { tris, count, min, max, grid, cell, samples };
}

/** Strict inside test by +Z ray parity over the xy grid, the point nudged off shared edges. */
function inside(mesh: Mesh, p: V3): boolean {
  const eps = 1e-6;
  if (p[0] <= mesh.min[0] + eps || p[0] >= mesh.max[0] - eps || p[1] <= mesh.min[1] + eps || p[1] >= mesh.max[1] - eps || p[2] <= mesh.min[2] + eps || p[2] >= mesh.max[2] - eps) return false;
  const px = p[0] + 3.1e-7, py = p[1] + 1.7e-7;
  const list = mesh.grid.get(`${Math.floor((px - mesh.min[0]) / mesh.cell)},${Math.floor((py - mesh.min[1]) / mesh.cell)}`) ?? [];
  const hits: number[] = [];
  for (const t of list) {
    const o = 9 * t, tr = mesh.tris;
    const ax = tr[o], ay = tr[o + 1], bx = tr[o + 3], by = tr[o + 4], cx = tr[o + 6], cy = tr[o + 7];
    const det = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy);
    if (Math.abs(det) < 1e-12) continue;
    const l1 = ((by - cy) * (px - cx) + (cx - bx) * (py - cy)) / det;
    const l2 = ((cy - ay) * (px - cx) + (ax - cx) * (py - cy)) / det;
    const l3 = 1 - l1 - l2;
    if (l1 < 0 || l2 < 0 || l3 < 0) continue;
    const z = l1 * tr[o + 2] + l2 * tr[o + 5] + l3 * tr[o + 8];
    if (z > p[2]) hits.push(z);
  }
  hits.sort((a, b) => a - b);
  let crossings = 0;
  for (let i = 0; i < hits.length; i += 1) if (i === 0 || hits[i] - hits[i - 1] > 1e-7) crossings += 1;
  return crossings % 2 === 1;
}

function pointTriangleDistance(p: V3, tr: Float64Array, o: number): number {
  const a: V3 = [tr[o], tr[o + 1], tr[o + 2]], b: V3 = [tr[o + 3], tr[o + 4], tr[o + 5]], c: V3 = [tr[o + 6], tr[o + 7], tr[o + 8]];
  const sub = (u: V3, v: V3): V3 => [u[0] - v[0], u[1] - v[1], u[2] - v[2]];
  const dot = (u: V3, v: V3) => u[0] * v[0] + u[1] * v[1] + u[2] * v[2];
  const ab = sub(b, a), ac = sub(c, a), ap = sub(p, a);
  const d1 = dot(ab, ap), d2 = dot(ac, ap);
  let q: V3;
  if (d1 <= 0 && d2 <= 0) q = a;
  else {
    const bp = sub(p, b), d3 = dot(ab, bp), d4 = dot(ac, bp);
    if (d3 >= 0 && d4 <= d3) q = b;
    else {
      const vc = d1 * d4 - d3 * d2;
      if (vc <= 0 && d1 >= 0 && d3 <= 0) { const v = d1 / (d1 - d3); q = [a[0] + v * ab[0], a[1] + v * ab[1], a[2] + v * ab[2]]; }
      else {
        const cp = sub(p, c), d5 = dot(ab, cp), d6 = dot(ac, cp);
        if (d6 >= 0 && d5 <= d6) q = c;
        else {
          const vb = d5 * d2 - d1 * d6;
          if (vb <= 0 && d2 >= 0 && d6 <= 0) { const w = d2 / (d2 - d6); q = [a[0] + w * ac[0], a[1] + w * ac[1], a[2] + w * ac[2]]; }
          else {
            const va = d3 * d6 - d5 * d4;
            if (va <= 0 && d4 - d3 >= 0 && d5 - d6 >= 0) { const w = (d4 - d3) / (d4 - d3 + (d5 - d6)); q = [b[0] + w * (c[0] - b[0]), b[1] + w * (c[1] - b[1]), b[2] + w * (c[2] - b[2])]; }
            else { const denom = 1 / (va + vb + vc); const v = vb * denom, w = vc * denom; q = [a[0] + ab[0] * v + ac[0] * w, a[1] + ab[1] * v + ac[1] * w, a[2] + ab[2] * v + ac[2] * w]; }
          }
        }
      }
    }
  }
  const d = sub(p, q);
  return Math.sqrt(dot(d, d));
}

interface Body { id: string; mesh: Mesh; position: V3; rotation: Quat; worldMin: V3; worldMax: V3 }

function body(id: string, mesh: Mesh, position: V3, rotation: Quat): Body {
  const worldMin: V3 = [Infinity, Infinity, Infinity], worldMax: V3 = [-Infinity, -Infinity, -Infinity];
  for (const cx of [mesh.min[0], mesh.max[0]]) for (const cy of [mesh.min[1], mesh.max[1]]) for (const cz of [mesh.min[2], mesh.max[2]]) {
    const w = rotate(rotation, [cx, cy, cz]);
    for (let axis = 0; axis < 3; axis += 1) { worldMin[axis] = Math.min(worldMin[axis], w[axis] + position[axis]); worldMax[axis] = Math.max(worldMax[axis], w[axis] + position[axis]); }
  }
  return { id, mesh, position, rotation, worldMin, worldMax };
}

/** Deepest reach of `a`'s surface into `b`'s solid. */
function reach(a: Body, b: Body): number {
  let deepest = 0;
  const inverse = conj(b.rotation);
  for (const sample of a.mesh.samples) {
    const world = rotate(a.rotation, sample);
    const w: V3 = [world[0] + a.position[0], world[1] + a.position[1], world[2] + a.position[2]];
    if (w[0] < b.worldMin[0] || w[0] > b.worldMax[0] || w[1] < b.worldMin[1] || w[1] > b.worldMax[1] || w[2] < b.worldMin[2] || w[2] > b.worldMax[2]) continue;
    const local = rotate(inverse, [w[0] - b.position[0], w[1] - b.position[1], w[2] - b.position[2]]);
    if (!inside(b.mesh, local)) continue;
    let nearest = Infinity;
    for (let t = 0; t < b.mesh.count; t += 1) nearest = Math.min(nearest, pointTriangleDistance(local, b.mesh.tris, 9 * t));
    deepest = Math.max(deepest, nearest);
  }
  return deepest;
}

function penetrations(bodies: Body[]): { a: string; b: string; depth: number }[] {
  const found: { a: string; b: string; depth: number }[] = [];
  for (let i = 0; i < bodies.length; i += 1) {
    for (let j = i + 1; j < bodies.length; j += 1) {
      const [a, b] = [bodies[i], bodies[j]];
      if (a.worldMax[0] < b.worldMin[0] || b.worldMax[0] < a.worldMin[0] || a.worldMax[1] < b.worldMin[1] || b.worldMax[1] < a.worldMin[1] || a.worldMax[2] < b.worldMin[2] || b.worldMax[2] < a.worldMin[2]) continue;
      const coincident = Math.hypot(a.position[0] - b.position[0], a.position[1] - b.position[1], a.position[2] - b.position[2]) < 1e-4 && Math.abs(a.rotation[0] * b.rotation[0] + a.rotation[1] * b.rotation[1] + a.rotation[2] * b.rotation[2] + a.rotation[3] * b.rotation[3]) > 1 - 1e-6 && a.mesh === b.mesh;
      const depth = coincident ? Infinity : Math.max(reach(a, b), reach(b, a));
      if (depth > 0) found.push({ a: a.id, b: b.id, depth });
    }
  }
  return found.sort((x, y) => y.depth - x.depth);
}

const catalog = JSON.parse(readFileSync(`${repo}/🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json`, "utf8")) as { entries: { url: string; source: string }[] };
const meshCache = new Map<string, Mesh>();
function meshForUrl(url: string): Mesh {
  const entry = catalog.entries.find((candidate) => candidate.url === url);
  if (!entry) throw new Error(`no mesh catalog entry for ${url}`);
  return meshCache.get(url) ?? meshCache.set(url, loadGlb(`${repo}/${entry.source}`)).get(url)!;
}
const urls = catalog.entries.map((e) => e.url).filter((u) => /concrete-forest/.test(u));
const out: Record<string, number[]> = {};
for (const url of urls) out[url] = Array.from(meshForUrl(url).tris);
writeFileSync(`${import.meta.dir}/🗑️generated/W5-mac-react-e2e/debug-meshes.json`, JSON.stringify(out));
console.log(Object.entries(out).map(([k, v]) => `${k} ${v.length / 9}`).join("\n"));
