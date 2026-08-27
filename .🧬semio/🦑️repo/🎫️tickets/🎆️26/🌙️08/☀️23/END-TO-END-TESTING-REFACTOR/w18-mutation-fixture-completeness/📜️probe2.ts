import { readdirSync, lstatSync } from "node:fs";
import { join, relative, basename } from "node:path";
const root = process.cwd();
const SKIP = new Set(["node_modules", "target", ".git", "temp"]);
let found = 0, entered = 0;
const hits: string[] = [];
const stack = [root];
while (stack.length) {
  const dir = stack.pop()!;
  let entries: string[]; try { entries = readdirSync(dir); } catch { continue; }
  for (const name of entries) {
    const abs = join(dir, name);
    let st; try { st = lstatSync(abs); } catch { continue; }
    if (!st.isDirectory() || st.isSymbolicLink()) continue;
    if (SKIP.has(name)) continue;
    if (name.includes("oracle")) { found++; hits.push(relative(root, abs)); continue; }
    entered++;
    stack.push(abs);
  }
}
console.log("dirs containing 'oracle':", found, "entered:", entered);
console.log("sample:", hits.slice(0, 5));
const target = "🧪️oracle";
console.log("exact matches to literal:", hits.filter((h) => basename(h) === target).length);
const codes = [...new Set(hits.map((h) => [...basename(h)].map((c) => c.codePointAt(0)!.toString(16)).join(",")))];
console.log("distinct basename codepoint signatures:", codes.slice(0, 6));
console.log("literal signature:", [...target].map((c) => c.codePointAt(0)!.toString(16)).join(","));
