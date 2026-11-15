/** bun author-63-clusters.ts <asset> <scale> [view 0|1|2] : connected mismatch clusters (3D-only blue / drawn-only red) with world bbox and area, largest first */
import { entryOf, loadSource, merged, featureM } from "./volumes-lib.ts";
import { measureAsset } from "./🗑️generated/author-63/sil-copy.ts";
const [spec, sc, vi = "0"] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m: any = merged(asset, loadSource(entry)!);
const f = featureM(Number(sc), m);
const dbgs: any[] = [];
Object.defineProperty(globalThis, "__dbg", { set(v) { dbgs.push(v); }, get() { return dbgs[dbgs.length - 1]; }, configurable: true });
measureAsset(m, Number(sc), m.presentation.defaultPose, undefined, f);
const d = dbgs[Number(vi)];
const { area, ink, volume, box, cell } = d; const w = volume.width, h = volume.height;
const lab = new Int8Array(w * h);
for (let k = 0; k < w * h; k++) { const dr = area.cells[k] | ink.cells[k], v = volume.cells[k]; lab[k] = dr && v ? 0 : v ? 1 : dr ? 2 : 0; }
const seen = new Uint8Array(w * h); const out: any[] = [];
for (let k = 0; k < w * h; k++) if (lab[k] && !seen[k]) {
  const t = lab[k]!; const st = [k]; seen[k] = 1; let n = 0, x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9;
  while (st.length) { const q = st.pop()!; n++; const i = q % w, j = (q / w) | 0; x0 = Math.min(x0, i); x1 = Math.max(x1, i); y0 = Math.min(y0, j); y1 = Math.max(y1, j);
    for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { const a = i + di!, b = j + dj!; if (a < 0 || b < 0 || a >= w || b >= h) continue; const r = b * w + a; if (!seen[r] && lab[r] === t) { seen[r] = 1; st.push(r); } } }
  out.push({ t: t === 1 ? "3D-only" : "drawn-only", area: +(n * cell * cell * 1e4).toFixed(0), x: [+(box.minX + x0 * cell).toFixed(3), +(box.minX + (x1 + 1) * cell).toFixed(3)], y: [+(box.minY + y0 * cell).toFixed(3), +(box.minY + (y1 + 1) * cell).toFixed(3)] });
}
out.sort((a, b) => b.area - a.area);
for (const o of out.slice(0, 14)) console.log(o.t.padEnd(10), String(o.area).padStart(5), "cm2  x", o.x.join(".."), " y", o.y.join(".."));
