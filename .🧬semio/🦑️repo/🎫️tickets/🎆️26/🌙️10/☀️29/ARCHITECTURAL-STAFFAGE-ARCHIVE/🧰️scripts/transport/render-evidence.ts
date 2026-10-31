/** Renders the transport evidence frames and clips. Usage: bun render-evidence.ts <outDir> [case...]; cases: commuter, headturn, excavator, oak. Also proves export = scrub (scene JSON equal in play order, reverse order and with a cold cache). */
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync, rmSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { loadCatalogFromFiles } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveTransport } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/💨️transport/🟦️.ts";
import { defaultDepictionOptions, depict } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { printProfile } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { writeSvg } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { fixedCanvas } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎬️cli/🟦️.ts";
import type { Asset, Environment } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import type { DrawingScene, EnvironmentTimeline, EvaluationQuery, TransportLayer } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";

const [outDir, ...wanted] = process.argv.slice(2);
const catalog = await loadCatalogFromFiles();
const asset = (id: string): Asset => {
  const a = catalog.assets.find((c) => c.id === id)!;
  return a.thermal.passengers === undefined ? { ...a, thermal: { ...a.thermal, passengers: [] } } : a;
};
const env = (airTempC: number, relativeHumidity: number, windMS: number, windDirectionDeg: number): Environment => ({ airTempC, meanRadiantTempC: airTempC, relativeHumidity, windMS, windDirectionDeg, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 });

type Case = { id: string; asset: Asset; query: EvaluationQuery; environment: Environment | EnvironmentTimeline; scale: number; layers: TransportLayer[]; fps: number; fromS: number; toS: number; stills: number[]; px: number; streamStills?: number[] };

const commuter = asset("walking-commuter");
const base = asset("walking-commuter");
const turned: Asset = {
  ...base,
  clips: [
    ...base.clips,
    { ...base.clips.find((c) => c.id === "idle-breathing")!, id: "head-turn", durationS: 12, loop: false, keys: [{ t: 0, pose: "standing", ease: "smooth" }, { t: 3, pose: "standing", ease: "smooth" }, { t: 3.8, pose: "standing", ease: "smooth", bones: { head: { angleDeg: 50 }, neck: { angleDeg: 15 } } }, { t: 12, pose: "standing", ease: "smooth", bones: { head: { angleDeg: 50 }, neck: { angleDeg: 15 } } }], tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }] }] },
  ],
};
const dig = asset("excavator");
const ramp: Asset = {
  ...dig,
  clips: [
    ...dig.clips,
    { ...dig.clips.find((c) => c.id === "start-and-idle")!, id: "ignition-idle-rated", durationS: 14, loop: false, tracks: [{ target: "operating:engine-load", ease: "linear", keys: [{ t: 0, v: 0 }, { t: 0.8, v: 0.5 }, { t: 1.5, v: 0.2 }, { t: 6, v: 0.2 }, { t: 8, v: 1 }, { t: 14, v: 1 }] }, { target: "operating:hydraulic", ease: "linear", keys: [{ t: 0, v: 0 }, { t: 2, v: 0.1 }, { t: 6, v: 0.1 }, { t: 8, v: 0.9 }, { t: 14, v: 0.9 }] }, { target: "emitter:exhaust-stack", ease: "step", keys: [{ t: 0, v: 0 }, { t: 0.8, v: 1 }] }] },
  ],
};
const oak = asset("english-oak");

const cases: Case[] = [
  { id: "commuter", asset: commuter, query: { kind: "clip", clip: "walk-cycle" }, environment: { keys: [{ t: 0, environment: env(-2, 0.92, 0.3, 0), ease: "step" }, { t: 6, environment: env(-2, 0.92, 0.3, 0), ease: "linear" }, { t: 8, environment: env(-2, 0.92, 3, 180), ease: "step" }] }, scale: 50, layers: ["parcels", "pathlines", "condensation"], fps: 12, fromS: 2, toS: 11, stills: [3, 5.5, 7, 8.5, 10, 11], px: 6, streamStills: [10] },
  { id: "headturn", asset: turned, query: { kind: "clip", clip: "head-turn" }, environment: env(2, 0.9, 0, 0), scale: 20, layers: ["parcels", "pathlines", "condensation"], fps: 12, fromS: 2.2, toS: 7, stills: [2.6, 3.4, 3.9, 4.3, 5.2, 6.4], px: 12, streamStills: [3.4] },
  { id: "excavator", asset: ramp, query: { kind: "clip", clip: "ignition-idle-rated" }, environment: env(10, 0.6, 1, 0), scale: 50, layers: ["parcels", "pathlines", "condensation"], fps: 12, fromS: 0, toS: 13, stills: [0.7, 1.3, 4, 8.2, 10, 13], px: 5 },
  { id: "oak", asset: oak, query: { kind: "clip", clip: "wind-sway" }, environment: env(24, 0.45, 1.2, 0), scale: 100, layers: ["parcels", "pathlines", "streamlines"], fps: 6, fromS: 10, toS: 28, stills: [10, 16, 22, 28], px: 5 },
];

