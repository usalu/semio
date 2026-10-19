import { readdirSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/";
const dirs = [ROOT + "🔬️interactivity-audit/🗄️database/🧪️tests/", ROOT + "🎯️tool-job-coverage/🧪️tests/"];
for (const d of dirs) for (const n of readdirSync(d)) {
  const m = await import(d + n + "/🟦️.ts");
  for (const [k, f] of Object.entries<any>(m)) {
    if (typeof f !== "function" || !/SelfTests$/.test(k)) continue;
    if (f.length > 0) { console.log("SKIP(args)", n, k, f.length); continue; }
    try { const r = await f(); console.log("PASS", n, k, JSON.stringify(r) ?? ""); } catch (e: any) { console.log("FAIL", n, k, String(e?.message ?? e).slice(0, 200)); }
  }
}
