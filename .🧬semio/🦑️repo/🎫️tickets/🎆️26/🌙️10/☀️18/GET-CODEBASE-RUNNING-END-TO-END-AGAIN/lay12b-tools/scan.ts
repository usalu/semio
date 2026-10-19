import { readdirSync, readFileSync, writeFileSync, statSync } from "node:fs";
import { join, resolve, relative, dirname } from "node:path";
const root = resolve(process.argv[2]!);
const out = process.argv[3]!;
const UI = "🧰️framework/🔨️modules/🖱️ui";
const files: string[] = [];
const walk = (d: string) => { for (const e of readdirSync(d, { withFileTypes: true })) { if (e.name === "node_modules") continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.(ts|tsx|mts|cts)$/u.test(e.name)) files.push(p); } };
walk(join(root, UI));
const re = /(?:import|export)\s+(?:type\s+)?(?:[^'";]*?\sfrom\s+)?["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)/gu;
const rows: { from: string; spec: string; names: string; kind: string }[] = [];
for (const f of files) {
  const src = readFileSync(f, "utf8");
  for (const m of src.matchAll(re)) {
    const spec = (m[1] ?? m[2])!;
    const stmt = m[0];
    const names = /\{([^}]*)\}/su.exec(stmt)?.[1]?.replace(/\s+/gu, " ").trim() ?? (/\*\s+as\s+(\w+)/u.exec(stmt) ? "*" : "default/side");
    let kind = "";
    if (spec.startsWith("@semio-tech/") && !spec.startsWith("@semio-tech/ui-")) kind = "pkg";
    else if (spec.startsWith(".")) { const t = relative(root, resolve(dirname(f), spec)); if (!t.startsWith(UI + "/") && t !== UI) kind = "rel"; }
    if (kind) rows.push({ from: relative(root, f), spec: kind === "rel" ? relative(root, resolve(dirname(f), spec)) : spec, names, kind });
  }
}
writeFileSync(out, JSON.stringify(rows, null, 1));
console.log(rows.length, files.length);
