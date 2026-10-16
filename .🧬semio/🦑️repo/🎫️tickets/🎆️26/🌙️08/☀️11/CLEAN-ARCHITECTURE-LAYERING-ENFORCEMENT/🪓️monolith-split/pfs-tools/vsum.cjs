// usage: node vsum.cjs <vitest.json> [--titles]  : counts + per-file counts; with --titles prints "status\tfullName" sorted lines (file-independent)
const fs = require("fs");
const j = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const rows = [];
for (const f of j.testResults) for (const t of f.assertionResults) rows.push({ file: f.name.replace(/^.*\/(?=🧰️framework|🎓️teaching|✏️s)/, ""), status: t.status, name: t.fullName });
if (process.argv[3] === "--titles") { console.log(rows.map((r) => `${r.status}\t${r.name}`).sort().join("\n")); process.exit(0); }
if (process.argv[3] === "--files") { const m = {}; for (const r of rows) { m[r.file] ??= { passed: 0, failed: 0, other: 0 }; m[r.file][r.status === "passed" ? "passed" : r.status === "failed" ? "failed" : "other"]++; } for (const [k, v] of Object.entries(m)) console.log(`${v.passed}\t${v.failed}\t${v.other}\t${k.slice(-110)}`); process.exit(0); }
const c = {}; for (const r of rows) c[r.status] = (c[r.status] || 0) + 1;
console.log(`files=${j.testResults.length} tests=${rows.length}`, JSON.stringify(c), "suiteFailures=" + j.testResults.filter((f) => f.status === "failed" && f.assertionResults.length === 0).length);
for (const f of j.testResults) if (f.status === "failed" && !f.assertionResults.length) console.log("SUITE FAIL:", f.name.slice(-100), (f.message || "").slice(0, 200));
