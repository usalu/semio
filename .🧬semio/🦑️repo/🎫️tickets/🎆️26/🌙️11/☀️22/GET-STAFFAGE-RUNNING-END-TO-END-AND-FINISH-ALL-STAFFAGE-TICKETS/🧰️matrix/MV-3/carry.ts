/** 🍃️ Gives the flat front and plan drawing of a part its depth, as the last step of the chain of a plant: `bun carry.ts <id|file.json> [--out file] [--write] [--stations 3,4,6,8,11,14] [--goal 0.75] [--limit 0.75] [--only bone,bone] [--area <smallest plate in m², default (tolerance / 4)²>] [--scales 50,100] [--points <most points of a plate>] [--inside] [--keep <scale:bone,…>]`.
 *
 * A flat front or plan outline is one sheet at one depth: when the wind poses turn its bone in the sagittal plane the sheet foreshortens, while the elevation outline of the same part turns as a whole (cause D of `multiview-diagnosis.md`). The run measures, per authored scale and over every subject of the matrix (poses, clip instants, transitions), how far the height (front) or forward (plan) extent of each bone leaves the extent of its elevation shapes, and for every bone beyond `limit` × tolerance whose views agree in the default pose to half the tolerance (a bone whose views draw different parts is not a turning sheet) it adds depth plates below the drawn outline:
 * - the elevation outline of the bone is cut at a number of stations (the first rung of the ladder `--stations` that brings the bone below `goal` × tolerance) along the bone axis that lies nearest to the viewing direction; each cut is a chord of the part in the sagittal plane;
 * - at each station a plate stands across that axis: an `extruded` solid 0.004 mm thick whose profile is the drawn outline of the bone in that view (the union of its filled flat shapes) cut to the band the chord covers (cut by the outline itself, scaled down about its middle to the height of the band, so a plate is a smaller figure of the same lobes and shows no straight cut; a slender part that its scaled outline misses is cut straight), so in the default pose every plate lies inside the drawn outline and below it in the paint order (`z` below the lowest shape of the bone): the drawing at rest is unchanged;
 * - in a turned pose the bone carries each plate at its own depth, so the plates show at the windward and lee edge what the sheet alone loses, and the three views stay one object.
 * - the drawn outline itself stays the sheet on top, but a sheet that shares its id with an elevation shape of its bone is carried by the engine at one depth (the middle of that twin) and then overshoots on the lee side; it takes the suffix `-mvs`, so the engine carries each of its vertices at the depth of the elevation silhouette of the bone there and the sheet never leaves the turned part.
 * - a bone that stands askew to the paper at rest (spread or twisted) has its elevation outline as a plate already (`rebind.ts`); its stations are cut from that profile, each plate is the drawn outline cut straight to the strip the chord covers and solved into the plane across the bone axis.
 * - a rung `0` in `--stations` (for example `--stations 0,3,4,6`) tries the sheet alone first: the drawn outline takes the suffix `-mvs` and no plate is added; only a bone that this does not bring below the goal goes on to the plates of the next rung (for a file that must not grow: hanging curtains follow their limb as sheets carried vertex by vertex).
 * - `--points <n>` cuts convex corners off every plate until at most n points are left (only corners whose triangle holds no other point of the plate, so a plate never leaves the drawn outline): for a tree whose outlines have hundreds of points and whose file must not grow by megabytes.
 * - `--inside` keeps every plate inside the drawn outline of its bone in the DEFAULT pose too: where the default pose is not the rest pose (a curtain that hangs half a degree off) a plate at depth stands beside the sheet, and a few centimetres of it close a slit of the drawing that the body has, which `🧊️catalog-silhouette` measures as a boundary half a metre off (measured with `refdiff.ts` and `culprit.ts`). Such a plate is cut to the outline its bone draws in the default pose (every 20 mm of its edge is tested, so a plate that bridges a slit is found); what is still outside afterwards is dropped.
 * - `--keep 50:bone,100:bone` leaves the sheets of these bones at these scales under their own id (carried at one depth, as before the repair): where the default pose tilts a bone, a sheet carried vertex by vertex moves its lower edge in the rest drawing, a few centimetres that `🧊️catalog-silhouette` does not allow at the fine scales; the plates alone then give the part its depth.
 * - a plate that the default print profile of the product refuses at its scale (smaller than the smallest printable feature) is not kept: the scale must still print.
 * Plates carry the id `mvc-<bone>-<station>[-<piece>]` and the look of the largest shape they lie under. The run is idempotent: it removes its own plates and suffixes first, measures the sheets alone and writes the same plates again.
 * @see ../MR/gate.ts — the measurement of the matrix
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate, flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const clipping: any = (await import(Bun.resolveSync("polygon-clipping", `${join(S, "🧪️tests", "🧊️catalog-silhouette", "🔍️oracles")}/`))).default;

type P = [number, number];
type View = "front" | "plan";
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const LADDER = (flag("stations") ?? "3,4,6,8,11,14").split(",").map(Number);
const GOAL = Number(flag("goal") ?? 0.75);
const LIMIT = Number(flag("limit") ?? 0.75);
const only = flag("only")?.split(",");
const scales = flag("scales")?.split(",");
const tolerance: number = engine.toleranceM(asset);
const COMPARED = ["body", "detail", "glazing"];
const PREFIX = "mvc-";
const AREA = Number(flag("area") ?? (tolerance / 4) ** 2);
const SHEET = "-mvs";
const KEEP = new Set((flag("keep") ?? "").split(",").filter((entry) => entry !== ""));
const POINTS = flag("points") === undefined ? undefined : Number(flag("points"));
const trim = (source: P[]): P[] => {
  if (POINTS === undefined) return source;
  const turn = (a: P, b: P, c: P): number => (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
  let ring = source.slice();
  const sign = Math.sign(ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0));
  while (ring.length > Math.max(POINTS, 3)) {
    let best = -1, least = Infinity;
    for (let i = 0; i < ring.length; i++) {
      const [a, b, c] = [ring[(i + ring.length - 1) % ring.length]!, ring[i]!, ring[(i + 1) % ring.length]!];
      const area = turn(a, b, c) * sign;
      if (area < 0 || area >= least) continue;
      if (ring.some((p) => p !== a && p !== b && p !== c && turn(a, b, p) * sign >= 0 && turn(b, c, p) * sign >= 0 && turn(c, a, p) * sign >= 0)) continue;
      best = i; least = area;
    }
    if (best < 0) break;
    ring = ring.filter((_, i) => i !== best);
  }
  return ring;
};
const THICK = 2e-6;
const frames = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
const round = (n: number): number => Math.round(n * 1e6) / 1e6;

/** 📏️ Per scale, view and bone the largest distance between the extent of the bone in the view and in the elevation along their common axis, over every subject. */
function drifts(asset: any): Map<string, { m: number; subject: string; rest: number }> {
  const out = new Map<string, { m: number; subject: string; rest: number }>();
  for (const key of authored)
    for (const subject of engine.subjects(asset, Number(key))) {
      const state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
      const range = (view: string, axis: "x" | "y"): Map<string, [number, number]> => {
        const ranges = new Map<string, [number, number]>();
        for (const shape of state.shapes[view]) {
          if (shape.shape.role === "ground") continue;
          const r = ranges.get(shape.shape.bone) ?? ranges.set(shape.shape.bone, [Infinity, -Infinity]).get(shape.shape.bone)!;
          for (const ring of shape.rings) for (const p of ring) { r[0] = Math.min(r[0], p[axis]); r[1] = Math.max(r[1], p[axis]); }
        }
        return ranges;
      };
      for (const [view, own, other] of [["front", "y", "y"], ["plan", "x", "x"]] as const) {
        const a = range("elevation", other);
        for (const [bone, r] of range(view, own)) {
          const e = a.get(bone);
          if (e === undefined) continue;
          const m = Math.max(Math.abs(e[0] - r[0]), Math.abs(e[1] - r[1]));
          const k = `${key}|${view}|${bone}`;
          const known = out.get(k) ?? out.set(k, { m: -1, subject: subject.id, rest: 0 }).get(k)!;
          if (subject.isDefault) known.rest = Math.max(known.rest, m);
          if (m > known.m) Object.assign(known, { m, subject: subject.id });
        }
      }
    }
  return out;
}

