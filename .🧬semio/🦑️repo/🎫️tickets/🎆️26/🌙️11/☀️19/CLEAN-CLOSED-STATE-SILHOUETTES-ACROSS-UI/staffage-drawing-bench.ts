import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { depictionOptions, evaluateState, inlineSvg, sceneOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧮️pipeline/🟦️.ts";
import type { Asset, Manifest, View } from "@teaching/staffage";

const catalog = resolve(import.meta.dirname, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const manifest = JSON.parse(readFileSync(resolve(catalog, "🔣️.json"), "utf8")) as Manifest;
const categories = readdirSync(catalog, { withFileTypes: true }).filter(entry => entry.isDirectory()).map(entry => entry.name);
const samples = new Set(manifest.collections.find(collection => collection.id === "street-life")!.assets);
for (const category of manifest.categories) {
  const entries = manifest.entries.filter(entry => entry.category === category.id);
  const heights = entries.map(entry => entry.heightM).sort((a, b) => a - b);
  const cap = Math.max(4, 2.5 * (heights[Math.floor(heights.length / 2)] ?? 0));
  for (const entry of entries.filter(entry => entry.heightM <= cap).slice(0, 4)) samples.add(entry.id);
}
const fps = 12;
const frameCount = 24;
const ms = (values: readonly number[]): string => (values.toSorted((a, b) => a - b)[Math.ceil(values.length * 0.95) - 1] ?? 0).toFixed(2);
const all: number[] = [];
let maxCold = { ms: 0, id: "" };
for (const id of samples) {
  const entry = manifest.entries.find(entry => entry.id === id)!;
  const folder = categories.find(name => name.endsWith(categoryFolder(entry.category)));
  if (!folder) continue;
  let asset: Asset;
  try {
    asset = (JSON.parse(readFileSync(resolve(catalog, folder, id, "🔣️.json"), "utf8")) as { asset: Asset }).asset;
  } catch { continue; }
  const clip = asset.clips.find(clip => clip.id === asset.presentation.defaultClip);
  if (!clip) continue;
  for (const view of ["dark-line", "dark-solid"] as const satisfies readonly View[]) {
    const options = depictionOptions({ view, viewpoint: "elevation", scale: { denominator: 100 }, language: "en", transparent: true,
      annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: view === "dark-solid" } });
    const cold: number[] = [];
    const warm: number[] = [];
    const evaluate: number[] = [];
    const depict: number[] = [];
    const svg: number[] = [];
    let maxBytes = 0;
    try {
      for (const run of [cold, warm]) for (let frame = 0; frame < frameCount; frame++) {
        const timeS = clip.loop && clip.durationS > 0 ? Math.round((frame / fps % clip.durationS) * 1e9) / 1e9 : frame / fps;
        const start = performance.now();
        const state = evaluateState(asset, { kind: "clip", clip: clip.id, timeS }, options.representation);
        const evaluated = performance.now();
        const scene = sceneOf(state, options);
        const depicted = performance.now();
        maxBytes = Math.max(maxBytes, inlineSvg(scene).length);
        const done = performance.now();
        if (run === cold) { evaluate.push(evaluated - start); depict.push(depicted - evaluated); svg.push(done - depicted); }
        run.push(done - start);
      }
    } catch (error) { console.log(`[DEBUG] ${id} ${view}: ${String(error)}`); continue; }
    const slowest = Math.max(...cold);
    if (slowest > maxCold.ms) maxCold = { ms: slowest, id: `${id} ${view}` };
    all.push(...cold);
    console.log(`[DEBUG] ${id} ${view} ${clip.durationS}s: cold ${ms(cold)}ms (evaluate ${ms(evaluate)}, depict ${ms(depict)}, SVG ${ms(svg)}), repeat ${ms(warm)}ms, max ${slowest.toFixed(2)}ms, ${maxBytes} SVG chars`);
  }
}
console.log(`[DEBUG] ${samples.size} assets, ${all.length} frames, cold p95 ${ms(all)}ms, max ${maxCold.ms.toFixed(2)}ms (${maxCold.id})`);

function categoryFolder(id: string): string {
  return manifest.categories.find(category => category.id === id)!.folder;
}
