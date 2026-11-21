/** 🖋️ Round 5: redraws the 2D drawings of `humans/worker-carrying-box` at every scale as the orthographic views of that scale's handcrafted body (ruling E1). Head, trunk, pelvis, legs and boots are outlines of their solids (bone-local in the side elevation, rest-world in front and plan); sleeves, forearms and glove cuffs are revolved profiles of their loft sections, gloves, thumbs and every carton part are extruded profiles, so the arms that leave the sagittal plane and the turned carton stay one definition in all three views. Ids, tones, regions, materials and hatches of the previous drawing are kept where the part keeps its id; one id per part in all three views.
 * `bun r3-worker-carrying-box-draw2d.ts` reads `🗑️generated/r3-worker-carrying-box/🔣️.json` (draft rig with the authoring volumes) and writes `🗑️generated/r3-worker-carrying-box/shapes.json` (geometry per scale), which `r3-worker-carrying-box-roots.py` applies; prints the default-pose extent of the 1:50 body. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, localGeometry, qualityOfAsset, resolveList } from "./volumes-lib.ts";

type P = [number, number];
type V = "elevation" | "front" | "plan";
type Shape = Record<string, any>;
const OUT = join(GENERATED, "r3-worker-carrying-box");
const asset = JSON.parse(readFileSync(join(OUT, "🔣️.json"), "utf8")).asset;
const q = qualityOfAsset(asset);
const rest = evaluate({ ...asset, poses: [{ ...asset.poses[0], id: "x", bones: {}, ground: "authored" }] } as never, { kind: "pose", pose: "x" } as never) as any;
const posed = evaluate(asset as never, { kind: "pose", pose: asset.presentation.defaultPose } as never) as any;
const r4 = (v: number): number => Math.round(v * 1e4) / 1e4;

/** 🕸️ Triangles of solids as 2D points: bone-local (u, v) for the elevation, rest-world (z, y) for the front, (x, z) for the plan. */
function triangles(solids: any[], view: V): P[][] {
  const out: P[][] = [];
  for (const s of solids) {
    const g = localGeometry(s, q);
    const f = rest.bones[s.bone].frame;
    const pt = (i: number): P => {
      const [a, b, c] = [g.positions[3 * i]!, g.positions[3 * i + 1]!, g.positions[3 * i + 2]!];
      if (view === "elevation") return [a, b];
      const w = (k: "x" | "y" | "z"): number => f.o[k] + a * f.u[k] + b * f.v[k] + c * f.w[k];
      return view === "front" ? [w("z"), w("y")] : [w("x"), w("z")];
    };
    for (let i = 0; i < g.indices.length; i += 3) out.push([pt(g.indices[i]!), pt(g.indices[i + 1]!), pt(g.indices[i + 2]!)]);
  }
  return out;
}

/** 📐 Range of the bone-local w coordinate over the meshes of solids (the lateral span of an extruded profile). */
function wRange(solids: any[]): [number, number] {
  let [lo, hi] = [Infinity, -Infinity];
  for (const s of solids) {
    const g = localGeometry(s, q);
    for (let i = 2; i < g.positions.length; i += 3) [lo, hi] = [Math.min(lo, g.positions[i]!), Math.max(hi, g.positions[i]!)];
  }
  return [r4(lo), r4(hi)];
}

