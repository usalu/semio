/** ⏱️ The cost of the union of a prism with a long profile, as `solid3` builds it (`unionPolygons([low, high, ...sweeps])`, one quad per profile edge): `bun prism-cost.ts <polygon module> <points…>` times a wavy strip outline of that many points (a hanging curtain: 0.08 m wide, 5 m long, swept 3 cm and 1.5 cm sideways) and prints the milliseconds, the number of rings and the area, so two versions of the polygon module can be compared on the same input.
 * @see ../../../multiview-MX-2.md — answer to the request of MV-3 */
const [module, ...counts] = process.argv.slice(2);
const { unionPolygons } = await import(module!);
for (const n of counts.map(Number)) {
  const half = n / 2;
  const side = (s: number): { x: number; y: number }[] => Array.from({ length: half }, (_, i) => ({ x: s * (0.04 + 0.01 * Math.sin(i * 0.7)), y: (i / (half - 1)) * 5 }));
  const left = side(-1);
  const right = side(1).reverse();
  const ring = [...left, ...right];
  const shift = (p: { x: number; y: number }, d: number) => ({ x: p.x + d * 0.03, y: p.y + d * 0.015 });
  const low = ring.map((p) => shift(p, 0));
  const high = ring.map((p) => shift(p, 1));
  const sweeps = low.map((p, i) => [p, low[(i + 1) % low.length]!, high[(i + 1) % high.length]!, high[i]!]);
  const started = performance.now();
  const rings = unionPolygons([low, high, ...sweeps]);
  const ms = performance.now() - started;
  const area = rings.reduce((sum: number, r: { x: number; y: number }[]) => sum + r.reduce((a: number, p, k) => a + p.x * r[(k + 1) % r.length]!.y - r[(k + 1) % r.length]!.x * p.y, 0) / 2, 0);
  console.log(`${n} points: ${ms.toFixed(0)} ms, ${rings.length} rings, area ${area.toFixed(6)}`);
}
