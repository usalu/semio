/** 🎯 Newton refinement of rewritten front and plan shapes: the evaluated rings of every edited shape (edits-<slug>.json) against their world targets; prints the worst residual and writes the corrections in the local coordinates of the shape.
 *  bun vegetation-refine.ts <asset> <edits.json> <out.json> */
import { readFileSync, writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate, flattenGeometry } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [name, editsFile, out] = process.argv.slice(2) as string[];
const edits = JSON.parse(readFileSync(editsFile, "utf8")) as { scale: string; view: string; id: string; bone: string; target: string[] }[];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
type P = [number, number];
const corrections: unknown[] = [];
let worst = 0;
let bad = 0;
for (const scale of [...new Set(edits.map((e) => e.scale))]) {
  const feature = featureOf(Number(scale));
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } }) as any;
  for (const e of edits.filter((x) => x.scale === scale)) {
    const ev = (state.shapes[e.view] as any[]).find((s) => s.shape.id === e.id && s.shape.bone === e.bone);
    if (ev === undefined) { console.log("[DEBUG] missing", scale, e.view, e.id); bad++; continue; }
    const flat = flattenGeometry(ev.shape).rings as { x: number; y: number }[][];
    const target = e.target.map((r) => r.split(";").map((p) => p.split(" ").map(Number) as P));
    if (flat.length !== target.length || flat.some((r, i) => r.length !== target[i]!.length || ev.rings[i].length !== r.length)) { console.log("[DEBUG] ring mismatch", scale, e.view, e.id, flat.map((r) => r.length), target.map((r) => r.length), ev.rings.map((r: any[]) => r.length)); bad++; continue; }
    const p = flat.flat().map((q) => [q.x, q.y] as P);
    const q = (ev.rings as { x: number; y: number }[][]).flat().map((r) => [r.x, r.y] as P);
    const w = target.flat();
    const n = p.length;
    const resid = Math.max(...q.map((r, i) => Math.hypot(r[0] - w[i]![0], r[1] - w[i]![1])));
    worst = Math.max(worst, resid / feature);
    if (resid < 0.1 * feature) continue;
    const mean = (f: (i: number) => number): number => p.reduce((s, _, i) => s + f(i), 0) / n;
    const [mpx, mpy, mqx, mqy] = [mean((i) => p[i]![0]), mean((i) => p[i]![1]), mean((i) => q[i]![0]), mean((i) => q[i]![1])];
    let [sxx, sxy, syy, uxx, uxy, uyx, uyy] = [0, 0, 0, 0, 0, 0, 0];
    for (let i = 0; i < n; i++) {
      const [dx, dy, ex, ey] = [p[i]![0] - mpx, p[i]![1] - mpy, q[i]![0] - mqx, q[i]![1] - mqy];
      sxx += dx * dx; sxy += dx * dy; syy += dy * dy;
      uxx += ex * dx; uxy += ex * dy; uyx += ey * dx; uyy += ey * dy;
    }
    const det = sxx * syy - sxy * sxy;
    let A = [1, 0, 0, 1];
    if (Math.abs(det) > 1e-12 * (sxx + syy) ** 2 && det !== 0) A = [(uxx * syy - uxy * sxy) / det, (uxy * sxx - uxx * sxy) / det, (uyx * syy - uyy * sxy) / det, (uyy * sxx - uyx * sxy) / det];
    const da = A[0]! * A[3]! - A[1]! * A[2]!;
    if (Math.abs(da) < 1e-3) A = [1, 0, 0, 1];
    const inv = (() => { const dd = A[0]! * A[3]! - A[1]! * A[2]!; return [A[3]! / dd, -A[1]! / dd, -A[2]! / dd, A[0]! / dd]; })();
    let k = 0;
    const rings = flat.map((r) => r.map(() => {
      const i = k++;
      const [ex, ey] = [w[i]![0] - q[i]![0], w[i]![1] - q[i]![1]];
      return [p[i]![0] + inv[0]! * ex + inv[1]! * ey, p[i]![1] + inv[2]! * ex + inv[3]! * ey] as P;
    }));
    corrections.push({ scale, view: e.view, id: e.id, rings: rings.map((r) => r.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";")) });
  }
}
writeFileSync(out, JSON.stringify(corrections));
console.log(`[DEBUG] refine worst residual ${worst.toFixed(2)} features, corrections ${corrections.length}, bad ${bad}`);
