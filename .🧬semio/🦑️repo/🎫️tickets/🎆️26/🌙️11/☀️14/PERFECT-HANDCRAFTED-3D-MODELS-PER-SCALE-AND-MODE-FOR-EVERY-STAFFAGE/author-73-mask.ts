/** 🖼️ Author 28: PNG of the product's own silhouette masks (drawn area/ink vs volume). bun author-28-mask.ts <asset> <scale> [--source file] [--W 520] */
import { entryOf, loadSource, merged, featureM, TICKET, outPath } from "./volumes-lib.ts";
import { Canvas } from "./volumes-raster.ts";
import { writeFileSync, readFileSync } from "node:fs";
import { measureAsset } from "./🗑️generated/author-73/sil-copy.ts";
const a = process.argv.slice(2);
const spec = a[0]!, sc = Number(a[1]);
const si = a.indexOf("--source");
const { entry, asset } = entryOf(spec);
const src: any = si >= 0 ? JSON.parse(readFileSync(a[si + 1]!, "utf8")) : loadSource(entry)!;
const m: any = merged(asset, src);
const f = featureM(sc, m);
const softR = new Set(m.thermal.regions.filter((r: any) => ['foliage', 'fur', 'feather', 'hair'].includes(r.kind)).map((r: any) => r.id));
const softIds = new Set<string>(m.volumes[String(sc)].thermal.flatMap((s: any) => (s.of !== undefined && softR.has(s.region) ? [s.of] : [])));
const W = a.includes("--W") ? Number(a[a.indexOf("--W") + 1]) : 520;
const panels: Canvas[] = [];
const out: string[] = [];
for (const view of ["elevation", "front", "plan"] as const) {
  const state = measureAsset(m, sc, m.presentation.defaultPose, undefined, f, softIds) as any;
  const mv = state.views[view];
  out.push(`${view} iou ${mv.iou.toFixed(4)} drawn ${mv.counts.drawn} volume ${mv.counts.volume} both ${mv.counts.both} p95 ${mv.p95M}`);
  break;
}
// per-view masks: re-run measureAsset capturing __dbg after each view is not possible in one call; run per view by monkeypatch order (elevation, front, plan)
const dbgs: any[] = [];
const g: any = globalThis;
let cnt = 0;
Object.defineProperty(g, "__dbg", { set(v) { dbgs.push(v); }, get() { return dbgs[dbgs.length - 1]; }, configurable: true });
const r = measureAsset(m, sc, m.presentation.defaultPose, undefined, f, softIds) as any;
const names = ["elevation", "front", "plan"];
console.log("dbgs", dbgs.map((d)=>`${d.volume.width}x${d.volume.height}@${(d.cell*1000).toFixed(1)}`).join(" "));
dbgs.slice(0, 3).forEach((d, i) => {
  const { area, ink, volume, box, cell } = d;
  const w = volume.width, h = volume.height;
  const k = W / w;
  const H = Math.round(h * k);
  const cv = new Canvas(W, H + 16, [255, 255, 255]);
  for (let j = 0; j < H; j++) for (let ii = 0; ii < W; ii++) {
    const si2 = Math.min(w - 1, Math.floor(ii / k)), sj = Math.min(h - 1, Math.floor((H - 1 - j) / k));
    const idx = sj * w + si2;
    const dr = area.cells[idx] | ink.cells[idx], v = volume.cells[idx]; const sf = d.soft ? d.soft.cells[idx] : 0;
    cv.set(ii, j + 16, sf && !(dr && v) ? [200, 60, 200] : dr && v ? (sf ? [120, 190, 120] : [150, 210, 150]) : area.cells[idx] ? [230, 60, 60] : ink.cells[idx] ? [240, 150, 150] : v ? [70, 110, 230] : [250, 250, 250]);
  }
  cv.text(2, 2, `${names[i]} IoU ${r.views[names[i]!].iou.toFixed(3)} cell ${(cell * 1000).toFixed(1)}mm  red=fill-only pink=ink-only blue=3D-only magenta=soft-mismatch`, [0, 0, 0], 1);
  panels.push(cv);
});

if (process.env.ASCII) {
  const cols = Number(process.env.ASCII);
  dbgs.slice(0, 3).forEach((d, i) => {
    if (process.env.VIEW && process.env.VIEW !== names[i]) return;
    const { area, ink, volume, cell, box } = d; const w = volume.width, h = volume.height;
    const k = Math.max(1, Math.ceil(w / cols)); const kv = k * 2;
    console.log(`--- ${names[i]} ${w}x${h} cell ${(cell*1000).toFixed(1)}mm, one char = ${k} x ${kv} cells = ${(k*cell*1000).toFixed(0)} x ${(kv*cell*1000).toFixed(0)} mm; origin minX ${box.minX.toFixed(3)} minY ${box.minY.toFixed(3)}; # both  r drawn-only  b 3D-only`);
    for (let j0 = h - 1; j0 >= 0; j0 -= kv) {
      let line = "";
      for (let i0 = 0; i0 < w; i0 += k) {
        let both = 0, dr = 0, vo = 0;
        for (let jj = Math.max(0, j0 - kv + 1); jj <= j0; jj++) for (let ii = i0; ii < Math.min(w, i0 + k); ii++) { const idx = jj * w + ii; const dd = area.cells[idx] | ink.cells[idx], v = volume.cells[idx]; if (dd && v) both++; else if (dd) dr++; else if (v) vo++; }
        line += dr + vo === 0 ? (both ? "#" : ".") : dr > vo ? (dr * 2 >= both + dr ? "R" : "r") : (vo * 2 >= both + vo ? "B" : "b");
      }
      console.log(line);
    }
  });
}
const H = Math.max(...panels.map((p) => p.height));
const sheet = new Canvas(W * 3 + 8, H, [255, 255, 255]);
panels.forEach((p, i) => sheet.blit(p, i * (W + 4), 0));
const path = `${TICKET}/🗑️generated/author-73/mask-${spec.replace("/", "-")}.s${sc}.png`;
writeFileSync(path, sheet.png());
console.log(out.join("\n"));
console.log(path);
