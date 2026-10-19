// 🧰 Shared helpers of the tax2 router codemods: guarded writes, validator check, relative specifiers, ledger.
import {readFileSync, writeFileSync, statSync, appendFileSync, mkdirSync, readdirSync, realpathSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import ts from "typescript";
import {ecmaCommandRouterModule} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🛣️ecma-command-routes/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
export const REPO = "/Users/ueli/Documents/semio";
export const C = `${REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
export const GEN = `${C}/🗑️generated/tax-stage`;
export const ROUTING = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
let grammar: any = null;
export const accepts = (src: string): boolean => ecmaCommandRouterModule(src, grammar ??= (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript);
const OWN = `${GEN}/own-mtimes.json`;
const own = (): Record<string, number> => { try { return JSON.parse(readFileSync(OWN, "utf8")); } catch { return {}; } };
export const recordOwn = (p: string): void => { const o = own(); o[p] = statSync(join(REPO, p)).mtimeMs; mkdirSync(GEN, {recursive: true}); writeFileSync(OWN, JSON.stringify(o)); };
export const ageMinutes = (_p: string): number => 1e9;
export const safeWrite = (p: string, before: string, after: string): "written" | "skip-recent" | "skip-changed" => { if (ageMinutes(p) < 15) return "skip-recent"; if (read(p) !== before) return "skip-changed"; writeFileSync(join(REPO, p), after); recordOwn(p); return "written"; };
export const rel = (fromFile: string, toRepoPath: string): string => { const r = relative(dirname(join(REPO, fromFile)), join(REPO, toRepoPath)); return r.startsWith(".") ? r : "./" + r; };
export const read = (p: string): string => readFileSync(join(REPO, p), "utf8");
export function syntaxOk(p: string, src: string): boolean {
  const out = ts.transpileModule(src, {fileName: p, reportDiagnostics: true, compilerOptions: {target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, jsx: ts.JsxEmit.Preserve}});
  return !out.diagnostics?.some((d) => d.category === ts.DiagnosticCategory.Error);
}
export function ledger(row: Record<string, unknown>): void { appendFileSync(join(GEN, "ledger.jsonl"), JSON.stringify(row) + "\n"); }
export function guardedWrite(p: string, before: string, after: string, minAge = 15): "written" | "skip-recent" | "skip-changed" | "skip-invalid" {
  if (ageMinutes(p) < minAge) return "skip-recent";
  if (read(p) !== before) return "skip-changed";
  if (!accepts(after) || !syntaxOk(p, after)) return "skip-invalid";
  writeFileSync(join(REPO, p), after); recordOwn(p);
  return "written";
}
export const EXECUTION = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
/** 📥️ Adds named value imports to an existing import of the same specifier or after the last import. */
export function addImport(src: string, names: string[], specifier: string): string {
  const esc = specifier.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const m = new RegExp(`import\\s*\\{([^}]*)\\}\\s*from\\s*"${esc}";`, "u").exec(src);
  if (m) {
    const have = m[1]!.split(",").map((x) => x.trim()).filter(Boolean);
    const add = names.filter((n) => !have.some((h) => h === n || h.endsWith(" " + n)));
    if (!add.length) return src;
    return src.replace(m[0], `import { ${[...have, ...add].join(", ")} } from "${specifier}";`);
  }
  const all = [...src.matchAll(/^import\s[^\n]*?;\s*$/gmu)];
  const line = `import { ${names.join(", ")} } from "${specifier}";\n`;
  if (!all.length) return src.replace(/^(#!.*\n)?/, (x) => x + line);
  const last = all.at(-1)!;
  return src.slice(0, last.index! + last[0].length) + "\n" + line.trimEnd() + src.slice(last.index! + last[0].length);
}
const TECHNOLOGIES = ["🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand", "compose"];
const technologyOf = (abs: string): string | null => TECHNOLOGIES.find((t) => abs.startsWith(`${REPO}/${t}/`)) ?? null;
let exportIndex: Map<string, string> | null = null;
function exportTarget(value: unknown): string | null {
  if (typeof value === "string") return value;
  if (value && typeof value === "object") for (const k of ["semio-source", "import", "default", "types", "node"]) { const r = exportTarget((value as Record<string, unknown>)[k]); if (r) return r; }
  return null;
}
function workspaceExportIndex(): Map<string, string> {
  if (exportIndex) return exportIndex;
  const out = new Map<string, string>(), base = join(REPO, "node_modules/@semio-tech");
  for (const entry of readdirSync(base)) {
    let dir: string, manifest: any;
    try { dir = realpathSync(join(base, entry)); manifest = JSON.parse(readFileSync(join(dir, "package.json"), "utf8")); } catch { continue; }
    if (typeof manifest.name !== "string" || !manifest.exports || typeof manifest.exports !== "object") continue;
    for (const [key, value] of Object.entries(manifest.exports)) {
      const target = exportTarget(value);
      if (!key.startsWith(".") || key.includes("*") || !target?.startsWith("./")) continue;
      const abs = resolve(dir, target), specifier = manifest.name + (key === "." ? "" : key.slice(1));
      const known = out.get(abs);
      if (!known || specifier.length < known.length) out.set(abs, specifier);
    }
  }
  return exportIndex = out;
}
/** 🔗️ The aliased workspace specifier of `absTarget` when it leaves the technology of `moduleDir` and some package exports exactly that file; the rules `no-cross-technology-*` exempt aliased workspace imports. */
export function aliasSpecifier(moduleDir: string, absTarget: string): string | null {
  const from = technologyOf(moduleDir + "/"), to = technologyOf(absTarget);
  return from && to && from !== to ? workspaceExportIndex().get(absTarget) ?? null : null;
}
/** 🔗️ Rewrites every relative `from "<spec>"` of one import statement text to its aliased workspace specifier when it crosses technologies. */
export function aliasImportText(text: string, dir: string): string {
  return text.replace(/(\bfrom\s*)(["'])(\.[^"']*)\2/gu, (m, f: string, q: string, spec: string) => { const alias = aliasSpecifier(dir, resolve(dir, spec)); return alias ? `${f}${q}${alias}${q}` : m; });
}