const areaOf = (ring: readonly P[]): number => Math.abs(ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0)) / 2;

/** ✂️ A closed ring thinned by Douglas-Peucker to `eps`. */
function thin(ring: readonly P[], eps: number): P[] {
  const pts = ring.length > 1 && ring[0]![0] === ring[ring.length - 1]![0] && ring[0]![1] === ring[ring.length - 1]![1] ? ring.slice(0, -1) : ring.slice();
  if (pts.length < 5) return pts as P[];
  let far = 0;
  pts.forEach((p, i) => { if (Math.hypot(p[0] - pts[0]![0], p[1] - pts[0]![1]) > Math.hypot(pts[far]![0] - pts[0]![0], pts[far]![1] - pts[0]![1])) far = i; });
  const dp = (line: readonly P[]): P[] => {
    if (line.length < 3) return line.slice(0, -1) as P[];
    const [a, b] = [line[0]!, line[line.length - 1]!];
    const length = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-12;
    let worst = 0, at = 0;
    for (let i = 1; i + 1 < line.length; i++) {
      const d = Math.abs((b[0] - a[0]) * (a[1] - line[i]![1]) - (a[0] - line[i]![0]) * (b[1] - a[1])) / length;
      if (d > worst) { worst = d; at = i; }
    }
    return worst <= eps ? [a] : [...dp(line.slice(0, at + 1)), ...dp(line.slice(at))];
  };
  return [...dp(pts.slice(0, far + 1) as P[]), ...dp([...pts.slice(far), pts[0]!] as P[])];
}

