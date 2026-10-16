import fs from "node:fs";
const [baseNorm, afterNorm, ...scopeFiles] = process.argv.slice(2);
const R = "/Users/ueli/Documents/semio/";
const scope = new Set();
const prefixes = [];
for (const f of scopeFiles) {
  if (f.startsWith("prefix:")) { prefixes.push(f.slice(7)); continue; }
  for (const l of fs.readFileSync(f, "utf8").split("\n")) { const p = l.split("\t")[0].replace(/:\d+$/, "").replace(R, ""); if (p) scope.add(p); }
}
const inScope = (line) => { const file = line.split(": error ")[0].replace(R, ""); return scope.has(file) || prefixes.some((p) => file.startsWith(p)); };
const read = (f) => fs.readFileSync(f, "utf8").split("\n").filter(Boolean).filter(inScope);
const a = read(baseNorm), b = read(afterNorm);
const count = (xs) => xs.reduce((m, x) => m.set(x, (m.get(x) ?? 0) + 1), new Map());
const ca = count(a), cb = count(b);
const only = (x, y) => [...x].filter(([k, v]) => (y.get(k) ?? 0) < v);
console.log("scoped baseline", a.length, "after", b.length);
console.log("only in baseline:", only(ca, cb).map(([k, v]) => `${v - (cb.get(k) ?? 0)}x ${k.slice(0, 200)}`).join("\n  ") || "none");
console.log("only after:", only(cb, ca).map(([k, v]) => `${v - (ca.get(k) ?? 0)}x ${k.slice(0, 200)}`).join("\n  ") || "none");
