/** 🌾️ Gives the flat plan tresses of hanging foliage their body, as the last step of the chain of a plant: `bun strands.ts <id|file.json> --bones curt-w1,curt-w2 [--out file] [--write] [--stations 5] [--fit 0.92] [--shift 0.08] [--least 0.25] [--group 200:1.4,500:2.4] [--tip 0.2] [--scales 50,100] [--no-print]`.
 *
 * A plan outline is one sheet at one height: when the wind swings a curtain that hangs plumb at rest, the sheet moves by the swing of its own height only, while the elevation shows the curtain from its bough to its hem. The volume of every scale already holds the curtain as hanging sprays (`tress-<id>-spray-<n>`, polyhedra of the bone). Under the plan sheets of the bone the run lays one `loft` shape per tress:
 * - the sprays of a tress are cut across the bone at `--stations` heights from their top to their tip; each cut is the box of the section (forward and lateral range), drawn as an ellipse of `--fit` times its half extents, so the loft is inside the body and tapers as the sprays do;
 * - at the scales of `--group <scale>:<metres>` neighbouring tresses of a bone are one strand as long as they are together at most so wide across, and no section is narrower than `--tip` millimetres on the paper either side of its middle, so a small scale holds few blunt strands;
 * - the sections are shrunk and the whole strand is narrowed until every section lies inside the drawn plan outline of the bone at rest, shortened forward by `--shift` metres per degree the default pose turns the bone, so nothing of the strand shows at rest and in the default pose; a strand below `--least` millimetres on the paper at its widest section is not kept.
 * In a wind pose the strands of a curtain stream out of the plan as far as the elevation shows, as tapering bodies; nothing of the sheets is touched. A strand the default print profile refuses in a named pose (too fine, or a gap beside it too narrow to print) is not kept: the scale must still print.
 * Strands carry the id `mvt-<tress>` and stand directly under the sheet of their tress, or under the lowest sheet of their bone. The run is idempotent: it removes its own strands first.
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte
 * @see ./carry.ts — depth plates, the tool this one replaces for hanging curtains */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const clipping: any = (await import(Bun.resolveSync("polygon-clipping", `${join(S, "🧪️tests", "🧊️catalog-silhouette", "🔍️oracles")}/`))).default;

type P = [number, number];
type Multi = P[][][];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const PREFIX = "mvt-";
const bones = flag("bones")!.split(",");
const scales = flag("scales")?.split(",");
const STATIONS = Number(flag("stations") ?? 5);
const FIT = Number(flag("fit") ?? 0.92);
const SHIFT = Number(flag("shift") ?? 0.08);
const LEAST = Number(flag("least") ?? 0.25);
const TIP = Number(flag("tip") ?? 0.2);
const GROUP = new Map((flag("group") ?? "200:1.4,500:2.4").split(",").map((entry) => entry.split(":")).map(([key, metres]) => [key!, Number(metres)]));
const frames = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
const pose = asset.poses.find((p: any) => p.id === asset.presentation.defaultPose);
const round = (n: number): number => Math.round(n * 1e4) / 1e4;

const turn = (a: P, b: P, c: P): number => (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
const areaOf = (ring: readonly P[]): number => ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0) / 2;

/** ✂️ A closed ring thinned by Douglas-Peucker to `eps`. */
function thin(ring: readonly P[], eps: number): P[] {
  const pts = ring.slice() as P[];
  if (pts.length > 1 && pts[0]![0] === pts[pts.length - 1]![0] && pts[0]![1] === pts[pts.length - 1]![1]) pts.pop();
  if (pts.length < 5) return pts;
  let far = 0;
  pts.forEach((p, i) => { if (Math.hypot(p[0] - pts[0]![0], p[1] - pts[0]![1]) > Math.hypot(pts[far]![0] - pts[0]![0], pts[far]![1] - pts[0]![1])) far = i; });
  const line = (run: P[]): P[] => {
    const keep = new Uint8Array(run.length);
    keep[0] = keep[run.length - 1] = 1;
    const stack: [number, number][] = [[0, run.length - 1]];
    while (stack.length > 0) {
      const [a, b] = stack.pop()!;
      const length = Math.hypot(run[b]![0] - run[a]![0], run[b]![1] - run[a]![1]) || 1e-12;
      let worst = eps, at = -1;
      for (let i = a + 1; i < b; i++) {
        const d = Math.abs(turn(run[a]!, run[b]!, run[i]!)) / length;
        if (d > worst) { worst = d; at = i; }
      }
      if (at >= 0) { keep[at] = 1; stack.push([a, at], [at, b]); }
    }
    return run.filter((_, i) => keep[i] === 1);
  };
  return [...line(pts.slice(0, far + 1)).slice(0, -1), ...line([...pts.slice(far), pts[0]!]).slice(0, -1)];
}

