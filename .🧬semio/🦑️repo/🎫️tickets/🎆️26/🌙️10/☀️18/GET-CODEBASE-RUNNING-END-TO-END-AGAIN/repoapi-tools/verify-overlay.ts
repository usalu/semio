/**
 * Type-checks the planned rewrite without touching a source file: the changed files are checked once as they are on disk
 * (baseline) and once with the rewritten text overlaid, bare specifiers of manifest keys that do not exist yet resolved
 * through the virtual map of exports-final.json. Diagnostics that exist only after the rewrite are the regressions.
 *
 *   bun repoapi-tools/apply.ts dry          (writes 🗑️generated/repoapi/overlay.json)
 *   bun repoapi-tools/verify-overlay.ts [--only=<substring>]
 */
import ts from "typescript";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { GENERATED, ROOT, posix, readJson, rel, writeJson } from "./lib.ts";
import { packageIndex } from "./packages.ts";
import { buildOtherGlue, extensionText } from "./glue-others.ts";

const only = process.argv.find(a => a.startsWith("--only="))?.slice(7);
const overlay = readJson<Record<string, string>>(resolve(GENERATED, "overlay.json"));
const final = readJson<any>(resolve(import.meta.dir, "exports-final.json"));
const index = packageIndex();
const files = Object.keys(overlay).filter(f => /\.(ts|tsx)$/u.test(f) && existsSync(resolve(ROOT, f)) && (!only || f.includes(only)));

const raw = ts.readConfigFile(resolve(ROOT, "tsconfig.json"), ts.sys.readFile);
const options = { ...ts.parseJsonConfigFileContent(raw.config, ts.sys, ROOT).options, noEmit: true, incremental: false, tsBuildInfoFile: undefined, skipLibCheck: true };

const virtual = new Map<string, string>();
const pending = new Set<string>();
const generated = buildOtherGlue();
const virtualFiles = new Map<string, string>();
for (const g of generated.create) virtualFiles.set(resolve(ROOT, g.file), g.content);
for (const x of generated.extend) virtualFiles.set(resolve(ROOT, x.file), extensionText(x, final));
const libGlue: Record<string, string> = final.repoLib.exportsToAdd;
const lib = index.byName.get("@semio-tech/repo-lib")!;
for (const [k, v] of Object.entries(libGlue)) if (!(k in lib.exports)) virtual.set(`@semio-tech/repo-lib/${k.slice(2)}`, resolve(ROOT, lib.dir, v).replace(/\.mjs$/u, ".d.mts"));
for (const [name, p] of Object.entries<any>(final.packages)) {
  const live = index.byName.get(name);
  for (const e of Object.values<any>(p.entries)) {
    if (e.existing || e.action === "extend-existing-glue" || e.kind === "asset") continue;
    if (live && e.subpath in live.exports && !(e.glueFile && virtualFiles.has(resolve(ROOT, e.glueFile)) && !existsSync(resolve(ROOT, e.glueFile)))) continue;
    const spec = `${name}/${e.subpath.slice(2)}`;
    const abs = resolve(ROOT, e.glueFile ?? e.targetFile);
    if (e.glueFile && !existsSync(abs) && !virtualFiles.has(abs)) pending.add(spec); else virtual.set(spec, abs);
  }
}

function program(withOverlay: boolean): ts.Program {
  const host = ts.createCompilerHost(options, true);
  const baseRead = host.readFile.bind(host), baseSource = host.getSourceFile.bind(host);
  const text = (fileName: string): string | undefined => (withOverlay ? overlay[rel(fileName)] ?? virtualFiles.get(fileName) : undefined);
  const baseExists = host.fileExists.bind(host);
  host.fileExists = f => (withOverlay && virtualFiles.has(f)) || baseExists(f);
  const baseDirectory = host.directoryExists?.bind(host);
  const virtualDirectories = new Set<string>();
  for (const f of virtualFiles.keys()) for (let d = dirname(f); d.length > ROOT.length; d = dirname(d)) virtualDirectories.add(d);
  host.directoryExists = d => (withOverlay && virtualDirectories.has(d)) || (baseDirectory ? baseDirectory(d) : ts.sys.directoryExists(d));
  host.readFile = f => text(f) ?? baseRead(f);
  host.getSourceFile = (f, lang, onError, shouldCreate) => { const t = text(f); return t !== undefined ? ts.createSourceFile(f, t, lang, true, f.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : baseSource(f, lang, onError, shouldCreate); };
  host.resolveModuleNames = (names, containing) => names.map(n => {
    const v = virtual.get(n);
    if (v && (existsSync(v) || (withOverlay && virtualFiles.has(v)))) return { resolvedFileName: v, extension: v.endsWith(".d.mts") ? ts.Extension.Dmts : v.endsWith(".tsx") ? ts.Extension.Tsx : v.endsWith(".json") ? ts.Extension.Json : ts.Extension.Ts, isExternalLibraryImport: false } as ts.ResolvedModuleFull;
    return ts.resolveModuleName(n, containing, options, host).resolvedModule;
  });
  return ts.createProgram({ rootNames: files.map(f => resolve(ROOT, f)), options, host });
}

const normal = (d: ts.Diagnostic): string => `${d.code}|${ts.flattenDiagnosticMessageText(d.messageText, " ").replace(/'[^']*[\\/][^']*'/gu, "'…'").replace(/"[^"]*[\\/][^"]*"/gu, '"…"').slice(0, 200)}`;
function diagnostics(p: ts.Program): Map<string, string[]> {
  const out = new Map<string, string[]>();
  for (const f of files) {
    const sf = p.getSourceFile(resolve(ROOT, f));
    if (!sf) { (out.get(f) ?? out.set(f, []).get(f)!).push("0|source file missing"); continue; }
    for (const d of [...p.getSyntacticDiagnostics(sf), ...p.getSemanticDiagnostics(sf)]) (out.get(f) ?? out.set(f, []).get(f)!).push(normal(d));
  }
  return out;
}

const t0 = Date.now();
console.log(`checking ${files.length} changed files; virtual entries ${virtual.size}, pending glue ${pending.size}`);
const before = diagnostics(program(false));
console.log(`baseline done ${Math.round((Date.now() - t0) / 1000)} s`);
const after = diagnostics(program(true));
console.log(`after done ${Math.round((Date.now() - t0) / 1000)} s`);
const regressions: string[] = [], fixed: string[] = [], pendingGlue: string[] = [];
for (const f of files) {
  const a = [...(before.get(f) ?? [])], b = [...(after.get(f) ?? [])];
  for (const m of b) { const i = a.indexOf(m); if (i >= 0) a.splice(i, 1); else (m.startsWith("2307|") ? pendingGlue : regressions).push(`${f}: ${m}`); }
  for (const m of a) fixed.push(`${f}: ${m}`);
}
writeJson(resolve(GENERATED, "verify-overlay.json"), { files: files.length, baselineDiagnostics: [...before.values()].reduce((n, x) => n + x.length, 0), afterDiagnostics: [...after.values()].reduce((n, x) => n + x.length, 0), regressions, fixed, pendingGlue });
console.log(JSON.stringify({ files: files.length, baselineDiagnostics: [...before.values()].reduce((n, x) => n + x.length, 0), afterDiagnostics: [...after.values()].reduce((n, x) => n + x.length, 0), regressions: regressions.length, fixed: fixed.length, pendingGlue: pendingGlue.length }));
for (const r of regressions.slice(0, 40)) console.log("REGRESSION", r);
void dirname; void posix; void readFileSync;
