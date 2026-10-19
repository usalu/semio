import { readFileSync } from "node:fs";
const rows = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as any[];
const OS = "🧰️framework/🛍️products/💻️os/";
const os = rows.filter(r => r.to?.startsWith(OS) && !r.sdk);
const g: Record<string, any[]> = {};
for (const r of os) (g[r.to] ??= []).push(r);
for (const [t, rs] of Object.entries(g).sort()) { console.log("\n## " + t.slice(OS.length)); for (const r of rs) console.log("  " + r.spec.slice(0,60).padEnd(60) + "  " + r.from.replace(/✏️s\/🔌️plugins\//u, "")); }
console.log("\nUNRESOLVED"); for (const r of rows.filter(r=>r.to===null)) console.log(r.spec, r.from);
