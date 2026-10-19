import { readFileSync } from "node:fs";
const rows = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as any[];
const OS = "🧰️framework/🛍️products/💻️os/";
const os = rows.filter(r => r.to?.startsWith(OS) && !r.sdk);
const unresolved = rows.filter(r => r.to === null);
console.log("os edges", os.length, "files", new Set(os.map(r => r.from)).size, "unresolved", unresolved.length);
const by: Record<string, number> = {};
for (const r of os) { const k = r.to.slice(OS.length).split("/").slice(0, 3).join("/"); by[k] = (by[k] ?? 0) + 1; }
console.log(Object.entries(by).sort((a, b) => b[1] - a[1]).slice(0, 60).map(([k, v]) => v + " " + k).join("\n"));
const st = os.filter(r => /\/(🏪️store|🖥️host)\//u.test(r.to));
console.log("state-outside-os-ish", st.length);
