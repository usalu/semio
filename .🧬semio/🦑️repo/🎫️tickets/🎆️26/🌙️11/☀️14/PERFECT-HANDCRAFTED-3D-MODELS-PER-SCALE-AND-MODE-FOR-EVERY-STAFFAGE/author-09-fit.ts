/** 🎯️ Silhouette fitter: coordinate descent on per-solid cross-section multipliers so the projected body of one scale agrees with that scale's drawings in the default pose.
 * Usage: bun author-09-fit.ts <category>/<slug> <scale> [--ids a,b] [--rounds 3] [--steps 0.12,0.06,0.03] [--write] [--source file] [--bounds 0.5,2.0]
 */
import { readFileSync, writeFileSync } from "node:fs";

import { Mask, VIEWS, entryOf, loadSource, merged, meshesAt, poseState, projectWorld, sourcePath, type Json, type P2, type SolidMesh, type View } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const flag = (n: string, d?: string) => (args.includes(n) ? args[args.indexOf(n) + 1] : d);
const [spec, scaleArg] = args;
const scale = Number(scaleArg);
const { entry, asset } = entryOf(spec!);
const file = flag("--source") ?? sourcePath(entry);
const source = flag("--source") ? JSON.parse(readFileSync(file, "utf8")) : loadSource(entry)!;
const m = merged(asset, source);
const volume = source.volumes[String(scale)];
const body: Json[] = volume.body;
const only = flag("--ids")?.split(",");
const rounds = Number(flag("--rounds", "3"));
const steps = (flag("--steps", "0.12,0.06,0.03")!).split(",").map(Number);
const [kLo, kHi] = (flag("--bounds", "0.55,2.2")!).split(",").map(Number) as [number, number];
const feature = (0.15 * scale) / 1000;
const state = poseState(m, scale);
const ROLES = new Set(["body", "detail", "glazing"]);

type ViewState = { box: { minX: number; minY: number; maxX: number; maxY: number }; cell: number; truth: Mask; count: Int16Array; both: number; either: number; scratch: Mask };
const vs = new Map<View, ViewState>();
for (const v of VIEWS) {
  const shapes = state[v].filter((s: Json) => s.closed && ROLES.has(s.shape.role) && s.rings.length > 0);
  const pts: P2[] = shapes.flatMap((s: Json) => s.rings.flat());
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const p of pts) {
    b.minX = Math.min(b.minX, p.x);
    b.maxX = Math.max(b.maxX, p.x);
    b.minY = Math.min(b.minY, p.y);
    b.maxY = Math.max(b.maxY, p.y);
  }
  const long = Math.max(b.maxX - b.minX, b.maxY - b.minY);
  const pad = long * 0.12;
  const cell = long / 400;
  const box = { minX: b.minX - pad, minY: b.minY - pad, maxX: b.maxX + pad, maxY: b.maxY + pad };
  const truth = new Mask(box, cell);
  for (const s of shapes) truth.fill(s.rings);
  let either = 0;
  for (let i = 0; i < truth.cells.length; i++) either += truth.cells[i]!;
  vs.set(v, { box, cell, truth, count: new Int16Array(truth.cells.length), both: 0, either, scratch: new Mask(box, cell) });
}

const cellsOf = (mesh: SolidMesh): Map<View, Int32Array> => {
  const out = new Map<View, Int32Array>();
  for (const v of VIEWS) {
    const s = vs.get(v)!;
    const stamp = new Uint8Array(s.count.length);
    const list: number[] = [];
    const xy = new Float64Array((mesh.world.length / 3) * 2);
    for (let i = 0; i < mesh.world.length / 3; i++) {
      const p = projectWorld(v, [mesh.world[i * 3]!, mesh.world[i * 3 + 1]!, mesh.world[i * 3 + 2]!]);
      xy[i * 2] = p.x;
      xy[i * 2 + 1] = p.y;
    }
    for (let t = 0; t < mesh.indices.length; t += 3) {
      const tri: P2[] = [0, 1, 2].map((k) => ({ x: xy[mesh.indices[t + k]! * 2]!, y: xy[mesh.indices[t + k]! * 2 + 1]! }));
      s.scratch.fill([tri], (index) => {
        if (stamp[index]) return;
        stamp[index] = 1;
        list.push(index);
      });
    }
    out.set(v, Int32Array.from(list));
  }
  return out;
};

