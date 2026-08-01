import {
  formatMicroCommitMetricLine,
  formatMicroCommitMetricsLines,
  formatBundleUlocSuffix,
  formatBundleDateLine,
  formatMetricRatio,
} from "/Users/ueli/Documents/semio/\ud83e\uddf0\ufe0fframework/\ud83d\udecd\ufe0fproduct/\ud83e\udd91\ufe0frepo/\ud83d\udd28\ufe0fmodule/\ud83d\udcda\ufe0flib/\u26a1\ufe0fimplementation/\ud83d\udfe6\ufe0ftypescript/\ud83d\udce6\ufe0findex.ts";

console.log(JSON.stringify({
  r001: formatMetricRatio(0.001),
  rtiny: formatMetricRatio(0.0000854),
  r0305: formatMetricRatio(0.0305),
  r00264: formatMetricRatio(0.00264),
  r10pct: formatMetricRatio(0.10061),
  r121pct: formatMetricRatio(1.21001),
  shellPlus: formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 2, added: 2, removed: 0 }),
  shellMinus: formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 2, added: 0, removed: 2 }),
  rust: formatMicroCommitMetricsLines([{ lang: "Rust", emoji: "🦀️", code: 200_000, edited: 2220, added: 2000, removed: 500 }]),
  tot: formatMicroCommitMetricsLines([
    { lang: "TypeScript", emoji: "🟦️", code: 3000, edited: 10, added: 8, removed: 0 },
    { lang: "Markdown", emoji: "📝️", code: 44, edited: 0, added: 0, removed: 0 },
  ]),
  bundle: formatBundleUlocSuffix({ added: 700, edited: 200, removed: 10 }, 65_000),
  date: formatBundleDateLine("🎆️26🌙️06☀️04", { added: 700, edited: 200, removed: 10 }, 65_000),
}, null, 2));
