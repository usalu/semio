const { posed } = await import("./trace.ts");
const [file, sc, view, xs, ys] = process.argv.slice(2);
const a = (await Bun.file(file!).json()).asset;
const { state } = posed(a, sc!);
const P = { x: Number(xs), y: Number(ys) };
const inside = (ring: any[]) => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const [p, q] = [ring[i], ring[j]]; if ((p.y > P.y) !== (q.y > P.y) && P.x < ((q.x - p.x) * (P.y - p.y)) / (q.y - p.y) + p.x) c = !c; } return c; };
for (const s of state.shapes[view!]) {
  const hit = s.rings.some((r: any) => inside(r));
  const near = Math.min(...s.rings.flat().map((q: any) => Math.hypot(q.x - P.x, q.y - P.y)));
  if (hit || near < 0.01) console.log(s.shape.id, s.shape.filled, s.closed, hit, (near * 1000).toFixed(1));
}