/** 🕳️ A multipolygon cut along the lateral axis through every hole, so each piece is one ring (a solid profile has no holes). */
function withoutHoles(multi: P[][][], lateral: 0 | 1): P[][] {
  const queue = multi.slice();
  const out: P[][] = [];
  for (let guard = 0; queue.length > 0 && guard < 400; guard++) {
    const polygon = queue.pop()!;
    if (polygon.length === 1) { out.push(polygon[0]!); continue; }
    const hole = polygon[1]!;
    const c = hole.reduce((sum, p) => sum + p[lateral], 0) / hole.length;
    const B = 1e4;
    const half = (lo: number, hi: number): P[][] => [lateral === 0 ? [[lo, -B], [hi, -B], [hi, B], [lo, B]] : [[-B, lo], [B, lo], [B, hi], [-B, hi]]];
    queue.push(...clipping.intersection([polygon], [half(-B, c)]), ...clipping.intersection([polygon], [half(c, B)]));
  }
  return out;
}

/** 🧱️ The plates of one bone in one view at one scale; an empty list with a reason when the bone gives none. */
function platesOf(asset: any, stations: number, key: string, view: View, bone: string): { shapes: any[]; note?: string } {
  const geometry = asset.representations[key].geometry;
  const flatFilled = (list: any[]): any[] => list.filter((shape) => shape.bone === bone && shape.filled === true && shape.solid === undefined && shape.kind !== "member" && COMPARED.includes(shape.role) && flattenGeometry(shape).closed);
  const drawn = flatFilled(geometry[view]);
  const { o, u, v, w } = frames[bone]!.frame;
  const askew = Math.abs(w[2]!) < 0.999;
  const profile = askew ? geometry.elevation.filter((shape: any) => shape.bone === bone && shape.filled === true && shape.solid?.kind === "extruded" && shape.solid.sweep === "w" && COMPARED.includes(shape.role)) : flatFilled(geometry.elevation);
  if (geometry[view].some((shape: any) => shape.bone === bone && shape.solid !== undefined && COMPARED.includes(shape.role))) return { shapes: [], note: "draws solids already" };
  if (drawn.length === 0 || profile.length === 0) return { shapes: [], note: `no filled flat ${drawn.length === 0 ? view : "elevation"} shape` };
  const c = view === "front" ? 1 : 0;
  const lateral = view === "front" ? 0 : 1;
  const seen = (axis: number[]): P => (view === "front" ? [axis[2]!, axis[1]!] : [axis[0]!, axis[2]!]);
  const cross = (p: P, q: P): number => p[0] * q[1] - p[1] * q[0];
  const [a, t, sweep] = (askew ? Math.abs(cross(seen(v), seen(w))) >= Math.abs(cross(seen(u), seen(w))) : Math.abs(v[c]!) >= Math.abs(u[c]!)) ? [u, v, "u"] : [v, u, "v"];
  const [tv, wv] = [seen(t), seen(w)];
  if (askew && Math.abs(cross(tv, wv)) < 0.25) return { shapes: [], note: "the plates of the askew bone would stand edge-on to the view" };
  const local = profile.flatMap((shape: any) => flattenGeometry(shape).rings.map((ring: any[]) => ring.map((p) => {
    if (askew) return (a === u ? [p.x, p.y] : [p.y, p.x]) as P;
    const d = [p.x * u[0]! + p.y * v[0]!, p.x * u[1]! + p.y * v[1]!];
    return [d[0]! * a[0]! + d[1]! * a[1]!, d[0]! * t[0]! + d[1]! * t[1]!] as P;
  })));
  const alphas = local.flat().map((p) => p[0]);
  const [a0, a1] = [Math.min(...alphas), Math.max(...alphas)];
  const evenOdd = (shape: any): P[][][] => flattenGeometry(shape).rings.filter((ring: any[]) => ring.length > 2).reduce((acc: P[][][], ring: any[]) => clipping.xor(acc, [[ring.map((p) => [p.x, p.y] as P)]]), [] as P[][][]);
  const outline: P[][][] = clipping.union(...drawn.map(evenOdd));
  const largest = drawn.slice().sort((p, q) => Math.max(...flattenGeometry(q).rings.map((ring: any[]) => areaOf(ring.map((x) => [x.x, x.y] as P)))) - Math.max(...flattenGeometry(p).rings.map((ring: any[]) => areaOf(ring.map((x) => [x.x, x.y] as P)))))[0]!;
  const zLow = Math.min(...geometry[view].filter((shape: any) => shape.bone === bone && !String(shape.id).startsWith(PREFIX)).map((shape: any) => shape.z));
  const look = Object.fromEntries(["role", "tone", "region", "regions", "material"].filter((name) => largest[name] !== undefined).map((name) => [name, largest[name]]));
  const shapes: any[] = [];
  const along = outline.flat(2).map((p) => p[view === "front" ? 1 : 0]);
  const reach = [Math.min(...along), Math.max(...along)] as const;
  const across = outline.flat(2).map((p) => p[lateral]);
  const middle = (Math.min(...across) + Math.max(...across)) / 2;
  const B = 1e4;
  for (let k = 0; k < stations; k++) {
    const alpha = a0 + ((a1 - a0) * (1 - Math.cos((Math.PI * (k + 0.5)) / stations))) / 2;
    const taus: number[] = [];
    for (const ring of local)
      for (let i = 0; i < ring.length; i++) {
        const [p, q] = [ring[i]!, ring[(i + 1) % ring.length]!];
        if ((p[0] - alpha) * (q[0] - alpha) > 0 || p[0] === q[0]) continue;
        taus.push(p[1] + ((q[1] - p[1]) * (alpha - p[0])) / (q[0] - p[0]));
      }
    if (taus.length < 2) continue;
    if (askew) {
      const ov = seen([o[0]! + alpha * a[0]!, o[1]! + alpha * a[1]!, o[2]! + alpha * a[2]!]);
      const corner = (tau: number, omega: number): P => [ov[0] + tau * tv[0] + omega * wv[0], ov[1] + tau * tv[1] + omega * wv[1]];
      const [tlo, thi] = [Math.min(...taus), Math.max(...taus)];
      const strip: P[][] = [[corner(tlo, -1e3), corner(thi, -1e3), corner(thi, 1e3), corner(tlo, 1e3)]];
      const det = cross(tv, wv);
      const cut = withoutHoles(clipping.intersection(outline, [strip]), lateral)
        .map((ring) => trim(thin(ring, tolerance / 8)))
        .filter((ring) => ring.length >= 3 && areaOf(ring) >= AREA)
        .sort((p, q) => areaOf(q) - areaOf(p));
      cut.forEach((ring, n) => {
        const points = ring.map((p) => [round(cross([p[0] - ov[0], p[1] - ov[1]], wv) / det), round(cross(tv, [p[0] - ov[0], p[1] - ov[1]]) / det)]);
        shapes.push({ id: `${PREFIX}${bone}-${k + 1}${cut.length > 1 ? `-${n + 1}` : ""}`, bone, ...look, z: round(zLow - 1e-4 * (shapes.length + 1)), filled: true, kind: "poly", points, closed: true, solid: { kind: "extruded", sweep, z0: round(alpha - THICK), z1: round(alpha + THICK) } });
      });
      continue;
    }
    const at = (tau: number): number => o[c]! + alpha * a[c]! + tau * t[c]!;
    const [lo, hi] = [at(Math.min(...taus)), at(Math.max(...taus))].sort((p, q) => p - q) as [number, number];
    const [from, to] = [Math.max(lo, reach[0]), Math.min(hi, reach[1])];
    const scale = (to - from) / (reach[1] - reach[0]);
    const squeezed: P[][][] = outline.map((polygon) => polygon.map((ring) => ring.map((p) => (view === "front" ? [middle + (p[0] - middle) * scale, from + (p[1] - reach[0]) * scale] : [from + (p[0] - reach[0]) * scale, middle + (p[1] - middle) * scale]) as P)));
    const lobed: P[][][] = to - from >= reach[1] - reach[0] - 1e-6 ? outline : clipping.intersection(outline, squeezed);
    const covered = lobed.flat(2).map((p) => p[view === "front" ? 1 : 0]);
    const band: P[][] = [view === "front" ? [[-B, lo], [B, lo], [B, hi], [-B, hi]] : [[lo, -B], [hi, -B], [hi, B], [lo, B]]];
    const pieces = withoutHoles(covered.length > 0 && Math.max(...covered) - Math.min(...covered) >= 0.9 * (Math.min(hi, reach[1]) - Math.max(lo, reach[0])) ? lobed : clipping.intersection(outline, [band]), lateral)
      .flatMap((ring) => {
        const slim = thin(ring, tolerance / 8);
        if (slim.length < 3) return [];
        try {
          return withoutHoles(clipping.intersection([[ring]], [[slim]]), lateral);
        } catch {
          return [ring];
        }
      })
      .map((ring) => trim(ring.length > 1 && ring[0]![0] === ring[ring.length - 1]![0] && ring[0]![1] === ring[ring.length - 1]![1] ? ring.slice(0, -1) : ring))
      .filter((ring) => ring.length >= 3 && areaOf(ring) >= AREA)
      .sort((p, q) => areaOf(q) - areaOf(p));
    pieces.forEach((ring, n) => {
      const points = ring.map((p) => [round((p[c === 1 ? 1 : 0] - o[c]! - alpha * a[c]!) / t[c]!), round(p[lateral] - o[2]!)]);
      shapes.push({ id: `${PREFIX}${bone}-${k + 1}${pieces.length > 1 ? `-${n + 1}` : ""}`, bone, ...look, z: round(zLow - 1e-4 * (shapes.length + 1)), filled: true, kind: "poly", points, closed: true, solid: { kind: "extruded", sweep, z0: round(alpha - THICK), z1: round(alpha + THICK) } });
    });
  }
  return { shapes };
}

