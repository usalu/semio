import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const R = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/";
const { evaluate } = await import(R + "🧮️evaluation/🟦️.ts");
const { solveStructure } = await import(R + "🏗️structure/🟦️.ts");
const { solveThermal } = await import(R + "🌡️thermal/🟦️.ts");
const { svgMarkup } = await import(R + "📤️export/🖼️svg/🟦️.ts");
const { defaultDepictionOptions, depict } = await import(R + "🎨️depiction/🟦️.ts");
// args: file asset out scale dpi views(comma) [poses(comma|all|named)] [tag]
const [file, id, out, scale, dpi, viewsArg, posesArg = "named", tag = "rv"] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(file!, "utf8"));
const asset = doc.assets.find((a: any) => a.id === id);
const views = viewsArg!.split(",");
const poses = posesArg === "all" ? asset.poses : posesArg === "named" ? asset.poses.filter((p: any) => !p.key) : asset.poses.filter((p: any) => posesArg.split(",").includes(p.id));
const layout = process.env.ROW === "1" ? "row" : "grid";
const rows = poses.map((p: any) => {
  const st = evaluate(asset, { kind: "pose", pose: p.id });
  const an = { thermal: solveThermal(st), structure: solveStructure(st) };
  return { label: p.id, scenes: views.map((v) => { const o = defaultDepictionOptions(v, "en", Number(scale)); return depict(st, { ...o, annotations: { ...o.annotations, legend: false } }, an); }) };
});
const cellW = Math.max(...rows.flatMap((r: any) => r.scenes.map((s: any) => s.widthMm)));
const cellH = Math.max(...rows.flatMap((r: any) => r.scenes.map((s: any) => s.heightMm)));
let cells = "", W = 0, H = 0;
const text = (x: number, y: number, t: string) => `<text x="${x}" y="${y}" font-size="2.5" font-family="Helvetica, Arial, sans-serif" fill="#001117">${t}</text>`;
const put = (s: any, x: number, y: number) => `<svg x="${x}" y="${y}" width="${s.widthMm}" height="${s.heightMm}" viewBox="0 0 ${s.widthMm} ${s.heightMm}">${svgMarkup(s)}</svg>`;
if (layout === "row") {
  // views stacked? single view per file: all poses in one row
  rows.forEach((r: any, i: number) => { cells += put(r.scenes[0], i * cellW, 8) + text(i * cellW + 2, 5, r.label); });
  W = cellW * rows.length; H = 8 + cellH;
} else {
  rows.forEach((r: any, i: number) => { r.scenes.forEach((s: any, c: number) => { cells += put(s, 30 + c * cellW, 8 + i * cellH); }); cells += text(2, 8 + i * cellH + 6, r.label); });
  views.forEach((v, c) => (cells += text(30 + c * cellW + 2, 5, v)));
  W = 30 + cellW * views.length; H = 8 + cellH * rows.length;
}
const svg = `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" version="1.1" width="${W}mm" height="${H}mm" viewBox="0 0 ${W} ${H}"><rect width="${W}" height="${H}" fill="#ffffff"/>${cells}</svg>\n`;
mkdirSync(out!, { recursive: true });
const base = join(out!, `${id}--${tag}-1-${scale}`);
writeFileSync(base + ".svg", svg);
execFileSync("rsvg-convert", ["-d", dpi!, "-p", dpi!, "-f", "png", "-o", base + ".png", base + ".svg"]);
console.log(base + ".png", W.toFixed(0), "x", H.toFixed(0), "mm");
