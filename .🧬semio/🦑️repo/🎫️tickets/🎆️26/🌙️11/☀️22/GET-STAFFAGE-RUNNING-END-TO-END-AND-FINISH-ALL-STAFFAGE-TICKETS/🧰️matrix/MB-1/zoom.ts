/** 🔍️ A close look at the evaluated shapes of the three views of one pose at one scale, each view fitted to its own panel (fills by tone in the order of `z`, every outline as a thin stroke, added shapes of a list in red): bun zoom.ts <id> <scale> <out.png> [pose|-] [red ids, comma separated] [views] */
import { join } from "node:path";
import sharp from "sharp";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [id, scale, out, pose = "-", red = "", only = "elevation,front,plan"] = process.argv.slice(2);
const marked = new Set(red.split(",").filter(Boolean));
const asset = engine.loadCatalog().find((a: any) => a.id === id);
const state = evaluate(asset, { kind: "pose", pose: pose === "-" ? asset.presentation.defaultPose : pose }, engine.scaleSelector(Number(scale)));
const views = only.split(",");
const PW = Math.floor(1560 / views.length), PH = 520, PAD = 24;
let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${PW * views.length}" height="${PH}"><rect width="100%" height="100%" fill="#ffffff"/>`;
views.forEach((view, n) => {
  const shapes = [...state.shapes[view]].filter((s: any) => s.shape.role !== "ground").sort((a: any, b: any) => a.shape.z - b.shape.z);
  const e = engine.extentOf(shapes);
  const k = Math.min((PW - 2 * PAD) / Math.max(e.maxX - e.minX, 1e-6), (PH - 2 * PAD - 16) / Math.max(e.maxY - e.minY, 1e-6));
  const px = (p: any): string => `${(n * PW + PAD + (p.x - e.minX) * k).toFixed(1)},${(PH - PAD - (p.y - e.minY) * k).toFixed(1)}`;
  svg += `<text x="${n * PW + 6}" y="14" font-family="Helvetica" font-size="12">${id} 1:${scale} ${view} (${((e.maxX - e.minX) * 1000).toFixed(0)} x ${((e.maxY - e.minY) * 1000).toFixed(0)} mm)</text>`;
  for (const s of shapes) {
    const d = s.rings.map((ring: any[]) => `M ${ring.map(px).join(" L ")}${s.closed ? " Z" : ""}`).join(" ");
    const tone = Math.round(255 * (0.45 + 0.55 * (s.shape.tone ?? 0.5)));
    const fill = s.closed && s.shape.filled ? `rgb(${tone},${tone},${tone})` : "none";
    svg += `<path d="${d}" fill="${fill}" fill-rule="evenodd" stroke="${marked.has(s.shape.id) ? "#d00000" : "#000000"}" stroke-width="${marked.has(s.shape.id) ? 1.6 : 0.8}"/>`;
  }
});
await sharp(Buffer.from(svg + "</svg>")).png().toFile(out);
console.log(out);
