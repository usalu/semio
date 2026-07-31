const MARGIN = 32;
const STRENGTH = 1.5;
const visible = (zoom, w = 800, h = 600) => Math.hypot((w * 0.5) / zoom, (h * 0.5) / zoom);
const fade = (zoom, step = 10) => {
  const coverage = visible(zoom) * MARGIN;
  const target = Math.max(step * 24, coverage);
  const cells = 2 ** Math.ceil(Math.log2(Math.max(target / step, 1)));
  return Math.max(coverage, step * cells);
};
const cornerAlpha = (zoom) => {
  const r = visible(zoom);
  const f = fade(zoom);
  const d = 1 - Math.min(r / f, 1);
  return d ** STRENGTH;
};
console.log("[DEBUG] ortho zoom → fade vs visible radius + corner alpha");
for (const zoom of [80, 40, 10, 2, 0.5, 0.1]) {
  const r = visible(zoom);
  const f = fade(zoom);
  const a = cornerAlpha(zoom);
  console.log(`[DEBUG] zoom=${zoom} visible=${r.toFixed(1)} fade=${f.toFixed(1)} covers=${f >= r} cornerAlpha=${a.toFixed(3)}`);
  if (!(f >= r * MARGIN * 0.99) || a < 0.9) {
    console.error("[DEBUG] FAIL coverage/opacity");
    process.exit(1);
  }
}
console.log("[DEBUG] ok: fade overfills viewport with clean corner opacity at every zoom");
