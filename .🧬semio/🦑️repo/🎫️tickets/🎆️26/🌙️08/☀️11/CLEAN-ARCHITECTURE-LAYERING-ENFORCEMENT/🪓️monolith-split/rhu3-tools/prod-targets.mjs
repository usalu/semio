import fs from "node:fs";
const rows = fs.readFileSync(process.argv[2], "utf8").trim().split("\n").map(l => JSON.parse(l));
const isTest = p => /\/🧪️tests\//.test(p) || /\/🧫️fixtures\//.test(p) || /\.test\.tsx?$/.test(p) || /📖️stories/.test(p);
const OS = "🧰️framework/🛍️products/💻️os/", ENG = OS + "🔨️modules/📺️renderer/🧑‍🎨engine/";
const sh = p => p.replace(ENG + "🧱️elements/", "E/").replace(ENG, "RT/").replace(OS, "OS/").replace("🧰️framework/", "F/");
const by = new Map();
for (const r of rows) { if (isTest(r.from) || r.to.startsWith(".cache/")) continue; const k = sh(r.to); if (!by.has(k)) by.set(k, []); by.get(k).push(sh(r.from) + "  [" + r.spec.slice(0,60) + "]"); }
for (const [k, v] of [...by].sort((a,b)=>a[0].localeCompare(b[0]))) { console.log(v.length + "  " + k); if (process.argv[3]) for (const x of v) console.log("     <- " + x); }
