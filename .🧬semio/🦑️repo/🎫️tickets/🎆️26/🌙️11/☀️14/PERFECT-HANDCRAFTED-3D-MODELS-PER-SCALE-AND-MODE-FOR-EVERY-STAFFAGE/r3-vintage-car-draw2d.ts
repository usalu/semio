/** 🖼️ Draws the 2D shapes of one scale (elevation, front, plan in `presentation.defaultPose`) of vehicles/vintage-car-1929 as one SVG under 🗑️generated/r3-vintage-car-1929/. Usage: bun r3-vintage-car-draw2d.ts <scale> */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { GENERATED, entryOf, poseState, VIEWS } from "./volumes-lib.ts";

const scale = Number(process.argv[2] ?? 50);
const { asset } = entryOf("vehicles/vintage-car-1929");
const st: any = poseState(asset, scale);
const K = 300;
const S = 6.3;
const origin: Record<string, [number, number]> = { elevation: [2.6, 1.95], plan: [2.6, 3.15], front: [2.6, 6.1] };
const parts: string[] = [];
for (const view of VIEWS) {
  const [cx, yOff] = origin[view]!;
  const list = [...st[view]].sort((a: any, b: any) => (a.shape.z ?? 0) - (b.shape.z ?? 0));
  for (const s of list) {
    if (s.rings.length === 0) continue;
    const d = s.rings.map((r: any[]) => "M" + r.map((p) => `${((cx + p.x) * K).toFixed(1)},${((yOff - p.y) * K).toFixed(1)}`).join("L") + (s.closed ? "Z" : "")).join("");
    const g = Math.round(255 * (s.shape.tone ?? 0.5));
    const fill = s.closed && s.shape.filled ? (s.shape.role === "glazing" ? "rgb(170,205,235)" : `rgb(${g},${g},${g})`) : "none";
    const stroke = s.shape.role === "texture" ? "#c33" : s.shape.role === "detail" ? "#036" : "#000";
    parts.push(`<path d="${d}" fill="${fill}" fill-rule="evenodd" stroke="${stroke}" stroke-width="1.2"/>`);
  }
  parts.push(`<text x="20" y="${(yOff - (view === "plan" ? 0.95 : 1.8)) * K}" font-size="26">${view} 1:${scale}</text>`);
}
const x0 = S;
const H = S;
const dir = join(GENERATED, "r3-vintage-car-1929");
mkdirSync(dir, { recursive: true });
const tag = process.argv[3] ?? "";
writeFileSync(join(dir, `2d-s${scale}${tag}.svg`), `<svg xmlns="http://www.w3.org/2000/svg" width="${x0 * K}" height="${H * K}"><rect width="100%" height="100%" fill="#fff"/>${parts.join("")}</svg>`);
process.stdout.write(join(dir, `2d-s${scale}${tag}.svg`) + "\n");
