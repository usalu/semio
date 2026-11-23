/** 🖼️ A contact sheet of PNG files: a grid of `columns` cells of `cell` pixels (each image fitted into its cell on white), written as one PNG.
 *  bun contact.ts <out.png> <columns> <cell> <file.png>… */
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";

const [out, columnsArg, cellArg, ...files] = process.argv.slice(2);
const columns = Number(columnsArg), cell = Number(cellArg);
const rows = Math.ceil(files.length / columns);
const tiles = await Promise.all(files.map(async (file, i) => ({ input: await sharp(file).resize(cell, cell, { fit: "contain", background: "#ffffff" }).png().toBuffer(), left: (i % columns) * cell, top: Math.floor(i / columns) * cell })));
await sharp({ create: { width: columns * cell, height: rows * cell, channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(out!);
console.log(`${out}: ${files.length} images, ${columns * cell} x ${rows * cell}`);
