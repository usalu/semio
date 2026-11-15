import { Kit, bboxOfShapes } from "./author-02-lib.ts";
const [spec, S, view, id] = process.argv.slice(2);
const k = new Kit(spec!, Number(S));
const sh = (view === "E" ? k.E : view === "F" ? k.F : k.P).call(k, (s: any) => s.id === id);
const b = bboxOfShapes(sh);
const W = 70, H = 28;
const grid = Array.from({ length: H }, () => Array(W).fill(" "));
for (const s of sh) for (const r of s) for (let i = 0; i < r.length; i++) {
  const a = r[i]!, c = r[(i + 1) % r.length]!;
  for (let t = 0; t < 1; t += 0.2) { const x = a.x + (c.x - a.x) * t, y = a.y + (c.y - a.y) * t; const gx = Math.round(((x - b.minX) / (b.maxX - b.minX)) * (W - 1)), gy = Math.round(((b.maxY - y) / (b.maxY - b.minY)) * (H - 1)); grid[gy]![gx] = "#"; }
}
console.log(JSON.stringify(b), "pts", sh[0]![0]!.length);
console.log(grid.map((r) => r.join("")).join("\n"));
