/** 🖼️ Author 28: PNG of the product's own silhouette masks (drawn area/ink vs volume). bun author-28-mask.ts <asset> <scale> [--source file] [--W 520] */
import { entryOf, loadSource, merged, featureM, TICKET, outPath } from "./volumes-lib.ts";
import { Canvas } from "./volumes-raster.ts";
import { writeFileSync, readFileSync } from "node:fs";
import { measureAsset } from "./🗑️generated/author-28/sil-copy.ts";
const a = process.argv.slice(2);
const spec = a[0]!, sc = Number(a[1]);
const si = a.indexOf("--source");
const { entry, asset } = entryOf(spec);
const src: any = si >= 0 ? JSON.parse(readFileSync(a[si + 1]!, "utf8")) : loadSource(entry)!;
const m: any = merged(asset, src);
const f = featureM(sc, m);
const W = a.includes("--W") ? Number(a[a.indexOf("--W") + 1]) : 520;
const panels: Canvas[] = [];
const out: string[] = [];
for (const view of ["elevation", "front", "plan"] as const) {
  const state = measureAsset(m, sc, m.presentation.defaultPose, undefined, f) as any;
  const mv = state.views[view];
  out.push(`${view} iou ${mv.iou.toFixed(4)} drawn ${mv.counts.drawn} volume ${mv.counts.volume} both ${mv.counts.both} p95 ${mv.p95M}`);
  break;
}
// per-view masks: re-run measureAsset capturing __dbg after each view is not possible in one call; run per view by monkeypatch order (elevation, front, plan)
const dbgs: any[] = [];
const g: any = globalThis;
let cnt = 0;
Object.defineProperty(g, "__dbg", { set(v) { dbgs.push(v); }, get() { return dbgs[dbgs.length - 1]; }, configurable: true });
const r = measureAsset(m, sc, m.presentation.defaultPose, undefined, f) as any;
const names = ["elevation", "front", "plan"];
console.log("dbgs", dbgs.length);
dbgs.slice(-3).forEach((d, i) => {
  const { area, ink, volume, box, cell } = d;
  const w = volume.width, h = volume.height;
  const k = W / w;
  const H = Math.round(h * k);
  const cv = new Canvas(W, H + 16, [255, 255, 255]);
  for (let j = 0; j < H; j++) for (let ii = 0; ii < W; ii++) {
    const si2 = Math.min(w - 1, Math.floor(ii / k)), sj = Math.min(h - 1, Math.floor((H - 1 - j) / k));
    const idx = sj * w + si2;
    const dr = area.cells[idx] | ink.cells[idx], v = volume.cells[idx];
    cv.set(ii, j + 16, dr && v ? [150, 210, 150] : area.cells[idx] ? [230, 60, 60] : ink.cells[idx] ? [240, 150, 150] : v ? [70, 110, 230] : [250, 250, 250]);
  }
  cv.text(2, 2, `${names[i]} IoU ${r.views[names[i]!].iou.toFixed(3)} cell ${(cell * 1000).toFixed(1)}mm  red=fill-only pink=ink-only blue=3D-only`, [0, 0, 0], 1);
  panels.push(cv);
});
const H = Math.max(...panels.map((p) => p.height));
const sheet = new Canvas(W * 3 + 8, H, [255, 255, 255]);
panels.forEach((p, i) => sheet.blit(p, i * (W + 4), 0));
const path = `${TICKET}/🗑️generated/author-28/mask-${spec.replace("/", "-")}.s${sc}.png`;
writeFileSync(path, sheet.png());
console.log(out.join("\n"));
console.log(path);
