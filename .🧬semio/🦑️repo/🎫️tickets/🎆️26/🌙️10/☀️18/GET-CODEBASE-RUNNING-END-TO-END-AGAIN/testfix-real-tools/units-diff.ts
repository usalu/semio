import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import Parser from "web-tree-sitter";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
const root = process.argv[2]!, repo = "/Users/ueli/Documents/semio";
const unitDir = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units");
await Parser.init({ wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root)) });
const p = new Parser(); p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const sha = (s: string) => createHash("sha256").update(s).digest("hex");
const asserts = (s: string) => (s.match(/\bassert(?:_eq|_ne)?!/g) ?? []).length;
const norm = (s: string) => s.split("\n").map(l => l.replace(/[ \t]+$/u, "")).join("\n");
function fns(src: string, lawOnly = false) {
  const t = p.parse(src); const out: { name: string; text: string }[] = [];
  for (const fn of t.rootNode.descendantsOfType("function_item")) {
    let isLaw = false;
    for (let a = fn.previousNamedSibling; a?.type === "attribute_item"; a = a.previousNamedSibling) if (/\btest\b|::async_test\b/.test(a.text)) isLaw = true;
    if (lawOnly && !isLaw) continue;
    out.push({ name: fn.childForFieldName("name")!.text, text: fn.text });
  }
  t.delete(); return out;
}
const commits = ["bfe5c8314f5", "4dab415ed90", "7880cc36f78", "8f91299152c", "fd14da55f18", "bd9e4e02bd0"];
const histCache = new Map<string, string | null>();
function hist(c: string, path: string): string | null {
  const k = c + ":" + path; if (histCache.has(k)) return histCache.get(k)!;
  let v: string | null = null; try { v = execFileSync("git", ["show", `${c}:${path}`], { cwd: repo, maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "ignore"] }).toString("utf8"); } catch { v = null; }
  histCache.set(k, v); return v;
}
const preserved = JSON.parse(readFileSync(resolve(unitDir, "📋️preserved.json"), "utf8"));
const report: any = { changed: [], neutral: [], newNames: [], removed: [], unknown: [] };
const actual = new Map<string, { source: string; text: string }>();
for (const path of new Set<string>([preserved.original, ...preserved.laws.map((r: any) => r.source)])) {
  const src = path === preserved.original ? readModuleTreeSource(resolve(root, path)) : readFileSync(resolve(root, path), "utf8");
  for (const f of fns(src, true)) actual.set(f.name, { source: path, text: f.text });
}
const declared = new Map<string, any>(preserved.laws.map((r: any) => [r.name, r]));
for (const [name, a] of actual) {
  const d = declared.get(name);
  if (!d) { report.newNames.push({ name, source: a.source, assertions: asserts(a.text), sha: sha(a.text) }); continue; }
  if (sha(a.text) === d.sha256) continue;
  let found: string | null = null, how = "";
  for (const c of commits) { const h = hist(c, d.source); if (!h) continue; const m = fns(h).find(f => f.name === name && sha(f.text) === d.sha256); if (m) { found = m.text; how = c; break; } }
  if (found === null) report.unknown.push({ name, source: a.source, declaredSource: d.source });
  else if (norm(found) === norm(a.text)) report.neutral.push({ name, via: how });
  else report.changed.push({ name, via: how, oldLen: found.length, newLen: a.text.length, assertionsOld: asserts(found), assertionsNew: asserts(a.text) });
}
for (const [name, d] of declared) if (!actual.has(name)) report.removed.push({ name, source: d.source });
writeFileSync("/private/tmp/testfix-real/units-preserved.json", JSON.stringify(report, null, 1));
console.log("actual", actual.size, "declared", declared.size, "neutral", report.neutral.length, "changed", report.changed.length, "unknown", report.unknown.length, "new", report.newNames.length, "removed", report.removed.length);
