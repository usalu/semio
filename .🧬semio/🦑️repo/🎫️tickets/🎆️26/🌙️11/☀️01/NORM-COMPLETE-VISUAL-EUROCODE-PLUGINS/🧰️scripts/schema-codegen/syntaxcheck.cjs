const ts = require(require("path").join(process.env.SEMIO_ROOT || require("path").resolve(__dirname, "../../../../../../../../..", ""), "node_modules/typescript"));
const fs = require("fs"), path = require("path");
const root = process.argv[2];
let n = 0, bad = 0;
const walk = (d) => { for (const e of fs.readdirSync(d, { withFileTypes: true })) { if (["target","node_modules","🗑️generated"].includes(e.name)) continue; const p = path.join(d, e.name); if (e.isDirectory()) walk(p); else if (e.name === "🟦️.ts") { const t = fs.readFileSync(p, "utf8"); if (!t.includes("Generated from `./🔣️.json`")) continue; n++; const r = ts.transpileModule(t, { reportDiagnostics: true, compilerOptions: { target: "esnext", module: "esnext" } }); if (r.diagnostics.length) { bad++; if (bad < 6) console.log(p.slice(-80), r.diagnostics[0].messageText); } } } };
walk(root);
console.log("twins parsed", n, "syntax errors", bad);
