/** 🔍️ Round-3 viewer of `humans/standing-adult`: contact sheets of one list from several directions and the 2D drawing of a scale, read from the draft asset (root fixes not yet in the catalog) when it exists.
 *
 * Usage: `bun r3-standing-adult-view.ts 3d --scale 50 --list body [--dirs 90:0,0:0,270:0,180:0,35:20,35:60] [--pose relaxed] [--name tag] [--ghost]`
 *        `--focus head,neck` fits the panels to the body solids of those bones (close-up).
 *        `bun r3-standing-adult-view.ts 2d --scale 50 [--pose relaxed] [--name tag]`
 * Azimuth 90 looks at the front of the figure, 0 at its left side, 180 at its right side, 270 at its back.
 * Output: `🗑️generated/r3-standing-adult/view/*.png`.
 * @see ./round3-brief.md */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { Canvas, type RGB } from "./volumes-raster.ts";
import { bodyItems, drawAxon, fit, structureItems, thermalItems } from "./volumes-render.ts";
import { GENERATED, entryOf, loadSource, merged, poseState, scaleMeshes, type Json } from "./volumes-lib.ts";

export const SPEC = "humans/standing-adult";
export const OUT = join(GENERATED, "r3-standing-adult");
export const DRAFT = join(OUT, "asset.json");

/** 📂️ The asset the round works on: the draft with root fixes when present, else the catalog asset, with the authoring source's volumes merged over it. */
export function draftAsset(pose?: string): Json {
  const { entry, asset } = entryOf(SPEC);
  const base: Json = existsSync(DRAFT) ? JSON.parse(readFileSync(DRAFT, "utf8")).asset : asset;
  const source = loadSource(entry);
  const m = source === null ? base : merged(base, source);
  return pose === undefined ? m : { ...m, presentation: { ...m.presentation, defaultPose: pose } };
}

const camera = (azDeg: number, elDeg: number) => {
  const az = (azDeg * Math.PI) / 180;
  const el = (elDeg * Math.PI) / 180;
  const e: [number, number, number] = [Math.sin(az) * Math.cos(el), Math.sin(el), Math.cos(az) * Math.cos(el)];
  const rr = [e[2], 0, -e[0]];
  const l = Math.hypot(...rr) || 1;
  const r: [number, number, number] = [rr[0]! / l, 0, rr[2]! / l];
  const u: [number, number, number] = [e[1] * r[2] - e[2] * r[1], e[2] * r[0] - e[0] * r[2], e[0] * r[1] - e[1] * r[0]];
  return { r, u, e };
};

const save = (name: string, c: Canvas): string => {
  const dir = join(OUT, "view");
  mkdirSync(dir, { recursive: true });
  const path = join(dir, name);
  writeFileSync(path, c.png());
  return path;
};

function fillRings(c: Canvas, rings: { x: number; y: number }[][], color: RGB): void {
  let lo = Infinity;
  let hi = -Infinity;
  for (const r of rings) for (const p of r) {
    lo = Math.min(lo, p.y);
    hi = Math.max(hi, p.y);
  }
  for (let y = Math.max(0, Math.floor(lo)); y <= Math.min(c.height - 1, Math.ceil(hi)); y++) {
    const py = y + 0.5;
    const xs: number[] = [];
    for (const r of rings) for (let i = 0, k = r.length - 1; i < r.length; k = i++) {
      const a = r[k]!;
      const b = r[i]!;
      if (a.y <= py !== b.y <= py) xs.push(a.x + ((py - a.y) / (b.y - a.y)) * (b.x - a.x));
    }
    xs.sort((p, q) => p - q);
    for (let i = 0; i + 1 < xs.length; i += 2) for (let x = Math.max(0, Math.round(xs[i]!)); x < Math.min(c.width, Math.round(xs[i + 1]!)); x++) c.set(x, y, color);
  }
}