const add = (cells: Map<View, Int32Array>, sign: 1 | -1): void => {
  for (const v of VIEWS) {
    const s = vs.get(v)!;
    for (const i of cells.get(v)!) {
      const before = s.count[i]! > 0;
      s.count[i]! += sign;
      const after = s.count[i]! > 0;
      if (before === after) continue;
      const t = s.truth.cells[i]! === 1;
      if (after) {
        if (t) s.both++;
        else s.either++;
      } else if (t) s.both--;
      else s.either--;
    }
  }
};

const score = (): number => {
  let sum = 0;
  for (const v of VIEWS) {
    const s = vs.get(v)!;
    sum += s.both / s.either;
  }
  return sum / 3;
};

const mesh1 = (solid: Json): SolidMesh => meshesAt(m, scale, [solid])[0]!;

const perSection = args.includes("--sections");
const smooth = Number(flag("--smooth", "0.0004"));
const params = (s: Json): number[] => (s.kind === "loft" ? (perSection ? new Array(2 * s.sections.length).fill(1) : [1, 1]) : s.kind === "ellipsoid" ? [1, 1, 1] : s.kind === "capsule" ? [1] : []);
const rough = (k: number[]): number => {
  if (!perSection || k.length < 4) return 0;
  const n = k.length / 2;
  let r = 0;
  for (let i = 1; i < n; i++) r += Math.log(k[i]! / k[i - 1]!) ** 2 + Math.log(k[n + i]! / k[n + i - 1]!) ** 2;
  return smooth * r;
};
const apply = (orig: Json, k: number[]): Json => {
  const s = JSON.parse(JSON.stringify(orig));
  if (s.kind === "loft") s.sections.forEach((q: Json, i: number) => {
    const n = orig.sections.length;
    q.rx = Math.max(orig.sections[i].rx * (perSection ? k[i]! : k[0]!), 0.55 * feature);
    q.ry = Math.max(orig.sections[i].ry * (perSection ? k[n + i]! : k[1]!), 0.55 * feature);
  });
  else if (s.kind === "ellipsoid") s.radii = orig.radii.map((r: number, i: number) => Math.max(r * k[i]!, 0.55 * feature));
  else if (s.kind === "capsule") {
    s.r0 = Math.max(orig.r0 * k[0]!, 0.55 * feature);
    s.r1 = Math.max(orig.r1 * k[0]!, 0.55 * feature);
  }
  return s;
};

const items = body.map((s) => ({ orig: JSON.parse(JSON.stringify(s)), cur: s, k: params(s), cells: new Map<View, Int32Array>() }));
for (const it of items) {
  it.cells = cellsOf(mesh1(it.cur));
  add(it.cells, 1);
}
const startScore = score();
const fitIds = (it: (typeof items)[number]) => it.k.length > 0 && (only === undefined ? it.cur.role !== "detail" : only.includes(it.cur.id));
for (let round = 0; round < rounds; round++) {
  const step = steps[Math.min(round, steps.length - 1)]!;
  for (const it of items) {
    if (!fitIds(it)) continue;
    for (let p = 0; p < it.k.length; p++) {
      for (const dir of [1, -1]) {
        for (;;) {
          const nk = [...it.k];
          nk[p] = Math.min(kHi, Math.max(kLo, nk[p]! * (1 + dir * step)));
          if (nk[p] === it.k[p]) break;
          const before = score() - rough(it.k);
          const cand = apply(it.orig, nk);
          const cells = cellsOf(mesh1(cand));
          add(it.cells, -1);
          add(cells, 1);
          if (score() - rough(nk) > before + 2e-5) {
            it.k = nk;
            it.cells = cells;
            it.cur = cand;
          } else {
            add(cells, -1);
            add(it.cells, 1);
            break;
          }
        }
      }
    }
  }
  process.stderr.write(`round ${round + 1} step ${step} score ${score().toFixed(4)}\n`);
}
process.stdout.write(`fit 1:${scale} ${spec} mean IoU ${startScore.toFixed(4)} -> ${score().toFixed(4)} (${VIEWS.map((v) => `${v} ${(vs.get(v)!.both / vs.get(v)!.either).toFixed(3)}`).join(", ")})\n`);
for (const it of items) if (fitIds(it) && it.k.some((x) => Math.abs(x - 1) > 0.02)) process.stdout.write(`  ${it.cur.id}: ${it.k.map((x) => x.toFixed(2)).join(" ")}\n`);
if (args.includes("--write")) {
  volume.body = items.map((it) => it.cur);
  writeFileSync(file, JSON.stringify(source));
  process.stdout.write(`written ${file}\n`);
}
