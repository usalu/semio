import {
  formatMetricLocCount,
  formatMetricRatio,
  formatUlocMetricsBody,
  formatMicroCommitMetricsLines,
} from "../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";

const checks = [
  ["422k", formatMetricLocCount(422377)],
  ["52.8k", formatMetricLocCount(52759)],
  ["237k", formatMetricLocCount(237216)],
  ["1.8M", formatMetricLocCount(1800000)],
  ["0.0001", formatMetricRatio(0.0000854)],
  ["0.03", formatMetricRatio(0.0305)],
  ["0.003", formatMetricRatio(0.00264)],
  ["0.008", formatMetricRatio(1500 / 198500)],
  ["0.01", formatMetricRatio(690 / 64310)],
];
let failed = 0;
for (const [want, got] of checks) {
  const ok = got === want;
  if (!ok) failed++;
  console.log(ok ? "ok" : "FAIL", want, got);
}
console.log(
  formatUlocMetricsBody({
    code: 1800000,
    added: 237216,
    edited: 704,
    removed: 184457,
  }),
);
console.log(
  formatMicroCommitMetricsLines([
    { lang: "Rust", emoji: "🦀", code: 200000, edited: 2220, added: 2000, removed: 500 },
  ]).join("\n"),
);
process.exit(failed);