import { heapStats } from "bun:jsc";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [id, scale] = process.argv.slice(2);
const mb = (n: number): number => Math.round(n / 1048576);
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const asset = catalog.assets.find((a) => a.id === id)!;
const rep = asset.representations[scale as "50"] as never;
const rss0 = process.resourceUsage().maxRSS / 1024;
let calls = 0;
let liveMax = 0;
const sort = Array.prototype.sort;
Array.prototype.sort = function (this: unknown[], ...args: never[]) {
  if (process.env.SAMPLE && ++calls % 4000 === 0) {
    Bun.gc(true);
    liveMax = Math.max(liveMax, process.memoryUsage().rss);
  }
  return sort.apply(this, args);
} as never;
const start = performance.now();
const result = measureSuitability(asset, rep, Number(scale), DEFAULT_PRINT_PROFILE);
const ms = performance.now() - start;
const h = heapStats();
console.log(JSON.stringify({ id, scale, ms: Math.round(ms), rssBeforeMB: Math.round(rss0), peakRssMB: Math.round(process.resourceUsage().maxRSS / 1024), heapMB: mb(h.heapSize), extraMB: mb(h.extraMemorySize), liveMaxMB: Math.round(liveMax / 1048576), suitable: result.suitable, violations: result.violations.length, minFeature: result.minFeatureMm, minGap: result.minGapMm }));
