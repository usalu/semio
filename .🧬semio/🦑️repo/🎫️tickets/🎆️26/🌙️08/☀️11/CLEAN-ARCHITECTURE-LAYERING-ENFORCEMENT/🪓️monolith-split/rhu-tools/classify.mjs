import fs from "node:fs";
const rows = fs.readFileSync(process.argv[2], "utf8").trim().split("\n").map(l => JSON.parse(l));
const uniq = new Map();
for (const r of rows) uniq.set(r.from + "\u0000" + r.to + "\u0000" + r.spec, r);
const isTest = p => /\/🧪️tests\//.test(p) || /\/🧫️fixtures\//.test(p) || /\.test\.tsx?$/.test(p) || /📖️stories/.test(p);
const FW = "🧰️framework/";
function cls(to) {
  if (to.startsWith(".cache/b/d/m/")) { const m = to.match(/node_modules\/((?:@[^/]+\/)?[^/]+)/); return "npm:" + (m ? m[1] : "?"); }
  if (!to.includes("/") ) return "builtin:" + to;
  if (to.startsWith("node_modules/")) return "npm:" + to.split("/")[1];
  const OS = FW + "🛍️products/💻️os/";
  if (to.startsWith(OS + "🔨️modules/")) { const rest = to.slice((OS + "🔨️modules/").length).split("/"); return "os-modules:" + (rest[0] === "📺️renderer" ? "renderer-other" : rest[0]); }
  if (to.startsWith(OS + "📦️packages/")) return "os-packages";
  if (to.startsWith(OS)) return "os-other:" + to.slice(OS.length).split("/")[0];
  if (to.startsWith(FW + "🛍️products/")) return "product:" + to.split("/")[2];
  if (to.startsWith(FW + "🔨️modules/")) return "fw-modules:" + to.split("/").slice(2, 4).join("/");
  if (to.startsWith("✏️s/")) return "s:" + to.split("/").slice(0, 3).join("/");
  return "other:" + to.split("/").slice(0, 3).join("/");
}
const out = { prod: new Map(), test: new Map() };
for (const r of uniq.values()) { const k = cls(r.to); const m = isTest(r.from) ? out.test : out.prod; m.set(k, (m.get(k) ?? 0) + 1); }
for (const [n, m] of Object.entries(out)) { console.log("==", n, [...m.values()].reduce((a, b) => a + b, 0)); for (const [k, v] of [...m].sort((a, b) => b[1] - a[1])) console.log(v, k); }
