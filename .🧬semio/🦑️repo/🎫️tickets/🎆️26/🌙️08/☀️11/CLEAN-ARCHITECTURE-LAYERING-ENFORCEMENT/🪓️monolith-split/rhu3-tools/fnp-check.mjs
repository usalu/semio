import fs from "node:fs";
import path from "node:path";
import { imports } from "./clos.mjs";
const R = "/Users/ueli/Documents/semio/";
const roots = ["🗂️plugin-catalog", "🐚️shell", "🔌️plugin-module/🌎️hub-source/🪝️host-port", "📇️directory", "🛰️backbone", "📞️directory-client", "🌐️browser-bundle", "🤖️mcp", "♾️infinite", "🏷️component-receipt", "🖼️assets/🌐️published-page", "💡️inference/🚪️opening", "🔌️plugin-module", "🎚️config", "🖼️canvas/💾️renderer-resident", "🖼️canvas/🧵️renderer-work"].map(r => R + "🧰️framework/🔨️modules/" + r).concat([R + "🧰️framework/📦️packages/🟦️typescript"]);
const files = [];
const walk = d => { let es; try { es = fs.readdirSync(d, { withFileTypes: true }); } catch { return; } for (const e of es) { if (e.name === "node_modules" || e.name === "dist") continue; const p = path.join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.(tsx?|mts)$/.test(e.name)) files.push(p.replace(R, "").normalize("NFC")); } };
roots.forEach(walk);
let bad = 0;
for (const f of files) for (const d of imports(R + f)) if (d.file && d.file.startsWith(R + "🧰️framework/🛍️products/")) { bad++; console.log("F->products", f.slice(-90), "|", d.spec.slice(-70)); }
console.log("checked", files.length, "violations", bad);
