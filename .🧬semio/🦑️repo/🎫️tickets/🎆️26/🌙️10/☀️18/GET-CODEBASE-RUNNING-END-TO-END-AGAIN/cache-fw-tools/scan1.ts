import { readFileSync, existsSync, statSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { ROOT } from "./lib.ts";
const file = process.argv[2];
const abs = join(ROOT, file);
const src = readFileSync(abs, "utf8");
const lits = [...src.matchAll(/(["'`])((?:\.{1,2}\/)[^"'`\n$]*?)\1/g)].map((m) => m[2]);
const imports = [...src.matchAll(/(?:from\s*|import\s*\(\s*|require\s*\(\s*)(["'])([^"']+)\1/g)].map((m) => m[2]);
const set = new Set(lits.filter((l) => !imports.includes(l)));
for (const l of set) { const p = resolve(dirname(abs), l); console.log(l, existsSync(p) ? (statSync(p).isDirectory() ? "DIR" : "file") : "missing"); }
