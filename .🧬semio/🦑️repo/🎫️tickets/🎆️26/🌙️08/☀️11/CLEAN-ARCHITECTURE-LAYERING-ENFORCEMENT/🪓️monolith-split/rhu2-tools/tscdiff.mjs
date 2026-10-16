import fs from "node:fs";
const [a, b, ...moveSpecs] = process.argv.slice(2);
const moves = new Map();
for (const p of moveSpecs) for (const m of JSON.parse(fs.readFileSync(p, "utf8")).moves ?? []) moves.set(m.from.normalize("NFC"), m.to.normalize("NFC"));
const norm = f => moves.get(f.normalize("NFC")) ?? f.normalize("NFC");
const parse = (p, mapped) => {
  const m = new Map();
  for (const line of fs.readFileSync(p, "utf8").split("\n")) {
    const x = line.match(/^(.*?)\(\d+,\d+\): error (TS\d+): (.*)$/);
    if (!x) continue;
    const msg = x[3].replace(/(?:'(?:\.\.\/)+[^']*'|"(?:\.\.\/)+[^"]*")/g, "<relpath>").replace(/@semio-tech\/framework(?:\/[\w\/.-]+)?/g, "<sdk>");
    const key = `${mapped ? norm(x[1]) : x[1].normalize("NFC")} ${x[2]} ${msg}`;
    m.set(key, (m.get(key) ?? 0) + 1);
  }
  return m;
};
const A = parse(a, true), B = parse(b, false);
let onlyA = 0, onlyB = 0; const lines = [];
for (const [k, n] of A) { const d = n - (B.get(k) ?? 0); if (d > 0) { onlyA += d; lines.push(`- (${d}) ${k.slice(0, 330)}`); } }
for (const [k, n] of B) { const d = n - (A.get(k) ?? 0); if (d > 0) { onlyB += d; lines.push(`+ (${d}) ${k.slice(0, 330)}`); } }
console.log(`before=${[...A.values()].reduce((x, y) => x + y, 0)} after=${[...B.values()].reduce((x, y) => x + y, 0)} removed=${onlyA} added=${onlyB}`);
for (const l of lines) console.log(l);
