import fs from "node:fs";
const [json, out] = process.argv.slice(2);
const r = JSON.parse(fs.readFileSync(json, "utf8"));
const rows = [], files = [];
for (const f of r.testResults) {
  const short = f.name.replace("/Users/ueli/Documents/semio/", "");
  const counts = {};
  for (const t of f.assertionResults) { counts[t.status] = (counts[t.status] ?? 0) + 1; rows.push(`${t.status}\t${t.fullName}`); }
  files.push(`${short}\t${f.status}\t${JSON.stringify(counts)}${f.message ? "\tFILEMSG:" + f.message.split("\n")[0].slice(0, 160) : ""}`);
}
rows.sort();
if (out) { fs.writeFileSync(out, rows.join("\n") + "\n"); fs.writeFileSync(out.replace(/\.rows$/, ".files"), files.sort().join("\n") + "\n"); }
console.log(`files=${r.numTotalTestSuites} total=${r.numTotalTests} passed=${r.numPassedTests} failed=${r.numFailedTests} pending=${r.numPendingTests} todo=${r.numTodoTests}`);
if (!process.argv[4]) console.log(files.join("\n"));
