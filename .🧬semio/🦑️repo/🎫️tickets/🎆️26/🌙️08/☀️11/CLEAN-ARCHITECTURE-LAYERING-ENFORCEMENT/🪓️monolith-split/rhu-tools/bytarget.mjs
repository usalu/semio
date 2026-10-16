import fs from "node:fs";
const rows = fs.readFileSync(process.argv[2], "utf8").trim().split("\n").map(l => JSON.parse(l));
const filter = process.argv[3] ? new RegExp(process.argv[3]) : /./;
const testOnly = process.argv[4] === "test";
const isTest = p => /\/🧪️tests\//.test(p) || /\/🧫️fixtures\//.test(p) || /\.test\.tsx?$/.test(p);
const seen = new Set(); const by = new Map();
for (const r of rows) { const k = r.from + "|" + r.to + "|" + r.spec; if (seen.has(k)) continue; seen.add(k); if (isTest(r.from) !== testOnly) continue; if (!filter.test(r.to)) continue; if (r.to.startsWith(".cache/") ) continue; const a = by.get(r.to) ?? []; a.push(r.from.replace("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/", "E/")); by.set(r.to, a); }
for (const [t, a] of [...by].sort((x, y) => y[1].length - x[1].length)) console.log(a.length, t.replace("🧰️framework/🛍️products/💻️os/", "OS/").replace("🧰️framework/", "F/"), "\n    ", a.slice(0, 6).join("\n     "));
