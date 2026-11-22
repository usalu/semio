/** 🖼️ Renders a PNG contact sheet per scale of a source: silhouette overlays (3D body over the 2D drawing of that scale) and shaded axonometric views of body, thermal (cutaway, colour per region and layer) and structure (members, joints, supports, masses).
 *
 * Usage: `bun volumes-render.ts <category>/<slug> [--scale N] [--list body|thermal|structure] [--az 35] [--el 25] [--no-cut] [--width 1600]`
 * Output: `🗑️generated/volumes-tools/render/<category>/<slug>.s<N>.png` (all three lists, overlays on top) or `<slug>.s<N>.<list>.png` (one list, large). Overlay colours: green = both, red = drawn but not in the 3D body, blue = in the 3D body but not drawn. Thermal: nested layer shells are visible in the cut; structure: members blue, joints orange, supports green, masses magenta, the body as ghost.
 * @see ./volumes-tools.md */
import { writeFileSync, mkdirSync } from "node:fs";
import { dirname } from "node:path";

import { shadedTriangle } from "./digital-shaded-triangle.ts";
import { Canvas, hashHue, hsl, type RGB } from "./volumes-raster.ts";
import { drawnSet, addSectioned } from "./volumes-silhouette.ts";
import { addMeshes, emptySet, rasterize } from "./volumes-tiles.ts";
import { Mask, SCALES, poseState, VIEWS, entryOf, extentOf, featureM, loadSource, merged, outPath, poseOf, scaleMeshes, type Json, type ListName, type SolidMesh } from "./volumes-lib.ts";

type Item = { mesh: SolidMesh & {normals?: Float32Array}; color: RGB; cutAxes?: [number, number, number][]; slender?: boolean };
const clamp = (c: number) => Math.max(0, Math.min(255, c));

const AX: Record<string, [number, number]> = { "-u": [0, -1], "+u": [0, 1], "-v": [1, -1], "+v": [1, 1], "-w": [2, -1], "+w": [2, 1] };

/** ✂️ The world directions of the declared section faces of a solid (C1) and whether it is slender (C2). */
export function marks(asset: Json, scale: number, solid: Json): Pick<Item, "cutAxes" | "slender"> {
  const f = poseState(asset, scale).frames[solid.bone];
  const axes = f === undefined ? [] : ((solid.sectionFaces ?? []) as string[]).map((k) => (f[(["u", "v", "w"] as const)[AX[k]![0]]] as number[]).map((x) => x * AX[k]![1]) as [number, number, number]);
  return { ...(axes.length > 0 ? { cutAxes: axes } : {}), ...(solid.slender === true ? { slender: true } : {}) };
}

export function bodyItems(asset: Json, sm: ReturnType<typeof scaleMeshes>, scale = 0): Item[] {
  return sm.body.map((mesh, i) => {
    const s = sm.solids.body[i]!;
    const g = 70 + (s.tone ?? 0.5) * 150;
    const tint = hsl(hashHue(s.material ?? ""), 0.3, 0.5);
    const base: RGB = [clamp(g * 0.7 + tint[0] * 0.3), clamp(g * 0.7 + tint[1] * 0.3), clamp(g * 0.7 + tint[2] * 0.3)];
    return { mesh, color: s.slender === true ? ([255, 190, 0] as RGB) : base, ...marks(asset, scale, s) };
  });
}

export function thermalLegend(asset: Json): { key: string; label: string; color: RGB }[] {
  const out: { key: string; label: string; color: RGB }[] = [];
  (asset.thermal?.regions ?? []).forEach((r: Json, i: number) => {
    const h = (i * 137.5) % 360;
    const layers: Json[] = r.layers ?? [];
    if (layers.length <= 1) out.push({ key: r.id, label: `${r.id}${r.hidden ? " (hidden)" : ""}`, color: hsl(h, 0.65, 0.5) });
    else layers.forEach((l, k) => out.push({ key: `${r.id}/${l.id}`, label: `${r.id}/${l.id}`, color: hsl(h, 0.65, 0.3 + (0.4 * (k + 1)) / layers.length) }));
  });
  return out;
}

export function thermalItems(asset: Json, sm: ReturnType<typeof scaleMeshes>, scale = 0): Item[] {
  const legend = new Map(thermalLegend(asset).map((l) => [l.key, l.color]));
  return sm.thermal.map((mesh, i) => {
    const s = sm.solids.thermal[i]!;
    const color = legend.get(s.layer === undefined ? s.region : `${s.region}/${s.layer}`) ?? legend.get(s.region) ?? ([150, 150, 150] as RGB);
    return { mesh, color, ...marks(asset, scale, s) };
  });
}

