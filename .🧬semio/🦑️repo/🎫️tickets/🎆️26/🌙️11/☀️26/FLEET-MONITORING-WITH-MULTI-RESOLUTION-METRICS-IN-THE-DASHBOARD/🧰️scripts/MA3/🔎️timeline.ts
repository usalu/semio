/** 🔎️ Read-only timeline of one origin and metric: runs of consecutive points per tier over the last hour. */
import {readSeriesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";
const origin = process.argv[2] ?? "g1-m01";
const id = process.argv[3] ?? "host.cpu.busy";
const dir = `/Users/Shared/semio-store/v1/origins/${origin}/metrics`;
const now = Date.now();
const series = readSeriesV1({directory: dir, fromMs: now - 3_600_000, toMs: now, ids: [id]});
const points = [...series.metrics[id]!].sort((a, b) => a[0] - b[0]);
const runs: string[] = [];
let run: {tier: number; from: number; to: number; n: number} | null = null;
for (const [start, width, tier] of points) {
  if (run && run.tier === tier && start <= run.to + 1) { run.to = start + width; run.n++; continue; }
  if (run) runs.push(`t${run.tier} ${new Date(run.from).toISOString().slice(11, 19)}..${new Date(run.to).toISOString().slice(11, 19)} n=${run.n}`);
  run = {tier, from: start, to: start + width, n: 1};
}
if (run) runs.push(`t${run.tier} ${new Date(run.from).toISOString().slice(11, 19)}..${new Date(run.to).toISOString().slice(11, 19)} n=${run.n}`);
process.stdout.write(`now ${new Date(now).toISOString().slice(11, 19)} points=${points.length}\n${runs.join("\n")}\n`);
