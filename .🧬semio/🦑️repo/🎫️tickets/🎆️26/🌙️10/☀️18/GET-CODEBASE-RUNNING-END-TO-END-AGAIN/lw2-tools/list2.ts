import { readFileSync } from "node:fs";
const rows = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as any[];
const OS = "🧰️framework/🛍️products/💻️os/";
const os = rows.filter(r => r.to?.startsWith(OS) && !r.sdk);
const g: Record<string, any[]> = {};
for (const r of os) (g[r.spec.startsWith(".") ? "REL " + r.to.slice(OS.length) : r.spec] ??= []).push(r);
for (const [t, rs] of Object.entries(g).sort()) console.log(String(rs.length).padStart(3), t, rs.length<=2? "  <- "+rs.map(r=>r.from.replace(/✏️s\/🔌️plugins\//u, "")).join(" | "):"");
