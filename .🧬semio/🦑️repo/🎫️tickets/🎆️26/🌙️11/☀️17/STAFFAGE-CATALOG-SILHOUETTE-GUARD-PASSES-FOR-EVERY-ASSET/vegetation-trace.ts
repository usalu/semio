/** ✒️ The outline of the volume projection of every bone of a vegetation asset, per scale and view, as simplified rings in the coordinates the drawing of that bone uses (elevation: bone-local metres, front and plan: world metres).
 *  bun vegetation-trace.ts <asset> <out.json> [scale ...] */
import { writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundsOf, distanceField, drawnFor, measureView, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, out, ...scalesArg] = process.argv.slice(2) as string[];
const EPS = Number(process.env.EPS ?? 0.2);
const HOLE = Number(process.env.HOLE ?? 2);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
type P = [number, number];

function trace(m: Mask): P[][] {
  const { width: w, height: h, cells } = m;
  const stride = w + 1;
  const from: number[] = [];
  const to: number[] = [];
  const out = new Map<number, number[]>();
  const add = (a: number, b: number): void => {
    const k = from.length;
    from.push(a);
    to.push(b);
    const l = out.get(a);
    if (l === undefined) out.set(a, [k]);
    else l.push(k);
  };
  const on = (i: number, j: number): boolean => i >= 0 && j >= 0 && i < w && j < h && cells[j * w + i] === 1;
  for (let j = 0; j < h; j++)
    for (let i = 0; i < w; i++) {
      if (!cells[j * w + i]) continue;
      if (!on(i, j - 1)) add(j * stride + i, j * stride + i + 1);
      if (!on(i + 1, j)) add(j * stride + i + 1, (j + 1) * stride + i + 1);
      if (!on(i, j + 1)) add((j + 1) * stride + i + 1, (j + 1) * stride + i);
      if (!on(i - 1, j)) add((j + 1) * stride + i, j * stride + i);
    }
  const used = new Uint8Array(from.length);
  const loops: P[][] = [];
  for (let s = 0; s < from.length; s++) {
    if (used[s]) continue;
    const ring: P[] = [];
    let e = s;
    for (;;) {
      used[e] = 1;
      ring.push([from[e]! % stride, Math.floor(from[e]! / stride)]);
      const next = out.get(to[e]!)!.filter((k) => !used[k] || k === s);
      if (next.length === 0) break;
      let pick = next[0]!;
      if (next.length > 1) {
        const [dx, dy] = [(to[e]! % stride) - (from[e]! % stride), Math.floor(to[e]! / stride) - Math.floor(from[e]! / stride)];
        pick = next.find((k) => {
          const [ex, ey] = [(to[k]! % stride) - (from[k]! % stride), Math.floor(to[k]! / stride) - Math.floor(from[k]! / stride)];
          return dx * ey - dy * ex > 0;
        }) ?? next[0]!;
      }
      e = pick;
      if (e === s) break;
    }
    loops.push(ring);
  }
  return loops;
}

function rdp(points: P[], eps: number): P[] {
  const n = points.length;
  if (n < 5) return points;
  let far = 0;
  let best = -1;
  for (let i = 1; i < n; i++) {
    const d = (points[i]![0] - points[0]![0]) ** 2 + (points[i]![1] - points[0]![1]) ** 2;
    if (d > best) {
      best = d;
      far = i;
    }
  }
  const keep = new Uint8Array(n);
  keep[0] = 1;
  keep[far] = 1;
  const stack: [number, number][] = [[0, far], [far, n]];
  while (stack.length > 0) {
    const [a, b] = stack.pop()!;
    const [p, q] = [points[a]!, points[b % n]!];
    const [dx, dy] = [q[0] - p[0], q[1] - p[1]];
    const len = Math.hypot(dx, dy);
    let m = -1;
    let md = eps;
    for (let i = a + 1; i < b; i++) {
      const r = points[i]!;
      const d = len === 0 ? Math.hypot(r[0] - p[0], r[1] - p[1]) : Math.abs(dx * (r[1] - p[1]) - dy * (r[0] - p[0])) / len;
      if (d > md) {
        md = d;
        m = i;
      }
    }
    if (m >= 0) {
      keep[m] = 1;
      stack.push([a, m], [m, b]);
    }
  }
  return points.filter((_, i) => keep[i]);
}

