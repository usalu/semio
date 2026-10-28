// usage: bun scope-overlay.ts [overlayDir]: the executor-command closure + scheduling split of the checkout (optionally with staged files shadowing it)
import { createHash } from "node:crypto";
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, normalize, relative, resolve, sep } from "node:path";
import ts from "typescript";
const checkout = process.cwd(), overlay = process.argv[2] ? resolve(process.argv[2]) : undefined;
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const lib = (p: string) => import(`${checkout}/${L}/${p}`);
const { bazelScopesV1 } = await lib("🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🟦️.ts");
const { splitBuildSourceClosureV1, buildSchedulingV1, buildSchedulingPortsV1, buildSchedulingPrefixesV1 } = await lib("🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts");
const corpus = JSON.parse(readFileSync(join(checkout, L, "🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🔣️.json"), "utf8"));
const entry = normalize(resolve(checkout, L, "🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests", corpus.entryFromTests));
const shadow = (path: string) => { if (overlay) { const o = join(overlay, relative(checkout, path)); if (existsSync(o)) return o; } return path; };
const regular = (path: string) => { const p = shadow(path); return existsSync(p) && statSync(p).isFile() && !path.split(sep).some((part) => part === "node_modules" || part === ".cache"); };
const read = (path: string) => readFileSync(shadow(path));
const schemaFiles = (folder: string): string[] => existsSync(folder) ? readdirSync(folder, { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : 1)).flatMap((e) => e.isDirectory() && e.name !== "node_modules" ? schemaFiles(join(folder, e.name)) : e.isFile() && e.name.endsWith(".json") ? [join(folder, e.name)] : []) : [];
const staticSpecifiers = (path: string): string[] => { const f = ts.createSourceFile(path, read(path).toString("utf8"), ts.ScriptTarget.Latest, true), out: string[] = []; for (const n of f.statements) { if (ts.isImportDeclaration(n) && !n.importClause?.isTypeOnly && ts.isStringLiteral(n.moduleSpecifier)) out.push(n.moduleSpecifier.text); if (ts.isExportDeclaration(n) && !n.isTypeOnly && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) out.push(n.moduleSpecifier.text); } return out; };
const jsonLiteral = /(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;
const closureOf = (roots: readonly string[], leaves: readonly string[]) => {
  const seen = new Set<string>(), pending = [normalize(entry), ...roots.map((r) => normalize(join(checkout, r)))];
  while (pending.length) { const path = pending.pop()!; if (seen.has(path) || !regular(path)) continue; seen.add(path); if (path.endsWith(".json")) continue; pending.push(...schemaFiles(join(dirname(path), "🧬️schema"))); for (const s of staticSpecifiers(path)) if (s.startsWith(".")) pending.push(normalize(join(dirname(path), s))); for (const m of read(path).toString("utf8").matchAll(jsonLiteral)) pending.push(normalize(join(dirname(path), m[2]!))); }
  for (const leaf of leaves) if (regular(normalize(join(checkout, leaf)))) seen.add(normalize(join(checkout, leaf)));
  return seen;
};
for (const row of bazelScopesV1()) {
  const closure = closureOf(row.scope.roots, row.scope.leaves);
  const files = [...closure].sort().map((path) => { const bytes = read(path), imports: Record<string, string> = {}; if (!path.endsWith(".json")) for (const s of staticSpecifiers(path)) { if (!s.startsWith(".")) continue; const t = normalize(join(dirname(path), s)); if (closure.has(t)) imports[s] = relative(checkout, t); } return { path: relative(checkout, path), sha256: createHash("sha256").update(bytes).digest("hex"), base64: bytes.toString("base64"), imports }; });
  const { keyed, overlay: sched } = splitBuildSourceClosureV1({ version: 1, root: checkout, entry: relative(checkout, entry), files }, row.scheduling);
  const policy = buildSchedulingV1(), ports = buildSchedulingPortsV1(policy, row.scheduling), own = new Set(keyed.files.map((f: any) => f.path));
  const stray = keyed.files.flatMap((f: any) => Object.values(f.imports as Record<string, string>).filter((t) => !own.has(t) && !ports.has(t)).map((t) => f.path.replace(L, "L") + " -> " + t.replace(L, "L")));
  console.log(row.id, "closure", closure.size, "keyed", keyed.files.length, "scheduling", (sched?.files ?? []).length, "STRAY", stray.length);
  for (const s of stray) console.log("  STRAY", s);
  if (process.env.LIST) for (const p of [...closure].sort()) console.log("  F", relative(checkout, p).replace(L, "L"));
}
if (process.env.BARE) for (const row of bazelScopesV1()) for (const path of closureOf(row.scope.roots, row.scope.leaves)) { if (path.endsWith(".json")) continue; for (const s of staticSpecifiers(path)) if (!s.startsWith(".") && !s.startsWith("node:") && !s.startsWith("bun:") && s !== "bun") console.log("BARE", relative(checkout, path).replace(L, "L"), s); }