export const PART_COLOR: Record<string, RGB> = { member: [70, 120, 210], joint: [240, 140, 30], support: [40, 170, 80], mass: [210, 50, 170] };

export function structureItems(asset: Json, sm: ReturnType<typeof scaleMeshes>, scale = 0): Item[] {
  const alt = new Map<string, number>();
  return sm.structure.map((mesh, i) => {
    const s = sm.solids.structure[i]!;
    const base = PART_COLOR[s.part] ?? [120, 120, 120];
    const k = (alt.get(s.member) ?? alt.set(s.member, alt.size).get(s.member)!) % 2;
    return { mesh, color: base.map((c) => clamp(c * (k === 0 ? 1 : 0.78))) as unknown as RGB, ...marks(asset, scale, s) };
  });
}

type Cam = { r: [number, number, number]; u: [number, number, number]; e: [number, number, number] };
const camera = (azDeg: number, elDeg: number): Cam => {
  const az = (azDeg * Math.PI) / 180;
  const el = (elDeg * Math.PI) / 180;
  const e: [number, number, number] = [Math.sin(az) * Math.cos(el), Math.sin(el), Math.cos(az) * Math.cos(el)];
  const rr = [e[2], 0, -e[0]];
  const l = Math.hypot(...rr);
  const r: [number, number, number] = [rr[0]! / l, 0, rr[2]! / l];
  const u: [number, number, number] = [e[1] * r[2] - e[2] * r[1], e[2] * r[0] - e[0] * r[2], e[0] * r[1] - e[1] * r[0]];
  return { r, u, e };
};

export type Axon = { cam: Cam; scale: number; cx: number; cy: number; cz: number; ox: number; oy: number; zc: number };

export function fit(meshes: readonly SolidMesh[], cam: Cam, w: number, h: number, margin = 0.045): Axon {
  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
  const ext = extentOf(meshes);
  for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) {
    const x = m.world[i]! * cam.r[0] + m.world[i + 1]! * cam.r[1] + m.world[i + 2]! * cam.r[2];
    const y = m.world[i]! * cam.u[0] + m.world[i + 1]! * cam.u[1] + m.world[i + 2]! * cam.u[2];
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
  }
  const scale = Math.min((w * (1 - 2 * margin)) / (maxX - minX || 1), (h * (1 - 2 * margin)) / (maxY - minY || 1));
  return { cam, scale, cx: (minX + maxX) / 2, cy: (minY + maxY) / 2, cz: 0, ox: w / 2, oy: h / 2, zc: ext === null ? 0 : (ext.min[2] + ext.max[2]) / 2 };
}

export function drawAxon(c: Canvas, ax: Axon, items: readonly Item[], opts: { cut?: boolean; alpha?: number; clearDepth?: boolean; unlit?: boolean } = {}): void {
  const { cam } = ax;
  const light = (() => {
    const v = [-0.4, 0.8, 0.5];
    const l = Math.hypot(...v);
    return v.map((x) => x / l);
  })();
  if (opts.clearDepth !== false) c.clearDepth();
  for (const { mesh, color, cutAxes } of items) {
    const w = mesh.world;
    const n = w.length / 3;
    const intensities = mesh.normals && !opts.unlit ? new Float32Array(n) : undefined;
    if (intensities && mesh.normals) for (let i=0; i<n; i++) {
      const normals=mesh.normals, j=i*3;
      intensities[i]=0.43+0.57*Math.max(0,normals[j]!*light[0]!+normals[j+1]!*light[1]!+normals[j+2]!*light[2]!);
    }
    const sx = new Float64Array(n);
    const sy = new Float64Array(n);
    const sd = new Float64Array(n);
    for (let i = 0; i < n; i++) {
      const x = w[i * 3]!, y = w[i * 3 + 1]!, z = w[i * 3 + 2]!;
      sx[i] = ax.ox + (x * cam.r[0] + y * cam.r[1] + z * cam.r[2] - ax.cx) * ax.scale;
      sy[i] = ax.oy - (x * cam.u[0] + y * cam.u[1] + z * cam.u[2] - ax.cy) * ax.scale;
      sd[i] = x * cam.e[0] + y * cam.e[1] + z * cam.e[2];
    }
    for (let t = 0; t < mesh.indices.length; t += 3) {
      const [a, b, d] = [mesh.indices[t]!, mesh.indices[t + 1]!, mesh.indices[t + 2]!];
      const ux = w[b * 3]! - w[a * 3]!, uy = w[b * 3 + 1]! - w[a * 3 + 1]!, uz = w[b * 3 + 2]! - w[a * 3 + 2]!;
      const vx = w[d * 3]! - w[a * 3]!, vy = w[d * 3 + 1]! - w[a * 3 + 1]!, vz = w[d * 3 + 2]! - w[a * 3 + 2]!;
      let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      const nl = Math.hypot(nx, ny, nz) || 1;
      nx /= nl; ny /= nl; nz /= nl;
      const facing = nx * cam.e[0] + ny * cam.e[1] + nz * cam.e[2];
      const lam = Math.abs(nx * light[0]! + ny * light[1]! + nz * light[2]!);
      const k = opts.unlit ? 1 : (0.4 + 0.6 * lam) * (facing >= 0 ? 1 : 0.62);
      const col: RGB = intensities ? color : [clamp(color[0] * k), clamp(color[1] * k), clamp(color[2] * k)];
      const corners=[a,b,d].map(i=>[sx[i]!,sy[i]!,sd[i]!,w[i*3+2]!]);
      if (intensities) {
        shadedTriangle(c,corners,color,[intensities[a]!,intensities[b]!,intensities[d]!],.06/ax.scale);
      } else {
        c.triangle(corners[0]!,corners[1]!,corners[2]!,col,opts.cut ? ax.zc : Infinity,0.06/ax.scale,cutAxes !== undefined && cutAxes.some((q) => nx*q[0]+ny*q[1]+nz*q[2]>=Math.SQRT1_2) ? [255,255,255] : null);
      }
    }
  }
}

