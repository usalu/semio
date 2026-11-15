import { loadAsset, resolveAsset, restState } from "./volumes-base.ts";
const [slug, scale, view, id] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", string];
const st: any = restState(loadAsset(resolveAsset(slug).path), Number(scale));
for (const e of st[view]) if (e.shape.id === id) { console.log(e.shape.id, e.closed, e.rings.length, "pts", e.rings.map((r: any) => r.length)); console.log(e.rings.map((r: any) => r.map((p: any) => `${p.x.toFixed(2)},${p.y.toFixed(2)}`).join(" ")).join(" | ")); }
