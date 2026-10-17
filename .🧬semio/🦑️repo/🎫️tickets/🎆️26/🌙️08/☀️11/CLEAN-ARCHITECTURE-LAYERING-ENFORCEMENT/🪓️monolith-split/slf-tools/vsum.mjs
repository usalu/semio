/** 📊️ Summarises vitest JSON reports into load status and failing-test sets: `node vsum.mjs <phase> <label...>` writes `slf-sum-<phase>.json`. */
import { readFileSync, writeFileSync, existsSync } from "node:fs";
const G = process.env.G;
const [phase, ...labels] = process.argv.slice(2);
const out = {};
for (const label of labels) {
  const p = `${G}/slf-vt-${phase}-${label}.json`;
  if (!existsSync(p)) { out[label] = { report: false }; continue; }
  const j = JSON.parse(readFileSync(p, "utf8"));
  const files = {};
  const failedTests = [];
  let loadFail = 0;
  for (const t of j.testResults) {
    const name = t.name.replace(/^.*\/Documents\/semio\//u, "");
    const loaded = !(t.status === "failed" && t.assertionResults.length === 0);
    if (!loaded) loadFail++;
    files[name] = { loaded, tests: t.assertionResults.length, failed: t.assertionResults.filter((a) => a.status === "failed").length, loadError: loaded ? undefined : (t.message || "").split("\n")[0].slice(0, 240) };
    for (const a of t.assertionResults) if (a.status === "failed") failedTests.push(`${name} :: ${a.fullName}`);
  }
  out[label] = { report: true, files: j.testResults.length, loaded: j.testResults.length - loadFail, loadFail, tests: j.numTotalTests, passed: j.numPassedTests, failed: j.numFailedTests, failedTests: failedTests.sort(), perFile: files };
}
writeFileSync(`${G}/slf-sum-${phase}.json`, JSON.stringify(out, null, 1));
for (const [l, v] of Object.entries(out)) console.log(l.padEnd(24), v.report ? `files=${v.files} loaded=${v.loaded} loadFail=${v.loadFail} tests=${v.tests} passed=${v.passed} failed=${v.failed}` : "NO REPORT");
