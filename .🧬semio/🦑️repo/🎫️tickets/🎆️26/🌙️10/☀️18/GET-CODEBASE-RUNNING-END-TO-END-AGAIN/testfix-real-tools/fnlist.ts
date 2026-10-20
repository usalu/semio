import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import Parser from "web-tree-sitter";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
const root = process.argv[2]!, out = process.argv[3]!, jobs = JSON.parse(readFileSync(process.argv[4]!, "utf8")) as { id: string; path: string; mode: "file" | "tree"; kind?: string; name?: string }[];
await Parser.init({ wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root)) });
const p = new Parser(); p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const sha = (s: string) => createHash("sha256").update(s).digest("hex");
const asserts = (s: string) => (s.match(/\bassert(?:_eq|_ne)?!/g) ?? []).length;
const res: Record<string, unknown> = {};
for (const j of jobs) {
  const src = j.mode === "tree" ? readModuleTreeSource(resolve(root, j.path)) : readFileSync(resolve(root, j.path), "utf8");
  const t = p.parse(src);
  const rows: unknown[] = [];
  if (j.kind === "mod") { const m = t.rootNode.descendantsOfType("mod_item").find((n: any) => n.childForFieldName("name")?.text === j.name); res[j.id] = m ? { sha256: sha(m.text), len: m.text.length, fns: m.descendantsOfType("function_item").map((f: any) => ({ name: f.childForFieldName("name")?.text, sha256: sha(f.text), assertions: asserts(f.text) })) } : null; }
  else { for (const f of t.rootNode.descendantsOfType("function_item")) rows.push({ name: f.childForFieldName("name")?.text, sha256: sha(f.text), assertions: asserts(f.text), bytes: Buffer.byteLength(f.text), isLaw: (() => { for (let a = f.previousNamedSibling; a?.type === "attribute_item"; a = a.previousNamedSibling) if (/\btest\b|::async_test\b/.test(a.text)) return true; return false; })() }); res[j.id] = { bytes: Buffer.byteLength(src), sha256: sha(src), assertions: asserts(src), fns: rows }; }
  t.delete();
}
writeFileSync(out, JSON.stringify(res));
