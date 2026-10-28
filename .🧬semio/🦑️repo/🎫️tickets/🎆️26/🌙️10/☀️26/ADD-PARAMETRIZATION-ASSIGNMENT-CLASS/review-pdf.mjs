import { createRequire } from "node:module";
import { readFile, mkdir, writeFile } from "node:fs/promises";
import { dirname, join, basename } from "node:path";
import { pathToFileURL, fileURLToPath } from "node:url";

const require = createRequire(join(process.cwd(), "package.json"));
const canvas = require("@napi-rs/canvas");
for (const name of ["DOMMatrix", "ImageData", "Path2D"]) globalThis[name] ??= canvas[name];
const { getDocument } = await import(pathToFileURL(require.resolve("pdfjs-dist/legacy/build/pdf.mjs")).href);
const sharp = require("sharp");
const output = join(dirname(fileURLToPath(import.meta.url)), "🗑️generated", "review");
await mkdir(output, { recursive: true });
for (const input of process.argv.slice(2)) {
  const pdf = await getDocument({ data: new Uint8Array(await readFile(input)), useSystemFonts: true }).promise;
  const stem = basename(input, ".pdf");
  const pages = [];
  const thumbnails = [];
  for (let index = 1; index <= pdf.numPages; index++) {
    const page = await pdf.getPage(index);
    const viewport = page.getViewport({ scale: 1.3 });
    const surface = canvas.createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
    await page.render({ canvasContext: surface.getContext("2d"), viewport }).promise;
    const png = surface.toBuffer("image/png");
    const path = join(output, `${stem}-${index}.png`);
    await writeFile(path, png);
    const text = (await page.getTextContent()).items.filter(item => "str" in item);
    pages.push({ page: index, width: page.view[2], height: page.view[3], text: text.map(item => item.str).join(" "), outside: text.filter(item => item.str.trim() && (item.transform[4] < 0 || item.transform[5] < 0 || item.transform[4] + item.width > page.view[2] + 1 || item.transform[5] > page.view[3])).map(item => item.str) });
    thumbnails.push(await sharp(png).resize({ width: 420 }).png().toBuffer());
    page.cleanup();
  }
  const height = (await sharp(thumbnails[0]).metadata()).height;
  const sheet = await sharp({ create: { width: 840, height: height * Math.ceil(thumbnails.length / 2), channels: 4, background: "#dddddd" } }).composite(thumbnails.map((input, i) => ({ input, left: (i % 2) * 420, top: Math.floor(i / 2) * height }))).png().toBuffer();
  await writeFile(join(output, `${stem}-contact.png`), sheet);
  await writeFile(join(output, `${stem}.json`), JSON.stringify(pages, null, 2));
  console.log(JSON.stringify({ input, pages: pages.length, outside: pages.flatMap(page => page.outside), contact: join(output, `${stem}-contact.png`) }));
  await pdf.destroy();
}
