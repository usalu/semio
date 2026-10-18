/** 📊️ Summarises one vitest JSON report: `node sum.mjs <suite> <phase>` prints per-file status and writes tsh-sum-<suite>-<phase>.json. */
import { readFileSync, writeFileSync, existsSync } from "node:fs";
const G = process.env.G;
const [suite, phase] = process.argv.slice(2);
const p = `${G}/tsh-vt-${suite}-${phase}.json`;
if (!existsSync(p)) { console.log("NO JSON", p); process.exit(1); }
const j = JSON.parse(readFileSync(p, "utf8"));
const files = {}; const failedTests = [];
for (const t of j.testResults) {
  const name = t.name.replace(/^.*\/Documents\/semio\//u, "");
  const loaded = !(t.status === "failed" && t.assertionResults.length === 0);
  files[name] = { loaded, tests: t.assertionResults.length, failed: t.assertionResults.filter((a) => a.status === "failed").length, loadError: loaded ? undefined : (t.message || "").split("\n")[0].slice(0, 300) };
  for (const a of t.assertionResults) if (a.status === "failed") failedTests.push(`${name} :: ${a.fullName}`);
}
const out = { files: j.testResults.length, loaded: Object.values(files).filter((f) => f.loaded).length, tests: j.numTotalTests, passed: j.numPassedTests, failed: j.numFailedTests, skipped: j.numPendingTests, failedTests: failedTests.sort(), perFile: files };
writeFileSync(`${G}/tsh-sum-${suite}-${phase}.json`, JSON.stringify(out, null, 1));
console.log(`files=${out.files} loaded=${out.loaded} tests=${out.tests} passed=${out.passed} failed=${out.failed} skipped=${out.skipped}`);
for (const [f, r] of Object.entries(files)) console.log(r.loaded ? (r.failed ? `FAILS(${r.failed})` : "ok") : "NOLOAD", f.slice(-80), r.loadError ?? "");