for (const key of Object.keys(asset.representations)) {
  const geometry = asset.representations[key].geometry;
  if (geometry === undefined || (scales !== undefined && !scales.includes(key))) continue;
  for (const view of ["front", "plan"] as const) {
    geometry[view] = geometry[view].filter((shape: any) => !String(shape.id).startsWith(PREFIX) || (only !== undefined && !only.includes(shape.bone)));
    for (const shape of geometry[view]) if (String(shape.id).endsWith(SHEET) && (only === undefined || only.includes(shape.bone))) shape.id = shape.id.slice(0, -SHEET.length);
  }
}
const before = drifts(structuredClone(asset));
const wanted = [...before.keys()].sort().filter((k) => {
  const { m, rest } = before.get(k)!;
  return m > LIMIT * tolerance && rest <= 0.5 * tolerance && (only === undefined || only.includes(k.split("|")[2]!));
});
const notes = new Map<string, string>();
const rung = new Map<string, number>(wanted.map((k) => [k, 0]));
const plates = new Map<string, any[]>();

/** 🏗️ The asset with the plates chosen so far. */
function built(): any {
  const copy = structuredClone(asset);
  for (const k of wanted) {
    const shapes = plates.get(k) ?? [];
    if (shapes.length === 0 && (LADDER[rung.get(k)!] !== 0 || notes.has(k))) continue;
    const [key, view, bone] = k.split("|") as [string, View, string];
    const list: any[] = copy.representations[key].geometry[view];
    for (const shape of list) if (!KEEP.has(`${key}:${bone}`) && shape.bone === bone && shape.solid === undefined && copy.representations[key].geometry.elevation.some((twin: any) => twin.id === shape.id && twin.bone === bone)) shape.id = `${shape.id}${SHEET}`;
    list.splice(list.findIndex((shape) => shape.bone === bone), 0, ...structuredClone(shapes));
  }
  return copy;
}

