const fs = require("node:fs");
const counts: Record<string, number> = {};
const dup = new Map<string, number>();
const original = fs.lstatSync;
fs.lstatSync = (...a: any[]) => { const key = a[1]?.bigint ? "big" : "plain"; counts[key] = (counts[key] ?? 0) + 1; if (key === "plain") dup.set(a[0], (dup.get(a[0]) ?? 0) + 1); return original(...a); };
process.on("exit", () => { console.error("[DEBUG] lstat kinds", JSON.stringify(counts), "distinct plain paths", dup.size); console.error([...dup].sort((a, b) => b[1] - a[1]).slice(0, 8).map(([p, n]) => n + " " + p.slice(-70)).join("\n")); });
