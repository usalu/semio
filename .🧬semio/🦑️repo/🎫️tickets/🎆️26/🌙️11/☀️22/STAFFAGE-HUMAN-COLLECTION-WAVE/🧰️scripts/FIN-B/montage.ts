/** 🧩️ One 2 × 2 montage per folder of multiview sheets (1:50, 1:100, 1:200, 1:500; title block cropped away), remaining SVGs rasterised first.
 *  bun montage.ts <folder> ... */
import { readdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const CELL = 800;
for (const folder of process.argv.slice(2)) {
  for (const f of readdirSync(folder).filter((n) => n.endsWith(".svg"))) {
    await sharp(join(folder, f), { density: 600, limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toFile(join(folder, f.replace(/\.svg$/u, ".png")));
    rmSync(join(folder, f));
  }
  const files = readdirSync(folder).filter((n) => n.endsWith(".png") && !n.startsWith("montage"));
  const cells: { input: Buffer; left: number; top: number }[] = [];
  for (const [i, d] of ["50", "100", "200", "500"].entries()) {
    const f = files.find((n) => n.endsWith(`-1-${d}.png`)) ?? files.find((n) => n.includes(`--1-${d}--explanation`));
    if (f === undefined) continue;
    const image = sharp(join(folder, f));
    const meta = await image.metadata();
    const crop = f.includes("explanation") ? image : image.extract({ left: 0, top: 0, width: meta.width!, height: Math.round(meta.height! * 0.76) });
    cells.push({ input: await crop.resize(CELL, CELL, { fit: "contain", background: "#ffffff" }).png().toBuffer(), left: (i % 2) * CELL, top: Math.floor(i / 2) * CELL });
  }
  await sharp({ create: { width: 2 * CELL, height: 2 * CELL, channels: 3, background: "#ffffff" } }).composite(cells).png().toFile(join(folder, "montage.png"));
  console.log(join(folder, "montage.png"));
}
