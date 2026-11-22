/** ⏱️ Times the phases of `r3-validate.ts <asset> --catalog-only` without touching the product: the print measure module is instrumented in memory while it is loaded (Bun `onLoad`), so every distance transform, flood and raster of the feature-width measurement is attributed to its call site.
 *
 * Usage: `[SPEED_OLD=1] bun validator-speed-prof.ts <category>/<slug>` (`SPEED_OLD=1` measures with the cover dilation as a whole-grid distance transform, the code before the windowed one, for back-to-back comparisons); prints one `[DEBUG] ` line per phase and per call site (seconds, calls) and a fingerprint of the `validateRanges` issues.
 * @see ./r3-validate.ts
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts */
import { readFileSync } from "node:fs";
import { plugin } from "bun";

type Timers = Record<string, { s: number; n: number }>;
const timers: Timers = {};
const scope = globalThis as unknown as { __T: <R>(name: string, run: () => R) => R; __C: (name: string, by?: number) => void };
scope.__T = (name, run) => {
  const t = performance.now();
  try {
    return run();
  } finally {
    const slot = (timers[name] ??= { s: 0, n: 0 });
    slot.s += (performance.now() - t) / 1000;
    slot.n++;
  }
};
scope.__C = (name, by = 1) => {
  const slot = (timers[name] ??= { s: 0, n: 0 });
  slot.n += by;
};

const keys = new Map<string, number[]>();
(scope as unknown as { __K: (tag: string, key: string, atLeast: number) => void }).__K = (tag, key, atLeast) => {
  const id = `${tag}|${key.length}|${Bun.hash(key)}`;
  const seen = keys.get(id);
  scope.__C(`${tag}-calls`);
  if (seen === undefined) keys.set(id, [atLeast]);
  else {
    seen.push(atLeast);
    scope.__C(`${tag}-repeat-calls`);
  }
};

const WINDOWED = "squaredDistanceTransform(union, nx, ny, cutScratch, { from: cutFrom, to: cutTo, margin: span, cap: span })";
const UNWINDOWED = "squaredDistanceTransform(union, nx, ny, cutScratch)";
const OLD = process.env.SPEED_OLD === "1";

const WRAPS: readonly (readonly [string, string, string])[] = [
  ["dt-cover-pen", "const reach = penMm > 0 ? ", OLD ? UNWINDOWED : WINDOWED],
  ["dt-outside", "const toOutside = ", "squaredDistanceTransform(outside, nx, ny, scratch, { from, to, margin: 1, cap: nx + ny })"],
  ["dt-core-bisection", "const reach = ", "squaredDistanceTransform(core, nx, ny, scratch, { from, to, margin: span, cap: span, sparse: true })"],
  ["drop-small-pieces", "    ", "dropSmallPieces(mask, nx, ny, Math.ceil((minFeatureMm / 2) ** 2 / (pitch * pitch)))"],
  ["rasterize-shape", "const mask = ", 'rasterize(rings, pitch, x0, y0, nx, ny, lease("mask", Uint8Array, nx * ny))'],
  ["rasterize-cover", "      ", "rasterize(cover.rings, pitch, x0, y0, nx, ny, union)"],
  ["measure-feature-total", "const { widthMm, exact, boxMinMm } = ", "measureFeatureWidthMm(rings, minFeatureMm, atLeastMm, covers)"],
  ["measure-feature-total", "const { widthMm, exact } = ", "measureFeatureWidthMm(rings, minFeatureMm, atLeastMm, [])"],
  ["white-sliver-dt", "", ""],
];