const AA = 2;

function axonPanel(w: number, h: number, meshes: readonly SolidMesh[], items: readonly Item[], cam: Cam, cut: boolean, title: string, ghost: readonly Item[] = []): Canvas {
  const big = new Canvas(w * AA, h * AA, [247, 247, 244]);
  const ax = fit(meshes, cam, w * AA, h * AA);
  if (ghost.length > 0) {
    drawAxon(big, ax, ghost, { cut });
    for (let i = 0; i < big.width * big.height; i++) if (big.depth[i]! > -Infinity) {
      for (let k = 0; k < 3; k++) big.data[i * 3 + k] = 247 - (247 - big.data[i * 3 + k]!) * 0.22;
      big.depth[i] = -Infinity;
    }
  }
  drawAxon(big, ax, items, { cut, clearDepth: ghost.length === 0 });
  const out = big.downsample(AA);
  out.text(8, 6, title, [30, 30, 30], 2);
  out.frame(0, 0, w, h, [190, 190, 185]);
  return out;
}

type Numbers = Record<string, number>;

/** 🔢️ The silhouette numbers of the catalog validator for one scale (`iou:view`, `p95M:view`), empty when it is unavailable or slow to answer. */
async function productNumbers(asset: Json, scale: number): Promise<Numbers> {
  try {
    const mod = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
    const out: Numbers = {};
    for (const x of mod.measureVolumes(asset as never, { scale: scale as never, rules: ["volumes-silhouette"] as never })) out[x.name] = x.value;
    return out;
  } catch {
    return {};
  }
}

