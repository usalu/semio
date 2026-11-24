/** 🔬️ Why single cells of one view differ between the drawn and the volume mask: over all tiles, the cells only the volume has (how many of them only by the guard's sealing of hairlines, how many under a stroke of a slender solid) and the cells only the drawing has, with the first places.
 *  bun diag.ts <id|file.json> <scale> <view> */
import { Mask } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { scene, tileMasks, tiles } from "./lib.ts";
import { fileOf } from "./plants.ts";
const [name, scaleArg, view] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const sc = scene(asset, Number(scaleArg), view!);
let [volOnly, sealed, strokeOnly, drawnOnly] = [0, 0, 0, 0];
const places: string[] = [];
const seen: Record<string, number> = {};
for (const t of tiles(sc)) {
  const { drawn, vol, width, height } = tileMasks(sc, t);
  const [raw, quads] = [new Mask(t.box, sc.cell), new Mask(t.box, sc.cell)];
  for (const g of t.reference) { if (g.length === 1 && g[0]!.length === 3) raw.fill(g as never); else quads.fill(g as never); }
  const [i0, i1, j0, j1] = [Math.round((t.core.minX - t.box.minX) / sc.cell), Math.round((t.core.maxX - t.box.minX) / sc.cell), Math.round((t.core.minY - t.box.minY) / sc.cell), Math.round((t.core.maxY - t.box.minY) / sc.cell)];
  for (let j = j0; j < j1; j++) for (let i = i0; i < i1; i++) {
    const k = j * width + i;
    if (vol[k] && !drawn[k]) { volOnly++; const kind = quads.cells[k] ? "stroke" : raw.cells[k] ? "triangle" : "sealed"; if (kind === "sealed") sealed++; if (kind === "stroke") strokeOnly++; if (kind !== "sealed" && (seen[kind] = (seen[kind] ?? 0) + 1) % 40 === 1) places.push(`volume-only ${kind} (${(t.box.minX + (i + 0.5) * sc.cell).toFixed(4)}, ${(t.box.minY + (j + 0.5) * sc.cell).toFixed(4)})`); }
    if (!vol[k] && drawn[k]) { drawnOnly++; if ((seen.drawn = (seen.drawn ?? 0) + 1) % 40 === 1) places.push(`drawn-only (${(t.box.minX + (i + 0.5) * sc.cell).toFixed(4)}, ${(t.box.minY + (j + 0.5) * sc.cell).toFixed(4)})`); }
  }
  void height;
}
console.log(`${asset.id} 1:${sc.scale} ${view}: cell ${(sc.cell * 1000).toFixed(2)} mm, ${volOnly} volume-only cells (${sealed} sealed, ${strokeOnly} under a stroke), ${drawnOnly} drawn-only cells`);
for (const p of places) console.log(`   ${p}`);
