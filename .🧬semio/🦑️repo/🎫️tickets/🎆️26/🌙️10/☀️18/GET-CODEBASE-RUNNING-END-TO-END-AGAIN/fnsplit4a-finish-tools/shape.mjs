// 🧪 Shape law over split directories. usage: bun shape.mjs [--root <repoRoot>] [--depth N] <dir>...  (repo-relative). Per dir: every 🟦️.ts|tsx|🦀️.rs from the dir down to depth N (default 3) must satisfy: path <= 255 UTF-16 units, components <= 255 bytes, module doc emoji equals folder emoji, doc emojis unique inside the file, folder emoji RGI and unique among siblings.
import {readdirSync, readFileSync, statSync} from "node:fs";
const isDir = (p) => { try { return statSync(p).isDirectory(); } catch { return false; } };
const isFile = (p) => { try { return statSync(p).isFile(); } catch { return false; } };
import {join} from "node:path";
const args = process.argv.slice(2);
let root = "/Users/ueli/Documents/semio", depth = 1, only = null;
const dirs = [];
for (let i = 0; i < args.length; i += 1) { if (args[i] === "--root") root = args[++i]; else if (args[i] === "--depth") depth = +args[++i]; else if (args[i] === "--only") only = new Set(args[++i].split(",")); else dirs.push(args[i]); }
const V = "️";
const emojiOf = (n) => { const i = n.indexOf(V); return i >= 0 ? n.slice(0, i) : n.slice(0, 1); };
const rgi = (n) => { const i = n.indexOf(V); const b = i >= 0 ? n.slice(0, i) : n.slice(0, 1); return i >= 0 && (/^\p{RGI_Emoji}$/v.test(b) || /^\p{RGI_Emoji}$/v.test(b + V)); };
const norm = (s) => s.replaceAll(V, "");
const isImpl = (f) => /^🟦️\.(ts|tsx)$|^🦀️\.rs$/u.test(f);
const problems = [];
const add = (p, m) => problems.push(`${p}: ${m}`);
const walk = (rel, d) => {
  const abs = join(root, rel);
  let ents;
  try { ents = readdirSync(abs, {withFileTypes: true}); } catch { return; }
  const subs = ents.filter((e) => isDir(join(abs, e.name)) && !e.name.startsWith(".") && e.name !== "node_modules");
  const seen = new Map();
  for (const e of subs) {
    if (!rgi(e.name)) add(join(rel, e.name), "folder emoji not RGI");
    const em = norm(emojiOf(e.name));
    if (seen.has(em)) { if (d === 0 && (!only || only.has(e.name) || only.has(seen.get(em)))) add(join(rel, e.name), `folder emoji ${em} duplicates ${seen.get(em)}`); } else seen.set(em, e.name);
  }
  for (const e of ents) if (isFile(join(abs, e.name)) && isImpl(e.name) && (d > 0 && (!only || only.has(rel.split("/").pop())))) {
    const p = join(rel, e.name);
    if (p.length > 255) add(p, `path ${p.length} units`);
    for (const c of p.split("/")) if (Buffer.byteLength(c) > 255) add(p, `component ${Buffer.byteLength(c)} bytes`);
    const folder = rel.split("/").pop();
    const text = readFileSync(join(root, p), "utf8");
    const m = /^(?:\/\/!|\/\*\*)\s*(\S+)/.exec(text);
    const head = m ? norm(m[1]) : "";
    if (folder && head !== norm(emojiOf(folder))) add(p, `head emoji ${JSON.stringify(m && m[1])} != folder ${JSON.stringify(norm(emojiOf(folder)))}`);
    const docs = [...text.matchAll(/^[ \t]*(?:\/\/\/|\/\*\*)[ \t]*(\S+)/gm)].map((x) => norm(x[1]));
    const dup = [...new Set(docs.filter((x, i) => docs.indexOf(x) !== i))];
    if (dup.length) add(p, `duplicate doc emoji ${dup.join(" ")}`);
  }
  if (d < depth) for (const e of subs) walk(join(rel, e.name), d + 1);
};
for (const d of dirs) walk(d, 0);
for (const p of problems) console.log("FINDING", p);
console.log(`shape problems=${problems.length}`);
