import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

// 📊️ Aggregates `🐍️reconcile-profile-probe.mjs`'s sampling profile over the console-silent windows,
// so the ~10.4 s that follow a reconcile-sourced `more-work` answer are attributed to named host
// frames instead of guessed at.
const dir = join(import.meta.dir, "🗑️generated", "reconcile-silence", process.argv[2] ?? "profile-1");
const profile = JSON.parse(readFileSync(join(dir, "profile.cpuprofile"), "utf8"));
const meta = JSON.parse(readFileSync(join(dir, "meta.json"), "utf8"));
const console_ = readFileSync(join(dir, "console.txt"), "utf8").split("\n");

const nodes = new Map(profile.nodes.map((n) => [n.id, n]));
const parents = new Map();
for (const n of profile.nodes) for (const child of n.children ?? []) parents.set(child, n.id);
const label = (id) => {
  const n = nodes.get(id);
  if (!n) return `#${id}`;
  const f = n.callFrame;
  const file = (f.url ?? "").split("/").slice(-2).join("/");
  return `${f.functionName || "(anonymous)"} @ ${file}:${f.lineNumber + 1}`;
};

const times = [];
let acc = 0;
for (let i = 0; i < profile.samples.length; i += 1) { acc += profile.timeDeltas[i] ?? 0; times.push(meta.profileStart + acc / 1000); }

const stamps = console_.map((l) => ({ t: Number.parseFloat(l), text: l })).filter((r) => Number.isFinite(r.t));
const windows = [];
for (let i = 1; i < stamps.length; i += 1) {
  const gap = stamps[i].t - stamps[i - 1].t;
  if (gap > Number(process.env.SEMIO_GAP_MS ?? 2000)) windows.push({ from: stamps[i - 1].t, to: stamps[i].t, gap, prev: stamps[i - 1].text.slice(0, 180), next: stamps[i].text.slice(0, 180) });
}

const report = [];
const tally = (from, to, title) => {
  const self = new Map();
  const total = new Map();
  let count = 0;
  for (let i = 0; i < times.length; i += 1) {
    if (times[i] < from || times[i] > to) continue;
    count += 1;
    const id = profile.samples[i];
    self.set(id, (self.get(id) ?? 0) + 1);
    const seen = new Set();
    for (let cur = id; cur !== undefined; cur = parents.get(cur)) { if (seen.has(cur)) break; seen.add(cur); total.set(cur, (total.get(cur) ?? 0) + 1); }
  }
  const top = (m) => [...m.entries()].sort((a, b) => b[1] - a[1]).slice(0, 18).map(([id, c]) => `    ${String(c).padStart(6)} (${((c / Math.max(count, 1)) * 100).toFixed(1)}%) ${label(id)}`).join("\n");
  report.push(`\n## ${title}  [${from.toFixed(0)} .. ${to.toFixed(0)} ms]  samples=${count}\n  SELF:\n${top(self)}\n  TOTAL:\n${top(total)}`);
};

report.push(`profile samples=${profile.samples.length} span=${times[0]?.toFixed(0)}..${times.at(-1)?.toFixed(0)} ms`);
report.push(`\n# Silent windows (console gap > ${process.env.SEMIO_GAP_MS ?? 2000} ms)`);
for (const w of windows) report.push(`  ${w.gap.toFixed(0)} ms  ${w.from.toFixed(0)} -> ${w.to.toFixed(0)}\n    PREV ${w.prev}\n    NEXT ${w.next}`);
for (const w of windows.filter((x) => x.gap > 5000)) tally(w.from, w.to, `silent window ${w.gap.toFixed(0)} ms`);
tally(times[0] ?? 0, times.at(-1) ?? 0, "whole run");
writeFileSync(join(dir, "profile-report.txt"), report.join("\n"));
console.log(report.join("\n"));
