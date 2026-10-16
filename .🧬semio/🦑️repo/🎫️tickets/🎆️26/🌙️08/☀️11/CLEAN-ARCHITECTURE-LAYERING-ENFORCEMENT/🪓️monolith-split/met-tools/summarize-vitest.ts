import { readFileSync, writeFileSync } from "node:fs";
const [input, output] = process.argv.slice(2) as [string, string];
const j = JSON.parse(readFileSync(input, "utf8"));
const rows: string[] = [];
const files: Record<string, number> = {};
for (const f of j.testResults) {
  const rel = String(f.name).replace(/^.*\/(?=[^/]*$)/, "");
  files[f.name] = f.assertionResults.length;
  for (const a of f.assertionResults) rows.push(`${a.status}\t${a.fullName}`);
}
rows.sort();
writeFileSync(output, rows.join("\n") + "\n");
console.log(JSON.stringify({ total: j.numTotalTests, passed: j.numPassedTests, failed: j.numFailedTests, pending: j.numPendingTests, suites: j.numTotalTestSuites, failedSuites: j.numFailedTestSuites, files }, null, 1));
