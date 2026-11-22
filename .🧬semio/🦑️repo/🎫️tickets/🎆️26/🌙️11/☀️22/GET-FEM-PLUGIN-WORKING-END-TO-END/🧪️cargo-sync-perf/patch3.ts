const fs = require("node:fs");
const stacks = new Map<string, number>();
const original = fs.lstatSync;
fs.lstatSync = (...a: any[]) => { if (String(a[0]).endsWith("🔨️modules") && !a[1]?.bigint) { const s = (new Error().stack ?? "").split("\n").slice(2, 7).map(l => l.replace(/\(file:.*[\/]/, "(")).join(" | "); stacks.set(s, (stacks.get(s) ?? 0) + 1); } return original(...a); };
process.on("exit", () => { for (const [s, n] of [...stacks].sort((a, b) => b[1] - a[1]).slice(0, 5)) console.error("[DEBUG]", n, s.slice(0, 600)); });
