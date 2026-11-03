import { readdirSync } from "node:fs";
import { load, CAT, stateAt, bands } from "./lib.ts";
const inside = (px: number, py: number, ring: any[]) => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const a = ring[i], b = ring[j]; if ((a.y > py) !== (b.y > py) && px < ((b.x - a.x) * (py - a.y)) / (b.y - a.y) + a.x) c = !c; } return c; };
export const LEAFY = /leaf|leaves|needle|blade|frond|plume|curtain|crown|canopy/i;
export function coverage(asset: any, den: number) {
  const st = stateAt(asset, den); const D = asset.dimensions;
  const crown = st.shapes.plan.filter((s: any) => s.shape.role === "body" && (LEAFY.test(s.shape.region ?? (s.shape.regions ?? []).join(" ")) || /^(fol|curt|cap)/.test(s.shape.bone)));
  const pts = crown.flatMap((s: any) => s.rings.flat()); if (!pts.length) return { cov: -1, holes: [] as any[] };
  const x0 = Math.min(...pts.map((p: any) => p.x)), x1 = Math.max(...pts.map((p: any) => p.x)), y0 = Math.min(...pts.map((p: any) => p.y)), y1 = Math.max(...pts.map((p: any) => p.y));
  const cx = (x0 + x1) / 2, cy = (y0 + y1) / 2, ax = (x1 - x0) / 2, ay = (y1 - y0) / 2; let n = 0, ok = 0; const holes: any[] = [];
  for (let i = 0; i < 60; i++) for (let j = 0; j < 60; j++) { const px = x0 + ((i + 0.5) / 60) * (x1 - x0), py = y0 + ((j + 0.5) / 60) * (y1 - y0); if (((px - cx) / ax) ** 2 + ((py - cy) / ay) ** 2 > 0.8) continue; n++; if (crown.some((s: any) => s.rings.some((r: any) => inside(px, py, r)))) ok++; else holes.push([px, py]); }
  return { cov: ok / n, holes, crown };
}
if (import.meta.main) for (const d of readdirSync(CAT)) { const { asset } = load(d); console.log(d.padEnd(24), bands(asset).map((b: number) => `${b}:${coverage(asset, b).cov.toFixed(2)}`).join(" ")); }
