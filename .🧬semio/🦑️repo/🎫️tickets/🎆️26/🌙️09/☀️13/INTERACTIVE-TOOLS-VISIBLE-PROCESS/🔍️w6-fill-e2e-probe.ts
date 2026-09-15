/** 🩺️ (semio-91) End-to-end gate of the puzzle 3d fill tool run in the React shell: an uninterrupted run to Complete, Finalize,
 * Undo, a mid-run Abort and a mid-run Finalize — and for every finalized result the committed pieces, measured on the REAL GLB
 * geometry the renderer draws (GLB frame +90° X, then the instance pose), must not penetrate each other beyond the contact
 * tolerance. Penetration is how far one body's surface reaches into the other solid, to that solid's nearest surface (flush
 * docking reads 0, the docking host counts, coincident bodies collide).
 * Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS. Run from the ticket folder:
 * `bun 🔍️w6-fill-e2e-probe.ts [--port=6014] [--count=200] [--tolerance=0.005] [--only=complete,abort,finalize]`. */
import { chromium, type Page } from "@playwright/test";
import { readFileSync, writeFileSync } from "node:fs";

const arg = (name: string, fallback: string) => process.argv.find((entry) => entry.startsWith(`--${name}=`))?.slice(name.length + 3) ?? fallback;
const port = arg("port", "6014");
const count = arg("count", "200");
const tolerance = Number(arg("tolerance", "0.005"));
const only = new Set(arg("only", "complete,abort,finalize").split(","));
const repo = `${import.meta.dir}/../../../../../../..`;
const out = `${import.meta.dir}/🗑️generated/W5-mac-react-e2e`;

//#region 🧊️Geometry
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
//#endregion

if (process.argv.includes("--selftest")) {
  // Known answers on the real left mesh: a copy shoved along +x by its full extent docks flush (0), shoved 10 cm less
  // penetrates ~10 cm, a coincident copy collides, and a copy 1 m above the top clears.
  const url = "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb";
  const mesh = meshForUrl(url);
  const extent: V3 = [mesh.max[0] - mesh.min[0], mesh.max[1] - mesh.min[1], mesh.max[2] - mesh.min[2]];
  const identity: Quat = [0, 0, 0, 1];
  const base = body("base", mesh, [0, 0, 0], identity);
  const cases: [string, V3, Quat][] = [
    ["flush +x", [extent[0], 0, 0], identity],
    ["flush +z", [0, 0, extent[2]], identity],
    ["10 cm into +z", [0, 0, extent[2] - 0.1], identity],
    ["2 mm into +z", [0, 0, extent[2] - 0.002], identity],
    ["coincident", [0, 0, 0], identity],
    ["1 m above", [0, 0, extent[2] + 1], identity],
  ];
  console.log(`mesh ${mesh.count} tris, ${mesh.samples.length} samples, local min ${mesh.min.map((v) => v.toFixed(3))} max ${mesh.max.map((v) => v.toFixed(3))}`);
  for (const [name, offset, rotation] of cases) {
    const found = penetrations([base, body(name, mesh, offset, rotation)]);
    console.log(`${name}: ${found.length ? (Number.isFinite(found[0].depth) ? `${(found[0].depth * 100).toFixed(2)} cm` : "coincident") : "clear"}`);
  }
  // Is the mesh closed? A point well inside its bounding box core should be inside, one outside should not.
  const center: V3 = [(mesh.min[0] + mesh.max[0]) / 2, (mesh.min[1] + mesh.max[1]) / 2, (mesh.min[2] + mesh.max[2]) / 2];
  let insideCount = 0;
  const grid = 12;
  for (let i = 0; i < grid; i += 1) for (let j = 0; j < grid; j += 1) for (let k = 0; k < grid; k += 1) insideCount += Number(inside(mesh, [mesh.min[0] + extent[0] * (i + 0.5) / grid, mesh.min[1] + extent[1] * (j + 0.5) / grid, mesh.min[2] + extent[2] * (k + 0.5) / grid]));
  console.log(`inside fraction of bounding box ${(insideCount / grid ** 3).toFixed(3)}, center inside ${inside(mesh, center)}`);
  process.exit(0);
}

