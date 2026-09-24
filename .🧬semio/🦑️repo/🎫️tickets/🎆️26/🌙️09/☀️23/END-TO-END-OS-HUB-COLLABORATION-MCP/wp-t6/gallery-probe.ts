/** 🖼️ Exercises the gallery matrix and measures one variant, so the measurement module runs before the full regeneration. */
import { measurePrintGalleryVariant, printGalleryMatrix, printGallerySource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🔬️probes/🟦️.ts";
const matrix = printGalleryMatrix();
console.log("variants", matrix.length, "sections", new Set(matrix.map((v) => v.section)).size, matrix[0]);
const variant = matrix.find((v) => v.id === (process.argv[2] ?? "viz-3/dark/en"))!;
console.log(printGallerySource(variant).split("\n")[1]);
const m = await measurePrintGalleryVariant(variant);
console.log(JSON.stringify({ pages: m.pages, hash: m.hash, kinds: Object.fromEntries(Object.entries(m.kinds).map(([k, v]) => [k, v.page])) }));
