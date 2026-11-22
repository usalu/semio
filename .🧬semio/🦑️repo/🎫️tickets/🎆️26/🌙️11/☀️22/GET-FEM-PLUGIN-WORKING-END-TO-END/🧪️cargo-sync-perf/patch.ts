const fs = require("node:fs");
const counts: Record<string, number> = { lstat: 0, readdir: 0, read: 0, time_lstat_ms: 0 };
for (const [name, key] of [["lstatSync", "lstat"], ["readdirSync", "readdir"], ["readFileSync", "read"]] as const) {
  const original = fs[name];
  fs[name] = (...a: any[]) => { counts[key]++; const s = performance.now(); try { return original(...a); } finally { if (key === "lstat") counts.time_lstat_ms += performance.now() - s; } };
}
process.on("exit", () => console.error("[DEBUG] fs counts", JSON.stringify(counts)));
