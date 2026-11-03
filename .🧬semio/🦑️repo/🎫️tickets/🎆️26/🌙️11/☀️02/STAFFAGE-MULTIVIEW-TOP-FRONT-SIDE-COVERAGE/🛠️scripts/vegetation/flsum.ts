import { readFileSync } from "node:fs";
const f = JSON.parse(readFileSync(process.argv[2]!, "utf8")); const rows = Object.entries<any>(f.assets ?? f).filter(([k]) => k.startsWith("vegetation"));
for (const [k, v] of rows) { let mm = 0, deg = 0, n = 0, bad = 0;
  for (const i of v.issues ?? []) if (i.code === "lateral-ambiguous") { n++; const a = /up to ([\d.]+) mm wrong/.exec(i.message), b = /up to ([\d.]+) degrees/.exec(i.message); if (a) mm = Math.max(mm, +a[1]); if (b) deg = Math.max(deg, +b[1]); }
  console.log(k.padEnd(34), "lateral-ambiguous issues", n, "max error mm", mm, "theta_max deg", deg); }
