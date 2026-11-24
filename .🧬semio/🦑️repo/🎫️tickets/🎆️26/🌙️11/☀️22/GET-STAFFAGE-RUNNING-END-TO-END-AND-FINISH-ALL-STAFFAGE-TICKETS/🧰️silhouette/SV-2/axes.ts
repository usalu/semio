/** 📐️ The extent of the body of a plant per authored scale and axis against the declared dimensions, with the solids that reach furthest on each side; `--plan` proposes the declared value every scale accepts (within `--share` of its smallest feature, default 0.5) or names the scales whose volume has to move.
 *  bun axes.ts <slug>… [--plan] [--top 3] [--share 0.5] */
import { parseArgs } from "node:util";
import { bodyOf, featureOf, load, stateOf } from "./lib.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { plan: { type: "boolean" }, top: { type: "string" }, share: { type: "string" } } });
const NAMES = ["widthM", "heightM", "depthM"] as const;
for (const slug of positionals) {
  const { asset } = await load(slug);
  const dims = NAMES.map((n) => asset.dimensions[n] as number);
  const scales = [50, 100, 200, 500].filter((d) => asset.volumes[String(d)]?.status === "authored");
  const ext: Record<number, { lo: number[]; hi: number[] }> = {};
  for (const scale of scales) {
    const solids = bodyOf(asset, stateOf(asset, scale), scale);
    const per = solids.map((s: any) => { const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity]; for (let i = 0; i < s.positions.length; i += 3) for (let k = 0; k < 3; k++) { const v = s.positions[i + k]; if (v < lo[k]!) lo[k] = v; if (v > hi[k]!) hi[k] = v; } return { id: s.id as string, bone: s.bone as string, lo, hi }; });
    const lo = [0, 1, 2].map((k) => Math.min(...per.map((p) => p.lo[k]!))), hi = [0, 1, 2].map((k) => Math.max(...per.map((p) => p.hi[k]!)));
    ext[scale] = { lo, hi };
    const f = featureOf(scale), n = Number(values.top ?? 3);
    for (let k = 0; k < 3; k++) {
      const off = Math.abs(hi[k]! - lo[k]! - dims[k]!) > f;
      console.log(`${slug} 1:${scale} ${NAMES[k]} ${(hi[k]! - lo[k]!).toFixed(4)} / ${dims[k]}${off ? " FAIL" : ""}  [${lo[k]!.toFixed(4)} .. ${hi[k]!.toFixed(4)}]`);
      console.log(`    low:  ${per.slice().sort((a, b) => a.lo[k]! - b.lo[k]!).slice(0, n).map((p) => `${p.id}[${p.bone}] ${p.lo[k]!.toFixed(4)}`).join("; ")}`);
      console.log(`    high: ${per.slice().sort((a, b) => b.hi[k]! - a.hi[k]!).slice(0, n).map((p) => `${p.id}[${p.bone}] ${p.hi[k]!.toFixed(4)}`).join("; ")}`);
    }
  }
  if (values.plan) for (let k = 0; k < 3; k++) {
    const share = Number(values.share ?? 0.5);
    const size = (d: number): number => ext[d]!.hi[k]! - ext[d]!.lo[k]!;
    const low = Math.max(...scales.map((d) => size(d) - share * featureOf(d))), high = Math.min(...scales.map((d) => size(d) + share * featureOf(d)));
    if (low <= high) { const keep = dims[k]! >= low && dims[k]! <= high; console.log(`PLAN ${slug} ${NAMES[k]}: ${keep ? `keep ${dims[k]}` : `declare ${Number(Math.min(high, Math.max(low, size(scales[0]!))).toFixed(3))}`} (accepted ${low.toFixed(4)} .. ${high.toFixed(4)})`); }
    else console.log(`PLAN ${slug} ${NAMES[k]}: the scales disagree (${scales.map((d) => `1:${d} ${size(d).toFixed(4)}`).join(", ")}): volumes must move`);
  }
}
