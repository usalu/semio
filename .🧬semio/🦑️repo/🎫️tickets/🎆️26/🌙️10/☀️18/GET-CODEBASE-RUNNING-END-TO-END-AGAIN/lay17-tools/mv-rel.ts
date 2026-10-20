/**
 * 🚚 Moves one source file and re-points every relative specifier so it still resolves to the same target.
 *
 *   bun mv-rel.ts <from> <to> [--dry]      (paths relative to the repository root)
 */
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../../..");
const [from, to] = [resolve(ROOT, process.argv[2]!), resolve(ROOT, process.argv[3]!)];
const dry = process.argv.includes("--dry");
const text = readFileSync(from, "utf8");
const rel = (target: string): string => {
  const r = relative(dirname(to), target).split("\\").join("/");
  return r.startsWith(".") ? r : "./" + r;
};
const rewrite = (spec: string): string => {
  if (!spec.startsWith(".")) return spec;
  const [path, query] = spec.split("?");
  const target = resolve(dirname(from), path!);
  return rel(target) + (query === undefined ? "" : "?" + query);
};
let count = 0;
const patterns = [
  /(\bfrom\s*)(["'])(\.[^"']*)\2/gu,
  /(\bimport\s*\(\s*)(["'])(\.[^"']*)\2/gu,
  /(\bimport\s+)(["'])(\.[^"']*)\2/gu,
  /(\b(?:vi\.mock|vi\.doMock|vi\.importActual|require)\s*\(\s*)(["'])(\.[^"']*)\2/gu,
  /(new URL\(\s*)(["'])(\.[^"']*)\2/gu,
];
let out = text;
for (const re of patterns) out = out.replace(re, (_m, pre: string, q: string, spec: string) => { const next = rewrite(spec); if (next !== spec) count++; return `${pre}${q}${next}${q}`; });
console.log(`${process.argv[2]} -> ${process.argv[3]}: ${count} specifiers re-pointed`);
if (dry) process.exit(0);
if (existsSync(to)) throw Error("target exists: " + to);
mkdirSync(dirname(to), { recursive: true });
writeFileSync(to, out);
rmSync(from);