/** 🖊️ The outer outline of projected triangles: filled on a raster of `cell`, closed by `close` cells (notches narrower than that are filled), traced along the pixel edges, smoothed and reduced to the points that matter within `tol`. */
function outline(tris: P[][], cell: number, tol: number, close = 0): P[] {
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const t of tris) for (const p of t) [x0, y0, x1, y1] = [Math.min(x0, p[0]), Math.min(y0, p[1]), Math.max(x1, p[0]), Math.max(y1, p[1])];
  x0 -= close * cell;
  y0 -= close * cell;
  const W = Math.ceil((x1 - x0) / cell) + 4 + close;
  const H = Math.ceil((y1 - y0) / cell) + 4 + close;
  const m = new Uint8Array(W * H);
  const gx = (x: number): number => (x - x0) / cell + 2;
  const gy = (y: number): number => (y - y0) / cell + 2;
  for (const t of tris) {
    const [a, b, c] = t.map((p) => [gx(p[0]), gy(p[1])]) as [P, P, P];
    const area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    if (Math.abs(area) < 1e-12) continue;
    for (let j = Math.max(0, Math.floor(Math.min(a[1], b[1], c[1]))); j <= Math.min(H - 1, Math.ceil(Math.max(a[1], b[1], c[1]))); j++)
      for (let i = Math.max(0, Math.floor(Math.min(a[0], b[0], c[0]))); i <= Math.min(W - 1, Math.ceil(Math.max(a[0], b[0], c[0]))); i++) {
        const [px, py] = [i + 0.5, j + 0.5];
        const w0 = ((b[0] - a[0]) * (py - a[1]) - (b[1] - a[1]) * (px - a[0])) / area;
        const w1 = ((c[0] - b[0]) * (py - b[1]) - (c[1] - b[1]) * (px - b[0])) / area;
        const w2 = ((a[0] - c[0]) * (py - c[1]) - (a[1] - c[1]) * (px - c[0])) / area;
        if (w0 >= -0.02 && w1 >= -0.02 && w2 >= -0.02) m[j * W + i] = 1;
      }
  }
  const morph = (grow: boolean): void => {
    const src = m.slice();
    for (let j = 0; j < H; j++)
      for (let i = 0; i < W; i++) {
        let hit = !grow;
        for (let dj = -1; dj <= 1 && hit !== grow; dj++)
          for (let di = -1; di <= 1; di++) {
            const v = i + di < 0 || j + dj < 0 || i + di >= W || j + dj >= H ? 0 : src[(j + dj) * W + i + di]!;
            if (grow ? v === 1 : v === 0) {
              hit = grow;
              break;
            }
          }
        m[j * W + i] = hit ? 1 : 0;
      }
  };
  for (let k = 0; k < close; k++) morph(true);
  for (let k = 0; k < close; k++) morph(false);
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= W || j >= H ? 0 : m[j * W + i]!);
  for (let again = true; again; ) {
    again = false;
    for (let j = 0; j < H - 1; j++)
      for (let i = 0; i < W - 1; i++) {
        const [p, qq, r, t] = [m[j * W + i]!, m[j * W + i + 1]!, m[(j + 1) * W + i]!, m[(j + 1) * W + i + 1]!];
        if ((p && t && !qq && !r) || (qq && r && !p && !t)) {
          m[j * W + i] = m[j * W + i + 1] = m[(j + 1) * W + i] = m[(j + 1) * W + i + 1] = 1;
          again = true;
        }
      }
  }
  const next = new Map<string, [number, number]>();
  for (let j = 0; j < H; j++)
    for (let i = 0; i < W; i++) {
      if (!at(i, j)) continue;
      if (!at(i, j - 1)) next.set(`${i},${j}`, [i + 1, j]);
      if (!at(i + 1, j)) next.set(`${i + 1},${j}`, [i + 1, j + 1]);
      if (!at(i, j + 1)) next.set(`${i + 1},${j + 1}`, [i, j + 1]);
      if (!at(i - 1, j)) next.set(`${i},${j + 1}`, [i, j]);
    }
  let best: P[] = [];
  const seen = new Set<string>();
  for (const start of next.keys()) {
    if (seen.has(start)) continue;
    const loop: P[] = [];
    let key = start;
    while (!seen.has(key) && next.has(key)) {
      seen.add(key);
      const [i, j] = key.split(",").map(Number) as [number, number];
      loop.push([i, j]);
      const n = next.get(key)!;
      key = `${n[0]},${n[1]}`;
    }
    if (loop.length > best.length) best = loop;
  }
  const world = best.map(([i, j]) => [x0 + (i - 2) * cell, y0 + (j - 2) * cell] as P);
  const smooth = world.map((_, i) => {
    const [a, b, c] = [world[(i + world.length - 1) % world.length]!, world[i]!, world[(i + 1) % world.length]!];
    return [(a[0] + 2 * b[0] + c[0]) / 4, (a[1] + 2 * b[1] + c[1]) / 4] as P;
  });
  const rdp = (pts: P[]): P[] => {
    if (pts.length < 3) return pts;
    const [a, b] = [pts[0]!, pts[pts.length - 1]!];
    const len = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-9;
    let [far, idx] = [0, 0];
    pts.forEach((p, i) => {
      const d = Math.abs((b[0] - a[0]) * (a[1] - p[1]) - (a[0] - p[0]) * (b[1] - a[1])) / len;
      if (d > far) [far, idx] = [d, i];
    });
    return far <= tol ? [a, b] : [...rdp(pts.slice(0, idx + 1)).slice(0, -1), ...rdp(pts.slice(idx))];
  };
  let split = 0;
  smooth.forEach((p, i) => {
    if (Math.hypot(p[0] - smooth[0]![0], p[1] - smooth[0]![1]) > Math.hypot(smooth[split]![0] - smooth[0]![0], smooth[split]![1] - smooth[0]![1])) split = i;
  });
  return [...rdp(smooth.slice(0, split + 1)).slice(0, -1), ...rdp([...smooth.slice(split), smooth[0]!]).slice(0, -1)].map((p) => [r4(p[0]), r4(p[1])] as P);
}

