/**
 * 🩹 One-off repair of `repoapi-tools/exports-final.json` after the `.//` generator fix: normalises every path string (values and keys) that carries an empty segment.
 *
 *   bun glue-tools/fix-final-spec.ts
 */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const file = resolve(import.meta.dir, "../repoapi-tools/exports-final.json");
const text = readFileSync(file, "utf8");
let changed = 0;
const fix = (s: string): string => { if (!/(^|[^:])\/\//u.test(s)) return s; changed++; return s.replace(/^\.\/\//u, "./").replace(/([^:/])\/\/+/gu, "$1/"); };
const walk = (v: any): any => typeof v === "string" ? fix(v) : Array.isArray(v) ? v.map(walk) : v && typeof v === "object" ? Object.fromEntries(Object.entries(v).map(([k, x]) => [fix(k), walk(x)])) : v;
const indent = /^\{\n( +)"/u.exec(text)?.[1]?.length ?? 2;
writeFileSync(file, JSON.stringify(walk(JSON.parse(text)), null, indent) + "\n");
console.log(`normalised ${changed} strings`);
