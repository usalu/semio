/** ⚖️ Measures what one cached instant of a figure costs: the points of its evaluated side elevation, the characters of its markup and the heap it holds (state, scene and markup together). Usage: bun state-weight.ts <category-folder/asset> ... */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { Asset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { depictionOptions, evaluateState, inlineSvg, sceneOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧮️pipeline/🟦️.ts";

const catalog = resolve(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const options = depictionOptions({ view: "dark-line", viewpoint: "elevation", scale: { denominator: 100 }, language: "en" });
for (const name of process.argv.slice(2)) {
  const asset = (JSON.parse(readFileSync(resolve(catalog, name, "🔣️.json"), "utf8")) as { asset: Asset }).asset;
  const clip = asset.clips.find((candidate) => candidate.id === asset.presentation.defaultClip);
  const kept: unknown[] = [];
  let points = 0, characters = 0;
  Bun.gc(true);
  const before = process.memoryUsage().heapUsed;
  const frames = 40;
  for (let frame = 0; frame < frames; frame += 1) {
    const state = evaluateState(asset, { kind: "clip", clip: asset.presentation.defaultClip, timeS: 100 + frame / 30 }, options.representation);
    const markup = inlineSvg(sceneOf(state, options));
    kept.push(state, markup);
    points += state.shapes.elevation.reduce((sum, shape) => sum + shape.rings.reduce((ring, at) => ring + at.length, 0), 0);
    characters += markup.length;
  }
  Bun.gc(true);
  const heap = process.memoryUsage().heapUsed - before;
  console.log(`[DEBUG] ${name}: clip ${clip?.id} ${clip?.durationS} s loop ${clip?.loop}; per instant ${Math.round(points / frames)} points, ${Math.round(characters / frames)} markup characters, ${Math.round(heap / frames / 1024)} kB heap; ${Math.round(heap / points)} B per point`);
}
