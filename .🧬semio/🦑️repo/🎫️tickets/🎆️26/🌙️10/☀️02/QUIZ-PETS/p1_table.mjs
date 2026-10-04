/** 📋️ Ticket tool of work package P1: turns the lines of `p1_measure.mjs` into the tables of the report — per measured label and pets configuration the frame rate of the clean runs, the renderer's counters per frame, the traced main thread per frame by kind, the forced style and layout inside scripts by owner, the time inside the pets' modules, and the pets' DOM calls per second of the counted run.
 *
 * Usage (from the repository root): node ".../p1_table.mjs" <measure-*.jsonl>...
 */
import { readFileSync } from "node:fs";
import { basename } from "node:path";

const median = (values) => {
  if (values.length === 0) return NaN;
  const sorted = [...values].sort((a, b) => a - b);
  const middle = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[middle] : (sorted[middle - 1] + sorted[middle]) / 2;
};
const fixed = (value, digits = 1) => (Number.isFinite(value) ? value.toFixed(digits) : "–");
const modes = ["off", "still", "calm", "lively"];
const clean = ["| label | pets | fps (runs) | median fps | frame gap p50 / p95 ms | long tasks per 10 s (ms) | task / script / layout / style ms per frame |", "|---|---|---|---|---|---|---|"];
const traced = ["| label | pets | traced fps | main busy % | scripting / style / layout / paint / other ms per frame | forced style + layout by pets: n, ms | forced by the page: n, ms | inside pets ms per frame (share of busy) | raster / GPU-task ms per frame | drawn frames |", "|---|---|---|---|---|---|---|---|---|---|"];
const counted = ["| label | pets | counted fps | pets per second: layout reads · computed styles · tree walks · element matches · rAF · setAttribute · style writes · observer calls (records) |", "|---|---|---|---|"];
for (const file of process.argv.slice(2)) {
  const runs = readFileSync(file, "utf8")
    .split("\n")
    .filter((line) => line.trim() !== "")
    .map((line) => JSON.parse(line));
  const label = runs[0]?.label ?? basename(file);
  for (const mode of modes) {
    const mine = runs.filter((run) => run.mode === mode && run.instrumented === false);
    if (mine.length > 0) {
      const per = (pick) => median(mine.map(pick));
      clean.push(`| ${label} | ${mode} | ${mine.map((run) => fixed(run.fps)).join(" / ")} | **${fixed(per((run) => run.fps))}** | ${fixed(per((run) => run.gapMs.p50))} / ${fixed(per((run) => run.gapMs.p95))} | ${fixed(per((run) => (run.longTasks.count * 10) / run.wallSeconds))} (${fixed(per((run) => (run.longTasks.ms * 10) / run.wallSeconds), 0)}) | ${fixed(per((run) => run.perFrameMs.task), 2)} / ${fixed(per((run) => run.perFrameMs.script), 2)} / ${fixed(per((run) => run.perFrameMs.layout), 2)} / ${fixed(per((run) => run.perFrameMs.style), 2)} |`);
    }
    const trace = runs.find((run) => run.mode === mode && run.instrumented === "traced");
    if (trace?.trace && !trace.trace.error) {
      const t = trace.trace;
      const frames = Math.max(trace.frames, 1);
      const per = (ms) => fixed(ms / frames, 2);
      const sum = (group) => Object.entries(group ?? {}).reduce((total, [, bucket]) => ({ count: total.count + bucket.count, ms: total.ms + bucket.ms }), { count: 0, ms: 0 });
      const petsForced = sum({ a: t.forced.layout.pets ?? { count: 0, ms: 0 }, b: t.forced.style.pets ?? { count: 0, ms: 0 } });
      const pageForced = sum(Object.fromEntries([...Object.entries(t.forced.layout), ...Object.entries(t.forced.style)].filter(([who]) => who !== "pets").map(([who, bucket], index) => [`${who}${index}`, bucket])));
      const inside = trace.profile?.insidePetsMs ?? 0;
      traced.push(`| ${label} | ${mode} | ${fixed(trace.fps)} | ${fixed((100 * t.busyMs) / t.windowMs, 0)} | ${per(t.selfMs.scripting)} / ${per(t.selfMs.style)} / ${per(t.selfMs.layout)} / ${per(t.selfMs.paint)} / ${per(t.selfMs.other + t.selfMs.gc + t.selfMs.hitTest)} | ${petsForced.count}, ${fixed(petsForced.ms, 1)} | ${pageForced.count}, ${fixed(pageForced.ms, 1)} | ${per(inside)} (${fixed((100 * inside) / Math.max(t.busyMs, 1), 1)} %) | ${per(t.rasterMs)} / ${per(t.gpuTaskMs)} | ${t.drawFrames} |`);
    }
    const count = runs.find((run) => run.mode === mode && run.instrumented === "counted");
    if (count) {
      const pets = (what) => fixed((count.calls?.[what]?.pets ?? 0) / count.wallSeconds, 0);
      const styles = ["style.transform", "style.opacity", "style.width", "style.height", "style.setProperty"].reduce((total, what) => total + (count.calls?.[what]?.pets ?? 0), 0);
      counted.push(`| ${label} | ${mode} | ${fixed(count.fps)} | ${pets("getBoundingClientRect")} · ${pets("getComputedStyle")} · ${pets("createTreeWalker")} · ${pets("matches")} · ${pets("requestAnimationFrame")} · ${pets("setAttribute")} · ${fixed(styles / count.wallSeconds, 0)} · ${pets("MutationObserver calls")} (${pets("MutationObserver records")}) |`);
    }
  }
}
process.stdout.write(`${clean.join("\n")}\n\n${traced.join("\n")}\n\n${counted.join("\n")}\n`);
