/** 🪞️ Two sheets of `wind.ts` against each other, cell by cell of the 3 × 3 grid (poses as rows; side, front, plan as columns): `bun compare.ts <before.png> <now.png> [<out folder>]` (run from the folder of the 3D ticket, whose `sharp` it uses). Prints per cell the dark pixels of both and the pixels that differ; with a folder it writes per cell that differs a strip `before | now | difference`. Read-only.
 * @see ./wind.ts — the writer of the sheets */
import { mkdirSync } from "node:fs";
import { basename, join } from "node:path";

const sharp: any = (await import(Bun.resolveSync("sharp", `${process.cwd()}/`))).default;
const [before, now, folder] = process.argv.slice(2);
const grey = async (path: string) => sharp(path).greyscale().raw().toBuffer({ resolveWithObject: true });
const a = await grey(before!);
const b = await grey(now!);
if (a.info.width !== b.info.width || a.info.height !== b.info.height) throw new Error("the sheets differ in size");
const { width, height } = a.info;
if (folder) mkdirSync(folder, { recursive: true });
for (let row = 0; row < 3; row++) {
  for (let column = 0; column < 3; column++) {
    const x0 = Math.floor((column * width) / 3), y0 = Math.floor((row * height) / 3), w = Math.floor(width / 3), h = Math.floor(height / 3);
    const strip = Buffer.alloc(w * 3 * h, 255);
    let darkBefore = 0, darkNow = 0, differ = 0;
    for (let y = 0; y < h; y++) {
      for (let x = 0; x < w; x++) {
        const p = a.data[(y0 + y) * width + x0 + x]!, q = b.data[(y0 + y) * width + x0 + x]!;
        if (p < 90) darkBefore++;
        if (q < 90) darkNow++;
        const off = Math.abs(p - q) > 60;
        if (off) differ++;
        strip[y * w * 3 + x] = p;
        strip[y * w * 3 + w + x] = q;
        strip[y * w * 3 + 2 * w + x] = off ? 0 : 255;
      }
    }
    console.log(`row ${row} column ${column}: dark before ${darkBefore}, now ${darkNow}, differing ${differ}`);
    if (folder && differ > 0) await sharp(strip, { raw: { width: w * 3, height: h, channels: 1 } }).png().toFile(join(folder, `${basename(now!, ".png")}.r${row}c${column}.png`));
  }
}