function render(c: Case, times: number[], layers: TransportLayer[], cold = false): DrawingScene[] {
  const a = cold ? structuredClone(c.asset) : c.asset;
  const sel = { kind: "scale", scale: { denominator: c.scale } } as const;
  const options = { ...defaultDepictionOptions("light-line", "en", c.scale), profile: { ...printProfile("large-format-plot"), minStrokeMm: 0, minGapMm: 0, minFeatureMm: 0, minHatchSpacingMm: 0 }, representation: sel, transport: layers, range: { mode: "shared" as const }, ...("keys" in c.environment ? { environmentTimeline: c.environment } : { environment: c.environment }) };
  return times.map((timeS) => {
    const query = { ...c.query, timeS } as EvaluationQuery;
    const state = evaluate(a, query, sel);
    const field = solveTransport(a, query, c.environment);
    return depict(state, options, { transport: field });
  });
}

function rasterise(scenes: DrawingScene[], directory: string, pxPerMm: number, names: string[]): void {
  mkdirSync(directory, { recursive: true });
  const canvas = fixedCanvas(scenes, pxPerMm);
  scenes.forEach((scene, i) => {
    const svg = join(directory, `${names[i]}.svg`);
    writeFileSync(svg, writeSvg(scene));
    const o = canvas.offsets[i]!;
    const r = spawnSync("rsvg-convert", ["-d", String(25.4 * pxPerMm), "-p", String(25.4 * pxPerMm), "--page-width", String(canvas.widthPx), "--page-height", String(canvas.heightPx), "--left", String(o.left), "--top", String(o.top), "-b", "#ffffff", "-o", join(directory, `${names[i]}.png`), svg], { encoding: "utf8" });
    if (r.status !== 0) throw new Error(r.stderr);
    rmSync(svg);
  });
}

const digest = (s: DrawingScene): string => createHash("sha256").update(JSON.stringify(s)).digest("hex").slice(0, 16);
const report: Record<string, unknown> = {};

for (const c of cases) {
  if (wanted.length > 0 && !wanted.includes(c.id)) continue;
  const dir = join(outDir!, c.id);
  const t0 = performance.now();
  const stills = render(c, c.stills, c.layers);
  rasterise(stills, dir, c.px, c.stills.map((t) => `still-t${t.toFixed(1).padStart(5, "0")}`));
  if (c.streamStills) {
    const s = render(c, c.streamStills, ["streamlines", "parcels"]);
    rasterise(s, dir, c.px, c.streamStills.map((t) => `streamlines-t${t.toFixed(1).padStart(5, "0")}`));
  }
  const times: number[] = [];
  for (let t = c.fromS; t <= c.toS + 1e-9; t += 1 / c.fps) times.push(Math.round(t * 1000) / 1000);
  const play = render(c, times, c.layers);
  const sample = times.filter((_, i) => i % 7 === 3);
  const reverse = render(c, [...sample].reverse(), c.layers, true).reverse();
  const playSample = sample.map((t) => play[times.indexOf(t)]!);
  const same = sample.every((t, i) => digest(playSample[i]!) === digest(reverse[i]!));
  report[c.id] = { frames: times.length, exportEqualsScrub: same, sampled: sample.length, seconds: Math.round((performance.now() - t0) / 100) / 10 };
  console.log("[DEBUG]", c.id, JSON.stringify(report[c.id]));
  const frameDir = join(dir, "frames");
  rasterise(play, frameDir, c.px, times.map((_, i) => `f${String(i).padStart(4, "0")}`));
  const ff = spawnSync("repo", ["tools", "exec", "ffmpeg", "--", "-y", "-framerate", String(c.fps), "-i", join(frameDir, "f%04d.png"), "-c:v", "libx264", "-pix_fmt", "yuv420p", "-vf", "pad=ceil(iw/2)*2:ceil(ih/2)*2", join(dir, `${c.id}.mp4`)], { encoding: "utf8" });
  if (ff.status !== 0) console.log("[DEBUG] ffmpeg failed", ff.stderr.slice(-400));
  else rmSync(frameDir, { recursive: true, force: true });
}
writeFileSync(join(outDir!, "export-vs-scrub.json"), JSON.stringify(report, null, 1));
console.log("[DEBUG] done", readdirSync(outDir!).join(","));
