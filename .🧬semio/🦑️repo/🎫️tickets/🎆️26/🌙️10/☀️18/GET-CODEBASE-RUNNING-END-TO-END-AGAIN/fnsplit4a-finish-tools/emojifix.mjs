// 🪄 Applies a doc-emoji spec to copies of live files: line edits (old emoji asserted) and an inserted module doc head. usage: bun emojifix.mjs <spec.json> <srcRoot> <outRoot>
// Proves: only comments changed (TypeScript token streams equal), head emoji equals the folder emoji, doc emojis unique in the file.
import {readFileSync, realpathSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {createRequire} from "node:module";
const ts = createRequire(import.meta.url)("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const [specPath, srcRoot, outRoot] = process.argv.slice(2);
const V = "️";
const norm = (s) => s.replaceAll(V, "");
const emojiOf = (n) => { const i = n.indexOf(V); return i >= 0 ? n.slice(0, i) : n.slice(0, 1); };
const DOC = /^([ \t]*(?:\/\/\/|\/\*\*)[ \t]*)(\S+)/;
const tokens = (file, text) => {
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const out = [];
  const walk = (n) => { if (n.kind >= ts.SyntaxKind.FirstJSDocNode && n.kind <= ts.SyntaxKind.LastJSDocNode) return; const kids = n.getChildren(sf); if (!kids.length) out.push(n.kind + ":" + n.getText(sf)); else kids.forEach(walk); };
  walk(sf);
  return out;
};
const spec = JSON.parse(readFileSync(specPath, "utf8"));
let bad = 0;
for (const f of spec.files) {
  const text = readFileSync(join(srcRoot, f.file), "utf8");
  const lines = text.split("\n");
  const problems = [];
  for (const e of [...(f.set ?? [])].sort((a, b) => b.line - a.line)) {
    const m = DOC.exec(lines[e.line - 1] ?? "");
    if (!m) { problems.push(`line ${e.line} is not a doc line`); continue; }
    if (norm(m[2]) !== norm(e.from)) { problems.push(`line ${e.line}: expected ${e.from}, found ${m[2]}`); continue; }
    lines[e.line - 1] = lines[e.line - 1].replace(DOC, (_, pre) => pre + e.to + V);
  }
  const parts = f.file.split("/");
  const folderEmoji = norm(emojiOf(parts[parts.length - 2]));
  if (f.head) lines.unshift(`/** ${folderEmoji}${V} ${f.head} */`);
  const out = lines.join("\n");
  const first = DOC.exec(out.split("\n")[0]);
  if (!first || norm(first[2]) !== folderEmoji) problems.push(`head emoji ${first && first[2]} != folder ${folderEmoji}`);
  const docs = out.split("\n").map((l) => DOC.exec(l)).filter(Boolean).map((m) => norm(m[2]));
  const dup = [...new Set(docs.filter((d, i) => docs.indexOf(d) !== i))];
  if (dup.length) problems.push(`duplicate doc emoji ${dup.join(" ")}`);
  const a = tokens(f.file, text).join("\n"), b = tokens(f.file, out).join("\n");
  if (a !== b) problems.push("token streams differ");
  if (problems.length) { bad += 1; console.log("PROBLEM", f.file.split("/").slice(-2).join("/"), problems.join("; ")); continue; }
  if (!realpathSync(dirname(join(outRoot, f.file))).startsWith(realpathSync(outRoot))) throw new Error("would write through a symlink: " + f.file);
  writeFileSync(join(outRoot, f.file), out);
  console.log("OK", f.file.split("/").slice(-2).join("/"));
}
console.log(`emojifix problems=${bad} files=${spec.files.length}`);
