import { readFileSync } from "node:fs";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
let ok = 0; const bad: string[] = [];
for (const f of files) {
  try { new Bun.Transpiler({ loader: f.endsWith("x") ? "tsx" : "ts" }).transformSync(readFileSync(f, "utf8")); ok++; } catch (e) { bad.push(`${f.split("/").slice(-3).join("/")}: ${(e as Error).message.slice(0, 120)}`); }
}
console.log(`transpiled ${ok}/${files.length}`);
for (const b of bad) console.log("FAIL", b);