//#region 🖱️Page
type Instance = { id: string; meshId: string; position: V3; rotation: Quat; scale: V3; provisional?: boolean };
type Sample = { t: number; panel: string; finalizeDisabled: boolean | null; abortDisabled: boolean | null; committed: number; provisional: number; records: number; testing: number; success: number; warning: number; danger: number };

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
const t0 = Date.now();
const consoleLines: string[] = [];
let intakeRejected = 0, pageErrors = 0, consoleErrors = 0;
const memoryLines: string[] = [];
if (process.argv.includes("--diag")) await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (msg) => {
  const text = msg.text();
  if (/guest linear memory turn=|performInvocation \{"invocationKind":"action".*toolRun|allocation of|install-peak/.test(text)) memoryLines.push(`+${Date.now() - t0}ms ${text.slice(0, 260)}`);
  if (/intake-rejected/.test(text)) intakeRejected += 1;
  if (msg.type() === "error" && !/\[DEBUG\]/.test(text)) { consoleErrors += 1; consoleLines.push(`+${Date.now() - t0}ms error ${text.slice(0, 400)}`); }
});
page.on("pageerror", (error) => { pageErrors += 1; consoleLines.push(`+${Date.now() - t0}ms pageerror ${String(error).slice(0, 400)}`); });

const PERSPECTIVE = '[data-surface-id="window:puzzle3d-main-perspective"]';
const sample = (start: number): Promise<Sample> =>
  page.evaluate(([start, selector]) => {
    const panel = document.querySelector('[id^="panel:framework.toolRun"]') as HTMLElement | null;
    const button = (name: string) => [...(panel?.querySelectorAll("button") ?? [])].find((node) => node.textContent?.trim() === name) as HTMLButtonElement | undefined;
    const state = (node: HTMLButtonElement | undefined) => (node ? node.disabled || node.getAttribute("aria-disabled") === "true" : null);
    const win = document.querySelector(selector as string);
    let instances: { provisional?: boolean }[] = [];
    try { instances = JSON.parse(win?.getAttribute("data-instances-json") ?? "[]"); } catch {}
    return { t: Date.now() - (start as number), panel: (panel?.innerText ?? "").split("\n").filter(Boolean).slice(0, 3).join(" | "), finalizeDisabled: state(button("Finalize")), abortDisabled: state(button("Abort")), committed: instances.filter((instance) => !instance.provisional).length, provisional: instances.filter((instance) => instance.provisional).length, records: Number(win?.getAttribute("data-tool-run-records") ?? 0), testing: Number(win?.getAttribute("data-tool-run-testing") ?? 0), success: Number(win?.getAttribute("data-tool-run-success") ?? 0), warning: Number(win?.getAttribute("data-tool-run-warning") ?? 0), danger: Number(win?.getAttribute("data-tool-run-danger") ?? 0) };
  }, [start, PERSPECTIVE] as const);
const committedInstances = (): Promise<{ instances: Instance[]; meshes: { id: string; url?: string; kind?: string }[] }> =>
  page.evaluate((selector) => {
    const win = document.querySelector(selector);
    return { instances: JSON.parse(win?.getAttribute("data-instances-json") ?? "[]").filter((instance: { provisional?: boolean }) => !instance.provisional), meshes: JSON.parse(win?.getAttribute("data-meshes-json") ?? "[]") };
  }, PERSPECTIVE);
const panelButton = (name: string) => page.locator('[id^="panel:framework.toolRun"] button', { hasText: name }).first();
async function waitPanel(pattern: RegExp, start: number, limitMs: number, trail: string[], label: string): Promise<Sample | null> {
  let last = "";
  while (Date.now() - start < limitMs) {
    const now = await sample(start);
    const key = JSON.stringify({ ...now, t: 0 });
    if (key !== last) trail.push(`${label} ${JSON.stringify(now)}`);
    last = key;
    if (pattern.test(now.panel)) return now;
    await page.waitForTimeout(200);
  }
  return null;
}
//#endregion

const verdicts: { gate: string; pass: boolean; evidence: string }[] = [];
const trail: string[] = [];
const gate = (name: string, pass: boolean, evidence: string) => { verdicts.push({ gate: name, pass, evidence }); trail.push(`${pass ? "PASS" : "FAIL"} ${name}: ${evidence}`); };

async function geometryGate(label: string, baselineIds: Set<string>) {
  const { instances, meshes } = await committedInstances();
  const bodies: Body[] = [];
  const unresolved: string[] = [];
  for (const instance of instances) {
    const url = meshes.find((mesh) => mesh.id === instance.meshId)?.url;
    if (!url) { unresolved.push(`${instance.id}→${instance.meshId}`); continue; }
    if (instance.scale.some((value) => Math.abs(value - 1) > 1e-9)) { unresolved.push(`${instance.id} scale ${instance.scale}`); continue; }
    bodies.push(body(instance.id, meshForUrl(url), instance.position, instance.rotation));
  }
  gate(`${label}: every committed piece renders a catalog mesh`, unresolved.length === 0, unresolved.length ? unresolved.slice(0, 5).join(", ") : `${bodies.length} bodies`);
  const started = Date.now();
  const found = penetrations(bodies);
  if (process.argv.includes("--dump")) writeFileSync(`${out}/w6-committed-${label.replace(/[^a-z]+/gi, "-")}-${Date.now()}.json`, JSON.stringify({ instances, meshes, found: found.slice(0, 50) }, null, 1));
  const violations = found.filter((entry) => entry.depth > tolerance + 1e-4);
  const involvesNew = violations.filter((entry) => !baselineIds.has(entry.a) || !baselineIds.has(entry.b));
  gate(`${label}: no committed pieces penetrate beyond ${tolerance} m`, involvesNew.length === 0, `${bodies.length} bodies, ${found.length} touching pairs, ${violations.length} over tolerance (${involvesNew.length} involving placed pieces) in ${Date.now() - started} ms; deepest ${violations.slice(0, 6).map((entry) => `${entry.a.slice(-8)}/${entry.b.slice(-8)} ${Number.isFinite(entry.depth) ? `${(entry.depth * 100).toFixed(1)} cm` : "coincident"}`).join(", ") || "none"}`);
}