export function overlayPanel(w: number, h: number, asset: Json, scale: number, view: (typeof VIEWS)[number], sm: ReturnType<typeof scaleMeshes>, numbers: Numbers): Canvas {
  const out = new Canvas(w, h, [247, 247, 244]);
  const body = sm.body.filter((_, i) => sm.solids.body[i]!.texture !== true);
  if (body.length === 0) {
    out.text(8, 6, `${view} 1:${scale} NO BODY`, [170, 20, 20], 2);
    return out;
  }
  const vol = emptySet();
  const section = asset.representations[String(scale)]?.sections?.[view];
  if (section === undefined) addMeshes(vol, view, body);
  else addSectioned(vol, view, body, section.plane);
  const drawnS = drawnSet(asset, scale, view, vol);
  const box = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  const grow = (x: number, y: number) => {
    box.minX = Math.min(box.minX, x);
    box.maxX = Math.max(box.maxX, x);
    box.minY = Math.min(box.minY, y);
    box.maxY = Math.max(box.maxY, y);
  };
  for (const s of [vol, drawnS]) {
    for (let i = 0; i < s.tris.length; i += 2) grow(s.tris[i]!, s.tris[i + 1]!);
    for (const p of s.polys) {
      grow(p.box.minX, p.box.minY);
      grow(p.box.maxX, p.box.maxY);
    }
  }
  const cell = Math.max(box.maxX - box.minX, box.maxY - box.minY) / 640;
  const frame = { minX: box.minX - 2 * cell, minY: box.minY - 2 * cell, maxX: box.maxX + 2 * cell, maxY: box.maxY + 2 * cell };
  const drawn = rasterize(drawnS, frame, cell);
  const volume = rasterize(vol, frame, cell);
  const [mw, mh] = [drawn.width, drawn.height];
  const k = Math.min((w - 16) / mw, (h - 56) / mh);
  const ox = (w - mw * k) / 2;
  const oy = 26 + (h - 52 - mh * k) / 2;
  for (let y = 0; y < Math.ceil(mh * k); y++) for (let x = 0; x < Math.ceil(mw * k); x++) {
    const mx = Math.min(mw - 1, Math.floor(x / k));
    const my = Math.min(mh - 1, Math.floor(y / k));
    const idx = (mh - 1 - my) * mw + mx;
    const a = drawn.cells[idx]!;
    const b = volume.cells[idx]!;
    if (a || b) out.set(Math.round(ox + x), Math.round(oy + y), a && b ? [150, 205, 160] : a ? [225, 70, 70] : [60, 110, 230]);
  }
  const iou = numbers[`iou:${view}`];
  const p95 = numbers[`p95M:${view}`];
  const ok = iou === undefined || iou >= (view === "elevation" ? 0.9 : 0.88);
  out.text(8, 6, `${view} 1:${scale}${iou === undefined ? "" : ` IOU ${iou.toFixed(3)}`}${p95 === undefined ? "" : ` P95 ${(p95 * 1000).toFixed(0)} MM`}${section === undefined ? "" : " SECTION"}`, ok ? [20, 90, 30] : [170, 20, 20], 2);
  out.text(8, h - 14, `POSE ${String(poseOf(asset)).toUpperCase()}  ${iou === undefined ? "NUMBERS: OVERLAY ONLY" : "CATALOG VALIDATOR NUMBERS"}`, [90, 90, 90], 1);
  out.frame(0, 0, w, h, [190, 190, 185]);
  return out;
}

function legendPanel(w: number, entries: { label: string; color: RGB }[], title: string): Canvas {
  const cols = 3;
  const rows = Math.max(1, Math.ceil(entries.length / cols));
  const c = new Canvas(w, 30 + rows * 18, [255, 255, 255]);
  c.text(8, 6, title, [30, 30, 30], 2);
  entries.forEach((e, i) => {
    const x = 8 + (i % cols) * Math.floor((w - 16) / cols);
    const y = 28 + Math.floor(i / cols) * 18;
    c.rect(x, y, 12, 12, e.color);
    c.text(x + 18, y + 1, e.label.slice(0, 38), [40, 40, 40], 1);
  });
  return c;
}

const save = (path: string, c: Canvas) => {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, c.png());
};

/** 🧭️ A wall-mounted asset (support anchors well to one side of the body along x) is looked at from the side away from the wall, so that its front details face the camera. */
export function autoAzimuth(asset: Json, scale: number, sm: ReturnType<typeof scaleMeshes>): number {
  const e = extentOf(sm.body);
  const f = poseState(asset, scale).frames;
  const pts: number[] = [];
  for (const id of asset.structure?.supports ?? []) {
    const a = (asset.anchors ?? []).find((x: Json) => x.id === id);
    const fr = a === undefined ? undefined : f[a.bone];
    if (a !== undefined && fr !== undefined) pts.push(fr.o[0] + a.x * fr.u[0] + a.y * fr.v[0]);
  }
  if (e === null || pts.length === 0) return 35;
  const dx = pts.reduce((n, x) => n + x, 0) / pts.length - (e.min[0] + e.max[0]) / 2;
  return Math.abs(dx) > 0.1 * (e.max[0] - e.min[0]) ? (dx > 0 ? -35 : 35) : 35;
}