function rdpOpen(points: P[], eps: number): P[] {
  const n = points.length;
  if (n < 3) return points;
  const keep = new Uint8Array(n);
  keep[0] = 1;
  keep[n - 1] = 1;
  const stack: [number, number][] = [[0, n - 1]];
  while (stack.length > 0) {
    const [a, b] = stack.pop()!;
    const [p, q] = [points[a]!, points[b]!];
    const [dx, dy] = [q[0] - p[0], q[1] - p[1]];
    const len = Math.hypot(dx, dy);
    let m = -1;
    let md = eps;
    for (let i = a + 1; i < b; i++) {
      const r = points[i]!;
      const d = len === 0 ? Math.hypot(r[0] - p[0], r[1] - p[1]) : Math.abs(dx * (r[1] - p[1]) - dy * (r[0] - p[0])) / len;
      if (d > md) {
        md = d;
        m = i;
      }
    }
    if (m >= 0) {
      keep[m] = 1;
      stack.push([a, m], [m, b]);
    }
  }
  return points.filter((_, i) => keep[i]);
}

function dilate(m: Mask): void {
  const { width: w, height: h, cells } = m;
  const src = cells.slice();
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    if (src[j * w + i]) continue;
    if ((i > 0 && src[j * w + i - 1]) || (i + 1 < w && src[j * w + i + 1]) || (j > 0 && src[(j - 1) * w + i]) || (j + 1 < h && src[(j + 1) * w + i])) cells[j * w + i] = 1;
  }
}

const area = (r: P[]): number => {
  let s = 0;
  for (let i = 0; i < r.length; i++) {
    const [a, b] = [r[i]!, r[(i + 1) % r.length]!];
    s += a[0] * b[1] - b[0] * a[1];
  }
  return s / 2;
};

