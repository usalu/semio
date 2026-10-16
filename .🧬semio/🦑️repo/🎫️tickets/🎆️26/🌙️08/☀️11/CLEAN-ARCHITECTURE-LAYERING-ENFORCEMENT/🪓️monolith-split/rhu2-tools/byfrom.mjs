import fs from "node:fs";
const rows = fs.readFileSync(process.argv[2], "utf8").trim().split("\n").map(l => JSON.parse(l));
const isTest = p => /\/🧪️tests\//.test(p) || /\/🧫️fixtures\//.test(p) || /\.test\.tsx?$/.test(p);
const seen = new Set(); const by = new Map(); const files = new Map();
for (const r of rows) { const k = r.from + "|" + r.to + "|" + r.spec; if (seen.has(k)) continue; seen.add(k); if (isTest(r.from)) continue; if (r.to.includes("node_modules/react") ) continue; const f = r.from.replace(/^.*?engine\//u, ""); const parts = f.split("/"); const key = parts[0] === "🧱️elements" ? parts.slice(0, 2).join("/") : parts.slice(0, 2).join("/"); by.set(key, (by.get(key) ?? 0) + 1); const s = files.get(key) ?? new Set(); s.add(r.from); files.set(key, s); }
let t = 0; for (const [k, v] of [...by].sort((a, b) => b[1] - a[1])) { t += v; console.log(v, files.get(k).size + "f", k); } console.log("total", t);