await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`);
await page.waitForFunction(() => document.querySelectorAll("[data-tool-run-records]").length >= 1, undefined, { timeout: 600000 });
await page.waitForTimeout(Number(arg("boot-wait-ms", "8000")));
await page.getByText("Skip", { exact: true }).first().click({ force: true, timeout: 3000 }).catch(() => {});
const tab = page.getByRole("button", { name: "Tool runs", exact: true }).first();
if ((await tab.getAttribute("aria-pressed")) === "false") await tab.click();
const utility = () => page.evaluate((selector) => (JSON.parse(document.querySelector(selector)?.getAttribute("data-interaction-json") ?? "{}") as { activeUtility?: string }).activeUtility, PERSPECTIVE);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Tool", exact: true }).first().click();
await page.waitForTimeout(3000);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Fill", exact: true }).first().click();
const spin = page.getByRole("spinbutton").first();
await spin.fill(count);
await spin.press("Enter");
await page.waitForTimeout(2000);
const baseline = await committedInstances();
const baselineIds = new Set(baseline.instances.map((instance) => instance.id));
trail.push(`meshes at start ${JSON.stringify(baseline.meshes)}`);
gate("boot: fill tool ready", /Ready to start/.test((await sample(Date.now())).panel), `panel "${(await sample(Date.now())).panel}", ${baseline.instances.length} committed`);
await geometryGate("baseline document", new Set());

if (only.has("complete")) {
  const start = Date.now();
  // Every animation frame records the provisional count the renderer holds, so a fast run's intermediate states are seen.
  await page.evaluate((selector) => {
    const counts = new Set<number>();
    (window as unknown as { __semio91Counts: Set<number> }).__semio91Counts = counts;
    const tick = () => {
      try { counts.add(JSON.parse(document.querySelector(selector)?.getAttribute("data-instances-json") ?? "[]").filter((instance: { provisional?: boolean }) => instance.provisional).length); } catch {}
      if (counts.size < 100000) requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
  }, PERSPECTIVE);
  await page.getByRole("button", { name: "Start", exact: true }).first().click({ timeout: 30000 });
  const progress = new Set<number>();
  let complete: Sample | null = null;
  let last = "";
  while (Date.now() - start < Number(arg("run-limit-ms", "300000"))) {
    const now = await sample(start);
    progress.add(now.provisional);
    const key = JSON.stringify({ ...now, t: 0 });
    if (key !== last) trail.push(`complete-run ${JSON.stringify(now)}`);
    last = key;
    if (/^Complete/i.test(now.panel)) { complete = now; break; }
    await page.waitForTimeout(200);
  }
  if (process.argv.includes("--dump")) writeFileSync(`${out}/w6-provisional-at-complete-${Date.now()}.json`, JSON.stringify(await page.evaluate((selector) => JSON.parse(document.querySelector(selector)?.getAttribute("data-instances-json") ?? "[]"), PERSPECTIVE), null, 1));
  gate("run: reaches Complete", complete !== null, complete ? `after ${complete.t} ms with ${complete.provisional} provisional` : "never completed in 300 s");
  for (const value of await page.evaluate(() => [...(window as unknown as { __semio91Counts: Set<number> }).__semio91Counts])) progress.add(value);
  gate("run: the process is visible while it runs", progress.size >= 4, `${progress.size} distinct provisional counts: ${[...progress].sort((a, b) => a - b).join(",")}`);
  gate("run: every requested piece is provisional at Complete", complete?.provisional === Number(count), `${complete?.provisional} of ${count}`);
  const clicked = Date.now();
  await panelButton("Finalize").click({ timeout: 5000 });
  const finalized = await waitPanel(/^Finalized/, clicked, 60000, trail, "finalize");
  const after = await sample(clicked);
  gate("finalize: commits the run", finalized !== null && after.committed === baseline.instances.length + Number(count) && after.provisional === 0, `${finalized ? `Finalized at +${finalized.t} ms` : "never Finalized"}; committed ${after.committed} (expected ${baseline.instances.length + Number(count)}), provisional ${after.provisional}`);
  gate("finalize: no tested candidate lingers over the committed pieces", after.success <= 1 && after.testing === 0, `${after.records} trace records: testing ${after.testing}, success ${after.success}, warning ${after.warning}, danger ${after.danger} (marked vortices stay until Dismiss)`);
  await geometryGate("finalize (complete run)", baselineIds);
  await panelButton("Dismiss").click({ timeout: 3000 }).catch(() => {});
  let dismissed = await sample(clicked);
  for (let i = 0; i < 25 && dismissed.records > 0; i += 1) { await page.waitForTimeout(200); dismissed = await sample(clicked); }
  gate("dismiss: the run's trace leaves the scene", dismissed.records === 0, `${dismissed.records} trace records after Dismiss, panel "${dismissed.panel}"`);
  await page.keyboard.press("Meta+z");
  let undone = await sample(clicked);
  for (let i = 0; i < 50 && undone.committed !== baseline.instances.length; i += 1) { await page.waitForTimeout(200); undone = await sample(clicked); }
  gate("undo: one undo removes the whole run", undone.committed === baseline.instances.length && undone.provisional === 0, `committed ${undone.committed}, provisional ${undone.provisional}`);
}

if (only.has("abort")) {
  const start = Date.now();
  await page.getByRole("button", { name: "Start", exact: true }).first().click({ timeout: 30000 });
  await page.waitForFunction((selector) => JSON.parse(document.querySelector(selector)?.getAttribute("data-instances-json") ?? "[]").some((instance: { provisional?: boolean }) => instance.provisional), PERSPECTIVE, { timeout: 60000 }).catch(() => {});
  const before = await sample(start);
  const clicked = Date.now();
  await panelButton("Abort").click({ timeout: 5000 });
  const aborted = await waitPanel(/^Aborted/, clicked, 30000, trail, "abort");
  const after = await sample(clicked);
  gate("abort: mid-run abort changes nothing", aborted !== null && after.provisional === 0 && after.committed === baseline.instances.length, `${before.provisional} provisional at the click; ${aborted ? `"${aborted.panel}" at +${aborted.t} ms` : "never Aborted"}; committed ${after.committed}, provisional ${after.provisional}`);
  await panelButton("Dismiss").click({ timeout: 3000 }).catch(() => {});
  await page.waitForTimeout(1000);
}

if (only.has("finalize")) {
  const start = Date.now();
  await page.getByRole("button", { name: "Start", exact: true }).first().click({ timeout: 30000 });
  await page.waitForFunction((selector) => JSON.parse(document.querySelector(selector)?.getAttribute("data-instances-json") ?? "[]").filter((instance: { provisional?: boolean }) => instance.provisional).length >= 3, PERSPECTIVE, { timeout: 60000 }).catch(() => {});
  const before = await sample(start);
  const clicked = Date.now();
  await panelButton("Finalize").click({ timeout: 5000 });
  const finalized = await waitPanel(/^Finalized/, clicked, 60000, trail, "partial-finalize");
  const after = await sample(clicked);
  gate("finalize mid-run: commits the partial result", finalized !== null && after.provisional === 0 && after.committed > baseline.instances.length, `${before.provisional} provisional at the click; ${finalized ? `Finalized at +${finalized.t} ms` : "never Finalized"}; committed ${after.committed}, provisional ${after.provisional}`);
  await geometryGate("finalize (mid-run)", baselineIds);
  await panelButton("Dismiss").click({ timeout: 3000 }).catch(() => {});
  await page.waitForTimeout(800);
  await page.keyboard.press("Meta+z");
  await page.waitForTimeout(3000);
}

gate("console: no intake rejections", intakeRejected === 0, `${intakeRejected}`);
gate("console: no page errors or non-debug console errors", pageErrors === 0 && consoleErrors === 0, `${pageErrors} page errors, ${consoleErrors} console errors`);
await page.screenshot({ path: `${out}/w6-e2e-final-${port}.png` });
const failed = verdicts.filter((verdict) => !verdict.pass).length;
const summary = `fill e2e :${port} count=${count} tolerance=${tolerance}: PASS=${verdicts.length - failed} FAIL=${failed}`;
const report = [summary, "", ...verdicts.map((verdict) => `${verdict.pass ? "PASS" : "FAIL"} | ${verdict.gate} | ${verdict.evidence}`), "", "TRAIL", ...trail, "", "CONSOLE", ...consoleLines, "", "MEMORY", ...memoryLines].join("\n");
writeFileSync(`${out}/w6-fill-e2e-${port}-${Date.now()}.txt`, report);
console.log(report.split("\nTRAIL")[0]);
await browser.close();
process.exit(failed === 0 ? 0 : 1);
