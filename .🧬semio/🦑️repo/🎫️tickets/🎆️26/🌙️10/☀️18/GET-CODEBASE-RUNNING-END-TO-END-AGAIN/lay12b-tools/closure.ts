import { readdirSync, readFileSync } from "node:fs";
import { join, resolve, relative, dirname } from "node:path";
const root = process.cwd();
const dir = resolve(process.argv[2]!);
const files: string[] = [];
const walk = (d: string) => { for (const e of readdirSync(d, { withFileTypes: true })) { if (e.name === "node_modules" || e.name === "target") continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.(ts|tsx)$/u.test(e.name)) files.push(p); } };
walk(dir);
const re = /(?:import|export)\s+(?:type\s+)?(?:[^'";]*?\sfrom\s+)?["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)/gu;
const out: Record<string, Set<string>> = {};
for (const f of files) {
  const test = /(^|\/)🧪️(tests|testing)\/|\.test\./u.test(relative(root, f));
  const src = readFileSync(f, "utf8");
  for (const m of src.matchAll(re)) {
    const spec = (m[1] ?? m[2])!;
    let key = "";
    if (spec.startsWith("@semio-tech/")) key = spec.split("/").slice(0, 2).join("/") + (spec.split("/").length > 2 ? "/" + spec.split("/")[2] : "");
    else if (spec.startsWith(".")) { const t = relative(root, resolve(dirname(f), spec)); if (!t.startsWith(relative(root, dir) + "/")) key = t.split("/").slice(0, 4).join("/"); }
    if (key) (out[(test ? "TEST " : "SRC  ") + key.replace(/[^\x00-\x7f\/@\-.a-z]/gu, "")] ??= new Set()).add(relative(root, f));
  }
}
for (const [k, v] of Object.entries(out).sort()) console.log(k, v.size, process.argv[3] ? [...v].map((x) => x.replace(/[^\x00-\x7f]/gu, "")).join(" ; ") : "");