const multiOf = (shape: any, eps: number, place: (p: P) => P): Multi =>
  (flattenGeometry(shape).rings as { x: number; y: number }[][])
    .map((ring) => thin(ring.map((p) => place([p.x, p.y])), eps))
    .filter((ring) => ring.length > 2 && Math.abs(areaOf(ring)) > 1e-6)
    .reduce((acc: Multi, ring) => clipping.xor(acc, [[ring]]), [] as Multi);


type V3 = [number, number, number];
type Section = { u: number; v: number; w: number; rx: number; ry: number };

const within = (p: P, multi: Multi): boolean => {
  let odd = false;
  for (const polygon of multi) for (const ring of polygon) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if (ring[i]![1] > p[1] !== ring[j]![1] > p[1] && p[0] < ((ring[j]![0] - ring[i]![0]) * (p[1] - ring[i]![1])) / (ring[j]![1] - ring[i]![1]) + ring[i]![0]) odd = !odd;
  return odd;
};

/** 🔪️ The box of the cut of polyhedra across the bone at the height `u`: forward and lateral range. */
function slice(bodies: any[], u: number): { v: P; w: P } | undefined {
  let [v, w]: [P, P] = [[Infinity, -Infinity], [Infinity, -Infinity]];
  for (const body of bodies) for (const face of body.triangles as number[][]) for (let i = 0; i < 3; i++) {
    const [a, b] = [body.vertices[face[i]!] as V3, body.vertices[face[(i + 1) % 3]!] as V3];
    if (a[0] > u === b[0] > u) continue;
    const t = (u - a[0]) / (b[0] - a[0]);
    const [pv, pw] = [a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
    v = [Math.min(v[0], pv), Math.max(v[1], pv)];
    w = [Math.min(w[0], pw), Math.max(w[1], pw)];
  }
  return v[0] < v[1] ? { v, w } : undefined;
}

let added = 0;
for (const [key, decision] of Object.entries(asset.representations) as [string, any][]) {
  if (decision.status !== "authored" || (scales !== undefined && !scales.includes(key))) continue;
  const geometry = decision.geometry;
  geometry.plan = geometry.plan.filter((shape: any) => !String(shape.id).startsWith(PREFIX));
  const paper = Number(key) / 1000;
  for (const bone of bones) {
    const { o, u, v, w } = frames[bone]!.frame;
    if (Math.abs(w[2]!) < 0.9999 || Math.abs(u[1]!) < 0.99) throw new Error(`${bone} does not hang plumb in the sagittal plane at rest`);
    const sheets = geometry.plan.filter((shape: any) => shape.bone === bone && shape.filled === true && shape.solid === undefined && shape.role === "body" && flattenGeometry(shape).closed);
    const outlines = new Map<string, Multi>(sheets.map((shape: any) => [shape.id, multiOf(shape, 0.1 * paper, (p) => p)]));
    const tilt = Math.abs(pose?.bones?.[bone]?.angleDeg ?? 0) * SHIFT;
    const groups = new Map<string, any[]>();
    for (const body of asset.volumes[key].body) if (body.bone === bone && body.kind === "polyhedron") {
      const id = String(body.id).replace(/-spray-\d+$/u, "");
      (groups.get(id) ?? groups.set(id, []).get(id)!).push(body);
    }
    let [kept, gone] = [0, 0];
    const limit = GROUP.get(key);
    const across = (bodies: any[]): P => bodies.flatMap((body) => body.vertices.map((p: V3) => p[2])).reduce((r: P, q: number) => [Math.min(r[0], q), Math.max(r[1], q)] as P, [Infinity, -Infinity] as P);
    const strands: [string, any[]][] = limit === undefined ? [...groups] : [...groups].filter(([id]) => id.startsWith("tress-")).sort(([, a], [, b]) => across(a)[0] - across(b)[0]).reduce((list: [string, any[]][], [id, bodies]) => {
      const last = list[list.length - 1];
      const wide = last === undefined ? Infinity : across([...last[1], ...bodies]).reduce((lo, hi) => hi - lo);
      if (last === undefined || wide > limit) list.push([id, bodies]);
      else last[1] = [...last[1], ...bodies];
      return list;
    }, []);
    for (const [id, bodies] of strands) {
      const us = bodies.flatMap((body) => body.vertices.map((p: V3) => p[0]));
      const [top, tip] = [Math.min(...us), Math.max(...us)];
      const sections: Section[] = [];
      const floor = limit === undefined ? 0 : TIP * paper;
      for (let i = 0; i < STATIONS; i++) {
        const at = top + (tip - top) * (0.02 + (0.96 * i) / (STATIONS - 1));
        const box = slice(bodies, at);
        if (box !== undefined) sections.push({ u: at, v: (box.v[0] + box.v[1]) / 2, w: (box.w[0] + box.w[1]) / 2, rx: Math.max(floor, (FIT * (box.v[1] - box.v[0])) / 2), ry: Math.max(floor, (FIT * (box.w[1] - box.w[0])) / 2) });
      }
      const inside = (s: Section): boolean => Array.from({ length: 24 }, (_, j) => (2 * Math.PI * j) / 24).every((t) => {
        const [pv, pw] = [s.v + (s.rx + tilt) * Math.cos(t), s.w + s.ry * Math.sin(t)];
        const p: P = [o[0]! + s.u * u[0]! + pv * v[0]!, o[2]! + pw * w[2]!];
        return [...outlines.values()].some((multi) => within(p, multi));
      });
      const fitted: Section[] = [];
      for (const s of sections) {
        let now = s;
        for (let n = 0; n < 9 && !inside(now); n++) now = { ...now, rx: now.rx * 0.85, ry: now.ry * 0.85 };
        if (inside(now) && now.rx > 0.01 && now.ry > 0.01) fitted.push(now);
      }
      if (fitted.length < 2 || Math.max(...fitted.map((s) => 2 * Math.min(s.rx, s.ry))) < LEAST * paper) { gone++; continue; }
      const sheet = sheets.find((shape: any) => shape.id === id) ?? sheets.slice().sort((a: any, b: any) => (a.z ?? 0) - (b.z ?? 0))[0];
      if (sheet === undefined) { gone++; continue; }
      const { kind: _kind, d: _d, points: _points, closed: _closed, fillRule: _rule, ...look } = sheet;
      geometry.plan.splice(geometry.plan.indexOf(sheet), 0, { ...look, id: `${PREFIX}${id}`, z: round((sheet.z ?? 0) - 0.0005), kind: "loft", sections: fitted.map((s) => ({ at: [round(s.u), round(s.v), round(s.w)], rx: round(s.rx), ry: round(s.ry), rotation: [0.5, 0.5, 0.5, 0.5] })) });
      kept++;
    }
    added += kept;
    console.log(`${key} ${bone}: ${sheets.length} sheets, ${kept} strands, ${gone} not kept`);
  }
}
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(join(S, "🔨️modules", "🖨️representation", "🟦️.ts"));
let dropped = 0;
for (const [key, decision] of Object.entries(asset.representations) as [string, any][]) {
  if (args.includes("--no-print") || decision.status !== "authored" || (scales !== undefined && !scales.includes(key))) continue;
  for (let turn = 0; turn < 12; turn++) {
    const resolved: any = resolveRepresentation(structuredClone(asset), { denominator: Number(key) } as never, DEFAULT_PRINT_PROFILE);
    const bad = new Set<string>();
    for (const violation of resolved.suitability?.violations ?? resolved.violations ?? []) {
      const own = (violation.shapes ?? []).map(String).filter((id: string) => id.startsWith(PREFIX));
      console.log(`${key} print: ${JSON.stringify(violation).slice(0, 200)}`);
      if (own.length > 0) bad.add(own[own.length - 1]);
    }
    if (bad.size === 0) break;
    dropped += bad.size;
    decision.geometry.plan = decision.geometry.plan.filter((shape: any) => !bad.has(shape.id));
  }
}
console.log(`${asset.id}: ${added - dropped} strands, ${dropped} not kept because the scale would not print`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  const written = rewrite(text, previous, next);
  if (written === text && out === undefined) console.log(`${asset.id}: unchanged`);
  else { writeFileSync(out ?? file, written); console.log(`${asset.id}: written to ${out ?? file} (${text.length} -> ${written.length} characters)`); }
}
