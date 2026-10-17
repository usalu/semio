import fs from "node:fs";
const g = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const edges = fs.readFileSync(process.argv[3], "utf8").trim().split("\n").map(l => JSON.parse(l));
const ENG = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/";
const isTest = p => /\/🧪️tests\//.test(p) || /\/🧫️fixtures\//.test(p) || /\.test\.tsx?$/.test(p) || /📖️stories/.test(p);
const rev = new Map();
for (const m of g.modules) for (const d of m.dependencies ?? []) { if (!d.resolved) continue; const a = rev.get(d.resolved) ?? []; a.push(m.source); rev.set(d.resolved, a); }
const targets = new Set(edges.filter(e => !isTest(e.from) && /💻️os\//.test(e.to) && !e.to.startsWith(ENG)).map(e => e.to));
const filt = process.argv[4] ? new RegExp(process.argv[4]) : /./;
for (const t of [...targets].sort()) {
  if (!filt.test(t)) continue;
  const imp = [...new Set(rev.get(t) ?? [])];
  const inR = imp.filter(p => p.startsWith(ENG)); const out = imp.filter(p => !p.startsWith(ENG));
  const outSrc = out.filter(p => !isTest(p)); const outTest = out.filter(p => isTest(p));
  console.log(t.replace("🧰️framework/🛍️products/💻️os/", "OS/"), "| renderer:", inR.length, "| other-src:", outSrc.length, "| other-test:", outTest.length);
  if (process.argv[5] === "v") for (const p of outSrc.slice(0, 8)) console.log("      ", p.replace("🧰️framework/🛍️products/💻️os/", "OS/"));
}