function draw2d(asset: Json, scale: number, name: string): string {
  const st = poseState(asset, scale);
  const W = 2400;
  const H = 1500;
  const AA = 2;
  const big = new Canvas(W * AA, H * AA, [250, 250, 247]);
  const views = ["elevation", "front", "plan"] as const;
  const pw = (W * AA) / 3;
  views.forEach((view, vi) => {
    const shapes = [...(st[view] as Json[])].sort((a, b) => a.shape.z - b.shape.z);
    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
    for (const s of shapes) for (const r of s.rings) for (const p of r) {
      minX = Math.min(minX, p.x);
      maxX = Math.max(maxX, p.x);
      minY = Math.min(minY, p.y);
      maxY = Math.max(maxY, p.y);
    }
    const k = Math.min((pw * 0.9) / (maxX - minX), (H * AA * 0.9) / (maxY - minY), (H * AA * 0.9) / 1.75);
    const ox = vi * pw + pw / 2 - ((minX + maxX) / 2) * k;
    const oy = (H * AA) / 2 + ((minY + maxY) / 2) * k;
    const map = (p: { x: number; y: number }) => ({ x: ox + p.x * k, y: oy - p.y * k });
    for (const s of shapes) {
      const rings = (s.rings as { x: number; y: number }[][]).map((r) => r.map(map));
      if (s.shape.filled && s.closed) {
        const g = 40 + s.shape.tone * 205;
        fillRings(big, rings, [g, g, g]);
      }
      const ink: RGB = s.shape.role === "body" ? [0, 0, 0] : s.shape.role === "detail" ? [30, 30, 160] : [150, 60, 20];
      const thick = s.shape.role === "body" ? 3 : 2;
      for (const r of rings) for (let i = 0; i < r.length - (s.closed ? 0 : 1); i++) {
        const a = r[i]!;
        const b = r[(i + 1) % r.length]!;
        for (let t = 0; t < thick; t++) for (let q = 0; q < thick; q++) big.line(a.x + t, a.y + q, b.x + t, b.y + q, ink);
      }
    }
    for (const [id, f] of Object.entries(st.frames)) {
      const o = view === "elevation" ? { x: f.o[0], y: f.o[1] } : view === "front" ? { x: f.o[2], y: f.o[1] } : { x: f.o[0], y: f.o[2] };
      const p = map(o);
      big.rect(Math.round(p.x) - 4, Math.round(p.y) - 4, 9, 9, id.endsWith("-l") ? [0, 150, 220] : [220, 40, 40]);
    }
  });
  const out = big.downsample(AA);
  out.text(8, 6, `2D 1:${scale} POSE ${asset.presentation.defaultPose}  ELEVATION / FRONT / PLAN  RED,BLUE SQUARES = JOINTS`, [30, 30, 30], 2);
  return save(`${name}.png`, out);
}

function draw3d(asset: Json, scale: number, list: "body" | "thermal" | "structure", dirs: [number, number][], name: string, ghost: boolean, cut: boolean, focus: string[] = []): string {
  const sm = scaleMeshes(asset, scale, asset.volumes[String(scale)]);
  const cols = Math.min(dirs.length, 6);
  const rows = Math.ceil(dirs.length / cols);
  const pw = Math.floor(2400 / cols);
  const ph = focus.length > 0 ? pw : Math.round(pw * (dirs.some(([, el]) => el > 45) ? 2.2 : 2.6));
  const sheet = new Canvas(pw * cols, Math.min(ph, 1500) * rows, [255, 255, 255]);
  const h = Math.min(ph, 1500);
  const AA = 2;
  const bodyI = bodyItems(asset, sm, scale);
  const items = list === "body" ? bodyI : list === "thermal" ? thermalItems(asset, sm, scale) : structureItems(asset, sm, scale);
  const focused = focus.length === 0 ? [] : sm.body.filter((_, i) => focus.includes(sm.solids.body[i]!.bone));
  const all = focused.length > 0 ? focused : [...sm.body, ...sm[list]];
  dirs.forEach(([az, el], i) => {
    const big = new Canvas(pw * AA, h * AA, [247, 247, 244]);
    const cam = camera(az, el);
    const ax = fit(all, cam, pw * AA, h * AA, 0.03);
    if (list !== "body" && ghost) {
      drawAxon(big, ax, bodyI, {});
      for (let p = 0; p < big.width * big.height; p++) if (big.depth[p]! > -Infinity) {
        for (let k = 0; k < 3; k++) big.data[p * 3 + k] = 247 - (247 - big.data[p * 3 + k]!) * 0.2;
        big.depth[p] = -Infinity;
      }
      drawAxon(big, ax, items, { clearDepth: false, cut });
    } else drawAxon(big, ax, items, { cut });
    const small = big.downsample(AA);
    small.text(6, 6, `AZ ${az} EL ${el}`, [30, 30, 30], 2);
    small.frame(0, 0, pw, h, [190, 190, 185]);
    sheet.blit(small, (i % cols) * pw, Math.floor(i / cols) * h);
  });
  sheet.text(6, h * rows - 18, `${list} 1:${scale} ${sm[list].length} SOLIDS POSE ${asset.presentation.defaultPose}`, [90, 90, 90], 2);
  return save(`${name}.png`, sheet);
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const flag = (n: string, d?: string) => (args.indexOf(n) < 0 ? d : args[args.indexOf(n) + 1]);
  const mode = args[0];
  const scale = Number(flag("--scale", "50"));
  const pose = flag("--pose");
  const asset = draftAsset(pose);
  if (mode === "2d") process.stdout.write(`${draw2d(asset, scale, flag("--name", `2d.s${scale}${pose === undefined ? "" : `.${pose}`}`)!)}\n`);
  else {
    const list = flag("--list", "body") as "body" | "thermal" | "structure";
    const dirs = flag("--dirs", "90:0,0:0,270:0,180:0,35:20,125:55")!.split(",").map((d) => d.split(":").map(Number) as [number, number]);
    process.stdout.write(`${draw3d(asset, scale, list, dirs, flag("--name", `3d.s${scale}.${list}${pose === undefined ? "" : `.${pose}`}`)!, !args.includes("--no-ghost"), args.includes("--cut"), (flag("--focus", "") as string).split(",").filter((x) => x !== ""))}\n`);
  }
}
