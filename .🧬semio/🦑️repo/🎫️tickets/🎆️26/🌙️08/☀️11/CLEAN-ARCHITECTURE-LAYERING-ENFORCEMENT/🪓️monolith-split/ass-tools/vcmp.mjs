/** ⚖️ Compares base and after vitest summaries per suite: load matrix, regressions, failure-set containment, newly loaded files. */
import { readFileSync } from "node:fs";
const G = process.env.G;
const b = JSON.parse(readFileSync(`${G}/ass-sum-base.json`, "utf8"));
const a = JSON.parse(readFileSync(`${G}/ass-sum-after.json`, "utf8"));
const rows = [];
let regress = 0, newFail = 0;
for (const label of Object.keys(b)) {
  const x = b[label], y = a[label];
  const lostLoad = Object.keys(x.perFile).filter((f) => x.perFile[f].loaded && !(y.perFile[f]?.loaded));
  const newlyLoaded = Object.keys(y.perFile).filter((f) => y.perFile[f].loaded && !(x.perFile[f]?.loaded));
  const failedBefore = new Set(x.failedTests);
  const newFailures = y.failedTests.filter((t) => !failedBefore.has(t) && x.perFile[t.split(" :: ")[0]]?.loaded);
  const newlyLoadedFailures = y.failedTests.filter((t) => newlyLoaded.includes(t.split(" :: ")[0]));
  const fixedFailures = x.failedTests.filter((t) => !y.failedTests.includes(t));
  regress += lostLoad.length + newFailures.length;
  rows.push({ label, files: y.files, loadedBase: x.loaded, loadedAfter: y.loaded, testsBase: x.tests, testsAfter: y.tests, passBase: x.passed, passAfter: y.passed, failBase: x.failed, failAfter: y.failed, lostLoad, newFailures, newlyLoaded: newlyLoaded.length, newlyLoadedFailures: newlyLoadedFailures.length, fixedFailures: fixedFailures.length });
}
const pad = (v, n) => String(v).padStart(n);
console.log("suite".padEnd(24), "files", "loaded b>a", "tests b>a", "pass b>a", "fail b>a", "lostLoad", "newFail(old files)", "newlyLoaded", "failIn(new)");
for (const r of rows) console.log(r.label.padEnd(24), pad(r.files, 5), `${pad(r.loadedBase, 4)}>${pad(r.loadedAfter, 3)}`.padStart(10), `${pad(r.testsBase, 5)}>${pad(r.testsAfter, 5)}`.padStart(11), `${pad(r.passBase, 5)}>${pad(r.passAfter, 5)}`.padStart(11), `${pad(r.failBase, 3)}>${pad(r.failAfter, 3)}`.padStart(8), pad(r.lostLoad.length, 8), pad(r.newFailures.length, 18), pad(r.newlyLoaded, 11), pad(r.newlyLoadedFailures, 11));
const sum = (k) => rows.reduce((s, r) => s + r[k], 0);
console.log("TOTAL".padEnd(24), pad(sum("files"), 5), `${pad(sum("loadedBase"), 4)}>${pad(sum("loadedAfter"), 3)}`.padStart(10), `${pad(sum("testsBase"), 5)}>${pad(sum("testsAfter"), 5)}`.padStart(11), `${pad(sum("passBase"), 5)}>${pad(sum("passAfter"), 5)}`.padStart(11), `${pad(sum("failBase"), 3)}>${pad(sum("failAfter"), 3)}`.padStart(8));
console.log("regressions (files that stopped loading + failing tests in previously loaded files that did not fail before):", regress);
for (const r of rows) { for (const f of r.lostLoad) console.log("LOST", r.label, f); for (const t of r.newFailures) console.log("NEWFAIL", r.label, t); }
console.log("suites fully loaded after:", rows.filter((r) => r.loadedAfter === r.files).map((r) => r.label).join(", "));
console.log("suites with load failures after:", rows.filter((r) => r.loadedAfter !== r.files).map((r) => `${r.label}(${r.files - r.loadedAfter})`).join(", "));