const result: Record<string, unknown> = {};
const scales = (scalesArg.length > 0 ? scalesArg : ["50", "100", "200", "500"]).map(Number).filter((s) => (asset.volumes as any)[String(s)]?.status === "authored");
for (const scale of scales) {
  const feature = featureOf(scale);
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
  const solids = worldSolids(asset, state.bones as never, scale).filter((s) => s.texture !== true);
  const sections = (asset.representations[String(scale) as never] as any).sections ?? {};
  const bones = [...new Set(solids.map((s) => s.bone!))];
  const perView: Record<string, unknown> = {};
  for (const view of VIEWPOINTS) {
    const whole = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
    const viewFail = whole.iou < GATE.iou || whole.hausdorffM > 2 * feature;
    if (sections[view] !== undefined) {
      perView[view] = { section: true };
      continue;
    }
    void drawnFor;
    const wc = feature / 2;
    const wbox = { minX: whole.box.minX, minY: whole.box.minY, maxX: whole.box.maxX, maxY: whole.box.maxY };
    const dall = new Mask(wbox, wc);
    const vall = new Mask(wbox, wc);
    for (const g of whole.filled) dall.fill(g);
    for (const g of whole.outlines) dall.fill(g);
    for (const g of whole.reference) vall.fill(g, true);
    vall.seal();
    const toD = distanceField(dall.cells, dall.width, dall.height);
    const toV = distanceField(vall.cells, vall.width, vall.height);
    const perBone: Record<string, unknown> = { viewFail: { iou: +whole.iou.toFixed(4), h: +whole.hausdorffM.toFixed(4), fail: viewFail } };
    const shapes = (state.shapes[view] as any[]).filter((s) => ["body", "detail", "glazing"].includes(s.shape.role) && s.rings.length > 0 && s.closed && s.shape.filled !== false);
    for (const bone of bones) {
      const own = solids.filter((s) => s.bone === bone && s.slender !== true);
      if (own.length === 0) continue;
      const groups = referenceOf(view, own, undefined, undefined).cut;
      const [x0, x1, y0, y1] = groups.reduce((acc, g) => {
        const b = boundsOf(g);
        return [Math.min(acc[0], b[0]), Math.max(acc[1], b[1]), Math.min(acc[2], b[2]), Math.max(acc[3], b[3])];
      }, [Infinity, -Infinity, Infinity, -Infinity]);
      const cell = feature / 4;
      const box = { minX: x0 - 2 * cell, minY: y0 - 2 * cell, maxX: x1 + 2 * cell, maxY: y1 + 2 * cell };
      const m = new Mask(box, cell);
      for (const g of groups) m.fill(g, true);
      m.seal();
      if (process.env.DILATE === "1") dilate(m);
      const mx = state.bones[bone].world as readonly number[];
      const det = mx[0]! * mx[3]! - mx[1]! * mx[2]!;
      const back = ([x, y]: P): P => {
        const [dx, dy] = [x - mx[4]!, y - mx[5]!];
        return [(mx[3]! * dx - mx[2]! * dy) / det, (-mx[1]! * dx + mx[0]! * dy) / det];
      };
      const rings: string[] = [];
      const target: string[] = [];
      let total = 0;
      let perimeter = 0;
      for (const loop of trace(m)) {
        const world = loop.map(([i, j]) => [box.minX + i * cell, box.minY + j * cell] as P);
        const a = area(world);
        if (a < 0 ? -a < (HOLE * feature) ** 2 : a < (0.25 * feature) ** 2) continue;
        const simple = rdp(world, EPS * feature);
        if (simple.length < 3) continue;
        total += a;
        perimeter += simple.reduce((s, q, i) => s + Math.hypot(q[0] - simple[(i + 1) % simple.length]![0], q[1] - simple[(i + 1) % simple.length]![1]), 0);
        const local = view === "elevation" ? simple.map(back) : simple;
        rings.push(local.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";"));
        target.push(simple.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";"));
      }
      const mine = shapes.filter((s) => s.shape.bone === bone);
      const bm = new Mask(wbox, wc);
      for (const g of groups) bm.fill(g, true);
      bm.seal();
      const dm = new Mask(wbox, wc);
      for (const s of mine) dm.fill(s.rings);
      let miss = 0;
      let extra = 0;
      let both = 0;
      let either = 0;
      for (let i = 0; i < bm.cells.length; i++) {
        if (bm.cells[i]! && !dall.cells[i]! && toD[i]! > miss) miss = toD[i]!;
        if (dm.cells[i]! && !vall.cells[i]! && toV[i]! > extra) extra = toV[i]!;
        both += bm.cells[i]! & dm.cells[i]!;
        either += bm.cells[i]! | dm.cells[i]!;
      }
      perBone[bone] = { thick: perimeter === 0 ? 0 : +((2 * Math.abs(total)) / perimeter).toFixed(4), area: +total.toFixed(4), iou: either === 0 ? 1 : +(both / either).toFixed(4), miss: +(Math.sqrt(miss) * wc).toFixed(4), extra: +(Math.sqrt(extra) * wc).toFixed(4), drawn: mine.map((s) => s.shape.id), rings, target };
    }
    const clips: unknown[] = [];
    const closedShapes = (state.shapes[view] as any[]).filter((q) => ["body", "detail", "glazing"].includes(q.shape.role) && q.rings.length > 0 && q.closed);
    for (const s of closedShapes) {
      if (s.shape.solid !== undefined || s.shape.role === "texture") continue;
      const dm = new Mask(wbox, wc);
      dm.fill(s.rings);
      let extra = 0;
      for (let i = 0; i < dm.cells.length; i++) if (dm.cells[i]! && !vall.cells[i]! && toV[i]! > extra) extra = toV[i]!;
      const e = Math.sqrt(extra) * wc;
      if (e <= 0.3 * 2 * feature) continue;
      const gb = s.rings.reduce((acc: number[], r: any[]) => r.reduce((a2: number[], q: any) => [Math.min(a2[0]!, q.x), Math.max(a2[1]!, q.x), Math.min(a2[2]!, q.y), Math.max(a2[3]!, q.y)], acc), [Infinity, -Infinity, Infinity, -Infinity]);
      const cell = feature / 4;
      const box = { minX: gb[0]! - 2 * cell, minY: gb[2]! - 2 * cell, maxX: gb[1]! + 2 * cell, maxY: gb[3]! + 2 * cell };
      const ms = new Mask(box, cell);
      ms.fill(s.rings);
      const vm = new Mask(box, cell);
      for (const g of whole.reference) vm.fill(g, true);
      vm.seal();
      for (let i = 0; i < ms.cells.length; i++) ms.cells[i] = ms.cells[i]! & vm.cells[i]!;
      const f = state.bones[s.shape.bone].world as readonly number[];
      const det = f[0]! * f[3]! - f[1]! * f[2]!;
      const back = ([x, y]: P): P => {
        const [dx, dy] = [x - f[4]!, y - f[5]!];
        return [(f[3]! * dx - f[2]! * dy) / det, (-f[1]! * dx + f[0]! * dy) / det];
      };
      const rings: string[] = [];
      const target: string[] = [];
      for (const loop of trace(ms)) {
        const world = loop.map(([i, j]) => [box.minX + i * cell, box.minY + j * cell] as P);
        { const aw = area(world); if (aw < 0 ? -aw < (HOLE * feature) ** 2 : aw < (0.25 * feature) ** 2) continue; }
        const simple = rdp(world, EPS * feature);
        if (simple.length < 3) continue;
        const local = view === "elevation" ? simple.map(back) : simple;
        rings.push(local.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";"));
        target.push(simple.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";"));
      }
      const ar = s.rings.reduce((acc: number, r: { x: number; y: number }[]) => acc + Math.abs(area(r.map((q) => [q.x, q.y] as P))), 0);
      const pr = s.rings.reduce((acc: number, r: { x: number; y: number }[]) => acc + r.reduce((a2: number, q, i) => a2 + Math.hypot(q.x - r[(i + 1) % r.length]!.x, q.y - r[(i + 1) % r.length]!.y), 0), 0);
      clips.push({ id: s.shape.id, bone: s.shape.bone, extra: +e.toFixed(4), thick: pr === 0 ? 0 : +((2 * ar) / pr).toFixed(4), rings, target });
    }
    (perBone as any).clips = clips;
    const rep = asset.representations[String(scale) as never] as any;
    const slenders: unknown[] = [];
    for (const bone of bones) {
      const sl = solids.filter((s) => s.bone === bone && s.slender === true);
      if (sl.length === 0) continue;
      const big = shapes.some((s) => s.shape.bone === bone && s.shape.solid === undefined && s.rings.reduce((acc: number, r: { x: number; y: number }[]) => acc + Math.abs(area(r.map((q) => [q.x, q.y] as P))), 0) > 20 * feature * feature);
      const none = !shapes.some((s) => s.shape.bone === bone);
      if (!big && !none) continue;
      const fb = state.bones[bone].world as readonly number[];
      const detb = fb[0]! * fb[3]! - fb[1]! * fb[2]!;
      const backb = ([x, y]: P): P => {
        const [dx, dy] = [x - fb[4]!, y - fb[5]!];
        return [(fb[3]! * dx - fb[2]! * dy) / detb, (-fb[1]! * dx + fb[0]! * dy) / detb];
      };
      for (const s of sl) {
        const groups = referenceOf(view, [s], undefined, undefined).cut;
        const rr: string[] = [];
        const tt: string[] = [];
        for (const g of groups) {
          const r = g[0]!;
          if (r.length !== 4) continue;
          const [m03, m12] = [{ x: (r[0]!.x + r[3]!.x) / 2, y: (r[0]!.y + r[3]!.y) / 2 }, { x: (r[1]!.x + r[2]!.x) / 2, y: (r[1]!.y + r[2]!.y) / 2 }];
          const hw = Math.hypot(r[0]!.x - r[3]!.x, r[0]!.y - r[3]!.y) / 2;
          const len = Math.hypot(m12.x - m03.x, m12.y - m03.y);
          const [ux, uy] = len < 1e-9 ? [0, 0] : [(m12.x - m03.x) / len, (m12.y - m03.y) / len];
          const [a, b] = [{ x: m03.x + ux * hw, y: m03.y + uy * hw }, { x: m12.x - ux * hw, y: m12.y - uy * hw }];
          const loc = ([x, y]: P): P => (view === "elevation" ? backb([x, y]) : [x, y]);
          const [la, lb] = [loc([a.x, a.y]), loc([b.x, b.y])];
          rr.push(`${+la[0].toFixed(4)} ${+la[1].toFixed(4)};${+lb[0].toFixed(4)} ${+lb[1].toFixed(4)}`);
          tt.push(`${+a.x.toFixed(4)} ${+a.y.toFixed(4)};${+b.x.toFixed(4)} ${+b.y.toFixed(4)}`);
        }
        if (rr.length > 0) slenders.push({ id: s.id + "-line", bone, solid: s.id, runs: rr, target: tt });
      }
    }
    (perBone as any).slenders = slenders;
    const strokes: unknown[] = [];
    for (const s of (state.shapes[view] as any[]).filter((q) => ["body", "detail", "glazing"].includes(q.shape.role) && q.rings.length > 0 && q.shape.solid === undefined && !(q.closed && q.shape.filled !== false))) {
      const pen = (((s.shape.penMm ?? rep.pens[s.shape.role]) ?? 0) * scale) / 1000;
      const lim = (0.3 * 2 * feature + pen / 2) / wc;
      const samples: { x: number; y: number; inside: boolean; ring: number }[] = [];
      s.rings.forEach((ring: { x: number; y: number }[], ri: number) => {
        const pts = s.closed ? [...ring, ring[0]!] : ring;
        for (let i = 0; i + 1 < pts.length; i++) {
          const [p0, p1] = [pts[i]!, pts[i + 1]!];
          const n = Math.max(1, Math.ceil(Math.hypot(p1.x - p0.x, p1.y - p0.y) / (feature / 2)));
          for (let k = i === 0 ? 0 : 1; k <= n; k++) {
            const [x, y] = [p0.x + ((p1.x - p0.x) * k) / n, p0.y + ((p1.y - p0.y) * k) / n];
            samples.push({ x, y, inside: vall.has(x, y), ring: ri });
          }
        }
      });
      let bad = false;
      for (const q of samples) {
        const [i, j] = [Math.floor((q.x - wbox.minX) / wc), Math.floor((q.y - wbox.minY) / wc)];
        if (i < 0 || j < 0 || i >= vall.width || j >= vall.height || toV[j * vall.width + i]! > lim * lim) bad = true;
      }
      if (!bad) continue;
      const f = state.bones[s.shape.bone].world as readonly number[];
      const det = f[0]! * f[3]! - f[1]! * f[2]!;
      const back = ([x, y]: P): P => {
        const [dx, dy] = [x - f[4]!, y - f[5]!];
        return [(f[3]! * dx - f[2]! * dy) / det, (-f[1]! * dx + f[0]! * dy) / det];
      };
      const runs: P[][] = [];
      let cur: P[] = [];
      let last = -1;
      for (const q of samples) {
        if (q.inside && q.ring === last || q.inside && cur.length === 0) cur.push([q.x, q.y]);
        else {
          if (cur.length >= 2) runs.push(cur);
          cur = q.inside ? [[q.x, q.y]] : [];
        }
        last = q.ring;
      }
      if (cur.length >= 2) runs.push(cur);
      const simple = runs.map((r) => rdpOpen(r, 0.2 * feature)).filter((r) => r.length >= 2);
      strokes.push({ id: s.shape.id, bone: s.shape.bone, runs: simple.map((r) => r.map(([x, y]) => { const l = view === "elevation" ? back([x, y]) : [x, y]; return `${+l[0]!.toFixed(4)} ${+l[1]!.toFixed(4)}`; }).join(";")), target: simple.map((r) => r.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";")) });
    }
    (perBone as any).strokes = strokes;
    perView[view] = perBone;
  }
  result[String(scale)] = perView;
  console.log("[DEBUG] scale", scale, "done", Object.entries(perView).map(([v, b]) => `${v}:${Object.keys(b as object).length}`).join(" "));
}
writeFileSync(out, JSON.stringify(result));