/** 🗺️ A point of the standing rest pose (x, y, z; ground at y = 0, hip joints at 0.9406) in the side-elevation frame of `bone` (u, v) or in the front (z, y) and plan (x, z) frames. */
const pointIn = (view: V, bone: string, g: [number, number, number]): P => {
  const p: [number, number, number] = [g[0], g[1] + rest.bones.pelvis.frame.o.y - 0.9406, g[2]];
  if (view === "front") return [r4(p[2]), r4(p[1])];
  if (view === "plan") return [r4(p[0]), r4(p[2])];
  const f = rest.bones[bone].frame;
  const d = { x: p[0] - f.o.x, y: p[1] - f.o.y, z: p[2] - f.o.z };
  return [r4(d.x * f.u.x + d.y * f.u.y + d.z * f.u.z), r4(d.x * f.v.x + d.y * f.v.y + d.z * f.v.z)];
};

const META: Record<string, { region: string; tone: number; material: string; hatch?: string }> = {
  skin: { region: "head-skin", tone: 0.86, material: "skin" },
  hair: { region: "hair", tone: 0.18, material: "hair" },
  helmet: { region: "helmet", tone: 0.92, material: "hdpe-shell" },
  shirt: { region: "torso-clothing", tone: 0.45, material: "cotton-clothing", hatch: "knit" },
  vest: { region: "vest", tone: 0.9, material: "hi-vis-polyester", hatch: "shell" },
  band: { region: "vest", tone: 0.55, material: "retroreflective" },
  sleeve: { region: "arm-clothing", tone: 0.45, material: "cotton-clothing", hatch: "knit" },
  glove: { region: "gloves", tone: 0.3, material: "glove" },
  trousers: { region: "leg-clothing", tone: 0.26, material: "cotton-clothing", hatch: "weave" },
  boot: { region: "shoes", tone: 0.1, material: "leather-shoe" },
  box: { region: "box-surface", tone: 0.66, material: "cardboard" },
};

/** 🧾 One drawn part: solids, bone, kind of geometry and its layer per view (a view without a layer does not draw the part). */
type Part = { id: string; bone: string; solids: string[]; meta: keyof typeof META; geo: "flat" | "rev" | "ext"; z: Partial<Record<V, number>>; role?: string; tone?: number; filled?: boolean; profile?: P[]; span?: [number, number] };

const SLOT_POINTS = (cu: number, cv: number, hl: number, hh: number, n: number): P[] => {
  const ring: P[] = [];
  for (const [c, start] of [[cu + hl - hh, -90], [cu - hl + hh, 90]] as [number, number][])
    for (let i = 0; i <= n; i++) {
      const a = ((start + (180 * i) / n) * Math.PI) / 180;
      ring.push([r4(c + hh * Math.cos(a)), r4(cv + hh * Math.sin(a))]);
    }
  return ring;
};

const BOX = { cu: 0, cv: -0.085, cw: 0.217, hu: 0.15, hv: 0.15, hw: 0.2, t: 0.0075 };
const boxRect: P[] = [[BOX.cu - BOX.hu, BOX.cv - BOX.hv], [BOX.cu + BOX.hu, BOX.cv - BOX.hv], [BOX.cu + BOX.hu, BOX.cv + BOX.hv], [BOX.cu - BOX.hu, BOX.cv + BOX.hv]].map(([a, b]) => [r4(a!), r4(b!)] as P);
const top = BOX.cv + BOX.hv;

