/** 👥️ Counts consumers per SDK subpath among plugin files; reports entries used only by layout-owned files or by nobody. Usage: bun consumers.ts <files.txt> */
import { readFileSync } from "node:fs";
import { ROOT, SDK_DIR, scanSource, readLines } from "./lib.ts";
const files = readLines(process.argv[2]).map(f => f.startsWith("/") ? f.slice(ROOT.length + 1) : f);
const pkg = JSON.parse(readFileSync(`${ROOT}/${SDK_DIR}/package.json`, "utf8"));
const use = new Map<string, { other: number; layout: number }>();
for (const k of Object.keys(pkg.exports)) use.set(k, { other: 0, layout: 0 });
const LAYOUT = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/";
for (const f of files) {
  let text: string; try { text = readFileSync(ROOT + "/" + f, "utf8"); } catch { continue; }
  if (!text.includes("@semio-tech/framework") && !text.includes("📦️packages/🟦️typescript")) continue;
  for (const r of scanSource(f, text)) {
    const m = r.spec.match(/^@semio-tech\/framework(\/.*)?$/);
    const key = m ? (m[1] ? "." + m[1] : ".") : null;
    if (key && use.has(key)) { const u = use.get(key)!; if (f.startsWith(LAYOUT)) u.layout++; else u.other++; continue; }
    if (r.resolved && r.resolved.startsWith(SDK_DIR + "/")) {
      for (const [k, t] of Object.entries(pkg.exports as Record<string, string>)) if ((SDK_DIR + "/" + t.slice(2)).normalize("NFC") === r.resolved) { const u = use.get(k)!; if (f.startsWith(LAYOUT)) u.layout++; else u.other++; }
    }
  }
}
let unused = 0, layoutOnly = 0;
for (const [k, u] of use) { if (u.other === 0 && u.layout === 0) { unused++; console.log("UNUSED", k); } else if (u.other === 0) { layoutOnly++; console.log("LAYOUT-ONLY", k, u.layout); } }
console.log({ entries: use.size, unused, layoutOnly });
