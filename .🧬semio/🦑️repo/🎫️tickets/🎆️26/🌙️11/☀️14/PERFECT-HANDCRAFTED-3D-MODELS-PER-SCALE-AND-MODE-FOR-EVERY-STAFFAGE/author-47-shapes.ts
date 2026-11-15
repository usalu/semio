import { openCtx, drawn, flag } from "./kit-lib.ts";
const a = process.argv.slice(2);
const c = openCtx(a[0]!, Number(flag(a, "--scale")));
const full = a.includes("--list");
const inside = (r: any[], x: number, y: number) => { let w = false; for (let i = 0, j = r.length - 1; i < r.length; j = i++) { if ((r[i].y > y) !== (r[j].y > y) && x < ((r[j].x - r[i].x) * (y - r[i].y)) / (r[j].y - r[i].y) + r[i].x) w = !w; } return w; };
for (const v of ["elevation", "front", "plan"] as const) {
  const sh = drawn(c, v);
  let X0 = 1e9, X1 = -1e9, Y0 = 1e9, Y1 = -1e9;
  for (const s of sh) { let x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9; for (const r of s.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); } X0 = Math.min(X0, x0); X1 = Math.max(X1, x1); Y0 = Math.min(Y0, y0); Y1 = Math.max(Y1, y1);
    if (full) console.log(" ", s.id, s.bone, s.region ?? "", `x[${x0.toFixed(3)},${x1.toFixed(3)}] y[${y0.toFixed(3)},${y1.toFixed(3)}] pts ${s.rings[0]!.length}`); }
  console.log("VIEW", v, `x[${X0.toFixed(3)},${X1.toFixed(3)}] y[${Y0.toFixed(3)},${Y1.toFixed(3)}]`, sh.length, "shapes");
  const W = 64, H = Math.max(4, Math.round((W * (Y1 - Y0)) / (X1 - X0) / 2));
  for (let j = H - 1; j >= 0; j--) { let row = ""; for (let i = 0; i < W; i++) { const x = X0 + ((i + 0.5) / W) * (X1 - X0), y = Y0 + ((j + 0.5) / H) * (Y1 - Y0); row += sh.some((s) => s.rings.some((r) => inside(r, x, y))) ? "#" : "."; } console.log(row); }
}