function parts(scale: number): Part[] {
  const L: Part[] = [];
  const add = (p: Part): void => void L.push(p);
  const fine = scale <= 100;
  if (scale !== 500) add({ id: "neck", bone: "head", solids: ["neck"], meta: "skin", geo: "flat", z: { elevation: 4.5, front: 4.6 } });
  if (scale === 500) add({ id: "head", bone: "head", solids: ["head"], meta: "skin", geo: "flat", z: { elevation: 5 } });
  else add({ id: "head", bone: "head", solids: scale === 50 ? ["head", "chin", "nose", "lips"] : scale === 100 ? ["head", "nose"] : ["head"], meta: "skin", geo: "flat", z: { elevation: 5, front: 5 } });
  add({ id: "hair", bone: "head", solids: ["hair"], meta: "hair", geo: "flat", z: scale === 500 ? { elevation: 4.9 } : { elevation: 5.05, front: 4.95 } });
  if (fine) {
    add({ id: "ear", bone: "head", solids: ["ear-r"], meta: "skin", geo: "flat", z: { elevation: 5.2 }, role: "detail", tone: 0.7 });
    for (const s of ["l", "r"]) add({ id: `ear-${s}`, bone: "head", solids: [`ear-${s}`], meta: "skin", geo: "flat", z: { front: 4.98 }, role: "detail", tone: 0.7 });
  }
  if (scale === 50) {
    add({ id: "eye", bone: "head", solids: ["eye-r"], meta: "skin", geo: "flat", z: { elevation: 5.3 }, role: "detail", tone: 0.1 });
    add({ id: "brow", bone: "head", solids: ["brow-r"], meta: "skin", geo: "flat", z: { elevation: 5.3 }, role: "detail", tone: 0.3 });
    for (const s of ["l", "r"]) {
      add({ id: `eye-${s}`, bone: "head", solids: [`eye-${s}`], meta: "skin", geo: "flat", z: { front: 5.3 }, role: "detail", tone: 0.1 });
      add({ id: `brow-${s}`, bone: "head", solids: [`brow-${s}`], meta: "skin", geo: "flat", z: { front: 5.3 }, role: "detail", tone: 0.3 });
    }
  }
  if (fine) add({ id: "nose", bone: "head", solids: ["nose"], meta: "skin", geo: "flat", z: { front: 5.25 }, role: "detail", filled: false });
  const helmet = scale === 50 ? ["helmet", "helmet-rim", "helmet-peak", "helmet-headband"] : scale === 100 ? ["helmet", "helmet-rim", "helmet-peak"] : scale === 200 ? ["helmet", "helmet-peak"] : ["helmet"];
  add({ id: "helmet", bone: "head", solids: helmet, meta: "helmet", geo: "flat", z: scale === 500 ? { elevation: 5.5, plan: 6 } : { elevation: 5.5, front: 5.5, plan: 6 } });
  if (scale === 500) add({ id: "helmet", bone: "head", solids: ["helmet", "head"], meta: "helmet", geo: "flat", z: { front: 5.5 } });
  if (fine) add({ id: "helmet-ridge", bone: "head", solids: ["helmet-ridge"], meta: "helmet", geo: "flat", z: { elevation: 5.6, front: 5.6, plan: 6.1 }, role: "detail", tone: 0.8 });
  if (scale === 500) {
    add({ id: "torso", bone: "torso", solids: ["vest"], meta: "vest", geo: "flat", z: { elevation: 4, front: 4, plan: 4 } });
    add({ id: "collar", bone: "torso", solids: ["collar"], meta: "shirt", geo: "flat", z: { elevation: 4.2, front: 4.2, plan: 4.2 } });
  }
  else {
    add({ id: "torso", bone: "torso", solids: scale === 50 ? ["shirt", "shirt-collar"] : scale === 100 ? ["shirt-yoke"] : ["shirt-collar"], meta: "shirt", geo: "flat", z: { elevation: 3.9, front: 3.9, plan: 3.9 } });
    add({ id: "vest", bone: "torso", solids: ["vest"], meta: "vest", geo: "flat", z: { elevation: 4, front: 4, plan: 4 } });
    add({ id: "vest-hem", bone: "pelvis", solids: ["vest-hem"], meta: "vest", geo: "flat", z: { elevation: 3.1, front: 3.1, plan: 3.1 } });
    for (const b of ["vest-band-low", "vest-band-high"]) add({ id: b, bone: "torso", solids: [b], meta: "band", geo: "flat", z: { elevation: 4.1, front: 4.1 }, role: "detail", filled: scale === 50 });
  }
  if (fine) {
    add({ id: "vest-strap", bone: "torso", solids: ["vest-brace-r"], meta: "band", geo: "flat", z: { elevation: 4.15 }, role: "detail" });
    for (const s of ["l", "r"]) add({ id: `vest-strap-${s}`, bone: "torso", solids: [`vest-brace-${s}`], meta: "band", geo: "flat", z: { front: 4.15, plan: 4.15 }, role: "detail" });
  }
  const seat = scale === 50 ? ["trousers-seat", "buttock-l", "buttock-r"] : scale === 500 ? ["seat"] : ["trousers-seat"];
  add({ id: "pelvis", bone: "pelvis", solids: seat, meta: "trousers", geo: "flat", z: scale === 500 ? { elevation: 3, plan: 3 } : { elevation: 3, front: 3, plan: 3 } });
  for (const s of ["l", "r"]) {
    const near = s === "r";
    const ez = near ? 2 : 1;
    const az = near ? 7 : 0.5;
    add({ id: `thigh-${s}`, bone: `thigh-${s}`, solids: [`thigh-${s}`], meta: "trousers", geo: "flat", z: { elevation: ez, front: 1, plan: 1.1 } });
    add({ id: `shank-${s}`, bone: `shank-${s}`, solids: fine ? [`shank-${s}`, `knee-${s}`] : [`shank-${s}`], meta: "trousers", geo: "flat", z: { elevation: ez + 0.02, front: 1.02, plan: 1.05 } });
    if (scale === 50) {
      add({ id: `knee-pad-${s}`, bone: `shank-${s}`, solids: [`knee-pad-${s}`], meta: "trousers", geo: "flat", z: near ? { elevation: ez + 0.05, front: 1.05 } : { front: 1.05 }, role: "detail", tone: 0.18 });
      add({ id: `cargo-pocket-${s}`, bone: `thigh-${s}`, solids: [`cargo-pocket-${s}`], meta: "trousers", geo: "flat", z: near ? { elevation: ez + 0.03, front: 1.03 } : { front: 1.03 }, role: "detail", tone: 0.2 });
    }
    const boot = scale === 500 ? [`boot-${s}`] : scale === 200 ? [`boot-shaft-${s}`, `boot-${s}`] : [`boot-shaft-${s}`, `boot-${s}`, `sole-${s}`];
    add({ id: `shoe-${s}`, bone: `foot-${s}`, solids: boot, meta: "boot", geo: "flat", z: { elevation: ez + 0.1, front: 1.1, plan: 1 } });
    add({ id: `shoe-toe-${s}`, bone: `toe-${s}`, solids: scale === 50 ? [`toe-cap-${s}`, `toe-sole-${s}`] : [`toe-${s}`], meta: "boot", geo: "flat", z: { elevation: ez + 0.1, front: 1.12, plan: 1.02 } });
    if (fine) add({ id: `sole-${s}`, bone: `foot-${s}`, solids: [`sole-${s}`], meta: "boot", geo: "flat", z: { elevation: ez + 0.15 }, role: "detail", tone: 0.04 });
    if (scale === 50) add({ id: `sole-toe-${s}`, bone: `toe-${s}`, solids: [`toe-sole-${s}`], meta: "boot", geo: "flat", z: { elevation: ez + 0.15 }, role: "detail", tone: 0.04 });
    add({ id: `upper-arm-${s}`, bone: `upper-arm-${s}`, solids: [`sleeve-${s}`], meta: "sleeve", geo: "rev", z: { elevation: az, front: 4.5, plan: 4.5 } });
    add({ id: `forearm-${s}`, bone: `forearm-${s}`, solids: [`forearm-${s}`], meta: "sleeve", geo: "rev", z: { elevation: az + 0.02, front: 4.52, plan: 3.4 } });
    if (scale !== 500) add({ id: `cuff-${s}`, bone: `forearm-${s}`, solids: [`glove-cuff-${s}`], meta: "glove", geo: "rev", z: { elevation: az + 0.03, front: 4.53, plan: 3.42 } });
    const hand = scale === 50 ? ["hand", "thenar", ...["0", "1", "2", "3"].flatMap((i) => ["a", "b", "c"].map((j) => `finger-${s}-${i}${j}`)), `thumb-${s}-0`, `thumb-${s}-1`].map((x) => (x.startsWith("finger") || x.startsWith("thumb") ? x : `${x}-${s}`)) : scale === 500 ? [`fist-${s}`] : scale === 100 ? [`hand-${s}`, `fingers-${s}`, `thumb-${s}`] : [`hand-${s}`, `fingers-${s}`];
    add({ id: `hand-${s}`, bone: `hand-${s}`, solids: hand, meta: "glove", geo: "ext", z: { elevation: az + 0.2, front: 6.2, plan: 3.3 } });
  }
  add({ id: "box", bone: "box-r", solids: [], meta: "box", geo: "ext", z: { elevation: 6, front: 6, plan: 2 }, profile: boxRect, span: [r4(BOX.cw - BOX.hw), r4(BOX.cw + BOX.hw)] });
  if (scale <= 200) add({ id: "box-handle", bone: "box-r", solids: [], meta: "box", geo: "ext", z: { elevation: 6.05 }, role: "detail", tone: 0.15, profile: SLOT_POINTS(0, -0.01, 0.0525, 0.018, scale === 50 ? 6 : 3), span: [r4(BOX.cw - BOX.hw), r4(BOX.cw - BOX.hw + BOX.t)] });
  if (fine) add({ id: "box-tape", bone: "box-r", solids: [], meta: "box", geo: "ext", z: { elevation: 6.05, plan: 2.05 }, role: "detail", tone: 0.82, profile: [[-0.025, r4(top - 0.04)], [0.025, r4(top - 0.04)], [0.025, r4(top + 0.0012)], [-0.025, r4(top + 0.0012)]], span: [r4(BOX.cw - BOX.hw - 0.0012), r4(BOX.cw + BOX.hw + 0.0012)] });
  if (scale === 50) add({ id: "box-label", bone: "box-r", solids: [], meta: "box", geo: "ext", z: { front: 6.05 }, role: "texture", filled: false, tone: 0.5, profile: [[r4(BOX.hu), r4(BOX.cv - 0.005)], [r4(BOX.hu + 0.001), r4(BOX.cv - 0.005)], [r4(BOX.hu + 0.001), r4(BOX.cv + 0.065)], [r4(BOX.hu), r4(BOX.cv + 0.065)]], span: [r4(BOX.cw + 0.03), r4(BOX.cw + 0.13)] });
  return L;
}

