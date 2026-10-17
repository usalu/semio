/** 📊️ Summarises one hub-tests vitest JSON report: `node sum.mjs <phase>` prints load status per file and writes `hsc-sum-<phase>.json`. */
import { readFileSync, writeFileSync } from "node:fs";
const G = process.env.G;
const phase = process.argv[2];
const j = JSON.parse(readFileSync(`${G}/hsc-vt-${phase}.json`, "utf8"));
const files = {};
const failedTests = [];
for (const t of j.testResults) {
  const name = t.name.replace(/^.*\/Documents\/semio\//u, "");
  const loaded = !(t.status === "failed" && t.assertionResults.length === 0);
  files[name] = { loaded, tests: t.assertionResults.length, failed: t.assertionResults.filter((a) => a.status === "failed").length, loadError: loaded ? undefined : (t.message || "").split("\n")[0].slice(0, 300) };
  for (const a of t.assertionResults) if (a.status === "failed") failedTests.push(`${name} :: ${a.fullName}`);
}
const out = { files: j.testResults.length, loaded: Object.values(files).filter((f) => f.loaded).length, tests: j.numTotalTests, passed: j.numPassedTests, failed: j.numFailedTests, skipped: j.numPendingTests, failedTests: failedTests.sort(), perFile: files };
writeFileSync(`${G}/hsc-sum-${phase}.json`, JSON.stringify(out, null, 1));
console.log(`files=${out.files} loaded=${out.loaded} tests=${out.tests} passed=${out.passed} failed=${out.failed} skipped=${out.skipped}`);
for (const [f, r] of Object.entries(files)) console.log(r.loaded ? (r.failed ? `FAILS(${r.failed})` : "ok") : "NOLOAD", f.slice(-70), r.loadError ?? "");
