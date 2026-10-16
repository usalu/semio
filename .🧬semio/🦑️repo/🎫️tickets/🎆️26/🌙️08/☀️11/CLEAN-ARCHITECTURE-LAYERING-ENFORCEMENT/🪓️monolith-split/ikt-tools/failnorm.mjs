import fs from "node:fs";
const [log] = process.argv.slice(2);
const t = fs.readFileSync(log, "utf8").replace(/\x1b\[[0-9;]*m/g, "");
const tests = new Set(), files = new Set();
for (const l of t.split("\n")) {
  const m = /^ FAIL +\|[^|]*\| (.*)$/.exec(l) ?? /^ FAIL +(.*)$/.exec(l);
  if (!m) continue;
  const body = m[1].trim();
  const fm = /^(\S.*?) \[ (.*) \]$/.exec(body);
  if (fm) { files.add(fm[1].replace(/^(\.\.\/)+/, "")); continue; }
  const idx = body.indexOf(" > ");
  if (idx < 0) { files.add(body.replace(/^(\.\.\/)+/, "")); continue; }
  tests.add(body.slice(idx + 3));
}
console.log([...tests].sort().map((x) => "T " + x).concat([...files].sort().map((x) => "F " + x)).join("\n"));
