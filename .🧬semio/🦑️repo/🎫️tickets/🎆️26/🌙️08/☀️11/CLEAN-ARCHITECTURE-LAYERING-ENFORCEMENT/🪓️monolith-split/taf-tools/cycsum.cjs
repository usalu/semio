const fs = require("fs");
const G = process.argv[2], P = process.argv[3];
const trees = {
  kernel: ["🧰️framework/🔨️modules/🎠️kernel/"],
  worker: ["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/", "🧰️framework/🛍️products/💻️os/🧪️tests/📚️worker-law-suites/", "🧰️framework/🛍️products/💻️os/🧪️tests/🧰️worker-law-fakes/"],
  rootscript: ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/"],
};
const out = [];
for (const [t, prefs] of Object.entries(trees)) {
  const f = `${G}/taf-cyc-${P}-${t}.txt`;
  if (!fs.existsSync(f)) { out.push(`${t}: missing`); continue; }
  const text = fs.readFileSync(f, "utf8");
  const chunks = text.split(/\n\s*error no-circular:/).slice(1);
  const m = text.match(/(\d+) dependency violations/);
  const seen = new Map();
  for (const ch of chunks) {
    const nodes = ch.split("→").map(s => s.replace(/\s+/g, " ").trim()).filter(s => /\.(ts|tsx|rs|json|cjs|mjs)$/.test(s));
    if (!nodes.length) continue;
    const uniq = [...new Set(nodes)];
    const key = uniq.slice().sort().join("|");
    if (!seen.has(key)) seen.set(key, uniq);
  }
  const inTree = n => prefs.some(p => n.startsWith(p));
  const all = [...seen.values()];
  const touching = all.filter(ns => ns.some(inTree));
  const inside = all.filter(ns => ns.every(inTree));
  const viaTest = all.filter(ns => ns.some(n => /worker-law-suites|worker-law-fakes|createturnoutcomebroadcast|🧪️createturnoutcomebroadcast/.test(n)));
  out.push(`${t}: reported violations=${m ? m[1] : "?"}; distinct cycles=${all.length}; touching tree=${touching.length}; entirely inside tree=${inside.length}; through the worker-law or turn-outcome test modules=${viaTest.length}`);
  fs.writeFileSync(`${G}/taf-cyc-${P}-${t}-touching.txt`, touching.map(n => n.join("  ->  ")).join("\n") + "\n");
}
fs.writeFileSync(`${G}/taf-cyc-${P}-summary.txt`, out.join("\n") + "\n");
console.log(out.join("\n"));
