/** 🔍️ Round 3 red squirrel: the three 2D drawings of a pose enlarged (the product's `evaluate` placement, painted in z order with the tone as fill and the body pen as outline), as one SVG and, through QuickLook, one PNG. Usage: bun r3-red-squirrel-view2d.ts <asset document json> <scale> [pose] */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, scale, poseArg] = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const asset: any = JSON.parse(readFileSync(file!, "utf8")).asset;
const pose = poseArg ?? asset.presentation.defaultPose;
const state: any = evaluate(asset, { kind: "pose", pose } as never, { kind: "scale", scale: { denominator: Number(scale) } } as never);
const pen = (asset.representations[scale!].pens.body * Number(scale)) / 1000;
const W = 1200, H = 1200, K = 4600;
const hair = !process.argv.includes("--pen");
let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W * 2}" height="${H * 2}" viewBox="0 0 ${W * 2} ${H * 2}"><rect width="${W * 2}" height="${H * 2}" fill="#f6f3ea"/>`;
["elevation", "front", "plan"].forEach((view, n) => {
  const shapes = [...state.shapes[view]].sort((a: any, b: any) => a.shape.z - b.shape.z);
  const pts = shapes.flatMap((s: any) => s.rings.flat());
  const cx = (Math.min(...pts.map((p: any) => p.x)) + Math.max(...pts.map((p: any) => p.x))) / 2;
  const cy = (Math.min(...pts.map((p: any) => p.y)) + Math.max(...pts.map((p: any) => p.y))) / 2;
  const X = (p: any): string => `${(W * (n % 2) + W / 2 + (p.x - cx) * K).toFixed(1)},${(H * Math.floor(n / 2) + H / 2 - (p.y - cy) * K).toFixed(1)}`;
  for (const s of shapes) {
    const g = Math.round(40 + s.shape.tone * 200);
    const d = s.rings.map((r: any) => `M${r.map(X).join("L")}Z`).join("");
    svg += `<path d="${d}" fill="rgb(${g},${g - 8},${g - 20})" stroke="#111" stroke-width="${hair ? 2 : (pen * K).toFixed(1)}" stroke-linejoin="round" fill-rule="evenodd"/>`;
  }
  svg += `<text x="${W * (n % 2) + 12}" y="${H * Math.floor(n / 2) + 30}" font-family="Helvetica" font-size="18">${view} 1:${scale} ${pose} (${shapes.length} shapes)</text>`;
});
svg += "</svg>";
const dir = "🗑️generated/r3-red-squirrel";
mkdirSync(dir, { recursive: true });
const name = `view2d.s${scale}.${pose}.svg`;
writeFileSync(`${dir}/${name}`, svg);
execFileSync("qlmanage", ["-t", "-s", "2400", "-o", dir, `${dir}/${name}`], { stdio: "ignore" });
console.log(`${dir}/${name}.png`);
