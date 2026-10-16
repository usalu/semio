import fs from "node:fs";
const [json, ...pats] = process.argv.slice(2);
const r = JSON.parse(fs.readFileSync(json, "utf8"));
const rows = [];
for (const f of r.testResults) for (const t of f.assertionResults) rows.push([f.name.split("/").slice(-2, -1)[0], t.fullName]);
for (const p of pats) {
  const re = new RegExp(p);
  const by = new Map();
  for (const [file, name] of rows) if (re.test(name)) by.set(file, (by.get(file) ?? 0) + 1);
  console.log(`${p.slice(0, 90)} -> ${[...by].map(([k, v]) => `${k}(${v})`).join(" ")}`);
}
