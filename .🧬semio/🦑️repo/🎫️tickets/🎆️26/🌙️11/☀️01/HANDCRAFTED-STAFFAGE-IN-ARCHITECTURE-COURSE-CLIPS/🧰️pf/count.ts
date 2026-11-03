import { resolve } from "node:path";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles, loadEnvironments } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermalAt, thermalSession } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const presets = new Map(((await loadEnvironments(DEFAULT_CATALOG_ROOT))?.presets ?? []).map((p) => [p.id, p.environment]));
for (const [assetId, clip, preset, step, n] of [["toaster", "toast-cycle", "indoor-heated-winter", 5, 12], ["walking-commuter", "walk-cycle", "winter-overcast", 0.5, 12], ["server-rack", "power-up", "office-neutral", 3, 12]] as const) {
  const asset = catalog.assets.find((a) => a.id === assetId)!;
  for (const shared of [false, true]) {
    let calls = 0;
    const session = shared ? thermalSession() : undefined;
    const t0 = performance.now();
    for (let i = 0; i < n; i++) solveThermalAt(asset, { kind: "clip", clip, timeS: i * step }, presets.get(preset)!, { evaluate: (a, q) => (calls++, evaluate(a, q)), ...(session ? { session } : {}) });
    console.log(assetId, shared ? "shared" : "alone ", calls, (performance.now() - t0).toFixed(0), "ms");
  }
}
