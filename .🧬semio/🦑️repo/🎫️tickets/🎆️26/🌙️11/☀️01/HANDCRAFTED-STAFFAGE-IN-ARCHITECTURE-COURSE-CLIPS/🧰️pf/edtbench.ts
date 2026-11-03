import { makeScratch, squaredDistanceTransform } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";
const nx = 1600, ny = 1560;
const scratch = makeScratch(nx, ny);
const src = new Uint8Array(nx * ny);
const from = new Int32Array(ny).fill(nx), to = new Int32Array(ny).fill(-1);
for (let y = 0; y < ny; y++) for (let x = Math.floor(y * 1.0); x < Math.min(nx, y + 60); x++) { src[y * nx + x] = 1; from[y] = Math.min(from[y]!, x); to[y] = Math.max(to[y]!, x); }
for (const [name, win] of [["full", undefined], ["window r=20", { from, to, margin: 21, cap: 21 }]] as const) {
  for (let k = 0; k < 4; k++) { const t = performance.now(); squaredDistanceTransform(src, nx, ny, scratch, win as never); if (k === 3) console.log(name, "edt ms", (performance.now() - t).toFixed(0)); }
}