plugin({
  name: "validator-speed-instrument",
  setup(build) {
    build.onLoad({ filter: /representation\/[^/]*measure\/[^/]*\.ts$/ }, (args) => {
      let text = readFileSync(args.path, "utf8");
      if (!text.includes(WINDOWED)) throw new Error("[DEBUG] windowed cover transform not found");
      if (OLD) text = text.replace(WINDOWED, UNWINDOWED);
      for (const [name, before, call] of WRAPS) {
        if (call === "") continue;
        const at = text.indexOf(before + call);
        if (at < 0) throw new Error(`[DEBUG] call site ${name} not found`);
        text = `${text.slice(0, at)}${before}__T(${JSON.stringify(name)}, () => ${call})${text.slice(at + before.length + call.length)}`;
      }
      const cells = "const cut = covers.length > 0;";
      if (!text.includes(cells)) throw new Error("[DEBUG] cells site not found");
      text = text.replace(cells, `${cells} __C("cells-total", nx * ny); __C(cut ? "shapes-with-covers" : "shapes-without-covers");`);
      for (const [tag, site] of [["visible", "if (visibleMemo.has(key)) return visibleMemo.get(key)!;"], ["plain", "const known = featureMemo.get(key);"]] as const) {
        if (!text.includes(site)) throw new Error(`[DEBUG] key site ${tag} not found`);
        text = text.replace(site, `__K(${JSON.stringify(tag)}, key, atLeastMm); ${site}`);
      }
      const memo = "if (!exact || key.length > MEMO_KEY_LIMIT) return result;";
      if (!text.includes(memo)) throw new Error("[DEBUG] memo site not found");
      text = text.replace(memo, `if (!exact) __C("visible-not-memoised-inexact"); else if (key.length > MEMO_KEY_LIMIT) __C("visible-not-memoised-long-key"); ${memo}`);
      return { contents: `declare const __T: <R>(name: string, run: () => R) => R; declare const __C: (name: string, by?: number) => void; declare const __K: (tag: string, key: string, atLeast: number) => void;\n${text}`, loader: "ts" };
    });
  },
});

const spec = process.argv[2] ?? "humans/standing-adult";
const phase = async <R>(name: string, run: () => R | Promise<R>): Promise<R> => {
  const t = performance.now();
  const result = await run();
  console.log(`[DEBUG] phase ${name}: ${((performance.now() - t) / 1000).toFixed(2)} s`);
  return result;
};
const staffage = "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { resolveAsset } = await import("./volumes-lib.ts");
const catalog = await phase("import catalog", () => import(`${staffage}/🔨️modules/🗂️catalog/🟦️.ts`));
const files = await import(`${staffage}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const representation = await import(`${staffage}/🔨️modules/🖨️representation/🟦️.ts`);
const { referencedAssets } = await import("./r3-validate.ts");
const entry = resolveAsset(spec);
const document = await phase("json parse", () => JSON.parse(readFileSync(entry.path, "utf8")));
const asset = document.asset;
const environments = new Set(((await files.loadEnvironments())?.presets ?? []).map((p: { id: string }) => p.id));
const issues = await phase("validateAsset", () => catalog.validateAsset(asset, { assets: referencedAssets(asset), environments }));
const ranges = await phase("validateRanges", () => catalog.validateRanges(asset, representation.DEFAULT_PRINT_PROFILE));
const suitabilities: unknown[] = [];
await phase("measureSuitability (cached)", () => {
  for (const [scale, rep] of Object.entries(asset.representations ?? {}) as [string, { status: string; geometry?: unknown; validRange: { minDenominator: number; maxDenominator: number } }][])
    if (rep.status === "authored" && rep.geometry !== undefined)
      for (const at of new Set([Number(scale), rep.validRange.minDenominator, rep.validRange.maxDenominator])) {
        if (!Number.isFinite(at) || at <= 0) continue;
        const { minStrokeMm, minGapMm, minFeatureMm, violations } = representation.measureSuitability(asset, rep, at, representation.DEFAULT_PRINT_PROFILE);
        suitabilities.push([scale, at, `${minStrokeMm}`, `${minGapMm}`, `${minFeatureMm}`, violations]);
      }
});
for (const [name, slot] of Object.entries(timers).sort((a, b) => b[1].s - a[1].s)) console.log(`[DEBUG] site ${name}: ${slot.s.toFixed(2)} s, ${slot.n}`);
console.log(`[DEBUG] issues validateAsset ${issues.length}, validateRanges ${ranges.length}, fingerprint ${Bun.hash(JSON.stringify([issues, ranges])).toString(16)}, suitability at ${suitabilities.length} scales fingerprint ${Bun.hash(JSON.stringify(suitabilities)).toString(16)}`);
console.log(`[DEBUG] distinct shape keys ${keys.size}`);
