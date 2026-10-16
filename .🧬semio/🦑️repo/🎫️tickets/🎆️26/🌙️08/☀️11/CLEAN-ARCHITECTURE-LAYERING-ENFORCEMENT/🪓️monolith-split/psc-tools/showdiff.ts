/** 👁️ Prints the changed lines of snapshot files against the live tree. Usage: bun showdiff.ts <snapshot-dir> <list.txt|substring> */
import { readFileSync, readdirSync, existsSync } from "node:fs";
const [dir, sel] = process.argv.slice(2);
const wanted = existsSync(sel) ? readFileSync(sel, "utf8").split("\n").filter(Boolean) : null;
for (const n of readdirSync(dir)) {
  if (!n.endsWith(".json")) continue;
  const snap: Record<string, string> = JSON.parse(readFileSync(dir + "/" + n, "utf8"));
  for (const [f, before] of Object.entries(snap)) {
    if (wanted ? !wanted.includes(f) : !f.includes(sel)) continue;
    const a = before.split("\n"), b = readFileSync("/Users/ueli/Documents/semio/" + f, "utf8").split("\n");
    console.log("==", f.slice(-100));
    for (let i = 0; i < Math.max(a.length, b.length); i++) if (a[i] !== b[i]) console.log(`- ${a[i]?.slice(0, 230)}\n+ ${b[i]?.slice(0, 230)}`);
  }
}
