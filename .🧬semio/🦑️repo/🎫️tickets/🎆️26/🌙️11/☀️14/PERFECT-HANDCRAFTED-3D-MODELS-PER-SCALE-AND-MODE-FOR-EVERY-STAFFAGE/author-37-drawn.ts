import { openCtx, drawn } from "./kit-lib.ts";
const ctx: any = openCtx(process.argv[2]!, Number(process.argv[3] ?? 50));
const view = process.argv[4] ?? "elevation";
const shapes: any[] = drawn(ctx, view as any);
for (const s of shapes) {
  const pts = s.rings.flat();
  const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
  console.log(String(s.shape?.id ?? s.id).padEnd(22), String(s.bone).padEnd(12), "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3), "n", pts.length, s.closed ? "closed" : "open", s.shape?.filled);
}
