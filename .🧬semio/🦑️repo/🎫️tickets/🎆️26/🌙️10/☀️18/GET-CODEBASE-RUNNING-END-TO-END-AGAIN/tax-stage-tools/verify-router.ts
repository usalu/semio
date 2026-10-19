// 🔬 Runs the command-router validator over every 📜️script.ts of the mirror library (real files) and prints the rejected ones. usage: bun verify-router.ts <mirror> [out.txt]
import {readdirSync, readFileSync, statSync, writeFileSync, lstatSync} from "node:fs";
import {join, resolve} from "node:path";
import {accepts} from "./lib.ts";
import {LIB} from "./vfs.ts";
const mirror = resolve(process.argv[2]!), bad: string[] = [];
let total = 0;
const walk = (dir: string): void => {
  for (const name of readdirSync(dir)) {
    const full = join(dir, name), st = lstatSync(full);
    if (st.isSymbolicLink()) continue;
    if (st.isDirectory()) { if (name !== "node_modules") walk(full); continue; }
    if (name === "📜️script.ts") { total++; if (!accepts(readFileSync(full, "utf8"))) bad.push(full.slice(mirror.length + 1).replace(LIB, "L")); }
  }
};
walk(join(mirror, LIB));
console.log(JSON.stringify({total, rejected: bad.length}));
for (const b of bad) console.log("REJECTED", b);
if (process.argv[3]) writeFileSync(process.argv[3], bad.join("\n") + "\n");