let open = wanted.slice();
let result = asset;
let after = before;
while (open.length > 0) {
  for (const k of open) {
    const [key, view, bone] = k.split("|") as [string, View, string];
    const { shapes, note }: { shapes: any[]; note?: string } = LADDER[rung.get(k)!] === 0 ? { shapes: [] } : platesOf(asset, LADDER[rung.get(k)!]!, key, view, bone);
    if (note !== undefined) notes.set(k, note);
    plates.set(k, shapes);
  }
  result = built();
  after = drifts(structuredClone(result));
  const spent = open.filter((k) => !notes.has(k) && rung.get(k)! + 1 >= LADDER.length && after.get(k)!.m > 0.9 * before.get(k)!.m);
  for (const k of spent) { notes.set(k, "plates do not help this part: none kept"); plates.set(k, []); }
  if (spent.length > 0) { result = built(); after = drifts(structuredClone(result)); }
  open = open.filter((k) => !notes.has(k) && after.get(k)!.m > GOAL * tolerance && rung.get(k)! + 1 < LADDER.length);
  for (const k of open) rung.set(k, rung.get(k)! + 1);
}
if (args.includes("--inside")) {
  type Q = { x: number; y: number };
  const within = (p: Q, rings: Q[][]): boolean => {
    let odd = false;
    for (const ring of rings) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if (ring[i]!.y > p.y !== ring[j]!.y > p.y && p.x < ((ring[j]!.x - ring[i]!.x) * (p.y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) odd = !odd;
    return odd;
  };
  const off = (p: Q, rings: Q[][]): number => {
    let least = Infinity;
    for (const ring of rings) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const [a, b] = [ring[i]!, ring[j]!];
      const l = (b.x - a.x) ** 2 + (b.y - a.y) ** 2;
      const t = l === 0 ? 0 : Math.min(1, Math.max(0, ((p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y)) / l));
      least = Math.min(least, Math.hypot(p.x - a.x - t * (b.x - a.x), p.y - a.y - t * (b.y - a.y)));
    }
    return least;
  };
  const sampled = (rings: Q[][]): Q[] => rings.flatMap((ring) => ring.flatMap((a, i) => {
    const b = ring[(i + 1) % ring.length]!;
    const n = Math.min(50, Math.max(1, Math.ceil(Math.hypot(b.x - a.x, b.y - a.y) / 0.02)));
    return Array.from({ length: n }, (_, k) => ({ x: a.x + ((b.x - a.x) * k) / n, y: a.y + ((b.y - a.y) * k) / n }));
  }));
  let [cut, gone] = [0, 0];
  for (let pass = 0; pass < 2; pass++) {
    const fixes = new Map<string, any[]>();
    for (const key of authored) {
      const state: any = evaluate(result, { kind: "pose", pose: result.presentation.defaultPose }, engine.scaleSelector(Number(key)));
      for (const view of ["front", "plan"] as const) {
        const sheets = (state.shapes[view] as any[]).filter((s) => s.shape.filled === true && s.shape.role !== "ground" && !String(s.shape.id).startsWith(PREFIX));
        const outlines = new Map<string, P[][][] | undefined>();
        for (const s of state.shapes[view] as any[]) {
          if (!String(s.shape.id).startsWith(PREFIX)) continue;
          const mates = sheets.filter((o) => o.shape.bone === s.shape.bone);
          if (!sampled(s.rings).some((p) => !mates.some((o) => within(p, o.rings)) && Math.min(...mates.map((o) => off(p, o.rings))) > 0.001)) continue;
          const pieces: any[] = [];
          const solid = s.shape.solid;
          const f = state.bones[s.shape.bone].frame as Record<"o" | "u" | "v" | "w", { x: number; y: number; z: number }>;
          if (pass === 0 && (solid.sweep === "u" || solid.sweep === "v")) {
            const [axis, e1, e2] = solid.sweep === "u" ? [f.u, f.v, f.w] : [f.v, f.u, f.w];
            const alpha = (solid.z0 + solid.z1) / 2;
            const all = (s.rings as Q[][]).flat();
            const middle = [(Math.min(...all.map((q) => q.x)) + Math.max(...all.map((q) => q.x))) / 2, (Math.min(...all.map((q) => q.y)) + Math.max(...all.map((q) => q.y))) / 2];
            const maps = [1, -1].map((sign) => {
              const paper = (v: { x: number; y: number; z: number }): P => (view === "plan" ? [v.x, sign * v.z] : [sign * v.z, v.y]);
              const [origin, a, b] = [paper({ x: f.o.x + alpha * axis.x, y: f.o.y + alpha * axis.y, z: f.o.z + alpha * axis.z }), paper(e1), paper(e2)];
              const image = (s.shape.points as P[]).map((q) => [origin[0] + q[0] * a[0] + q[1] * b[0], origin[1] + q[0] * a[1] + q[1] * b[1]] as P);
              const centre = [(Math.min(...image.map((q) => q[0])) + Math.max(...image.map((q) => q[0]))) / 2, (Math.min(...image.map((q) => q[1])) + Math.max(...image.map((q) => q[1]))) / 2];
              return { origin, a, b, image, miss: Math.hypot(centre[0]! - middle[0]!, centre[1]! - middle[1]!) };
            }).sort((m, n) => m.miss - n.miss)[0]!;
            const det = maps.a[0] * maps.b[1] - maps.a[1] * maps.b[0];
            if (maps.miss < 0.005 && Math.abs(det) > 1e-9) {
              if (!outlines.has(s.shape.bone)) {
                try { outlines.set(s.shape.bone, clipping.union(...mates.map((o) => (o.rings as Q[][]).filter((ring) => ring.length > 2).reduce((acc: P[][][], ring) => clipping.xor(acc, [[ring.map((q) => [q.x, q.y] as P)]]), [] as P[][][])))); } catch { outlines.set(s.shape.bone, undefined); }
              }
              const outline = outlines.get(s.shape.bone);
              let inside: P[][] = [];
              try { inside = outline === undefined ? [] : withoutHoles(clipping.intersection([[maps.image]], outline), 1); } catch { inside = []; }
              inside.map((ring) => trim(ring.length > 1 && ring[0]![0] === ring[ring.length - 1]![0] && ring[0]![1] === ring[ring.length - 1]![1] ? ring.slice(0, -1) : ring)).filter((ring) => ring.length >= 3 && areaOf(ring) >= AREA).slice(0, 8).forEach((ring, n, list) => {
                pieces.push({ ...s.shape, id: `${s.shape.id}${list.length > 1 ? "abcdefgh"[n] : ""}`, points: ring.map((q) => [round((maps.b[1] * (q[0] - maps.origin[0]) - maps.b[0] * (q[1] - maps.origin[1])) / det), round((-maps.a[1] * (q[0] - maps.origin[0]) + maps.a[0] * (q[1] - maps.origin[1])) / det)]) });
              });
            }
          }
          fixes.set(`${key}|${view}|${s.shape.id}`, pieces);
          if (pieces.length > 0) cut++;
          else gone++;
        }
      }
    }
    if (fixes.size === 0) break;
    for (const k of wanted) {
      const [key, view] = k.split("|");
      plates.set(k, (plates.get(k) ?? []).flatMap((shape) => structuredClone(fixes.get(`${key}|${view}|${shape.id}`) ?? [shape])));
    }
    result = built();
  }
  after = drifts(structuredClone(result));
  console.log(`${asset.id}: ${cut} plates cut to the outline of the default pose, ${gone} plates not kept`);
}
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(join(S, "🔨️modules", "🖨️representation", "🟦️.ts"));
for (let round = 0, dropped = 0; round < 8; round++) {
  const unprintable = new Map<string, Set<string>>();
  for (const key of authored) {
    const resolved: any = resolveRepresentation(result, { denominator: Number(key) } as never, DEFAULT_PRINT_PROFILE);
    for (const violation of resolved.suitability?.violations ?? resolved.violations ?? []) for (const id of violation.shapes ?? []) if (String(id).startsWith(PREFIX)) (unprintable.get(key) ?? unprintable.set(key, new Set()).get(key)!).add(String(id));
  }
  if (unprintable.size === 0) { if (dropped > 0) console.log(`${asset.id}: ${dropped} plates below the smallest printable feature of their scale not kept`); break; }
  for (const k of wanted) {
    const bad = unprintable.get(k.split("|")[0]!);
    const kept = (plates.get(k) ?? []).filter((shape) => !(bad?.has(shape.id) ?? false));
    dropped += (plates.get(k) ?? []).length - kept.length;
    plates.set(k, kept);
  }
  result = built();
  after = drifts(structuredClone(result));
}
let added = 0, points = 0;
for (const k of wanted) {
  const { m, subject, rest } = before.get(k)!;
  const now = after.get(k)!;
  const shapes = plates.get(k) ?? [];
  added += shapes.length;
  points += shapes.reduce((sum, shape) => sum + shape.points.length, 0);
  console.log(`${k.replaceAll("|", " ")}: ${(m * 1000).toFixed(0)} mm @ ${subject} (default pose ${(rest * 1000).toFixed(0)} mm) -> ${(now.m * 1000).toFixed(0)} mm @ ${now.subject}; ${notes.get(k) ?? `${LADDER[rung.get(k)!]} stations, ${shapes.length} plates`}${now.m > tolerance ? "  STILL BEYOND" : ""}`);
}
for (const [k, { m, subject, rest }] of before) if (m > tolerance && !wanted.includes(k) && (only === undefined || only.includes(k.split("|")[2]!))) console.log(`LEFT ${k.replaceAll("|", " ")}: ${(m * 1000).toFixed(0)} mm @ ${subject}, default pose ${(rest * 1000).toFixed(0)} mm: the views draw different parts of the bone`);
for (const [k, { m, subject }] of after) if (m > LIMIT * tolerance && (before.get(k)?.m ?? 0) <= LIMIT * tolerance) console.log(`NEW ${k.replaceAll("|", " ")}: ${(m * 1000).toFixed(0)} mm @ ${subject}`);
console.log(`${asset.id}: tolerance ${(tolerance * 1000).toFixed(0)} mm, ${added} plates with ${points} points`);
next.asset = result;
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  const written = rewrite(text, previous, next);
  if (written === text && out === undefined) console.log(`${asset.id}: unchanged`);
  else { writeFileSync(out ?? file, written); console.log(`${asset.id}: written to ${out ?? file} (${text.length} -> ${written.length} characters)`); }
}
