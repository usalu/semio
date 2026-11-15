/** 🐈️ Author 41: patch file (for author-29-shape.py) that repairs the 2D drawings of the domestic cat at the root: smooth barrel profiles for the revolved trunk shapes, filled closed body outlines, and the 1:200 elevation rebuilt from the 1:100 limb, tail and trunk outlines.
 * Usage: bun author-41-cat2d.ts > 🗑️generated/author-41/cat2d-patch.json */
import { openCtx, drawn } from "./kit-lib.ts";
import { hull, simplifyRing, stationOutline, range, type P } from "./author-41-geo.ts";

const A = "animals/domestic-cat";
const c50 = openCtx(A, 50);
const c100 = openCtx(A, 100);
const c200 = openCtx(A, 200);
const patches: any[] = [];
const r5 = (x: number) => Math.round(x * 1e5) / 1e5;

const WX: P[] = [[-0.056, 0], [-0.052, 0.02], [-0.044, 0.034], [-0.03, 0.045], [-0.005, 0.052], [0.03, 0.054], [0.07, 0.05], [0.11, 0.047], [0.15, 0.054], [0.19, 0.061], [0.235, 0.064], [0.275, 0.062], [0.31, 0.052], [0.34, 0.036], [0.372, 0]];
const W = (x: number): number => {
  if (x <= WX[0]![0]) return WX[0]![1];
  for (let i = 1; i < WX.length; i++) if (x <= WX[i]![0]) { const a = WX[i - 1]!; const b = WX[i]!; return a[1] + ((b[1] - a[1]) * (x - a[0])) / (b[0] - a[0]); }
  return 0;
};
const profile = (x0: number, x1: number, ox: number, n: number, rump: boolean, front: boolean): P[] => {
  const pts: P[] = [[r5(x0 - ox), 0]];
  for (const x of range(x0, x1, n)) pts.push([r5(x - ox), r5(W(x))]);
  pts.push([r5(x1 - ox), 0]);
  return pts;
};

const trunk: Record<string, { bone: string; ids: Record<string, [string, number, number, number]> }> = {
  "50": { bone: "", ids: { hindbody: ["lumbar", -0.055, 0.203, 10], midbody: ["thorax-a", 0.12, 0.291, 8], chest: ["thorax-b", 0.229, 0.345, 7] } },
  "100": { bone: "", ids: { hindbody: ["lumbar", -0.055, 0.2, 7], midbody: ["thorax-a", 0.11, 0.29, 6], chest: ["thorax-b", 0.231, 0.372, 6] } },
  "200": { bone: "", ids: { hindbody: ["lumbar", -0.055, 0.215, 7], chest: ["thorax-b", 0.208, 0.372, 7] } },
};
for (const [sc, t] of Object.entries(trunk)) {
  const c = sc === "50" ? c50 : sc === "100" ? c100 : c200;
  for (const [id, [bone, x0, x1, n]] of Object.entries(t.ids)) {
    const ox = (c.state.frames[bone]!.o as number[])[0]!;
    for (const view of ["front", "plan"] as const) patches.push({ scale: sc, view, id, set: { points: profile(x0, x1, ox, n, id === "hindbody", true) } });
  }
}

for (const [sc, view, id] of [["50", "elevation", "jaw"], ["100", "elevation", "jaw"]] as const) patches.push({ scale: sc, view, id, set: { filled: true } });

const world = (c: ReturnType<typeof openCtx>, id: string): P[] => drawn(c, "elevation").find((s) => s.id === id)!.rings[0]!.map((p) => [p.x, p.y] as P);
const local = (bone: string, p: P): P => {
  const f = c200.state.frames[bone]!;
  const d: P = [p[0] - (f.o as number[])[0]!, p[1] - (f.o as number[])[1]!];
  const u = f.u as number[];
  const v = f.v as number[];
  return [r5(d[0] * u[0]! + d[1] * u[1]!), r5(d[0] * v[0]! + d[1] * v[1]!)];
};
const set200 = (id: string, bone: string, world: P[], tol: number) => patches.push({ scale: "200", view: "elevation", id, set: { kind: "poly", closed: true, filled: true, points: simplifyRing(hull(world), tol).map((p) => local(bone, p)) } });
const set200ring = (id: string, bone: string, world: P[], tol: number) => patches.push({ scale: "200", view: "elevation", id, set: { kind: "poly", closed: true, filled: true, points: simplifyRing(world, tol).map((p) => local(bone, p)) } });
const sides = [["n", "near"], ["f", "far"]] as const;
for (const [s, nm] of sides) {
  const [fh, ff] = [s === "n" ? "n" : "f", s];
  set200ring(`${fh}-hind-thigh`, `thigh-h${ff}`, world(c100, `${nm}-thigh`), 0.0045);
  set200ring(`${fh}-hind-lower`, `shank-h${ff}`, world(c100, `${nm}-shank`), 0.0045);
  set200(`${fh}-hind-foot`, `metatarsus-h${ff}`, [...world(c100, `${nm}-hock`), ...world(c100, `${nm}-hind-paw`)], 0.004);
  set200(`${fh}-fore-upper`, `humerus-f${ff}`, [...world(c100, `${nm}-scapula`), ...world(c100, `${nm}-humerus`)], 0.0045);
  set200ring(`${fh}-fore-lower`, `forearm-f${ff}`, world(c100, `${nm}-forearm`), 0.0045);
  set200(`${fh}-fore-foot`, `metacarpus-f${ff}`, [...world(c100, `${nm}-wrist`), ...world(c100, `${nm}-fore-paw`)], 0.004);
}
const tail = (a: string[]) => a.flatMap((i) => world(c100, i));
set200("tail-a", "tail-2", tail(["tail-1", "tail-2", "tail-3"]), 0.004);
set200("tail-b", "tail-4", tail(["tail-4", "tail-5"]), 0.004);
const trunkRings = (ids: string[]) => ids.map((i) => world(c100, i));
const xs = (a: number, b: number, n: number) => range(a, b, n);
patches.push({ scale: "200", view: "elevation", id: "hindbody", set: { points: simplifyRing(stationOutline(trunkRings(["hindbody", "midbody"]), xs(-0.054, 0.289, 40)), 0.004).map((p) => [r5(p[0] - 0.06), r5(p[1] - 0.2284)]), filled: true } });
patches.push({ scale: "200", view: "elevation", id: "chest", set: { points: simplifyRing(stationOutline(trunkRings(["chest"]), xs(0.232, 0.371, 22)), 0.004).map((p) => [r5(p[0] - 0.215), r5(p[1] - 0.2284)]), filled: true } });
for (const [id, bone] of [["jaw", "jaw"], ["near-ear", "ear-n"], ["far-ear", "ear-f"]] as const) patches.push({ scale: "200", view: "elevation", id, set: { filled: true } });
void c50; void bone;
console.log(JSON.stringify(patches));
function bone() {}