export function renderScale(asset: Json, scale: number, numbers: Numbers, lists: readonly ListName[], opts: { az?: number; el: number; cut: boolean; width: number }, paths: (list: ListName | "sheet") => string): string[] {
  const sm = scaleMeshes(asset, scale, asset.volumes[String(scale)]);
  const cam = camera(opts.az === undefined ? autoAzimuth(asset, scale, sm) : opts.az, opts.el);
  const written: string[] = [];
  const everything = [...sm.body, ...sm.thermal, ...sm.structure];
  const bodyI = bodyItems(asset, sm, scale);
  const nSlender = [...sm.solids.body, ...sm.solids.thermal, ...sm.solids.structure].filter((x) => x.slender === true).length;
  const nCut = [...sm.solids.body, ...sm.solids.thermal, ...sm.solids.structure].filter((x) => (x.sectionFaces ?? []).length > 0).length;
  const legendFor = (l: ListName): { label: string; color: RGB }[] => (l === "thermal" ? thermalLegend(asset) : l === "structure" ? Object.entries(PART_COLOR).map(([label, color]) => ({ label, color })) : []);
  const panel = (l: ListName, w: number, h: number): Canvas => {
    if (l === "body") return axonPanel(w, h, everything, bodyI, cam, false, `BODY ${sm.body.length} SOLIDS${nSlender > 0 ? ` (${nSlender} SLENDER = YELLOW)` : ""}${nCut > 0 ? " HATCH = SECTION FACE" : ""}`);
    if (l === "thermal") return axonPanel(w, h, everything, thermalItems(asset, sm, scale), cam, opts.cut, `THERMAL ${sm.thermal.length} SOLIDS${opts.cut ? " CUTAWAY" : ""}`, opts.cut ? bodyI : []);
    return axonPanel(w, h, everything, structureItems(asset, sm, scale), cam, false, `STRUCTURE ${sm.structure.length} SOLIDS`, bodyI);
  };
  if (lists.length === 1) {
    const l = lists[0]!;
    const W = opts.width;
    const main = panel(l, W, Math.round(W * 0.62));
    const leg = legendFor(l);
    const legend = legendPanel(W, leg, l === "structure" ? "STRUCTURE PARTS" : l === "thermal" ? "THERMAL REGIONS AND LAYERS" : "");
    const sheet = new Canvas(W, main.height + (leg.length > 0 ? legend.height : 0), [255, 255, 255]);
    sheet.blit(main, 0, 0);
    if (leg.length > 0) sheet.blit(legend, 0, main.height);
    save(paths(l), sheet);
    written.push(paths(l));
    return written;
  }
  const W = opts.width;
  const pw = Math.floor(W / 3);
  const ph = Math.round(pw * 0.8);
  const top = VIEWS.map((v) => overlayPanel(pw, Math.round(pw * 0.62), asset, scale, v, sm, numbers));
  const row = lists.map((l) => panel(l, pw, ph));
  const legend = legendPanel(W, [...thermalLegend(asset).map((e) => ({ ...e, label: `T ${e.label}` })), ...Object.entries(PART_COLOR).map(([label, color]) => ({ label: `S ${label}`, color }))], "LEGEND  OVERLAY GREEN=BOTH RED=DRAWN ONLY (MISSING IN 3D) BLUE=3D ONLY (NOT DRAWN)");
  const sheet = new Canvas(W, top[0]!.height + ph + legend.height, [255, 255, 255]);
  top.forEach((c, i) => sheet.blit(c, i * pw, 0));
  row.forEach((c, i) => sheet.blit(c, i * pw, top[0]!.height));
  sheet.blit(legend, 0, top[0]!.height + ph);
  save(paths("sheet"), sheet);
  written.push(paths("sheet"));
  return written;
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const flag = (n: string) => (args.indexOf(n) < 0 ? undefined : args[args.indexOf(n) + 1]);
  const spec = args.find((a, i) => !a.startsWith("--") && !["--scale", "--list", "--az", "--el", "--width"].includes(args[i - 1] ?? ""));
  if (spec === undefined) {
    process.stderr.write("usage: volumes-render.ts <category>/<slug> [--scale N] [--list body|thermal|structure] [--az 35] [--el 25] [--no-cut] [--width 1600]\n");
    process.exit(2);
  }
  const { entry, asset } = entryOf(spec);
  const source = loadSource(entry);
  if (source === null) {
    process.stderr.write(`no source for ${spec}; run volumes-seed.ts\n`);
    process.exit(2);
  }
  const m = merged(asset, source);
  const scales = flag("--scale") === undefined ? SCALES : [Number(flag("--scale"))];
  const lists = flag("--list") === undefined ? (["body", "thermal", "structure"] as ListName[]) : [flag("--list") as ListName];
  for (const scale of scales) {
    if (m.volumes[String(scale)]?.status !== "authored") continue;
    const files = renderScale(m, scale, args.includes("--no-numbers") ? {} : await productNumbers(m, scale), lists, { az: flag("--az") === undefined ? undefined : Number(flag("--az")), el: Number(flag("--el") ?? 25), cut: !args.includes("--no-cut"), width: Number(flag("--width") ?? 1600) }, (l) => outPath("render", entry, `${entry.slug}.s${scale}${l === "sheet" ? "" : `.${l}`}.png`));
    for (const f of files) process.stdout.write(`${f}\n`);
  }
}
