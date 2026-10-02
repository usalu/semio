/** 📊️ Summarises a vitest `--reporter=json` report: per file the number of tests, the summed test time and the wall time, then the slowest tests. `node wp_q_durations.mjs <report.json> [top]`. */
import { readFileSync } from "node:fs";

const [, , file, top = "25"] = process.argv;
const report = JSON.parse(readFileSync(file, "utf8"));
const rows = [];
let total = 0;
for (const suite of report.testResults) {
  const sum = suite.assertionResults.reduce((s, a) => s + (a.duration ?? 0), 0);
  total += sum;
  const name = suite.name.split("\\").join("/").split("/🧪️tests/")[0].split("/").slice(-2).join("/");
  process.stdout.write(`${name.padEnd(34)} tests=${String(suite.assertionResults.length).padStart(4)} sum=${(sum / 1000).toFixed(2)}s wall=${((suite.endTime - suite.startTime) / 1000).toFixed(2)}s status=${suite.status}\n`);
  for (const a of suite.assertionResults) rows.push({ name, title: a.fullName, ms: a.duration ?? 0, status: a.status });
}
process.stdout.write(`TOTAL tests=${report.numTotalTests} passed=${report.numPassedTests} failed=${report.numFailedTests} skipped=${report.numPendingTests} sum=${(total / 1000).toFixed(2)}s\n`);
rows.sort((a, b) => b.ms - a.ms);
for (const r of rows.slice(0, Number(top))) process.stdout.write(`${r.ms.toFixed(0).padStart(6)}ms ${r.name} :: ${r.title.slice(0, 160)}\n`);