const shapes: Record<string, { geometry: Record<V, Shape[]> }> = {};
for (const scale of [50, 100, 200, 500]) {
  const rep = asset.representations[String(scale)];
  const old: Record<V, Map<string, Shape>> = { elevation: new Map(), front: new Map(), plan: new Map() };
  for (const v of ["elevation", "front", "plan"] as V[]) for (const s of rep.geometry[v]) old[v].set(s.id, s);
  const body: any[] = resolveList(asset.volumes[String(scale)], "body");
  const byId = new Map(body.map((s) => [s.id, s]));
  const cell = { 50: 0.001, 100: 0.001, 200: 0.002, 500: 0.003 }[scale]!;
  const tol = { 50: 0.0012, 100: 0.0025, 200: 0.005, 500: 0.010 }[scale]!;
  const views: Record<V, Shape[]> = { elevation: [], front: [], plan: [] };
  for (const part of parts(scale)) {
    const solids = part.solids.map((id) => {
      const s = byId.get(id);
      if (s === undefined) throw new Error(`1:${scale} has no body solid ${id} for ${part.id}`);
      return s;
    });
    const meta = META[part.meta]!;
    for (const v of ["elevation", "front", "plan"] as V[]) {
      const z = part.z[v];
      if (z === undefined) continue;
      const o = old[v].get(part.id);
      const shape: Shape = { id: part.id, bone: part.bone, role: part.role ?? "body", z, filled: part.filled ?? true, tone: part.tone ?? meta.tone, region: meta.region, material: meta.material };
      const known = new Set((rep.hatches ?? []).map((h: any) => h.id));
      const hatch = [o?.hatch, meta.hatch].find((h) => h !== undefined && known.has(h));
      if (hatch !== undefined && shape.filled && shape.role === "body") shape.hatch = hatch;
      if (part.geo === "flat") Object.assign(shape, { kind: "poly", points: outline(triangles(solids, v), cell, tol), closed: true });
      else if (part.geo === "rev") {
        const sec = [...solids[0].sections].sort((a: any, b: any) => a.at[0] - b.at[0]);
        const profile: P[] = [[r4(sec[0].at[0]), 0], ...sec.map((x: any) => [r4(x.at[0]), r4(x.rx)] as P), [r4(sec[sec.length - 1].at[0]), 0]];
        const lateral = r4(sec.reduce((n: number, x: any) => n + x.ry / x.rx, 0) / sec.length);
        Object.assign(shape, { kind: "poly", points: profile, closed: true, solid: { kind: "revolved", axisBone: part.bone, axis: "u", axisX: 0, mirror: true, lateralScale: lateral } });
      } else {
        const profile = part.profile ?? outline(triangles(solids, "elevation"), cell, tol, Math.round(0.004 / cell));
        const [w0, w1] = part.span ?? wRange(solids);
        const base = rest.bones[part.bone].frame.o.z;
        const [z0, z1] = [r4(base + w0), r4(base + w1)];
        Object.assign(shape, { kind: "poly", points: profile, closed: true, solid: { kind: "extruded", sweep: "w", z0, z1 } });
      }
      views[v].push(shape);
    }
  }
  if (scale <= 100) {
    const line = (view: V, id: string, bone: string, meta: keyof typeof META, z: number, pts: [number, number, number][], role = "detail"): void => {
      const m = META[meta]!;
      views[view].push({ id, bone, role, z, filled: false, tone: 0.3, region: m.region, material: m.material, kind: "poly", points: pts.map((p) => pointIn(view, bone, p)), closed: false });
    };
    const T = 1.771;
    line("elevation", "mouth-line", "head", "skin", 5.3, [[0.1005, T - 0.188, -0.02], [0.088, T - 0.191, -0.02]]);
    line("front", "mouth", "head", "skin", 5.3, [[0.1, T - 0.188, -0.021], [0.1, T - 0.1895, 0], [0.1, T - 0.188, 0.021]]);
    line("elevation", "jaw-line", "head", "skin", 5.3, [[-0.006, T - 0.150, -0.06], [0.004, T - 0.196, -0.06], [0.050, T - 0.214, -0.04]]);
    line("front", "vest-zip", "torso", "vest", 4.12, [[0.15, 1.045, 0], [0.13, 1.30, 0], [0.11, 1.44, 0]], "texture");
    if (scale === 50) {
      line("elevation", "shirt-collar-line", "torso", "shirt", 4.12, [[0.040, 1.505, -0.06], [0.050, 1.530, -0.04], [0.056, 1.552, 0]], "texture");
      line("front", "shirt-collar-line", "torso", "shirt", 4.12, [[0.06, 1.552, -0.062], [0.08, 1.515, 0], [0.06, 1.552, 0.062]], "texture");
    }
  }
  for (const v of ["elevation", "front", "plan"] as V[]) views[v].sort((a, b) => a.z - b.z);
  shapes[String(scale)] = { geometry: views };
  console.log(`1:${scale}`, (["elevation", "front", "plan"] as V[]).map((v) => `${v} ${views[v].length} shapes ${views[v].reduce((n, s) => n + (s.points?.length ?? 0), 0)} points`).join(", "));
}
writeFileSync(join(OUT, "shapes.json"), JSON.stringify(shapes));

let [x0, x1, y1, z0, z1] = [Infinity, -Infinity, -Infinity, Infinity, -Infinity];
for (const s of resolveList(asset.volumes["50"], "body") as any[]) {
  if (s.texture) continue;
  const g = localGeometry(s, q);
  const f = posed.bones[s.bone].frame;
  for (let i = 0; i < g.positions.length; i += 3) {
    const w = (k: "x" | "y" | "z"): number => f.o[k] + g.positions[i]! * f.u[k] + g.positions[i + 1]! * f.v[k] + g.positions[i + 2]! * f.w[k];
    [x0, x1, y1, z0, z1] = [Math.min(x0, w("x")), Math.max(x1, w("x")), Math.max(y1, w("y")), Math.min(z0, w("z")), Math.max(z1, w("z"))];
  }
}
console.log("default-pose 1:50 body: height", y1.toFixed(4), "forward", (x1 - x0).toFixed(4), `(${x0.toFixed(4)} .. ${x1.toFixed(4)})`, "lateral", (z1 - z0).toFixed(4), `(${z0.toFixed(4)} .. ${z1.toFixed(4)})`);
