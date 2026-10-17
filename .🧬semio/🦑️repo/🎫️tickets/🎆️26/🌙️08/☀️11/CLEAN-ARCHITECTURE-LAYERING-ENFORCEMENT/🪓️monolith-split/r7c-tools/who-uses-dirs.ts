// 🔎️ Names of the top-level items of one Rust file that the .rs files below the given directories use (per name: the files): usage: bun who-uses-dirs.ts <defining file> <dir>...
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { parseRustSourceTree, usedNames } from "./lib-imports.ts";
const [def, ...dirs] = process.argv.slice(2);
const names = parseRustSourceTree(readFileSync(def!, "utf8")).items.filter((i: any) => i.name && i.kind !== "use" && i.kind !== "impl").map((i: any) => i.name as string);
const walk = (d: string, acc: string[] = []): string[] => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) walk(p, acc); else if (e.endsWith(".rs")) acc.push(p); } return acc; };
const byName = new Map<string, string[]>();
for (const d of dirs) for (const f of walk(d)) for (const n of usedNames(readFileSync(f, "utf8"), names)) byName.set(n, [...(byName.get(n) ?? []), f.split("/").slice(-4).join("/")]);
for (const [n, fs] of [...byName].sort()) console.log(`${n}: ${[...new Set(fs)].slice(0, 3).join(" ; ")}`);
