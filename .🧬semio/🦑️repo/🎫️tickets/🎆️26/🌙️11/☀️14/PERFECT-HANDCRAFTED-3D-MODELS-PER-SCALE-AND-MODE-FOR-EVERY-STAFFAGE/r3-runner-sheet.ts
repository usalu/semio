/** 🎞️ Contact sheet of humans/runner-with-blade-prosthesis for the round-3 author: one list of one scale in one pose from several true (unmirrored) directions, optionally with the rig drawn over it, or the three 2D overlays of that pose.
 *
 * Usage: `bun r3-runner-sheet.ts [--asset work|catalog] [--scale 50] [--list body|thermal|structure|overlay] [--pose id] [--dirs 0:0,90:0,180:0,270:0,35:20,35:60] [--rig] [--y 1.2:1.85] [--cut] [--pw 300] [--ph 900] [--name x]`
 * Directions are `azimuth:elevation` of the product axonometric camera: azimuth 0 = front (viewer ahead, the figure's left on paper right), 90 = right side (the side elevation), 180 = back, 270 = left side.
 * Output: `🗑️generated/r3-runner-with-blade-prosthesis/<name>.png`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { Canvas, type RGB } from "./volumes-raster.ts";
import { bodyItems, drawAxon, structureItems, thermalItems, type Axon } from "./volumes-render.ts";
import { drawnSet } from "./volumes-silhouette.ts";
import { addMeshes, emptySet, rasterize } from "./volumes-tiles.ts";
import { GENERATED, VIEWS, entryOf, loadSource, merged, poseState, scaleMeshes, type Json, type SolidMesh } from "./volumes-lib.ts";

export const SPEC = "humans/runner-with-blade-prosthesis";
export const OUT = join(GENERATED, "r3-runner-with-blade-prosthesis");
export const WORK = join(OUT, "🔣️.json");

export function loadWork(which: string): Json {
  if (which === "catalog") {
    const { entry, asset } = entryOf(SPEC);
    return merged(asset, loadSource(entry)!);
  }
  return JSON.parse(readFileSync(WORK, "utf8")).asset;
}

type V = [number, number, number];
const RAD = Math.PI / 180;
const cam = (azDeg: number, elDeg: number): { r: V; u: V; e: V } => {
  const a = azDeg * RAD;
  const e = elDeg * RAD;
  return { r: [Math.sin(a), 0, Math.cos(a)], u: [-Math.sin(e) * Math.cos(a), Math.cos(e), Math.sin(e) * Math.sin(a)], e: [Math.cos(e) * Math.cos(a), Math.sin(e), -Math.cos(e) * Math.sin(a)] };
};

if (import.meta.main) {
  const args = process.argv.slice(2);
  const flag = (n: string, d?: string) => (args.indexOf(n) < 0 ? d : args[args.indexOf(n) + 1]);
  const scale = Number(flag("--scale", "50"));
  const list = flag("--list", "body")!;
  const raw = loadWork(flag("--asset", "work")!);
  const poses = flag("--poses") === undefined ? undefined : flag("--poses")!.split(",");
  const pose = poses?.[0] ?? flag("--pose", raw.presentation.defaultPose)!;
  const posed = (id: string): Json => ({ ...raw, presentation: { ...raw.presentation, defaultPose: id } });
  const asset = posed(pose);
  const dirs0 = flag("--dirs", "0:0,90:0,180:0,270:0,35:20,215:20")!.split(",").map((d) => d.split(":").map(Number) as [number, number]);
  const dirs = poses === undefined ? dirs0 : poses.map(() => dirs0[0]!);
  const [pw, ph] = [Number(flag("--pw", "300")), Number(flag("--ph", "900"))];
  const yr = flag("--y") === undefined ? undefined : flag("--y")!.split(":").map(Number);
  const xr = flag("--x") === undefined ? undefined : flag("--x")!.split(":").map(Number);
  let sm = scaleMeshes(asset, scale, asset.volumes[String(scale)]);
  let frames = poseState(asset, scale).frames;
  const name = flag("--name", `${list}.s${scale}.${pose}`)!;
  mkdirSync(OUT, { recursive: true });
  const AA = 2;
  if (list === "overlay") {
    const sheet = new Canvas(pw * 3, ph, [255, 255, 255]);
    VIEWS.forEach((view, vi) => {
      const vol = emptySet();
      addMeshes(vol, view, sm.body.filter((_, i) => sm.solids.body[i]!.texture !== true));
      const dr = drawnSet(asset, scale, view, vol);
      const box = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
      const grow = (x: number, y: number) => { box.minX = Math.min(box.minX, x); box.maxX = Math.max(box.maxX, x); box.minY = Math.min(box.minY, y); box.maxY = Math.max(box.maxY, y); };
      for (const s of [vol, dr]) {
        for (let i = 0; i < s.tris.length; i += 2) grow(s.tris[i]!, s.tris[i + 1]!);
        for (const p of s.polys) { grow(p.box.minX, p.box.minY); grow(p.box.maxX, p.box.maxY); }
      }
      const cell = Math.max((box.maxX - box.minX) / (pw - 20), (box.maxY - box.minY) / (ph - 40));
      const frame = { minX: box.minX - 2 * cell, minY: box.minY - 2 * cell, maxX: box.maxX + 2 * cell, maxY: box.maxY + 2 * cell };
      const a = rasterize(dr, frame, cell);
      const b = rasterize(vol, frame, cell);
      for (let y = 0; y < a.height; y++) for (let x = 0; x < a.width; x++) {
        const i = y * a.width + x;
        if (a.cells[i] || b.cells[i]) sheet.set(vi * pw + 8 + x, ph - 12 - y, a.cells[i] && b.cells[i] ? [150, 205, 160] : a.cells[i] ? [225, 70, 70] : [60, 110, 230]);
      }
      sheet.text(vi * pw + 6, 6, `${view} 1:${scale} ${pose}`, [30, 30, 30], 1);
      sheet.frame(vi * pw, 0, pw, ph, [190, 190, 185]);
    });
    writeFileSync(join(OUT, `${name}.png`), sheet.png());
    process.stdout.write(`${join(OUT, `${name}.png`)}\n`);
    process.exit(0);
  }
  let bodyI = bodyItems(asset, sm, scale);
  let items = list === "body" ? bodyI : list === "thermal" ? thermalItems(asset, sm, scale) : structureItems(asset, sm, scale);
  let meshes: SolidMesh[] = [...sm.body, ...sm.thermal, ...sm.structure];
  const sheet = new Canvas(pw * dirs.length, ph, [255, 255, 255]);
  dirs.forEach(([az, el], di) => {
    if (poses !== undefined && di > 0) {
      const a2 = posed(poses[di]!);
      sm = scaleMeshes(a2, scale, a2.volumes[String(scale)]);
      frames = poseState(a2, scale).frames;
      bodyI = bodyItems(a2, sm, scale);
      items = list === "body" ? bodyI : list === "thermal" ? thermalItems(a2, sm, scale) : structureItems(a2, sm, scale);
      meshes = [...sm.body, ...sm.thermal, ...sm.structure];
    }
    const c = cam(az, el);
    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
    for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) {
      const [X, Y, Z] = [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!];
      if (yr !== undefined && (Y < yr[0]! || Y > yr[1]!)) continue;
      if (xr !== undefined && (X < xr[0]! || X > xr[1]!)) continue;
      const x = X * c.r[0] + Y * c.r[1] + Z * c.r[2];
      const y = X * c.u[0] + Y * c.u[1] + Z * c.u[2];
      minX = Math.min(minX, x); maxX = Math.max(maxX, x); minY = Math.min(minY, y); maxY = Math.max(maxY, y);
    }
    const [w, h] = [pw * AA, ph * AA];
    const s = Math.min((w * 0.92) / (maxX - minX || 1), (h * 0.94) / (maxY - minY || 1));
    const ax: Axon = { cam: c, scale: s, cx: (minX + maxX) / 2, cy: (minY + maxY) / 2, cz: 0, ox: w / 2, oy: h / 2, zc: 0 };
    const big = new Canvas(w, h, [247, 247, 244]);
    const cut = args.includes("--cut");
    if (list !== "body") {
      drawAxon(big, ax, bodyI, { cut: false });
      for (let i = 0; i < big.width * big.height; i++) if (big.depth[i]! > -Infinity) {
        for (let k = 0; k < 3; k++) big.data[i * 3 + k] = 247 - (247 - big.data[i * 3 + k]!) * 0.2;
        big.depth[i] = -Infinity;
      }
    }
    drawAxon(big, ax, items, { cut, clearDepth: list === "body" });
    if (args.includes("--rig")) for (const b of asset.rig.bones as Json[]) {
      const f = frames[b.id]!;
      const p = (q: V): [number, number] => [ax.ox + (q[0] * c.r[0] + q[1] * c.r[1] + q[2] * c.r[2] - ax.cx) * s, ax.oy - (q[0] * c.u[0] + q[1] * c.u[1] + q[2] * c.u[2] - ax.cy) * s];
      const a = p(f.o as V);
      const t = p([f.o[0] + f.u[0] * b.lengthM, f.o[1] + f.u[1] * b.lengthM, f.o[2] + f.u[2] * b.lengthM]);
      for (let d = -2; d <= 2; d++) { big.line(a[0] + d, a[1], t[0] + d, t[1], [230, 30, 30]); big.line(a[0], a[1] + d, t[0], t[1] + d, [230, 30, 30]); }
      for (let dx = -6; dx <= 6; dx++) for (let dy = -6; dy <= 6; dy++) if (dx * dx + dy * dy <= 36) big.set(Math.round(a[0] + dx), Math.round(a[1] + dy), [255, 220, 0]);
    }
    const out = big.downsample(AA);
    out.text(6, 6, `${list} 1:${scale} ${poses?.[di] ?? ""} az ${az} el ${el}`, [30, 30, 30], 1);
    out.frame(0, 0, pw, ph, [190, 190, 185]);
    sheet.blit(out, di * pw, 0);
  });
  writeFileSync(join(OUT, `${name}.png`), sheet.png());
  process.stdout.write(`${join(OUT, `${name}.png`)}\n`);
}
