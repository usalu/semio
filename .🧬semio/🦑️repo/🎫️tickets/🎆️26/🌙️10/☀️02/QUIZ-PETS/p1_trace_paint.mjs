/** 🎨️ Ticket tool of work package P1: what the renderer's main thread did between the sweep's marks of a kept trace — the total and self time of every event name, the `Paint` events grouped by the node they painted and the size of their clip, the style recalculations by element count, and the instant events of layer updates — to tell which paints and recalculations a configuration of pets adds.
 *
 * Usage: node ".../p1_trace_paint.mjs" <trace.json> [top]
 */
import { readFileSync } from "node:fs";

const trace = JSON.parse(readFileSync(process.argv[2], "utf8"));
const top = Number(process.argv[3] ?? "25");
const events = trace.traceEvents ?? trace;
const start = events.find((event) => event.name === "p1-sweep-start");
const end = events.find((event) => event.name === "p1-sweep-end");
const from = start.ts;
const to = end.ts;
const main = events.filter((event) => event.pid === start.pid && event.tid === start.tid && event.ts >= from && event.ts <= to);
const complete = main.filter((event) => event.ph === "X").sort((a, b) => a.ts - b.ts || b.dur - a.dur);
const stack = [];
const totals = {};
for (const event of complete) {
  while (stack.length > 0 && stack[stack.length - 1].ts + stack[stack.length - 1].dur <= event.ts) stack.pop();
  const parent = stack[stack.length - 1];
  event.own = event.dur;
  if (parent !== undefined) parent.own -= event.dur;
  stack.push(event);
}
for (const event of complete) {
  const bucket = (totals[event.name] ??= { count: 0, total: 0, self: 0 });
  bucket.count += 1;
  bucket.total += event.dur / 1000;
  bucket.self += Math.max(0, event.own) / 1000;
}
process.stdout.write(`window ${Math.round((to - from) / 1000)} ms; main-thread events by self time:\n`);
for (const [name, bucket] of Object.entries(totals)
  .sort((a, b) => b[1].self - a[1].self)
  .slice(0, top))
  process.stdout.write(`  ${name}: n ${bucket.count}, total ${bucket.total.toFixed(1)} ms, self ${bucket.self.toFixed(1)} ms\n`);
const paints = {};
for (const event of complete.filter((candidate) => candidate.name === "Paint")) {
  const data = event.args?.data ?? {};
  const clip = data.clip ?? [];
  const width = clip.length >= 8 ? Math.round(Math.max(clip[0], clip[2], clip[4], clip[6]) - Math.min(clip[0], clip[2], clip[4], clip[6])) : -1;
  const height = clip.length >= 8 ? Math.round(Math.max(clip[1], clip[3], clip[5], clip[7]) - Math.min(clip[1], clip[3], clip[5], clip[7])) : -1;
  const key = `node ${data.nodeId ?? "?"} layer ${data.layerId ?? "?"} ${width}x${height}`;
  const bucket = (paints[key] ??= { count: 0, ms: 0 });
  bucket.count += 1;
  bucket.ms += event.dur / 1000;
}
process.stdout.write("Paint by node, layer and clip:\n");
for (const [key, bucket] of Object.entries(paints)
  .sort((a, b) => b[1].ms - a[1].ms)
  .slice(0, top))
  process.stdout.write(`  ${key}: n ${bucket.count}, ${bucket.ms.toFixed(1)} ms\n`);
const styles = complete.filter((event) => event.name === "UpdateLayoutTree").map((event) => ({ ms: event.dur / 1000, elements: event.args?.elementCount ?? event.args?.endData?.elementCount ?? 0 }));
const bands = { "≤50": [0, 0, 0], "51–300": [0, 0, 0], "301–1000": [0, 0, 0], ">1000": [0, 0, 0] };
for (const style of styles) {
  const band = style.elements <= 50 ? "≤50" : style.elements <= 300 ? "51–300" : style.elements <= 1000 ? "301–1000" : ">1000";
  bands[band][0] += 1;
  bands[band][1] += style.ms;
  bands[band][2] += style.elements;
}
process.stdout.write(`UpdateLayoutTree by elements: ${Object.entries(bands)
  .map(([band, [count, ms, elements]]) => `${band}: n ${count}, ${ms.toFixed(1)} ms, ${elements} elements`)
  .join("; ")}\n`);
